# META
~~~sexpr
(snapshot (type semantic) (description "View-body recovery preserves malformed content and resumes at prefixed attribute starters in definitions and usages (SysML 8.2.2.26)."))
~~~
# SOURCE
~~~sysml
view def Document {
    garbage!!;
    derived attribute total : ScalarValues::Real;
}
view document : Document {
    garbage!!;
    in attribute input : ScalarValues::Real;
    ref attribute shared : ScalarValues::String;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "view_body_attribute_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 24) (line 2) (column 5) (len 15)) (message "unrecognized declaration `garbage` in view definition body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 118) (line 6) (column 5) (len 15)) (message "unrecognized declaration `garbage` in view body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
view def Document {
    garbage!!;
    derived attribute total : ScalarValues::Real;
}

view document : Document {
    garbage!!;
    in attribute input : ScalarValues::Real;
    ref attribute shared : ScalarValues::String;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 65) (line 3) (column 31) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 65) (line 3) (column 31) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 79) (line 3) (column 45) (len 4)))))
    (reference r1 (scope relative) (span (offset 103) (line 5) (column 17) (len 8)) (segments (segment 0 (token "Document") (name "Document") (separator none) (span (offset 103) (line 5) (column 17) (len 8)))))
    (reference r2 (scope relative) (span (offset 154) (line 7) (column 26) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 154) (line 7) (column 26) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 168) (line 7) (column 40) (len 4)))))
    (reference r3 (scope relative) (span (offset 201) (line 8) (column 28) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 201) (line 8) (column 28) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 215) (line 8) (column 42) (len 6)))))
  )
  (root (view-def (name "Document") (short-name none) (modifiers) (specializes none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "garbage!!;") (span (offset 24) (line 2) (column 5) (len 15))) (attribute-usage (declaration-name "total") (direction none) (derived true) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "document") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "garbage!!;") (span (offset 118) (line 6) (column 5) (len 15))) (attribute-usage (declaration-name "input") (direction in) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "shared") (direction none) (derived false) (usage-prefix none) (constant false) (reference true) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))
)
~~~
