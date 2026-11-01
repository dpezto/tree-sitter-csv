{
  "targets": [
    {
      "target_name": "tree_sitter_csv_binding",
      "include_dirs": [
        "<!@(node -p \"require('node-addon-api').include\")",
        "csv/src"
      ],
      "dependencies": ["<!(node -p \"require('node-addon-api').gyp\")"],
      "defines": ["NAPI_VERSION=8", "NAPI_DISABLE_CPP_EXCEPTIONS"],
      "sources": [
        "bindings/node/binding.cc",
        "csv/src/parser.c",
        "csv/src/scanner.c",
        "psv/src/parser.c",
        "psv/src/scanner.c",
        "tsv/src/parser.c",
        "tsv/src/scanner.c",
      ],
      "cflags_c": [
        "-std=c99",
      ]
    }
  ]
}
