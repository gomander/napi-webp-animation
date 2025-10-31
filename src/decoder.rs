use napi::bindgen_prelude::Buffer;
use napi::{Error as NapiError, Result, Status};
use napi_derive::napi;
use webp_animation::Decoder;

#[napi(object)]
struct WebpFrame {
  pub data: Buffer,
  pub timestamp: f64,
}

#[napi(object)]
struct DecodedWebp {
  pub width: u32,
  pub height: u32,
  pub frames: Vec<WebpFrame>,
}

#[napi]
fn decode_webp(buffer: Buffer) -> Result<DecodedWebp> {
  match Decoder::new(&buffer) {
    Ok(decoder) => {
      let (width, height) = decoder.dimensions();
      let frames = decoder
        .into_iter()
        .map(|frame| WebpFrame {
          data: frame.data().into(),
          timestamp: frame.timestamp() as f64,
        })
        .collect();
      Ok(DecodedWebp {
        width,
        height,
        frames,
      })
    }
    Err(e) => Err(NapiError::new(Status::GenericFailure, e.to_string())),
  }
}
