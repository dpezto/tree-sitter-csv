#include "tree_sitter/parser.h"

#define DEFINE_EMPTY_FIELD_SCANNER(name) \
  void *tree_sitter_##name##_external_scanner_create(void) { return NULL; } \
  void tree_sitter_##name##_external_scanner_destroy(void *payload) { (void)payload; } \
  unsigned tree_sitter_##name##_external_scanner_serialize(void *payload, char *buffer) { \
    (void)payload; \
    (void)buffer; \
    return 0; \
  } \
  void tree_sitter_##name##_external_scanner_deserialize(void *payload, const char *buffer, unsigned length) { \
    (void)payload; \
    (void)buffer; \
    (void)length; \
  } \
  bool tree_sitter_##name##_external_scanner_scan(void *payload, TSLexer *lexer, const bool *valid_symbols) { \
    (void)payload; \
    if (!valid_symbols[0] || !lexer->eof(lexer) || lexer->get_column(lexer) == 0) return false; \
    lexer->mark_end(lexer); \
    lexer->result_symbol = 0; \
    return true; \
  }
