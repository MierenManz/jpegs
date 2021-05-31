import init, {
  encode as wasmEncode,
  source,
} from "./wasm.js";

await init(source);

type ValueOf<T> = T[keyof T];

enum ColorType {
  bgr,
  bgra,
  cmyk,
  cmykasycck,
  luma,
  rgb,
  rgba,
  ycbcr,
  ycck,
}


export function encode(
  image: Uint8Array,
  width: number,
  height: number,
  colortype: ValueOf<typeof ColorType>,
  quality: number,
): Uint8Array {
  if (quality > 0 && quality < 101) {
    return wasmEncode(image, width, height, colortype, quality);
  }
  throw "Quality should be between 1 and 100";
}
