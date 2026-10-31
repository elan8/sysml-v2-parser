# META
~~~sexpr
(snapshot (type semantic) (description "Lexical inner binding shadows an incompatible imported binding"))
~~~
# SOURCE
~~~sysml
package A {
    part def T;
}
package C {
    private import A::*;
    part T;
    part p : T;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "lexical_inner_shadow.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package A {
    part def T;
}

package C {
    private import A::*;
    part T;
    part p : T;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 61) (line 5) (column 20) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 61) (line 5) (column 20) (len 1)))))
    (reference r1 (scope relative) (span (offset 92) (line 7) (column 14) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 92) (line 7) (column 14) (len 1)))))
  )
  (root (package (visibility none) (name "A") (body brace (part-def (name "T") (modifiers) (body semicolon)))) (package (visibility none) (name "C") (body brace (import (visibility private) (target (span (span (offset 61) (line 5) (column 20) (len 4))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 62) (line 5) (column 21) (len 3))) (separator (span (offset 62) (line 5) (column 21) (len 2))) (marker (span (offset 64) (line 5) (column 23) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "T") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))
)
~~~
