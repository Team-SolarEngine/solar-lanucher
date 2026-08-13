# Solar Launcher Changelog

## 0.3.1

### Added
- Icons and GameBanana in the PromptForNew popup.

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