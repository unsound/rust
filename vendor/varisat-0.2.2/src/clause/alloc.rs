//! Clause allocator.
use std::{mem::transmute, slice};

use varisat_formula::{lit::LitIdx, Lit};

use super::{Clause, ClauseHeader, HEADER_LEN};

/// Integer type used to store offsets into [`ClauseAlloc`]'s memory.
type ClauseOffset = u32;

/// Bump allocator for clause storage.
///
/// Clauses are allocated from a single continuous buffer. Clauses cannot be freed individually. To
/// reclaim space from deleted clauses, a new `ClauseAlloc` is created and the remaining clauses are
/// copied over.
///
/// When the `ClauseAlloc`'s buffer is full, it is reallocated using the growing strategy of
/// [`Vec`]. External references ([`ClauseRef`]) store an offset into the `ClauseAlloc`'s memory and
/// remaind valid when the buffer is grown. Clauses are aligned and the offset represents a multiple
/// of the alignment size. This allows using 32-bit offsets while still supporting up to 16GB of
/// clauses.
#[derive(Default)]
pub struct ClauseAlloc {
    buffer: Vec<LitIdx>,
}

impl ClauseAlloc {
    /// Create an emtpy clause allocator.
    pub fn new() -> ClauseAlloc {
        ClauseAlloc::default()
    }

    /// Create a clause allocator with preallocated capacity.
    pub fn with_capacity(capacity: usize) -> ClauseAlloc {
        ClauseAlloc {
            buffer: Vec::with_capacity(capacity),
        }
    }

    /// Allocate space for and add a new clause.
    ///
    /// Clauses have a minimal size of 3, as binary and unit clauses are handled separately. This is
    /// enforced on the ClauseAlloc level to safely avoid extra bound checks when accessing the
    /// initial literals of a clause.
    ///
    /// The size of the header will be set to the size of the given slice. The returned
    /// [`ClauseRef`] can be used to access the new clause.
    pub fn add_clause(&mut self, mut header: ClauseHeader, lits: &[Lit]) -> ClauseRef {
        let offset = self.buffer.len();

        assert!(
            lits.len() >= 3,
            "ClauseAlloc can only store ternary and larger clauses"
        );

        // TODO Maybe let the caller handle this?
        assert!(
            offset <= (ClauseRef::max_offset() as usize),
            "Exceeded ClauseAlloc's maximal buffer size"
        );

        header.set_len(lits.len());

        self.buffer.extend_from_slice(&header.data);

        let lit_idx_slice = unsafe {
            // This is safe as Lit and LitIdx have the same representation
            slice::from_raw_parts(lits.as_ptr() as *const LitIdx, lits.len())
        };

        self.buffer.extend_from_slice(lit_idx_slice);

        ClauseRef {
            offset: offset as ClauseOffset,
        }
    }

    /// Access the header of a clause.
    pub fn header(&self, cref: ClauseRef) -> &ClauseHeader {
        let offset = cref.offset as usize;
        assert!(
            offset as usize + HEADER_LEN <= self.buffer.len(),
            "ClauseRef out of bounds"
        );
        unsafe { self.header_unchecked(cref) }
    }

    /// Mutate the header of a clause.
    pub fn header_mut(&mut self, cref: ClauseRef) -> &mut ClauseHeader {
        let offset = cref.offset as usize;
        assert!(
            offset as usize + HEADER_LEN <= self.buffer.len(),
            "ClauseRef out of bounds"
        );
        unsafe { self.header_unchecked_mut(cref) }
    }

    unsafe fn header_unchecked(&self, cref: ClauseRef) -> &ClauseHeader {
        let offset = cref.offset as usize;
        let header_pointer = self.buffer.as_ptr().add(offset) as *const ClauseHeader;
        &*header_pointer
    }

    /// Mutate the header of a clause without bound checks.
    pub unsafe fn header_unchecked_mut(&mut self, cref: ClauseRef) -> &mut ClauseHeader {
        let offset = cref.offset as usize;
        let header_pointer = self.buffer.as_mut_ptr().add(offset) as *mut ClauseHeader;
        &mut *header_pointer
    }

    /// Access a clause.
    pub fn clause(&self, cref: ClauseRef) -> &Clause {
        let header = self.header(cref);
        let len = header.len();

        // Even on 32 bit systems these additions can't overflow as we never create clause refs with
        // an offset larger than ClauseRef::max_offset()

        let lit_offset = cref.offset as usize + HEADER_LEN;
        let lit_end = lit_offset + len;
        assert!(lit_end <= self.buffer.len(), "ClauseRef out of bounds");
        unsafe { self.clause_with_len_unchecked(cref, len) }
    }

    /// Mutate a clause.
    pub fn clause_mut(&mut self, cref: ClauseRef) -> &mut Clause {
        let header = self.header(cref);
        let len = header.len();

        // Even on 32 bit systems these additions can't overflow as we never create clause refs with
        // an offset larger than ClauseRef::max_offset()

        let lit_offset = cref.offset as usize + HEADER_LEN;
        let lit_end = lit_offset + len;
        assert!(lit_end <= self.buffer.len(), "ClauseRef out of bounds");
        unsafe { self.clause_with_len_unchecked_mut(cref, len) }
    }

    /// Mutate the literals of a clause without bound checks.
    pub unsafe fn lits_ptr_mut_unchecked(&mut self, cref: ClauseRef) -> *mut Lit {
        let offset = cref.offset as usize;
        self.buffer.as_ptr().add(offset + HEADER_LEN) as *mut Lit
    }

    /// Perform a manual bound check on a ClauseRef assuming a given clause length.
    pub fn check_bounds(&self, cref: ClauseRef, len: usize) {
        // Even on 32 bit systems these additions can't overflow as we never create clause refs with
        // an offset larger than ClauseRef::max_offset()

        let lit_offset = cref.offset as usize + HEADER_LEN;
        let lit_end = lit_offset + len;
        assert!(lit_end <= self.buffer.len(), "ClauseRef out of bounds");
    }

    unsafe fn clause_with_len_unchecked(&self, cref: ClauseRef, len: usize) -> &Clause {
        let offset = cref.offset as usize;
        #[allow(clippy::transmute_ptr_to_ptr)]
        transmute::<&[LitIdx], &Clause>(slice::from_raw_parts(
            self.buffer.as_ptr().add(offset),
            len + HEADER_LEN,
        ))
    }

    unsafe fn clause_with_len_unchecked_mut(&mut self, cref: ClauseRef, len: usize) -> &mut Clause {
        let offset = cref.offset as usize;
        #[allow(clippy::transmute_ptr_to_ptr)]
        transmute::<&mut [LitIdx], &mut Clause>(slice::from_raw_parts_mut(
            self.buffer.as_mut_ptr().add(offset),
            len + HEADER_LEN,
        ))
    }

    /// Current buffer size in multiples of [`LitIdx`].
    pub fn buffer_size(&self) -> usize {
        self.buffer.len()
    }
}

/// Compact reference to a clause.
///
/// Used with [`ClauseAlloc`] to access the clause.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct ClauseRef {
    offset: ClauseOffset,
}

impl ClauseRef {
    /// The largest offset supported by the ClauseAlloc
    const fn max_offset() -> ClauseOffset {
        // Make sure we can savely add a length to an offset without overflowing usize
        ((usize::max_value() >> 1) & (ClauseOffset::max_value() as usize)) as ClauseOffset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use varisat_formula::{cnf::strategy::*, CnfFormula, ExtendFormula};

    use proptest::*;

    proptest! {
        #[test]
        fn roundtrip_from_cnf_formula(input in cnf_formula(1..100usize, 0..1000, 3..30)) {

            let mut clause_alloc = ClauseAlloc::new();
            let mut clause_refs = vec![];

            for clause_lits in input.iter() {
                let header = ClauseHeader::new();
                clause_refs.push(clause_alloc.add_clause(header, clause_lits));
            }

            let mut recovered = CnfFormula::new();

            for cref in clause_refs {
                let clause = clause_alloc.clause(cref);
                prop_assert_eq!(clause.header().len(), clause.lits().len());
                recovered.add_clause(clause.lits());
            }

            // Ignore difference caused by unused vars
            recovered.set_var_count(input.var_count());

            prop_assert_eq!(input, recovered);
        }

        #[test]
        fn clause_mutation(input in cnf_formula(1..100usize, 0..1000, 3..30)) {

            let mut clause_alloc = ClauseAlloc::new();
            let mut clause_refs = vec![];

            for clause_lits in input.iter() {
                let header = ClauseHeader::new();
                clause_refs.push(clause_alloc.add_clause(header, clause_lits));
            }

            for &cref in clause_refs.iter() {
                let clause = clause_alloc.clause_mut(cref);
                clause.lits_mut().reverse();
            }

            for &cref in clause_refs.iter() {
                let clause_len = clause_alloc.clause(cref).lits().len();
                if clause_len > 3 {
                    clause_alloc.header_mut(cref).set_len(clause_len - 1);
                }
            }

            for (&cref, lits) in clause_refs.iter().zip(input.iter()) {
                let expected = if lits.len() > 3 {
                    lits[1..].iter().rev()
                } else {
                    lits.iter().rev()
                };
                prop_assert!(clause_alloc.clause(cref).lits().iter().eq(expected));
            }
        }
    }
}
