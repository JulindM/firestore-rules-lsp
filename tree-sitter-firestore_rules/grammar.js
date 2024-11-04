/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "firestore_rules",

  extras: ($) => [$.comment, /\s/],

  rules: {
    source_file: ($) =>
      seq(optional($.rules_version_def), $.service_name, $.match_body),

    rules_version_def: ($) => seq("rules_version", "=", $.string, ";"),

    service_name: ($) => seq("service", "cloud.firestore"),

    comment: ($) => /\/\/.*\r?\n/,

    identifier: (_) => /[_a-zA-Z][_a-zA-Z0-9]*/,

    literal: ($) => choice($.number, "true", "false", $.null, $.string),

    number: (_) => /\d+(\.\d+)?/,

    null: (_) => "null",

    string: ($) =>
      choice(
        seq(
          '"',
          repeat(
            choice(
              alias($.unescaped_double_string_fragment, $.string_fragment),
              $.escape_sequence
            )
          ),
          '"'
        ),
        seq(
          "'",
          repeat(
            choice(
              alias($.unescaped_single_string_fragment, $.string_fragment),
              $.escape_sequence
            )
          ),
          "'"
        )
      ),

    unescaped_double_string_fragment: (_) =>
      token.immediate(prec(1, /[^"\\\r\n]+/)),
    unescaped_single_string_fragment: (_) =>
      token.immediate(prec(1, /[^'\\\r\n]+/)),

    escape_sequence: (_) =>
      token.immediate(seq("\\", choice("\\", "?", '"', "'", "`"))),

    path_segment: ($) => choice($.identifier, seq("$(", $.expr, ")")),

    path: ($) => repeat1(seq("/", $.path_segment)),

    function_argument: ($) => choice($.path, $.expr),

    function_call: ($) =>
      prec.left(
        1,
        seq(
          alias($.identifier, "function_calling_name"),
          token.immediate("("),
          optional(
            seq($.function_argument, repeat(seq(",", $.function_argument)))
          ),
          ")"
        )
      ),

    variable: ($) => $.identifier,

    expr_group: ($) => seq("(", $.expr, ")"),

    list: ($) =>
      seq(
        "[",
        optional(
          seq($.expr, repeat(seq(",", $.expr)))
        ),
        "]"
      ),

    primary: ($) =>
      choice($.literal, $.variable, $.expr_group, $.list, $.function_call),

    indexing: ($) =>
      prec.left(8, seq(choice($.member, $.primary), "[", $.expr, "]")),

    member: ($) =>
      prec.left(
        8,
        seq(
          choice($.member, $.primary),
          ".",
          choice($.variable, $.function_call)
        )
      ),

    unary: ($) =>
      prec.right(
        7,
        choice(seq(repeat1("!"), $.expr), seq(repeat1("-"), $.expr))
      ),

    multiplication: ($) =>
      prec.left(6, seq($.expr, choice("*", "/", "%"), $.expr)),

    addition: ($) => prec.left(5, seq($.expr, choice("+", "-"), $.expr)),

    relation: ($) =>
      prec.left(
        4,
        seq($.expr, choice("<", "<=", ">=", ">", "==", "!=", "in"), $.expr)
      ),

    conditional_and: ($) => prec.left(3, seq($.expr, "&&", $.expr)),

    conditional_or: ($) => prec.left(2, seq($.expr, "||", $.expr)),

    ternary: ($) => prec.right(1, seq($.expr, "?", $.expr, ":", $.expr)),

    expr: ($) =>
      choice(
        $.ternary,
        $.conditional_or,
        $.conditional_and,
        $.relation,
        $.addition,
        $.multiplication,
        $.unary,
        $.member,
        $.indexing,
        $.primary
      ),

    variable_def: ($) => seq("let", $.variable, "=", $.expr, ";"),

    fun_return: ($) => seq("return", $.expr, ";"),

    function_body: ($) => seq("{", repeat($.variable_def), $.fun_return, "}"),

    function_def: ($) =>
      seq(
        "function",
        alias($.identifier, "function_name"),
        "(",
        alias(
          optional(seq($.identifier, repeat(seq(",", $.identifier)))),
          "param"
        ),
        ")",
        $.function_body,
      ),

    collection_path_seg: (_) => /\/[_a-zA-Z][_a-zA-Z0-9]*/,
    single_path_seg: (_) => /\/\{[_a-zA-Z][_a-zA-Z0-9]*\}/,
    multi_path_seg: (_) => /\/\{[_a-zA-Z][_a-zA-Z0-9]*=\*\*\}/,

    match_path: ($) =>
      repeat1(
        choice($.collection_path_seg, $.single_path_seg, $.multi_path_seg)
      ),

    method: ($) =>
      choice("read", "write", "get", "list", "create", "update", "delete"),

    rule_def: ($) =>
      seq(
        "allow",
        seq($.method, repeat(seq(",", $.method))),
        optional(seq(":", "if", $.expr)),
        ";"
      ),

    match_def: ($) => seq("match", $.match_path, $.match_body),

    match_body: ($) =>
      seq("{", repeat(choice($.function_def, $.match_def, $.rule_def)), "}"),
  },
});
