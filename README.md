# Arcane Frontier — Rust 3D Action-RPG Starter

A small third-person 3D arena action-RPG prototype made with Rust + Macroquad. The Android build is handled by GitHub Actions.

## Current gameplay

- Third-person follow camera and a small 3D arena
- Touch joystick on the left side of the screen
- Magic Pulse button on the right (an abstract area-effect ability)
- Desktop keyboard support: WASD / arrow keys to move, Space to pulse
- Creatures that approach the player and drain vitality at close range
- Health, XP, level-ups, and a simple arena-clear/recovery loop

This is a playable foundation, not a finished commercial RPG. It currently uses simple 3D shapes rather than imported character models, animations, music, or a large world.

## Build on GitHub (phone-friendly)

1. Create a new GitHub repository.
2. Upload the contents of this folder to the repository's `main` branch. Make sure `.github/workflows/android.yml` is included.
3. Open the repository's **Actions** tab.
4. Select **Build Android APK** and press **Run workflow**, or push a commit to `main`.
5. When the workflow finishes, open the successful run and find **Artifacts** near the bottom.
6. Download `arcane-frontier-android-apk`, extract the ZIP, and install the APK on your Android device. You may need to allow installation from the file manager you use.

The workflow uses Macroquad's documented `cargo quad-apk build --release` Docker build path. The project currently targets Android ARM64 (`aarch64`), which is suitable for most modern Android phones. [Macroquad Android build guide](https://macroquad.rs/articles/android/).

## Run on a computer

Install Rust, then run:

```bash
cargo run
```

## Main code

- `src/main.rs` — game loop, 3D scene, player movement, touch controls, creatures, ability, XP and UI
- `Cargo.toml` — Rust dependency and Android package settings
- `.github/workflows/android.yml` — automatic APK build and artifact upload

## Next development steps

1. Add a title screen, pause menu, and settings.
2. Add a proper character model and animation system.
3. Add a larger world with quests and NPCs.
4. Add saved progress and more abilities.
5. Add sound effects and music.
