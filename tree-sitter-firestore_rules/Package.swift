// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "TreeSitterFirestoreRules",
    products: [
        .library(name: "TreeSitterFirestoreRules", targets: ["TreeSitterFirestoreRules"]),
    ],
    dependencies: [
        .package(url: "https://github.com/ChimeHQ/SwiftTreeSitter", from: "0.8.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterFirestoreRules",
            dependencies: [],
            path: ".",
            exclude: [
                "Cargo.toml",
                "Makefile",
                "binding.gyp",
                "bindings/c",
                "bindings/go",
                "bindings/node",
                "bindings/python",
                "bindings/rust",
                "prebuilds",
                "grammar.js",
                "package.json",
                "package-lock.json",
                "pyproject.toml",
                "setup.py",
                "test",
                "examples",
                ".editorconfig",
                ".github",
                ".gitignore",
                ".gitattributes",
                ".gitmodules",
            ],
            sources: [
                "src/parser.c",
                // NOTE: if your language has an external scanner, add it here.
            ],
            resources: [
                .copy("queries")
            ],
            publicHeadersPath: "bindings/swift",
            cSettings: [.headerSearchPath("src")]
        ),
        .testTarget(
            name: "TreeSitterFirestoreRulesTests",
            dependencies: [
                "SwiftTreeSitter",
                "TreeSitterFirestoreRules",
            ],
            path: "bindings/swift/TreeSitterFirestoreRulesTests"
        )
    ],
    cLanguageStandard: .c11
)
