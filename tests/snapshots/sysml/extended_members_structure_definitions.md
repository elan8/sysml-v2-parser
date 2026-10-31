# META
~~~sexpr
(snapshot (type semantic) (description "Issue 200 case 1: extended members structure definitions preserve typed ExtendedDefinition and ExtendedUsage members through their grammar-owned bodies."))
~~~
# SOURCE
~~~sysml
package ExtendedOwners {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    part def P {
        #q def Inner;
        #q instance : Inner;
    }
    item def I {
        #q def Inner;
        #q instance : Inner;
    }
    attribute def A {
        #q def Inner;
        #q instance : Inner;
    }
    port def Port {
        #q def Inner;
        #q instance : Inner;
    }
    connection def C {
        #q def Inner;
        #q instance : Inner;
    }
    occurrence def O {
        #q def Inner;
        #q instance : Inner;
    }
    allocation def Allocation {
        #q def Inner;
        #q instance : Inner;
    }
    flow def Flow {
        #q def Inner;
        #q instance : Inner;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "extended_members_structure_definitions.md"
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
    (reference r0 (scope relative) (span (offset 51) (line 2) (column 27) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 51) (line 2) (column 27) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 64) (line 2) (column 40) (len 16)))))
    (reference r1 (scope relative) (span (offset 108) (line 4) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 108) (line 4) (column 10) (len 1)))))
    (reference r2 (scope relative) (span (offset 130) (line 5) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 130) (line 5) (column 10) (len 1)))))
    (reference r3 (scope relative) (span (offset 143) (line 5) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 143) (line 5) (column 23) (len 5)))))
    (reference r4 (scope relative) (span (offset 182) (line 8) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 182) (line 8) (column 10) (len 1)))))
    (reference r5 (scope relative) (span (offset 204) (line 9) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 204) (line 9) (column 10) (len 1)))))
    (reference r6 (scope relative) (span (offset 217) (line 9) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 217) (line 9) (column 23) (len 5)))))
    (reference r7 (scope relative) (span (offset 261) (line 12) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 261) (line 12) (column 10) (len 1)))))
    (reference r8 (scope relative) (span (offset 283) (line 13) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 283) (line 13) (column 10) (len 1)))))
    (reference r9 (scope relative) (span (offset 296) (line 13) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 296) (line 13) (column 23) (len 5)))))
    (reference r10 (scope relative) (span (offset 338) (line 16) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 338) (line 16) (column 10) (len 1)))))
    (reference r11 (scope relative) (span (offset 360) (line 17) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 360) (line 17) (column 10) (len 1)))))
    (reference r12 (scope relative) (span (offset 373) (line 17) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 373) (line 17) (column 23) (len 5)))))
    (reference r13 (scope relative) (span (offset 418) (line 20) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 418) (line 20) (column 10) (len 1)))))
    (reference r14 (scope relative) (span (offset 440) (line 21) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 440) (line 21) (column 10) (len 1)))))
    (reference r15 (scope relative) (span (offset 453) (line 21) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 453) (line 21) (column 23) (len 5)))))
    (reference r16 (scope relative) (span (offset 498) (line 24) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 498) (line 24) (column 10) (len 1)))))
    (reference r17 (scope relative) (span (offset 520) (line 25) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 520) (line 25) (column 10) (len 1)))))
    (reference r18 (scope relative) (span (offset 533) (line 25) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 533) (line 25) (column 23) (len 5)))))
    (reference r19 (scope relative) (span (offset 587) (line 28) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 587) (line 28) (column 10) (len 1)))))
    (reference r20 (scope relative) (span (offset 609) (line 29) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 609) (line 29) (column 10) (len 1)))))
    (reference r21 (scope relative) (span (offset 622) (line 29) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 622) (line 29) (column 23) (len 5)))))
    (reference r22 (scope relative) (span (offset 664) (line 32) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 664) (line 32) (column 10) (len 1)))))
    (reference r23 (scope relative) (span (offset 686) (line 33) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 686) (line 33) (column 10) (len 1)))))
    (reference r24 (scope relative) (span (offset 699) (line 33) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 699) (line 33) (column 23) (len 5)))))
  )
  (root (package (visibility none) (name "ExtendedOwners") (body brace (metadata-def (name "Q") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (part-def (name "P") (modifiers) (body brace (extended-def (prefix-keywords ((ref r1))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r2)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (item-def (name "I") (modifiers) (individual false) (specializes none) (body brace (extended-def (prefix-keywords ((ref r4))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r5)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (attribute-def (declaration-name "A") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (extended-def (prefix-keywords ((ref r7))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r8)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (port-def (name "Port") (modifiers) (specializes none) (body brace (extended-def (prefix-keywords ((ref r10))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r11)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (connection-def (name "C") (modifiers) (extensions) (specializes none) (body brace (extended-def (prefix-keywords ((ref r13))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r14)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (occurrence-def (name "O") (short-name none) (modifiers) (specializes none) (body brace (extended-def (prefix-keywords ((ref r16))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r17)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (allocation-def (name "Allocation") (modifiers) (body brace (extended-def (prefix-keywords ((ref r19))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r20)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (flow-def (name "Flow") (modifiers) (body brace (extended-def (prefix-keywords ((ref r22))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r23)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))))))
)
~~~
