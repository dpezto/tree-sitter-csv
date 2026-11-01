package tree_sitter_csv

// #cgo CFLAGS: -std=c11 -fPIC -I${SRCDIR}/../../csv/src
// #include "../../csv/src/parser.c"
// #if __has_include("../../csv/src/scanner.c")
// #include "../../csv/src/scanner.c"
// #endif
import "C"

import "unsafe"

// Get the tree-sitter Language for this grammar.
func Language() unsafe.Pointer {
	return unsafe.Pointer(C.tree_sitter_csv())
}
