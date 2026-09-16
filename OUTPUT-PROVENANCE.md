# Surface output lookup

The `layershellev/` directory contains the published layershellev 0.19.1 crate.
Crates.io archive SHA256: `69fbd3ced50d91a5a118e3e00f49c72040ea6b48b78b159a3936104fc2632925`.
The upstream MIT license and author attribution are retained.

The only source addition is `WindowState::output_name`: it resolves a surface's
entered Wayland output through the existing xdg-output name cache. The seven
existing unsafe raw-handle borrows are unchanged from the published crate.
The unsafe baseline includes those imported lines.

The adapter records these names while refreshing each window and removes them
when the window closes. `iced_layershell::output_name` returns the latest name
without a compositor-specific command or a polling thread.
