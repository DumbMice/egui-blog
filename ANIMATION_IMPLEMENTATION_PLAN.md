# Panel Focus Animation Implementation Plan

## Overview
Replace the current ugly blue border with an animated focus indicator featuring:
1. Quick flash on focus change
2. Sustained gentle pulse while focused
3. Adjustable parameters via debug configuration window
4. Modular, non-spaghetti code architecture

## Architecture

### Module Structure
```
src/animation/
├── mod.rs          # Public API
├── config.rs       # Configuration parameters
├── state.rs        # Animation state machine
└── renderer.rs     # Drawing logic
```

### Integration Points
1. `BlogApp` struct: Add `FocusAnimationState` field
2. `DebugState` struct: Add `FocusAnimationConfig` field
3. `layout.rs`: Replace blue border with animation renderer
4. `components.rs`: Add debug menu entry
5. `debug_windows/mod.rs`: Add configuration window

## Implementation Steps

### Phase 1: Create Animation Module
1. ✅ Create `src/animation/config.rs` - Configuration struct with defaults
2. Create `src/animation/state.rs` - Animation state machine
3. Create `src/animation/renderer.rs` - Drawing logic
4. Create `src/animation/mod.rs` - Public API

### Phase 2: Integrate with BlogApp
1. Add `FocusAnimationState` to `BlogApp` struct
2. Initialize in `BlogApp::default()` and `BlogApp::new()`
3. Add update logic in `ui()` method
4. Handle focus change events

### Phase 3: Update UI Layout
1. Modify `side_panel()` to use animation renderer
2. Modify `main_content()` to use animation renderer
3. Pass animation state via app context
4. Remove old blue border drawing code

### Phase 4: Add Debug Configuration
1. Add `animation_config` to `DebugState`
2. Create configuration window in debug_windows
3. Add to `debug_menu()` in components.rs
4. Register window in main `ui()` method

## Key Design Decisions

### 1. State Access Pattern
- Animation state stored in `BlogApp` struct
- Configuration stored in `DebugState` (debug builds only)
- Accessed via app context passed to layout functions

### 2. Parameter Validation
- No runtime validation needed
- Debug UI provides constrained sliders
- Defaults compiled into binary

### 3. Testing Strategy
- Manual testing for parameter tuning
- Basic function correctness ensured
- No unit tests for animation logic

### 4. Documentation
- All public APIs documented
- Configuration parameters explained
- Usage examples in doc comments

## Default Parameters
- `intensity: 0.6` (balanced)
- `flash_duration_ms: 150`
- `fade_duration_ms: 350`
- `pulse_cycle_ms: 2000`
- `flash_max_opacity: 0.3`
- `pulse_min_opacity: 0.05`
- `pulse_max_opacity: 0.1`
- `enable_background_tint: true`
- `enable_glow_border: true`
- `glow_thickness: 1.0`

## Animation Phases
1. **Flash** (0-150ms): Quick bright highlight
2. **Fade to Pulse** (150-500ms): Transition to sustained pulse
3. **Pulse** (500ms+): Gentle breathing effect

## Visual Effects
- Background tint (subtle color overlay)
- Glow border (soft multi-layer glow)
- Linear interpolation for simplicity

## Next Steps
1. Complete animation module creation
2. Integrate with existing codebase
3. Test with default parameters
4. Add debug configuration UI