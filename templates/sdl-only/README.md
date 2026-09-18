# {{project-name}}

A [mirui](https://github.com/W-Mai/mirui) application targeting the SDL2 desktop backend.

## Build and run

```bash
cargo run
```

The first build links SDL2 from the host system. On macOS Apple Silicon, Homebrew installs SDL2 at `/opt/homebrew/lib/`, which is outside the linker's default search list:

```bash
export LIBRARY_PATH="/opt/homebrew/lib:$LIBRARY_PATH"
```

On Debian / Ubuntu: `apt-get install libsdl2-dev`.

## What's in this project

- `Cargo.toml` pins `mirui = "{{mirui-version}}"` with the `sdl` feature.
- `src/main.rs` opens a 480×320 SDL window and stacks a header, content area, and footer. Replace the body with your own `ui!` tree.

## Where to go next

- Full mirui Quickstart: <https://github.com/W-Mai/mirui/blob/main/docs/quickstart.md>
- mirui examples: <https://github.com/W-Mai/mirui/tree/main/gallery/examples>
- Other targets use sibling templates in <https://github.com/W-Mai/mirui-templates>.

## License

This project starts under the MIT license. Adjust `Cargo.toml` and add a `LICENSE` file as appropriate.
