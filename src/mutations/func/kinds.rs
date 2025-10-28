use crate::types::{Mutation, MutationSeverity};

pub const FUNC_MUTATIONS: &[Mutation] = &[
    Mutation {
        slug: "INF",
        description: "If Not False: Hardcode an ifnot expression to false",
        severity: MutationSeverity::Low,
    },
    Mutation {
        slug: "INT",
        description: "If Not True: Hardcode an ifnot expression to true",
        severity: MutationSeverity::Low,
    },
    Mutation {
        slug: "RZ",
        description: "Repeat Zero: Hardcode a repeat expression to repeat 0 times",
        severity: MutationSeverity::Low,
    },
    Mutation {
        slug: "UF",
        description: "Until False: Hardcode an until expression to false",
        severity: MutationSeverity::Low,
    },
    Mutation {
        slug: "SU",
        description: "Store Uint: replace stored uint with zero",
        severity: MutationSeverity::Low,
    },
    Mutation {
        slug: "SI",
        description: "Store Int: replace stored int with zero",
        severity: MutationSeverity::Low,
    },
    Mutation {
        slug: "SC",
        description: "Store Coins: replace stored coins with zero",
        severity: MutationSeverity::Low,
    },
    // FunC-specific operator shuffles (not shared)
    Mutation {
        slug: "DOS",
        description: "Division Operator Shuffle: Replace division operators (/, ~/, ^/)",
        severity: MutationSeverity::Low,
    },
    Mutation {
        slug: "DAOS",
        description: "Division Assignment Operator Shuffle: Replace division assignment operators (/=, ~/=, ^/=)",
        severity: MutationSeverity::Low,
    },
    Mutation {
        slug: "MOS",
        description: "Modulo Operator Shuffle: Replace modulo operators (%, ~%, ^%)",
        severity: MutationSeverity::Low,
    },
    Mutation {
        slug: "MAOS",
        description: "Modulo Assignment Operator Shuffle: Replace modulo assignment operators (%=, ~%=, ^%=)",
        severity: MutationSeverity::Low,
    },
];
