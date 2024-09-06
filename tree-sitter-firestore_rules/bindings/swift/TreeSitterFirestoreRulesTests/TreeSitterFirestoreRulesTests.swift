import XCTest
import SwiftTreeSitter
import TreeSitterFirestoreRules

final class TreeSitterFirestoreRulesTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_firestore_rules())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading FirestoreRules grammar")
    }
}
