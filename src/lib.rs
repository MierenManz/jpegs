use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[allow(clippy::needless_return)]
#[wasm_bindgen]
pub fn encode(
  image: &[u8],
  width: u16,
  height: u16,
  colortype: u32,
  quality: u8,
) -> Result<Vec<u8>, JsValue> {
  let mut dest: Vec<u8> = Vec::new();
  let encoder = jpeg_encoder::Encoder::new(&mut dest, quality);

  let color_enum = match colortype {
    0 => jpeg_encoder::ColorType::Bgr,
    1 => jpeg_encoder::ColorType::Bgra,
    2 => jpeg_encoder::ColorType::Cmyk,
    3 => jpeg_encoder::ColorType::CmykAsYcck,
    4 => jpeg_encoder::ColorType::Luma,
    5 => jpeg_encoder::ColorType::Rgb,
    6 => jpeg_encoder::ColorType::Rgba,
    7 => jpeg_encoder::ColorType::Ycbcr,
    8 => jpeg_encoder::ColorType::Ycck,
    _ => return Err(JsValue::from_str("Invalid colortype")),
  };

  if let Err(err) = encoder.encode(image, width, height, color_enum) {
    return Err(JsValue::from_str(&format!("{}", err)));
  }

  return Ok(dest);
}

#[wasm_bindgen]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodeResult {
  image: Vec<u8>,
  width: u32,
  height: u32,
  pixel_format: String,
}

#[allow(clippy::needless_return)]
#[wasm_bindgen]
pub fn decode(image: &[u8]) -> Result<DecodeResult, JsValue> {
  let mut decoder = jpeg_decoder::Decoder::new(image);

  let img = match decoder.decode() {
    Ok(vec) => vec,
    Err(err) => return Err(JsValue::from_str(&format!("{}", err))),
  };

  let metadata = match decoder.info() {
    Some(meta) => meta,
    None => return Err(JsValue::from_str("Could not retrieve metadata")),
  };

  let pixel_format = match metadata.pixel_format {
    jpeg_decoder::PixelFormat::CMYK32 => "CMYK32".to_string(),
    jpeg_decoder::PixelFormat::L8 => "L8".to_string(),
    jpeg_decoder::PixelFormat::RGB24 => "RGB24".to_string(),
  };

  let result = DecodeResult {
    image: img,
    width: metadata.width as u32,
    height: metadata.height as u32,
    pixel_format,
  };

  return Ok(result);
}
