# META
~~~sexpr
(snapshot (type recovery) (description "A malformed direction-prefixed parameter declaration becomes an explicit recovery node with a precise span while the valid sibling parameter after it still parses."))
~~~
# SOURCE
~~~sysml
action def Recovering {
    in : ;
    in ok : Real = 1.0;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "directed_parameter_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 28) (line 2) (column 5) (len 11)) (message "unexpected token in action body"))
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
    (reference r0 (scope relative) (span (offset 47) (line 3) (column 13) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 47) (line 3) (column 13) (len 4)))))
  )
  (root (action-def (name "Recovering") (specializes none) (body brace (malformed (code "recovered_action_body_element") (found "in : ;") (span (offset 28) (line 2) (column 5) (len 11))) (in-out (direction in) (reference false) (declaration "ok") (subsets none) (type (ref r0)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 54) (line 3) (column 20) (len 3)) (real "1.0"))))) (span (offset 39) (line 3) (column 5) (len 19))))))
)
~~~
