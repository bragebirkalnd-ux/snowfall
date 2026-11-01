# Snowhush (portable, offline build)

Soft snow on your Windows desktop. Based on https://github.com/mbezruchko/Snowhush (MIT, see LICENSE).

What is different from the original:
- **One portable `Snowhush.exe`.** No installer, nothing copied into Program Files or your profile, no administrator rights (the app manifest is `asInvoker`).
- **No network access.** The update check (it called api.github.com) is gone, along with the "Check for Updates" menu item.
- **No `cmd.exe` self-delete trick.** The installer's uninstaller ran `cmd.exe /c ping ... & rmdir` to delete itself, a pattern antivirus tools often flag. Removed with the installer.

Everything else (snow, drifts on windows, sweeping, Melt, etc.) is the original code.

## Run
Double-click `Snowhush.exe`. The snow icon appears in the tray; right-click it for options.
"Open at Login" writes one value under `HKEY_CURRENT_USER\...\Run` (no admin needed).

## Remove
Quit from the tray menu, untick Open at Login first, then delete `Snowhush.exe`.
Optional leftovers: registry key `HKEY_CURRENT_USER\Software\Snowhush` and folder `%LOCALAPPDATA%\Snowhush`.

## Build without admin rights
1. Put this folder in a new GitHub repository of yours.
2. Actions tab -> "Build Windows" -> Run workflow.
3. Download the `Snowhush-portable` artifact (contains `Snowhush.exe`).

Or locally: `cargo build --release --manifest-path windows/Cargo.toml` (needs Rust and the MSVC build tools).
