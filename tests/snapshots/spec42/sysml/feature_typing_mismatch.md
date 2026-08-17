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
  (root (attribute-def) (part-usage (declaration-name "p") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)))
)
~~~
