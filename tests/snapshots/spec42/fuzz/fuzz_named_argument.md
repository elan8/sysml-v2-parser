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
    (reference r0 (scope relative) (span (offset 36) (line 2) (column 25) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 36) (line 2) (column 25) (len 1)))))
  )
  (root (package (name "P") (body brace (calc-def (name "F") (modifiers) (body brace (in-out (direction in) (reference false) (declaration "p") (subsets none) (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 29) (line 2) (column 18) (len 9))))) (attribute-usage) (attribute-usage))))
)
~~~
