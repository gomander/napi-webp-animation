use napi::bindgen_prelude::{AsyncTask, Buffer};
use napi::{Env, Error as NapiError, Result, Task};
use napi_derive::napi;
use std::io::Error as IoError;
use webp_animation::{
  AnimParams, Encoder, EncoderOptions, EncodingConfig, EncodingType, Error as WebpAnimationError,
  LossyEncodingConfig,
};

#[derive(Clone)]
struct WebpEncoderFrame {
  frame_data: Vec<u8>,
  duration: Option<f64>,
}

#[derive(Clone)]
pub struct WebpEncoder {
  width: u32,
  height: u32,
  frame_rate: u16,
  frames: Vec<WebpEncoderFrame>,
}

impl WebpEncoder {
  pub fn new(width: u32, height: u32) -> Self {
    WebpEncoder {
      width,
      height,
      frame_rate: 30,
      frames: Vec::new(),
    }
  }

  pub fn set_frame_rate(&mut self, frame_rate: u16) {
    self.frame_rate = frame_rate;
  }

  pub fn add_frame(&mut self, frame_data: Vec<u8>, duration: Option<f64>) {
    self.frames.push(WebpEncoderFrame {
      frame_data,
      duration,
    });
  }

  pub fn get_buffer(&self, options: Option<&EncoderOptions>) -> Result<Vec<u8>> {
    let options = options.cloned().unwrap_or_default();
    let mut encoder = Encoder::new_with_options((self.width, self.height), options)
      .map_err(EncoderError::EncoderError)?;
    let mut timestamp: i32 = 0;
    for frame in &self.frames {
      encoder
        .add_frame(&frame.frame_data, timestamp)
        .map_err(EncoderError::EncoderError)?;
      timestamp += frame
        .duration
        .unwrap_or(1000. / (self.frame_rate as f64))
        .max(1.0) as i32;
    }
    Ok(
      encoder
        .finalize(timestamp)
        .map_err(EncoderError::EncoderError)?
        .to_vec(),
    )
  }

  pub fn write_to_file(&self, path: String, options: Option<&EncoderOptions>) -> Result<Vec<u8>> {
    let buffer = self.get_buffer(options)?;
    std::fs::write(
      if path.ends_with(".webp") {
        path
      } else {
        format!("{path}.webp")
      },
      &buffer,
    )
    .map_err(EncoderError::WriteError)?;
    Ok(buffer)
  }

  pub fn clear_frames(&mut self) {
    self.frames.clear();
  }

  pub fn set_dimensions(&mut self, width: u32, height: u32) {
    self.width = width;
    self.height = height;
  }
}

#[napi(object, js_name = "WebpEncoderOptions")]
pub struct JsWebpEncoderOptions {
  pub lossless: Option<bool>,
  pub quality: Option<u8>,
  pub loop_count: Option<i32>,
}

pub struct AsyncGetBuffer {
  encoder: WebpEncoder,
  options: Option<EncoderOptions>,
}

#[napi]
impl Task for AsyncGetBuffer {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    self.encoder.get_buffer(self.options.as_ref())
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

pub struct AsyncWriteToFile {
  encoder: WebpEncoder,
  path: String,
  options: Option<EncoderOptions>,
}

#[napi]
impl Task for AsyncWriteToFile {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    self
      .encoder
      .write_to_file(self.path.clone(), self.options.as_ref())
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi(js_name = "WebpEncoder")]
pub struct JsWebpEncoder {
  encoder: WebpEncoder,
}

#[napi]
impl JsWebpEncoder {
  #[napi(constructor)]
  pub fn new(width: u32, height: u32) -> Self {
    JsWebpEncoder {
      encoder: WebpEncoder::new(width, height),
    }
  }

  #[napi]
  pub fn set_frame_rate(&mut self, frame_rate: u16) {
    self.encoder.set_frame_rate(frame_rate)
  }

  #[napi]
  pub fn add_frame(
    &mut self,
    frame_data: Buffer,
    #[napi(ts_arg_type = "number")] duration: Option<f64>,
  ) {
    self.encoder.add_frame(frame_data.into(), duration)
  }

  #[napi]
  pub fn get_buffer(
    &self,
    #[napi(ts_arg_type = "WebpEncoderOptions")] options: Option<JsWebpEncoderOptions>,
  ) -> AsyncTask<AsyncGetBuffer> {
    AsyncTask::new(AsyncGetBuffer {
      encoder: self.encoder.clone(),
      options: options.map(map_js_webp_encoder_options),
    })
  }

  #[napi]
  pub fn get_buffer_sync(
    &self,
    #[napi(ts_arg_type = "WebpEncoderOptions")] options: Option<JsWebpEncoderOptions>,
  ) -> Result<Buffer> {
    Ok(
      self
        .encoder
        .get_buffer(options.map(map_js_webp_encoder_options).as_ref())?
        .into(),
    )
  }

  #[napi]
  pub fn write_to_file(
    &mut self,
    path: String,
    #[napi(ts_arg_type = "WebpEncoderOptions")] options: Option<JsWebpEncoderOptions>,
  ) -> AsyncTask<AsyncWriteToFile> {
    AsyncTask::new(AsyncWriteToFile {
      encoder: self.encoder.clone(),
      path,
      options: options.map(map_js_webp_encoder_options),
    })
  }

  #[napi]
  pub fn write_to_file_sync(
    &self,
    path: String,
    #[napi(ts_arg_type = "WebpEncoderOptions")] options: Option<JsWebpEncoderOptions>,
  ) -> Result<Buffer> {
    Ok(
      self
        .encoder
        .write_to_file(path, options.map(map_js_webp_encoder_options).as_ref())?
        .into(),
    )
  }

  #[napi]
  pub fn clear_frames(&mut self) {
    self.encoder.clear_frames();
  }

  #[napi]
  pub fn set_dimensions(&mut self, width: u32, height: u32) {
    self.encoder.set_dimensions(width, height);
  }
}

fn map_js_webp_encoder_options(options: JsWebpEncoderOptions) -> EncoderOptions {
  EncoderOptions {
    anim_params: AnimParams {
      loop_count: options.loop_count.unwrap_or(0),
    },
    encoding_config: Some(EncodingConfig {
      encoding_type: if options.lossless.unwrap_or(true) {
        EncodingType::Lossless
      } else {
        EncodingType::Lossy(LossyEncodingConfig::default())
      },
      quality: options.quality.unwrap_or(1) as f32,
      ..Default::default()
    }),
    ..Default::default()
  }
}

#[derive(thiserror::Error, Debug)]
enum EncoderError {
  #[error("WebP encoder encountered an error: {0}")]
  EncoderError(#[from] WebpAnimationError),
  #[error("Failed to write to file: {0}")]
  WriteError(#[from] IoError),
}

impl From<EncoderError> for NapiError {
  fn from(value: EncoderError) -> Self {
    NapiError::from_reason(value.to_string())
  }
}
