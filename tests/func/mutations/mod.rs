#![allow(non_snake_case)]

use muton::languages::func::engine::FuncLanguageEngine;
use mewt::{LanguageEngine, types::Mutant};

use super::common::func_target;

pub(super) fn mutants_for_slug(source: &str, slug: &str) -> Vec<Mutant> {
    let fixture = func_target(source);
    let engine = FuncLanguageEngine::new();
    engine
        .mutate(fixture.target())
        .into_iter()
        .filter(|m| m.mutation_slug == slug)
        .collect()
}

#[path = "AAOS.rs"]
mod aaos;
#[path = "AOS.rs"]
mod aos;
#[path = "AS.rs"]
mod r#as;
#[path = "BAOS.rs"]
mod baos;
#[path = "BL.rs"]
mod bl;
#[path = "BOS.rs"]
mod bos;
#[path = "COS.rs"]
mod cos;
#[path = "CR.rs"]
mod cr;
#[path = "DAOS.rs"]
mod daos;
#[path = "DOS.rs"]
mod dos;
#[path = "ER.rs"]
mod er;
#[path = "IF.rs"]
mod r#if;
#[path = "INF.rs"]
mod inf;
#[path = "INT.rs"]
mod int;
#[path = "IT.rs"]
mod it;
#[path = "LC.rs"]
mod lc;
#[path = "LOS.rs"]
mod los;
#[path = "MAOS.rs"]
mod maos;
#[path = "MOS.rs"]
mod mos;
#[path = "RZ.rs"]
mod rz;
#[path = "SAOS.rs"]
mod saos;
#[path = "SC.rs"]
mod sc;
#[path = "SI.rs"]
mod si;
#[path = "SOS.rs"]
mod sos;
#[path = "SU.rs"]
mod su;
#[path = "UF.rs"]
mod uf;
#[path = "WF.rs"]
mod wf;
