# Blotworx Engine

Blotworx is an experimental Rust runtime for loading local Epic Mickey game data. It builds independently of BlueThinner. The project has no game installation, extracted game assets, or decompiled Lua source in Git.

## Public demo

Download the Windows demo and open **Blotworx.exe**. On first launch, supply your Epic Mickey Wii disc image. Blotworx imports it locally and opens the original title screen. Use the same save slot to resume from its last checkpoint. [Releases](https://github.com/Bloticle/blotworx/releases) · [Setup and compatibility](docs/PUBLIC_DEMO.md)

## Build and run

The current development target is 64-bit Windows and macOS. Install Rust and the Visual Studio C++ build tools. Cargo builds the bundled native dependencies. The default build includes the EM1 gameplay HUD renderer. It loads the HUD movie and textures from your game installation.

```powershell
cargo run --locked
```

Import a Wii disc image or choose an existing extracted game folder. Open **F10 → Game** to change levels and **F10 → Tools** to unlock preview abilities for the session. In the EM2 PC preview, **F6** toggles the shared controller.

For automated captures and validation, use `cargo run --locked -- diagnose --help`.

For interactive debugging, use the standard `dev` build shown above. It keeps debug assertions and optimizes the engine and its dependencies across all levels. Avoid reusing executables built with an `opt-level=0` override for playtesting. Press F3 in EM1 to view runtime frame timings.

## Epic Mickey 2 (PC)

Point `--data` at an EM2 PC installation (the folder with `DEM2.exe` and `packfiles`). `--level` takes a level descriptor name, optionally with a state; without one, Blotworx starts in Yen Sid's Lab.

```powershell
cargo run --locked -- --data "C:\Games\Epic Mickey 2" --level MeanStreet_South.visit2 --shared-playtest
```

`--shared-playtest` starts with movement and the follow camera on (F6 toggles it). Every warning and script error is printed to the console as well as written to the log. Script services that aren't implemented yet are logged once as `EM2 world service not implemented (ignored): <name>` and skipped, so the level's scripts keep running.

The world, characters and sky draw with the game's own shaders, so normal and specular maps, scrolling textures and the level's baked lighting (`.lit`) show. EM2 bloom and colour correction run as a post pass.

Settings (environment variables):

| Variable | Effect |
| --- | --- |
| `BLOTWORX_EM2_ENV_NATIVE=0` | World surfaces use the generic fallback renderer instead of the game's shaders |
| `BLOTWORX_EM2_ENV_NATIVE_SKIP=a,b` | Keeps world shaders whose names contain any of these parts on the fallback |
| `BLOTWORX_EM2_CHAR_NATIVE=0` | Characters use the fallback renderer |
| `BLOTWORX_EM2_POST=0` | Turns off EM2 bloom and colour correction |
| `BLOTWORX_STRICT_EM2=1` | Unimplemented script services stop the scripts again (for finding gaps) |
| `BLOTWORX_VSYNC=0` | Removes the refresh-rate frame cap |

To check a level's scripts, run a timed capture and read `scene_events.world.halted` and `source_errors` in the `.runtime.json` written next to the image:

```powershell
cargo run --locked -- diagnose --data "C:\Games\Epic Mickey 2" --level MeanStreet_South.visit1 --shared-playtest --capture shot.png --capture-tick 1800
```

Not done yet: the game's depth pre-pass, projected shadows and per-object point lights on world surfaces; animation of non-camera props from scripts; health, inventory and several sequence event kinds; logic timers. Mean Street South is the most tested level.

## Layout

- `src/blotworx/` contains the runtime, simulation, rendering integration, and platform compatibility.
- `src/render_support/` contains the shared scene, texture, material, and viewport code used by the runtime.
- `crates/asset-formats/` contains the parser snapshot required by this checkout.
- `third_party/` contains native dependencies and the optional embedded UI runtime. See [dependency provenance](third_party/README.md).
- `tools/blotworx/fixtures/` contains reference data used by the Rust tests. [Coverage notes](docs/blotworx-coverage.json) track work in progress.

The extracted runtime reads the game scripts supplied by the game installation. BlueThinner's reviewed script snapshot and its embedded GV cleanup override are excluded. Those script-specific paths may therefore behave differently from an existing BlueThinner build.

The build includes HavokLib under GPL-3.0. Individual third-party libraries and the format parser retain their own licenses.
