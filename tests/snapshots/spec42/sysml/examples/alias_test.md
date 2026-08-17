# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): AliasTest"))
~~~
# SOURCE
~~~sysml
package AliasTest {
	private import ISQSpaceTime::breadth; // import of an alias
	attribute b :> breadth;
	
    part def P1 {
        port porig1;
        alias po1 for porig1;
    }

    part p1 : P1 {
        port po1 :>> po1;
    }

    part p2 : P1 {
        port pdest;
        alias pd1 for pdest;
    }


    connect p1.po1 to p2.pdest;
	connect p1.po1 to p2.pd1;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "alias_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AliasTest {
    private import ISQSpaceTime::breadth;
    attribute def b :> breadth;
    part def P1 {
        port porig1;
        alias po1 for porig1;
    }
    part p1 : P1 {
        port po1 :>> po1;
    }
    part p2 : P1 {
        port pdest;
        alias pd1 for pdest;
    }
    connect p1.po1 to p2.pdest;
    connect p1.po1 to p2.pd1;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 36) (line 2) (column 17) (len 21)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 36) (line 2) (column 17) (len 12))) (segment 1 (token "breadth") (name "breadth") (separator colon-colon) (span (offset 50) (line 2) (column 31) (len 7)))))
    (reference r1 (scope relative) (span (offset 169) (line 7) (column 23) (len 6)) (segments (segment 0 (token "porig1") (name "porig1") (separator none) (span (offset 169) (line 7) (column 23) (len 6)))))
    (reference r2 (scope relative) (span (offset 198) (line 10) (column 15) (len 2)) (segments (segment 0 (token "P1") (name "P1") (separator none) (span (offset 198) (line 10) (column 15) (len 2)))))
    (reference r3 (scope relative) (span (offset 250) (line 14) (column 15) (len 2)) (segments (segment 0 (token "P1") (name "P1") (separator none) (span (offset 250) (line 14) (column 15) (len 2)))))
    (reference r4 (scope relative) (span (offset 324) (line 20) (column 13) (len 6)) (segments (segment 0 (token "p1") (name "p1") (separator none) (span (offset 324) (line 20) (column 13) (len 2))) (segment 1 (token "po1") (name "po1") (separator dot) (span (offset 327) (line 20) (column 16) (len 3)))))
    (reference r5 (scope relative) (span (offset 334) (line 20) (column 23) (len 8)) (segments (segment 0 (token "p2") (name "p2") (separator none) (span (offset 334) (line 20) (column 23) (len 2))) (segment 1 (token "pdest") (name "pdest") (separator dot) (span (offset 337) (line 20) (column 26) (len 5)))))
    (reference r6 (scope relative) (span (offset 353) (line 21) (column 10) (len 6)) (segments (segment 0 (token "p1") (name "p1") (separator none) (span (offset 353) (line 21) (column 10) (len 2))) (segment 1 (token "po1") (name "po1") (separator dot) (span (offset 356) (line 21) (column 13) (len 3)))))
    (reference r7 (scope relative) (span (offset 363) (line 21) (column 20) (len 6)) (segments (segment 0 (token "p2") (name "p2") (separator none) (span (offset 363) (line 21) (column 20) (len 2))) (segment 1 (token "pd1") (name "pd1") (separator dot) (span (offset 366) (line 21) (column 23) (len 3)))))
  )
  (root (package (name "AliasTest") (body brace (import (target (span (span (offset 36) (line 2) (column 17) (len 21))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (attribute-def) (part-def (name "P1") (body brace (port-usage (declaration-name "porig1") (direction none) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (alias (name "po1") (target (ref r1)) (body semicolon)))) (part-usage (declaration-name "p1") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (body brace (port-usage))) (part-usage (declaration-name "p2") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (body brace (port-usage) (alias-def))) (connect (from (expression (span (offset 324) (line 20) (column 13) (len 6)) (ref r4))) (to (expression (span (offset 334) (line 20) (column 23) (len 8)) (ref r5))) (body semicolon) (subsets none) (redefines none)) (connect (from (expression (span (offset 353) (line 21) (column 10) (len 6)) (ref r6))) (to (expression (span (offset 363) (line 21) (column 20) (len 6)) (ref r7))) (body semicolon) (subsets none) (redefines none)))))
)
~~~
