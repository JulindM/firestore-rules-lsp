/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "firestore_rules",

  extras: ($) => [$.comment, /\s/],

  rules: {
    source_file: ($) =>
      seq(optional($.rules_version_def), $.service_name, $.service_body),

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

    function_calling_name: ($) => $.identifier,

    function_call: ($) =>
      prec.left(
        1,
        seq(
          $.function_calling_name,
          token.immediate("("),
          optional(
            seq($.function_argument, repeat(seq(",", $.function_argument)))
          ),
          ")"
        )
      ),

    variable: ($) => $.identifier,

    expr_group: ($) => seq("(", $.expr, ")"),

    list: ($) => seq("[", optional(seq($.expr, repeat(seq(",", $.expr)))), "]"),

    map_entry: ($) =>
      seq(alias($.expr, "map_key"), ":", alias($.expr, "map_value")),

    map: ($) =>
      seq("{", optional(seq($.map_entry, repeat(seq(",", $.map_entry)))), "}"),

    primary: ($) =>
      choice(
        $.literal,
        $.variable,
        $.function_call,
        $.expr_group,
        $.list,
        $.map
      ),

    range: (_) => seq(/\d+/, ":", "/d+/"),

    indexing: ($) =>
      prec.left(
        8,
        seq(
          choice($.variable, $.expr_group, $.function_call, $.list, $.map),
          "[",
          choice($.expr, $.range),
          "]"
        )
      ),

    field_indexing: ($) =>
      prec.left(
        8,
        seq(
          choice(choice($.variable, $.function_call), $.list),
          "[",
          choice($.expr, $.range),
          "]"
        )
      ),

    member_object: ($) => $.primary,

    member_field: ($) =>
      seq(".", choice($.variable, $.function_call, $.field_indexing, $.member)),

    member: ($) => prec.right(8, seq($.member_object, $.member_field)),

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

    param_list: ($) => seq($.identifier, repeat(seq(",", $.identifier))),

    function_def: ($) =>
      seq(
        "function",
        alias($.identifier, "function_name"),
        "(",
        optional($.param_list),
        ")",
        $.function_body
      ),

    collection_path_seg: (_) => /\/[_a-zA-Z0-9][_a-zA-Z0-9\-]*/,
    single_path_seg: (_) => /\/\{[_a-zA-Z][_a-zA-Z0-9]*\}/,
    multi_path_seg: (_) => /\/\{[_a-zA-Z][_a-zA-Z0-9]*=\*\*\}/,

    match_path: ($) =>
      repeat1(
        choice($.collection_path_seg, $.single_path_seg, $.multi_path_seg)
      ),

    method: (_) =>
      choice("read", "write", "get", "list", "create", "update", "delete"),

    rule_def: ($) =>
      seq(
        "allow",
        seq($.method, repeat(seq(",", $.method))),
        optional(seq(":", "if", $.expr)),
        ";"
      ),

    match_def: ($) => seq("match", $.match_path, $.match_body),

    service_body: ($) =>
      seq("{", repeat(choice($.function_def, $.match_def, $.rule_def)), "}"),

    match_body: ($) =>
      seq("{", repeat(choice($.function_def, $.match_def, $.rule_def)), "}"),
  },
});
