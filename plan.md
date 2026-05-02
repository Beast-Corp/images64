# App Plan: Images64 (Base64 <-> Image Converter)

## Overview
A high-performance, low-memory desktop application built with Tauri (Rust) and Svelte to seamlessly convert Base64 strings to images, manage multiple sessions via tabs, and provide quick image manipulation and saving tools.

## Tech Stack
- **Backend:** Tauri (Rust) for native OS interactions (file system, dialogs, clipboard).
- **Frontend:** Svelte + Vite + TypeScript + TailwindCSS for a highly optimized, low-memory, and reactive UI.
- **State Management:** Svelte stores (for tabs, settings, and active session state).

## Features & Implementation Checklist

### Phase 1: Project Scaffolding & Setup
- [ ] Initialize Tauri project with Svelte and Vite in the current directory.
- [ ] Setup TailwindCSS for rapid UI styling.
- [ ] Configure basic window settings in `tauri.conf.json` (title, dimensions, minimum size).

### Phase 2: Core UI & Layout
- [ ] Implement responsive Left Sidebar for Tab/History management.
- [ ] Implement Right Main Area for the active conversion workspace.
- [ ] Build Dark/Light mode toggle.
- [ ] Implement Toast Notification system for application logs (success, error, warnings).

### Phase 3: Session & Tab Management
- [ ] Create state management for Tabs (each tab containing its own input string, loaded image, and state).
- [ ] Implement "Add New Tab", "Close Tab", and "Switch Tab" functionality.
- [ ] Ensure session persistence (save active tabs and their data to `localStorage` or local file so they restore on reopen).

### Phase 4: Input Mechanisms
- [ ] Implement large text area for direct Base64 string input.
- [ ] Create upward-collapsing animation for the text area once a string is entered/processed.
- [ ] Implement File Input: Read `.txt` files containing Base64 strings from the computer.
- [ ] Implement Drag & Drop: Allow dropping `.txt` files directly into the main area.
- [ ] **Bonus Feature:** Intelligent Auto-Formatting (detect if the base64 string is missing `data:image/png;base64,` and auto-prefix it based on magic bytes).

### Phase 5: Image Rendering & Output Actions
- [ ] Display the decoded image below the collapsed input field.
- [ ] **Bonus Feature:** Display Image Metadata (File Size, Format, Dimensions).
- [ ] Implement "Save Image" native dialog via Tauri API.
- [ ] Add Settings page/modal to configure a default mutable save path to bypass the dialog if desired.
- [ ] Implement "Full Screen Image" view / Lightbox modal.
- [ ] Implement "Copy Image to Clipboard" button.

### Phase 6: Extra Polish & Power-User Features
- [ ] **Reverse Conversion:** Allow dropping an actual image file (PNG/JPG) to generate and copy its Base64 string.
- [ ] Implement a "History" view in the sidebar to access previously converted images quickly.
- [ ] Optimize rendering for very large Base64 strings (preventing UI freezes during paste).
