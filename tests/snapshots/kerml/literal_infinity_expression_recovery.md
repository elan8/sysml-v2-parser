# META
~~~sexpr
(snapshot (type recovery) (description "A malformed ** exponent with no operand produces an error recovery node; the following valid LiteralInfinity value remains typed.") (name-language kerml))
~~~
# SOURCE
~~~sysml
package LiteralInfinityRecovery {
    feature broken = **;
    feature infinite = *;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "literal_infinity_expression_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 38) (line 2) (column 5) (len 20)) (message "unexpected token in package body"))
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
  (root (package (name "LiteralInfinityRecovery") (body brace (malformed (code "recovered_package_body_element") (found "feature broken = **;") (span (offset 38) (line 2) (column 5) (len 20))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "infinite") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 82) (line 3) (column 24) (len 1)) (infinity))))) (body semicolon)))))
)
~~~
