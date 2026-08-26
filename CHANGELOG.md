# Solar Launcher Changelog

## 0.6.0

### Added
- Sidebar collapsing
  - PSA: please for the love of god don't make a issue that the text on the sidebar fucks up. I KNOW IT DOES.
- Favourite path saving
  - Saves the favourite path for each instance so it can be restored on the component being mounted.
- Splash text just like minecraft!
- A 10% chance of having a funny misspelled word in the empty main content.
- Extra repositories for installing your preferred engine.
- If your description it will be displayed as HTML in a card below the mods section.
  - This only works IF README.md and Changelog.md doesn't exist in that working directory

## Misc
- Combine `imagesrc.ts` with `sys.ts`
- A dedicated website for Solar Launcher; https://team-solarengine.github.io/solar-lanucher/
  - self promo teehee https://github.com/daveberrys/READMEtoWebsite

## 0.5.1

### Added
- System notification when a download finishes.
- Instance selection for GameBanana mods.
  - Pick which instance's mods folder the mod gets downloaded into.
- Folder/file explorer when adding or editing an instance.

### Fixed
- Mods section overflowing.
- Linux launches failing when the executable lacked permissions.
- Engine folder conflicts when downloading.
  - Folders are now date-stamped to avoid clashing with existing ones.
- GameBanana and drag-and-drop not showing newly added instances.

### Removed
- Native file explorer opening when Engine Downloads or Gamebanana are done.

## 0.5.0

### Added
- Native folder/file selection for GameBanana and GitHub.
- Mod sections in the main content.
  - Supports multiple engines by detecting `pack.json`.
  - Toggle mods on and off with a checkbox.
  - Move mods to `disabled-mods` when toggled off.
- Drag-and-drop feature to copy mods into your instances.
  - Shows an overlay while dragging.
  - Lets you pick a custom folder or one of your instances.
- Compact mode for the sidebar.

### Changed
- Full start extras in the main content.
- Cleaner implementation for README and Changelog rendering.

### Fixed
- TS errors and unneeded imports.

## 0.4.1

### Fixed
- Windows' path issue with backslashes and forwardslashes.

## 0.4.0

### Added
- Icons and GameBanana in the PromptForNew popup.
- Deeplinks with `solar-launch://` protocol support.
  - Usage; `solar-launch://gb-mods/{gamebanana-mod-id}`
- Automatically downloading in the app and putting it in your preferred folder.
- Use snackbars for error messages.

### Changed
- Rendering w/o banner in the Main content would center the thing you'd see to launch the game.
  - With README.md and Changelog.md alongside, the content is rendered but not being centered y-axis.

## 0.3.0

### Added
- A way to download engines!
  - Currently supports;
    - Solar Engine
    - Codename Engine
    - Psych Engine
    - Funkin
- Option to use `Local` for putting in your FNF instance, or `Download` to download FNF Engines.
- Added a GitHub Token option.

## 0.2.2

### Fixed
- Sidebar being all janky and not wanting to scroll at the bottom.

## 0.2.1

### Fixed
- Default pet icon now uses `images/sussy.png` when no icon URL is provided
- Windows's `C:\` path in case of future breaks
- Made README.md and Changelog.md scrollable.

### Misc
- Changed Version.svelte's `else` to something sensible.

## 0.2.0

### Added
- Local icon, banner and pet via path instead of using URLs
- Unix `Open in Terminal` now actually functions with specific supported terminal

### Fixes
- README.md and Changelog.md is now dependent. When one is missing and the other is present, only the present one is rendered.

## 0.1.1

### Added
- Version checker to check your version against the latest tag release.

### Fixed
- Last child of CardApp menu goes up instead of down
- Fix z-index StartExtra goes below CardApp

## 0.1.0
Initial release

---

For **developers**, please use this format;
```markdown
## {version}
### Added
- {lists of added}

### Updated
- {lists of updated}

### Fixed
- {lists of fixed}

### Removed
- {lists of removed}

### Misc
- {lists of misc}
```
