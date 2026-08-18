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
  (root (package (name "InterfaceUsageSubsettingExample") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "a") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "b") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (interface-usage) (interface-usage))))
)
~~~
