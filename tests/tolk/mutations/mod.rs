#![allow(non_snake_case)]

use mewt::{types::Mutant, LanguageEngine};
use muton::languages::tolk::engine::TolkLanguageEngine;

use super::common::{sort_by_byte_offset, tolk_target};

pub(super) fn mutants_for_slug(source: &str, slug: &str) -> Vec<Mutant> {
    let fixture = tolk_target(source);
    let engine = TolkLanguageEngine::new();
    engine
        .mutate(fixture.target())
        .into_iter()
        .filter(|m| m.mutation_slug == slug)
        .collect()
}

pub(super) fn first_mutated_source(source: &str, slug: &str) -> Option<String> {
    let fixture = tolk_target(source);
    let target = fixture.target();
    let engine = TolkLanguageEngine::new();
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
#[path = "SAOS.rs"]
mod saos;
#[path = "SOS.rs"]
mod sos;
#[path = "WF.rs"]
mod wf;
