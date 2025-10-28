use std::fs;
use std::path::PathBuf;

use muton::mutations::get_mutations_for_language;
use muton::types::{Hash, Language as MutationLanguage, Target};
use tempfile::tempdir;

fn write_temp_file(name: &str, contents: &str) -> (tempfile::TempDir, PathBuf) {
    let dir = tempdir().expect("failed to create temp dir");
    let path = dir.path().join(name);
    fs::write(&path, contents).expect("failed to write file");
    (dir, path)
}

fn make_target(path: PathBuf, contents: &str, lang: MutationLanguage) -> Target {
    Target {
        id: 1,
        path,
        file_hash: Hash::digest(contents.to_string()),
        text: contents.to_string(),
        language: lang,
    }
}

#[test]
fn cross_language_shared_slugs_presence_and_counts() {
    // FunC sample with if and a call with 2 args
    let func_src = r#"()
main() {
    var x = 1;
    if (x > 0) {
        return x;
    }
    foo(1, 2);
}
"#;
    let (_func_tmp, func_path) = write_temp_file("test.fc", func_src);
    let func_target = make_target(func_path, func_src, MutationLanguage::FunC);

    // Tact sample with if and a call with 2 args
    let tact_src = r#"contract C {
    fun f() {
        let x: Int = 1;
        if (x > 0) {
            return;
        }
        doSomething(1, 2);
    }
}
"#;
    let (_tact_tmp, tact_path) = write_temp_file("test.tact", tact_src);
    let tact_target = make_target(tact_path, tact_src, MutationLanguage::Tact);

    let func_engine = get_mutations_for_language(&MutationLanguage::FunC);
    let tact_engine = get_mutations_for_language(&MutationLanguage::Tact);

    let func_mutants = func_engine.apply_all_mutations(&func_target);
    let tact_mutants = tact_engine.apply_all_mutations(&tact_target);

    fn count(mutants: &[muton::types::Mutant], slug: &str) -> usize {
        mutants.iter().filter(|m| m.mutation_slug == slug).count()
    }

    let func_er = count(&func_mutants, "ER");
    let func_cr = count(&func_mutants, "CR");
    let func_as = count(&func_mutants, "AS");

    let tact_er = count(&tact_mutants, "ER");
    let tact_cr = count(&tact_mutants, "CR");
    let tact_as = count(&tact_mutants, "AS");

    println!("func ER/CR/AS: {func_er}/{func_cr}/{func_as}");
    println!("tact ER/CR/AS: {tact_er}/{tact_cr}/{tact_as}");

    assert!(
        func_er > 0 && tact_er > 0,
        "ER should be present in both languages"
    );
    assert!(
        func_cr > 0 && tact_cr > 0,
        "CR should be present in both languages"
    );
    // Only assert AS exists where implemented; require presence in at least one language.
    assert!(
        func_as > 0 || tact_as > 0,
        "AS should be present in at least one language"
    );
}

#[test]
fn cross_language_mutations_ignore_comments() {
    // FunC sample with commented-out code and real code
    let func_src = r#"()
main() {
    ;; if (true) { throw(1); }
    {- let y = 10; -}
    var x = 1;
    if (x > 0) { return x; }
}
"#;
    // NOTE: Keep this list in sync with func_src above.
    // Lines are 0-based and refer to fully-commented lines only.
    let func_commented_lines: &[usize] = &[2, 3];
    let (_func_tmp, func_path) = write_temp_file("comments.fc", func_src);
    let func_target = make_target(func_path, func_src, MutationLanguage::FunC);

    // Tact sample with commented-out code and real code
    let tact_src = r#"// if (true) { require(false); }
// let x: Int = 1 + 2;
// if (1 < 2) { let y: Int = 3; }
// this.callMe(10, 20);
// while (true) { break; }
contract C {
    init() { }
    receive("hello") { }
    fun f(a: Int, b: Int): Int {
        return a + b;
    }
}
"#;
    // NOTE: Keep this list in sync with tact_src above.
    // Lines are 0-based and refer to fully-commented lines only.
    let tact_commented_lines: &[usize] = &[0, 1, 2, 3, 4];
    let (_tact_tmp, tact_path) = write_temp_file("comments.tact", tact_src);
    let tact_target = make_target(tact_path, tact_src, MutationLanguage::Tact);

    let func_engine = get_mutations_for_language(&MutationLanguage::FunC);
    let tact_engine = get_mutations_for_language(&MutationLanguage::Tact);

    let func_mutants = func_engine.apply_all_mutations(&func_target);
    let tact_mutants = tact_engine.apply_all_mutations(&tact_target);

    // FunC: ensure no mutants are on fully-commented lines
    for m in &func_mutants {
        let line = m.line_offset as usize;
        assert!(
            !func_commented_lines.contains(&line),
            "FunC mutated on commented line: slug={} line={}",
            m.mutation_slug,
            line
        );
    }

    // Tact: ensure no mutants are on fully-commented lines
    for m in &tact_mutants {
        let line = m.line_offset as usize;
        assert!(
            !tact_commented_lines.contains(&line),
            "Tact mutated on commented line: slug={} line={}",
            m.mutation_slug,
            line
        );
    }
}
