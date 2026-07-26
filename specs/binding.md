---
title: binding
alias: substrate binding, neural substrate, neural runtime binding
tags: cyber, neural, spec
crystal-type: spec
crystal-domain: cyber
status: draft
---

# binding

what [[neural]] is, as code, against the built [[soft3]] stack. the other specs give the language its law; this one names the crates it rides, so "implement neural" has a seam. neural adds **no new core deps** — it rides the same crates the [[cyb]] cell already compiles ([[nox]] → [[bbg]] → [[cybergraph]] → [[inf]]).

## the split

neural **writes and parses; it never computes meaning.** φ\*, [[motif]]s, and [[dialect]] discovery are [[tru]]'s job (the [[tri-kernel]] fixed point); neural references them.

| neural does | rides | neural never |
|---|---|---|
| parse a [[sentence]] (surface → AST) | own code (`sigil`) | run the [[tri-kernel]] |
| resolve a [[word]] to a [[particle]] | `cyber-hemera` (`hash`) | compute φ\* |
| expand a sentence to [[link]]s (a [[dialect]]) | `bbg::state::axon_id` | discover [[motif]]s |
| seal a [[signal]] | [[cybergraph]] via the cell | rank the [[lexicon]] |
| read the graph | [[inf]] (datalog over [[bbg]]) | settle a proof ([[zheng]]) |

## write path

a [[sentence]] becomes a [[signal]] and lands through the cell — neural is a *client of the cell*, not a second runtime.

- **word → particle:** `word_particle(name) = hemera::hash(name)` — deterministic, so speaking a word merges into the graph.
- **relation → links:** the [[dialect]] expands `subject relation object` into `[subject → object, axon_id(subject,object) → relation]` (see [[relation]]). `axon_id = H(from‖to)` is public (`bbg::state::axon_id`) and materialises on link (A6).
- **seal:** the links go into one [[signal]] on the neuron's chain and apply via `Cybergraph::link` (the cell's durable choke point). the [[wire]] is unchanged.

## read path

the read half of neural is **[[inf]]** — the datalog engine already compiled into the cell. neural does not add a query language; it *is* [[inf]] on the read side. `particles`, `axons`, `focus` are inf relations; the [[lexicon]] is `particles` ordered by φ\*, a typed [[relation]] is a join on `axons`.

## crate map

```
neural/rs/                  a detached workspace, sibling crates by path
  sigil/    text → Sentence ; word → particle        deps: cyber-hemera
  dialect/  Sentence → the links it lands as          deps: sigil, bbg
  cli/      `neu` — client of the cell                deps: sigil, dialect,
                                                             cyb-core, cybergraph, bbg
```

what stays *referenced*, not built here: φ\* and the [[lexicon]] ([[tru]]), [[motif]] discovery ([[tru]]), proofs ([[zheng]]), the wire bytes ([[tape]]/foculus codec).

## status

slice-1 is real and green: `neu "cat is-a animal"` parses → expands to the base + axon-type link → lands on a durable cell → reads back with the relation-word ranked by [[focus]]. the foresight verbs (`ask`, `prove`) wait on [[tru]]'s motif-transition kernel and are not on this seam yet. see [[neu]] and the plan.
