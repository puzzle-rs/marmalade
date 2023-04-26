#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::future_not_send,
    clippy::cast_possible_truncation,
    clippy::missing_panics_doc,
    clippy::cast_sign_loss
)]

pub mod console;
pub mod image;
pub mod input;
pub mod render;
pub mod time;
