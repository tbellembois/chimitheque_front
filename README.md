```mermaid
flowchart TD
    A[main.rs] -->|run eframe| B
    B[ui/app.rs] -->|setup fonts and style, get connected user and then...| C[ui/pages/main/ui.rs]
    C -->|render main page and route...|D[click?]
    D -->[ui/pages/product/*.rs]
    D -->[ui/pages/storelocation/*.rs]
```
