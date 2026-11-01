// swift-tools-version: 5.6
import PackageDescription

let package = Package(
    name: "TreeSitterCSV",
    platforms: [.macOS(.v10_13), .iOS(.v11)],
    products: [.library(name: "TreeSitterCSV", targets: ["TreeSitterCSV"])],
    targets: [
        .target(
            name: "TreeSitterCSV",
            path: ".",
            sources: [
                "csv/src/parser.c",
                "csv/src/scanner.c",
                "psv/src/parser.c",
                "psv/src/scanner.c",
                "tsv/src/parser.c",
                "tsv/src/scanner.c",
            ],
            publicHeadersPath: "bindings/swift/TreeSitterCSV",
            cSettings: [.headerSearchPath("csv/src")]
        ),
        .testTarget(
            name: "TreeSitterCSVTests",
            dependencies: ["TreeSitterCSV"],
            path: "bindings/swift/TreeSitterCSVTests"
        ),
    ]
)
