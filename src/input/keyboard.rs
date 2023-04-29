use std::{cell::RefCell, collections::HashSet, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, KeyboardEvent, Window};

use super::key::Key;

pub struct Keyboard {
    keys_down: Rc<RefCell<HashSet<Key>>>,
    keys_pressed: Rc<RefCell<HashSet<Key>>>,

    _key_down_closure: Closure<dyn FnMut(KeyboardEvent) -> bool>,
    _key_up_closure: Closure<dyn FnMut(KeyboardEvent)>,
}

impl Keyboard {
    #[must_use]
    pub fn new(window: &Window) -> Self {
        let keys_down = Rc::new(RefCell::new(HashSet::new()));
        let keys_pressed = Rc::new(RefCell::new(HashSet::new()));

        let keys_down_clone = keys_down.clone();
        let keys_pressed_clone = keys_pressed.clone();

        let key_down_closure = Closure::new(move |event: KeyboardEvent| {
            if let Some(key) = Key::from_code(event.code().as_str()) {
                keys_down_clone.borrow_mut().insert(key.clone());
                keys_pressed_clone.borrow_mut().insert(key);
            }

            false
        });

        let keys_down_clone = keys_down.clone();

        let key_up_closure = Closure::new(move |event: KeyboardEvent| {
            if let Some(key) = Key::from_code(event.code().as_str()) {
                keys_down_clone.borrow_mut().remove(&key);
            }
        });

        window.set_onkeydown(Some(key_down_closure.as_ref().unchecked_ref()));
        window.set_onkeyup(Some(key_up_closure.as_ref().unchecked_ref()));

        Self {
            keys_down,
            keys_pressed,
            _key_down_closure: key_down_closure,
            _key_up_closure: key_up_closure,
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
