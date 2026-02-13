---
title: "Learning egui"
date: "2026-02-11"
tags: ["tutorial", "egui", "learning"]
---

Today I learned about egui's immediate mode GUI. It's quite different from retained mode frameworks but very intuitive.

### What I like:
- Easy to get started
- Great documentation
- Cross-platform (native and web)

### Code snippet:
```rust
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("My App");
        if ui.button("Click me").clicked() {
            // handle click
        }
    });
}
```