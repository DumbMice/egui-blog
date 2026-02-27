---
title: "Test Typst Math"
date: "2026-02-25"
tags: ["test", "typst", "math"]
---

## Testing Typst Math Rendering

### Inline Math
Here's some inline math: $E = m c^2$ and $1/2 + 1/4 = 3/4$.

### Display Math
Display math should be centered:

$ integral_(-infinity)^infinity e^(-x^2) dif x = sqrt(pi) $

$ sum_(n=1)^infinity 1/n^2 = pi^2/6 $

### More Complex Examples
The quadratic formula:

$ x = (-b +- sqrt(b^2 - 4 a c)) / (2a) $

Matrix example:

$ mat(1, 2; 3, 4) $

### Mixed Content
You can have text with $sum_(i=1)^n i = n(n+1)/2$ and then more text.

### Test Error Reporting
This formula has invalid Typst syntax: $ invalid { syntax } $

### New Test Formula
Let's test a new formula: $a^2 + b^2 = c^2$.