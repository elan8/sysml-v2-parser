use sysml_v2_parser::parse_for_editor;
#[test]
fn scan() {
    let root = std::path::Path::new("sysml-v2-release/sysml.library");
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for e in std::fs::read_dir(&dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().map(|x| x == "sysml").unwrap_or(false) {
                files.push(p);
            }
        }
    }
    files.sort();
    let (mut total, mut with) = (0usize, 0usize);
    for f in &files {
        let r = parse_for_editor(&std::fs::read_to_string(f).unwrap());
        if r.errors.is_empty() {
            continue;
        }
        with += 1;
        total += r.errors.len();
        println!(
            "FILE {:3}  {}",
            r.errors.len(),
            f.strip_prefix(root).unwrap().display()
        );
        for e in &r.errors {
            println!(
                "      {} | {}",
                e.code.clone().unwrap_or_default(),
                e.found
                    .clone()
                    .unwrap_or_default()
                    .chars()
                    .take(46)
                    .collect::<String>()
                    .replace('\n', " ")
            );
        }
    }
    println!(
        "LIBRARY files={} with_diagnostics={} total={}",
        files.len(),
        with,
        total
    );
}
