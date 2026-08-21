# META
~~~sexpr
(snapshot (type semantic) (description "Projects the complete pinned ForLoopNode production across action-definition, action-usage, and use-case-definition bodies: shared ActionNodePrefix, typed source-backed variable UsageDeclaration, in node parameter, and ActionBodyParameter. SysML textual BNF 954-965 and 1151-1155; pinned Pilot SysML.xtext 1438-1439 and 1624-1628."))
~~~
# SOURCE
~~~sysml
package ForLoopNodes {
    action def DefinitionOwner {
        ref action <iteration> iteration : Step for <index> index : Natural in sourceItems action <bodyStep> definitionBody : BodyStep {
            action nestedDefinitionBody;
        }
    }
    action UsageOwner {
        for entry : Item in usageItems->size() {
            action usageBody;
        }
    }
    use case def UseCaseOwner {
        for scenario : Scenario in useCaseItems {
            action useCaseBody;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "for_loop_nodes.md"
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
    (reference r0 (scope relative) (span (offset 99) (line 3) (column 44) (len 4)) (segments (segment 0 (token "Step") (name "Step") (separator none) (span (offset 99) (line 3) (column 44) (len 4)))))
    (reference r1 (scope relative) (span (offset 124) (line 3) (column 69) (len 7)) (segments (segment 0 (token "Natural") (name "Natural") (separator none) (span (offset 124) (line 3) (column 69) (len 7)))))
    (reference r2 (scope relative) (span (offset 135) (line 3) (column 80) (len 11)) (segments (segment 0 (token "sourceItems") (name "sourceItems") (separator none) (span (offset 135) (line 3) (column 80) (len 11)))))
    (reference r3 (scope relative) (span (offset 182) (line 3) (column 127) (len 8)) (segments (segment 0 (token "BodyStep") (name "BodyStep") (separator none) (span (offset 182) (line 3) (column 127) (len 8)))))
    (reference r4 (scope relative) (span (offset 294) (line 8) (column 21) (len 4)) (segments (segment 0 (token "Item") (name "Item") (separator none) (span (offset 294) (line 8) (column 21) (len 4)))))
    (reference r5 (scope relative) (span (offset 302) (line 8) (column 29) (len 10)) (segments (segment 0 (token "usageItems") (name "usageItems") (separator none) (span (offset 302) (line 8) (column 29) (len 10)))))
    (reference r6 (scope relative) (span (offset 424) (line 13) (column 24) (len 8)) (segments (segment 0 (token "Scenario") (name "Scenario") (separator none) (span (offset 424) (line 13) (column 24) (len 8)))))
    (reference r7 (scope relative) (span (offset 436) (line 13) (column 36) (len 12)) (segments (segment 0 (token "useCaseItems") (name "useCaseItems") (separator none) (span (offset 436) (line 13) (column 36) (len 12)))))
  )
  (root (package (name "ForLoopNodes") (body brace (action-def (name "DefinitionOwner") (modifiers) (specializes none) (body brace (for-loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (action-declaration (name "iteration") (short-name "iteration") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)))) (variable (for-variable (name "index") (short-name "index") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (in (expression (span (offset 135) (line 3) (column 80) (len 11)) (ref r2))) (body-parameter (action-declaration (name "definitionBody") (short-name "bodyStep") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (body brace (action-usage (name "nestedDefinitionBody") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))))))) (action-usage (name "UsageOwner") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (for-loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (variable (for-variable (name "entry") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (in (expression (span (offset 302) (line 8) (column 29) (len 18)) (collection-op (operator "size") (base (expression (span (offset 302) (line 8) (column 29) (len 10)) (ref r5))) (arguments) (brace-body none)))) (body-parameter (action-declaration none) (body brace (action-usage (name "usageBody") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))))))) (use-case-def (name "UseCaseOwner") (modifiers) (body brace (for-loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (variable (for-variable (name "scenario") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (in (expression (span (offset 436) (line 13) (column 36) (len 12)) (ref r7))) (body-parameter (action-declaration none) (body brace (action-usage (name "useCaseBody") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))))))))))
)
~~~
