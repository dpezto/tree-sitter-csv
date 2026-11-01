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
}
