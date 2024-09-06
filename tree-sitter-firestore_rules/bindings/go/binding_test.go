package tree_sitter_firestore_rules_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_firestore_rules "github.com/tree-sitter/tree-sitter-firestore_rules/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_firestore_rules.Language())
	if language == nil {
		t.Errorf("Error loading FirestoreRules grammar")
	}
}
