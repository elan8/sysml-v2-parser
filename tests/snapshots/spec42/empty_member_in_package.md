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
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 15) (line 1) (column 16) (len 1)) (message "unexpected token in package body"))
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
  (root (package (name "MyPkg") (body brace (malformed (code "recovered_package_body_element") (found ";") (span (offset 15) (line 1) (column 16) (len 1))))))
)
~~~
