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

### Thermodynamic Ensemble on Tokens

Let's take a small detour to physics. In thermodynamics, the probability, $p_i$, of finding a system of temperature $T$ at a state of index $i$ with energy $E_i$, is given by the Boltzmann distribution

$ p_i = (e^(-E_i / (k_B T)))/Z $

where $Z equiv sum_i e^(-E_i/(k_B T))$ is the partition function, which normalizes the distribution, and $k_B$ is called Boltzmann coefficient.

In quantum mechanics or quantum field theory, same patterns appear in path integral formulation, where each trajectory $bold(x)$ contributes a complex phase factor,

$ phi[bold(x)] = e^((i S[bold(x)])/hbar) $

where $S[bold(x)] equiv  integral_(t_i)^(t_f) L(bold(x), dot(bold(x))) dif t$ is the integral of Lagrangian along the path $bold(x)$ from $t_i$ to $t_f$.

A neural networks, a highly related function is called softmax function, 

$ "softmax"( bold(x))_i =  e^(x_i)/(sum_i e^(x_i)) $

which maps an array of values $bold(x)$ to a probability distribution $"softmax"(bold(x))$. Softmax function could be viewed as a Boltzmann distribution with $x_i -> (-E_i)/(k_B T)$. 

Note, the quantity on the exponent has to be dimensionless, otherwise quantities of different physical dimensions are added together in Taylor expansion of exponential function, $e^x = sum_(n=0)^infinity (x^n)/(n!)$. For Boltzmann distribution, $E_i$ and $k_B T$ have dimension of energy, and in path integral, $A$ and $hbar$ have dimension of energy times time. Their ratios on the exponent are dimensionless. The denominator of this fraction, often called temperature in consistent with its meaning in thermodynamics, controls the spikiness or peakiness of the distribution. As temperature $-> infinity$, the distribution becomes flat; as temperature $->0$, the distribution peaks at max or min (if there is negative sign). The temperature is also responsible setting a standard scale of energy. If energy levels increase and temperature scales by the same factor, then the distribution keeps the original.

Back to our bilinear forms on tokens. In the end we want to define a distribution using query scores among tokens. The probability distribution should be invariant with detailed implementation, such as the dimension of projections $d$. In practice, we can assume the following statistical features about tokens and matrices, 

$ &EE[x_(mu)] = 0, "Var"[x_(mu)]=1 \ &lr(Q^mu)_nu tilde cal(N)(0;1), lr(K^mu)_nu tilde cal(N)(0;1) $,

then the projected vector $q_mu$ follows,

$ EE[q_mu]=EE[lr(Q^mu)_nu x_(nu)] = 0, "Var"[q_mu]="Var"[lr(Q^mu)_nu]"Var"[x_(nu)]=d times 1/d times 1 = 1. $

Similarly, the projected vector $k_mu$ follows

$ EE[k_mu]= 0, "Var"[k_mu]= 1. $

The dot product, or the linear 

In probability theory and thermodynamics, we usually define an expectation value of some variable $O(i)$ as 

$ EE[O] equiv chevron.l O chevron.r equiv sum_i p_i O(i). $ 

For each new token $x_t$, following a sequence of tokens $[x_1, dots, x_(t-1)]$, each self-attention incapsulates the contribution of all preceding tokens to this token,

$ "softmax"() $
