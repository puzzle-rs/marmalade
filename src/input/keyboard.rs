use std::{cell::RefCell, collections::HashSet, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, KeyboardEvent, Window};

use super::key::Key;

pub struct Keyboard {
    keys_down: Rc<RefCell<HashSet<Key>>>,
    keys_pressed: Rc<RefCell<HashSet<Key>>>,
}

impl Keyboard {
    #[must_use]
    pub fn new(window: &Window) -> Self {
        let keys_down = Rc::new(RefCell::new(HashSet::new()));
        let keys_pressed = Rc::new(RefCell::new(HashSet::new()));

        let keys_down_clone = keys_down.clone();
        let keys_pressed_clone = keys_pressed.clone();

        window.set_onkeydown(Some(
            Closure::<dyn Fn(KeyboardEvent) -> bool>::new(move |event: KeyboardEvent| {
                if let Some(key) = Key::from_code(event.code().as_str()) {
                    keys_down_clone.borrow_mut().insert(key.clone());
                    keys_pressed_clone.borrow_mut().insert(key);
                }

                false
            })
            .into_js_value()
            .unchecked_ref(),
        ));

        let keys_down_clone = keys_down.clone();

        window.set_onkeyup(Some(
            Closure::<dyn Fn(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                if let Some(key) = Key::from_code(event.code().as_str()) {
                    keys_down_clone.borrow_mut().remove(&key);
                }
            })
            .into_js_value()
            .unchecked_ref(),
        ));

        Self {
            keys_down,
            keys_pressed,
        }
    }

    #[must_use]
    pub fn is_down(&self, key: &Key) -> bool {
        self.keys_down.borrow().contains(key)
    }

    #[must_use]
    pub fn is_pressed(&self, key: &Key) -> bool {
        self.keys_pressed.borrow_mut().remove(key)
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        let window = window().unwrap();

        window.set_onkeydown(None);
        window.set_onkeyup(None);
    }
}
