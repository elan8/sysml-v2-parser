# META
~~~sexpr
(snapshot (type semantic) (description "SysML Feature Typing Kind Mismatch (SC-4)"))
~~~
# SOURCE
~~~sysml
attribute def Foo {}
part p : Foo;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "feature_typing_mismatch.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
attribute def Foo {
}

part p : Foo;
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 30) (line 2) (column 10) (len 3)) (segments (segment 0 (token "Foo") (name "Foo") (separator none) (span (offset 30) (line 2) (column 10) (len 3)))))
  )
  (root (attribute-def (declaration-name "Foo") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))
)
~~~
