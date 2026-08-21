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
    (reference r0 (scope relative) (span (offset 80) (line 4) (column 15) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 80) (line 4) (column 15) (len 4)))))
  )
  (root (library-package (name "Broken") (standard true) (body brace (kerml-classifier (keyword function) (abstract false) (name "f") (specializes none) (body brace (malformed (code "recovered_calc_body_element") (found "in : ;") (span (offset 59) (line 3) (column 9) (len 15))) (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "x") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity (lower (expression (span (offset 85) (line 4) (column 20) (len 1)) (integer 1))) (upper (expression (span (offset 85) (line 4) (column 20) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (return-declaration (name none) (short-name none)))))))
)
~~~
