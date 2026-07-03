---
title: neural wire
alias: neural wire format, signal format
tags: cyber, neural, spec
crystal-type: spec
crystal-domain: cyber
status: draft
---

# neural wire

how a [[sentence]] reaches the [[cybergraph]]: the wire format of a [[signal]]. neural arranges [[link]]s; the wire seals them — staked, valenced, timed — into the [[cybergraph]].

## signal

a signal is one atomic submission — a batch that lands together or not at all. it is the boundary that makes a [[sentence]] an utterance.

```
Signal {
  neuron:  Particle      // @who — the author
  links:   [Link]        // the ordered batch
  time:    Timestamp     // epoch
  proof:   Proof         // zheng proof of well-formedness
}
Link { from: Ref, to: Ref, amount: Amount, valence: i8 }
            // Ref = a sigil resolved to a Particle (hemera id)
            // valence: +1 affirm · 0 neutral · -1 negate
```

each [[link]] is one [[neuron]]'s staked assertion; the [[cybergraph]] bundles the links on a pair into an [[axon]].

## submission

- intent → seal — a two-phase submit, or one-shot for small signals
- atomicity — the whole signal commits or aborts; half a [[sentence]] does not exist
- addressing — `from` / `to` are [[sigil]]s resolved to [[particles]]; `~` names resolve deterministically at seal time

## encoding

the canonical schema lives with the [[soft3]] SDK `schema/`; the rust, ts, and py SDKs generate from it. ids are [[Goldilocks field]] elements ([[hemera]] outputs), used directly in field arithmetic.

see [[cybergraph]] for routing and persistence, [[zheng]] for the proof, [[tru]] for how recorded [[focus]] feeds φ\*.
