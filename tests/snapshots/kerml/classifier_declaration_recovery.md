# META
~~~sexpr
(snapshot (type recovery) (description "A malformed member inside a KerML metaclass body recovers as an explicit calc-body error node while the following feature member still parses."))
~~~
# SOURCE
~~~sysml
standard library package Broken {
    metaclass M specializes Element {
        in : ;
        var feature ok : Element[1];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "classifier_declaration_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_calc_body_element") (severity error) (category parseerror) (span (offset 80) (line 3) (column 9) (len 15)) (message "unexpected token in calc body"))
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
    (reference r0 (scope relative) (span (offset 62) (line 2) (column 29) (len 7)) (segments (segment 0 (token "Element") (name "Element") (separator none) (span (offset 62) (line 2) (column 29) (len 7)))))
  )
  (root (library-package (name "Broken") (standard true) (body brace (kerml-classifier (keyword metaclass) (abstract false) (name "M") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0))))))))
)
~~~
