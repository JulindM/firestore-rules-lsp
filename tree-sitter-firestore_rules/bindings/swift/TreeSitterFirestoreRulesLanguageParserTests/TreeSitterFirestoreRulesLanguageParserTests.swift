import XCTest
import SwiftTreeSitter
import TreeSitterFirestoreRulesLanguageParser

final class TreeSitterFirestoreRulesLanguageParserTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_firestore_rules_language_parser())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading FirestoreRulesLanguageParser grammar")
    }
}
