# META
~~~sexpr
(snapshot (type recovery) (description "An empty bracket operand is a malformed BracketExpression, not a unit-name failure, and recovery preserves the later valid attribute sibling."))
~~~
# SOURCE
~~~sysml
package BracketExpressionRecovery {
    part def P {
        attribute broken = [];
        attribute retained = 60[SI::mm];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "bracket_expression_recovery.md"
    (diagnostics
      (diagnostic (code "invalid_bracket_expression") (severity error) (category parseerror) (span (offset 61) (line 3) (column 9) (len 31)) (message "expected an expression inside '[ ]'"))
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
    (reference r0 (scope relative) (span (offset 116) (line 4) (column 33) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 116) (line 4) (column 33) (len 2))) (segment 1 (token "mm") (name "mm") (separator colon-colon) (span (offset 120) (line 4) (column 37) (len 2)))))
  )
  (root (package (name "BracketExpressionRecovery") (body brace (part-def (name "P") (modifiers) (body brace (malformed (code "invalid_bracket_expression") (found "attribute broken = [];") (span (offset 61) (line 3) (column 9) (len 31))) (attribute-usage (declaration-name "retained") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 113) (line 4) (column 30) (len 10)) (bracket (base (expression (span (offset 113) (line 4) (column 30) (len 2)) (integer 60))) (operands (sequence-list (element first (expression (span (offset 116) (line 4) (column 33) (len 6)) (ref r0)))))))))) (body semicolon)))))))
)
~~~
