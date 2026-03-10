---
title: "Memory Limitation of LLM"
date: "2026-03-03"
tags: ["llm", "memory", "ai"]
---

The linearly increasing computation cost has been the most cumbersome problem leaving for scaling llm as a general purpose long-term agent.

## Introduction to Self-Attention

Self-attention captures how a sequence of texts attents to itself. Let it be a sentence of _greeting_, a _scientific report_ or a _history of chatting_.

### Texts as Tokens

Mathematically, they are all sequences of tokens $[bold(x)_1, dots, bold(x)_T]$, and each token $bold(x)_t in RR^(d_("tok"))$.
Using the bold notation $bold(x)_t$, we are actually hiding $d_"tok"$ dimensions of its _token feature_. To explain full details of self-attention,
I would abuse the tensor notation for multi-dimensional array, and denote a sequence of tokens simply as $x equiv [bold(x)_1, dots, bold(x)_T]$ composing of  components $x_(mu t)$ at position index $1<=t<=T$ and feature index $1<=mu<=d_"tok"$.

From now on, we will use greek letters,  e.g. $mu, nu, tau$, for feature indices and alphabet, e.g. $t, s, u$, for positional index.

### Asymmetric Bilinear Form on Tokens

When we say $bold(x)_i$ attends to $bold(x)_j$, this defines a **directional** relation. For directional relation, we could not use dot product or any symmetric calculation. Self attention adopts a straightforward method that utilizes **a learnable asymmetric bilinear form** to represent this directional relation.

Self-attention requires each token to have two **projections**, _query_  $bold(q)_t in RR^d$ and _key_  $bold(k)_t in RR^d$. When $bold(x)_i$ attends to $bold(x)_j$, it _queries_ the attended's _key_ with a dot product, $bold(q)_i dot.c bold(k)_j$.
The simpliest way of obtaining query-key projections are multiplying tokens with two learnable matrices, $Q, K$, as linear transformations $RR^(d_"tok") -> RR^(d)$. The definitions of $q$ and $k$ and its tensor representations are given as,

$ bold(q)_t = Q bold(x)_t  &<=> q_(mu t) = lr(Q_mu)^nu x_(nu t) \  bold(k)_t = K bold(x)_t &<=> k_(mu t) = lr(K_mu)^nu x_(nu t), $

where a sum over $nu$, i.e. $sum_(nu=1)^(d_"tok")$, is implied when it appears twice, one time as an upper index and the other time as a lower index. This contraction of repeated indices, often called _Einstein notation_, is commonly used in linear algebra and differential geometry and is used throughout this post.

**Note**: Although we previously used $mu$ as feature index of token $x_(mu t)$ and now as feature indice of query-key pair $q_(mu t)$ and $k_(mu t)$, the meanings and dimensions are different, $d_"tok"$ vs. $d$. Their meaning should be clear from the context and one feature index will not stand for different meanings simultaneously in one term.

To make sure this relation is directional, we define the query result between two tokens $r(x_i,x_j) in RR$ as

$ r(x_i, x_j)  equiv q_i dot k_j = q^mu_i k_(mu j) = Q^(mu nu) x_(nu i) lr(K_mu)^tau x_(tau j) = x_(nu i) ( Q^(mu nu) lr(K_mu)^tau )  x_(tau j) = x_(nu i) R^(nu tau) x_(tau j) = bold(x)^tack.b_i R bold(x)_j $

where  $R$ could be represented as

$ R= Q^tack.b K <=> R^(nu tau) = Q^(mu nu) lr(K_mu)^tau = Q^( tack.b nu mu)  lr(K_mu)^tau. $

The query result funciton, $r(dot.c, dot.c)$, is a _bilinear form_, $RR^d times RR^d -> RR$, with matrix $R$. In general, there is no constraint put on $Q$ and $K$, and thus $R$ is almost always asymmetric, and so is the corresponding bilinear form $r$.

### Ensemble on Tokens

#### Detour to Physics

Let's take a small detour to physics. In thermodynamics, the probability, $p_i$, of finding a system of temperature $T$ at a state of index $i$ with energy $E_i$, is given by the Boltzmann distribution

$ p_i = (e^(-E_i / (k_B T)))/Z $

where $Z equiv sum_i e^(-E_i/(k_B T))$ is the partition function, which normalizes the distribution, and $k_B$ is called Boltzmann coefficient.

In quantum mechanics or quantum field theory, same patterns appear in path integral formulation, where each trajectory $bold(x)$ contributes a complex phase factor,

$ phi[bold(x)] = e^((i S[bold(x)])/hbar) $

where $S[bold(x)] equiv  integral_(t_i)^(t_f) L(bold(x), dot(bold(x))) dif t$ is the integral of Lagrangian along the path $bold(x)$ from $t_i$ to $t_f$.

A neural networks, a highly related function is called softmax function,

$ "softmax"( bold(x))_i =  e^(x_i)/(sum_i e^(x_i)) $

which maps an array of values $bold(x)$ to a probability distribution $"softmax"(bold(x))$. Softmax function could be viewed as a Boltzmann distribution with $x_i -> (-E_i)/(k_B T)$.

Note, the quantity on the exponent has to be dimensionless, otherwise quantities of different physical dimensions are added together in Taylor expansion of exponential function, $e^x = sum_(n=0)^infinity (x^n)/(n!)$. For Boltzmann distribution, $E_i$ and $k_B T$ have dimension of energy, and in path integral, $A$ and $hbar$ have dimension of energy times time. Their ratios on the exponent are dimensionless.

The denominator of this fraction, often called temperature in consistent with its meaning in thermodynamics, controls the spikiness or peakiness of the distribution. As temperature $-> infinity$, the distribution becomes flat; as temperature $->0$, the distribution peaks at max or min (if there is negative sign). The temperature also sets a standard scale of energy. If energy levels and temperature scale by the same factor, then the distribution keeps the original.

#### Normalized Query Score

Back to our bilinear forms on tokens. In the end we want to define a distribution using query scores among tokens. The probability distribution should be invariant regardless of representations, such as the dimensions $d$ and $d_"tok"$. In practice, we can assume entries in tokens and matrices are i.i.d. random variables with the following mean and variance,

$ &EE[x_(mu)] = 0, "Var"[x_(mu)]=1 \ &lr(Q^mu)_nu tilde cal(N)(0,1/d_"tok"), lr(K^mu)_nu tilde cal(N)(0,1/d_"tok"). $

Then for entries in the bilinear matrix $R$, it follows that

$ &EE[R^(mu nu)] = EE[lr(Q^tack.b)^(mu tau) K_tau^nu]= 0 \ &"Var"[R^(mu nu)] = "Var"[lr(Q^tack.b)^(mu tau)] dot "Var"[K_tau^nu] = d/lr(d_"tok")^2. $

The query score $r_(i j) equiv r(bold(x)_i, bold(x)_j)$ hence has the mean and variance,

$ &EE[r_(i j)] = EE(x_(mu i) R^(mu nu) x_(nu j)) = 0 \ &"Var"[r_(i j)] = "Var"[x_(mu i)] dot "Var"[R^(mu nu)] dot "Var"[x_(nu j)] = lr(d_"tok")^2 dot 1 dot d/lr(d_"tok")^2 dot 1 = d. $

As a result, this bilinear form, which should define the relation among tokens, has a variance that scales linearly with the dimension of projection dimension $d$, which is a intermediate parameter that depends on actual implementation.

To get ride of this dependency, we define a normalized query score $hat(r)_(i j)$ as

$ hat(r)_(i j) = hat(r)(bold(x)_i, bold(x)_j) = (bold(x)^(tack.b)_i R bold(x)_j) / sqrt(d) $,

which is normalized score with means and variance independent of $d$ ,

$ &EE[hat(r)_(i j)] =  0 \ &"Var"[hat(r)_(i j)] = 1 $

and this $sqrt(d)$ can be interpreted either as a dimension quantity that renders $hat(r)$ dimensionless or a temperature that preserves the peakiness of distribution given from $hat(r)$.

#### Self-Attention as Ensemble on Tokens

Given a query token $bold(x)_i$, we can use normalized query score to define a distribution (or an ensemble), $bold(p)_i$ on attended tokens $bold(x)_j$,

$ (bold(p)_i)_j = (e^(r_(i j)))/(sum_j e^(r_(i j))) = (e^(r_(i j)))/(Z_i), $

where $Z_i$ is the partition function. This could also be written as a softmax function if we define $(bold(r)_i)_j equiv r_(i j)$,

$ bold(p)_i = "softmax"(bold(r)_i). $

Then we could also define the expected value of any function $O(x_j)$ of $x_j$ under $bold(p)_i$ as

$ EE_(x_j tilde bold(p)_i)[O] equiv sum_j (bold(p)_i)_j O(x_j). $

If you regard assemble of query score as the result of a routing process, distributing attention to tokens of useful information, then we still need to extract information from them. This involves the last piece of self attention, extracting values from attended tokens. This requires a third projection of tokens,  value projection $bold(v)_t in RR^(d)$,

$ bold(v)_t = V bold(bold(x)_t), $

where $V$ is the third matrix of linear transformation $RR^(d_"tok") -> RR^d$.

The value extracted from all attended tokens is the expected value of value projection function,

$ EE_(x_j tilde bold(p)_i)[V] equiv sum_j (bold(p)_i)_j V bold(x_j) $

where we are abusing $V$ to denote both the linear transformation and its matrix.

As a result, _self attention calculates the expectation value of value projection map evaluated on a ensemble induced by normalized query score_.

In the end, let's the final output from self-attention module as $y equiv [bold(y)_1, dots, bold(y)_T]$ composing of components $y_(mu t)$, and each output feature $bold(y)_t in RR^d$. Using the tensor annotation, the calculation of self attention can be captured as a single term

$  y_(mu t) = sum_j (e^(r_(t j)))/(Z_t) dot.c V_mu^alpha x_(alpha j) =  sum_j "exp"(x_(nu t) R^(nu tau) x_(tau j))/(sum_i exp(x_(sigma t) R^(sigma rho) x_(rho i))) dot.c lr(V_mu)^alpha x_(alpha j). $

### Causal Structure on Tokens

In a decoder-only autoregressive transformers, an extra causal strucutre must be imposed on self-attention.
During inference, autoregressive model predicts next token using previous tokens. So during training, self-attention should also only use tokens in the past to predict next.

This causal structure requires the ensemble contribution from the future to be zero,

$ (bold(p_i))_(j>i) = 0 $

which could be equivalently achieved by restricting summation range $j<=i$, multiplying a causal mask $M_(i j)=bb(1)_(i>=j)$, or setting query score $r_(i<j)=-infinity$.
