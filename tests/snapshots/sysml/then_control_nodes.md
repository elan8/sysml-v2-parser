# META
~~~sexpr
(snapshot (type semantic) (description "All four ControlNode spellings retain their anonymous or named UsageDeclaration state and mandatory ActionBody when used as then targets. SysML textual BNF 969-998, with zero-width-capable UsageDeclaration at 42-44 and 308-312; Pilot SysML.xtext 1650-1685 spells the declaration optional."))
~~~
# SOURCE
~~~sysml
package ThenControls {
    action def DefinitionOwner {
        then merge;
        then merge MergeNode { action mergeMember; }
        then decide;
        then decide DecisionNode { action decisionMember; }
        then join;
        then join JoinNode { action joinMember; }
        then fork;
        then fork ForkNode { action forkMember; }
    }
    action usageOwner {
        then merge;
        then decide;
        then join;
        then fork;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "then_control_nodes.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ThenControls {
    action def DefinitionOwner {
        then merge;
        then merge MergeNode {
            action mergeMember;
        }
        then decide;
        then decide DecisionNode {
            action decisionMember;
        }
        then join;
        then join JoinNode {
            action joinMember;
        }
        then fork;
        then fork ForkNode {
            action forkMember;
        }
    }
    action usageOwner {
        then merge;
        then decide;
        then join;
        then fork;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 95) (line 4) (column 20) (len 9)) (segments (segment 0 (token "MergeNode") (name "MergeNode") (separator none) (span (offset 95) (line 4) (column 20) (len 9)))))
    (reference r1 (scope relative) (span (offset 170) (line 6) (column 21) (len 12)) (segments (segment 0 (token "DecisionNode") (name "DecisionNode") (separator none) (span (offset 170) (line 6) (column 21) (len 12)))))
    (reference r2 (scope relative) (span (offset 247) (line 8) (column 19) (len 8)) (segments (segment 0 (token "JoinNode") (name "JoinNode") (separator none) (span (offset 247) (line 8) (column 19) (len 8)))))
    (reference r3 (scope relative) (span (offset 316) (line 10) (column 19) (len 8)) (segments (segment 0 (token "ForkNode") (name "ForkNode") (separator none) (span (offset 316) (line 10) (column 19) (len 8)))))
  )
  (root (package (name "ThenControls") (body brace (action-def (name "DefinitionOwner") (modifiers) (specializes none) (body brace (then-control (merge (declaration anonymous) (body semicolon (span (span (offset 74) (line 3) (column 19) (len 1)))))) (then-control (merge (declaration (named (expression (span (offset 95) (line 4) (column 20) (len 9)) (ref r0)))) (body brace (open-brace (span (offset 105) (line 4) (column 30) (len 1))) (members (action-usage (keyword action) (name "mergeMember") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))) (close-brace (span (offset 127) (line 4) (column 52) (len 1)))))) (then-control (decide (declaration anonymous) (body semicolon (span (span (offset 148) (line 5) (column 20) (len 1)))))) (then-control (decide (declaration (named (expression (span (offset 170) (line 6) (column 21) (len 12)) (ref r1)))) (body brace (open-brace (span (offset 183) (line 6) (column 34) (len 1))) (members (action-usage (keyword action) (name "decisionMember") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))) (close-brace (span (offset 208) (line 6) (column 59) (len 1)))))) (then-control (join (declaration anonymous) (body semicolon (span (span (offset 227) (line 7) (column 18) (len 1)))))) (then-control (join (declaration (named (expression (span (offset 247) (line 8) (column 19) (len 8)) (ref r2)))) (body brace (open-brace (span (offset 256) (line 8) (column 28) (len 1))) (members (action-usage (keyword action) (name "joinMember") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))) (close-brace (span (offset 277) (line 8) (column 49) (len 1)))))) (then-control (fork (declaration anonymous) (body semicolon (span (span (offset 296) (line 9) (column 18) (len 1)))))) (then-control (fork (declaration (named (expression (span (offset 316) (line 10) (column 19) (len 8)) (ref r3)))) (body brace (open-brace (span (offset 325) (line 10) (column 28) (len 1))) (members (action-usage (keyword action) (name "forkMember") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))) (close-brace (span (offset 346) (line 10) (column 49) (len 1)))))))) (action-usage (keyword action) (name "usageOwner") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (then-control (merge (declaration anonymous) (body semicolon (span (span (offset 396) (line 13) (column 19) (len 1)))))) (then-control (decide (declaration anonymous) (body semicolon (span (span (offset 417) (line 14) (column 20) (len 1)))))) (then-control (join (declaration anonymous) (body semicolon (span (span (offset 436) (line 15) (column 18) (len 1)))))) (then-control (fork (declaration anonymous) (body semicolon (span (span (offset 455) (line 16) (column 18) (len 1)))))))))))
)
~~~
