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

A field-by-field comparison found 378 values identical to the old Pointcloud
tokens and seven deliberate changes to the Style Repo values: the five ribbon
group labels and Light-theme status text/label. These visible differences
align Pointcloud with the common Tauri-derived palette.
