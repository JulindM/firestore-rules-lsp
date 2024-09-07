/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: 'firestore_rules',

  rules: {
    source_file: $ => seq(
      token('service'),
      token('cloud.firestore'),
      seq("{", $.match_body, "}"),
    ),

    identifier: _ => /[_a-zA-Z][_a-zA-Z0-9]*/,

    literal: $ => choice(
      $.number,
      $.bool,
      $.null,
      $.string
    ),

    number: _ => /\d+/,

    bool: _ => choice("true", "false"),

    null: _ => "null",

    string: $ => choice(
      seq(
        '"',
        repeat(choice(
          alias($.unescaped_double_string_fragment, $.string_fragment),
          $.escape_sequence,
        )),
        '"',
      ),
      seq(
        '\'',
        repeat(choice(
          alias($.unescaped_single_string_fragment, $.string_fragment),
          $.escape_sequence,
        )),
        '\'',
      ),
    ),

    unescaped_double_string_fragment: _ => token.immediate(prec(1, /[^"\\\r\n]+/)),
    unescaped_single_string_fragment: _ => token.immediate(prec(1, /[^'\\\r\n]+/)),

    escape_sequence: _ => token.immediate(seq(
      '\\',
      choice("\\", "?", "\"", "\'", "`"),
    )),

    function_call: $ => prec.left(
      1,
      seq(
        field("name", $.identifier),
        choice(
          seq("(", $.pathspec, ")"),
          seq("(", optional($.expr_list), ")"),
        )
      )
    ),

    primary: $ => choice(
      $.literal,
      $.identifier,
      seq("(", $.expr, ")"),
      seq("[", optional($.expr_list), "]"),
      $.function_call
    ),

    eval_pathpart: $ => seq("$(", $.expr, ")"),

    pathspec: $ => repeat1(
      seq(
        "/",
        choice(
          $.identifier,
          $.eval_pathpart,
        ),
      ),
    ),

    member: $ => prec.left(
      1,
      choice(
        $.primary,
        seq($.member, ".", $.identifier),
        seq($.member, ".", $.function_call),
        seq($.member, "[", $.expr, "]"),
      ),
    ),

    unary: $ => prec.right(
      2, choice(
        $.member,
        seq(repeat1("!"), $.member),
        seq(repeat1("-"), $.member),
      ),
    ),

    multiplication: $ => prec.left(
      3,
      seq(
        optional(seq($.multiplication, choice("*", "/", "%"))),
        $.unary
      ),
    ),

    addition: $ => prec.left(
      4,
      seq(
        optional(seq($.addition, choice("+", "-"))),
        $.multiplication
      ),
    ),

    relation_op: _ => choice(
      "<", "<=", ">=", ">", "==", "!=", "in"
    ),

    relation: $ => prec.left(
      5,
      seq(
        optional(seq($.relation, $.relation_op)),
        $.addition
      ),
    ),

    conditional_and: $ => prec.left(
      6,
      seq(
        optional(seq($.conditional_and, "&&")),
        $.relation
      ),
    ),

    conditional_or: $ => prec.left(
      7,
      seq(
        optional(seq($.conditional_or, "||")),
        $.conditional_and,
      ),
    ),

    ternary: $ => prec.right(
      8,
      seq("?", $.conditional_or, $.expr),
    ),

    expr: $ => seq(
      $.conditional_or,
      optional($.ternary),
    ),

    expr_list: $ => seq(
      $.expr,
      repeat(seq(",", $.expr))
    ),

    variable_assignment: $ => seq("let", field("variable", $.identifier), "=", $.expr, ";"),

    return: $ => seq("return", $.expr, ";"),

    function_block: $ => seq(
      "{",
      repeat($.variable_assignment),
      $.return,
      "}"
    ),

    function_def: $ => seq(
      'function',
      field('name', $.identifier),
      field('param',
        seq(
          "(",
          optional(seq($.identifier, repeat(seq(",", $.identifier)))),
          ")",
        ),
      ),
      $.function_block,
    ),

    path_segment: $ => choice(
      $.identifier,
      seq("(", $.identifier, ")"),
      alias(seq("{", $.identifier, "}"), "singlePathSegment"),
      alias(seq("{", $.identifier, "=**}"), "multiPathSegment"),
    ),

    path: $ => repeat1(seq("/", $.path_segment)),

    method: $ => choice(
      "read",
      "write",
      "get",
      "list",
      "create",
      "update",
      "delete",
    ),

    rule: $ => seq(
      "allow",
      $.method,
      repeat(seq(",", $.method)),
      optional(seq(": if", $.expr)),
      ";"
    ),

    match_def: $ => seq(
      "match",
      $.path,
      seq("{", $.match_body, "}")
    ),

    match_body: $ => repeat1(choice($.function_def, $.match_def, $.rule)),
  }
})
