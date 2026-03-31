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

Mathematically, they are all sequences of tokens $[bold(x)_1, dots, bold(x)_T]$, and each token $bold(x)_t in RR^(d_("tok"))$.
The bold notation $bold(x)_t$ encapsulates $d_"tok"$ dimensions of token features.
To provide a complete mathematical treatment of self-attention,
we employ tensor notation for multi-dimensional arrays, representing a token sequence as $x equiv [bold(x)_1, dots, bold(x)_T]$ with components $x_(mu t)$ where $1<=t<=T$ indexes positions and $1<=mu<=d_"tok"$ indexes features.

For example, in GPT-3 (175B parameter model), $d_"tok"=12288$ with 96 attention heads, resulting in $d=128$ per-head attention dimension since $d_"tok" = N_"head" dot.c d$.

### Notation Convention

To maintain clarity throughout the mathematical derivations, we adopt the following notation:

| Symbol | Meaning | Range/Dimension |
|--------|---------|-----------------|
| $T$ | Sequence length | $1 <= t <= T$ |
| $d_"tok"$ | Token embedding dimension | $1 <= mu <= d_"tok"$ |
| $d$ | Per-head attention dimension | $1 <= alpha <= d$ |
| $N_"head"$ | Number of attention heads | $1 <= h <= N_"head"$ |
| $x^(mu)_t$ | Token feature matrix | $mu$ (superscript): feature, $t$ (subscript): position |
| $q^(alpha)_t$ | Query vector components | $alpha$ (superscript): attention feature, $t$ (subscript): position |
| $k^(alpha)_t$ | Key vector components | $alpha$ (superscript): attention feature, $t$ (subscript): position |
| $v^(alpha)_t$ | Value vector components | $alpha$ (superscript): attention feature, $t$ (subscript): position |
| $Q^(alpha)_(mu)$ | Query projection matrix | $alpha$ (superscript): output, $mu$ (subscript): input |
| $K_(alpha)^(mu)$ | Key projection matrix | $alpha$ (subscript): output, $mu$ (superscript): input |
| $V^(alpha)_(mu)$ | Value projection matrix | $alpha$ (superscript): output, $mu$ (subscript): input |

**Index Convention**: Greek letters index features: early Greek ($alpha, beta, gamma$) for attention dimension $d$, later Greek ($mu, nu, rho, sigma, tau$) for token dimension $d_"tok"$. Latin letters ($i, j, t, s$) index positions. Feature indices are superscripts: $x^(mu)_t$ is the $mu$-th feature at position $t$.

**Einstein Summation**: Repeated indices (one upper, one lower) imply summation. Index placement can be swapped for summation: $a^(mu) b_(mu) = a_(mu) b^(mu)$. For dot products: $bold(q)_i dot.c bold(k)_j = q^(alpha)_i k_(alpha j)$ sums over $alpha$.

The core innovation of attention is representing directional relationships between tokens.

### Asymmetric Bilinear Form on Tokens

When $bold(x)_i$ attends to $bold(x)_j$, this establishes a **directional** relationship.
Since symmetric operations like dot products cannot capture directionality, self-attention employs **a learnable asymmetric bilinear form** to represent these directed connections.

Self-attention computes two projections for each token:

- a _query_ vector $bold(q)_t in RR^d$
- and a _key_ vector $bold(k)_t in RR^d$.

The attention from $bold(x)_i$ to $bold(x)_j$ is determined by the dot product $bold(q)_i dot.c bold(k)_j$, where the query "asks" about information and the key "answers" with relevance.

These projections are obtained through linear transformations using learnable matrices $Q$ and $K$, mapping from $RR^(d_"tok")$ to $RR^(d)$. Their tensor representations are:

$ bold(q)_t = Q bold(x)_t  <=> q^(alpha)_t = Q^(alpha)_(mu) x^(mu)_t \  bold(k)_t = K bold(x)_t <=> k_(alpha)_t = K_(alpha)^(mu) x^(mu)_t, $

where a sum over $nu$, i.e. $sum_(nu=1)^(d_"tok")$, is implied when it appears twice, one time as an upper index and the other time as a lower index.
This contraction of repeated indices, often called _Einstein notation_, is commonly used in linear algebra and differential geometry and is used throughout this post.

**Notation Clarification**: The index $mu$ serves dual roles: as a feature index for tokens $x_(mu t)$ with dimension $d_"tok"$  and for query-key pairs $q_(mu t)$, $k_(mu t)$ with dimension $d$.
Context determines which dimension applies, and indices maintain consistent meaning within each expression.

To ensure this relation is directional, we define the _query score_ between two tokens $r(x_i,x_j) in RR$ as

$ r(x_i, x_j) equiv bold(q)_i dot.c bold(k)_j = q^(alpha)_i k_(alpha j) = Q^(alpha)_(mu) x^(mu)_i K_(alpha)^(nu) x^(nu)_j = x^(mu)_i ( Q^(alpha)_(mu) K_(alpha)^(nu) ) x^(nu)_j = x^(mu)_i R_(mu)^(nu) x^(nu)_j = bold(x)^tack.b_i R bold(x)_j $

where  $R$ could be represented as

$ R= Q^tack.b K <=> R^(nu tau) = Q^(mu nu) lr(K_mu)^tau = Q^( tack.b nu mu)  lr(K_mu)^tau. $

The query score function $r(dot.c, dot.c)$ constitutes a _bilinear form_ $RR^d times RR^d -> RR$ with matrix $R$.
Since $Q$ and $K$ are independent learnable matrices, $R$ is generally _asymmetric_, endowing the bilinear form with directional properties.

Furthermore, learning a low-rank matrix as $Q^tack.b K$ instead of a full $d_"tok" times d_"tok"$ matrix is more memory and computation efficient for $d < d_"tok"$.
After we discuss multi-head attention, this point is reiterated since each attention head focuses on only one "aspect", eliminating the need for a full $d_"tok" times d_"tok"$ matrix.

With query scores defined, we need to convert them into a probability distribution.

### Ensemble on Tokens

With the simple notion of _query score_ as a bilinear score between tokens, we can interpret self-attention as the expectation of a linear operator over a distribution or ensemble induced by query scores.
This interpretation helps us rethink the computational cost and how it could potentially be reduced.

#### Detour to Physics

This formulation connects to statistical physics through the Boltzmann distribution, which gives the probability $p_i$ of finding a system at temperature $T$ in state $i$ with energy $E_i$:

$ p_i = (e^(-E_i / (k_B T)))/Z $

where $Z equiv sum_i e^(-E_i/(k_B T))$ is the partition function, which normalizes the distribution, and $k_B$ is the Boltzmann constant.

Similar exponential weighting appears in quantum mechanics, where each path $bold(x)$ contributes a phase factor in the path integral formulation:

$ phi[bold(x)] = e^((i S[bold(x)])/hbar) $

where $S[bold(x)] equiv  integral_(t_i)^(t_f) L(bold(x), dot(bold(x))) dif t$ is the integral of Lagrangian along the path $bold(x)$ from $t_i$ to $t_f$.

In neural networks, the corresponding operation is the softmax function:

$ "softmax"( bold(x))_i =  e^(x_i)/(sum_i e^(x_i)) $

which maps an array of values $bold(x)$ to a probability distribution $"softmax"(bold(x))$.
The softmax function can be viewed as a Boltzmann distribution with $x_i -> (-E_i)/(k_B T)$.

The exponent must be dimensionless; otherwise, the Taylor expansion $e^x = sum_(n=0)^infinity (x^n)/(n!)$ would sum quantities with incompatible dimensions.
In the Boltzmann distribution, both $E_i$ and $k_B T$ have energy dimensions, while in path integrals, $A$ and $hbar$ share dimensions of energy × time, ensuring dimensionless ratios.

The denominator, analogous to temperature in thermodynamics, controls distribution concentration.
High temperatures produce uniform distributions, while low temperatures concentrate probability at extreme values.
Temperature establishes an energy scale; scaling both energies and temperature proportionally leaves the distribution invariant.

#### Normalized Query Score

Returning to attention, we aim to construct a probability distribution from query scores that remains invariant to representation choices like $d$ and $d_"tok"$.
We assume token and matrix entries are independent random variables with specified first and second moments:

$ EE[x^(mu)] = 0, "Var"[x^(mu)]=1 \ EE[Q^(alpha)_(mu)] = 0, "Var"[Q^(alpha)_(mu)] = 1/d_"tok" \ EE[K_(alpha)^(mu)] = 0, "Var"[K_(alpha)^(mu)] = 1/d_"tok". $

For entries in the bilinear matrix $R_(mu)^(nu) = Q^(alpha)_(mu) K_(alpha)^(nu)$, we compute:

$ EE[R_(mu)^(nu)] = EE[Q^(alpha)_(mu) K_(alpha)^(nu)] = 0 \ "Var"[R_(mu)^(nu)] = sum_(alpha=1)^d "Var"[Q^(alpha)_(mu)] dot "Var"[K_(alpha)^(nu)] = d dot (1/d_"tok") dot (1/d_"tok") = d/d_"tok"^2. $

The query score $r_(i j) equiv r(bold(x)_i, bold(x)_j) = x^(mu)_i R_(mu)^(nu) x^(nu)_j$ hence has the mean and variance,

$ EE[r_(i j)] = EE[x^(mu)_i R_(mu)^(nu) x^(nu)_j] = 0 \ "Var"[r_(i j)] = sum_(mu,nu=1)^(d_"tok") "Var"[x^(mu)_i] dot "Var"[R_(mu)^(nu)] dot "Var"[x^(nu)_j] = d_"tok"^2 dot 1 dot (d/d_"tok"^2) dot 1 = d. $

Consequently, the bilinear form's variance scales linearly with projection dimension $d$, an implementation-dependent hyperparameter.

To get rid of this dependency, we define a normalized bilinear matrix $hat(R)_(mu)^(nu) = R_(mu)^(nu) / sqrt(d)$ and the corresponding normalized query score $hat(r)_(i j)$ as

$ hat(r)_(i j) = hat(r)(bold(x)_i, bold(x)_j) = x^(mu)_i hat(R)_(mu)^(nu) x^(nu)_j = (bold(x)^(tack.b)_i R bold(x)_j) / sqrt(d) $,

which is a normalized score with mean and variance independent of $d$,

$ EE[hat(r)_(i j)] =  0 \ "Var"[hat(r)_(i j)] = 1 $

and this $sqrt(d)$ can be interpreted either as a dimensional quantity that renders $hat(r)$ dimensionless or as a temperature that preserves the peakedness of the distribution derived from $hat(r)$.

The $sqrt(d)$ scaling factor is essential for stable training - without it, attention scores would diverge with increasing model dimensions.

#### Self-Attention as Ensemble on Tokens

Given a query token $bold(x)_i$, we can use the normalized query score to define a distribution (or an ensemble), $bold(p)_i$ over attended tokens $bold(x)_j$,

$ (bold(p)_i)_j = (e^(hat(r)_(i j)))/(sum_j e^(hat(r)_(i j))) = (e^(hat(r)_(i j)))/(Z_i), $

where $Z_i$ is the partition function.
This can also be written as a softmax function if we define $(bold(hat(r))_i)_j equiv hat(r)_(i j)$,

$ bold(p)_i = "softmax"(bold(hat(r))_i). $

Then we could also define the expected value of any function $O(x_j)$ of $x_j$ under $bold(p)_i$ as

$ EE_(x_j tilde bold(p)_i)[O] equiv sum_j (bold(p)_i)_j O(x_j). $

Query scores route attention to relevant tokens, but information extraction requires a third projection: the value vector $bold(v)_t in RR^(d)$,

$ bold(v)_t = V bold(bold(x)_t), $

where $V$ is the third matrix of linear transformation $RR^(d_"tok") -> RR^d$.

The value projection $V$ extracts task-relevant features.
While $Q$ and $K$ govern attention allocation, $V$ controls feature extraction.

The aggregated value from attended tokens equals the expectation of the value projection:

$ EE_(x_j tilde bold(p)_i)[V] equiv sum_j (bold(p)_i)_j V bold(x_j) $

where $V$ denotes both the linear transformation and its matrix representation.

**Key Insight**: Self-attention performs three operations:

1. **Score computation**: Calculate directional relationships ($Q,K$)
2. **Distribution formation**: Convert scores to probabilities (softmax)
3. **Value extraction**: Weighted sum of transformed tokens ($V$)

Thus, _self-attention computes the expectation of the value projection over a distribution induced by normalized query scores_.

The self-attention output $y equiv [bold(y)_1, dots, bold(y)_T]$ with components $y^(alpha)_t$ and features $bold(y)_t in RR^d$ is compactly expressed as:

$  y^(alpha)_t = sum_(j=1)^T (e^(hat(r)_(t j)))/(Z_t) dot V^(alpha)_(mu) x^(mu)_j =  sum_(j=1)^T ("exp"(x^(mu)_t hat(R)_(mu)^(nu) x^(nu)_j))/(sum_(i=1)^T "exp"(x^(rho)_t hat(R)_(rho)^(sigma) x^(sigma)_i)) dot V^(alpha)_(mu) x^(mu)_j. $

This equation encapsulates the complete self-attention operation.
The summation over $j$ reveals the $O(T)$ computational complexity of each individual token underlying memory constraints.

#### Multi-head Self-attention

With self-attention, we can extract information $bold(y)_i$ from other tokens.
In practice, _multi-head self-attention_ employs $N_"head"$ parallel self-attention operations on $x$.
This multi-headedness operates in parallel to the computational complexity we focus on, but we mention it for completeness.

For each head $1<=h<=N_"head"$, there is a set of learnable parameters ${Q_h, K_h, V_h}$.
Each head defines a distinct relationship or focuses on one aspect among tokens.
Therefore, we obtain $N_"head"$ outputs for each head ${bold(y)^h_i}$.
These outputs are concatenated along the feature dimension into a single output with dimension $N_"head" dot.c d$.
In practice, it is often chosen such that $d_"tok" = N_"head" dot.c d$

Normally, an extra learnable matrix $W_o in RR^(N_"head" dot d) times RR^(N_"head" dot d)$ mixes features from different heads.
This mainly involves mixing information within feature channels and does not play an important role in the computational complexity.

### Causal Structure on Tokens

Decoder-only autoregressive transformers incorporate causal masking into self-attention.
During inference, models generate tokens sequentially using preceding context, necessitating that training also restricts attention to historical tokens.

This causal structure requires the ensemble contribution from the future to be zero,

$ (bold(p_i))_(j>i) = 0 $

which could be equivalently achieved by restricting summation range $j<=i$, multiplying a causal mask $M_(i j)=bold(1)_(i>=j)$, or setting query score $r_(i<j)="-infinity"$.

**Memory Implications**: Causal masking produces triangular attention patterns but preserves $O(T)$ complexity for each token.
Each token attends to all predecessors, demanding quadratic memory for attention matrices as tokens grow.

## Memory Bottlenecks in Transformer Attention

The fundamental memory limitation arises from the attention mechanism's computational structure. Let's analyze the scaling laws that constrain practical deployment.

### Quadratic Scaling of Attention

From the self-attention equation:

$ y^(alpha)_t = sum_(j=1)^T ("exp"(x^(mu)_t hat(R)_(mu)^(nu) x^(nu)_j))/(sum_(i=1)^T "exp"(x^(rho)_t hat(R)_(rho)^(sigma) x^(sigma)_i)) dot V^(alpha)_(mu) x^(mu)_j $

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
| Attention scores | $B dot N_"head" dot T^2$ (float32) | $O(T^2)$ |
| Key-Value cache | $B dot T dot d_"tok"$ (per layer) | $O(T)$ |
| Gradient storage | $2 dot$ forward pass | $O(T^2)$ |

For GPT-3 parameters ($d_"tok"=12288$, $N_"head"=96$):

- $T=2048$: ~$96 dot 2048^2 dot 4"B" approx 1.6"GB"$ per batch
- $T=8192$: ~$96 dot 8192^2 dot 4"B" approx 25.8"GB"$ per batch

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
