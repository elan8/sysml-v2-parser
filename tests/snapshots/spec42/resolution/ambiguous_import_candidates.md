# META
~~~sexpr
(snapshot (type semantic) (description "Ambiguous imported type preserves ordered candidate locations"))
~~~
# SOURCE
~~~sysml
package A { part def Thing; }
package B { part def Thing; }
package Use {
    import A::*;
    import B::*;
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
    import A::*;
    import B::*;
    part usage : Thing;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 85) (line 4) (column 12) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 85) (line 4) (column 12) (len 1)))))
    (reference r1 (scope relative) (span (offset 102) (line 5) (column 12) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 102) (line 5) (column 12) (len 1)))))
    (reference r2 (scope relative) (span (offset 125) (line 6) (column 18) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 125) (line 6) (column 18) (len 5)))))
  )
  (root (package (name "A") (body brace (part-def (name "Thing") (body semicolon)))) (package (name "B") (body brace (part-def (name "Thing") (body semicolon)))) (package (name "Use") (body brace (import (target (span (span (offset 85) (line 4) (column 12) (len 4))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 86) (line 4) (column 13) (len 3))) (separator (span (offset 86) (line 4) (column 13) (len 2))) (marker (span (offset 88) (line 4) (column 15) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 102) (line 5) (column 12) (len 4))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 103) (line 5) (column 13) (len 3))) (separator (span (offset 103) (line 5) (column 13) (len 2))) (marker (span (offset 105) (line 5) (column 15) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "usage") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))))
)
~~~
