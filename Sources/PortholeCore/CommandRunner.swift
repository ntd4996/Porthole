import Foundation

public protocol CommandRunner: Sendable {
    /// Run a binary and return stdout, or nil if it fails / is missing.
    func run(_ launchPath: String, _ arguments: [String]) async -> String?
}

public struct SystemCommandRunner: CommandRunner {
    public init() {}

    public func run(_ launchPath: String, _ arguments: [String]) async -> String? {
        await withCheckedContinuation { continuation in
            let process = Process()
            process.executableURL = URL(fileURLWithPath: launchPath)
            process.arguments = arguments
            let pipe = Pipe()
            process.standardOutput = pipe
            process.standardError = FileHandle.nullDevice
            do {
                try process.run()
            } catch {
                continuation.resume(returning: nil)
                return
            }
            let data = pipe.fileHandleForReading.readDataToEndOfFile()
            process.waitUntilExit()
            continuation.resume(returning: String(data: data, encoding: .utf8))
        }
    }
}
