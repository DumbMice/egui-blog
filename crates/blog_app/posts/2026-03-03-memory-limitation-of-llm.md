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

From now on, we will use Greek letters, e.g. $mu, nu, tau$, for feature indices and Latin letters, e.g. $t, s, u$, for positional indices.
For example, in GPT-3 (175B parameter model), $d_"tok"=12288$ with 96 attention heads, resulting in $d=128$ per-head attention dimension (since $d_"tok" = N_"head" dot.c d$).

The core innovation of attention is representing directional relationships between tokens.

### Asymmetric Bilinear Form on Tokens

When $bold(x)_i$ attends to $bold(x)_j$, this establishes a **directional** relationship.
Since symmetric operations like dot products cannot capture directionality, self-attention employs **a learnable asymmetric bilinear form** to represent these directed connections.

Self-attention computes two projections for each token: a _query_ vector $bold(q)_t in RR^d$ and a _key_ vector $bold(k)_t in RR^d$.
The attention from $bold(x)_i$ to $bold(x)_j$ is determined by the dot product $bold(q)_i dot.c bold(k)_j$, where the query "asks" about information and the key "answers" with relevance.
These projections are obtained through linear transformations using learnable matrices $Q$ and $K$, mapping from $RR^(d_"tok")$ to $RR^(d)$. Their tensor representations are:

$ bold(q)_t = Q bold(x)_t  &<=> q_(mu t) = lr(Q_mu)^nu x_(nu t) \  bold(k)_t = K bold(x)_t &<=> k_(mu t) = lr(K_mu)^nu x_(nu t), $

where a sum over $nu$, i.e. $sum_(nu=1)^(d_"tok")$, is implied when it appears twice, one time as an upper index and the other time as a lower index.
This contraction of repeated indices, often called _Einstein notation_, is commonly used in linear algebra and differential geometry and is used throughout this post.

**Notation Clarification**: The index $mu$ serves dual roles: as a feature index for tokens $x_(mu t)$ with dimension $d_"tok"$  and for query-key pairs $q_(mu t)$, $k_(mu t)$ with dimension $d$.
Context determines which dimension applies, and indices maintain consistent meaning within each expression.

To ensure this relation is directional, we define the _query score_ between two tokens $r(x_i,x_j) in RR$ as

$ r(x_i, x_j)  &equiv q_i dot k_j = q^mu_i k_(mu j) \ &= Q^(mu nu) x_(nu i) lr(K_mu)^tau x_(tau j) \ &= x_(nu i) ( Q^(mu nu) lr(K_mu)^tau )  x_(tau j) \ &= x_(nu i) R^(nu tau) x_(tau j) \ &= bold(x)^tack.b_i R bold(x)_j $

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
Assuming token and matrix entries are i.i.d. random variables with specified statistics:

$ &EE[x_(mu)] = 0, "Var"[x_(mu)]=1 \ &lr(Q^mu)_nu tilde cal(N)(0,1/d_"tok"), lr(K^mu)_nu tilde cal(N)(0,1/d_"tok"). $

Then for entries in the bilinear matrix $R$, it follows that

$ &EE[R^(mu nu)] = EE[lr(Q^tack.b)^(mu tau) K_tau^nu]= 0 \ &"Var"[R^(mu nu)] = "Var"[lr(Q^tack.b)^(mu tau)] dot "Var"[K_tau^nu] = d/lr(d_"tok")^2. $

The query score $r_(i j) equiv r(bold(x)_i, bold(x)_j)$ hence has the mean and variance,

$ &EE[r_(i j)] = EE(x_(mu i) R^(mu nu) x_(nu j)) = 0 \ &"Var"[r_(i j)] = "Var"[x_(mu i)] dot "Var"[R^(mu nu)] dot "Var"[x_(nu j)] = lr(d_"tok")^2 dot 1 dot d/lr(d_"tok")^2 dot 1 = d. $

Consequently, the bilinear form's variance scales linearly with projection dimension $d$, an implementation-dependent hyperparameter.

To get rid of this dependency, we define a normalized query score $hat(r)_(i j)$ as

$ hat(r)_(i j) = hat(r)(bold(x)_i, bold(x)_j) = (bold(x)^(tack.b)_i R bold(x)_j) / sqrt(d) $,

which is a normalized score with mean and variance independent of $d$,

$ &EE[hat(r)_(i j)] =  0 \ &"Var"[hat(r)_(i j)] = 1 $

and this $sqrt(d)$ can be interpreted either as a dimensional quantity that renders $hat(r)$ dimensionless or as a temperature that preserves the peakedness of the distribution derived from $hat(r)$.

The $sqrt(d)$ scaling factor is essential for stable training - without it, attention scores would diverge with increasing model dimensions.

#### Self-Attention as Ensemble on Tokens

Given a query token $bold(x)_i$, we can use the normalized query score to define a distribution (or an ensemble), $bold(p)_i$ over attended tokens $bold(x)_j$,

$ (bold(p)_i)_j = (e^(r_(i j)))/(sum_j e^(r_(i j))) = (e^(r_(i j)))/(Z_i), $

where $Z_i$ is the partition function.
This can also be written as a softmax function if we define $(bold(r)_i)_j equiv r_(i j)$,

$ bold(p)_i = "softmax"(bold(r)_i). $

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

The self-attention output $y equiv [bold(y)_1, dots, bold(y)_T]$ with components $y_(mu t)$ and features $bold(y)_t in RR^d$ is compactly expressed as:

$  y_(mu t) = sum_j (e^(r_(t j)))/(Z_t) dot.c V_mu^alpha x_(alpha j) =  sum_j "exp"(x_(nu t) R^(nu tau) x_(tau j))/(sum_i exp(x_(sigma t) R^(sigma rho) x_(rho i))) dot.c lr(V_mu)^alpha x_(alpha j). $

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

which could be equivalently achieved by restricting summation range $j<=i$, multiplying a causal mask $M_(i j)=bb(1)_(i>=j)$, or setting query score $r_(i<j)=-infinity$.

**Memory Implications**: Causal masking produces triangular attention patterns but preserves $O(T)$ complexity for each token.
Each token attends to all predecessors, demanding quadratic memory for attention matrices as tokens grow.
Subsequent analysis will examine how this scaling constrains context length and survey mitigation strategies including KV caching, sparse attention, and linear-time alternatives.
