import {
  UrlEncode,
  UrlDecode,
  Base64Encode,
  Base64Decode,
  HexEncode,
  HexDecode,
  HtmlDecode,
  HtmlEncode,
  Md5Hash,
  Sha1Hash,
  Sha2Hash,
} from "../../wasm-package";

describe("UrlEncoding", () => {
  it("Url encode string", () => {
    let encoder = new UrlEncode({ non_ascii: true, charset: "c" });
    let utf8Encode = new TextEncoder();
    let actual = encoder.apply(utf8Encode.encode("caido @éé🥖"));
    let expected = utf8Encode.encode("%63aido @%C3%A9%C3%A9%F0%9F%A5%96");

    expect(equal(actual, expected)).toBeTruthy();
  });

  it("Url decode string", () => {
    let encoder = new UrlDecode();
    let utf8Encode = new TextEncoder();
    let actual = encoder.apply(
      utf8Encode.encode("caido @%C3%A9%C3%A9%F0%9F%A5%96")
    );
    let expected = utf8Encode.encode("caido @éé🥖");

    expect(equal(actual, expected)).toBeTruthy();
  });
});

describe("Base64 encoding", () => {
  it("Base64 encode bytes", () => {
    let encoder = new Base64Encode();
    let utf8Encode = new TextEncoder();
    let actual = encoder.apply(utf8Encode.encode("caido"));
    let expected = utf8Encode.encode("Y2FpZG8=");

    expect(equal(actual, expected)).toBeTruthy();
  });

  it("Base64 decode bytes", () => {
    let encoder = new Base64Decode();
    let utf8Encode = new TextEncoder();
    let actual = encoder.apply(utf8Encode.encode("Y2FpZG8="));
    let expected = utf8Encode.encode("caido");

    expect(equal(actual, expected)).toBeTruthy();
  });
});

describe("Hex encoding", () => {
  it("Hex encode bytes", () => {
    let encoder = new HexEncode({
      format: "Upper",
      delimiter: "",
      bytes_per_line: 10,
    });
    let utf8Encode = new TextEncoder();
    let actual = encoder.apply(utf8Encode.encode("caido"));
    let expected = utf8Encode.encode("636169646F");

    expect(equal(actual, expected)).toBeTruthy();
  });

  it("Hex decode bytes", () => {
    let encoder = new HexDecode({
      delimiter: "0x",
    });
    let utf8Encode = new TextEncoder();
    let actual = encoder.apply(utf8Encode.encode("0x630x610x690x640x6f"));
    let expected = utf8Encode.encode("caido");

    expect(equal(actual, expected)).toBeTruthy();
  });
});

describe("Html encoding", () => {
  it("Html encode bytes", () => {
    let encoder = new HtmlEncode();
    let utf8Encode = new TextEncoder();
    let actual = encoder.apply(
      utf8Encode.encode('\\&<script>alert(1)</script>a"')
    );
    let expected = utf8Encode.encode(
      "&#39;&amp;&lt;script&gt;alert(1)&lt;/script&gt;a&quot;"
    );

    expect(equal(actual, expected)).toBeTruthy();
  });

  it("Html decode bytes", () => {
    let decoder = new HtmlDecode();
    let utf8Encode = new TextEncoder();
    let actual = decoder.apply(
      utf8Encode.encode(
        "&#39;&amp;&lt;script&gt;alert(1)&lt;/script&gt;a&quot;"
      )
    );
    let expected = utf8Encode.encode('\\&<script>alert(1)</script>a"');

    expect(equal(actual, expected)).toBeTruthy();
  });
});

describe("Hash", () => {
  it("Hash bytes with md5", () => {
    let encoder = new Md5Hash();
    let utf8Encode = new TextEncoder();
    let actual = encoder.apply(utf8Encode.encode("caido"));

    let hexEncoder = new HexDecode({});
    let expected = hexEncoder.apply(
      utf8Encode.encode("7542bf4fd4500c58ac741ae2e05a1521")
    );

    expect(equal(actual, expected)).toBeTruthy();
  });

  it("Hash bytes with sha1", () => {
    let encoder = new Sha1Hash();
    let utf8Encode = new TextEncoder();
    let actual = encoder.apply(utf8Encode.encode("caido"));

    let hexEncoder = new HexDecode({});
    let expected = hexEncoder.apply(
      utf8Encode.encode("ba91f49c2d9a785c0fe2af32afff42a51f410596")
    );

    expect(equal(actual, expected)).toBeTruthy();
  });

  it("Hash bytes with sha2", () => {
    let encoder = new Sha2Hash({ version: "Sha256" });
    let utf8Encode = new TextEncoder();
    let actual = encoder.apply(utf8Encode.encode("caido"));

    let hexEncoder = new HexDecode({});
    let expected = hexEncoder.apply(
      utf8Encode.encode(
        "d3492c16a592ec2356127c3eefff6bae98321f9932e625c1ce4ec5fedc5301b2"
      )
    );

    expect(equal(actual, expected)).toBeTruthy();
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

export { };
