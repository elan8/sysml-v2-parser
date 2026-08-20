# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): RootPackageTest"))
~~~
# SOURCE
~~~sysml
package P1 {
	part def A;
}

package P2 {
	private import P1::*;
	part a : A;
}

private import P2::*;

package P3 {
	part b subsets a;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "root_package_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P1 {
    part def A;
}

package P2 {
    private import P1::*;
    part a : A;
}

private import P2::*;

package P3 {
    part b :> a;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 58) (line 6) (column 17) (len 2)) (segments (segment 0 (token "P1") (name "P1") (separator none) (span (offset 58) (line 6) (column 17) (len 2)))))
    (reference r1 (scope relative) (span (offset 75) (line 7) (column 11) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 75) (line 7) (column 11) (len 1)))))
    (reference r2 (scope relative) (span (offset 96) (line 10) (column 16) (len 2)) (segments (segment 0 (token "P2") (name "P2") (separator none) (span (offset 96) (line 10) (column 16) (len 2)))))
    (reference r3 (scope relative) (span (offset 133) (line 13) (column 17) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 133) (line 13) (column 17) (len 1)))))
  )
  (root (package (name "P1") (body brace (part-def (name "A") (modifiers) (body semicolon)))) (package (name "P2") (body brace (import (target (span (span (offset 58) (line 6) (column 17) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 60) (line 6) (column 19) (len 3))) (separator (span (offset 60) (line 6) (column 19) (len 2))) (marker (span (offset 62) (line 6) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (import (target (span (span (offset 96) (line 10) (column 16) (len 5))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 98) (line 10) (column 18) (len 3))) (separator (span (offset 98) (line 10) (column 18) (len 2))) (marker (span (offset 100) (line 10) (column 20) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "P3") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "b") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r3))) (value none))) (redefines none) (value none) (body semicolon)))))
)
~~~
