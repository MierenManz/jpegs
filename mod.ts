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

interface DecodeResult {
  image: Uint8Array;
  width: number;
  height: number;
  pixelFormat: string;
}

export function decode(u8: Uint8Array): DecodeResult {
  const r = wasmDecode(u8);
  console.log(r);
  return {
    image: r.image,
    width: r.width,
    height: r.height,
    pixelFormat: r.pixelFormat,
  };
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

const r = decode(await Deno.readFile("test.jpg"));
console.log(r);
const _d = encode(r.image, 1460, 730, ColorType.rgb, 100);
// await Deno.writeFile("out.jpg", d);
