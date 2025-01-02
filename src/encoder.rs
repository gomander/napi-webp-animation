use bindgen_prelude::Buffer;
use napi::*;
use webp_animation::*;

struct WebpEncoderFrame {
    frame_data: Vec<u8>,
    duration: u16,
}

pub struct WebpEncoder {
    width: u32,
    height: u32,
    output_path: String,
    options: EncoderOptions,
    frames: Vec<WebpEncoderFrame>,
}

impl WebpEncoder {
    pub fn new(
        width: u32,
        height: u32,
        output_path: String,
        options: Option<EncoderOptions>,
    ) -> Self {
        WebpEncoder {
            width,
            height,
            output_path,
            options: options.unwrap_or_else(EncoderOptions::default),
            frames: Vec::new(),
        }
    }

    pub fn add_frame(&mut self, frame_data: Vec<u8>, duration: u16) -> Result<()> {
        self.frames.push(WebpEncoderFrame {
            frame_data,
            duration,
        });
        Ok(())
    }

    pub fn finish(&self, duration: u16) -> Result<()> {
        let mut encoder =
            Encoder::new_with_options((self.width, self.height), self.options.to_owned()).unwrap();
        let mut timestamp: i32 = 0;
        for frame in &self.frames {
            encoder.add_frame(&frame.frame_data, timestamp).unwrap();
            timestamp += frame.duration as i32;
        }
        let webp_data = encoder.finalize(duration as i32).unwrap();
        std::fs::write(&self.output_path, webp_data).unwrap();
        Ok(())
    }
}

#[napi(object)]
pub struct JsWebpEncoderOptions {
    pub lossless: Option<bool>,
    pub quality: Option<i32>,
    pub method: Option<u32>,
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
        output_path: String,
        options: Option<JsWebpEncoderOptions>,
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
                quality: if options.quality.is_some() {
                    options.quality.unwrap_or(1) as f32
                } else {
                    Default::default()
                },
                method: if options.method.is_some() {
                    options.method.unwrap_or(4) as usize
                } else {
                    Default::default()
                },
            }),
            ..Default::default()
        });
        JsWebpEncoder {
            encoder: WebpEncoder::new(width, height, output_path, options),
        }
    }

    #[napi]
    pub fn add_frame(&mut self, frame_data: Buffer, duration: u16) -> Result<()> {
        self.encoder.add_frame(frame_data.into(), duration)
    }

    #[napi]
    pub fn finish(&self, duration: u16) -> Result<()> {
        self.encoder.finish(duration)
    }
}

#[derive(thiserror::Error, Debug)]
enum EncoderError {
    #[error("The provided argument was too large")]
    ArgumentTooLarge(#[from] std::num::TryFromIntError),
    #[error("WebP encoder encountered an error")]
    EncoderError(#[from] webp_animation::Error),
}

impl From<EncoderError> for napi::Error {
    fn from(value: EncoderError) -> Self {
        napi::Error::from_reason(value.to_string())
    }
}
