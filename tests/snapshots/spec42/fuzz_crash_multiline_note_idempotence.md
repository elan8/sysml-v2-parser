# META
~~~sexpr
(snapshot (type semantic) (description "Unclosed multiline note preserved as-is (non-idempotent for malformed input)"))
~~~
# SOURCE
~~~sysml
package ers {
	//*>> baseTyclassifier A;,	classifier B;

	specializaaaaaaaaaaaaaaaaaaaaaaaaaaA specializes B;
	specialization swbclassifier B :> A;

	Uubclassifier C s cializes A;
	subclassifier C speciaer D disjoint fr_m C differecializes A, B;
		caassifier D disjoint fr_m C differences A, B;
	cla[sifie Conjugation {
er E specializes C intersects A, B;
	classifier F union^ A unions B;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_crash_multiline_note_idempotence.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 58) (line 4) (column 2) (len 51)) (message "unrecognized declaration `specializaaaaaaaaaaaaaaaaaaaaaaaaaaA` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 111) (line 5) (column 2) (len 36)) (message "unrecognized declaration `specialization` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 150) (line 7) (column 2) (len 29)) (message "unrecognized declaration `Uubclassifier` in package body"))
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 181) (line 8) (column 2) (len 64)) (message "the spec-valid KerML classifier declaration production is retained but not structurally implemented"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 248) (line 9) (column 3) (len 46)) (message "unrecognized declaration `caassifier` in package body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 296) (line 10) (column 2) (len 93)) (message "unrecognized declaration `cla` in package body"))
      (diagnostic (code "missing_closing_brace") (severity none) (category parseerror) (span (offset 390) (line 13) (column 2) (len 1)) (message "missing closing '}'"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package ers {
    specializaaaaaaaaaaaaaaaaaaaaaaaaaaA specializes B;
    specialization swbclassifier B :> A;
    Uubclassifier C s cializes A;
    subclassifier C speciaer D disjoint fr_m C differecializes A, B;
    caassifier D disjoint fr_m C differences A, B;
    cla[sifie Conjugation {
er E specializes C intersects A, B;
	classifier F union^ A unions B;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "ers") (body brace (malformed (code "unrecognized_declaration_in_scope") (found "specializaaaaaaaaaaaaaaaaaaaaaaaaaaA specializes B;") (span (offset 58) (line 4) (column 2) (len 51))) (malformed (code "unrecognized_declaration_in_scope") (found "specialization swbclassifier B :> A;") (span (offset 111) (line 5) (column 2) (len 36))) (malformed (code "unrecognized_declaration_in_scope") (found "Uubclassifier C s cializes A;") (span (offset 150) (line 7) (column 2) (len 29))) (classifier-declaration) (malformed (code "unrecognized_declaration_in_scope") (found "caassifier D disjoint fr_m C differences A, B;") (span (offset 248) (line 9) (column 3) (len 46))) (malformed (code "unrecognized_declaration_in_scope") (found "cla[sifie Conjugation {") (span (offset 296) (line 10) (column 2) (len 93))))))
)
~~~
