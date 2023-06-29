import base64

if __name__ == "__main__":
    print("Embedding WASM into js")
    # assumption: current working directory (relative to this script) is ../
    # assumption: release wasm binary at ./pkg/usdpl_bg.wasm
    with open("./pkg/fantastic_wasm_bg.wasm", mode="rb") as infile:
        with open("./pkg/fantastic_wasm.js", mode="ab") as outfile:
            outfile.write("\n\n// USDPL customization\nconst encoded = \"".encode())
            encoded = base64.b64encode(infile.read())
            outfile.write(encoded)
            outfile.write("\";\n\n".encode())
            outfile.write(
"""function asciiToBinary(str) {
  if (typeof atob === 'function') {
    return atob(str)
  } else {
    return new Buffer(str, 'base64').toString('binary');
  }
}

function decode() {
  var binaryString =  asciiToBinary(encoded);
  var bytes = new Uint8Array(binaryString.length);
  for (var i = 0; i < binaryString.length; i++) {
    bytes[i] = binaryString.charCodeAt(i);
  }
  return (async function() {
    return new Response(bytes.buffer, {
        status: 200,
        statusText: 'OK',
        headers: {
            'Content-Type': 'application/wasm'
        }
    });
  })();
}

export function init_embedded() {
    return __wbg_init(decode())
}
""".encode())
    with open("./pkg/fantastic_wasm.d.ts", "a") as outfile:
        outfile.write("\n\n// USDPL customization\nexport function init_embedded();\n")
    print("Done: Embedded WASM into js")
