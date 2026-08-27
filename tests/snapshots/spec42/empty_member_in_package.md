# META
~~~sexpr
(snapshot (type semantic) (description "Empty member (bare semicolon) inside package body"))
~~~
# SOURCE
~~~sysml
package MyPkg {;}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "empty_member_in_package.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package MyPkg {
    ;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "MyPkg") (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))
)
~~~
