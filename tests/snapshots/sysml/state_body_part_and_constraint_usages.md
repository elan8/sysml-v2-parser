# META
~~~sexpr
(snapshot (type semantic) (description "StateBodyItem owns PartUsage through NonBehaviorBodyItem -> StructureUsageMember and ConstraintUsage through its BehaviorUsageMember alternative (SysML textual BNF 1200-1205, 910-920, 262-268, 356-389, 623, and 1382-1395; pinned Pilot SysML agrees). Both existing source-backed usage nodes retain their typed headers and bodies in a state definition."))
~~~
# SOURCE
~~~sysml
package StateBodyPartAndConstraintUsages {
    part def Counter;
    constraint def Balanced;
    state def Counting {
        part counter : Counter;
        constraint balance : Balanced {
            counter == counter;
        }
        state done;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_body_part_and_constraint_usages.md"
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
    (reference r0 (scope relative) (span (offset 142) (line 5) (column 24) (len 7)) (segments (segment 0 (token "Counter") (name "Counter") (separator none) (span (offset 142) (line 5) (column 24) (len 7)))))
    (reference r1 (scope relative) (span (offset 180) (line 6) (column 30) (len 8)) (segments (segment 0 (token "Balanced") (name "Balanced") (separator none) (span (offset 180) (line 6) (column 30) (len 8)))))
    (reference r2 (scope relative) (span (offset 203) (line 7) (column 13) (len 7)) (segments (segment 0 (token "counter") (name "counter") (separator none) (span (offset 203) (line 7) (column 13) (len 7)))))
    (reference r3 (scope relative) (span (offset 214) (line 7) (column 24) (len 7)) (segments (segment 0 (token "counter") (name "counter") (separator none) (span (offset 214) (line 7) (column 24) (len 7)))))
  )
  (root (package (name "StateBodyPartAndConstraintUsages") (body brace (part-def (name "Counter") (modifiers) (body semicolon)) (constraint-def (name "Balanced") (modifiers) (specializes none) (body semicolon)) (state-def (name "Counting") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "counter") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "balance") (short-name none) (type (ref r1)) (multiplicity none) (subsets none) (redefines none) (body brace (expression (span (offset 203) (line 7) (column 13) (len 18)) (binary (operator "==") (left (expression (span (offset 203) (line 7) (column 13) (len 7)) (ref r2))) (right (expression (span (offset 214) (line 7) (column 24) (len 7)) (ref r3))))))) (state-usage))))))
)
~~~
