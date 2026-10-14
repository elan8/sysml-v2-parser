# META
~~~sexpr
(snapshot (type recovery) (description "Only a FunctionBody owns a ResultExpressionMember (KerML BNF), so only a function or predicate reports a semicolon after its result expression. A classifier TypeBody keeps a terminated expression member without that diagnostic.") (name-language kerml))
~~~
# SOURCE
~~~sysml
package TypeBodyResultExpression {
    classifier C {
        feature x;
        x;
    }
    function F {
        in a : T;
        a
    }
    function G {
        in a : T;
        a;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "type_body_result_expression_semicolon.md"
    (diagnostics
      (diagnostic (code "result_expression_semicolon") (severity error) (category parseerror) (span (offset 185) (line 12) (column 10) (len 1)) (message "a result expression must not end with a semicolon"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package TypeBodyResultExpression {
    classifier C {
        feature x;
        x
    }
    function F {
        in a : T;
        a
    }
    function G {
        in a : T;
        a;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language kerml)
  (references
    (reference r0 (scope relative) (span (offset 81) (line 4) (column 9) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 81) (line 4) (column 9) (len 1)))))
    (reference r1 (scope relative) (span (offset 122) (line 7) (column 16) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 122) (line 7) (column 16) (len 1)))))
    (reference r2 (scope relative) (span (offset 133) (line 8) (column 9) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 133) (line 8) (column 9) (len 1)))))
    (reference r3 (scope relative) (span (offset 173) (line 11) (column 16) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 173) (line 11) (column 16) (len 1)))))
  )
  (root (package (name "TypeBodyResultExpression") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "C") (multiplicity none) (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "x") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (expression (expression (span (offset 81) (line 4) (column 9) (len 1)) (ref r0))))) (kerml-classifier (keyword function) (abstract false) (name "F") (multiplicity none) (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "a") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (expression (expression (span (offset 133) (line 8) (column 9) (len 1)) (ref r2))))) (kerml-classifier (keyword function) (abstract false) (name "G") (multiplicity none) (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "a") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (malformed (code "result_expression_semicolon") (found ";") (span (offset 184) (line 12) (column 9) (len 2)) (diagnostic-span (span (offset 185) (line 12) (column 10) (len 1)))))))))
)
~~~
