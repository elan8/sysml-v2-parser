# META
~~~sexpr
(snapshot (type semantic) (description "The guarded-successions extracted from the upstream DecisionTest retain their optional `succession` UsageDeclaration, feature-chain source, `if` guard, typed connector-end target, and DefinitionBody. This is SysML textual BNF GuardedSuccession 1180-1185; pinned Pilot SysML.xtext 1719-1725 agrees."))
~~~
# SOURCE
~~~sysml
action def DecisionTest {
    attribute x = 1;

    succession S first A1
        if x == 0 then A2 {
        attribute branch;
    }

    action A1;
    action A2;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "guarded_succession_decision_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
action def DecisionTest {
    attribute x = 1;
    succession S first A1 if x == 0 then A2 {
        attribute branch;
    }
    action A1;
    action A2;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 71) (line 4) (column 24) (len 2)) (segments (segment 0 (token "A1") (name "A1") (separator none) (span (offset 71) (line 4) (column 24) (len 2)))))
    (reference r1 (scope relative) (span (offset 85) (line 5) (column 12) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 85) (line 5) (column 12) (len 1)))))
    (reference r2 (scope relative) (span (offset 97) (line 5) (column 24) (len 2)) (segments (segment 0 (token "A2") (name "A2") (separator none) (span (offset 97) (line 5) (column 24) (len 2)))))
  )
  (root (action-def (name "DecisionTest") (modifiers) (specializes none) (body brace (attribute-usage) (guarded-succession (succession (keyword (span (offset 52) (line 4) (column 5) (len 10))) (declaration (name "S") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)))) (first (ref r0)) (if (span (offset 82) (line 5) (column 9) (len 2))) (guard (expression (span (offset 85) (line 5) (column 12) (len 6)) (binary (operator "==") (left (expression (span (offset 85) (line 5) (column 12) (len 1)) (ref r1))) (right (expression (span (offset 90) (line 5) (column 17) (len 1)) (integer 0)))))) (then (span (offset 92) (line 5) (column 19) (len 4))) (target (connector-end (multiplicity none) (target (ref r2)) (references none))) (body brace (attribute-usage (declaration-name "branch") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (action-usage (keyword action) (name "A1") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (action-usage (keyword action) (name "A2") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))
)
~~~
