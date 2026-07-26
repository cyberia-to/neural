//! neural dialect — expands a [`sigil::Sentence`] into the cyberlinks it lands as.
//!
//! A dialect supplies structure for intent: the neuron says *what*
//! (`cat is-a animal`), the dialect builds the *right links*. This is the
//! settled relation model — a typed relation is a link **on the axon** of the
//! base pair:
//!
//! ```text
//! cat is-a animal   →   [ cat → animal ,  H(cat,animal) → is_a ]
//!                         └ the bare edge   └ the type, on its axon
//! ```
//!
//! The wire stays untyped (`{from,to,…}`); the relation-word `is_a` is a
//! first-class particle φ* can rank; and one pair can carry several types
//! (`→ is_a`, `→ causes`) whose tension is the polysemy signal. No new
//! mechanism — the axon `H(from,to)` already materializes on link (bbg A6).

use sigil::{word_particle, Particle, Sentence};

/// One directed cyberlink to stake: `from → to`.
pub type Edge = (Particle, Particle);

/// Expand a sentence into its ordered edges under the base dialect:
///   1. the bare relationship — `subject → object`
///   2. the type on its axon   — `axon_id(subject,object) → relation-word`
pub fn expand(s: &Sentence) -> Vec<Edge> {
    let subject = word_particle(&s.subject);
    let object = word_particle(&s.object);
    let relation = word_particle(&s.relation);
    let axon = bbg::state::axon_id(&subject, &object);
    vec![(subject, object), (axon, relation)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_to_base_and_axon_type() {
        let s = sigil::parse("cat is-a animal").unwrap();
        let edges = expand(&s);
        assert_eq!(edges.len(), 2, "base link + axon-type link");

        // 1. the bare pair
        assert_eq!(edges[0], (word_particle("cat"), word_particle("animal")));

        // 2. the type sits on the axon of the base pair, not on a new node
        let axon = bbg::state::axon_id(&word_particle("cat"), &word_particle("animal"));
        assert_eq!(edges[1], (axon, word_particle("is-a")));
    }
}
