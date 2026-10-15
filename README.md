![Drafting](docs/Drafting-logo-text.png)

# Drafting

A simple 2D CAD application for Linux, built with GTK4 and libadwaita. A Windows developer preview is also available.

![Drafting Screenshot](docs/Drafting-preview.png)

## Features

- Draw lines, circles, 3-point arcs, polylines and linear dimensions
- Object snap: endpoint, midpoint, center, quadrant and intersection, each toggleable from the snap settings dock
- Grid snapping with adaptive LOD grid
- Offset (line, circle, arc, polyline) with preview
- Trim and Extend (line, arc, circle, polyline) with live preview; hold `Shift` to swap between the two
- Multi-tab support, with an unsaved-changes prompt before closing
- Open, Save and Save As for DXF files (the file path is remembered per tab); large files open in the background
- Rectangular array with preview
- Multi-select with window/crossing selection (AutoCAD-style)
- Move tool with snap
- Undo / Redo
- Command bar interface
- Auto-pan at window edges

## DXF Support

DXF files are read with [acadrust](https://crates.io/crates/acadrust) (newer DXF versions supported) and written with the [dxf](https://crates.io/crates/dxf) crate.

| Entity | Open | Save |
|---|---|---|
| Line | Yes | Yes |
| Circle | Yes | Yes |
| Arc | Yes | Yes |
| LWPolyline | Yes (segments with a bulge are converted to arcs) | Yes |
| Spline | Yes (flattened to a polyline) | Saved as a polyline |
| Ellipse | Yes (flattened to a polyline) | Saved as a polyline |
| Linear dimension | No | Saved as three plain lines |

Other entity types (hatch, solid, text, block inserts, tables, ...) are skipped when opening a file.

## Installation

### Linux (Flatpak) — Recommended

```sh
flatpak install com.pungpondsalami.drafting
```

> Flatpak is the primary supported platform.

### Windows (MSIX) — Developer Preview

Windows builds are available as MSIX packages via [GitHub Actions](https://github.com/pungpondsalami/Drafting/actions).

Download the `drafting-windows` artifact and unzip it. It contains the package, the signing certificate and an installer script in one folder:

```
drafting.msix
drafting-cert.cer
install.bat
install.ps1
```

Double-click `install.bat` (it asks for administrator rights). The script trusts the certificate and installs the package.

The certificate is the same for every build, so after the first install you can simply double-click a newer `drafting.msix` to update.

If you prefer to do it manually, from an Administrator PowerShell:

```powershell
Import-Certificate -FilePath .\drafting-cert.cer -CertStoreLocation Cert:\LocalMachine\TrustedPeople
Add-AppxPackage .\drafting.msix
```

See the [installation guide](docs/Steps_for_installing_Drafting_on_the_Windows_developer_version.pdf) for more details.

## Building from Source

### Requirements

- Rust (stable)
- GTK4 ≥ 4.12
- libadwaita ≥ 1.4
- Meson + Ninja

### Build

```sh
meson setup build
ninja -C build
ninja -C build install
```

Or for development:

```sh
cargo build
cargo run
```

## Keyboard Shortcuts

| Key | Action |
|---|---|
| `L` | Line tool |
| `C` | Circle tool |
| `A` | 3-point arc tool |
| `P` | Polyline tool |
| `D` | Dimension tool |
| `O` | Offset tool |
| `T` | Trim tool (`Shift`: extend) |
| `E` | Extend tool (`Shift`: trim) |
| `S` | Select mode |
| `Z` | Zoom to cursor |

## Command Bar

Type commands in the command bar at the bottom:

| Command | Action |
|---|---|
| `l`, `line` | Line tool |
| `c`, `circle` | Circle tool |
| `a`, `arc` | 3-point arc tool |
| `pl`, `pline`, `polyline` | Polyline tool |
| `of`, `offset`, `of [dist]` | Offset tool |
| `tr`, `trim` | Trim tool |
| `ex`, `extend` | Extend tool |
| `ar`, `array` | Rectangular array |
| `fit` | Zoom to fit |
| `z` | Zoom in at cursor |
| `u`, `undo` | Undo |
| `r`, `redo` | Redo |
| `o`, `open` | Open a DXF file |
| `save` | Save |
| `saveas` | Save As |

## Trim and Extend

- **Trim**: click the part of an object to remove. Every other object acts as a cutting edge. A circle needs two cutting edges.
- **Extend**: click near the end of a line, arc or polyline to extend it to the nearest boundary.
- Objects that only touch at an endpoint do not count as crossing.

## License

GPL-3.0-or-later © 2026 Supakit Suptorranee
