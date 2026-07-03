---
title: sentence
alias: sentences, linkchain, linkchains
tags: cyber, neural, spec
crystal-type: spec
crystal-domain: cyber
status: draft
---

# sentence

the linear utterance of [[neural]]: an ordered chain of [[link]]s, `a → b → c → …`, cast in one [[signal]]. the signal boundary defines the utterance — a sentence is atomic, all of its links land together or none do. order is grammar.

## structure

a sentence is a sequence of links where each target is the next source: `(ℓ₁, …, ℓₙ)` with `to(ℓᵢ) = from(ℓᵢ₊₁)`. it is a [[linkchain]] — the same object. a sentence a [[neuron]] states and a chain [[tru]] discovers differ only in origin; both are linear paths of meaning.

## types by topology

shape gives kind:

| type | shape |
|---|---|
| assertion | chain ending in TRUE |
| query | open-ended chain |
| instruction | temporal sequence |
| argument | branch to TRUE / FALSE |
| definition | star around one particle |
| narrative | temporally ordered chain |

## properties

- length — shorter carries stronger relation
- width — parallel chains between the same endpoints make it robust
- weight — the product of [[link]] weights along the chain; the [[focus]] it carries
- composition — `C₁ ∘ C₂` joins when `end(C₁) = start(C₂)`; a cycle closes when `to(ℓₙ) = from(ℓ₁)`

## explicit and implicit

an explicit sentence is what a [[neuron]] states. the implicit chains [[tru]] finds by [[diffusion]] are conclusions the network did not state outright. both are sentences; [[tru]] ranks them by [[focus]]. a non-linear arrangement of the same links is a [[motif]].

## the linear flow

as a flow on the graph a sentence is the gradient — the linear, consistent component of meaning. a coherent argument has near-zero curl; a self-contradicting chain (`a → b → c → a`) is pure curl. a [[motif]] is the non-linear (curl / harmonic) component. see [[frontier]].
