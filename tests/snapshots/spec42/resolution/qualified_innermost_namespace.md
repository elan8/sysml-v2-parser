# META
~~~sexpr
(snapshot (type semantic) (description "Qualified segments resolve from the innermost namespace"))
~~~
# SOURCE
~~~sysml
package A {
    part def T;
}
package C {
    package A {
        part def T;
    }
    part p : A::T;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "qualified_innermost_namespace.md"
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
    package A {
        part def T;
    }
    part p : A::T;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 97) (line 8) (column 14) (len 4)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 97) (line 8) (column 14) (len 1))) (segment 1 (token "T") (name "T") (separator colon-colon) (span (offset 100) (line 8) (column 17) (len 1)))))
  )
  (root (package (name "A") (body brace (part-def (name "T") (body semicolon)))) (package (name "C") (body brace (package (name "A") (body brace (part-def (name "T") (body semicolon)))) (part-usage (declaration-name "p") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)))))
)
~~~
