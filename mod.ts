import init, {
  decode as wasmDecode,
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

export function decode(u8: Uint8Array): Uint8Array {
  return wasmDecode(u8);
}

export function encode(
  image: Uint8Array,
  width: number,
  height: number,
  colortype: ValueOf<typeof ColorType>,
  quality: number,
) {
  if (quality > 0 && quality < 101) {
    return wasmEncode(image, width, height, colortype, quality);
  }
  throw "Quality should be between 1 and 100";
}

const r = decode(await Deno.readFile("test.jpg"));
const d = encode(r, 1460, 730, ColorType.rgb, 100);
await Deno.writeFile("out.jpg", d);
