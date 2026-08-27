# META
~~~sexpr
(snapshot (type semantic) (description "SysML 8.3.21.7 validates that RequirementConstraintMembership is owned by a RequirementDefinition or RequirementUsage. The parser retains the same typed require/assume constraint syntax in a part-definition body so semantic consumers can diagnose the invalid owner, while preserving declared names, referenced shorthand, typing, and bodies. Pilot SysML.xtext 2037-2069; SysML 2.0 clauses 8.2.2.21.1 and 8.3.21.7."))
~~~
# SOURCE
~~~sysml
package RequirementConstraintOwners {
    constraint def Bound;
    requirement def Good {
        require constraint valid : Bound;
    }
    part def Bad {
        require constraint invalid : Bound;
        assume constraint assumed { true }
        require existing;
        attribute retained;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "require_constraint_invalid_owner.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package RequirementConstraintOwners {
    constraint def Bound;
    requirement def Good {
        require constraint valid : Bound;
    }
    part def Bad {
        require constraint invalid : Bound;
        assume constraint assumed {
            true;
        }
        require existing;
        attribute retained;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 126) (line 4) (column 36) (len 5)) (segments (segment 0 (token "Bound") (name "Bound") (separator none) (span (offset 126) (line 4) (column 36) (len 5)))))
    (reference r1 (scope relative) (span (offset 195) (line 7) (column 38) (len 5)) (segments (segment 0 (token "Bound") (name "Bound") (separator none) (span (offset 195) (line 7) (column 38) (len 5)))))
    (reference r2 (scope relative) (span (offset 261) (line 9) (column 17) (len 8)) (segments (segment 0 (token "existing") (name "existing") (separator none) (span (offset 261) (line 9) (column 17) (len 8)))))
  )
  (root (package (name "RequirementConstraintOwners") (body brace (constraint-def (name "Bound") (modifiers) (specializes none) (body semicolon)) (requirement-def (name "Good") (modifiers) (body brace (require-constraint (kind require) (constraint-keyword true) (name "valid") (target none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)))) (part-def (name "Bad") (modifiers) (body brace (require-constraint (kind require) (constraint-keyword true) (name "invalid") (target none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (body semicolon)) (require-constraint (kind assume) (constraint-keyword true) (name "assumed") (target none) (typing none) (body brace (expression (span (offset 238) (line 8) (column 37) (len 4)) (boolean true)))) (require-constraint (kind require) (constraint-keyword false) (name none) (target (ref r2)) (typing none) (body semicolon)) (attribute-usage (declaration-name "retained") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
