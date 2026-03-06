---
title: "Math Alignment Test"
date: "2026-03-05"
tags: ["test", "math", "alignment"]
---

## Math Baseline Alignment Test

This post tests the new baseline alignment for inline math formulas.

### Simple Formulas
Text before $x^2$ text after.

Text before $E=mc^2$ text after.

### Complex Fractions
Text before $frac{a}{b}$ text after.

Text before $frac{1}{2} + frac{1}{3} = frac{5}{6}$ text after.

### Tall Formulas
Text before $sum_{i=1}^n i$ text after.

Text before $int_0^1 x^2 dx$ text after.

### Mixed with Text Styles
**Bold text** $x^2$ *italic text*.

`Code text` $y = mx + b$ normal text.

### Multiple Inline Formulas
$x$, $y$, $z$ in sequence.

$a^2 + b^2 = c^2$ and $e^{i pi} + 1 = 0$ are famous formulas.

### Display Math (should remain centered)
$$
\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}
$$

$$
\frac{d}{dx} \left( \frac{1}{1 + e^{-x}} \right) = \frac{e^{-x}}{(1 + e^{-x})^2}
$$

### Test with Different Font Sizes
This is normal text $x^2$ with inline math.

**This is bold text** $frac{a}{b}$ with inline math.

*This is italic text* $sum_{i=1}^n i$ with inline math.

### Edge Cases
Empty formula (should not render): $$ (empty)

Formula with spaces: $  x + y  $ (spaces should be trimmed)

### Tall SVG Handling
Tall formulas like $sum_{i=1}^n i$ should not disrupt line spacing. If an SVG is taller than the text height, baseline offset is discarded and the image may be scaled down.

Test with tall summation: $sum_{i=1}^{100} i^2$

Test with integral: $int_0^{2pi} sin(x) dx$

## Expected Behavior
1. Inline math should align with text baseline (red line in debug mode)
2. SVG baseline (green line) should align with text baseline
3. Tall SVGs should not disrupt line spacing (offset discarded/scaled)
4. Display math should remain centered
5. Formulas should render crisply with proper spacing