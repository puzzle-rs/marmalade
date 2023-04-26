use futures_channel::mpsc;
use futures_util::StreamExt;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;

pub async fn load(src: &str) -> Result<HtmlImageElement, ()> {
    let img = HtmlImageElement::new().unwrap();

    let (mut send, mut recv) = mpsc::channel(1);

    let mut send_clone = send.clone();

    let load_closure = Closure::<dyn FnMut()>::new(move || {
        send_clone.try_send(true).unwrap();
    });
    img.set_onload(Some(load_closure.as_ref().unchecked_ref()));

    let err_closure = Closure::<dyn FnMut()>::new(move || {
        send.try_send(false).unwrap();
    });
    img.set_onerror(Some(err_closure.as_ref().unchecked_ref()));

    img.set_src(src);

    if recv.next().await.unwrap() {
        Ok(img)
    } else {
        Err(())
    }
}
