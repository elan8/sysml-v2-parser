# META
~~~sexpr
(snapshot (type semantic) (description "A ref member of a constraint definition body is an ActionBodyItem routed through the action body, which retains its default FeatureValue."))
~~~
# SOURCE
~~~sysml
package ConstraintBodyRefDefault {
    constraint def C {
        ref :>> a, b default that.that;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "constraint_body_ref_default.md"
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 87) (line 3) (column 30) (len 4)) (segments (segment 0 (token "that") (name "that") (separator none) (span (offset 87) (line 3) (column 30) (len 4)))))
    (reference r1 (scope relative) (span (offset 92) (line 3) (column 35) (len 4)) (segments (segment 0 (token "that") (name "that") (separator none) (span (offset 92) (line 3) (column 35) (len 4)))))
    (reference r2 (scope relative) (span (offset 74) (line 3) (column 17) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 74) (line 3) (column 17) (len 1)))))
    (reference r3 (scope relative) (span (offset 77) (line 3) (column 20) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 77) (line 3) (column 20) (len 1)))))
  )
  (root (package (name "ConstraintBodyRefDefault") (body brace (constraint-def (name "C") (modifiers) (specializes none) (body brace (ref (name none) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 87) (line 3) (column 30) (len 9)) (member-access (base (expression (span (offset 87) (line 3) (column 30) (len 4)) (ref r0))) (separator dot) (member (ref r1))))))) (redefines (relationship (kind redefines) (implied false) (targets (ref r2) (ref r3)))) (subsets none) (body semicolon)))))))
)
~~~
