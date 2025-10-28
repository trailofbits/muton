use crate::types::{Mutation, MutationSeverity};

pub const TACT_MUTATIONS: &[Mutation] = &[
    Mutation {
        slug: "RZ",
        description: "Repeat Zero: Hardcode repeat count/expression to 0",
        severity: MutationSeverity::Low,
    },
    Mutation {
        slug: "UF",
        description: "Until False: Hardcode do-until condition to false",
        severity: MutationSeverity::Low,
    },
    Mutation {
        slug: "TT",
        description: "Ternary True: Hardcode ternary condition to true",
        severity: MutationSeverity::Low,
    },
    Mutation {
        slug: "TF",
        description: "Ternary False: Hardcode ternary condition to false",
        severity: MutationSeverity::Low,
    },
];
