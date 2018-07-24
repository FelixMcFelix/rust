// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Finds items that are externally reachable, to determine which items
// need to have their metadata (and possibly their AST) serialized.
// All items that can be referred to through an exported name are
// reachable, and when a reachable thing is inline or generic, it
// makes all other generics or inline functions that it references
// reachable as well.

use hir::def_id::CrateNum;
use rustc_data_structures::sync::Lrc;
use ty::TyCtxt;
use ty::query::Providers;
use util::nodemap::NodeSet;

use hir::def_id::LOCAL_CRATE;

// We introduce a new-type here, so we can have a specialized HashStable
// implementation for it.
#[derive(Clone)]
pub struct ReachableSet(pub Lrc<NodeSet>);


fn reachable_set<'a, 'tcx>(tcx: TyCtxt<'a, 'tcx, 'tcx>, crate_num: CrateNum) -> ReachableSet {
    debug_assert!(crate_num == LOCAL_CRATE);
    let access_levels = &tcx.privacy_access_levels(LOCAL_CRATE);

    // Reachability analysis now moved to privacy_access_levels
    // Convert its result.
    let mut reachable_symbols = NodeSet();

    for (id, _) in &access_levels.map {
        reachable_symbols.insert(*id);
    }

    // Return the set of reachable symbols.
    ReachableSet(Lrc::new(reachable_symbols))
}

pub fn provide(providers: &mut Providers) {
    *providers = Providers {
        reachable_set,
        ..*providers
    };
}
