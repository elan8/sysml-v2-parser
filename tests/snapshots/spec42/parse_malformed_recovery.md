# META
~~~sexpr
(snapshot (type semantic) (description "Parser recovers from unexpected tokens"))
~~~
# SOURCE
~~~sysml
package Foo {
    + bad stuff;
    part def Bar;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parse_malformed_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 18) (line 2) (column 5) (len 17)) (message "unexpected token in package body"))
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
  (root (package (name "Foo") (body brace (malformed (code "recovered_package_body_element") (found "+ bad stuff;") (span (offset 18) (line 2) (column 5) (len 17))) (part-def (name "Bar") (modifiers) (body semicolon)))))
)
~~~
