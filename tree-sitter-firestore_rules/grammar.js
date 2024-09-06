/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: 'firestore_rules',

  rules: {
    source_file: $ => seq(
      'service',
      'cloud.firestore',
      $.block,
    ),

    identifier: $ => /[a-zA-Z\.]+/,

    block: $ => seq("{", "}"),
  }
})
