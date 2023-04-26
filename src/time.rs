use std::time::Duration;

use futures_channel::oneshot;
use wasm_bindgen::{prelude::Closure, JsCast};

pub async fn sleep(duration: Duration) {
    let (send, recv) = oneshot::channel();

    let callback = Closure::once(move || {
        send.send(()).unwrap();
    });

    web_sys::window()
        .unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.as_ref().unchecked_ref(),
            duration.as_millis() as i32,
        )
        .unwrap();

    recv.await.unwrap();
}
