# principle

```mermaid
flowchart TD
    A[main.rs] -->|run eframe| B[ui/app.rs]
    B -->|setup fonts and style, get connected user| C[ui/pages/main/ui.rs]
    C -->|render main page and route| D{Click?}
    D --> E[ui/pages/product/*.rs]
    D --> F[ui/pages/storelocation/*.rs]
```

# egui themes

Style: Root config for visuals, spacing, fonts Global or per-Ui
Visuals: Colors, shadows, widget states (hovered, active) Global or per-Ui
Spacing: Margins, padding, widget dimensions Global or per-Ui
TextStyle: Font mappings for labels, headers, etc. Global or per-Ui

# layout tutorial

<https://hackmd.io/@Hamze/Sys9nvF6Jl>

# third party crates

https://github.com/lucasmerlin/hello_egui/tree/main/crates/egui_form
https://github.com/amPerl/egui-phosphor

```

```
