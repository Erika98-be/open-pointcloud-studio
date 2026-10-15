# OpenAEC UI crate snapshot

This portable snapshot comes from the sibling `OpenAEC-app-kopie` Style Repo.
The Style Repo remains the source of truth until it has a published Git URL.
The desktop application uses this local workspace crate so a checkout builds
without relying on another directory on the developer's machine.

The integration uses its canonical semantic icons for titlebar commands,
its bundled Inter and Space Grotesk fonts, 77 common color-token fields in
each of the five themes, and the shared ribbon-button state styling. Pointcloud keeps local colors for its 3D scene,
drawings, sheets, result categories, shadows, and tooltips. The domain ribbon
and panels remain in the application while they are migrated to the shared
shell. Keep the source snapshot and license in sync when updating it.
The snapshot includes the crate's font license texts and the Apache-2.0
notice for the source-reviewed Speech microphone icon; the Style Repo's
`scripts/check_portable_ui.py` verifies these files alongside code and assets.

The desktop now uses the source Tauri shell heights from `openaec_ui::model::metrics`:
32 px title, 28 px ribbon tabs, 94 px ribbon tools, 30 px document strip and
22 px status. The document strip spans the explorer, canvas and properties.
`openaec_ui::frame::FrameSlots` now composes the complete Pointcloud window,
including File view. Pointcloud still owns the commands, adaptive ribbon groups
and 3D drawing; those domain behaviors are not replaced by the crate's demo shell.
The software-rendered [Light](../design/pointcloud-shell-light.png) and
[Forge](../design/pointcloud-shell-forge.png) views are review baselines.
Legacy OpenCAD command SVGs with fixed dark strokes are tinted to the shared
text color on dark themes. They need final per-icon vector harmonization; the
current tint keeps every command legible during that work.

A field-by-field comparison found 378 values identical to the old Pointcloud
tokens and seven deliberate changes to the Style Repo values: the five ribbon
group labels and Light-theme status text/label. These visible differences
align Pointcloud with the common Tauri-derived palette.
