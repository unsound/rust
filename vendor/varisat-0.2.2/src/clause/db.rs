//! Database for long clauses.
use std::mem::transmute;

use partial_ref::{partial, PartialRef};

use varisat_formula::Lit;

use crate::{
    context::{parts::*, Context},
    prop::Reason,
};

use super::{header::HEADER_LEN, ClauseAlloc, ClauseHeader, ClauseRef};

/// Partitions of the clause database.
///
/// The long clauses are partitioned into 4 [`Tier`]s. This follows the approach described by
/// Chanseok Oh in ["Between SAT and UNSAT: The Fundamental Difference in CDCL
/// SAT"](https://doi.org/10.1007/978-3-319-24318-4_23), section 4.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Tier {
    Irred = 0,
    Core = 1,
    Mid = 2,
    Local = 3,
}

impl Tier {
    /// Total number of tiers.
    pub const fn count() -> usize {
        4
    }

    /// Cast an index into the corresponding tier.
    pub unsafe fn from_index(index: usize) -> Tier {
        debug_assert!(index < Tier::count());
        transmute(index as u8)
    }
}

/// Database for long clauses.
///
/// Removal of clauses from the `clauses` and the `by_tier` fields can be delayed. The clause
/// header's deleted and tier fields need to be checked when iterating over these. `by_tier` may
/// also contain duplicate entries.
#[derive(Default)]
pub struct ClauseDb {
    /// May contain deleted clauses, see above
    pub(super) clauses: Vec<ClauseRef>,
    /// May contain deleted and moved clauses, see above
    pub(super) by_tier: [Vec<ClauseRef>; Tier::count()],
    /// These counts should always be up to date
    pub(super) count_by_tier: [usize; Tier::count()],
    /// Size of deleted but not collected clauses
    pub(super) garbage_size: usize,
}

impl ClauseDb {
    /// The number of long clauses of a given tier.
    pub fn count_by_tier(&self, tier: Tier) -> usize {
        self.count_by_tier[tier as usize]
    }
}

/// Add a long clause to the database.
pub fn add_clause(
    mut ctx: partial!(Context, mut ClauseAllocP, mut ClauseDbP, mut WatchlistsP),
    header: ClauseHeader,
    lits: &[Lit],
) -> ClauseRef {
    let tier = header.tier();

    let cref = ctx.part_mut(ClauseAllocP).add_clause(header, lits);

    ctx.part_mut(WatchlistsP)
        .watch_clause(cref, [lits[0], lits[1]]);

    let db = ctx.part_mut(ClauseDbP);

    db.clauses.push(cref);
    db.by_tier[tier as usize].push(cref);
    db.count_by_tier[tier as usize] += 1;

    cref
}

/// Change the tier of a long clause.
///
/// This is a noop for a clause already of the specified tier.
pub fn set_clause_tier(
    mut ctx: partial!(Context, mut ClauseAllocP, mut ClauseDbP),
    cref: ClauseRef,
    tier: Tier,
) {
    let (alloc, mut ctx) = ctx.split_part_mut(ClauseAllocP);
    let db = ctx.part_mut(ClauseDbP);

    let old_tier = alloc.header(cref).tier();
    if old_tier != tier {
        db.count_by_tier[old_tier as usize] -= 1;
        db.count_by_tier[tier as usize] += 1;

        alloc.header_mut(cref).set_tier(tier);
        db.by_tier[tier as usize].push(cref);
    }
}

/// Delete a long clause from the database.
pub fn delete_clause(
    mut ctx: partial!(Context, mut ClauseAllocP, mut ClauseDbP, mut WatchlistsP),
    cref: ClauseRef,
) {
    // TODO Don't force a rebuild of all watchlists here
    ctx.part_mut(WatchlistsP).disable();

    let (alloc, mut ctx) = ctx.split_part_mut(ClauseAllocP);
    let db = ctx.part_mut(ClauseDbP);

    let header = alloc.header_mut(cref);

    debug_assert!(
        !header.deleted(),
        "delete_clause for already deleted clause"
    );

    header.set_deleted(true);

    db.count_by_tier[header.tier() as usize] -= 1;

    db.garbage_size += header.len() + HEADER_LEN;
}

/// Delete a long clause from the database unless it is asserting.
///
/// Returns true if the clause was deleted.
pub fn try_delete_clause(
    mut ctx: partial!(
        Context,
        mut ClauseAllocP,
        mut ClauseDbP,
        mut WatchlistsP,
        ImplGraphP,
        AssignmentP,
    ),
    cref: ClauseRef,
) -> bool {
    let initial_lit = ctx.part(ClauseAllocP).clause(cref).lits()[0];
    let asserting = ctx.part(AssignmentP).lit_is_true(initial_lit)
        && ctx.part(ImplGraphP).reason(initial_lit.var()) == &Reason::Long(cref);

    if !asserting {
        delete_clause(ctx.borrow(), cref);
    }
    !asserting
}

/// Iterator over all long clauses.
///
/// This filters deleted (but uncollected) clauses on the fly.
pub fn clauses_iter<'a>(
    ctx: &'a partial!('a Context, ClauseAllocP, ClauseDbP),
) -> impl Iterator<Item = ClauseRef> + 'a {
    let alloc = ctx.part(ClauseAllocP);
    ctx.part(ClauseDbP)
        .clauses
        .iter()
        .cloned()
        .filter(move |&cref| !alloc.header(cref).deleted())
}

/// Iterate over all and remove some long clauses.
///
/// Takes a closure that returns true for each clause that should be kept and false for each that
/// should be deleted.
pub fn filter_clauses<F>(
    mut ctx: partial!(Context, mut ClauseAllocP, mut ClauseDbP, mut WatchlistsP),
    mut filter: F,
) where
    F: FnMut(&mut ClauseAlloc, ClauseRef) -> bool,
{
    ctx.part_mut(WatchlistsP).disable();

    let (alloc, mut ctx) = ctx.split_part_mut(ClauseAllocP);
    let db = ctx.part_mut(ClauseDbP);

    let count_by_tier = &mut db.count_by_tier;
    let garbage_size = &mut db.garbage_size;

    db.clauses.retain(|&cref| {
        if alloc.header(cref).deleted() {
            false
        } else if filter(alloc, cref) {
            true
        } else {
            let header = alloc.header_mut(cref);

            header.set_deleted(true);

            count_by_tier[header.tier() as usize] -= 1;

            *garbage_size += header.len() + HEADER_LEN;

            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use partial_ref::IntoPartialRefMut;

    use varisat_formula::cnf_formula;

    use crate::context::set_var_count;

    #[test]
    fn set_tiers_and_deletes() {
        let mut ctx = Context::default();

        let mut ctx = ctx.into_partial_ref_mut();

        let clauses = cnf_formula![
            1, 2, 3;
            4, -5, 6;
            -2, 3, -4;
            -3, 5, 2, 7, 5;
        ];

        set_var_count(ctx.borrow(), clauses.var_count());

        let tiers = vec![Tier::Irred, Tier::Core, Tier::Mid, Tier::Local];
        let new_tiers = vec![Tier::Irred, Tier::Local, Tier::Local, Tier::Core];

        let mut crefs = vec![];

        for (clause, &tier) in clauses.iter().zip(tiers.iter()) {
            let mut header = ClauseHeader::new();
            header.set_tier(tier);
            let cref = add_clause(ctx.borrow(), header, clause);
            crefs.push(cref);
        }

        for (&cref, &tier) in crefs.iter().rev().zip(new_tiers.iter().rev()) {
            set_clause_tier(ctx.borrow(), cref, tier);
        }

        // We only check presence, as deletion from these lists is delayed
        assert!(ctx.part(ClauseDbP).by_tier[Tier::Irred as usize].contains(&crefs[0]));
        assert!(ctx.part(ClauseDbP).by_tier[Tier::Core as usize].contains(&crefs[3]));
        assert!(ctx.part(ClauseDbP).by_tier[Tier::Local as usize].contains(&crefs[1]));
        assert!(ctx.part(ClauseDbP).by_tier[Tier::Local as usize].contains(&crefs[2]));

        assert_eq!(ctx.part(ClauseDbP).count_by_tier(Tier::Irred), 1);
        assert_eq!(ctx.part(ClauseDbP).count_by_tier(Tier::Core), 1);
        assert_eq!(ctx.part(ClauseDbP).count_by_tier(Tier::Mid), 0);
        assert_eq!(ctx.part(ClauseDbP).count_by_tier(Tier::Local), 2);

        delete_clause(ctx.borrow(), crefs[0]);
        delete_clause(ctx.borrow(), crefs[2]);

        assert_eq!(ctx.part(ClauseDbP).count_by_tier(Tier::Irred), 0);
        assert_eq!(ctx.part(ClauseDbP).count_by_tier(Tier::Core), 1);
        assert_eq!(ctx.part(ClauseDbP).count_by_tier(Tier::Mid), 0);
        assert_eq!(ctx.part(ClauseDbP).count_by_tier(Tier::Local), 1);
    }
}
