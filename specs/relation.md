---
title: relation
alias: relations, typed relation, axon typing, relation model
tags: cyber, neural, spec
crystal-type: spec
crystal-domain: cyber
status: draft
---

# relation

how a [[link]] carries a *type* — `is-a`, `causes`, `part-of` — without labelling the edge. the settled model of [[neural]]: **a relation is a link on the [[axon]].** the base edge stays untyped; its type is a second-order link from the pair's axon to a relation-[[word]].

## the model

`cat is-a animal` lands as two edges:

```
cat → animal                 the bare relationship
H(cat, animal) → is_a         the type — a link on the axon of the pair
```

the [[cybergraph]] already reifies every pair: all links on `(from, to)` bundle into an [[axon]], and `axon = H(from‖to)` is a [[particle]] that materializes on link (bbg axiom A6). so typing a relation needs **no new mechanism** — you stake a link *from* that axon *to* the relation-word.

## why this and not the alternatives

three ways to type a link; only this one holds:

- **edge label** (`Link{from,to,rel}`) — would change the [[wire]] and the running cell, and declares a relation schema. rejected: it breaks *types emerge, no schema*, and relations are not [[particle]]s so φ\* cannot rank them.
- **route-through-word** (`cat → is_a → animal`) — makes no real `cat–animal` edge and turns every relation into a **super-node** (one `is_a` particle collecting millions of links), wrecking [[focus]] and topology. rejected.
- **axon typing** (this) — the base edge is real, the type is a meta-link, and it satisfies every constraint at once (below).

## what it buys

- the **wire is untouched** — `Link { from, to, token, amount, valence }` stays; a relation is just another link. the running cell needs no change.
- relations are **first-class [[particle]]s** — a relation-word is ranked by φ\*, can be linked, staked, and disputed like any [[word]].
- **many types per pair** — a pair may carry `→ is_a` and `→ causes` at once; the **tension** among them is the polysemy / curvature signal (see [[frontier]]).
- **typed edges stay emergent** — the *type* is which word you route through, not a declared column; honours *[[neural]]'s types are not declared, they emerge*.
- **two granularities free** — type the [[axon]] for the pair-relationship; type a specific link-[[particle]] for one assertion. the [[cybergraph]] closure gives both.

## how it is authored

a neuron never hand-writes the two links. it speaks a [[sentence]] (`cat is-a animal`); the active [[dialect]] expands it into `[base link + axon-type link]`. this is exactly a dialect's job — *a smart contract over [[link]]s: it builds the right ones*. so:

> meaning is authored at the **[[sentence]]** altitude, lands as a **typed axon** (link on the pair), and is read back by the **[[dialect]]** that owns the relation-word.

## bootloader relation-words

the genesis [[dialect]] ships a small seed set, themselves [[particle]]s ranked by [[focus]]:

- the epistemic poles — `TRUE`, `FALSE` (see [[dialect]])
- the emergent relations — `is-a`, `causes`, `part-of`, `contradicts`, `follows`, `see-also`
- the ~15 universal image schemas as primitive relations — `PATH`, `CONTAINER`, `LINK`, `BALANCE`, `SOURCE-PATH-GOAL` (see [[frontier]])

new relation-words are not decreed; a [[word]] used in the relation slot across enough [[sentence]]s accrues [[focus]] and enters the dialect.

## reading a typed relation

`axon_id(from, to)` is deterministic and public (`bbg::state::axon_id`), and axons are exposed to [[inf]] as a relation. so "all `X is-a Y`" is an [[inf]] query: the `axons` whose `to` is `is_a`, joined back to the base pair each types.

see [[wire]] for the signal it rides, [[dialect]] for the expander, [[word]] for the relation-word, [[frontier]] for tension/curvature.
