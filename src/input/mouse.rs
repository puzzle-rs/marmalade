use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
    rc::Rc,
};

use glam::IVec2;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{window, AddEventListenerOptions, MouseEvent, WheelEvent};

use super::Button;

pub struct Mouse {
    buttons_down: Rc<RefCell<HashSet<Button>>>,
    buttons_pressed: Rc<RefCell<HashSet<Button>>>,
    wheel_move: Rc<Cell<f64>>,
    mouse_pos: Rc<Cell<IVec2>>,

    wheel_closure: JsValue,
}

impl Mouse {
    #[must_use]
    pub fn new() -> Self {
        let window = window().unwrap();

        let buttons_down = Rc::new(RefCell::new(HashSet::new()));
        let buttons_pressed = Rc::new(RefCell::new(HashSet::new()));
        let wheel_move = Rc::new(Cell::new(0.));
        let mouse_pos = Rc::new(Cell::new(IVec2::ZERO));

        let buttons_down_clone = buttons_down.clone();
        let buttons_pressed_clone = buttons_pressed.clone();
        window.set_onmousedown(Some(
            Closure::<dyn Fn(MouseEvent) -> bool>::new(move |event: MouseEvent| {
                if let Some(button) = Button::from_code(event.button()) {
                    buttons_down_clone.borrow_mut().insert(button.clone());
                    buttons_pressed_clone.borrow_mut().insert(button);
                }

                false
            })
            .into_js_value()
            .unchecked_ref(),
        ));

        let buttons_down_clone = buttons_down.clone();
        window.set_onmouseup(Some(
            Closure::<dyn Fn(MouseEvent)>::new(move |event: MouseEvent| {
                if let Some(button) = Button::from_code(event.button()) {
                    buttons_down_clone.borrow_mut().remove(&button);
                }
            })
            .into_js_value()
            .unchecked_ref(),
        ));

        window.set_oncontextmenu(Some(
            Closure::<dyn Fn() -> bool>::new(|| false)
                .into_js_value()
                .unchecked_ref(),
        ));

        let wheel_move_clone = wheel_move.clone();
        let wheel_closure = Closure::<dyn Fn(WheelEvent)>::new(move |event: WheelEvent| {
            wheel_move_clone.set(event.delta_y());
            event.prevent_default();
        })
        .into_js_value();

        let mut wheel_event_listener_options = AddEventListenerOptions::new();
        wheel_event_listener_options.passive(false);
        window
            .add_event_listener_with_callback_and_add_event_listener_options(
                "wheel",
                wheel_closure.unchecked_ref(),
                &wheel_event_listener_options,
            )
            .unwrap();

        let mouse_pos_clone = mouse_pos.clone();
        window.set_onmousemove(Some(
            Closure::<dyn Fn(MouseEvent)>::new(move |event: MouseEvent| {
                mouse_pos_clone.set(IVec2::new(event.page_x(), event.page_y()));
            })
            .into_js_value()
            .unchecked_ref(),
        ));

        Self {
            buttons_down,
            buttons_pressed,
            wheel_move,
            mouse_pos,
            wheel_closure,
        }
    }

    #[must_use]
    pub fn is_down(&self, button: &Button) -> bool {
        self.buttons_down.borrow().contains(button)
    }

    #[must_use]
    pub fn is_pressed(&self, button: &Button) -> bool {
        self.buttons_pressed.borrow_mut().remove(button)
    }

    #[must_use]
    pub fn wheel_scroll(&self) -> f64 {
        self.wheel_move.replace(0.)
    }

    #[must_use]
    pub fn position(&self) -> IVec2 {
        self.mouse_pos.get()
    }
}

impl Drop for Mouse {
    fn drop(&mut self) {
        let window = window().unwrap();

        window.set_onmousedown(None);
        window.set_onmouseup(None);

        window
            .remove_event_listener_with_callback("wheel", self.wheel_closure.unchecked_ref())
            .unwrap();
    }
}
