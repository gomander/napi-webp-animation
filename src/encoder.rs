use bindgen_prelude::Buffer;
use napi::*;
use webp_animation::*;

struct WebpEncoderFrame {
    frame_data: Vec<u8>,
    duration: Option<f64>,
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
                false => format!("{}.webp", path),
            },
            self.get_buffer()?,
        )
        .map_err(EncoderError::WriteError)?;
        Ok(())
    }
}

#[napi(object)]
pub struct JsWebpEncoderOptions {
    pub lossless: Option<bool>,
    pub quality: Option<u8>,
    pub method: Option<u8>,
    pub loop_count: Option<i32>,
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
                loop_count: options.loop_count.unwrap_or_default(),
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
    pub fn get_buffer(&self) -> Result<Buffer> {
        match self.encoder.get_buffer() {
            Ok(data) => Ok(data.into()),
            Err(err) => Err(err),
        }
    }

    #[napi]
    pub fn write_to_file(&self, path: String) -> Result<()> {
        self.encoder.write_to_file(path)
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
