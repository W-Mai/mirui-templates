# {{project-name}}

Android mirui application using the `{{backend}}` rendering path.

## Run

Install the Android SDK, NDK, Rust target, and `cargo-apk`, then build and launch the native activity:

```bash
rustup target add aarch64-linux-android
cargo install cargo-apk
cargo apk run --release
```

The `wgpu` selection renders mirui commands directly with WGPU. The `sw` selection rasterizes into a retained RGBA framebuffer and uploads dirty regions through WGPU for presentation.

The generated application follows Android configuration changes instead of locking one orientation. Its mirui `App`, ECS world, and reactive state survive NativeActivity suspend/resume while the native window and swapchain are recreated.

`SoftwareUploadConfig::default()` uses a 1.0 render scale and a 16 MiB CPU framebuffer budget. Change those explicit values in `src/lib.rs` when a device needs a different memory/quality tradeoff. A resize above the budget keeps the last admitted framebuffer instead of allocating an unbounded replacement.

View native logs while the application runs:

```bash
adb logcat | grep -i mirui
```
