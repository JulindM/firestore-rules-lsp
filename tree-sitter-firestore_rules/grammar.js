/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: 'firestore_rules',

  extras: $ => [
    $.comment,
    /\s/,
  ],

  rules: {
    source_file: $ => seq(
      token('service'),
      token('cloud.firestore'),
      seq("{", $.match_body, "}"),
    ),

    comment: $ => /\/\/.*\r?\n/,

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

    variable: $ => $.identifier,

    primary: $ => choice(
      $.literal,
      $.variable,
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

    indexing: $ => prec.left(
      8,
      seq(
        field("object", choice($.member, $.primary)),
        field("index", seq("[", $.expr, "]"))
      ),
    ),

    member: $ => prec.left(
      8,
      seq(
        field("object", seq(choice($.member, $.primary), ".")),
        field("field", choice($.variable, $.function_call)),
      ),
    ),

    unary: $ => prec.right(
      7,
      choice(
        seq(repeat1("!"), $.expr),
        seq(repeat1("-"), $.expr),
      ),
    ),

    multiplication: $ => prec.left(
      6,
      seq($.expr, choice("*", "/", "%"), $.expr)
    ),

    addition: $ => prec.left(
      5,
      seq($.expr, choice("+", "-"), $.expr)
    ),

    relation_op: _ => choice(
      "<", "<=", ">=", ">", "==", "!=", "in"
    ),

    relation: $ => prec.left(
      4,
      seq($.expr, $.relation_op, $.expr),
    ),

    conditional_and: $ => prec.left(
      3,
      seq($.expr, "&&", $.expr),
    ),

    conditional_or: $ => prec.left(
      2,
      seq($.expr, "||", $.expr),
    ),

    ternary: $ => prec.right(
      1,
      seq($.expr, "?", $.expr, ":", $.expr),
    ),

    expr: $ => choice(
      $.ternary,
      $.conditional_or,
      $.conditional_and,
      $.relation,
      $.addition,
      $.multiplication,
      $.unary,
      $.member,
      $.indexing,
      $.primary,
    ),

    expr_list: $ => seq(
      $.expr,
      repeat(seq(",", $.expr))
    ),

    return: $ => seq("return", $.expr, ";"),

    variable_assignment: $ => seq("let", $.variable, "=", $.expr, ";"),

    function_block: $ => seq(
      repeat($.variable_assignment),
      $.return,
    ),

    function_def: $ => seq(
      'function',
      field('name', $.identifier),
      "(",
      field(
        'param',
        optional(
          seq($.identifier, repeat(seq(",", $.identifier)))
        ),
      ),
      ")", "{",
      $.function_block,
      "}"
    ),

    path_segment: $ => choice(
      $.identifier,
      alias(seq("(", field("path", $.identifier), ")"), "globalPathSegment"),
      alias(seq("{", field("path", $.identifier), "}"), "singlePathSegment"),
      alias(seq("{", field("path", $.identifier), "=**}"), "multiPathSegment"),
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
      field(
        "rule",
        seq(
          $.method,
          repeat(seq(",", $.method)),
        ),
      ),
      field(
        "condition",
        optional(seq(": if", $.expr))
      ),
      ";"
    ),

    match_def: $ => seq(
      "match",
      field("path", $.path),
      seq("{", $.match_body, "}")
    ),

    match_body: $ => repeat1(choice($.function_def, $.match_def, $.rule)),
  }
})
