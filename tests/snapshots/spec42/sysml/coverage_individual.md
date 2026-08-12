# META
~~~sexpr
(snapshot (type semantic) (description "Coverage: individual keyword parsing paths"))
~~~
# SOURCE
~~~sysml
package CoverageIndividual {
	individual def D1;
	individual occurrence def D2;
	individual item def D3;
	individual part def D4;
	individual action def D5;
	individual state def D6;
	individual connection def D7;
	individual calc def D8;
	individual constraint def D9;
	individual requirement def D10;
	individual concern def D11;
	individual case def D12;
	individual analysis def D13;
	individual verification def D14;
	individual view def D15;
	individual viewpoint def D16;
	individual rendering def D17;

	individual p1;
	individual occurrence o1;
	individual item i1;
	individual part p2;
	individual port po1;
	individual action a1;
	individual state s1;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_individual.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 158) (line 7) (column 2) (len 26)) (message "unexpected token in package body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 158) (line 7) (column 2) (len 26)) (message "suppressed 12 cascading recovered diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package CoverageIndividual {
    individual def D1;
    individual occurrence def D2;
    individual item def D3;
    individual part def D4;
    individual action def D5;
    individual state def D6;
    individual connection def D7;
    individual calc def D8;
    individual constraint def D9;
    individual requirement def D10;
    individual concern def D11;
    individual case def D12;
    individual analysis def D13;
    individual verification def D14;
    individual view def D15;
    individual viewpoint def D16;
    individual rendering def D17;
    individual p1;
    individual occurrence o1;
    individual item def i1;
    individual part p2;
    individual port po1;
    individual action a1;
    individual state s1;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "CoverageIndividual") (body (individual-def) (occurrence-def) (item-def) (part-def (name "D4") (body semicolon)) (action-def) (malformed (code "recovered_package_body_element") (found "individual state def D6;") (span (offset 158) (line 7) (column 2) (len 26))) (malformed (code "recovered_package_body_element") (found "individual connection def D7;") (span (offset 184) (line 8) (column 2) (len 31))) (malformed (code "recovered_package_body_element") (found "individual calc def D8;") (span (offset 215) (line 9) (column 2) (len 25))) (malformed (code "recovered_package_body_element") (found "individual constraint def D9;") (span (offset 240) (line 10) (column 2) (len 31))) (malformed (code "recovered_package_body_element") (found "individual requirement def D10;") (span (offset 271) (line 11) (column 2) (len 33))) (malformed (code "recovered_package_body_element") (found "individual concern def D11;") (span (offset 304) (line 12) (column 2) (len 29))) (malformed (code "recovered_package_body_element") (found "individual case def D12;") (span (offset 333) (line 13) (column 2) (len 26))) (analysis-case-def) (malformed (code "recovered_package_body_element") (found "individual verification def D14;") (span (offset 389) (line 15) (column 2) (len 34))) (malformed (code "recovered_package_body_element") (found "individual view def D15;") (span (offset 423) (line 16) (column 2) (len 26))) (malformed (code "recovered_package_body_element") (found "individual viewpoint def D16;") (span (offset 449) (line 17) (column 2) (len 31))) (malformed (code "recovered_package_body_element") (found "individual rendering def D17;") (span (offset 480) (line 18) (column 2) (len 32))) (occurrence (portion none) (declaration "p1") (target none)) (malformed (code "recovered_package_body_element") (found "individual occurrence o1;") (span (offset 528) (line 21) (column 2) (len 27))) (item-def) (part-usage) (malformed (code "recovered_package_body_element") (found "individual port po1;") (span (offset 597) (line 24) (column 2) (len 22))) (action-usage) (state-usage))))
)
~~~
