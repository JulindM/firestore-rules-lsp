# Firestore Rules Tree Sitter Bindings/Grammar

This is the tree sitter grammar configuration for the firestore rules syntax.

The grammar is based heavily and implemented according to the following sources

- [Firestore Security Rules Language specification](https://firebase.google.com/docs/rules/rules-language#firestore)
- [Common Expression Language](https://github.com/google/cel-spec/blob/master/doc/langdef.md)

The common expression language is the base grammar; the firestore rules syntax is lightly modified version of the CEL.
