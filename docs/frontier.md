---
title: the frontier — what neural's primitives can do
alias: frontier, neural frontier, frontier map, neural research
tags: cyber, neural, research, explanation
crystal-type: process
crystal-domain: cyber
status: draft
---

# the frontier — what neural's primitives can do

> the weapon is their language. they gave it all to us. if you learn it, when you really learn it, you begin to perceive time the way that they do. so you can see what's to come.

a frontier survey across linguistics, network science, geometry and topology, AI and mechanistic interpretability, and category theory — read onto the primitives we invented for [[neural]] (`sigil → word → link → sentence → motif → dialect → lexicon`, over the [[tri-kernel]] fixed point φ\*). the question is not "what is neural" but "what can we DO with it," and the answer the frontier gives back is sharper than expected.

## the thesis — structure is foresight

[[neural]] is a language whose meaning lives in the shape of a graph. the Arrival wager is that a language like this, once read fluently, turns structure into foresight: the configuration of [[link]]s around a [[word]] tells you what link comes next. this is not metaphor. higher-order motif configuration predicts future edges better than pairwise history (Zhang et al. 2025); a dialect is characterised by its motif-transition kernel `P(motif_{t+1} | motif_t)` (Liu & Sariyüce 2023); zigzag persistence watches concepts being born and dying as the graph grows (Myers et al. 2023). the whole document below is the map of methods that make "see what's to come" a computation.

## six convergences

scanning five disciplines at the frontier, the same structures keep appearing — often under our own words. these six findings are load-bearing; everything else hangs off them.

### 1. one operator

the [[tri-kernel]] is not three metaphors. it is the graph Laplacian and its heat exponential, the most established object in network science, read three ways:

- diffusion = the heat kernel `e^{−τL}` over the Laplacian spectrum (Donnat et al. 2018) — "diffusion finds bridges" is the off-diagonal `[e^{−τL}]_{ij}` measuring reachability.
- springs = the energy-minimising spectral / Tutte embedding (same `L`, a boundary condition instead of a time) — a [[word]] pulled toward two incompatible clusters has high residual spring energy.
- heat = the scale `τ` — small `τ` is local and literal, large `τ` is diffuse and metaphorical (one scalar slides a word from denotation to connotation).

and φ\* (the fixed point) is simultaneously the leading eigenvector of attention, a PageRank stationary distribution, one GNN message-passing step (GRAND: Chamberlain et al. 2021 — the CGC-GNN isomorphism made literal), and a modern-Hopfield energy minimum (Ramsauer et al. 2020) and the active-inference fixed point (Smithe 2023). these are one fixed point seen five ways. that coherence is neural's biggest asset.

### 2. tension is one phenomenon, four operators

what springs call "tension," every discipline measures and locates:

- negative Ollivier-Ricci curvature on a [[link]] — the optimal-transport cost of reconciling two neighbourhoods; a word's count of negative incident edges is its polysemy (Ollivier; "It Means More if It Sounds Good," 2020).
- nonzero Hodge curl on an edge-flow — a local cycle `A→B→C→A` of "opposite" links that admits no consistent ranking (Jiang-Lim-Yao-Ye 2011).
- nonzero sheaf cohomology `H¹` — locally-held meanings that cannot glue into a global section; a located, computable contradiction (Hansen & Ghrist 2020).
- superposition interference — a feature forced into conflicting circuits sits under measurable energy (Elhage et al. 2022).

springs = curvature = `H¹` = superposition = polysemy. one phenomenon, four off-the-shelf operators (Ollivier-Ricci, Forman/AFRC, the sheaf Laplacian, interference geometry). polysemy stops being a mystery and becomes a graph defect you can locate and repair — split a word at the spectral fault line where the second eigenvector changes sign.

### 3. focus = compression = foresight

φ\* is three things at once: the distributional-semantics signal (meaning-as-position, Lenci & Sahlgren 2023), the eigenvector that concentrates on the nodes that most compress the graph (MDL motif inference, Liu et al. 2024), and the centrality that best predicts future links. focus, description-length, and predictive structure peak at the same nodes. the defensible thesis: meaning, compression, and foresight are one eigenvector viewed three ways.

### 4. the motif is a convergence point

the single richest finding. a "motif" in our sense is the meeting place of six independent traditions that have each been quietly describing the same object:

- a construction (Construction Grammar, Doumen et al. 2024) — a form↔meaning pairing at any grain; the radical, now-computational claim is that lexicon and grammar are one substance, so [[sigil]], [[word]], [[motif]], and [[dialect]] are not separate kinds but one substance at different schematicity.
- a network motif (Milo et al. 2002), defined rigorously by MDL: the subgraph set that most compresses the graph.
- a higher-order cell / simplex (Hodge / simplicial signal processing) — a triad is a filled 2-cell, an [[axon]] is the filled face of a pair; our "motif algebra" is the simplicial operations of join, face, and closure.
- a circuit / feature (mechanistic interpretability, Olah et al. 2020) — interpretability uses the word "motif" verbatim for recurring functional subgraphs, and finds them universal across unrelated networks.
- a persistent homology class (TDA) — a cycle motif is a generator of `H₁` that is born early and dies late; persistence, not an exact template, is what earns motif-hood.
- a string diagram / operadic operation (category theory, Coecke et al.) — a typed wiring with an interface; the motif algebra is operadic substitution.

six lenses, one object. that is our design license.

### 5. pure diffusion is motif-blind

message-passing and plain diffusion are bounded by the 1-Weisfeiler-Leman test: they provably cannot count triangles or cycles (Xu, Morris). so the bare [[tri-kernel]] cannot see the very shapes [[motif]] names. injecting motif counts as features (Graph Substructure Networks, Bouritsas et al. 2022) provably exceeds WL. this is the strongest technical argument in the whole survey: making [[motif]]s first-class is not decoration — it is the only way φ\* perceives the non-linear structure diffusion is blind to. foresight requires it.

### 6. the functorial spine

the whole ladder is one categorical picture (DisCoCat / applied category theory). sigils are the generators of a monoidal/operad category; words are objects (and, in DisCoCirc, persistent wires carrying state); a staked [[link]] is an optic / Bayesian lens (a forward assertion + a backward stake-update — Hedges; Smithe 2022); [[sentence]]s and [[motif]]s are string diagrams; a [[dialect]] is a category presented by generators-and-relations, or a sheaf of local meanings (langue = the category, parole = diagrams drawn in it); φ\* is the active-inference fixed point of the semantic functor. meaning is functorial: define it on the generators, and composition is forced. and the punchline — motif composition and Arrival-style foresight are the *same* string-diagram calculus (Smithe's "active inference in string diagrams" is DisCoCat's calculus reused), so building meaning and predicting it are one operation.

## the primitives at the frontier

what each primitive can BE and DO, once the frontier is mapped onto it.

### sigil

the densest operator. Peirce's icon / index / symbol trichotomy is a sigil-level alphabet of reference modes (resemblance / pointing / convention); a [[word]]'s type is its position in that sign space. the ~15 universal image schemas (PATH, CONTAINER, LINK, BALANCE, SOURCE-PATH-GOAL) are the most primitive, embodied, near-universal motifs — the bridge from sigil to motif, and a principled seed vocabulary. in heat-kernel terms the sigil is the small-`τ` (local) end of the one scale that runs the whole ladder up to dialect.

### word

a typed [[particle]] — and at the frontier a word is a point in a (mixed-curvature) manifold whose φ\* belief is a distribution, not a point (information geometry; Fisher metric). it is a monosemantic dictionary feature mined by a sparse autoencoder (Anthropic 2023); a KG entity acted on by relation-operators (TransE, RotatE). polysemy is the word occupying conflicting orbits / superposition / negative curvature — measurable and splittable.

### link and axon

a [[link]] is a staked, directed edge — which is exactly an optic / Bayesian lens (forward assertion, backward stake), so the [[reward]] (Shapley of Δφ\*) composes along sentences and motifs by optic composition. structurally a link is a 1-simplex; an [[axon]] (all links on a pair, itself a [[particle]]) is the filled 2-simplex, a hyperedge, a sheaf edge-stalk. homoiconicity (links about links) = the simpliciality axiom = the category being closed. the Hodge Laplacian operates identically on links and axons, so promotion up the ladder is geometrically forced, not bolted on.

### sentence

a [[sentence]] is a linear chain — a string diagram with linear topology, a composite of morphisms. its coherence is its Hodge gradient component: a consistent argument has near-zero curl, a self-contradicting one (`A→B→C→A`) is pure curl. sentence-type (assertion / argument / narrative) is position in the Hodge decomposition. a sentence and its non-linear sibling are dual views of one meaning, and the duality is trainable (AMR dual encoders, Hong et al. 2024).

### motif

the priority — its own section below.

### dialect

a convention, and at the frontier a [[dialect]] is four compatible things: a low-motif-conductance community discovered by re-running φ\*'s spectral machinery on the motif-weighted graph (Benson et al. 2016); a graphon whose identity is its motif-density spectrum, with provable transferability of φ\*-filters to larger instances (Ruiz et al. 2021); a sheaf of local meanings whose global consistency is a cohomology computation; and the attractor set of motifs that survives a transmission bottleneck (neural iterated learning, Ren et al. 2020). discovery-not-design has named mechanisms. dialect schism is a bifurcation of φ\*.

### lexicon

the top [[word]]s by φ\* — the high-mass objects of the stationary distribution, the SAE dictionary, the convergent Platonic vocabulary (Huh et al. 2024). its natural geometry is a mixed-curvature product manifold ℍ×𝕊×𝔼: hierarchy in hyperbolic space (lexicon = low-norm roots, Nickel & Kiela 2017), cyclic motifs on the sphere, flat clusters in Euclidean space. a Mapper graph of φ\*-space is the lexicon's visible skeleton.

## the motif — what it can be and do

the user's instinct is right: motifs are the mind-bending primitive. they are the poems, the reusable units of meaning. the frontier says a motif is, all at once:

1. a typed string diagram (a morphism with an interface — open wires await arguments, closed wires are a finished meaning).
2. an operadic operation (the motif algebra is operad substitution).
3. a higher-order cell carrying its own Laplacian (homoiconic, promotable to a node — exactly axon-as-particle).
4. a functional circuit (the feed-forward loop filters noise / detects persistence; a chain is inference; a diamond is corroboration; a cycle is self-reference or contradiction; a star is a hub or a definition).
5. a persistent homology class with a curvature signature and a sheaf-section status.
6. a generative-model factor — a reusable sub-circuit that makes prediction cheap, hence selected because it lowers free energy.

what motifs DO, that nothing below them can:

- carry meaning beyond their parts. "kick the bucket," and equally a fixed subgraph shape, means what its links do not. this is precisely where distributional vectors fail (the known compositional ceiling), so it is neural's territory: meaning that lives in shape, not in node content.
- reuse by morphism — the deepest one. metaphor is a motif morphism: a structure-preserving map of a validated subgraph onto a new frame (TIME-IS-MONEY maps the COMMERCE star onto the TIME frame). this is how a finite motif vocabulary covers unbounded meaning. the motif algebra therefore needs a mapping operator, not only composition.
- compress the graph — a good motif, named once and instanced many times, is a shorter description (MDL), directly tradeable against φ\*.
- ossify into grammar — a recurring sentence-motif loses content and gains structure over time (grammaticalization), drifting from word-like toward sigil-like; a measurable, directional process, and the causal semantic map predicts which motif grammaticalises next.
- predict — a motif transition kernel `P(m' | m)` is the dialect's grammar of expectation; recognising "which motif am I inside" constrains the next link. this is the Arrival engine.
- compute — beat the WL ceiling: motifs are exactly the structure φ\* is otherwise blind to.

### the motif algebra, formalised

operad substitution gives the four operations a rigorous backbone (Spivak; Fong & Spivak 2018), and double-pushout graph rewriting gives them productions with termination guarantees (Ehrig et al.):

- concatenation = monoidal tensor (place two motifs side by side).
- nesting = operadic substitution (plug a motif into a hole of another).
- intersection = pullback / shared interface (the pushout's `K`).
- complement = the under-represented, forbidden subgraph — an anti-motif that is itself meaning.
- and morphism = the mapping of one motif onto another frame (metaphor), the operator that makes the vocabulary unbounded.

because an operad algebra is a functor, defining meaning only on the [[sigil]] generators forces a canonical φ\*-meaning on every composite motif. that is compositional semantics for free.

### the topological fingerprint

every motif the algebra produces carries a triple that survives renaming and relabelling: (persistence lifetime, curvature signature, `H¹` status). persistence says whether the shape is real or noise and, via zigzag, whether it is about to be born or to die; curvature says whether it is a settled meaning (positive, a tight triad) or a fragile fault line (negative, a bridge); `H¹` says whether it is internally coherent (a stable morpheme) or a built-in paradox (an odd cycle of opposite links). this triple is a complete, computable signature of a motif's health and its semantic role.

## how the primitives bind into one thing

three independent spines, the same shape.

- the categorical spine. `sigils → words → links → sentences/motifs` is `generators → objects → morphisms → diagrams` in a free monoidal/operad category; [[dialect]] is the category (or a sheaf over it); φ\* is the fixed point of the interpreting functor; foresight is its forward pass; [[reward]] is the optic's backward pass; evolution is selection on the generators.
- the Hodge / simplicial spine. link = 1-simplex, axon = filled 2-simplex, motif = complex; promotion is the "k-simplex as node" move and homoiconicity is simpliciality. the Hodge decomposition draws a principled boundary: [[sentence]] = gradient flow (linear, consistent), [[motif]] = curl / harmonic flow (non-linear, cyclic). one decomposition separates our two structural primitives.
- the spectral spine. φ\* is the leading mode of the (motif/Hodge-weighted) Laplacian; the [[lexicon]] is its top entries; polysemy is spectral spread (spring residual = curl = orbit conflict = superposition). one eigenproblem yields φ\*, the lexicon, dialect discovery, and the tension meter as different readings.

these are not three analogies — they are one operator (the Laplacian) promoted up one ladder (the simplicial complex) and interpreted by one functor (the dialect). that is the conceptual unity a language called neural should have.

## what we can do — the programs

the moonshots from all five domains, consolidated and prioritised. each is buildable and each maps to the primitives.

1. the Arrival engine (foresight). estimate the motif-transition kernel `P(m' | m)` per dialect as a temporal point process, augment it with a JEPA-style predictor in motif-embedding space and an active-inference (expected-free-energy) lookahead, and surface the most probable next [[link]] before anyone stakes it. a [[neuron]] fluent in reading motifs reads the graph's future. this is the headline capability.

2. motif-dictionary mining (the auto-lexicon). train a sparse autoencoder over φ\*-message vectors and run MDL compression-based motif inference continuously; the motif set that best compresses the [[cybergraph]] is the emergent [[dialect]], discovered not designed. watch motifs split into families as the dictionary grows; new motifs that start compressing well are neologisms, ones that stop are dying constructions — a live readout of the language evolving.

3. super-WL focus via motif injection. augment the [[tri-kernel]] with motif-count features so φ\* provably perceives the triangles, cycles, and diamonds plain diffusion cannot. necessary for both foresight and meaning.

4. the tension meter. compute Ollivier-Ricci (exact, on important links) + Forman/AFRC (linear-time, whole graph) curvature, Hodge curl, and sheaf `H¹`; flag and locate every polysemous [[word]] and every self-contradicting [[dialect]] loop, then auto-split a word at its spectral fault line. polysemy and contradiction become located, repairable defects.

5. metaphor as motif morphism. implement `map(motif_A → frame_B)` that projects a validated subgraph onto another graph region, scored by structural fit — genuine analogical inference, and the mechanism by which a finite motif vocabulary expresses unbounded meaning.

6. the Hodge-lifted tri-kernel. promote axons and motifs to cells and run the same diffusion/springs/heat with Hodge Laplacians `L₁, L₂`. meaning then has a gradient (the consistent worldview), a curl (live contradictions), and a harmonic part (irreducible open questions — holes in the [[lexicon]]). a literal topology of what the network believes, disputes, and cannot resolve.

7. the string-diagram calculus (neural-as-DisCoPy) + operadic motif algebra. compile a [[motif]]/[[sentence]] diagram to its φ\*-semantics by a functor, with a normal-form rewriter and termination check that doubles as a contradiction detector. executable, compositional, interpretable meaning — the same path lambeq already walks for QNLP.

8. staked links as optics. model each staked [[link]] as an optic so the [[reward]] composes along sentences and motifs by optic composition — end-to-end, auditable incentive flow that matches the Shapley-of-Δφ\* design.

9. the geometry of the lexicon. embed words in a product ℍ×𝕊×𝔼 so a word's position encodes its role (tree-node vs cycle-member vs cluster-member), with the lexicon as the low-norm hyperbolic roots; align two dialects with no shared words by Gromov-Wasserstein on relational structure alone (translation without a Rosetta stone).

10. dialect bootstrapping by iterated learning + selection. evolve dialects through a transmission bottleneck so compositional motifs self-select (with TRUE/FALSE as the initial signaling-game payoff), and score motifs by assembly index × copy-number (Walker & Cronin 2023) under memetic selection so the language provably accumulates open-ended complexity and bifurcations are logged as dated language-change events.

11. concept-birth radar. run zigzag persistence over the growing graph: a persistent `H₁` loop being born is a concept crystallising, a long-lived loop dying is a gap being filled. present structure becomes near-future foresight as a measurable signal.

12. the sigil algebra. ground the sigils in the ~15 image schemas crossed with Peirce's icon/index/symbol modes, making "almost anything expressible with sigils" a testable claim rather than an aspiration.

## open problems

what to formalise before building.

- the motif's canonical definition: reconcile the six lenses into one operational definition (lead candidate: a persistent homology class carrying a (persistence, curvature, `H¹`) fingerprint, that also minimises description length).
- the morphism operator: a rigorous, scored metaphor map, and the conditions under which a motif transfers validly to a new frame.
- cross-algebra soundness of the Hodge-lifted tri-kernel, and whether φ\*-dynamics differ on a hypergraph vs a simplicial representation of the same motifs (Nat. Commun. 2023 says they do).
- the unity claim, made a theorem: focus = compression = foresight as one eigenvector, stated and proven over the stake-weighted graph.
- foresight under [[zheng]]: can motif-transition prediction be made a verifiable query (a [[zheng]] proof that the predicted next link follows from the current motif structure)?

## sources

linguistics & semiotics — Doumen, Beuls, Van Eecke, "The Computational Learning of Construction Grammars" (2024); Bonn et al., Uniform Meaning Representation (2024); CLICS³ colexification (List et al.); FrameBERT + image schemas (Li et al. 2023); Ren et al., neural iterated learning (ICLR 2020); Lenci & Sahlgren, *Distributional Semantics* (2023); weak linguistic relativity in LLMs (arXiv 2506.16151, 2025); Peirce, existential graphs / sign taxonomy.

graph theory & network science — Milo et al., network motifs (Science 2002); Benson, Gleich, Leskovec, higher-order organisation (Science 2016); Donnat et al., GraphWave (KDD 2018); Jiang-Lim-Yao-Ye, combinatorial Hodge theory (2011); Schaub, Barbarossa, Bianconi, topological signal processing; Lotito et al., higher-order motif analysis in hypergraphs (2022); Liu & Sariyüce, temporal motif transitions (2023) + stochastic event prediction (2026); Bouritsas et al., Graph Substructure Networks (TPAMI 2022); Ruiz, Chamon, Ribeiro, graphon signal processing (2021); Liu et al., compression-based motif sets (PLOS Comp Bio 2024); Ehrig et al., DPO graph grammars.

geometry, topology & curvature — Bodnar et al., neural sheaf diffusion (NeurIPS 2022); Hansen & Ghrist, opinion dynamics on discourse sheaves (2020); Nguyen et al., Ollivier-Ricci over-smoothing/squashing (ICML 2023); AFRC curvature (2023-24); "It Means More if It Sounds Good," Ollivier-Ricci polysemy (2020); persistent & zigzag homology (Myers et al., EPJ Data Science 2023); Nickel & Kiela, Poincaré / Lorentz embeddings (2017-18); Gu, Sala et al., mixed-curvature product manifolds (ICLR 2018); Bronstein et al., geometric deep learning (2021); Alvarez-Melis & Jaakkola, Gromov-Wasserstein alignment (2018); Amari, information geometry.

AI, neural networks & interpretability — Chamberlain et al., GRAND graph neural diffusion (ICML 2021); Joshi, transformers are GNNs (2025); Olah, Cammarata et al., circuits & motifs (Distill 2020); Olsson et al., induction heads (Anthropic 2022); Elhage et al., toy models of superposition (2022); Bricken et al., sparse autoencoders / monosemanticity (2023) + crosscoders (2024); Huh et al., the Platonic Representation Hypothesis (ICML 2024); Ramsauer et al., modern Hopfield networks (2020); Sun et al., RotatE (2019); LeCun, JEPA (2023-25); Edge et al., GraphRAG (2024); Xie et al., in-context learning as Bayesian inference (2022); WL expressivity bounds (Xu, Morris).

category theory, compositional semantics & complex systems — Coecke, Sadrzadeh, Clark, DisCoCat (2010) + DisCoCirc (2020); Kartsaklis et al., lambeq / DisCoPy (2021); Fong & Spivak, *Seven Sketches in Compositionality* (2018); Spivak, operad of wiring diagrams (2013); Hedges; Capucci, Gavranović, categorical cybernetics / optics (2021); Smithe, compositional active inference / Bayesian lenses (2022) + active inference in string diagrams (2023); Parr, Pezzulo, Friston, *Active Inference* (MIT Press 2022); Abramsky & Sadrzadeh, sheaf-theoretic semantic unification (2014); Walker & Cronin, assembly theory (Nature 2023); Cavalli-Sforza & Feldman, cultural transmission (1981).

---

discover all [[concepts]]. see [[neural]] for the language, the `specs/` for the primitives, and `explanation/` for the whitepaper.
