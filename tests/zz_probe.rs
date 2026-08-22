use sysml_v2_parser as p;
fn probe(gap: &str, src: &str) {
    let r = p::parse_for_editor(src);
    let codes: Vec<String> = r.errors.iter().filter_map(|e| e.code.clone()).collect();
    let mark = if codes.is_empty() { "OK  " } else { "FAIL" };
    println!("{mark} [{gap}] {src}\n        {codes:?}");
}
#[test]
fn t() {
    // --- 81: REGRESSION ---
    probe("81", "calc def C { in expr p : Boolean; }");
    probe("81", "calc def C { in bool redefines a; }");
    probe("81", "calc def C { in feature p : Boolean; }");
    probe("81", "constraint def C { in expr p : Boolean; }");
    probe("81-ok", "behavior B { in expr p : Boolean; }");
    probe("81-ok", "calc def C { in p : Boolean; }");
    // --- already fixed on this branch? ---
    probe("64", "classifier One conjugates A;");
    probe(
        "65",
        "package P { state def Machine parallel { state a; state b; } }",
    );
    probe("80", "package P { state S parallel { state child; } }");
    probe(
        "70",
        "package P { part x { @M { attribute named : Boolean = true; } } }",
    );
    // --- 62 / 61 ---
    probe(
        "62",
        "behavior M { feature s : T; feature t : T; flow of Thing of Thing from s to t; }",
    );
    probe("61", "classifier C { message m of T; }");
    // --- 66 ---
    probe("66", "feature two crosses a crosses b;");
    // --- 69 ---
    probe("69", "package P { binding b { end e1 : A; end e2 : B; } }");
    // --- 73 / 74 / 75 ---
    probe("73", "use case def U { include use case v; }");
    probe(
        "74",
        "package P { requirement def R { require constraint c : C; } }",
    );
    probe("75", "package P { port def PD { part x : T; } }");
    // --- 72 / 76 / 77 / 78 / 79 ---
    probe("72", "package P { action def G { perform L::doIt; } }");
    probe("76", "package P { action def Q { accept when true; } }");
    probe("76", "package P { action def Q { accept at now; } }");
    probe(
        "76",
        "package P { action def Q { if true then a1 else a2; } }",
    );
    probe("78", "abstract variation part def Good;");
    probe("78", "variation abstract part def G;");
    probe("79", "package P { expose A::*; }");
    probe(
        "79",
        "package P { part def H { verify requirement r : Q; } }",
    );
    probe("79", "package P { part def H { render asTree; } }");
    // --- 52: var ---
    probe(
        "52",
        "package P { occurrence def Happening { var attribute tracked : Reading; } }",
    );
    probe("75b", "package P { port def PD { composite part x : T; } }");
    probe("75c", "package P { port p { part x : T; } }");
    probe("if-a", "package P { action def Q { if true then a1; } }");
    probe(
        "if-b",
        "package P { action def Q { if true { a1; } else { a2; } } }",
    );
    probe(
        "if-c",
        "package P { action def Q { if true then a1; else a2; } }",
    );
    probe(
        "if-d",
        "package P { action def Q { if true then a1 else a2; } }",
    );
    probe(
        "if-e",
        "package P { action def Q { if true then action a1; else action a2; } }",
    );
    probe(
        "if-f",
        "package P { action def Q { if true { action a1; } else { action a2; } } }",
    );
    probe(
        "if-g",
        "package P { action def Q { if true { action a1; } else if false { action a2; } } }",
    );
}
