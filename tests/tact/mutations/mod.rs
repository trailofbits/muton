#![allow(non_snake_case)]

use mewt::{types::Mutant, LanguageEngine};
use muton::languages::tact::engine::TactLanguageEngine;

use super::common::{sort_by_byte_offset, tact_target};

pub(super) fn mutants_for_slug(source: &str, slug: &str) -> Vec<Mutant> {
    let fixture = tact_target(source);
    let engine = TactLanguageEngine::new();
    engine
        .mutate(fixture.target())
        .into_iter()
        .filter(|m| m.mutation_slug == slug)
        .collect()
}

pub(super) fn first_mutated_source(source: &str, slug: &str) -> Option<String> {
    let fixture = tact_target(source);
    let target = fixture.target();
    let engine = TactLanguageEngine::new();
    let mut mutants: Vec<_> = engine
        .mutate(target)
        .into_iter()
        .filter(|m| m.mutation_slug == slug)
        .collect();
    sort_by_byte_offset(&mut mutants);
    mutants
        .into_iter()
        .next()
        .and_then(|m| target.mutate(&m).ok())
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
#[path = "ER.rs"]
mod er;
#[path = "IF.rs"]
mod r#if;
#[path = "IT.rs"]
mod it;
#[path = "LC.rs"]
mod lc;
#[path = "LOS.rs"]
mod los;
#[path = "RZ.rs"]
mod rz;
#[path = "SAOS.rs"]
mod saos;
#[path = "SOS.rs"]
mod sos;
#[path = "TF.rs"]
mod tf;
#[path = "TT.rs"]
mod tt;
#[path = "UF.rs"]
mod uf;
#[path = "WF.rs"]
mod wf;
