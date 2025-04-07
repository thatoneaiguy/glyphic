
![Logo](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/th5xamgrr6se0x5ro4g6.png)


---

Glyphic is a next-generation animation and design studio built in Rust, combining the precision of vector graphics with the fluidity of pixel art. Designed for creators who demand both power and simplicity, this easy to use Illustrator-like is perfect for you.

![MPL Badge](https://img.shields.io/badge/license-mpl2-blue?style=for-the-badge)
![Rust Badge](https://img.shields.io/badge/uses-RUST-blue?style=for-the-badge)

## Installation

Find us on [itch.io](https://itch.io/) (soon) or the [Releases Page](https://github.com/thatoneaiguy/glyphic/releases)
    
## Feedback

If you have any feedback, please reach out to us on the [Aconite Studios Discord](https://discord.gg/DGTDSNfZWC)


## License

[Mozilla Public License 2.0](https://choosealicense.com/licenses/mpl-2.0/)


## Authors

- [@eeverest](https://www.github.com/thatoneaiguy)
- [@pomlmao](https://github.com/pomlmao)

## Contributors
### Web
- [@henryruss3](https://github.com/henryruss3)
### Lua
- Wizard (@wizard_0 on discord)

## Roadmap

### 1. Home Menu
- File type selection (`.animproj`, `.pixproj`)
- Canvas size presets (social media, 4K, custom)
- Drag-and-drop project loading
- Recent projects list

### 2. UI System
- HTML/CSS theme customization (Tauri)
- Resizable panels (toolbars, layers, timeline)
- **Toast Notifications**:
  - Themed backgrounds (error/update/success textures)
  - Right-side slide-in/out animations
  - Auto-dismiss after 3-5s
  - Interactive buttons (e.g., "Install Update")

### 3. Canvas & Rendering
- **Infinite Zoom/Pan**:
  - GPU-accelerated (wgpu + glam)
  - Mini-map navigation
  - Dynamic resolution scaling
- **Vector Graphics**:
  - Bézier curve editing (kurbo)
  - GPU tessellation (lyon)
  - Stroke/fill customization
- **Pixel Art Mode**:
  - Brush/eraser tools (image crate)
  - Palette system
  - Snap-to-grid

### 4. Plugin System
- **Lua Scripting**:
  - Sandboxed environment (mlua)
  - Plugin manager UI
  - Hot-reloading
- **Block Coding**:
  - Visual scripting (Blockly → Lua)
  - Pre-made animation blocks
  - Code preview panel

### 5. Additional Features
- Donation prompt (time-based, configurable)
- Project management (serde):
  - Save/load projects
  - Autosave
- Undo/redo system
- Text rendering (cosmic-text):
  - Font selection
  - GPU-accelerated

## Technical Stack

| Component           | Libraries/Tools                          |
|---------------------|------------------------------------------|
| **Core Framework**  | Tauri, React                            |
| **Rendering**       | wgpu, glam, lyon, image                 |
| **Vector Graphics** | kurbo, vello (experimental)             |
| **Plugins**         | mlua, Blockly                           |
| **Text**            | cosmic-text, font-kit                   |
| **Serialization**   | serde, serde_json                       |
| **UI Extras**       | egui-toast (native fallback)            |

## UX Highlights
✅ **Customizable** - CSS theming, layout presets  
✅ **Performant** - GPU rendering @ 60fps+  
✅ **Extensible** - Lua plugins + visual scripting  
✅ **Polished** - Animated toasts, undo history  
