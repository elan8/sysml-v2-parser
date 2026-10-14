# META
~~~sexpr
(snapshot (type recovery) (description "Recovery diagnostics classify a leading word by the document name language: feature is not a SysML keyword, so the same unrecognized member is an unrecognized declaration in a SysML document."))
~~~
# SOURCE
~~~sysml
package SysmlKeywordDiagnostic {
    classifier C {
        feature x y z;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "unexpected_keyword_name_language.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 60) (line 3) (column 9) (len 19)) (message "unrecognized declaration `feature` in calc body"))
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
  (name-language sysml)
  (references
  )
  (root (package (name "SysmlKeywordDiagnostic") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "C") (multiplicity none) (specializes none) (conjugates none) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "feature x y z;") (span (offset 60) (line 3) (column 9) (len 19))))))))
)
~~~
