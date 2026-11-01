from unittest import TestCase

import tree_sitter
import tree_sitter_csv


class TestLanguage(TestCase):
    def test_can_parse_typed_fields(self):
        language = tree_sitter.Language(tree_sitter_csv.language())
        parser = tree_sitter.Parser(language)
        tree = parser.parse(b"-12,+.5,false,2026-11-01,")
        self.assertFalse(tree.root_node.has_error)
        fields = tree.root_node.named_children[0].named_children
        self.assertEqual(
            [field.named_children[0].type for field in fields],
            ["number", "float", "boolean", "date", "text"],
        )
