# META
~~~sexpr
(snapshot (type malformed) (description "A malformed TypeBody package declaration is recovered as one member without consuming the later alias sibling. The private prefix is retained in the recovered source span rather than reinterpreted as a feature expression."))
~~~
# SOURCE
~~~sysml
package TypeBodyPackageRecovery {
    class Outer {
        private package { }
        alias afterBrokenPackage for Outer;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml_type_body_package_member_recovery.md"
    (diagnostics
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 60) (line 3) (column 9) (len 28)) (message "unexpected keyword `private` in calc body"))
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
    (reference r0 (scope relative) (span (offset 117) (line 4) (column 38) (len 5)) (segments (segment 0 (token "Outer") (name "Outer") (separator none) (span (offset 117) (line 4) (column 38) (len 5)))))
  )
  (root (package (name "TypeBodyPackageRecovery") (body brace (kerml-classifier (keyword class) (abstract false) (name "Outer") (specializes none) (body brace (malformed (code "unexpected_keyword_in_scope") (found "private package { }") (span (offset 60) (line 3) (column 9) (len 28))) (alias (name "afterBrokenPackage") (target (ref r0)) (body semicolon)))))))
)
~~~
