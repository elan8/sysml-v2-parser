# META
~~~sexpr
(snapshot (type semantic) (description "Empty member (bare semicolon) at file level after package"))
~~~
# SOURCE
~~~sysml
package MyPkg { }; in newX : Real;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "empty_member_after_package.md"
    (diagnostics
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 17) (line 1) (column 18) (len 17)) (message "expected a specific keyword or punctuation token"))
      (diagnostic (code "expected_keyword") (severity error) (category parseerror) (span (offset 19) (line 1) (column 20) (len 15)) (message "expected a specific keyword or punctuation token"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package MyPkg {
}

;

in newX : Real;
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "MyPkg") (body )) (malformed (code "expected_keyword") (found "; in newX : Real;") (span (offset 17) (line 1) (column 18) (len 1))) (malformed (code "expected_keyword") (found "in newX : Real;") (span (offset 19) (line 1) (column 20) (len 15))))
)
~~~
