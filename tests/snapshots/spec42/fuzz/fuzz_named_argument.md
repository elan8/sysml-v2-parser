# META
~~~sexpr
(snapshot (type semantic) (description "Fuzz: named arguments in invocations use = not => for idempotent reparse"))
~~~
# SOURCE
~~~sysml
package P {
    calc def F { in p : A; }
    attribute f = F(q = 1, p = a);
    attribute b = new A(y = a, x = "");
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_named_argument.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    calc def F {
        in p : A;
    }
    attribute f = F(q = 1, p = a);
    attribute b = new A(y = a, x = "");
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "P") (body brace (calc-def (name "F") (modifiers) (body brace (in-out-declaration))) (attribute-usage) (attribute-usage))))
)
~~~
