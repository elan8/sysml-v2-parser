# META
~~~sexpr
(snapshot (type semantic) (description "Malformed CalculationUsage prefixes (slots out of grammar order, a dangling prefix with no kind keyword) are reported where they occur, and the valid calculation usages that follow each one are still parsed as members of the same body."))
~~~
# SOURCE
~~~sysml
package CalculationPrefixRecovery {
    calc def C1;
    individual ref calc wrongOrder;
    calc afterWrongOrder : C1;
    calc def Host {
        snapshot individual calc reversed;
        calc afterReversed : C1;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "calculation_usage_prefix_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 57) (line 3) (column 5) (len 31)) (message "unexpected token in package body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 148) (line 6) (column 9) (len 43)) (message "unexpected keyword `snapshot` in calc body"))
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
    (reference r0 (scope relative) (span (offset 116) (line 4) (column 28) (len 2)) (segments (segment 0 (token "C1") (name "C1") (separator none) (span (offset 116) (line 4) (column 28) (len 2)))))
    (reference r1 (scope relative) (span (offset 212) (line 7) (column 30) (len 2)) (segments (segment 0 (token "C1") (name "C1") (separator none) (span (offset 212) (line 7) (column 30) (len 2)))))
  )
  (root (package (name "CalculationPrefixRecovery") (body brace (calc-def (name "C1") (modifiers) (body semicolon)) (malformed (code "recovered_package_body_element") (found "individual ref calc wrongOrder;") (span (offset 57) (line 3) (column 5) (len 31))) (calc-usage (name "afterWrongOrder") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-def (name "Host") (modifiers) (body brace (malformed (code "unexpected_keyword_in_scope") (found "snapshot individual calc reversed;") (span (offset 148) (line 6) (column 9) (len 43))) (calc-usage (name "afterReversed") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))
)
~~~
