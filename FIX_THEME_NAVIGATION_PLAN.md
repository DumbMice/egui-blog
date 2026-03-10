# Fix Theme Toggle Navigation Bug - Implementation Plan

## Problem Statement
When toggling theme (light/dark mode), the app navigates to the home page (first post) instead of staying on the current post.

## Root Cause Analysis
The bug is caused by a **conflict between three state management systems**:

1. **Browser URL routing** (`#/posts/slug`) - Managed by `Router` struct
2. **Local storage persistence** - Auto-saves app state every 30 seconds
3. **In-memory app state** - `selected_post`, `router.current_route`, etc.

**Specific issues identified**:
- Router deserialization sometimes fails → defaults to `Route::Home` (due to `serde(default)`)
- State restoration conflicts: Persisted state vs browser URL precedence unclear
- Side panel UI bug triggers `on_selection(None)` when theme changes
- Theme toggle workarounds (`selected_post_before_theme_change`) treat symptoms, not causes

## Implementation Plan

### Phase 1: Fix Router Serialization (FOUNDATION) ✅ COMPLETED
**Goal**: Ensure router state serializes/deserializes reliably
- [x] **1.1**: Remove `serde(default)` from Router struct
- [x] **1.2**: Add custom deserialization with graceful fallback
- [x] **1.3**: Add serialization version field to Router
- [x] **1.4**: Add comprehensive serialization tests
- [x] **1.5**: Add migration support for existing saved states

### Phase 2: Fix State Synchronization Protocol ✅ COMPLETED
**Goal**: Establish clear precedence between state sources
- [x] **2.1**: Unify state restoration in single method (`restore_state_with_precedence()`)
- [x] **2.2**: Define precedence: Browser URL > Persisted State > Default
- [x] **2.3**: Fix `just_restored` flag logic (clear for any route, not just Home)
- [x] **2.4**: Ensure `route_restored` is properly serialized and used
- [x] **2.5**: Fix timing of `handle_url_changes()` vs state restoration

### Phase 3: Fix Theme Toggle Navigation ✅ COMPLETED
**Goal**: Prevent theme changes from causing navigation
- [x] **3.1**: Remove `selected_post_before_theme_change` workaround
- [x] **3.2**: Fixed root cause (Router serialization + state precedence)
- [x] **3.3**: Theme changes don't trigger navigation with new state management
- [x] **3.4**: `sync_state_to_route()` skips when `just_restored` is true
- [x] **3.5**: All tests pass including regression test

### Phase 4: Fix Web-Specific Issues
**Goal**: Resolve WASM/browser-specific conflicts
- [ ] **4.1**: Fix `pending_url_update` handling (serialize or ensure completion)
- [ ] **4.2**: Improve `handle_url_changes()` timing
- [ ] **4.3**: Ensure browser history integration works with persistence
- [ ] **4.4**: Add web-specific tests

### Phase 5: Add Defensive Programming
**Goal**: Prevent future regressions
- [ ] **5.1**: Add comprehensive logging for state conflicts
- [ ] **5.2**: Add validation checks for router state consistency
- [ ] **5.3**: Add integration tests for full flow
- [ ] **5.4**: Document state management protocol

## Implementation Order
1. **Start with Phase 1** (Router serialization) - Root cause
2. **Then Phase 2** (State synchronization) - Core conflict
3. **Then Phase 3** (Theme toggle) - User-reported bug
4. **Then Phase 4** (Web issues) - Platform-specific
5. **Finally Phase 5** (Defensive programming) - Prevention

## Success Criteria
- [ ] Theme toggle does not cause navigation to home
- [ ] All existing tests pass
- [ ] Router serialization/deserialization is reliable
- [ ] State conflicts are handled gracefully
- [ ] No data loss for existing saved states

## Risk Assessment
- **High**: Changing Router serialization could break existing saved states
- **Medium**: Changing state synchronization could introduce race conditions
- **Low**: Fixing side panel UI is isolated change

## Migration Strategy
1. Add version field to serialized Router data
2. Support both old and new formats during transition
3. Add migration code in `BlogApp::new()`
4. Log migration events for debugging

## Progress Tracking
- **Created**: 2026-03-10
- **Status**: Planning phase complete, ready for implementation

## Notes
- Backward compatibility: Acceptable if existing saved states might need clearing
- Testing approach: Fix specific bug AND redesign state management
- Priority: All aspects important, implement one by one

---
*Last updated: 2026-03-10*