# META
~~~sexpr
(snapshot (type recovery) (description "A malformed member inside a KerML function body becomes an explicit calc-body recovery node while the following parameter and return still parse."))
~~~
# SOURCE
~~~sysml
standard library package Broken {
    function f {
        in : ;
        in x: Real[1];
        return : Real[1];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "function_declaration_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_calc_body_element") (severity error) (category parseerror) (span (offset 59) (line 3) (column 9) (len 15)) (message "unexpected token in calc body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Broken {
    function f {
        in : ;
        in x : Real[1];
        return : Real[1];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (library-package (name "Broken") (standard true) (body (kerml-classifier (keyword function) (abstract false) (name "f") (specializes none)))))
)
~~~
