# {{project-name}}

A mirui desktop application using the selected `{{backend}}` renderer.

## Run

```bash
cargo run
```

The `wgpu` option renders through a native GPU surface. The `sdl` option uses the software renderer and builds SDL2 from source with static linking. Building SDL requires a C compiler and CMake, but no manually installed SDL2 library or `LIBRARY_PATH` setting. Both outputs are platform-specific and may still depend on operating-system libraries.

## Layout

- `src/main.rs` constructs the selected surface and runs the application.
- `src/ui.rs` contains the shared interactive UI.
- `Cargo.toml` enables only the selected mirui backend.

Renderer selection happens at generation time. To change it later, update the mirui feature and surface construction in `Cargo.toml` and `src/main.rs`.

## References

- [mirui quickstart](https://github.com/W-Mai/mirui/blob/main/docs/quickstart.md)
- [mirui gallery examples](https://github.com/W-Mai/mirui/tree/main/gallery/examples)

## License

MIT — replace as needed.
