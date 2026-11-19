const assert = require('node:assert/strict');
const {test} = require('node:test');
const Parser = require('tree-sitter');
const grammars = require('.');

for (const [name, separator] of [['csv', ','], ['psv', '|'], ['tsv', '\t']]) {
  test(`${name} types and final empty field`, () => {
    const parser = new Parser();
    parser.setLanguage(grammars[name]);
    const input = ['-12', '+.5', 'false', '2026-11-01', ''].join(separator);
    const tree = parser.parse(input);
    assert.equal(tree.rootNode.hasError, false);
    const types = tree.rootNode.firstNamedChild.namedChildren.map(field => field.firstNamedChild.type);
    assert.deepEqual(types, ['number', 'float', 'boolean', 'date', 'text']);
  });

  test(`${name} dates in comments preserve row boundaries`, () => {
    const parser = new Parser();
    parser.setLanguage(grammars[name]);
    for (const newline of ['\n', '\r\n', '\r']) {
      const input = [
        '# Created 2026-11-01 at 12:34 PM',
        ['ABC#123', '"# 2026-11-02"'].join(separator),
        '  # Next 2026-11-03T01:02:03Z',
      ].join(newline);
      const tree = parser.parse(input);
      assert.equal(tree.rootNode.hasError, false);
      const nodes = tree.rootNode.namedChildren;
      assert.deepEqual(nodes.map(node => node.type), ['comment', 'row', 'comment']);
      assert.deepEqual(nodes[0].namedChildren.map(node => node.text), ['2026-11-01', '12:34 PM']);
      assert.equal(nodes[1].namedChildCount, 2);
      assert.equal(nodes[2].firstNamedChild.text, '2026-11-03T01:02:03Z');
    }
  });
}
