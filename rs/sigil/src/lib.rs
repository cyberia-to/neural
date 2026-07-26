//! neural sigil — the surface syntax.
//!
//! Two jobs at slice-1: turn a line of text into a typed [`Sentence`], and
//! resolve a `word` to its [`Particle`] (a typed particle is a word; here we
//! mint the particle — its content identity — from the name). Everything a
//! neuron speaks enters the graph through this crate.

/// A particle id — a 32-byte content address (the same shape as `bbg::Particle`).
pub type Particle = [u8; 32];

/// A parsed neural sentence: `subject —relation→ object`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sentence {
    pub subject: String,
    pub relation: String,
    pub object: String,
}

/// Why a line is not a well-formed sentence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// A slice-1 sentence is exactly three tokens; this had `found`.
    Arity { found: usize },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Arity { found } => {
                write!(f, "expected 3 tokens (subject relation object), got {found}")
            }
        }
    }
}
impl std::error::Error for ParseError {}

/// Parse `"cat is-a animal"` into a [`Sentence`]. Slice-1 grammar: exactly three
/// whitespace-separated tokens — subject, relation, object. Richer grammar
/// (sigil-explicit `#cat ~is-a #animal`, multi-link chains) layers on later.
pub fn parse(input: &str) -> Result<Sentence, ParseError> {
    let toks: Vec<&str> = input.split_whitespace().collect();
    if toks.len() != 3 {
        return Err(ParseError::Arity { found: toks.len() });
    }
    Ok(Sentence {
        subject: toks[0].to_string(),
        relation: toks[1].to_string(),
        object: toks[2].to_string(),
    })
}

/// Resolve a word to its particle — the hemera hash of its name. Deterministic:
/// the same name is the same particle everywhere in the graph, so speaking a
/// word *merges* into the graph rather than minting a fresh node.
pub fn word_particle(name: &str) -> Particle {
    let h = cyber_hemera::hash(name.as_bytes());
    let b = h.as_bytes();
    let mut out = [0u8; 32];
    let n = b.len().min(32);
    out[..n].copy_from_slice(&b[..n]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_triple() {
        let s = parse("cat is-a animal").unwrap();
        assert_eq!(
            s,
            Sentence { subject: "cat".into(), relation: "is-a".into(), object: "animal".into() }
        );
    }

    #[test]
    fn word_is_deterministic_and_distinct() {
        assert_eq!(word_particle("cat"), word_particle("cat"));
        assert_ne!(word_particle("cat"), word_particle("animal"));
    }

    #[test]
    fn rejects_non_triples() {
        assert!(parse("cat animal").is_err());
        assert!(parse("a b c d").is_err());
    }
}
