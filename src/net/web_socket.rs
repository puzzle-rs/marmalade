use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    rc::Rc,
};

use futures_channel::mpsc;
use futures_util::StreamExt;
use js_sys::{ArrayBuffer, JsString};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::MessageEvent;

pub enum Message {
    Binary(Vec<u8>),
    Text(String),
}

pub struct WebSocket {
    ws: web_sys::WebSocket,
    recv: Rc<RefCell<VecDeque<Message>>>,
    open: Rc<Cell<bool>>,
}

pub struct WebSocketError;

impl WebSocket {
    pub async fn new(url: &str) -> Result<Self, ()> {
        if let Ok(ws) = web_sys::WebSocket::new(url) {
            let (mut send, mut recv) = mpsc::channel(0);

            let mut send_clone = send.clone();

            ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

            ws.set_onopen(Some(
                Closure::once(move || {
                    send_clone.try_send(true).unwrap();
                })
                .into_js_value()
                .unchecked_ref(),
            ));

            ws.set_onerror(Some(
                Closure::once(move || {
                    send.try_send(false).unwrap();
                })
                .into_js_value()
                .unchecked_ref(),
            ));

            if recv.next().await.unwrap() {
                let recv = Rc::new(RefCell::new(VecDeque::new()));
                let open = Rc::new(Cell::new(true));

                let open_clone = open.clone();

                ws.set_onerror(Some(
                    Closure::once(move || {
                        open_clone.set(false);
                    })
                    .into_js_value()
                    .unchecked_ref(),
                ));

                let recv_clone = recv.clone();

                ws.set_onmessage(Some(
                    Closure::<dyn Fn(MessageEvent)>::new(move |e: MessageEvent| {
                        recv_clone.borrow_mut().push_back(
                            if let Ok(buffer) = e.data().dyn_into::<ArrayBuffer>() {
                                Message::Binary(js_sys::Uint8Array::new(&buffer).to_vec())
                            } else if let Ok(text) = e.data().dyn_into::<JsString>() {
                                Message::Text(text.into())
                            } else {
                                unreachable!()
                            },
                        );
                    })
                    .into_js_value()
                    .unchecked_ref(),
                ));

                Ok(Self { ws, recv, open })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    #[must_use]
    pub fn is_open(&self) -> bool {
        self.open.get()
    }

    pub fn close(&self) {
        self.ws.close().unwrap();
        self.open.set(false);
    }

    pub fn read(&self) -> Result<Option<Message>, WebSocketError> {
        if let Some(msg) = self.recv.borrow_mut().pop_front() {
            Ok(Some(msg))
        } else if self.open.get() {
            Ok(None)
        } else {
            Err(WebSocketError)
        }
    }

    pub fn send_str(&self, msg: &str) -> Result<(), WebSocketError> {
        if self.ws.send_with_str(msg).is_err() {
            self.open.set(false);
            Err(WebSocketError)
        } else {
            Ok(())
        }
    }

    pub fn send_bin(&self, msg: &[u8]) -> Result<(), WebSocketError> {
        if self.ws.send_with_u8_array(msg).is_err() {
            self.open.set(false);
            Err(WebSocketError)
        } else {
            Ok(())
        }
    }
}
