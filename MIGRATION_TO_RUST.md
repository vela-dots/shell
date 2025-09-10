Migration Plan: Port C++ Plugin To Rust

Scope
- Replace the QML plugin under `shell/plugin/src/Vela` (currently C++/Qt) with a Rust implementation.
- Keep the QML surface/API intact: import URI stays `Vela` and exported types keep the same names and signals.
- Remove all references to CMake C++ targets once the Rust plugin is stable.

Recommended Tooling
- cxx-qt: Rust-first bindings for Qt 6 with QML support.
  - Pros: modern, maintained, lets you expose `#[qobject]` types, properties, signals, and invokables to QML.
  - Docs: https://github.com/KDAB/cxx-qt
- Alternative: qmetaobject (mature, Qt 5/6). If you prefer a pure-Rust approach without generated C++ glue.
  - Docs: https://github.com/woboq/qmetaobject-rs

High-Level Steps
1) Create a Rust crate in `shell/plugin/vela_rust/` using `cxx-qt`.
   - Cargo.toml: use `cxx-qt`, `cxx-qt-lib`, and any FFI crates (pipewire, aubio, cava, qalculate).
   - build.rs: call `cxx_qt_build::bridge("src/lib.rs")?;` to generate glue.
2) Expose a QML module `Vela` from Rust
   - Use `#[cxx_qt::qobject(qml_element, qml_uri = "Vela")]` per exported type.
   - For singletons, add `qml_singleton` and register during plugin init.
3) Port types one-by-one, keeping QML API stable:
   - CUtils
     - Invokables: `saveItem(...)`, `copyFile(...)`, `getDominantColor(...)`, `getAverageLuminance(...)`, `toLocalFile(...)`.
     - Strategy: Use `QImage`/`QQuickItemGrabResult` via `cxx-qt-lib` wrappers. Compute colors in Rust.
   - FileSystemModel
     - Provide a `QAbstractListModel` equivalent.
     - With cxx-qt: implement a `#[qobject]` with a backing `Vec<Entry>` and override `rowCount`, `data`, `roles`.
     - Roles: match the current C++ roles to avoid QML breakage.
   - CachingImageManager
     - If it backs a `QQuickImageProvider`, mirror it in Rust using a tiny C++ shim or expose a Rust object with an invokable that returns `QImage`.
   - Qalculator
     - Use FFI to libqalculate. If no ready crate is suitable, add a small `-sys` wrapper via `bindgen`.
     - Keep API surface identical (`evaluate(expr) -> string|number`).
   - AudioCollector / AudioProvider
     - PipeWire: use the `pipewire` crate (or `pipewire-sys` + safe wrapper) to set up a main loop + stream listener.
     - Mirror the current signals/props: `chunkSize`, `valuesChanged(QVector<double>)`, etc.
   - CavaProvider / CavaProcessor
     - Link to libcava via FFI. Initialize plan, execute per-audio-chunk, apply the “monstercat” filter in Rust, emit values.
   - BeatTracker (aubio)
     - Use `aubio` crate/FFI for onset/tempo detection. Emit beat ticks/signals compatible with the current QML.
   - Service / ServiceRef
     - If these are light wrappers/registrars, implement as Rust QObjects that register and hold references to the above services.
4) Build integration
   - Keep `qt_add_qml_module` in CMake short-term to install the module directory.
   - Link the Rust-produced shared library into the module dir.
   - Option A (preferred): Let `cxx-qt` generate + build glue C++ that CMake installs next to the Rust lib in the `Vela` URI path.
5) Testing
   - Start with `CUtils` (fast to port + easy to test via QML).
   - Add `FileSystemModel` second (exercise model plumbing).
   - Move to audio (PipeWire) and cava/aubio last (external deps, runtime heavy).
   - Validate via `qs -c vela` and targeted QML harnesses for each object.

Example: CUtils skeleton (cxx-qt)
```rust
// shell/plugin/vela_rust/src/lib.rs
use cxx_qt::qobject;
use cxx_qt_lib::{QColor, QImage, QUrl};

#[qobject]
pub struct CUtils {}

#[qobject::qml_element]
#[qobject::qml_namespace = "Vela"]
impl qobject::CUtils {
    #[qinvokable]
    pub fn to_local_file(&self, url: &QUrl) -> String {
        if !url.is_local_file() { return String::new(); }
        url.to_local_file().to_string()
    }

    #[qinvokable]
    pub fn get_dominant_color(&self, image: &QImage, rescale: i32) -> QColor {
        // downscale, histogram, return dominant
        // map back to QColor
        QColor::from_rgba(0xff000000) // placeholder
    }
}
```

Model outline (FileSystemModel)
- Roles: e.g. `path`, `name`, `isDir`, `size`…
- Implement as a `#[qobject]` with a `Vec<Entry>`. In `data(role)`, match ints with QML names via `role_names()`.

FFI crates and notes
- PipeWire: `pipewire` crate; if not sufficient on your platform, fall back to `pipewire-sys` + event-loop in Rust.
- Aubio: `aubio` crate for onset/tempo; otherwise FFI.
- Cava: small unsafe FFI binding (`bindgen`) for `cava_init`, `cava_execute`, `cava_destroy`.
- Qalculate: `libqalculate` FFI, or high-level crate if suitable.

QML Compatibility
- Keep the QML URI `Vela` and type names (CUtils, FileSystemModel, etc.) identical.
- Where names changed (e.g., DominantColour -> DominantColor), provide a temporary alias method for compatibility until QML is updated everywhere.

Build/Install
- Install to the same `QMLDIR` path CMake uses now for the `Vela` module.
- Ensure the Rust lib is discoverable by Qt (rpath or system lib dir) and the `qmldir` and `plugins.qmltypes` are placed alongside.

Cutover Strategy
1) Land the Rust crate and a single ported type (CUtils) behind a feature flag.
2) Switch imports (`import Vela`) stay unchanged; verify calls.
3) Port each remaining C++ type; remove its C++ counterpart when done.
4) Finally, delete the C++ plugin target and simplify CMake.

Notes on External Naming
- This repo has been US-English normalized. Upstream types from Quickshell that expose British names (e.g., `Colouriser`, `ColouredIcon`) remain until upstream provides US aliases. Keep usage as-is to avoid runtime breakage.

