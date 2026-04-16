use super::mutants_for_slug;

#[test]
fn er_replaces_statements_with_throw() {
    let source = r#"
() protect() {
    var counter = 0;
}
"#;

    let mutants = mutants_for_slug(source, "ER");
    assert!(
        !mutants.is_empty(),
        "expected ER mutants to replace statements"
    );

    assert!(
        mutants
            .iter()
            .all(|m| m.new_text.contains("throw(65535)")),
        "expected ER mutants to inject `throw(65535);`"
    );
}
