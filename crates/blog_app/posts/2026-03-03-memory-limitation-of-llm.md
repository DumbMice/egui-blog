---
title: "Memory Limitation of LLM"
date: "2026-03-03"
tags: ["llm", "memory", "ai"]
---

The quadratic scaling of computation with sequence length poses a fundamental challenge for deploying LLMs as general-purpose agents requiring long-term context.

This post establishes the mathematical foundations of self-attention, which is essential for understanding memory constraints in transformers.
We'll cover the core mechanics first; a follow-up post will discuss how these fundamentals create memory bottlenecks.

## Introduction to Self-Attention

Self-attention enables each position in a sequence to attend to all other positions, capturing contextual relationships.
This mechanism operates identically whether processing a brief greeting, a detailed scientific report, or an extended conversation history.

Let's start by formalizing how text is represented in transformers.

### Texts as Tokens

Mathematically, they are all sequences of tokens $[bold(x)_1, dots, bold(x)_T]$, and each token $bold(x)_t in RR^(d_"tok")$.
The bold notation $bold(x)_t$ encapsulates $d_"tok"$ dimensions of token features.
To provide a complete mathematical treatment of self-attention,
we employ tensor notation for multi-dimensional arrays, representing a token sequence as $x equiv [bold(x)_1, dots, bold(x)_T]$ with components $lr(x^(mu))_t$ where $1<=t<=T$ indexes positions and $1<=mu<=d_"tok"$ indexes features.

For example, in GPT-3 (175B parameter model), $d_"tok"=12288$ with 96 attention heads, resulting in $d=128$ per-head attention dimension since $d_"tok" = N_"head" d$.

The core innovation of attention is representing directional relationships between tokens.

### Asymmetric Bilinear Form on Tokens

When $bold(x)_i$ attends to $bold(x)_j$, this establishes a **directional** relationship.
Since symmetric operations like dot products cannot capture directionality, self-attention employs **a learnable asymmetric bilinear form** to represent these directed connections.

Self-attention computes two projections for each token:

- a _query_ vector $bold(q)_t in RR^d$
- and a _key_ vector $bold(k)_t in RR^d$.

The attention from $bold(x)_i$ to $bold(x)_j$ is determined by the dot product $bold(q)_i dot.op bold(k)_j$, where the query "asks" about information and the key "answers" with relevance.

These projections are obtained through linear transformations using learnable matrices $Q$ and $K$, mapping from $RR^(d_"tok")$ to $RR^(d)$. Their tensor representations are:

$ bold(q)_t = Q bold(x)_t  <=> lr(q^(alpha))_t = lr(Q^(alpha))_(mu) lr(x^(mu))_t \  bold(k)_t = K bold(x)_t <=> lr(k^(alpha))_t = lr(K^(alpha))_(mu) lr(x^(mu))_t, $

where a sum over $mu$, i.e. $sum_(mu=1)^(d_"tok")$, is implied when it appears twice, one time as an upper index and the other time as a lower index.
This contraction of repeated indices, often called _Einstein notation_, is commonly used in linear algebra and differential geometry and is used throughout this post.

**Euclidean Geometry Assumption**: In this post, we work in Euclidean space where the metric tensor is the identity matrix $delta_(alpha beta)$. Consequently, superscript and subscript indices can be interchanged freely: $a^(alpha) = a_(alpha)$ and $K^(alpha)_(mu) = K_(alpha mu)$. This simplifies notation while maintaining mathematical rigor for our purposes.

With this notation established, we can now formalize the directional relationship between tokens. To ensure this relation is directional, we define the _query score_ between two tokens $r(x_i,x_j) in RR$ as

$ r(x_i, x_j) &equiv bold(q)_i dot.op bold(k)_j = lr(q^(alpha))_i lr(k_(alpha))_j = (lr(Q^(alpha))_(mu) lr(x^(mu))_i) (lr(K_(alpha nu)) lr(x^(nu))_j) = lr(x^(mu))_i ( lr(Q^(alpha))_(mu) lr(K_(alpha nu)) ) lr(x^(nu))_j \  &= lr(x^(mu))_i lr(R_(mu nu)) lr(x^(nu))_j = bold(x)^tack.b_i R bold(x)_j $

where  $R$ could be represented as

$ R= Q^tack.b K <=> R_(mu nu) = lr(Q^tack.b_(" "mu))^alpha lr(K_(alpha nu)) = lr(Q^(alpha))_(mu) lr(K_(alpha nu)). $

The query score function $r(dot.op, dot.op)$ constitutes a _bilinear form_ $RR^d times RR^d -> RR$ with matrix $R$.
Since $Q$ and $K$ are independent learnable matrices, $R$ is generally _asymmetric_, endowing the bilinear form with directional properties.

Furthermore, learning a low-rank matrix as $Q^tack.b K$ instead of a full $d_"tok" times d_"tok"$ matrix is more memory and computation efficient for $d << d_"tok"$.
After we discuss multi-head attention, this point is reiterated since each attention head focuses on only one "aspect", eliminating the need for a full $d_"tok" times d_"tok"$ matrix.

Before proceeding further, let's summarize the notation conventions we've established so far. This reference table will help clarify the symbols used in the upcoming derivations.

### Notation Reference

For clarity throughout the mathematical derivations, we adopt the following notation:

| Symbol | Meaning | Range/Dimension |
|--------|---------|-----------------|
| $T$ | Sequence length | - |
| $d_"tok"$ | Token embedding dimension | - |
| $d$ | Per-head attention dimension | - |
| $N_"head"$ | Number of attention heads | - |
| $lr(x^(mu))_t$ | Token feature matrix | $1<=mu<=d_"tok"$ : token feature, $1<=t<=T$ : position |
| $lr(q^(alpha))_t$ | Query vector components | $1<=alpha<=d$ : attention feature, $1<=t<=T$ : position |
| $lr(k^(alpha))_t$ | Key vector components | $1<=alpha<=d$ : attention feature, $1<=t<=T$ : position |
| $lr(v^(alpha))_t$ | Value vector components | $1<=alpha<=d$ : attention feature, $1<=t<=T$ : position |
| $lr(Q^(alpha))_(mu)$ | Query projection matrix | $1<=alpha<=d$ : attention feature, $1<=mu<=d_"tok"$ : token feature |
| $lr(K^(alpha))_(mu)$ | Key projection matrix | $1<=alpha<=d$ : attention feature, $1<=mu<=d_"tok"$ : token feature |
| $lr(V^(alpha))_(mu)$ | Value projection matrix | $1<=alpha<=d$ : attention feature, $1<=mu<=d_"tok"$ : token feature |

**Index Convention**: Greek letters index features: early Greek ($alpha, beta, gamma$) for attention feature dimension $d$, later Greek ($mu, nu, rho, sigma, tau$) for token feature dimension $d_"tok"$. Latin letters ($i, j, t, s$) index positions.

**Einstein Summation**: Repeated indices (one upper, one lower) imply summation. Index placement can be swapped for summation: $a^(mu) b_(mu) = a_(mu) b^(mu)$. For dot products: $bold(q)_i dot.op bold(k)_j= lr(q^(alpha))_i lr(k_(alpha))_j$ sums over $alpha$.

With query scores defined, we need to convert them into a probability distribution over tokens. This conversion enables the interpretation of self-attention as computing expectations over an ensemble of tokens.

### Ensemble on Tokens

The _query score_ $r(x_i, x_j)$ measures directional relationships between tokens. To use these scores for attention, we convert them into a probability distribution $bold(p)_i$ over tokens $x_j$. This allows us to interpret self-attention as computing the expectation of a linear operator over a distribution induced by query scores.

This ensemble perspective provides valuable intuition for understanding computational costs and potential optimizations.

#### Connection to Statistical Physics

This formulation connects to statistical physics through the Boltzmann distribution, which gives the probability $p_i$ of finding a system at temperature $T$ in state $i$ with energy $E_i$:

$ p_i = ("exp"(-E_i / (k_B T)))/Z $

where $Z equiv sum_i "exp"(-E_i/(k_B T))$ is the partition function, which normalizes the distribution, and $k_B$ is the Boltzmann constant.

In neural networks, the corresponding operation is the softmax function:

$ "softmax"( bold(x))_i =  "exp"(x_i)/(sum_i "exp"(x_i)) $

which maps an array of values $bold(x)$ to a probability distribution $"softmax"(bold(x))$.
The softmax function can be viewed as a Boltzmann distribution with $x_i -> -E_i \/ (k_B T)$.

The exponent must be dimensionless; otherwise, the Taylor expansion $e^x = sum_(n=0)^infinity (x^n)/(n!)$ would sum quantities with incompatible dimensions.
In the Boltzmann distribution, both $E_i$ and $k_B T$ have energy dimensions, ensuring a dimensionless ratio.

The denominator, analogous to temperature in thermodynamics, controls distribution concentration.
High temperatures produce uniform distributions, while low temperatures concentrate probability at extreme values.
Temperature establishes an energy scale; scaling both energies and temperature proportionally leaves the distribution invariant.

#### Normalized Query Score

Returning to attention, we aim to construct a probability distribution from query scores that remains invariant to representation choices like $d$ and $d_"tok"$.
We assume token and matrix entries are independent random variables with specified first and second moments:

$ EE[x^(mu)] = 0, "Var"[x^(mu)]=1 \ EE[lr(Q^(alpha))_(mu)] = 0, "Var"[lr(Q^(alpha))_(mu)] = 1/d_"tok" \ EE[lr(K^(alpha))_(mu)] = 0, "Var"[lr(K^(alpha))_(mu)] = 1/d_"tok". $

For entries in the bilinear matrix $lr(R_(mu nu)) = lr(Q^(alpha))_(mu) lr(K_(alpha nu))$, we compute:

$ EE[lr(R_(mu nu))] = EE[lr(Q^(alpha))_(mu) lr(K_(alpha nu))] = 0 \ "Var"[lr(R_(mu nu))] = sum_(alpha=1)^d "Var"[lr(Q^(alpha))_(mu)] "Var"[lr(K_(alpha nu))] = d (1/d_"tok") (1/d_"tok") = d/d_"tok"^2. $

The query score $r_(i j) equiv r(bold(x)_i, bold(x)_j) = lr(x^(mu))_i lr(R_(mu nu)) lr(x^(nu))_j$ hence has the mean and variance,

$ EE[r_(i j)] = EE[lr(x^(mu))_i lr(R_(mu nu)) lr(x^(nu))_j] = 0 \ "Var"[r_(i j)] = sum_(mu,nu=1)^(d_"tok") "Var"[lr(x^(mu))_i] "Var"[lr(R_(mu nu))] "Var"[lr(x^(nu))_j] = d_"tok"^2 (d/d_"tok"^2) = d. $

Consequently, the bilinear form's variance scales linearly with projection dimension $d$, an implementation-dependent hyperparameter.

To get rid of this dependency, we define a **normalized bilinear matrix**,

$ lr(hat(R)_(mu nu)) = lr(R_(mu nu)) / sqrt(d), $

and the **normalized query score** $hat(r)_(i j)$ as

$ hat(r)_(i j) = hat(r)(bold(x)_i, bold(x)_j) = lr(x^(mu))_i lr(hat(R)_(mu nu)) lr(x^(nu))_j = (bold(x)^(tack.b)_i R bold(x)_j) / sqrt(d) $,

which is a normalized score with mean and variance independent of $d$,

$ EE[hat(r)_(i j)] =  0 \ "Var"[hat(r)_(i j)] = 1 $

and this $sqrt(d)$ can be interpreted either as a dimensional quantity that renders $hat(r)$ dimensionless or as a temperature that preserves the peakedness of the distribution derived from $hat(r)$.

The $sqrt(d)$ scaling factor is essential for stable training - without it, attention scores would diverge with increasing model dimensions.

#### Self-Attention as Ensemble on Tokens

Given a query token $bold(x)_i$, we can use the normalized query score to define a distribution (or an ensemble), $bold(p)_i$ over attended tokens $bold(x)_j$,

$ (bold(p)_i)_j = ("exp"(hat(r)_(i j)))/(sum_j "exp"(hat(r)_(i j))) = ("exp"(hat(r)_(i j)))/(Z_i), $

where $Z_i$ is the partition function.
This can also be written as a softmax function if we define $(bold(hat(r))_i)_j equiv hat(r)_(i j)$,

$ bold(p)_i = "softmax"(bold(hat(r))_i). $

With this probability distribution, we can define the expected value of any function $O(dot.op)$ of $x_j$ under $bold(p)_i$ as

$ EE_(x_j tilde bold(p)_i)[O] equiv sum_j (bold(p)_i)_j O(x_j). $

This expectation formalism provides a powerful framework for understanding self-attention.

While query scores $hat(r)_(i j)$ determine attention allocation through $bold(p)_i$, the actual information extracted from attended tokens comes from a third projection: the _value vector_ $bold(v)_j in RR^(d)$,

$ bold(v)_j = V bold(bold(x)_j), $

where $V$ is the third matrix of linear transformation $RR^(d_"tok") -> RR^d$.

The value projection $V$ extracts "values" from the attended token $bold(x)_j$.
Thus, $Q$ and $K$ govern _where_ to attend, while $V$ controls _what_ to extract from the attended tokens.

The aggregated value from attended tokens equals the expectation of the value projection:

$ EE_(x_j tilde bold(p)_i)[V] = sum_j (bold(p)_i)_j V bold(x_j) $

where $V$ denotes both the linear projection (LHS) and its matrix representation (RHS).

**Key Insight**: Self-attention performs three operations:

1. **Score computation**: Calculate directional relationships ($Q,K$)
2. **Distribution formation**: Convert scores to probabilities (softmax)
3. **Value extraction**: Weighted sum of transformed tokens ($V$)

Thus, _self-attention computes the expectation of the value projection over a distribution induced by normalized query scores_.

The self-attention output $y equiv "Attn"(x) = [bold(y)_1, dots, bold(y)_T]$ with features $bold(y)_t in RR^d$ is defined as:

$ bold(y)_t equiv EE_(x_j tilde bold(p)_t)[V] = sum_(j=1)^T (bold(p)_t)_j V bold(x)_j. $

Expanding in index notation with components $lr(y^(alpha))_t$:

$  lr(y^(alpha))_t = sum_(j=1)^T ("exp"(hat(r)_(t j)))/(Z_t) lr(V^(alpha))_(mu) lr(x^(mu))_j =  (sum_(j=1)^T "exp"(lr(x^(mu))_t lr(hat(R)_(mu nu)) lr(x^(nu))_j) lr(V^(alpha))_(mu) lr(x^(mu))_j) /(sum_(i=1)^T "exp"(lr(x^(rho))_t lr(hat(R)_(rho sigma)) lr(x^(sigma))_i)). $

This equation encapsulates the complete self-attention operation.
The summation over $j$ reveals the $O(T)$ computational complexity of each individual token underlying memory constraints.

#### Multi-head Self-attention

With self-attention, we can extract information $bold(y)_i$ from other tokens. In practice, transformers use _multi-head self-attention_, which employs $N_"head"$ parallel self-attention operations on $x$.

While multi-headed attention increases model capacity by allowing different heads to focus on different aspects of relationships, it operates in parallel and does not change the fundamental $O(T)$ per-token computational complexity that creates memory bottlenecks.

For each head $1<=h<=N_"head"$, there is a set of learnable parameters ${Q_h, K_h, V_h}$.
Each head defines a distinct relationship or focuses on one aspect among tokens.
Therefore, we obtain $N_"head"$ outputs for each head ${bold(y)^h_i}$.
These outputs are concatenated along the feature dimension into a single output with dimension $N_"head" d$.
In practice, it is often chosen such that $d_"tok" = N_"head" d$

Typically, an output projection matrix $W_o in RR^(N_"head" d) times RR^(N_"head" d)$ combines features from different heads. While this mixing operation adds model capacity, its $O(d^2)$ computational cost is independent of sequence length $T$ and thus does not contribute to the quadratic scaling that creates memory bottlenecks.

### Causal Structure on Tokens

Decoder-only autoregressive transformers incorporate causal masking into self-attention.
During inference, models generate tokens sequentially using preceding context, necessitating that training also restricts attention to historical tokens.

This causal structure modifies the probability distribution such that a token cannot attend to future tokens or itself:

$ (bold(p)_i)_j = 0 quad "for" quad j >= i $

For causal self-attention, the output for token $i$ becomes:

$ bold(y)_i = EE_(x_j tilde bold(p)_i)[V] = sum_(j=1)^(i-1) (bold(p)_i)_j V bold(x)_j $

with the corresponding index notation:

$ lr(y^(alpha))_i = sum_(j=1)^(i-1) ("exp"(hat(r)_(i j)))/(Z_i) lr(V^(alpha))_(mu) lr(x^(mu))_j $

where the partition function $Z_i$ now sums only over $j < i$:

$ Z_i = sum_(j=1)^(i-1) "exp"(hat(r)_(i j)) $

This causal restriction can be implemented by:

- Restricting summation range to $j < i$
- Applying a causal mask $M_(i j) = bold(1)_(i > j)$
- Setting query scores $r_(i j) = -infinity$ for $j >= i$

**Memory Implications**: Causal masking produces triangular attention patterns but preserves $O(T)$ complexity for each token. Each token attends to all predecessors, demanding quadratic memory for attention matrices as sequence length grows.

This quadratic scaling leads directly to the fundamental memory bottleneck in transformer architectures.

## Memory Bottlenecks in Transformer Attention

The $O(T^2)$ memory requirement of self-attention creates a fundamental limitation for transformer-based models. Let's analyze the scaling laws that constrain practical deployment and explore why this quadratic dependence emerges from the mathematical formulation we've developed.

### Quadratic Scaling of Attention

From the self-attention equation:

$ lr(y^(alpha))_t = sum_(j=1)^T ("exp"(lr(x^(mu))_t lr(hat(R)_(mu nu)) lr(x^(nu))_j))/(sum_(i=1)^T "exp"(lr(x^(rho))_t lr(hat(R)_(rho sigma)) lr(x^(sigma))_i)) lr(V^(alpha))_(mu) lr(x^(mu))_j $

we observe two critical scaling behaviors:

1. **Computation per token**: $O(T)$ operations for each $t$
2. **Total computation**: $O(T^2)$ for the full sequence
3. **Memory for attention scores**: $O(T^2)$ to store all $hat(r)_(i j)$

For causal attention (decoder-only), the scaling is triangular: $O(T^2/2)$, but still quadratic in asymptotic analysis.

### Concrete Memory Requirements

Consider a transformer with:

- Sequence length: $T$
- Attention dimension: $d$
- Number of heads: $N_"head"$
- Batch size: $B$

The memory consumption breaks down as:

| Component | Memory | Scaling |
|-----------|--------|---------|
| Attention scores | $B N_"head" T^2$ (float32) | $O(T^2)$ |
| Key-Value cache | $B T d_"tok"$ (per layer) | $O(T)$ |
| Gradient storage | $2 times$ forward pass | $O(T^2)$ |

For GPT-3 parameters ($d_"tok"=12288$, $N_"head"=96$):

- $T=2048$: ~$96 times 2048^2 times 4" bytes" approx 1.6$ GB per batch
- $T=8192$: ~$96 times 8192^2 times 4" bytes" approx 25.8$ GB per batch

This quadratic growth quickly exhausts GPU memory, limiting context length.

### Mitigation Strategies

#### 1. KV Caching

During autoregressive generation, keys and values from previous tokens can be cached:

- Memory: $O(T)$ instead of recomputing $O(T^2)$
- Trade-off: Still requires storing growing cache
- Implementation: Standard in production systems

#### 2. Sparse Attention

Limit attention to a subset of tokens:

- **Local attention**: Attend to nearby tokens (sliding window)
- **Strided attention**: Attend at regular intervals  
- **Global attention**: Mix local with few global tokens
- Examples: Longformer, BigBird, Sparse Transformer

#### 3. Linear-Time Approximations

Reformulate attention to avoid quadratic computation:

- **Linear Transformers**: Replace softmax with kernel feature map
- **Performer**: Use random Fourier features for approximation
- **Linformer**: Low-rank projection of attention matrix
- **FlashAttention**: IO-aware exact attention with tiling

#### 4. Memory-Efficient Attention

Optimize memory access patterns:

- **FlashAttention**: Reduces HBM accesses via tiling
- **Memory-efficient attention**: Recomputation during backward pass
- **Checkpointing**: Store only subset of activations

### Practical Implications

The memory bottleneck manifests in several ways:

1. **Context window limits**: Most models limited to 2K-8K tokens
2. **Batch size constraints**: Larger $T$ forces smaller $B$
3. **Training cost**: $O(T^2)$ scaling makes long-context training expensive
4. **Inference latency**: Memory bandwidth limits throughput

### Future Directions

Emerging approaches address these limitations:

- **State-space models**: Mamba, RWKV replace attention with SSMs
- **Recurrent architectures**: Compress history into fixed-size state
- **Hybrid models**: Mix attention with efficient alternatives
- **Hardware-aware designs**: Co-design algorithms with memory hierarchy

### Conclusion

The quadratic memory scaling of self-attention poses a fundamental constraint on transformer-based LLMs. While the mathematical formulation provides expressive power, the $O(T^2)$ complexity limits practical context lengths. Understanding these foundations is crucial for developing next-generation architectures that balance expressive capacity with computational efficiency.

The trade-off between modeling power and memory constraints remains an active research frontier, with innovations in sparse attention, linear approximations, and alternative architectures gradually expanding the feasible context window while preserving the relational reasoning capabilities that make transformers effective.
