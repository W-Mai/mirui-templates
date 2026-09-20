# {{project-name}}

iOS mirui application using the `{{backend}}` rendering path. The Xcode target builds the Rust static library before linking the native application host.

## Run

Install the Rust targets, open the generated Xcode project, select an iOS Simulator or device, and run the application:

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
open ios/{{project-name}}.xcodeproj
```

The `wgpu` selection renders mirui commands directly with WGPU. The `sw` selection rasterizes into a retained RGBA framebuffer and uploads dirty regions through WGPU for presentation.

The application supports iPhone and iPad portrait and landscape configurations. Its mirui `App`, ECS world, and reactive state survive normal suspend/resume while the drawable surface is recreated. Focused text inputs open the native software keyboard, and native memory warnings release reconstructible caches without discarding application state.

`SoftwareUploadConfig::default()` uses native device density within a 32 MiB CPU framebuffer budget. If the native framebuffer exceeds the budget, mirui selects the highest fitting scale. Change the policy or budget in `src/lib.rs` when a device needs a different memory/quality tradeoff.

The renderer uses the full iOS drawable. The generated root applies logical safe-area insets around the Dynamic Island, home indicator, and system bars while allowing edge-to-edge backgrounds through `ignore_safe_area()`.
