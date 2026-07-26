//! neu — the neural CLI (slice-1).
//!
//! Speak one sentence, land it as a typed relation on the local cell, read it
//! back. neu is a *client of the cell*: it parses (sigil) → expands (dialect) →
//! writes through [`cyb_core::Cell`], and reads via inf. It holds no runtime of
//! its own.
//!
//! ```text
//! neu "cat is-a animal"   # speak: parse → expand → land on the cell
//! neu focus               # read: the graph's nodes by energy
//! ```

use std::path::PathBuf;

use cyb_core::Cell;
use cybergraph::Particle;

/// One directed cyberlink: `from → to`.
type Edge = (Particle, Particle);

/// The authoring neuron. Slice-1 uses a fixed local identity; a real key +
/// `H(secret)=address` proof (mudra) lands later.
fn neuron() -> Particle {
    sigil::word_particle("@you")
}

/// Where the durable cell logs its signals. Override with `NEU_LOG`.
fn graph_path() -> PathBuf {
    std::env::var("NEU_LOG")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::temp_dir().join("neu").join("graph.log"))
}

/// Parse → expand → land the sentence as **one atomic signal** on the cell.
/// Returns the edges staked. A sentence lands whole or not at all — the dialect
/// builds the base link and its axon-type link, and both ride one signal.
fn speak(cell: &mut Cell, input: &str) -> Result<Vec<Edge>, String> {
    let sentence = sigil::parse(input).map_err(|e| e.to_string())?;
    let edges = dialect::expand(&sentence);
    cell.cast(neuron(), edges.iter().copied()).map_err(|e| format!("{e:?}"))?;
    Ok(edges)
}

/// First 4 bytes of a particle, hex — enough to eyeball identity in the demo.
fn short(p: &Particle) -> String {
    p[..4].iter().map(|b| format!("{b:02x}")).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut cell = Cell::open(graph_path()).expect("open cell");

    match args.first().map(String::as_str) {
        None | Some("focus") => {
            let mut nodes = cell.nodes();
            nodes.sort_by(|a, b| b.1.cmp(&a.1));
            println!("● focus — {} particles, {} axons", cell.particles(), cell.axons().len());
            for (p, energy) in nodes.iter().take(20) {
                println!("  {}  energy {energy}", short(p));
            }
        }
        _ => {
            let input = args.join(" ");
            match speak(&mut cell, &input) {
                Ok(edges) => {
                    println!("● spoke: {input}");
                    println!("  base  {} → {}", short(&edges[0].0), short(&edges[0].1));
                    println!(
                        "  type  {} → {}   (on the axon of the pair)",
                        short(&edges[1].0),
                        short(&edges[1].1)
                    );
                }
                Err(e) => {
                    eprintln!("✗ {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speak_lands_a_typed_relation() {
        let mut cell = Cell::ephemeral();
        let edges = speak(&mut cell, "cat is-a animal").expect("speak");
        assert_eq!(edges.len(), 2, "base link + axon-type link");

        let cat = sigil::word_particle("cat");
        let animal = sigil::word_particle("animal");
        let is_a = sigil::word_particle("is-a");

        // the base pair: the object particle materialized
        assert!(cell.has_particle(&animal), "object particle exists");

        // the relation is a link ON the axon of (cat, animal) — the settled model
        let axon = bbg::state::axon_id(&cat, &animal);
        assert!(
            cell.axons().iter().any(|(from, to, _)| *from == axon && *to == is_a),
            "typed relation staked: axon(cat,animal) → is_a"
        );

        // the read path runs
        cell.query("?[particle, energy] := particles{particle, energy}").expect("query runs");
    }

    #[test]
    fn ill_formed_sentence_is_rejected() {
        let mut cell = Cell::ephemeral();
        assert!(speak(&mut cell, "cat animal").is_err(), "a non-triple is not a sentence");
    }

    #[test]
    fn sentence_is_one_atomic_signal() {
        let mut cell = Cell::ephemeral();
        let before = cell.len();
        let edges = speak(&mut cell, "cat is-a animal").expect("speak");
        assert_eq!(edges.len(), 2, "two links");
        // ...but ONE signal — the whole sentence is atomic, not two commits
        assert_eq!(cell.len() - before, 1, "the sentence lands as one signal, not two");

        // and that one signal still carries both the base link and its axon-type
        let cat = sigil::word_particle("cat");
        let animal = sigil::word_particle("animal");
        let is_a = sigil::word_particle("is-a");
        let axon = bbg::state::axon_id(&cat, &animal);
        assert!(cell.has_particle(&animal), "base link landed");
        assert!(
            cell.axons().iter().any(|(from, to, _)| *from == axon && *to == is_a),
            "axon-type link landed in the same signal"
        );
    }
}
