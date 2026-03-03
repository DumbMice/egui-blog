---
title: "Memory Limitation of LLM"
date: "2026-03-03"
tags: ["llm", "memory", "ai"]
---

The linearly increasing computation cost has been the most cumbersome problem leaving for scaling llm as a general purpose long-term agent.

Self-attention captures how a sequence of texts attents to itself. Let it be a sentence of _greeting_, a _scientific report_ or a _history of chatting_.

Mathematically, they are all sequences of tokens $[bold(x)_1, dots, bold(x)_T]$, and each token $bold(x)_t$ is itself a $n$-dimensional array.
While using the bold notation $bold(x)_t$ we are actually hiding $n$ dimensions of its _token feature_. To explain full details of self-attention, I would like to expose this _feature dimension_ as index $s$.

We would abuse the tensorial notation for multi-dimensional array, and denote a sequence of tokens simply as $x equiv [bold(x)_1, dots, bold(x)_T]$ composing of components $x_(t s)$ at position index $t$ and feature index $s$.

We can now start talking about how each token attends to each other. When we say $bold(x)_i$ attends to $bold(x)_j$, this defines a directional relationship. This implies we could not use dot product or any symmetric calculation to represents attention because they can only represent bidirectional/symmetric relationship. Self attention adopts a simple method, utilizing **noncommutativity** of matrix multiplication.

Self-attention requires each token to have two **projections**, query $bold(q)_t$ and key $bold(k)_t$. When $bold(x)_i$ tries to attend another $bold(x)_j$, the attention is measured by $bold(q)_i.bold(k)_j$ the dot product of the first's query and the second's key instead of tokens' dot product. 
The simpliest way of obtaining these projection are through two matrices $bold(Q), bold(K)$,
$ bold(q)_t = bold(Q) bold(x)_t,  bold(k)_t = bold(K)bold(x)_t  <=> q_(t s) = Q_ $

For each new token $x_t$, following a sequence of tokens $[x_1, dots, x_(t-1)]$, each self-attention incapsulates the contribution of all preceding tokens to this token,

$ "softmax"() $
