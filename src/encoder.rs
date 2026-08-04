use napi::bindgen_prelude::{AsyncTask, Buffer, Status};
use napi::{Env, Error as NapiError, Result, Task};
use napi_derive::napi;
use std::fs;
use std::io::Error as IoError;
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};
use webp_animation::{
  AnimParams, Encoder, EncoderOptions, EncodingConfig, EncodingType, Error as WebpAnimationError,
  LossyEncodingConfig,
};

struct WebpEncoderFrame {
  frame_data: Vec<u8>,
  duration: Option<i32>,
}

pub struct WebpEncoder {
  width: u32,
  height: u32,
  frame_rate: i32,
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

  pub fn set_frame_rate(&mut self, frame_rate: i32) {
    self.frame_rate = frame_rate;
  }

  pub fn add_frame(&mut self, frame_data: Vec<u8>, duration: Option<i32>) {
    self.frames.push(WebpEncoderFrame {
      frame_data,
      duration,
    });
  }

  pub fn get_buffer(&self, options: Option<&EncoderOptions>) -> Result<Vec<u8>> {
    let options = options.cloned().unwrap_or_default();
    let mut encoder = Encoder::new_with_options((self.width, self.height), options)
      .map_err(EncoderError::Encode)?;
    let mut timestamp = 0;
    for frame in &self.frames {
      encoder
        .add_frame(&frame.frame_data, timestamp)
        .map_err(EncoderError::Encode)?;
      timestamp += frame.duration.unwrap_or(1000 / self.frame_rate).max(1);
    }
    Ok(
      encoder
        .finalize(timestamp)
        .map_err(EncoderError::Encode)?
        .to_vec(),
    )
  }

  pub fn write_to_file(&self, path: &str, options: Option<&EncoderOptions>) -> Result<()> {
    fs::write(
      Path::new(path).with_extension("webp"),
      &self.get_buffer(options)?,
    )
    .map_err(EncoderError::Write)?;
    Ok(())
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
  encoder: Arc<Mutex<WebpEncoder>>,
  options: Option<EncoderOptions>,
}

#[napi]
impl Task for AsyncGetBuffer {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let encoder = self
      .encoder
      .lock()
      .map_err(|_| NapiError::from_reason("Mutex lock failed"))?;
    encoder.get_buffer(self.options.as_ref())
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

pub struct AsyncWriteToFile {
  encoder: Arc<Mutex<WebpEncoder>>,
  path: String,
  options: Option<EncoderOptions>,
}

#[napi]
impl Task for AsyncWriteToFile {
  type Output = ();
  type JsValue = ();

  fn compute(&mut self) -> Result<Self::Output> {
    let encoder = self
      .encoder
      .lock()
      .map_err(|_| NapiError::from_reason("Mutex lock failed"))?;
    encoder.write_to_file(&self.path, self.options.as_ref())
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi(js_name = "WebpEncoder")]
pub struct JsWebpEncoder {
  encoder: Arc<Mutex<WebpEncoder>>,
}

#[napi]
impl JsWebpEncoder {
  #[napi(constructor)]
  pub fn new(width: i32, height: i32) -> Result<Self> {
    if width < 1 || height < 1 {
      return Err(NapiError::new(
        Status::InvalidArg,
        "width and height must be positive integers",
      ));
    }
    Ok(JsWebpEncoder {
      encoder: Arc::new(Mutex::new(WebpEncoder::new(
        width.cast_unsigned(),
        height.cast_unsigned(),
      ))),
    })
  }

  fn lock(&self) -> Result<MutexGuard<'_, WebpEncoder>> {
    self
      .encoder
      .lock()
      .map_err(|_| NapiError::from_reason("Mutex lock failed"))
  }

  #[napi]
  pub fn set_frame_rate(&mut self, frame_rate: i32) -> Result<()> {
    if !(1..=1000).contains(&frame_rate) {
      return Err(NapiError::new(
        Status::InvalidArg,
        "frame_rate must be an integer between 1 and 1000",
      ));
    }
    let mut encoder = self.lock()?;
    encoder.set_frame_rate(frame_rate);
    Ok(())
  }

  #[napi]
  pub fn add_frame(
    &mut self,
    frame_data: Buffer,
    #[napi(ts_arg_type = "number")] duration: Option<i32>,
  ) -> Result<()> {
    if duration.is_some_and(|duration| duration < 1) {
      return Err(NapiError::new(
        Status::InvalidArg,
        "duration must be a positive integer",
      ));
    }
    let mut encoder = self.lock()?;
    encoder.add_frame(frame_data.into(), duration);
    Ok(())
  }

  #[napi]
  pub fn get_buffer(
    &self,
    #[napi(ts_arg_type = "WebpEncoderOptions")] options: Option<JsWebpEncoderOptions>,
  ) -> AsyncTask<AsyncGetBuffer> {
    AsyncTask::new(AsyncGetBuffer {
      encoder: Arc::clone(&self.encoder),
      options: options.map(|opts| map_js_webp_encoder_options(&opts)),
    })
  }

  #[napi]
  pub fn get_buffer_sync(
    &self,
    #[napi(ts_arg_type = "WebpEncoderOptions")] options: Option<JsWebpEncoderOptions>,
  ) -> Result<Buffer> {
    let encoder = self.lock()?;
    Ok(
      encoder
        .get_buffer(
          options
            .map(|opts| map_js_webp_encoder_options(&opts))
            .as_ref(),
        )?
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
      encoder: Arc::clone(&self.encoder),
      path,
      options: options.map(|opts| map_js_webp_encoder_options(&opts)),
    })
  }

  #[napi]
  pub fn write_to_file_sync(
    &self,
    path: String,
    #[napi(ts_arg_type = "WebpEncoderOptions")] options: Option<JsWebpEncoderOptions>,
  ) -> Result<()> {
    let encoder = self.lock()?;
    encoder.write_to_file(
      &path,
      options
        .map(|opts| map_js_webp_encoder_options(&opts))
        .as_ref(),
    )
  }

  #[napi]
  pub fn clear_frames(&mut self) -> Result<()> {
    let mut encoder = self.lock()?;
    encoder.clear_frames();
    Ok(())
  }

  #[napi]
  pub fn set_dimensions(&mut self, width: i32, height: i32) -> Result<()> {
    if width < 1 || height < 1 {
      return Err(NapiError::new(
        Status::InvalidArg,
        "width and height must be positive integers",
      ));
    }
    let mut encoder = self.lock()?;
    encoder.set_dimensions(width as u32, height as u32);
    Ok(())
  }
}

fn map_js_webp_encoder_options(options: &JsWebpEncoderOptions) -> EncoderOptions {
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
      quality: f32::from(options.quality.unwrap_or(1)),
      ..Default::default()
    }),
    ..Default::default()
  }
}

#[derive(thiserror::Error, Debug)]
enum EncoderError {
  #[error("WebP encoder encountered an error: {0}")]
  Encode(#[from] WebpAnimationError),
  #[error("Failed to write to file: {0}")]
  Write(#[from] IoError),
}

impl From<EncoderError> for NapiError {
  fn from(value: EncoderError) -> Self {
    NapiError::from_reason(value.to_string())
  }
}
