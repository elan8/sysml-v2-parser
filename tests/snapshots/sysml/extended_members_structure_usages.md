# META
~~~sexpr
(snapshot (type semantic) (description "Issue 200 case 1: extended members structure usages preserve typed ExtendedDefinition and ExtendedUsage members through their grammar-owned bodies."))
~~~
# SOURCE
~~~sysml
package ExtendedOwners {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    part p {
        #q def Inner;
        #q instance : Inner;
    }
    item i {
        #q def Inner;
        #q instance : Inner;
    }
    attribute a {
        #q def Inner;
        #q instance : Inner;
    }
    port p {
        #q def Inner;
        #q instance : Inner;
    }
    connection c {
        #q def Inner;
        #q instance : Inner;
    }
    occurrence o {
        #q def Inner;
        #q instance : Inner;
    }
    allocation assigned {
        #q def Inner;
        #q instance : Inner;
    }
    ref r {
        #q def Inner;
        #q instance : Inner;
    }
    enum value {
        #q def Inner;
        #q instance : Inner;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "extended_members_structure_usages.md"
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
    (reference r1 (scope relative) (span (offset 104) (line 4) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 104) (line 4) (column 10) (len 1)))))
    (reference r2 (scope relative) (span (offset 126) (line 5) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 126) (line 5) (column 10) (len 1)))))
    (reference r3 (scope relative) (span (offset 139) (line 5) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 139) (line 5) (column 23) (len 5)))))
    (reference r4 (scope relative) (span (offset 174) (line 8) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 174) (line 8) (column 10) (len 1)))))
    (reference r5 (scope relative) (span (offset 196) (line 9) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 196) (line 9) (column 10) (len 1)))))
    (reference r6 (scope relative) (span (offset 209) (line 9) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 209) (line 9) (column 23) (len 5)))))
    (reference r7 (scope relative) (span (offset 319) (line 16) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 319) (line 16) (column 10) (len 1)))))
    (reference r8 (scope relative) (span (offset 341) (line 17) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 341) (line 17) (column 10) (len 1)))))
    (reference r9 (scope relative) (span (offset 354) (line 17) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 354) (line 17) (column 23) (len 5)))))
    (reference r10 (scope relative) (span (offset 395) (line 20) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 395) (line 20) (column 10) (len 1)))))
    (reference r11 (scope relative) (span (offset 417) (line 21) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 417) (line 21) (column 10) (len 1)))))
    (reference r12 (scope relative) (span (offset 430) (line 21) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 430) (line 21) (column 23) (len 5)))))
    (reference r13 (scope relative) (span (offset 471) (line 24) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 471) (line 24) (column 10) (len 1)))))
    (reference r14 (scope relative) (span (offset 493) (line 25) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 493) (line 25) (column 10) (len 1)))))
    (reference r15 (scope relative) (span (offset 506) (line 25) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 506) (line 25) (column 23) (len 5)))))
    (reference r16 (scope relative) (span (offset 554) (line 28) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 554) (line 28) (column 10) (len 1)))))
    (reference r17 (scope relative) (span (offset 576) (line 29) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 576) (line 29) (column 10) (len 1)))))
    (reference r18 (scope relative) (span (offset 589) (line 29) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 589) (line 29) (column 23) (len 5)))))
    (reference r19 (scope relative) (span (offset 623) (line 32) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 623) (line 32) (column 10) (len 1)))))
    (reference r20 (scope relative) (span (offset 645) (line 33) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 645) (line 33) (column 10) (len 1)))))
    (reference r21 (scope relative) (span (offset 658) (line 33) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 658) (line 33) (column 23) (len 5)))))
    (reference r22 (scope relative) (span (offset 697) (line 36) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 697) (line 36) (column 10) (len 1)))))
    (reference r23 (scope relative) (span (offset 719) (line 37) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 719) (line 37) (column 10) (len 1)))))
    (reference r24 (scope relative) (span (offset 732) (line 37) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 732) (line 37) (column 23) (len 5)))))
  )
  (root (package (visibility none) (name "ExtendedOwners") (body brace (metadata-def (name "Q") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (extended-def (prefix-keywords ((ref r1))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r2)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "i") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (extended-def (prefix-keywords ((ref r4))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r5)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (attribute-usage) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (extended-def (prefix-keywords ((ref r7))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r8)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body brace (extended-def (prefix-keywords ((ref r10))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r11)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "o") (short-name none) (target none) (body brace (extended-def (prefix-keywords ((ref r13))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r14)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (allocation-usage (name "assigned") (short-name none) (typing none) (ends) (body brace (extended-def (prefix-keywords ((ref r16))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r17)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (ref (name "r") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body brace (extended-def (prefix-keywords ((ref r19))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r20)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (enumeration-usage (name "value") (short-name none) (typing none) (body brace (extended-def (prefix-keywords ((ref r22))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r23)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))))))
)
~~~
