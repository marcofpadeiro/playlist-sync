# playlist-sync

Project to sync Spotify playlists with a local music library.

This repository contains several crates and small applications (some are currently skeletons). Below is a summary of what is present, how to build and run components, and Windows-specific notes (PowerShell examples).

Quick summary
 - Name: playlist-sync
 - Goal: tools to parse Spotify playlists, match tracks with a local library and generate files (for example M3U8 playlists).

Repository layout (detected)

Top-level relevant files:

 - `Cargo.toml`           : Top-level workspace manifest
 - `README.md`           : This file
 - `src/main.rs`         : Top-level binary (currently prints "Hello, world!")

Subcrates / apps (located under `src/`):

 - `src/crates/cli/`             : `cli` crate (contains `src/main.rs` — currently empty)
 - `src/apps/common/`           : `common` crate (library, currently empty)
 - `src/apps/local-music-parser/`: `local-music-parser` crate (skeleton)
 - `src/apps/m3u8-writer/`      : `m3u8-writer` crate (skeleton)
 - `src/apps/spotify-parser/`   : `spotify-parser` crate (skeleton)

Note: many of the `lib.rs` and `main.rs` files are empty or contain minimal content. This suggests the project is in an early stage or some files were not filled yet.

Workspace note (important)

The top-level `Cargo.toml` currently lists `members = ["crates/*", "apps/cli"]`. However, the actual crates are located at `src/crates/*` and `src/apps/*`. This mismatch can cause `cargo` workspace commands to fail because the workspace members do not match the actual folder layout.

Options to fix/use the workspace:
 - Update the top-level `Cargo.toml` to point to the correct paths (for example `src/crates/*`, `src/apps/*`).
 - Or build/run crates individually using `--manifest-path` (examples below).

How to build (PowerShell)

Build whole workspace (may fail if workspace members are mismatched):

```powershell
# Build in release mode
cargo build --release
```

Build/Run a specific crate using its manifest (recommended while the workspace is misaligned):

```powershell
# Build the `cli` crate located at src/crates/cli
cargo build --manifest-path src\crates\cli\Cargo.toml --release

# Run the `cli` crate (generic example)
cargo run --manifest-path src\crates\cli\Cargo.toml -- <args>
```

Example run (PowerShell)

If one of the binaries accepts arguments, pass them after `--`. Generic example:

```powershell
# Example: run the CLI with arguments
cargo run --manifest-path src\crates\cli\Cargo.toml -- "arg1" "arg2"
```

Windows path notes

 - If some code writes Unix-style hard-coded paths, update them to Windows paths. Simple Rust example:
  const RESULTS_PATH: &str = "C:\\Users\\YourUser\\benchmark_results.csv";

Current project state

 - Many `src/*.rs` files are empty (skeletons). There is little implemented logic in several crates.
 - The top-level `src/main.rs` only contains `println!("Hello, world!")`.
 - I recommend aligning workspace members in the top-level `Cargo.toml` or moving crates to match the declared members.

Troubleshooting

 - Error: "package not found for workspace member ..." — check and fix paths in `Cargo.toml` or use `--manifest-path` for individual targets.
 - Permission error when writing files on Windows — point outputs to a directory where you have write permission (for example `C:\Users\YourUser\...`).
