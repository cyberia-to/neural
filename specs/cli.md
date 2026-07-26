---
title: neu
alias: neural cli, neu cli, the mouth
tags: cyber, neural, spec
crystal-type: spec
crystal-domain: cyber
status: draft
---

# neu

the command line of [[neural]] — its mouth. inside the cyber ecosystem neural needs no name of its own: it is the native tongue of the [[cyb]] terminal, what you speak into the prompt. `neu` is the same language carried to any shell as a standalone binary, so the world can adopt it. one surface, two hosts.

one rule governs everything below: **a command is a [[sentence]].** neu reads its arguments as a sentence, casts it as a [[signal]] in the active [[dialect]], and prints what φ\* says back. there are no hardcoded subcommands — the verbs are [[sigil]]s that live in the graph, so the language and its cli share one vocabulary.

## the one rule — command = sentence

argv is parsed into a [[sentence]] (a chain of [[link]]s over [[sigil]]s and [[word]]s). the sentence's **type** (see [[sentence]]) decides the effect — neu dispatches on grammar, not on a fixed command table:

| you speak | sentence type | neu does | face |
|---|---|---|---|
| `… is-a …`, `… → TRUE` | assertion | seal a [[signal]] — stake the links | write |
| `motif ask …`, open chain | query | read φ\* from [[tru]], render | read |
| `build doc.md`, temporal | instruction | evaluate the chain, land its delta | write |
| `… → TRUE / FALSE` branch | argument | settle a [[zheng]] proof | read |
| `word new …`, star | definition | mint / type a [[word]] | write |

because the verbs are [[sigil]]s, `neu sigil promote <word>` adds a new command **with no release** — promote a word to an operator and it is instantly speakable. the cli extends itself through the language it speaks.

## the surface — sentences, with a blessed porcelain

every command is a legal sentence, but the common ones have a canonical shape: `neu <layer> <verb> [args]`. these are not special-cased — `<layer>` and `<verb>` are just the highest-φ\* sigils of the base dialect — but they are the readable daily surface. two axes: the [[neural]] ladder gives the **nouns** (altitude of meaning), the three faces give the **verbs** (write / run / read).

### the layers as nouns

| noun | what it addresses | home |
|---|---|---|
| [[sigil]] | the operator alphabet — the extensibility seam | `sigils.md` |
| [[word]] | typed [[particle]]s | `word.md` |
| [[link]] | the atom — `from → to` | [[cybergraph]] |
| [[sentence]] | linear derivations | `sentence.md` |
| [[motif]] | non-linear shapes + foresight | `motif.md` |
| [[dialect]] | a sub-language — convention, scope | `dialect.md` |

[[lexicon]] is not a noun of its own — it is the view `word ls --by-focus`, surfaced as the `focus` shortcut. [[wire]] is not porcelain — it is the [[signal]] format the write path emits (`wire.md`), reachable only as plumbing.

### the verb grammar

one uniform set, reused at every layer (regularity is how the surface stays small):

```
ls            list / rank at this layer            (read)
show <id>     inspect one particle / link / shape  (read)
new …         author at this layer                 (write)
```

plus one special verb per layer, where the layer earns it:

```
sigil    promote <word>      lift a word to an operator
link     stake FROM TO       cast one cyberlink   ·   prove <link>   settle a zheng proof
sentence path A..B           render the linear derivation joining two words
motif    ask [seed|--gaps]   foresight — the next link the graph wants
dialect  enter D · new D · diff D1 D2   scope, carve, and contrast sub-languages
```

### shortcuts

blessed aliases over the canonical forms — the 90% paths, kept short because you type them constantly:

```
neu <sentence>     cast a sentence; effect = its type. the default, no subcommand.
neu                no args → repl: the live loop (author · watch φ\* · read)
neu focus [word]   ≡ neu word ls --by-focus     — the lexicon, scoped to the dialect
neu ask  [seed]    ≡ neu motif ask
neu build [path]   ≡ bulk word + link `new` from a document, sealed as one signal
neu prove <link>   ≡ neu link prove
```

## scope — every command speaks inside a dialect

a [[dialect]] is the langue; a command is parole. the active dialect fixes which [[sigil]]s and [[word]]s are in play and which [[link]]s are well-formed.

```
-d, --dialect D     the sub-language to speak (default: the cwd subgraph's dialect)
--as json|tree|prose   render format (default prose; json for pipes)
--dry               compose the signal and show it; do not seal
@neuron             author identity (default: the signed-in key)
```

cyber's subgraphs **are** dialects — `tru`, `nox`, `cybergraph`, `neural` each carry their own primitives and grammar. so `neu -d tru focus` reads what matters in the tru sub-language, and `neu dialect diff tru nox` surfaces where two subsystems use one [[word]] two ways — a vocabulary-collision detector across the whole stack.

## the write path — one axis, sealed as a signal

only `build`, `stake`, and any assertion / definition sentence mutate the graph. each produces a [[signal]] (see [[wire]]): an atomic batch of links by one [[neuron]], with a [[zheng]] proof of well-formedness.

- **intent → seal** — two-phase submit (one-shot for small signals); `--dry` stops at intent and prints the composed signal.
- **atomic** — the whole signal lands or none of it does; half a [[sentence]] never exists.
- **idempotent** — a [[word]] resolves to a global [[particle]] and a [[link]] restakes rather than duplicates, so re-running a `build` merges instead of appending.
- **typed by [[axon]]** — a typed [[relation]] (`cat is-a animal`) seals as two links, `[cat → animal, H(cat,animal) → is_a]`; the dialect builds both, the wire stays untyped (see [[relation]]).

### surface syntax

the authoring grammar is a [[sentence]], resolved by the active [[dialect]]:

```
neu "cat is-a animal"        # natural: the dialect maps `is-a` → the is_a relation-word
neu "#cat ~is-a #animal"     # sigil-explicit: disambiguate when a name is ambiguous
```

slice-1 grammar is exactly three tokens — `subject relation object`; the [[dialect]] resolves `relation` to a [[relation]]-word and expands to the base + axon-type link. multi-link chains and the full [[sigil]] grammar layer on later; a non-sentence is rejected, not guessed.

## the read path — neu never computes meaning

φ\* is consumed, never produced. `focus`, `ls`, `ask`, `motif`, `path` all read the [[focus]] distribution that [[tru]] computes (the [[tri-kernel]] fixed point) and [[zheng]] settles. neu holds no runtime state and runs no kernel — it translates a query into a graph read and renders the answer. this is the law that keeps neu a thin lens and not a second [[tru]].

## the hero — proof-carrying foresight

the one capability a graph client cannot have, because the runtime is a fixed point with history:

```
neu ask --gaps | neu prove
```

`ask --gaps` ranks the high-value [[link]]s the [[motif]] structure implies but that do not yet exist; `prove` attaches a [[zheng]] proof that each follows from the current structure. the result is a ranked, proof-carrying list of the most valuable missing connections in the graph — actionable by a [[neuron]] or an autonomous agent without re-deriving why. run against your own graph, it is a research agenda that checks its own work.

## what neu is not

- not the runtime — it calls [[tru]]; it never holds φ\* or runs the [[tri-kernel]].
- not a fixed command table — verbs are [[sigil]]s; the grammar is open and self-extending.
- not a mutator outside one axis — only `build` / `stake` / assertions write; every read is pure.

## see also

[[neural]] · [[sentence]] · [[dialect]] · [[sigil]] · [[motif]] · [[wire]] · [[tru]] · [[zheng]] · [[cyb]] · [[frontier]]
