# META
~~~sexpr
(snapshot (type semantic) (description "interface usage subsetting and redefinition clauses (:>/:>>)"))
~~~
# SOURCE
~~~sysml
package InterfaceUsageSubsettingExample {
    part a;
    part b;
    interface i : I :> baseI connect a to b;
    interface :>> redefinedI connect a to b;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "interface_usage_subsetting.md"
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
  )
  (root (package (name "InterfaceUsageSubsettingExample") (body (part-usage) (part-usage) (interface-usage) (interface-usage))))
)
~~~
