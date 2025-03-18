//! Clause storage.
use std::slice;

use varisat_formula::{lit::LitIdx, Lit};

pub mod activity;
pub mod alloc;
pub mod assess;
pub mod db;
pub mod gc;
pub mod header;
pub mod reduce;

pub use activity::{bump_clause_activity, decay_clause_activities, ClauseActivity};
pub use alloc::{ClauseAlloc, ClauseRef};
pub use assess::{assess_learned_clause, bump_clause};
pub use db::{ClauseDb, Tier};
pub use gc::collect_garbage;
pub use header::ClauseHeader;

use header::HEADER_LEN;

/// A clause.
///
/// This is stoed in a [`ClauseAlloc`] and thus must have a representation compatible with slice of
/// [`LitIdx`] values.
///
/// It would be nicer to use a DST struct with two members and `repr(C)`, but while that can be
/// declared in stable rust, it's almost impossible to work with.
#[repr(transparent)]
pub struct Clause {
    data: [LitIdx],
}

impl Clause {
    /// The clause's header
    pub fn header(&self) -> &ClauseHeader {
        unsafe {
            let header_ptr = self.data.as_ptr() as *const ClauseHeader;
            &*header_ptr
        }
    }

    /// Mutable reference to the clause's header
    pub fn header_mut(&mut self) -> &mut ClauseHeader {
        unsafe {
            let header_ptr = self.data.as_mut_ptr() as *mut ClauseHeader;
            &mut *header_ptr
        }
    }

    /// The clause's literals
    pub fn lits(&self) -> &[Lit] {
        unsafe {
            let lit_ptr = self.data.as_ptr().add(HEADER_LEN) as *const Lit;
            slice::from_raw_parts(lit_ptr, self.data.len() - HEADER_LEN)
        }
    }

    /// Mutable slice of the clause's literals
    pub fn lits_mut(&mut self) -> &mut [Lit] {
        unsafe {
            let lit_ptr = self.data.as_mut_ptr().add(HEADER_LEN) as *mut Lit;
            slice::from_raw_parts_mut(lit_ptr, self.data.len() - HEADER_LEN)
        }
    }
}
