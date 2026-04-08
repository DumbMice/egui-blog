# Embeddable UI Components in Blog Posts - Requirements Document

## Overview
This document captures the original vision and requirements for adding interactive UI components inside blog posts in the egui blog application.

## Core Vision
The goal is to enable blog authors to embed **interactive UI components** directly within markdown content, similar to how images are embedded today.

### Key Analogy (User's Insight)
> "In markdown, we can embed images with markdown syntax. The image's data is not present there, but we load its data and then load it as a figure UI component. Likewise, can we use some custom syntax to link some egui-related UI component, such as some plot. Then in compilation or build time, we will know this and generate related UI component in that position."

## Primary Requirements

### 1. **Markdown Integration**
- **Syntax**: Custom markdown syntax for embedding UI components
- **Location**: Components should render inline within the markdown flow
- **Configuration**: Support for component configuration (similar to image alt text or attributes)

### 2. **Component Types** (User's Priority)
1. **Data visualization widgets** - Charts, graphs, interactive diagrams
2. **Custom complex widgets** - Domain-specific interactive tools
3. **Potential 3D rendering** - Mentioned as a possibility ("five involves some 3d rendering maybe?")

### 3. **Build-time Processing**
- Components should be processed at **compilation or build time**
- The system should "know" about components during build and generate appropriate UI
- Similar to the existing math formula system: extract → process → embed → render

### 4. **Technical Approach Considerations**
- **egui_wings**: User identified this as potentially useful for the purpose
- **Rust file linking**: Possibility of linking to Rust files that can generate egui UI components
- **WASM plugins**: Dynamic loading of UI components

### 5. **Architecture Constraints**
- Must work with existing egui blog architecture
- Should follow patterns similar to math formula rendering
- Must support both web (WASM) and native targets
- Should maintain performance and not break existing functionality

## Success Criteria

### Functional Requirements
1. Authors can embed interactive components using simple markdown syntax
2. Components render correctly within blog post content
3. Component state can be interactive (respond to user input)
4. Components work in both web and native versions of the blog
5. Build system detects and processes component definitions

### Non-Functional Requirements
1. **Performance**: Should not significantly impact page load or rendering performance
2. **Security**: Components should be sandboxed appropriately
3. **Maintainability**: System should be extensible for new component types
4. **Developer Experience**: Easy for authors to create and use components

## Open Questions & Decisions Needed

### 1. **Syntax Design**
What markdown syntax should be used?
- Option A: `{{widget:type config}}`
- Option B: `:::widget[type=chart]...:::`
- Option C: HTML-like `<widget type="chart">`
- Option D: Other custom syntax

### 2. **Component Definition**
How are components defined?
- JSON configuration only
- Rust files that implement a trait
- WASM plugins (egui_wings approach)
- Hybrid approach

### 3. **Build System Integration**
How does this integrate with existing build pipeline?
- Extend current `build.rs` for math formulas
- Separate build process
- Runtime loading instead of build-time

### 4. **3D Rendering Priority**
How important is 3D rendering vs 2D components?
- Essential feature for MVP
- Nice-to-have for future
- Not required initially

### 5. **State Management**
How should component state be handled?
- Ephemeral (resets on page reload)
- Persisted in browser storage
- Saved with blog post content

## Context & Background

### Current System (Math Formulas)
The blog already has a sophisticated system for math formulas:
1. **Build-time**: `build.rs` extracts `$formula$` from markdown, generates SVGs
2. **Runtime**: Formulas replaced with `(hash.typ)` placeholders, rendered as images
3. **Caching**: Manifest tracks all formulas, SVGs embedded in binary

### Proposed Parallel System (UI Components)
The vision is to create a similar system for UI components:
1. **Build-time**: Extract component definitions, compile/prepare components
2. **Runtime**: Instantiate and render components at specified positions
3. **Caching**: Component definitions embedded/managed efficiently

## Technical Exploration Notes

### egui_wings Investigation
User identified `egui_wings` (https://github.com/DouglasDwyer/egui_wings) as potentially useful. This crate:
- Facilitates sharing `egui::Context` between host and WASM modules
- Allows WASM plugins to draw UI via the host
- Could enable dynamic loading of UI components

### Existing Architecture
The blog app has:
- `blog_macros` crate for embedding files
- Sophisticated build system for math formulas
- Markdown rendering pipeline with `ParagraphContent` enum
- Support for both web (WASM) and native targets

## Next Steps Required

1. **Decision on approach**: Choose between JSON config, Rust files, or WASM plugins
2. **Syntax finalization**: Determine exact markdown syntax
3. **Architecture design**: How components integrate with existing systems
4. **Implementation planning**: Phased approach vs big bang
5. **Prototyping**: Create proof-of-concept for chosen approach

## User's Original Statements (for reference)

> "I think we should rethink seriously about adding interactive ui inside blog post."

> "basically, in markdown, we can embed image with markdown syntax, right? the image's data is not present there, but we load its data and then load its as a figure ui component. likewise, can we use some custom syntax to link some egui related ui component, such as some plot. then in compilation or build time, we will know this and generate related ui component in that position."

> "maybe https://github.com/DouglasDwyer/egui_wings is useful for this purpose?"

> "I'd like to go straight to egui_wings actually..."

## Document Purpose
This document serves as the authoritative source of requirements for the embeddable UI components feature. It should guide all design and implementation decisions.

---
*Document created based on conversation on April 5, 2026*
*Last updated: April 5, 2026*