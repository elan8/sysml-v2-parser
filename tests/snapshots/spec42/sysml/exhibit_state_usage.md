# META
~~~sexpr
(snapshot (type semantic) (description "SysML ExhibitStateUsage in a package body: bare reference form and named 'exhibit state' form"))
~~~
# SOURCE
~~~sysml
package ExhibitStateUsageExample {
    exhibit s1;
    exhibit state s2 : StateDef;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "exhibit_state_usage.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 47) (line 2) (column 13) (len 2)) (segments (segment 0 (token "s1") (name "s1") (separator none) (span (offset 47) (line 2) (column 13) (len 2)))))
  )
  (root (package (name "ExhibitStateUsageExample") (body brace (exhibit (declaration none) (state (ref r0))) (exhibit (declaration "s2") (state none)))))
)
~~~
