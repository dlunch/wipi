# PROJECT KNOWLEDGE BASE

**Generated:** 2026-02-14
**Commit:** 92e0dff
**Branch:** main

## OVERVIEW

Rust no_std library for developing WIPI (Korean mobile platform) apps. Targets ARM v4T (thumbv4t-none-eabi). Desktop simulation via winit for development.

## STRUCTURE

```
wipi/               # Main public API crate (App trait, framebuffer, timer, database, etc)
wipi_types/         # Shared C-compatible type definitions (WIPICWord, TargetPtr, interface structs)
wipi_macros/        # #[wipi_main] proc macro — transforms user main() into clet entry point
wipi_boot/          # Platform-specific initialization (global interface pointers, clet registration)
wipi_build/         # Build script utilities (linker script injection)
wipic_sys/          # FFI bindings layer — uniform API, platform-specific implementations
wipic_simulation/   # Desktop simulation runtime (winit event loop, softbuffer rendering)
wipi_archiver/      # CLI tool for packaging .zip archives for device deployment
examples/           # Example apps (helloworld, paint, input, resource, image, timer)
```

## ARCHITECTURE

Three-layer platform abstraction with mutually exclusive features (`ktf`, `lgt`, `simulation`):

```
examples → wipi (safe API) → wipic_sys (FFI) → wipi_boot (init) → wipi_types (C structs)
                                  ↓ (simulation only)
                           wipic_simulation (std, winit)
```

### Platform FFI Patterns

**KTF**: Global interface pointer → transmute → call
```rust
let f: extern "C" fn(...) = unsafe { transmute((*WIPIC_KNLINTERFACE).func) };
```

**LGT**: Dynamic method lookup by ID → transmute → call
```rust
let f: extern "C" fn(...) = unsafe { transmute(get_external_method(ImportModule::WIPIC, WIPICMethod::Func as _)) };
```

**Simulation**: Direct delegation to `wipic_simulation` crate (uses std)

### Adding a New API Module

1. `wipic_sys/src/{ktf,lgt,simulation}/kernel.rs` — Add FFI function following platform pattern
2. `wipi_types/src/lgt/wipic.rs` — Add `WIPICMethod` enum variant with hex ID
3. `wipic_simulation/src/kernel.rs` — Add simulation implementation
4. `wipic_sys/src/simulation/kernel.rs` — Add delegation wrapper
5. `wipi/src/{module}.rs` — Safe wrapper struct with Drop cleanup
6. `wipi/src/lib.rs` — `pub mod {module};`

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Public API (user-facing) | `wipi/src/` | App trait, framebuffer, timer, database, etc |
| C type definitions | `wipi_types/src/wipic.rs` | WIPICWord, TargetPtr, interface structs |
| KTF interface table | `wipi_types/src/ktf/wipic.rs` | WIPICKnlInterface, WIPICGraphicsInterface |
| LGT method IDs | `wipi_types/src/lgt/wipic.rs` | WIPICMethod enum (hex IDs from wie project) |
| FFI bindings | `wipic_sys/src/{ktf,lgt,simulation}/` | kernel.rs, graphics.rs, database.rs per platform |
| Boot/init | `wipi_boot/src/{ktf,lgt}/start.rs` | Global pointer setup, clet registration |
| Lifecycle callbacks | `wipi/src/lifecycle.rs` | startClet, paintClet, handleCletEvent exports |
| Entry point macro | `wipi_macros/src/lib.rs` | Transforms `#[wipi_main] fn main()` |
| Simulation GUI | `wipic_simulation/src/lib.rs` | winit event loop, softbuffer rendering |
| Linker script | `wipi_build/src/lib.rs` | Injects ktf.ld for embedded builds |
| Archive packaging | `wipi_archiver/src/lib.rs` | .zip creation for device deployment |

## CONVENTIONS

- **Edition 2024**, nightly-only (rust-toolchain.toml)
- **no_std + alloc** everywhere except wipic_simulation
- **Panic = abort** in all profiles
- **Always run `cargo fmt --all` before committing**
- Platform features are **mutually exclusive** — never enable ktf+lgt simultaneously
- Safe wrappers in `wipi/` follow: struct + methods + `Drop` for cleanup (see database.rs)
- `WIPICIndirectPtr` must be dereferenced via `deref_indirect_ptr()` before use — KTF adds 8-byte offset, LGT/simulation direct cast
- Lifecycle callbacks run on **main thread only** — `RefCell` is safe for `APP` state
- LGT method IDs sourced from [dlunch/wie](https://github.com/dlunch/wie) project's LGT method table

## ANTI-PATTERNS

- `static mut` — use `RefCell` + `SyncRefCell` wrapper instead (see lifecycle.rs)
- `as any`, `@ts-ignore` equivalents — never suppress type errors
- Mixing platform features — always use `cfg_if!` for platform-conditional code
- `HACK: wie specific layout` in `wipic_sys/src/ktf.rs:18` — `deref_indirect_ptr` offset is wie-emulator-specific

## COMMANDS

```bash
# Dev (simulation, default)
cargo clippy --all-targets
cargo test --all

# Embedded (KTF)
cargo clippy --all -Zbuild-std=core,alloc --target thumbv4t-none-eabi --features ktf --no-default-features -- -D warnings

# Embedded (LGT)
cargo clippy --all -Zbuild-std=core,alloc --target thumbv4t-none-eabi --features lgt --no-default-features -- -D warnings

# Build examples for device
./build_examples.sh

# Run simulation example
cargo run -p examples --bin paint
```

## NOTES

- CI runs on macOS/Ubuntu/Windows with nightly + thumbv4t-none-eabi cross-check
- VSCode configured with `simulation` feature for rust-analyzer (`.vscode/settings.json`)
- `wipic_simulation` timer uses `std::thread::spawn` — cancellation not yet implemented
- 13 TODOs in wipi_boot (incomplete relocation, error handling, class discovery)
- `#[wipi_main]` generates different code for simulation vs embedded (`target_os = "none"`)
