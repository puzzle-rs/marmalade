use base64::{engine::general_purpose, Engine};
use futures_channel::mpsc;
use futures_util::StreamExt;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;

pub async fn load(src: &str) -> Result<HtmlImageElement, ()> {
    let img = HtmlImageElement::new().unwrap();

    let (mut send, mut recv) = mpsc::channel(0);

    let mut send_clone = send.clone();

    img.set_onload(Some(
        Closure::once(move || {
            send_clone.try_send(true).unwrap();
        })
        .into_js_value()
        .unchecked_ref(),
    ));

    img.set_onerror(Some(
        Closure::once(move || {
            send.try_send(false).unwrap();
        })
        .into_js_value()
        .unchecked_ref(),
    ));

    img.set_src(src);

    if recv.next().await.unwrap() {
        Ok(img)
    } else {
        Err(())
    }
}

pub enum Format {
    Apng,
    Avif,
    Gif,
    Jpeg,
    Png,
    Svg,
    WebP,
}

impl Format {
    #[must_use]
    pub const fn get_mime(&self) -> &str {
        match self {
            Self::Apng => "image/apng",
            Self::Avif => "image/avif",
            Self::Gif => "image/gif",
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::Svg => "image/svg+xml",
            Self::WebP => "image/webp",
        }
    }
}

pub async fn from_bytes(bytes: &[u8], format: &Format) -> Result<HtmlImageElement, ()> {
    let mut img = format!("data:{};base64,", format.get_mime());

    general_purpose::STANDARD.encode_string(bytes, &mut img);

    load(&img).await
}
