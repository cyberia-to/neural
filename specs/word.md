---
title: word
alias: words, lexical unit
tags: cyber, neural, spec
crystal-type: spec
crystal-domain: cyber
status: draft
---

# word

the lexical unit of [[neural]]: a typed [[particle]] — a unit of meaning. a bare [[particle]] is a content node; a word is a particle that carries a [[type]] and a [[name]], and means something. the [[lexicon]] is the words in active use.

## the four faces

a word is where soft3's existing pieces converge into one unit of meaning:

| face | what | already in soft3 |
|---|---|---|
| form | how the word is spelled — its content | [[field]] elements (yin / yang halves; a particle's leaves) |
| identity | the word as a node | [[particle]] — the content hash of its form |
| category | its part of speech | [[type]] — one of the 16 [[languages]] (Field, Tensor, Distribution…) |
| meaning | what it means | its position in the graph (the [[link]]s around it), ranked by [[focus]] φ* |

so: form ([[field]]) → identity ([[particle]]) → + category ([[type]]) = word → the [[lexicon]] (words by φ*).

## word and particle

every word is a [[particle]]; not every particle is a word. a raw blob is a particle; a typed, named concept is a word. a word is a particle elevated to meaning by a [[type]] and a [[name]] — and ranked, once it holds [[focus]], into the [[lexicon]].

at the frontier a word is a point in a (mixed-curvature) manifold whose [[focus]] belief is a distribution, not a point; a monosemantic entry mined from the graph. polysemy is the word held under tension between incompatible senses — measurable as curvature or superposition, and splittable at the spectral fault line. see [[frontier]].

## the type — two halves, one declared

a word's [[type]] spans two worlds, and the resolution is to *not* store both. one half is **declared**, the other **emerges**:

- **computational type (`ctype`) — declared, stored.** a value-word is typed by the 16 [[languages]] — a number is Field, a matrix is Tensor, a distribution is Distribution. this is fixed by [[nox]] at the form; a word carries it.
- **semantic role — emergent, derived.** is-a, causes, part-of are *not* a stored field. a word's role is its position — which [[relation]]-words it links through, ranked by [[focus]]. *neural's types are not declared; they emerge* (see the explanation). so the role is read from the graph, never written on the word.

so the stored word is `{ particle, ctype, name }`; the semantic type is a reading of the neighbourhood, not a column. that split is what makes `word` buildable — the "convergence" was only hard while we tried to store the half that emerges.

## form is not the word

the `word` is meaning; the grain of form beneath it is the [[field]] element, which splits into `yin` and `yang` — its two 32-bit halves (see [[nox]]). a word is spelled in fields; it is not a field. one name per layer: yin/yang for form, `word` for meaning.
