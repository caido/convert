import { UrlEncode } from "wasm-convert";

let encoder = new UrlEncode({ special_chars: false });
let utf8Encode = new TextEncoder();
let to_encode = "a@ test";
let encoded = encoder.apply(utf8Encode.encode(to_encode));
console.log(`${to_encode} url encoded is ${encoded}`);
