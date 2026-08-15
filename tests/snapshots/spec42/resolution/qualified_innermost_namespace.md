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
  )
  (root (package (name "A") (body brace (part-def (name "T") (body semicolon)))) (package (name "C") (body brace (package (name "A") (body brace (part-def (name "T") (body semicolon)))) (part-usage))))
)
~~~
