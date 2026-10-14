# META
~~~sexpr
(snapshot (type semantic) (description "Ambiguous imported type preserves ordered candidate locations"))
~~~
# SOURCE
~~~sysml
package A { part def Thing; }
package B { part def Thing; }
package Use {
    private import A::*;
    private import B::*;
    part usage : Thing;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ambiguous_import_candidates.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package A {
    part def Thing;
}

package B {
    part def Thing;
}

package Use {
    private import A::*;
    private import B::*;
    part usage : Thing;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 93) (line 4) (column 20) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 93) (line 4) (column 20) (len 1)))))
    (reference r1 (scope relative) (span (offset 118) (line 5) (column 20) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 118) (line 5) (column 20) (len 1)))))
    (reference r2 (scope relative) (span (offset 141) (line 6) (column 18) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 141) (line 6) (column 18) (len 5)))))
  )
  (root (package (name "A") (body brace (part-def (name "Thing") (modifiers) (body semicolon)))) (package (name "B") (body brace (part-def (name "Thing") (modifiers) (body semicolon)))) (package (name "Use") (body brace (import (target (span (span (offset 93) (line 4) (column 20) (len 4))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 94) (line 4) (column 21) (len 3))) (separator (span (offset 94) (line 4) (column 21) (len 2))) (marker (span (offset 96) (line 4) (column 23) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 118) (line 5) (column 20) (len 4))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 119) (line 5) (column 21) (len 3))) (separator (span (offset 119) (line 5) (column 21) (len 2))) (marker (span (offset 121) (line 5) (column 23) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "usage") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))
)
~~~
