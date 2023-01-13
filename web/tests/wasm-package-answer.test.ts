import { UrlEncode } from "wasm-convert";

describe("wasm-convert", () => {
  it("Url encode string", () => {
    let encoder = new UrlEncode({ special_chars: false });
    let utf8Encode = new TextEncoder();
    let actual = encoder.apply(utf8Encode.encode("a@ test"));
    let expected = utf8Encode.encode("a%40%20test");

    expect(equal(actual, expected));
  });
});

const equal = (buf1: ArrayBufferLike, buf2: ArrayBufferLike) => {
  if (buf1.byteLength != buf2.byteLength) return false;
  var dv1 = new Int8Array(buf1);
  var dv2 = new Int8Array(buf2);
  for (var i = 0; i != buf1.byteLength; i++) {
    if (dv1[i] != dv2[i]) return false;
  }
  return true;
};

export {};
