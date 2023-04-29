use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, Document, Window};

use crate::{input::Keyboard, render::Canvas};

pub struct MarmaladeContext {
    pub window: Rc<Window>,
    pub document: Document,
    pub canvas: Canvas,
    pub keyboard: Keyboard,

    draw_closure: Rc<RefCell<Box<dyn FnMut()>>>,
}

impl MarmaladeContext {
    #[must_use]
    pub fn new(canvas_id: &str) -> Self {
        let window = Rc::new(window().unwrap());
        let document = window.document().unwrap();
        let canvas = Canvas::new(&document, canvas_id);
        let keyboard = Keyboard::new(&window);

        let draw_closure: Rc<RefCell<Box<dyn FnMut()>>> = Rc::new(RefCell::new(Box::new(|| {})));

        let request_animation_frame_closure =
            Rc::new(RefCell::<Option<Closure<dyn FnMut()>>>::new(None));

        let request_animation_frame_closure_clone = request_animation_frame_closure.clone();
        let draw_closure_clone = draw_closure.clone();

        let window_clone = window.clone();
        *request_animation_frame_closure.borrow_mut() =
            Some(Closure::<dyn FnMut()>::new(move || {
                (draw_closure_clone.borrow_mut())();

                window_clone
                    .request_animation_frame(
                        request_animation_frame_closure_clone
                            .borrow()
                            .as_ref()
                            .unwrap()
                            .as_ref()
                            .unchecked_ref(),
                    )
                    .unwrap();
            }));

        window
            .request_animation_frame(
                request_animation_frame_closure
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .unchecked_ref(),
            )
            .unwrap();

        Self {
            window,
            document,
            canvas,
            keyboard,
            draw_closure,
        }
    }

    pub fn set_on_draw<T: FnMut() + 'static>(&mut self, closure: T) {
        *self.draw_closure.borrow_mut() = Box::new(closure);
    }
}
