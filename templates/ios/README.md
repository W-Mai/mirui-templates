# {{project-name}}

iOS mirui application using the `{{backend}}` rendering path. The Xcode target builds the Rust static library before linking the native application host.

## Run

Install the Rust targets, open the generated Xcode project, select an iOS Simulator or device, and run the application:

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
open ios/{{project-name}}.xcodeproj
```

The `wgpu` selection renders mirui commands directly with WGPU. The `sw` selection rasterizes into a retained RGBA framebuffer and uploads dirty regions through WGPU for presentation.

The application supports iPhone and iPad portrait and landscape configurations. Its mirui `App`, ECS world, and reactive state survive normal suspend/resume while the drawable surface is recreated.

`SoftwareUploadConfig::default()` uses a 1.0 render scale and a 16 MiB CPU framebuffer budget. Change those explicit values in `src/lib.rs` when a device needs a different memory/quality tradeoff. A resize above the budget keeps the last admitted framebuffer instead of allocating an unbounded replacement.
