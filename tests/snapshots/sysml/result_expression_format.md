# META
~~~sexpr
(snapshot (type semantic) (description "ResultExpressionMember formats without a terminating semicolon in constraints, calculations, and KerML functions. Keep explicitly introduced return expressions distinct and preserve the return keyword. Parse-format-reparse must preserve semantics and formatting must be idempotent."))
~~~
# SOURCE
~~~sysml
package ResultFormatting {
    attribute value = 1;
    constraint good { value == 1 }
    calc def goodCalculation { value + 1 }
    function goodFunction { value + 1 }
    calc def explicitReturn { return sum(values); }
    constraint explicitConstraintReturn { return check(values); }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "result_expression_format.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ResultFormatting {
    attribute value = 1;
    constraint good {
        value == 1
    }
    calc def goodCalculation {
        value + 1
    }
    function goodFunction {
        value + 1
    }
    calc def explicitReturn {
        return sum(values);
    }
    constraint explicitConstraintReturn {
        return check(values);
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 74) (line 3) (column 23) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 74) (line 3) (column 23) (len 5)))))
    (reference r1 (scope relative) (span (offset 118) (line 4) (column 32) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 118) (line 4) (column 32) (len 5)))))
    (reference r2 (scope relative) (span (offset 158) (line 5) (column 29) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 158) (line 5) (column 29) (len 5)))))
    (reference r3 (scope relative) (span (offset 207) (line 6) (column 38) (len 3)) (segments (segment 0 (token "sum") (name "sum") (separator none) (span (offset 207) (line 6) (column 38) (len 3)))))
    (reference r4 (scope relative) (span (offset 211) (line 6) (column 42) (len 6)) (segments (segment 0 (token "values") (name "values") (separator none) (span (offset 211) (line 6) (column 42) (len 6)))))
    (reference r5 (scope relative) (span (offset 271) (line 7) (column 50) (len 5)) (segments (segment 0 (token "check") (name "check") (separator none) (span (offset 271) (line 7) (column 50) (len 5)))))
    (reference r6 (scope relative) (span (offset 277) (line 7) (column 56) (len 6)) (segments (segment 0 (token "values") (name "values") (separator none) (span (offset 277) (line 7) (column 56) (len 6)))))
  )
  (root (package (name "ResultFormatting") (body brace (attribute-usage) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "good") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (expression (span (offset 74) (line 3) (column 23) (len 10)) (binary (operator "==") (left (expression (span (offset 74) (line 3) (column 23) (len 5)) (ref r0))) (right (expression (span (offset 83) (line 3) (column 32) (len 1)) (integer 1))))))) (calc-def (name "goodCalculation") (modifiers) (body brace (expression (expression (span (offset 118) (line 4) (column 32) (len 9)) (binary (operator "+") (left (expression (span (offset 118) (line 4) (column 32) (len 5)) (ref r1))) (right (expression (span (offset 126) (line 4) (column 40) (len 1)) (integer 1)))))))) (kerml-classifier (keyword function) (abstract false) (name "goodFunction") (multiplicity none) (specializes none) (conjugates none) (body brace (expression (expression (span (offset 158) (line 5) (column 29) (len 9)) (binary (operator "+") (left (expression (span (offset 158) (line 5) (column 29) (len 5)) (ref r2))) (right (expression (span (offset 166) (line 5) (column 37) (len 1)) (integer 1)))))))) (calc-def (name "explicitReturn") (modifiers) (body brace (return-expression (expression (span (offset 207) (line 6) (column 38) (len 11)) (invocation (callee (expression (span (offset 207) (line 6) (column 38) (len 3)) (ref r3))) (arguments (argument (parameter none) (value (expression (span (offset 211) (line 6) (column 42) (len 6)) (ref r4)))))))))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "explicitConstraintReturn") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (return-expression (expression (span (offset 271) (line 7) (column 50) (len 13)) (invocation (callee (expression (span (offset 271) (line 7) (column 50) (len 5)) (ref r5))) (arguments (argument (parameter none) (value (expression (span (offset 277) (line 7) (column 56) (len 6)) (ref r6)))))))))))))
)
~~~
