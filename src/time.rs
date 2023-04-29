use std::time::Duration;

use futures_channel::oneshot;
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::marmalade_context::MarmaladeContext;

impl MarmaladeContext {
    pub async fn sleep(&self, duration: Duration) {
        let (send, recv) = oneshot::channel();

        let callback = Closure::once(move || {
            send.send(()).unwrap();
        });

        self.window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                duration.as_millis() as i32,
            )
            .unwrap();

        recv.await.unwrap();
    }
}
