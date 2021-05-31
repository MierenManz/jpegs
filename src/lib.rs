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
