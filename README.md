# xclense

`xclense` is a static analysis and diagnostics tool design to inspect Kotlin Multiplatform frameworks exported
for iOS (XCFrameworks) and identify unused or overexposed APIs in Swift consumer codebases.

## Why `xclense`?

Kotlin Multiplatform allows for seamless code sharing across Android and iOS by exporting shared modules as XCFrameworks.
However, the exported symbols often include unused or internal declarations—cluttering the SDK, increasing binary size, and exposing unnecessary surface area.

`xclense` performs a **cross-language static analysis** between:
- The Swift codebase (consumer of the XCFramework i.e. iOS App),
- The Kotlin source metadata,
- The generated XCFramework umbrella headers,

...to determine what is actually used, and what can safely be trimmed or restricted (e.g., from `public` to `internal`).

## Why lean bridges matter

Reducing your public API surface isn't just about binary size—it improves modularity, enhances security,
and makes your application or SDK easier to test, reason about, and evolve. xclense helps keep your Kotlin ↔ Swift boundary clean, auditable, and intentional.

## Roadmap

- [ ] GitHub Actions plugin
- [ ] Kotlin Native .def auto-generator with trimmed output
- [ ] Visual symbol diff UI
- [ ] Full Kotlin symbol inspection via .klib and konan metadata

## Development Status

xclense is under active development. We welcome contributors, ideas, and feedback from Kotlin Multiplatform
and iOS communities!

## License

MIT License. © 2025-present, the xclense contributors.
