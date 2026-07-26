---
title: neural specs
alias: neural reference, neural spec index
tags: cyber, neural, spec
crystal-type: spec
crystal-domain: cyber
status: draft
---

# neural — reference

the reference layer for [[neural]], the semantic language of [[soft3]]. these specs are the law: the primitives, their data, their semantics. the conceptual treatment is in `explanation/`; the vision in [[neural|README]].

## the ladder

neural builds meaning from the [[sigil]] up — the densest operator, then the [[word]] it names, then structure to convention, then the [[lexicon]] it reads back:

| primitive | role | home |
|---|---|---|
| [[sigil]] | base operator — point and act; the densest unit | `sigils.md` |
| [[word]] | the unit of meaning — a typed [[particle]] | `word.md` |
| [[link]] | the atom relation — word → word | [[cybergraph]] (referenced) |
| [[sentence]] | linear — a chain of links in one signal | `sentence.md` |
| [[motif]] | non-linear — a recurring subgraph shape | `motif.md` |
| [[dialect]] | convention — an agreed collection of sentences and motifs | `dialect.md` |
| [[lexicon]] | the living vocabulary — top words by φ\* | `lexicon.md` |

the atom's type — a [[relation]] as a link on the [[axon]]: `relation.md`. the wire format that carries them: `wire.md`. the surface that speaks them: `cli.md` ([[neu]]). how it all binds to the built stack: `binding.md`.

## cross-cutting law

- a [[word]] is the unit of meaning — a typed [[particle]]; a bare particle is a content node, a word carries a [[type]] and a [[name]]. the [[lexicon]] holds the words in use (top by φ*).
- the atom relation is the [[link]] — a staked assertion binding two [[word]]s, `from → to`. neural is the language of links; [[link]] is the graph's unit of knowledge.
- the [[cybergraph]] bundles all the links on a pair into an [[axon]] — itself a [[particle]] (axiom A6), the grain the [[tri-kernel]] flows along. a link (the assertion) and an axon (the bundle) are the graph's two readings of an edge; both live in [[cybergraph]], neural references them.
- neural reads meaning; it does not compute it. φ\* — the [[focus]] distribution that ranks every [[particle]] and [[axon]] — is computed by [[tru]] (the [[tri-kernel]] fixed point) and settled by [[zheng]]. neural consumes φ\*; it never runs the kernel.
- a [[sigil]] is neural's base operator and its densest unit — point and act. like *the* in English (the most frequent word), the sigils carry structure over content; you can express almost anything with the alphabet alone. it lives in the neural [[sigils]] (sigils.md), folded in from the address language; every [[link]] is two sigils joined.
- a [[signal]] is the unit of submission: one atomic batch of links.
- one operator, one ladder: the [[tri-kernel]] is the graph Laplacian and its heat kernel (diffusion / springs / heat = the heat exponential / the spectral embedding / the scale τ), promoted from [[link]] to [[axon]] to [[motif]] along the simplicial ladder and interpreted by a [[dialect]] (a functor). φ\*, the [[lexicon]], dialect discovery, and the tension that marks polysemy are readings of one eigenproblem. a staked [[link]] is an optic, so [[reward]] composes along sentences and motifs. see [[frontier]].
