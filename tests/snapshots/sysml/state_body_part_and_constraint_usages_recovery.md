# META
~~~sexpr
(snapshot (type recovery) (description "State-body recovery synchronizes at the PartUsage and ConstraintUsage FIRST sets after malformed content, preserving both typed members and a later state sibling. The owning alternatives are StateBodyItem -> NonBehaviorBodyItem -> StructureUsageMember and StateBodyItem -> BehaviorUsageMember (SysML textual BNF 1200-1205, 910-920, 262-268, 356-389, 623, and 1382-1395; pinned Pilot SysML agrees)."))
~~~
# SOURCE
~~~sysml
package StateBodyPartAndConstraintUsagesRecovery {
    state def Counting {
        nonsense ???;
        part laterPart : Counter;
        broken ???;
        constraint laterConstraint : Balanced;
        state done;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_body_part_and_constraint_usages_recovery.md"
    (diagnostics
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 84) (line 3) (column 9) (len 22)) (message "unrecognized declaration `nonsense` in state body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 140) (line 5) (column 9) (len 20)) (message "unrecognized declaration `broken` in state body"))
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
    (reference r0 (scope relative) (span (offset 123) (line 4) (column 26) (len 7)) (segments (segment 0 (token "Counter") (name "Counter") (separator none) (span (offset 123) (line 4) (column 26) (len 7)))))
    (reference r1 (scope relative) (span (offset 189) (line 6) (column 38) (len 8)) (segments (segment 0 (token "Balanced") (name "Balanced") (separator none) (span (offset 189) (line 6) (column 38) (len 8)))))
  )
  (root (package (name "StateBodyPartAndConstraintUsagesRecovery") (body brace (state-def (name "Counting") (modifiers) (body brace (malformed (code "unrecognized_declaration_in_scope") (found "nonsense ???;") (span (offset 84) (line 3) (column 9) (len 22))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "laterPart") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "unrecognized_declaration_in_scope") (found "broken ???;") (span (offset 140) (line 5) (column 9) (len 20))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "laterConstraint") (short-name none) (type (ref r1)) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (state-usage (name "done") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))))
)
~~~
