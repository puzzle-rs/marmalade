#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::future_not_send,
    clippy::cast_possible_truncation,
    clippy::missing_panics_doc,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::new_without_default,
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap
)]

pub mod audio;
pub mod console;
pub mod dom_stack;
pub mod draw_scheduler;
pub mod global;
pub mod image;
pub mod input;
pub mod net;
pub mod render;
pub mod tick_scheduler;
pub mod time;
