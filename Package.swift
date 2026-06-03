// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "Porthole",
    platforms: [.macOS(.v13)],
    targets: [
        .target(
            name: "PortholeCore",
            path: "Sources/PortholeCore"
        ),
        .executableTarget(
            name: "porthole",
            dependencies: ["PortholeCore"],
            path: "Sources/App"
        ),
        .testTarget(
            name: "PortholeCoreTests",
            dependencies: ["PortholeCore"],
            path: "Tests/PortholeCoreTests"
        ),
    ]
)
