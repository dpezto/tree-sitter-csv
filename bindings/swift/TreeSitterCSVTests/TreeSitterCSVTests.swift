import XCTest
import TreeSitterCSV

final class TreeSitterCSVTests: XCTestCase {
    func testCanLoadGrammars() {
        XCTAssertNotNil(tree_sitter_csv())
        XCTAssertNotNil(tree_sitter_psv())
        XCTAssertNotNil(tree_sitter_tsv())
    }
}
