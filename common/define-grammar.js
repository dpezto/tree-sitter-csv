/**
 * @file CSV, PSV, & TSV grammar for tree-sitter
 * @author Amaan Qureshi <amaanq12@gmail.com>
 * @license MIT
 */

/* eslint-disable arrow-parens */
/* eslint-disable camelcase */
/* eslint-disable-next-line spaced-comment */
/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

/**
 *
 * @param {string} dialect
 *
 * @param {string} separator
 *
 * @return {GrammarSchema<any>}
 *
 */
module.exports = function defineGrammar(dialect, separator) {
  const decimal_digits = /\d(_?\d)*/;
  const sign = optional(choice('-', '+'));
  const signed_integer = seq(sign, decimal_digits);
  const exponent_part = seq(/[eE]/, signed_integer);
  const time = seq(
    /\d{1,2}:\d{2}(:\d{2}(\.\d+)?)?/,
    optional(seq(/ */, /[APap][Mm]/)),
    optional(choice(/[Zz]/, /[+-]\d{2}:?\d{2}/)),
  );

  return grammar({
    name: dialect,
    extras: _ => [],
    // The internal lexer prefers EOF over an empty text token after a separator.
    externals: $ => [$._empty_field],

    rules: {
      document: $ => seq(
        repeat(seq(choice($.row, $.comment), /\r\n|\r|\n/)),
        optional(choice($.row, $.comment)),
      ),

      comment: $ => seq(token(prec(1, / *#/)), repeat(choice($.date, $._comment_text))),
      _comment_text: _ => token(choice(/[^0-9\r\n]+/, /[0-9]/)),

      row: $ => seq($.field, repeat(seq(separator, $.field))),
      field: $ => choice($.text, $.number, $.float, $.boolean, $.date, alias($._empty_field, $.text)),

      number: _ => token(choice(signed_integer, /0[xX][\da-fA-F](_?[\da-fA-F])*/)),
      float: _ => token(choice(
        seq(signed_integer, exponent_part),
        seq(sign, '.', decimal_digits, optional(exponent_part)),
        seq(signed_integer, '.', optional(decimal_digits), optional(exponent_part)),
      )),
      boolean: _ => choice('true', 'false'),

      // Recognize date shapes; calendar and timezone validation belongs to consumers.
      date: _ => token(choice(
        seq(
          choice(/\d{1,2}[-./]\d{1,2}[-./]\d{2,4}/, /\d{2,4}[-./]\d{1,2}[-./]\d{1,2}/),
          optional(seq(choice(/[Tt]/, / +/), time)),
        ),
        seq(optional('T'), time),
      )),

      text: _ => token(choice(
        new RegExp(`[^${separator}\\r\\n"]*`),
        seq('"', repeat(choice(/[^"]/, '""')), '"'),
      )),
    },
  });
};
