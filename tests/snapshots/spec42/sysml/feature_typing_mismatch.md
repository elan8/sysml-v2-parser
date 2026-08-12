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
  )
  (root (attribute-def) (part-usage))
)
~~~
