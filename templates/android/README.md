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

`SoftwareUploadConfig::default()` uses native device density within a 32 MiB CPU framebuffer budget. If the native framebuffer exceeds the budget, mirui selects the highest fitting scale. Change the policy or budget in `src/lib.rs` when a device needs a different memory/quality tradeoff.

The generated root keeps interactive content inside Android system bars and display cutouts. Call `app.spawn_root().ignore_safe_area().id()` only when fullscreen content should occupy those regions.

View native logs while the application runs:

```bash
adb logcat | grep -i mirui
```
