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
    import A::*;
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
    import A::*;
    part T;
    part p : T;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 53) (line 5) (column 12) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 53) (line 5) (column 12) (len 1)))))
  )
  (root (package (name "A") (body brace (part-def (name "T") (body semicolon)))) (package (name "C") (body brace (import (target (span (span (offset 53) (line 5) (column 12) (len 4))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 54) (line 5) (column 13) (len 3))) (separator (span (offset 54) (line 5) (column 13) (len 2))) (marker (span (offset 56) (line 5) (column 15) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage) (part-usage))))
)
~~~
