# META
~~~sexpr
(snapshot (type recovery) (description "Recovery diagnostics classify a leading word by the document name language, as parsing does: feature is a KerML keyword, so an unrecognized feature member is an unexpected keyword in a KerML document.") (name-language kerml))
~~~
# SOURCE
~~~sysml
package KermlKeywordDiagnostic {
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
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 60) (line 3) (column 9) (len 19)) (message "unexpected keyword `feature` in calc body"))
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
  (name-language kerml)
  (references
  )
  (root (package (name "KermlKeywordDiagnostic") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "C") (multiplicity none) (specializes none) (conjugates none) (body brace (malformed (code "unexpected_keyword_in_scope") (found "feature x y z;") (span (offset 60) (line 3) (column 9) (len 19))))))))
)
~~~
