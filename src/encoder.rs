use bindgen_prelude::{AsyncTask, Buffer};
use napi::*;
use webp_animation::{AnimParams, Encoder, EncoderOptions, EncodingConfig, LossyEncodingConfig};

struct WebpEncoderFrame {
    frame_data: Vec<u8>,
    duration: Option<f64>,
}

impl Clone for WebpEncoderFrame {
    fn clone(&self) -> Self {
        WebpEncoderFrame {
            frame_data: self.frame_data.clone(),
            duration: self.duration,
        }
    }
}

pub struct WebpEncoder {
    width: u32,
    height: u32,
    options: EncoderOptions,
    frame_rate: u16,
    frames: Vec<WebpEncoderFrame>,
}

impl WebpEncoder {
    pub fn new(width: u32, height: u32, options: Option<EncoderOptions>) -> Self {
        WebpEncoder {
            width,
            height,
            options: options.unwrap_or_else(EncoderOptions::default),
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

    pub fn get_buffer(&self) -> Result<Vec<u8>> {
        let mut encoder =
            Encoder::new_with_options((self.width, self.height), self.options.to_owned())
                .map_err(EncoderError::EncoderError)?;
        let mut timestamp: i32 = 0;
        for frame in &self.frames {
            encoder
                .add_frame(&*frame.frame_data, timestamp)
                .map_err(EncoderError::EncoderError)?;
            timestamp += frame.duration.unwrap_or(1000. / (self.frame_rate as f64)) as i32;
        }
        Ok(encoder
            .finalize(timestamp)
            .map_err(EncoderError::EncoderError)?
            .to_vec())
    }

    pub fn write_to_file(&self, path: String) -> Result<()> {
        std::fs::write(
            match path.ends_with(".webp") {
                true => path,
                false => format!("{path}.webp"),
            },
            self.get_buffer()?,
        )
        .map_err(EncoderError::WriteError)?;
        Ok(())
    }

    pub fn clear_frames(&mut self) {
        self.frames.clear();
    }

    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn set_options(&mut self, options: EncoderOptions) {
        self.options = options;
    }

    pub fn clone(&self) -> Self {
        WebpEncoder {
            width: self.width,
            height: self.height,
            options: self.options.clone(),
            frame_rate: self.frame_rate,
            frames: self.frames.clone(),
        }
    }
}

#[napi(object)]
pub struct JsWebpEncoderOptions {
    pub lossless: Option<bool>,
    pub quality: Option<u8>,
    pub method: Option<u8>,
    pub loop_count: Option<i32>,
}

pub struct AsyncGetBuffer {
    encoder: WebpEncoder,
}

#[napi]
impl Task for AsyncGetBuffer {
    type Output = Vec<u8>;
    type JsValue = Buffer;

    fn compute(&mut self) -> Result<Self::Output> {
        self.encoder.get_buffer()
    }

    fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output.into())
    }
}

pub struct AsyncWriteToFile {
    encoder: WebpEncoder,
    path: String,
}

#[napi]
impl Task for AsyncWriteToFile {
    type Output = ();
    type JsValue = JsUndefined;

    fn compute(&mut self) -> Result<Self::Output> {
        self.encoder.write_to_file(self.path.clone())
    }

    fn resolve(&mut self, env: Env, _: Self::Output) -> Result<Self::JsValue> {
        Ok(env.get_undefined()?)
    }
}

#[napi(js_name = "WebpEncoder")]
pub struct JsWebpEncoder {
    encoder: WebpEncoder,
}

#[napi]
impl JsWebpEncoder {
    #[napi(constructor)]
    pub fn new(
        width: u32,
        height: u32,
        #[napi(ts_arg_type = "JsWebpEncoderOptions")] options: Option<JsWebpEncoderOptions>,
    ) -> Self {
        let options = options.map(|options| EncoderOptions {
            anim_params: AnimParams {
                loop_count: options.loop_count.unwrap_or(0),
            },
            encoding_config: Some(EncodingConfig {
                encoding_type: if options.lossless.unwrap_or(true) {
                    webp_animation::EncodingType::Lossless
                } else {
                    webp_animation::EncodingType::Lossy(LossyEncodingConfig::default())
                },
                quality: options.quality.unwrap_or(1) as f32,
                method: options.method.unwrap_or(4) as usize,
            }),
            ..Default::default()
        });
        JsWebpEncoder {
            encoder: WebpEncoder::new(width, height, options),
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
    pub fn get_buffer(&self) -> AsyncTask<AsyncGetBuffer> {
        AsyncTask::new(AsyncGetBuffer {
            encoder: self.encoder.clone(),
        })
    }

    #[napi]
    pub fn get_buffer_sync(&self) -> Result<Buffer> {
        Ok(self.encoder.get_buffer()?.into())
    }

    #[napi]
    pub fn write_to_file(&self, path: String) -> AsyncTask<AsyncWriteToFile> {
        AsyncTask::new(AsyncWriteToFile {
            encoder: self.encoder.clone(),
            path,
        })
    }

    #[napi]
    pub fn write_to_file_sync(&self, path: String) -> Result<()> {
        self.encoder.write_to_file(path)
    }

    #[napi]
    pub fn clear_frames(&mut self) {
        self.encoder.clear_frames();
    }

    #[napi]
    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.encoder.set_dimensions(width, height);
    }

    #[napi]
    pub fn set_options(&mut self, options: JsWebpEncoderOptions) {
        self.encoder.set_options(EncoderOptions {
            anim_params: AnimParams {
                loop_count: options.loop_count.unwrap_or(0),
            },
            encoding_config: Some(EncodingConfig {
                encoding_type: if options.lossless.unwrap_or(true) {
                    webp_animation::EncodingType::Lossless
                } else {
                    webp_animation::EncodingType::Lossy(LossyEncodingConfig::default())
                },
                quality: options.quality.unwrap_or(1) as f32,
                method: options.method.unwrap_or(4) as usize,
            }),
            ..Default::default()
        });
    }
}

#[derive(thiserror::Error, Debug)]
enum EncoderError {
    #[error("WebP encoder encountered an error: {0}")]
    EncoderError(#[from] webp_animation::Error),
    #[error("Failed to write to file: {0}")]
    WriteError(#[from] std::io::Error),
}

impl From<EncoderError> for napi::Error {
    fn from(value: EncoderError) -> Self {
        napi::Error::from_reason(value.to_string())
    }
}
