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
- Delete the selected objects (`Delete` key or `del` command)
- Layers: show/hide, lock and pick the current layer from the Layers panel (right dock, or the `la` command); new objects go on the current layer and are drawn in the layer's color
- Multi-tab support, with an unsaved-changes prompt before closing
- Open, Save and Save As for DXF files (the file path is remembered per tab); large files open in the background
- Rectangular array with preview
- Multi-select with window/crossing selection (AutoCAD-style)
- Move tool with snap
- Undo / Redo
- Command bar interface
- Auto-pan at window edges
- Pan with the middle mouse button, `Space` + drag, or a two-finger scroll on a touchpad; zoom with the wheel, `Ctrl` + scroll or a pinch
- Text and MText display (read, move, copy and save)
- Open DXF and DWG files (DWG is read-only; Save writes DXF)
- Block inserts and dimensions from other CAD programs
- Line weight and linetype, shown as stored in the file and editable per layer and per object

## DXF Support

DXF and DWG files are read, and DXF files are written, with [acadrust](https://crates.io/crates/acadrust) (newer DXF versions supported).

| Entity | Open | Save |
|---|---|---|
| Line | Yes | Yes |
| Circle | Yes | Yes |
| Arc | Yes | Yes |
| LWPolyline | Yes (segments with a bulge are converted to arcs) | Yes |
| Spline | Yes (flattened to a polyline) | Saved as a polyline |
| Ellipse | Yes (flattened to a polyline) | Saved as a polyline |
| Linear dimension (drawn in Drafting) | - | Saved as three plain lines |
| Layers | Yes (true color, on/off, lock, line weight, linetype) | Yes |
| Entity color | ACI and true color | Yes |
| Line weight / linetype | Yes (per layer and per object, ByLayer / ByBlock, LTSCALE) | Yes |
| Solid / Hatch | Yes (as filled areas) | SOLID or HATCH |
| Text / MText | Yes (plain text only; formatting is dropped) | Saved as TEXT, or MTEXT when multi-line |
| Block insert (INSERT) | Yes (exploded into plain objects) | Saved as plain objects |
| Dimension (any type) | Yes (drawn from the stored geometry, as plain lines and text) | - |

Other entity types (tables, leaders, images, ...) are skipped when opening a file.

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
- GTK4 ≥ 4.20
- libadwaita ≥ 1.6
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
| `Delete` | Delete the selected objects |
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
| `del`, `delete`, `erase` | Delete the selected objects |
| `la`, `layer`, `layers` | Show or hide the Layers panel |
| `ar`, `array` | Rectangular array |
| `fit` | Zoom to fit |
| `z` | Zoom in at cursor |
| `u`, `undo` | Undo |
| `r`, `redo` | Redo |
| `o`, `open` | Open a DXF or DWG file |
| `save` | Save |
| `saveas` | Save As |

## Trim and Extend

- **Trim**: click the part of an object to remove. Every other object acts as a cutting edge. A circle needs two cutting edges.
- **Extend**: click near the end of a line, arc or polyline to extend it to the nearest boundary.
- Objects that only touch at an endpoint do not count as crossing.

## Layers

Open the panel with the layers button in the right dock (or type `la`).

- **Eye**: show or hide a layer. Hidden layers can't be picked or selected, and they are ignored by object snap and by Trim/Extend.
- **Lock**: locked layers stay visible and snappable (and still act as cutting edges), but their objects can't be selected or edited. The lock state is saved to DXF.
- **Click a name** to make it the current layer. New objects are created on it.
- Layers, their colors and per-object colors are read from DXF files and written back when you save.
- Right-click a layer name to rename or delete it (its objects move to layer 0). Layer add/rename/colour/delete can be undone with Ctrl+Z.
- Under the list, pick the line weight and linetype of the current layer.
- The **Selected objects** row sets them for the selected objects. `ByLayer` means follow the layer.
- Line weight is drawn as a fixed number of pixels (0.25 mm = 2 px) and does not grow with zoom. Patterns that get too fine to see are drawn solid.

## Limitations

- MText formatting (bold, italic, colors, mixed sizes) is dropped; only plain text is kept.
- Block inserts are exploded when a file is opened, so they are saved as plain objects.
- Dimensions from a file are plain lines and text, not editable dimensions.
- Block attributes (ATTRIB) are not shown.
- DWG files can be opened but not saved; use Save As to write a DXF.
- Linetypes that contain text or shapes are drawn as plain dashes.

## License

GPL-3.0-or-later © 2026 Supakit Suptorranee
