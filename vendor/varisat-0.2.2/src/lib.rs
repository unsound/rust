//! Varisat is a [CDCL][cdcl] based SAT solver written in rust. Given a boolean formula in
//! [conjunctive normal form][cnf], it either finds a variable assignment that makes the formula
//! true or finds a proof that this is impossible.
//!
//! In addition to this API documentation, Varisat comes with a [user manual].
//!
//! [cdcl]: https://en.wikipedia.org/wiki/Conflict-Driven_Clause_Learning
//! [cnf]: https://en.wikipedia.org/wiki/Conjunctive_normal_form
//! [user manual]: https://jix.github.io/varisat/manual/0.2.1/

pub mod config;
pub mod solver;

mod analyze_conflict;
mod assumptions;
mod binary;
mod cdcl;
mod clause;
mod context;
mod decision;
mod glue;
mod load;
mod model;
mod proof;
mod prop;
mod schedule;
mod state;
mod tmp;
mod unit_simplify;
mod variables;

pub use solver::{ProofFormat, Solver};
pub use varisat_formula::{cnf, lit, CnfFormula, ExtendFormula, Lit, Var};

pub mod dimacs {
    //! DIMCAS CNF parser and writer.
    pub use varisat_dimacs::*;
}

pub mod checker {
    //! Proof checker for Varisat proofs.
    pub use varisat_checker::{
        CheckedProofStep, Checker, CheckerData, CheckerError, ProofProcessor,
        ProofTranscriptProcessor, ProofTranscriptStep,
    };
}
