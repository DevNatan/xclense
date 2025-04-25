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