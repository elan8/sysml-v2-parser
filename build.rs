use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn main() {
    const TARGET: &str = "docs/conformance-target";
    println!("cargo:rerun-if-changed={TARGET}");
    let fields = parse_target(&fs::read_to_string(TARGET).expect("read conformance target"));
    for key in ["release_tag", "release_repo", "grammar_content_hash"] {
        let value = fields
            .get(key)
            .unwrap_or_else(|| panic!("{TARGET} missing {key}"));
        println!(
            "cargo:rustc-env=SYSML_PARSER_{}={value}",
            key.to_ascii_uppercase()
        );
    }

    let bnf_root = Path::new("sysml-v2-release/bnf");
    let sysml = bnf_root.join("SysML-textual-bnf.kebnf");
    let kerml = bnf_root.join("KerML-textual-bnf.kebnf");
    println!("cargo:rerun-if-changed={}", sysml.display());
    println!("cargo:rerun-if-changed={}", kerml.display());
    if sysml.exists() && kerml.exists() {
        let actual = grammar_content_hash(&[
            (
                "SysML-textual-bnf.kebnf",
                fs::read(&sysml).expect("read SysML BNF"),
            ),
            (
                "KerML-textual-bnf.kebnf",
                fs::read(&kerml).expect("read KerML BNF"),
            ),
        ]);
        let expected = fields
            .get("grammar_content_hash")
            .expect("grammar hash field");
        assert_eq!(
            &actual, expected,
            "pinned grammar contents changed; update docs/conformance-target deliberately"
        );
    }
}

fn parse_target(source: &str) -> BTreeMap<String, String> {
    source
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (key, value) = line
                .split_once('=')
                .unwrap_or_else(|| panic!("invalid conformance target line: {line}"));
            Some((key.trim().to_owned(), value.trim().to_owned()))
        })
        .collect()
}

fn grammar_content_hash(files: &[(&str, Vec<u8>)]) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for (name, bytes) in files {
        for byte in name
            .as_bytes()
            .iter()
            .copied()
            .chain(std::iter::once(0))
            .chain(bytes.iter().copied())
            .chain(std::iter::once(0))
        {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    format!("fnv1a64:{hash:016x}")
}
