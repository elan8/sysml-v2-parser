# META
~~~sexpr
(snapshot (type recovery) (description "A `**` exponent operator is not two LiteralInfinity literals: a feature value spelled `**` has no operand and is not accepted as a LiteralInfinity value (it falls to the opaque KerML feature fallback), while the following valid `*` value still parses."))
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
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 38) (line 2) (column 5) (len 20)) (message "the spec-valid KerML feature declaration production is retained but not structurally implemented"))
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
  )
  (root (package (name "LiteralInfinityRecovery") (body brace (feature-declaration) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "infinite") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 82) (line 3) (column 24) (len 1)) (infinity))))) (body semicolon)))))
)
~~~
