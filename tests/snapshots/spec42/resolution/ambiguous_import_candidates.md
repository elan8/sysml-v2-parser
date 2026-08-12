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
  )
  (root (package (name "A") (body (part-def (name "Thing") (body semicolon)))) (package (name "B") (body (part-def (name "Thing") (body semicolon)))) (package (name "Use") (body (import (target (span (span (offset 85) (line 4) (column 12) (len 4))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 86) (line 4) (column 13) (len 3))) (separator (span (offset 86) (line 4) (column 13) (len 2))) (marker (span (offset 88) (line 4) (column 15) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 102) (line 5) (column 12) (len 4))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 103) (line 5) (column 13) (len 3))) (separator (span (offset 103) (line 5) (column 13) (len 2))) (marker (span (offset 105) (line 5) (column 15) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage))))
)
~~~
