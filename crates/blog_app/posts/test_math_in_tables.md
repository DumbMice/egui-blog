---
title: "Test: Math Formulas in Tables"
date: 2026-03-30
tags: [test, math, tables]
summary: "Test post demonstrating math formula rendering inside markdown tables"
---

# Math Formulas in Tables Test

This post tests the rendering of Typst math formulas inside markdown tables.

## Basic Table with Math

| Operation | Formula | Result |
|-----------|---------|--------|
| Addition | $a + b$ | Sum of a and b |
| Subtraction | $x - y$ | Difference of x and y |
| Multiplication | $p times q$ | Product of p and q |
| Division | $m/n$ | Quotient of m and n |

## Table with Display Math

| Equation Name | Formula | Description |
|---------------|---------|-------------|
| Pythagorean theorem | $ a^2 + b^2 = c^2 $ | Relationship in right triangles |
| Quadratic formula | $ x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a} $ | Solution to quadratic equations |
| Euler's identity | $ e^{i\pi} + 1 = 0 $ | Beautiful mathematical relationship |

## Mixed Content Table

| Cell Type | Example | Notes |
|-----------|---------|-------|
| Text only | Simple text cell | No math here |
| Text with inline math | The value of $\pi$ is approximately 3.14 | Math inside text |
| Multiple math | Calculate $x^2 + y^2$ when $x=3$ and $y=4$ | Two formulas in one cell |
| Display math only | $ \int_{0}^{\infty} e^{-x^2} dx = \frac{\sqrt{\pi}}{2} $ | Integral calculation |

## Table with Math in Headers

| $x$ values | $f(x) = x^2$ | $g(x) = \sqrt{x}$ |
|------------|--------------|-------------------|
| 1 | 1 | 1 |
| 2 | 4 | 1.414 |
| 3 | 9 | 1.732 |
| 4 | 16 | 2 |

## Complex Table Example

| Matrix Operation | Formula | Dimensions |
|------------------|---------|------------|
| Matrix multiplication | $C = AB$ where $C_{ij} = \sum_{k} A_{ik} B_{kj}$ | If $A$ is $m \times n$ and $B$ is $n \times p$, then $C$ is $m \times p$ |
| Determinant | $\det(A) = \sum_{\sigma \in S_n} \operatorname{sgn}(\sigma) \prod_{i=1}^n a_{i,\sigma(i)}$ | For $n \times n$ matrix |
| Eigenvalues | $A\mathbf{v} = \lambda\mathbf{v}$ | $\lambda$ is eigenvalue, $\mathbf{v}$ is eigenvector |

## Empty and Mixed Cells

| Column A | Column B | Column C |
|----------|----------|----------|
| | Empty cell | |
| Cell with $E = mc^2$ | | Just one formula |
| Normal text | Text with $a^2 + b^2$ | More text |

## Expected Behavior

1. **Math formulas should render as SVG images** instead of showing `(hash.typ)`
2. **Inline math** ($formula$) should align properly with text
3. **Display math** ($ formula $) should be centered
4. **Mixed content** (text + math) should render correctly
5. **Table formatting** (borders, spacing, alignment) should be preserved

## Notes

- This test complements the existing `test_math_contexts.md` post
- Tables were previously not processing math formulas correctly
- The fix should make tables work like other markdown elements (lists, blockquotes, etc.)

