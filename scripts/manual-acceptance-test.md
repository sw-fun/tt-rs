# Manual Acceptance Test: Drag and Drop

This document provides a checklist for manually testing the drag-and-drop functionality in tt-rs.

## Prerequisites

1. Start the development server:
   ```bash
   cd /Users/mike/github/sw-fun/tt-rs
   trunk serve --port 1140
   ```

2. Open browser to http://localhost:1140

## Test Cases

### Test 1: Basic Drag Operation

- [ ] **Click and hold** on any yellow number widget (e.g., `+ 5`)
- [ ] **Drag** the widget around the workspace
- [ ] **Verify** the widget follows the mouse cursor smoothly
- [ ] **Verify** the widget becomes slightly transparent (opacity: 0.8) while dragging
- [ ] **Release** the mouse button
- [ ] **Verify** the widget stays at the new position

### Test 2: Drop Target Highlighting

- [ ] **Start dragging** any widget
- [ ] **Verify** all empty box holes show:
  - Dashed blue border
  - Pulsing animation (border color alternates)
  - Light blue background gradient
- [ ] **Stop dragging** (release mouse)
- [ ] **Verify** the visual highlighting disappears from all box holes

### Test 3: Drop Widget into Box Hole

- [ ] **Drag** a number widget (e.g., `+ 1/2`) toward an empty box hole
- [ ] **Position** the widget over the box hole
- [ ] **Release** the mouse button
- [ ] **Verify** the widget snaps into the box hole
- [ ] **Verify** the widget becomes contained within the box

### Test 4: Drop String Widget into Box Hole

- [ ] **Drag** a string widget (e.g., `"Hello"`) toward an empty box hole
- [ ] **Release** over the hole
- [ ] **Verify** the string snaps into place in the box

### Test 5: Multiple Drops

- [ ] Drop multiple different widgets into different box holes
- [ ] **Verify** each widget lands in the correct hole
- [ ] **Verify** no visual artifacts remain after drops

### Test 6: Cancel Drag (Release Outside Drop Target)

- [ ] **Start dragging** a widget
- [ ] **Release** the mouse over empty workspace (not over a box hole)
- [ ] **Verify** the widget stays at the release position (doesn't snap back to origin)
- [ ] **Verify** all drop target highlighting disappears

### Test 7: Rapid Interactions

- [ ] Quickly drag and drop several widgets in succession
- [ ] **Verify** no visual glitches or stuck highlighting
- [ ] **Verify** all widgets respond correctly

## Expected Visual States

| State | Widget Appearance | Box Holes |
|-------|------------------|-----------|
| Idle | Full opacity, normal | Solid light border |
| Dragging | 80% opacity, z-index raised | Dashed blue pulsing border |
| Hovering over target | 80% opacity | Green solid border (if implemented) |
| After drop | Full opacity, in hole | Normal appearance |

## Troubleshooting

If drag-drop doesn't work:
1. Check browser console for errors
2. Ensure `pointer-events: none` is applied to dragged element
3. Verify `body.dragging-active` class is added during drag

If highlighting doesn't appear:
1. Check that `body.dragging-active` class is present during drag
2. Verify CSS is loaded (check DevTools > Elements > Styles)
