# caido-convert

An encoding library for web security applications. 

## Install
`npm install caido-convert`

## Usage
```
import { Base64Encode } from 'caido-convert'

let utf8Encode = new TextEncoder();
let utf8Decode = new TextDecoder();

let encoder = new Base64Decode();
let base64 = encoder.apply(utf8Encode.encode("caido")); 

console.log(utf8Decode.decode(base64)) // Logs "Y2FpZG8="

```

### Notes
This library takes a buffer of bytes in and output a buffer of bytes. This makes it possible to chain operation. You can get back a string a `let decoder = new TextDecoder()`.
