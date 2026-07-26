# neu — build plan

The neural CLI (`neu` for adoption; the native tongue of the [[cyb]] terminal inside the ecosystem). A homoiconic client of the cell: parse → expand → seal, read via inf. Rides `nox → bbg → cybergraph → inf` — **no new core deps**. Spec: `neural/specs/cli.md`; substrate seam: `neural/specs/binding.md`; relation model: `neural/specs/relation.md`.

Each slice is one green test — the same discipline as the cyb cell.

## ✅ Slice 1 — speak a typed relation (DONE)

`neu "cat is-a animal"` → sigil-parse → dialect expands to `[cat→animal, H(cat,animal)→is_a]` → lands on a durable `Cell` → `neu focus` reads it back, the relation-word ranked by focus.

- workspace `neural/rs/` (`sigil`, `dialect`, `cli`), path-deps to the sibling crates.
- **6 tests green**, full stack compiles ~15s. Binary runs: two sentences land, `is_a` shows energy 2 (relations are ranked particles, not super-nodes), object merges across sentences.
- **verified**: `RUSTC_BOOTSTRAP=1 cargo test --workspace`.

## ✅ Slice 2 — the sentence is atomic (DONE)

Base + axon-type links now fold into **one signal**. Added `Cell::cast(neuron, links)` in cyb-core (the durable choke point); `Cell::link` is now a one-link `cast`. `neu::speak` casts the whole sentence at once.

- **verified**: new test `sentence_is_one_atomic_signal` — two links, but `cell.len()` grows by exactly 1 (one signal), and both the base and the axon-type link are present. neu cli 3/3 green.
- **no regression**: all 5 cyb-core `Cell` tests still pass after the `link`→`cast` refactor (chain, gossip, durable).
- resolves the open "atomic cast seam" — chose the `Cell` method so the durable log stays the one choke point.

## Slice 3 — read is neural, not raw inf

Wrap the ~5 canned inf templates behind verbs: `focus` (particles by φ\*), `neighborhood W..X`, `path A..B`, and the typed-relation join (`neu ls --rel is-a` → all `X is-a Y` via the `axons` join). Design once, reuse. Test: `X is-a Y` round-trips through the query, not through `cell.axons()`.

## Slice 4 — dialect scope + genesis vocabulary

`--dialect D` scopes resolution. Ship the bootloader relation-words (`TRUE`/`FALSE` + `is-a`/`causes`/`part-of`/`contradicts` + the image schemas). `neu dialect ls/diff`. Test: the same word resolves to different relations under two dialects; `diff` surfaces a collision.

## Slice 5 — identity & proof-as-signature

Real neuron key; attach the `H(secret)=address` proof (mudra) to the signal instead of the fixed `@you`. Test: a signal carries a verifiable author; a forged author is rejected.

## Slice 6 — the REPL

`neu` (no args) → the live loop: speak, watch φ\* / lexicon move, read. Fuses the three faces. Test: a session of sentences converges the same state as the equivalent one-shots.

## Later — the hero (waits on tru)

`neu ask [--gaps]` (motif-transition foresight) and `neu prove` (zheng) need [[tru]]'s tri-kernel + the motif surface. Design is in `cli.md`; the engine is the MIND layer, post-testnet. Ship the write+read core first.

## Open decisions (gate specific slices)

- **graphlet vs motif** — gates `motif`'s output schema and `ask` (Slice ≥ hero). Settle the term (graphlet = form, motif = significant form) before `motif.md` is frozen.
- **`sigils.md` reconciliation** — the older markup-framed spec still lists obsolete future-work (negation → valence, weight → stake, typed-edges → `relation.md`); prune before the full sigil grammar lands (Slice 3+).
- **atomic cast seam** — Slice 2: add to `Cell` (touches cyb-core) vs build the signal in neu and apply via `cell.graph`. Prefer the `Cell` method — keeps the durable log as the one choke point.
