# META
~~~sexpr
(snapshot (type semantic) (description "A visibility-prefixed Message in a SysML CalculationBody stays a single typed flow usage whose membership retains the authored visibility."))
~~~
# SOURCE
~~~sysml
package CalculationBodyMessageVisibility {
    calc def C {
        private message m of T;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "calculation_body_message_visibility.md"
    (diagnostics
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
    (reference r0 (scope relative) (span (offset 89) (line 3) (column 30) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 89) (line 3) (column 30) (len 1)))))
  )
  (root (package (name "CalculationBodyMessageVisibility") (body brace (calc-def (name "C") (modifiers) (body brace (flow-usage (kind message) (visibility private) (declaration (declared (name "m") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload (name none) (type (ref r0)) (conjugated false) (multiplicity none)) (endpoints none))) (body (body semicolon))))))))
)
~~~
