use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{window, Document, Window};

use crate::{
    input::{Keyboard, Mouse},
    render::Canvas,
};

pub struct MarmaladeContext {
    pub window: Rc<Window>,
    pub document: Document,
    pub canvas: Canvas,
    pub keyboard: Keyboard,
    pub mouse: Mouse,

    draw_closure: Rc<RefCell<Box<dyn FnMut()>>>,
}

impl MarmaladeContext {
    #[must_use]
    pub fn new(canvas_id: &str) -> Self {
        let window = Rc::new(window().unwrap());
        let document = window.document().unwrap();
        let canvas = Canvas::new(&document, canvas_id);
        let keyboard = Keyboard::new(&window);
        let mouse = Mouse::new(&window);

        let draw_closure: Rc<RefCell<Box<dyn FnMut()>>> = Rc::new(RefCell::new(Box::new(|| {})));

        let request_animation_frame_closure = Rc::new(RefCell::<Option<JsValue>>::new(None));

        let request_animation_frame_closure_clone = request_animation_frame_closure.clone();
        let draw_closure_clone = draw_closure.clone();

        let window_clone = window.clone();
        *request_animation_frame_closure.borrow_mut() = Some(
            Closure::<dyn FnMut()>::new(move || {
                (draw_closure_clone.borrow_mut())();

                window_clone
                    .request_animation_frame(
                        request_animation_frame_closure_clone
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .unchecked_ref(),
                    )
                    .unwrap();
            })
            .into_js_value(),
        );

        window
            .request_animation_frame(
                request_animation_frame_closure
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .unchecked_ref(),
            )
            .unwrap();

        Self {
            window,
            document,
            canvas,
            keyboard,
            mouse,
            draw_closure,
        }
    }

    pub fn set_on_draw<T: FnMut() + 'static>(&mut self, closure: T) {
        *self.draw_closure.borrow_mut() = Box::new(closure);
    }
}
