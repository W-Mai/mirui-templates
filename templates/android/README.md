# {{project-name}}

Android mirui application using the `{{backend}}` rendering path.

## Run

Install JDK 17, Android SDK 35, NDK 27.2.12479018, and the Rust target. Set `ANDROID_HOME` to the SDK directory and `JAVA_HOME` to JDK 17. The Gradle wrapper builds Rust with cargo-ndk and packages the NativeActivity:

```bash
rustup target add aarch64-linux-android
cargo install cargo-ndk --version 4.1.2 --locked
cd android
./gradlew installDebug
adb shell am start -n {{android-package}}/android.app.NativeActivity
```

The `wgpu` selection renders mirui commands directly with WGPU. The `sw` selection rasterizes into a retained RGBA framebuffer and uploads dirty regions through WGPU for presentation.

On Windows, use `gradlew.bat`. Open the `android/` directory in Android Studio to edit the host manifest or Java sources. The default ABI is `arm64-v8a`; for an x86-64 emulator, install the `x86_64-linux-android` Rust target and pass `-Pabis=x86_64`. Comma-separated ABIs build a universal APK. Each requested ABI needs its Rust target installed.

The manifest uses a placeholder application icon and disables cloud backup and device-to-device transfer. Customize `android/src/main/res/drawable/ic_launcher.xml` and `android/src/main/res/xml/data_extraction_rules.xml` for the application. `./gradlew lintDebug` treats Android lint warnings as errors.

`./gradlew assembleDebug` writes `build/outputs/apk/debug/{{project-name}}-debug.apk`. Debug signing uses the Android debug keystore. `./gradlew assembleRelease` produces an unsigned release APK unless all four signing environment variables are set: `MIRUI_ANDROID_KEYSTORE`, `MIRUI_ANDROID_STORE_PASSWORD`, `MIRUI_ANDROID_KEY_ALIAS`, and `MIRUI_ANDROID_KEY_PASSWORD`. Keep release keys outside the source tree.

The generated application follows Android configuration changes instead of locking one orientation. Its mirui `App`, ECS world, and reactive state survive NativeActivity suspend/resume while the native window and swapchain are recreated. Focused text inputs open the native software keyboard, and native memory warnings release reconstructible caches without discarding application state.

`SoftwareUploadConfig::default()` uses native device density within a 32 MiB CPU framebuffer budget. If the native framebuffer exceeds the budget, mirui selects the highest fitting scale. Change the policy or budget in `src/lib.rs` when a device needs a different memory/quality tradeoff.

The generated root keeps interactive content inside Android system bars and display cutouts. Call `app.spawn_root().ignore_safe_area().id()` only when fullscreen content should occupy those regions.

View native logs while the application runs:

```bash
adb logcat | grep -i mirui
```
