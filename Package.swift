// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "Porthole",
    platforms: [.macOS(.v14)],
    dependencies: [
        .package(url: "https://github.com/sparkle-project/Sparkle", from: "2.6.0"),
    ],
    targets: [
        .target(
            name: "PortholeCore",
            path: "Sources/PortholeCore"
        ),
        .executableTarget(
            name: "porthole",
            dependencies: ["PortholeCore", .product(name: "Sparkle", package: "Sparkle")],
            path: "Sources/App"
        ),
        .testTarget(
            name: "PortholeCoreTests",
            dependencies: ["PortholeCore"],
            path: "Tests/PortholeCoreTests"
        ),
    ]
)
