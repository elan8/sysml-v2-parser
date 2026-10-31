# META
~~~sexpr
(snapshot (type recovery) (description "ResultExpressionMember has no terminator and closes a CaseBody, as in a CalculationBody. `1 + 2;` reports result_expression_semicolon on the semicolon, an expression before another member is malformed, and `standard` cannot follow `library`."))
~~~
# SOURCE
~~~sysml
case def Terminated {
    1 + 2;
}
case def NotLast {
    1 + 2
    part later;
}
calc def CalcNotLast {
    1 + 2
    attribute later;
}
library standard package Legacy;
part retained;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "case_result_expression_recovery.md"
    (diagnostics
      (diagnostic (code "result_expression_semicolon") (severity error) (category parseerror) (span (offset 31) (line 2) (column 10) (len 1)) (message "a result expression must not end with a semicolon"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 58) (line 5) (column 5) (len 10)) (message "unrecognized declaration `1` in use case body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 109) (line 9) (column 5) (len 10)) (message "unrecognized declaration `1` in calc body"))
      (diagnostic (code "expected_alt") (severity error) (category parseerror) (span (offset 138) (line 12) (column 1) (len 32)) (message "expected a package member (e.g. part, port, attribute, action, requirement, import)"))
    )
  )
)
~~~
# FORMAT
~~~sysml
case def Terminated {
    1 + 2;
}

case def NotLast {
    1 + 2
    part later;
}

calc def CalcNotLast {
    1 + 2
    attribute later;
}

library standard package Legacy;

part retained;
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
  )
  (root (case-def (name "Terminated") (modifiers) (body brace (malformed (code "result_expression_semicolon") (found ";") (span (offset 26) (line 2) (column 5) (len 6)) (diagnostic-span (span (offset 31) (line 2) (column 10) (len 1)))))) (case-def (name "NotLast") (modifiers) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "1 + 2") (span (offset 58) (line 5) (column 5) (len 10))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "later") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (calc-def (name "CalcNotLast") (modifiers) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "1 + 2") (span (offset 109) (line 9) (column 5) (len 10))) (attribute-usage (declaration-name "later") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (malformed (code "expected_alt") (found "library standard package Legacy;") (span (offset 138) (line 12) (column 1) (len 32))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "retained") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))
)
~~~
