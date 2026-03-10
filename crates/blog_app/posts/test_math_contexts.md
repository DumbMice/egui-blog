---
title: "Math Formula Context Test"
date: "2026-03-10"
tags: ["test", "math", "rendering", "context"]
---

## Math Formula Context Test

This post tests math formula rendering in various markdown contexts.

### Blockquotes with Math

> This blockquote contains inline math: $E = mc^2$

> Another blockquote with display math: $$ \int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi} $$

> Blockquote with multiple formulas: $x^2$, $y^3$, and $z = \sqrt{x^2 + y^2}$

### Lists with Math

#### Unordered List
- Item with math: $a + b = c$
- Another item: $\frac{1}{2} + \frac{1}{3} = \frac{5}{6}$
- Third item with **bold and math**: **Important formula** $F = ma$

#### Ordered List
1. First item: $x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$
2. Second item: $e^{i\pi} + 1 = 0$
3. Third item with *italic and math*: *Euler's identity* $e^{i\theta} = \cos\theta + i\sin\theta$

### Headings with Math

#### Heading with inline math: $H = -\sum p_i \log p_i$

##### Another heading: $\nabla \cdot \mathbf{E} = \frac{\rho}{\epsilon_0}$

### Bold and Italic with Math

**Bold text with math**: $f(x) = \int_{-\infty}^{\infty} \hat{f}(\xi) e^{2\pi i \xi x} d\xi$

*Italic text with math*: $\lim_{x \to \infty} \left(1 + \frac{1}{x}\right)^x = e$

**Bold and *italic* with math**: **Important *result***: $\sum_{n=1}^{\infty} \frac{1}{n^2} = \frac{\pi^2}{6}$

### Parentheses Issue Test

Test formulas with parentheses:

1. Simple: ($x$)
2. With operators: ($a + b$)
3. Multiple: (($x$))
4. Mixed: text ($x$) text
5. In sentence: The formula ($E = mc^2$) is famous.
6. With punctuation: Consider ($x^2$).
7. Nested: ((($x$)))

### Strikethrough with Math

~~Strikethrough with math: $x = y$~~

~~Multiple formulas: $a$, $b$, $c$~~

### Links with Math (if supported)

[Link with math $x^2$](https://example.com)

### Expected Behavior

1. All math formulas should render as SVGs, not as `(hash.typ)` placeholders
2. Formulas in blockquotes should render correctly
3. Formulas in lists should render correctly  
4. Formulas in headings should render correctly
5. Formulas in bold/italic/strikethrough should render correctly
6. Formulas with parentheses: `($x$)` should render as SVG, not `((hash.typ))`
7. All formulas should maintain proper baseline alignment