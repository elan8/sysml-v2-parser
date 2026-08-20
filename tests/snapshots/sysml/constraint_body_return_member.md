# META
~~~sexpr
(snapshot (type semantic) (description "ConstraintDefinition and ConstraintUsage both end in CalculationBody (SysML BNF 1378, 1382, 1359), whose CalculationBodyItem = ActionBodyItem | ReturnParameterMember (SysML BNF 1366, 1370), so a constraint body owns a `return` member exactly as a calculation body does. It owned none: the Systems Library's `return result = allTrue(assumptions()) implies allTrue(constraints()) { doc }` (Requirements.sysml:41) reached the body's terminal expression arm, which read the keyword as a name, invented `'return';` and `result;` as two members, and reported the rest of the real member as recovered_constraint_body_element. Both spellings that own the body appear, with the named-and-valued library form, a typed form with a multiplicity, and a bare `return <expr>;`. The bare form declares no parameter, so it lands on the shared expression member, whose node has nowhere to keep the keyword: re-emission drops it, and that one line is the whole reason FORMAT is spelled out rather than idempotent. It is the loss a calculation body already had; this scope now shares it instead of shredding the member into two."))
~~~
# SOURCE
~~~sysml
package ConstraintBodyReturnMember {
    abstract constraint def RequirementConstraintCheck {
        constraint assumptions[0..*] :> constraintChecks, subperformances;
        constraint constraints[0..*] :> constraintChecks, subperformances;
        return result = allTrue(assumptions()) implies allTrue(constraints()) {
            doc
            /* If all the assumptions are true, then all the required constraints must hold. */
        }
    }
    constraint check : RequirementConstraintCheck {
        return verdict : Boolean[1] = allTrue(assumptions());
        return totalMass <= massLimit;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "constraint_body_return_member.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ConstraintBodyReturnMember {
    abstract constraint def RequirementConstraintCheck {
        constraint assumptions[0..*] :> constraintChecks, subperformances;
        constraint constraints[0..*] :> constraintChecks, subperformances;
        return result = allTrue(assumptions()) implies allTrue(constraints()) {
            doc
            /* If all the assumptions are true, then all the required constraints must hold. */
        }
    }
    constraint check : RequirementConstraintCheck {
        return verdict : Boolean[1] = allTrue(assumptions());
        totalMass <= massLimit;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 134) (line 3) (column 41) (len 16)) (segments (segment 0 (token "constraintChecks") (name "constraintChecks") (separator none) (span (offset 134) (line 3) (column 41) (len 16)))))
    (reference r1 (scope relative) (span (offset 152) (line 3) (column 59) (len 15)) (segments (segment 0 (token "subperformances") (name "subperformances") (separator none) (span (offset 152) (line 3) (column 59) (len 15)))))
    (reference r2 (scope relative) (span (offset 209) (line 4) (column 41) (len 16)) (segments (segment 0 (token "constraintChecks") (name "constraintChecks") (separator none) (span (offset 209) (line 4) (column 41) (len 16)))))
    (reference r3 (scope relative) (span (offset 227) (line 4) (column 59) (len 15)) (segments (segment 0 (token "subperformances") (name "subperformances") (separator none) (span (offset 227) (line 4) (column 59) (len 15)))))
    (reference r4 (scope relative) (span (offset 475) (line 10) (column 24) (len 26)) (segments (segment 0 (token "RequirementConstraintCheck") (name "RequirementConstraintCheck") (separator none) (span (offset 475) (line 10) (column 24) (len 26)))))
    (reference r5 (scope relative) (span (offset 581) (line 12) (column 16) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 581) (line 12) (column 16) (len 9)))))
    (reference r6 (scope relative) (span (offset 594) (line 12) (column 29) (len 9)) (segments (segment 0 (token "massLimit") (name "massLimit") (separator none) (span (offset 594) (line 12) (column 29) (len 9)))))
  )
  (root (package (name "ConstraintBodyReturnMember") (body brace (constraint-def (name "RequirementConstraintCheck") (modifiers (abstract (span (offset 41) (line 2) (column 5) (len 8)))) (specializes none) (body brace (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "assumptions") (short-name none) (type none) (multiplicity (lower (expression (span (offset 125) (line 3) (column 32) (len 1)) (integer 0))) (upper unbounded)) (subsets (relationship (kind subsets) (implied false) (targets (ref r0) (ref r1)))) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "constraints") (short-name none) (type none) (multiplicity (lower (expression (span (offset 200) (line 4) (column 32) (len 1)) (integer 0))) (upper unbounded)) (subsets (relationship (kind subsets) (implied false) (targets (ref r2) (ref r3)))) (redefines none) (body semicolon)) (return-declaration (name "result") (short-name none)))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "check") (short-name none) (type (ref r4)) (multiplicity none) (subsets none) (redefines none) (body brace (return-declaration (name "verdict") (short-name none)) (expression (span (offset 581) (line 12) (column 16) (len 22)) (binary (operator "<=") (left (expression (span (offset 581) (line 12) (column 16) (len 9)) (ref r5))) (right (expression (span (offset 594) (line 12) (column 29) (len 9)) (ref r6))))))))))
)
~~~
