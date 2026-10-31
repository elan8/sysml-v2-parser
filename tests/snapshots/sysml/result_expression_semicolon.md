# META
~~~sexpr
(snapshot (type recovery) (description "SysML CalculationBody and KerML FunctionBody share ResultExpressionMember without a terminating semicolon. Highlight exactly the illegal terminator, retain expressions and intervening comments losslessly, roll back their reference allocations, and preserve later declarations. Include valid semicolon-terminated feature bindings and unterminated result expressions as controls."))
~~~
# SOURCE
~~~sysml
package ResultTerminators {
    attribute value = 1;
    constraint bad { value == 1 //* ; inside a note */ ; }
    calc def badCalculation { value + 1; }
    function badFunction { value + 1; }
    constraint multiline {
        value == 1
        // A comment before the offending terminator.
        ;
    }
    constraint recoveredSiblings { value == 1; attribute retained = 2; }
    constraint good { value == 1 }
    calc def goodCalculation { value + 1 }
    function goodFunction { value + 1 }
    attribute 'after' = 3;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "result_expression_semicolon.md"
    (diagnostics
      (diagnostic (code "result_expression_semicolon") (severity error) (category parseerror) (span (offset 108) (line 3) (column 56) (len 1)) (message "a result expression must not end with a semicolon"))
      (diagnostic (code "result_expression_semicolon") (severity error) (category parseerror) (span (offset 151) (line 4) (column 40) (len 1)) (message "a result expression must not end with a semicolon"))
      (diagnostic (code "result_expression_semicolon") (severity error) (category parseerror) (span (offset 191) (line 5) (column 37) (len 1)) (message "a result expression must not end with a semicolon"))
      (diagnostic (code "result_expression_semicolon") (severity error) (category parseerror) (span (offset 303) (line 9) (column 9) (len 1)) (message "a result expression must not end with a semicolon"))
      (diagnostic (code "result_expression_semicolon") (severity error) (category parseerror) (span (offset 356) (line 11) (column 46) (len 1)) (message "a result expression must not end with a semicolon"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package ResultTerminators {
    attribute value = 1;
    constraint bad {
        value == 1 //* ; inside a note */ ;
    }
    calc def badCalculation {
        value + 1;
    }
    function badFunction {
        value + 1;
    }
    constraint multiline {
        value == 1
        // A comment before the offending terminator.
        ;
    }
    constraint recoveredSiblings {
        value == 1;
        attribute retained = 2;
    }
    constraint good {
        value == 1
    }
    calc def goodCalculation {
        value + 1
    }
    function goodFunction {
        value + 1
    }
    attribute 'after' = 3;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 406) (line 12) (column 23) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 406) (line 12) (column 23) (len 5)))))
    (reference r1 (scope relative) (span (offset 450) (line 13) (column 32) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 450) (line 13) (column 32) (len 5)))))
    (reference r2 (scope relative) (span (offset 490) (line 14) (column 29) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 490) (line 14) (column 29) (len 5)))))
  )
  (root (package (visibility none) (name "ResultTerminators") (body brace (attribute-usage) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bad") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (malformed (code "result_expression_semicolon") (found ";") (span (offset 74) (line 3) (column 22) (len 35)) (diagnostic-span (span (offset 108) (line 3) (column 56) (len 1)))))) (calc-def (name "badCalculation") (modifiers) (body brace (malformed (code "result_expression_semicolon") (found ";") (span (offset 142) (line 4) (column 31) (len 10)) (diagnostic-span (span (offset 151) (line 4) (column 40) (len 1)))))) (kerml-classifier (keyword function) (abstract false) (name "badFunction") (multiplicity none) (specializes none) (conjugates none) (body brace (malformed (code "result_expression_semicolon") (found ";") (span (offset 182) (line 5) (column 28) (len 10)) (diagnostic-span (span (offset 191) (line 5) (column 37) (len 1)))))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "multiline") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (malformed (code "result_expression_semicolon") (found ";") (span (offset 230) (line 7) (column 9) (len 74)) (diagnostic-span (span (offset 303) (line 9) (column 9) (len 1)))))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "recoveredSiblings") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (malformed (code "result_expression_semicolon") (found ";") (span (offset 346) (line 11) (column 36) (len 11)) (diagnostic-span (span (offset 356) (line 11) (column 46) (len 1)))) (attribute-usage (declaration-name "retained") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 379) (line 11) (column 69) (len 1)) (integer 2))))) (body semicolon)))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "good") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (expression (span (offset 406) (line 12) (column 23) (len 10)) (binary (operator "==") (left (expression (span (offset 406) (line 12) (column 23) (len 5)) (ref r0))) (right (expression (span (offset 415) (line 12) (column 32) (len 1)) (integer 1))))))) (calc-def (name "goodCalculation") (modifiers) (body brace (expression (expression (span (offset 450) (line 13) (column 32) (len 9)) (binary (operator "+") (left (expression (span (offset 450) (line 13) (column 32) (len 5)) (ref r1))) (right (expression (span (offset 458) (line 13) (column 40) (len 1)) (integer 1)))))))) (kerml-classifier (keyword function) (abstract false) (name "goodFunction") (multiplicity none) (specializes none) (conjugates none) (body brace (expression (expression (span (offset 490) (line 14) (column 29) (len 9)) (binary (operator "+") (left (expression (span (offset 490) (line 14) (column 29) (len 5)) (ref r2))) (right (expression (span (offset 498) (line 14) (column 37) (len 1)) (integer 1)))))))) (attribute-usage))))
)
~~~
