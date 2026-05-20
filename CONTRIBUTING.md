# Contributing to Utah-Kernel / Utah-OS

Thank you for helping improve [utah-kernal](https://github.com/utahisnotastate/utah-kernal).

## Getting started

1. Read [docs/QUICKSTART.md](docs/QUICKSTART.md)
2. Read [REPO_ARCHITECTURE.md](REPO_ARCHITECTURE.md)
3. Build: `cargo check -p utah-kernel -p glass-forge`

## Where to change things

| Goal | Location |
|------|----------|
| New host call | `core/src/system_calls.rs` + `docs/HOST_API.md` |
| Kernel boot / init | `core/src/main.rs`, `core/src/utah_os.rs` |
| UI drawing | `ui/src/` |
| Packaging | `tools/utah-pack.py` |
| Host apps | `genesis/src/` |

## Pull requests

- Keep PRs focused; match existing Rust style in `core/` and `ui/`
- Run `cargo check` before submitting
- Update `docs/HOST_API.md` if you add or change imports
- Do not commit `target/` or `*.wasm` payloads

## Code of conduct

Be respectful. This project mixes experimental OS research with humor in docs — technical rigor in code is appreciated.
