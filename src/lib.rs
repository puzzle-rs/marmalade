#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::future_not_send,
    clippy::cast_possible_truncation,
    clippy::missing_panics_doc,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::new_without_default,
    clippy::option_if_let_else,
    clippy::module_name_repetitions
)]

pub mod console;
pub mod image;
pub mod input;
pub mod marmalade_context;
pub mod net;
pub mod render;
pub mod time;

pub extern crate console_error_panic_hook;
pub extern crate glam;
pub extern crate wasm_bindgen_futures;
