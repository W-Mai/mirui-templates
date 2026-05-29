# mirui-templates

[![CI](https://github.com/W-Mai/mirui-templates/actions/workflows/ci.yml/badge.svg)](https://github.com/W-Mai/mirui-templates/actions/workflows/ci.yml)

[cargo-generate](https://github.com/cargo-generate/cargo-generate)
templates for [mirui](https://github.com/W-Mai/mirui), a `no_std`
ECS-driven UI framework for embedded, desktop, and WebAssembly.

## Install cargo-generate

```bash
cargo install cargo-generate
```

## Available templates

| Template | Description |
|----------|-------------|
| `sdl-only` | Single-crate SDL desktop project. ~5 minutes from zero to a window. |
| `esp32c3` | Single-crate ESP32-C3 embedded project, framebuffer-driven, esp-hal stack. |
| `workspace` | Cargo workspace with a shared UI library and one binary crate per target (`targets/desktop`, `targets/esp32c3`). Globs in `targets/*` so adding a new MCU is `cp -r` plus four edits. |
| `wasm` | Stub for an upcoming `web-canvas` Surface backend. **Does not build yet** — see the template's README for the timeline. |

## Use a template

```bash
cargo generate W-Mai/mirui-templates sdl-only --name hello-mirui
cd hello-mirui
cargo run
```

Each template prompts for a project name and the mirui version to pin.
The generated project's `Cargo.toml` and `README.md` are filled in
from the prompts.

## Pinning to a mirui release

The templates default to whichever mirui version is current when this
repo's `main` was last bumped (see the per-template `cargo-generate.toml`
default). Pass `--define mirui-version=0.X` to override.

## Maintenance

This repo tracks the [mirui](https://github.com/W-Mai/mirui) release
cycle. When mirui ships a new minor, `cargo xtask templates-bump`
inside the mirui repo updates the per-template `cargo-generate.toml`
default and commits the bump here. The maintainer pushes after
reviewing the diff.

The mirui version that ends up in a generated project's `Cargo.toml`
comes from the cargo-generate prompt at generation time, not from
the template files — `--define mirui-version=0.24` overrides whatever
default the templates ship with.

## Contributing

To propose a new template (e.g. an additional MCU, a different desktop
backend, or a fully working wasm setup once `web-canvas` lands):

1. Add a `templates/<name>/` directory mirroring the layout of an
   existing template, including a per-template `cargo-generate.toml`
   declaring the placeholders.
2. Verify locally:
   ```bash
   cargo generate --path templates/<name> --name testfoo --define mirui-version=0.23
   cd testfoo && cargo build  # or cargo build --release for embedded
   ```
3. Add a job to `.github/workflows/ci.yml` that runs the same
   `cargo generate` + `cargo build` so CI catches regressions.
4. Update `README.md` and `CHANGELOG.md` with an entry for the new
   template.

For bug fixes to existing templates, file an issue with the exact
`cargo generate` invocation, the cargo error, and your toolchain
versions (`rustc -V`, `cargo -V`, `cargo generate -V`).

## See also

- [mirui](https://github.com/W-Mai/mirui) — the framework itself.
- [mirui-examples](https://github.com/W-Mai/mirui-examples) — full
  hardware demos including the ST7735S board file the `esp32c3`
  template stubs out.

## License

MIT — see `LICENSE`.
