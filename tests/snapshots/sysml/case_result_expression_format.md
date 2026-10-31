# META
~~~sexpr
(snapshot (type semantic) (description "ResultExpressionMember closes a case body with no terminator. The formatter writes none, because `x;` would reparse as a default reference usage and change the member, so format and reparse keep the result expression."))
~~~
# SOURCE
~~~sysml
case def Result {
    subject s;
    s.value
}
analysis def Analysis {
    subject s;
    s.value + 1
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "case_result_expression_format.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
case def Result {
    subject s;
    s.value
}

analysis def Analysis {
    subject s;
    s.value + 1
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 37) (line 3) (column 5) (len 1)) (segments (segment 0 (token "s") (name "s") (separator none) (span (offset 37) (line 3) (column 5) (len 1)))))
    (reference r1 (scope relative) (span (offset 39) (line 3) (column 7) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 39) (line 3) (column 7) (len 5)))))
    (reference r2 (scope relative) (span (offset 90) (line 7) (column 5) (len 1)) (segments (segment 0 (token "s") (name "s") (separator none) (span (offset 90) (line 7) (column 5) (len 1)))))
    (reference r3 (scope relative) (span (offset 92) (line 7) (column 7) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 92) (line 7) (column 7) (len 5)))))
  )
  (root (case-def (name "Result") (modifiers) (body brace (subject (name "s") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (expression (span (offset 37) (line 3) (column 5) (len 7)) (member-access (base (expression (span (offset 37) (line 3) (column 5) (len 1)) (ref r0))) (separator dot) (member (ref r1)))))) (analysis-case-def (name "Analysis") (modifiers) (body brace (subject (name "s") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (expression (span (offset 90) (line 7) (column 5) (len 11)) (binary (operator "+") (left (expression (span (offset 90) (line 7) (column 5) (len 7)) (member-access (base (expression (span (offset 90) (line 7) (column 5) (len 1)) (ref r2))) (separator dot) (member (ref r3))))) (right (expression (span (offset 100) (line 7) (column 15) (len 1)) (integer 1))))))))
)
~~~
