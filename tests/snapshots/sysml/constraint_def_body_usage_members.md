# META
~~~sexpr
(snapshot (type semantic) (description "A constraint definition body is a CalculationBody, whose items include usage members, so `attribute a : T;` is one attribute usage and the body has exactly one Expression element: its authored result expression `a <= 1`."))
~~~
# SOURCE
~~~sysml
package ConstraintBodyMembers {
    attribute def T;
    constraint def C {
        attribute a : T;
        private attribute b : T;
        a <= 1
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "constraint_def_body_usage_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ConstraintBodyMembers {
    attribute def T;
    constraint def C {
        attribute a : T;
        private attribute b : T;
        a <= 1;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 98) (line 4) (column 23) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 98) (line 4) (column 23) (len 1)))))
    (reference r1 (scope relative) (span (offset 131) (line 5) (column 31) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 131) (line 5) (column 31) (len 1)))))
    (reference r2 (scope relative) (span (offset 142) (line 6) (column 9) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 142) (line 6) (column 9) (len 1)))))
  )
  (root (package (name "ConstraintBodyMembers") (body brace (attribute-def (declaration-name "T") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (constraint-def (name "C") (modifiers) (specializes none) (body brace (attribute-usage (declaration-name "a") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "b") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (expression (span (offset 142) (line 6) (column 9) (len 6)) (binary (operator "<=") (left (expression (span (offset 142) (line 6) (column 9) (len 1)) (ref r2))) (right (expression (span (offset 147) (line 6) (column 14) (len 1)) (integer 1))))))))))
)
~~~
