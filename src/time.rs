use std::time::Duration;

use futures_channel::oneshot;
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::global::window;

pub async fn sleep(duration: &Duration) {
    let (send, recv) = oneshot::channel();

    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            Closure::once_into_js(move || {
                send.send(()).unwrap();
            })
            .unchecked_ref(),
            duration.as_millis() as i32,
        )
        .unwrap();

    recv.await.unwrap();
}
