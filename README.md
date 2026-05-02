# Images64

**Images64** is a high-performance, native desktop application designed to handle the conversion of gigantic Base64 strings into images (and vice-versa) instantly, without freezing or crashing your system. 

Built with the blazing speed of **Rust (Tauri v2)** and a lightweight **Svelte + Tailwind CSS v4** frontend, this app bypasses standard browser memory limits to provide a frictionless developer tool for managing encoded assets.

---

## ✨ Core Features

* **Blazingly Fast Rust Backend:** Memory-intensive operations (parsing strings, decoding bytes, and file system saves) are offloaded entirely to Rust. Say goodbye to the dreaded "browser tab frozen" problem when pasting 20MB Base64 strings!
* **Smart Auto-Detection:** Paste a raw Base64 string and Images64 automatically reads the magic bytes (PNG, JPEG, GIF, WEBP, BMP, SVG, AVIF) to instantly render the correct image format without needing manual `data:image/png;base64,...` prefixes.
* **Reverse Conversion (Image to Base64):** Drag and drop an actual image file (`.png`, `.jpg`, etc.) straight into the workspace to instantly generate its raw Base64 string payload.
* **Zero-Lag Tab Management:** Open multiple active conversion tabs at once. The app leverages asynchronous `IndexedDB` to persist your tabs and sessions gracefully without blocking the UI thread.
* **Dynamic Workspace Views:**
  * **Native Scroll View:** Unbinds the image constraints so you can view gigantic images at their 100% native resolution using smooth vertical and horizontal scrollbars.
  * **Fit to View:** A single click retracts the text area and perfectly scales the image down to fit squarely in your screen.
  * **Immersive Fullscreen:** A dedicated distraction-free gallery view to navigate through all your loaded images.
* **Robust OS Integration:** Fully utilizes native Tauri drag-and-drop file listeners, secure OS file dialogs, and native clipboard APIs.

## 🛠️ Tech Stack

* **Backend:** Rust, Tauri v2 API, Base64 Engine.
* **Frontend:** Svelte (SvelteKit), Vite, TypeScript.
* **Styling:** Tailwind CSS v4, Lucide Icons.
* **Storage:** `idb-keyval` (IndexedDB) for async persistence.

## 🚀 Getting Started

### Prerequisites
Make sure you have Node.js and Rust installed on your machine.

### Installation & Run

1. Clone the repository and navigate into it:
   ```bash
   cd images64
   ```
2. Install the frontend dependencies:
   ```bash
   npm install
   ```
3. Run the development server (this will compile Rust and start the desktop app):
   ```bash
   npm run tauri dev
   ```
4. Build for release:
   ```bash
   npm run tauri build
   ```

## 🎨 Themes & Customization
Images64 comes baked with a fully integrated dark/light mode toggle. The light mode uses a custom "soft slate" palette engineered to be incredibly easy on the eyes, avoiding harsh pure-white screen glares often found in default developer tools.

### Custom Logos
To use your own custom logo for the application:
Drop a high-resolution PNG image named `icon.png` into the `/static/logos/` directory. The UI will instantly pick it up. For release builds, you can auto-generate all required OS icons (`.icns`, `.ico`, etc.) by simply running:
```bash
npx tauri icon static/logos/icon.png --output static/logos
```