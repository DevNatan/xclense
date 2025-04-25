# xclense

**XCFramework Linting and Analysis for Kotlin Multiplatform SDKs**

`xclense` is a static analysis and diagnostics tool built in Rust, designed to inspect Kotlin Multiplatform (KMP) frameworks exported for iOS (XCFrameworks) and identify unused or overexposed APIs in Swift consumer codebases. Its purpose is to help teams enforce lean interoperability, reduce binary size, and tighten interface boundaries between Kotlin and Swift.

## 🔍 Why `xclense`?

Kotlin Multiplatform allows for seamless code sharing across Android and iOS by exporting shared modules as XCFrameworks. However, the exported symbols often include unused or internal declarations—cluttering the SDK, increasing binary size, and exposing unnecessary surface area.

`xclense` performs a **cross-language static analysis** between:
- The Swift codebase (consumer of the framework),
- The Kotlin source metadata,
- The generated XCFramework umbrella headers,

...to determine what is actually used, and what can safely be trimmed or restricted (e.g., from `public` to `internal`).

## ✨ Features

- ✅ Detect unused exported classes/functions/properties from Kotlin in Swift
- 📦 Parse `.swift` and `.h` files efficiently using `tree-sitter`
- ⚠️ Warn about overexposed API boundaries (e.g. internal classes marked as `public`)
- 📉 Help reduce binary size by generating `.def` trimming suggestions
- 🧪 Designed to integrate in CI pipelines
- 🔧 Configurable inclusion/exclusion filters per module or symbol
- 📂 Support for multiple Swift targets and frameworks in a single run

## 🛠️ Basic Usage

```shell
xclense analyze \
  --swift-path ./MyiOSApp/Sources \
  --xc-path ./Shared/build/XCFrameworks/Release/Shared.xcframework \
  --platform ios-arm64
```

Or use a configuration file and just `xclense analyze`

```shell
[swift]
path = "./MyApp/Sources"
exclude = ["Tests/", "Legacy/"]

[xcframework]
path = "./build/XCFrameworks/Release/MySDK.xcframework"
platform = "ios-arm64"

[report]
output = "json"
```

## 🧠 How it works

1. **Swift Static Analysis**
    - Uses `tree-sitter` to parse Swift source files.
    - Extracts all references to types, functions, properties, and constants used from the imported XCFramework.

2. **XCFramework Inspection**
    - Parses `.framework` headers (umbrella and modular).
    - Collects all publicly available symbols: classes, top-level functions, extensions, etc.

3. **Kotlin Metadata Extraction (Planned)**
    - Optionally reads metadata from `.klib` or `.def` files to track declaration origins.

4. **Diffing Engine**
    - Computes the delta between available symbols and used ones.
    - Outputs a detailed report of unused exports.

5. **Reporting**
    - CLI output: concise and colorized.
    - JSON and HTML outputs for CI and dashboard integration.

## 📦 Planned Integrations

* GitHub Actions plugin
* Kotlin Native .def auto-generator with trimmed output
* Visual symbol diff UI
* Full Kotlin symbol inspection via .klib and konan metadata

## 🧪 Development Status

xclense is under active development. We welcome contributions, ideas, and feedback from Kotlin Multiplatform and iOS communities!\
Want to contribute? Check out our CONTRIBUTING.md

## 🧼 Why lean bridges matter

Reducing your public API surface isn't just about binary size—it improves modularity, enhances security, and makes your application or SDK easier to test, reason about, and evolve. xclense helps keep your Kotlin ↔ Swift boundary clean, auditable, and intentional.

## License

MIT License. © 2025-present, the xclense contributors.