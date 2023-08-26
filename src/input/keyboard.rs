use std::{cell::RefCell, collections::HashSet, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::KeyboardEvent;

use crate::global::window;

use super::key::Key;

struct Keyboard {
    keys_down: Rc<RefCell<HashSet<Key>>>,
    keys_pressed: Rc<RefCell<HashSet<Key>>>,
}

impl Keyboard {
    #[must_use]
    fn new() -> Self {
        let window = window();

        let keys_down = Rc::new(RefCell::new(HashSet::new()));
        let keys_pressed = Rc::new(RefCell::new(HashSet::new()));

        let keys_down_clone = keys_down.clone();
        let keys_pressed_clone = keys_pressed.clone();

        window.set_onkeydown(Some(
            Closure::<dyn Fn(KeyboardEvent) -> bool>::new(move |event: KeyboardEvent| {
                if let Some(key) = Key::from_code(event.code().as_str()) {
                    keys_down_clone.borrow_mut().insert(key);
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
}

thread_local! {
    static KEYBOARD:Keyboard = Keyboard::new();
}

/// Returns true if the given key is currently down, false otherwise
#[must_use]
pub fn is_down(key: Key) -> bool {
    KEYBOARD.with(|k| k.keys_down.borrow().contains(&key))
}

/// Returns true if the given key has been pressed since last `is_pressed` call, false otherwise
#[must_use]
pub fn is_pressed(key: Key) -> bool {
    KEYBOARD.with(|k| k.keys_pressed.borrow_mut().remove(&key))
}
