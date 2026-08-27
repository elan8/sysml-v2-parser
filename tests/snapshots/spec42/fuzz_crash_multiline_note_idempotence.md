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
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 58) (line 4) (column 2) (len 331)) (message "unrecognized declaration `specializaaaaaaaaaaaaaaaaaaaaaaaaaaA` in package body"))
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
  (root (package (name "ers") (body brace (malformed (code "unrecognized_declaration_in_scope") (found "specializaaaaaaaaaaaaaaaaaaaaaaaaaaA specializes B;") (span (offset 58) (line 4) (column 2) (len 331))))))
)
~~~
