
//! static_probe! macro for injecting usdt probes into rust code
//!

//use rustc_data_structures::indexed_vec::{Idx, IndexVec};
use rustc::ty::TyCtxt;
use rustc::mir::*;
//use rustc::mir::visit::{MutVisitor, Visitor, PlaceContext};
//use rustc::session::config::DebugInfo;
use std::borrow::Cow;
use transform::{MirPass, MirSource};

pub struct StaticProbePlugin;

impl StaticProbePlugin {
}


impl MirPass for StaticProbePlugin {
    fn name<'a>(&'a self) -> Cow<'a, str> {
        Cow::Borrowed("StaticProbePlugin")
    }

    fn run_pass<'a, 'tcx>(&self,
                          _tcx: TyCtxt<'a, 'tcx, 'tcx>,
                          _src: MirSource,
                          mir: &mut Mir<'tcx>) {
        debug!("StaticProbePlugin - simplifying {:?}", mir);
    }
}
