# META
~~~sexpr
(snapshot (type semantic) (description "Issue 200 case 1: extended members behavior case usages preserve typed ExtendedDefinition and ExtendedUsage members through their grammar-owned bodies."))
~~~
# SOURCE
~~~sysml
package ExtendedOwners {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    action a {
        #q def Inner;
        #q instance : Inner;
    }
    state s {
        #q def Inner;
        #q instance : Inner;
    }
    calc c {
        #q def Inner;
        #q instance : Inner;
    }
    constraint check {
        #q def Inner;
        #q instance : Inner;
    }
    requirement r {
        #q def Inner;
        #q instance : Inner;
    }
    concern issue {
        #q def Inner;
        #q instance : Inner;
    }
    case c {
        #q def Inner;
        #q instance : Inner;
    }
    analysis a {
        #q def Inner;
        #q instance : Inner;
    }
    verification v {
        #q def Inner;
        #q instance : Inner;
    }
    use case u {
        #q def Inner;
        #q instance : Inner;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "extended_members_behavior_case_usages.md"
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
    (reference r1 (scope relative) (span (offset 106) (line 4) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 106) (line 4) (column 10) (len 1)))))
    (reference r2 (scope relative) (span (offset 128) (line 5) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 128) (line 5) (column 10) (len 1)))))
    (reference r3 (scope relative) (span (offset 141) (line 5) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 141) (line 5) (column 23) (len 5)))))
    (reference r4 (scope relative) (span (offset 177) (line 8) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 177) (line 8) (column 10) (len 1)))))
    (reference r5 (scope relative) (span (offset 199) (line 9) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 199) (line 9) (column 10) (len 1)))))
    (reference r6 (scope relative) (span (offset 212) (line 9) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 212) (line 9) (column 23) (len 5)))))
    (reference r7 (scope relative) (span (offset 247) (line 12) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 247) (line 12) (column 10) (len 1)))))
    (reference r8 (scope relative) (span (offset 269) (line 13) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 269) (line 13) (column 10) (len 1)))))
    (reference r9 (scope relative) (span (offset 282) (line 13) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 282) (line 13) (column 23) (len 5)))))
    (reference r10 (scope relative) (span (offset 327) (line 16) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 327) (line 16) (column 10) (len 1)))))
    (reference r11 (scope relative) (span (offset 349) (line 17) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 349) (line 17) (column 10) (len 1)))))
    (reference r12 (scope relative) (span (offset 362) (line 17) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 362) (line 17) (column 23) (len 5)))))
    (reference r13 (scope relative) (span (offset 404) (line 20) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 404) (line 20) (column 10) (len 1)))))
    (reference r14 (scope relative) (span (offset 426) (line 21) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 426) (line 21) (column 10) (len 1)))))
    (reference r15 (scope relative) (span (offset 439) (line 21) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 439) (line 21) (column 23) (len 5)))))
    (reference r16 (scope relative) (span (offset 481) (line 24) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 481) (line 24) (column 10) (len 1)))))
    (reference r17 (scope relative) (span (offset 503) (line 25) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 503) (line 25) (column 10) (len 1)))))
    (reference r18 (scope relative) (span (offset 516) (line 25) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 516) (line 25) (column 23) (len 5)))))
    (reference r19 (scope relative) (span (offset 551) (line 28) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 551) (line 28) (column 10) (len 1)))))
    (reference r20 (scope relative) (span (offset 573) (line 29) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 573) (line 29) (column 10) (len 1)))))
    (reference r21 (scope relative) (span (offset 586) (line 29) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 586) (line 29) (column 23) (len 5)))))
    (reference r22 (scope relative) (span (offset 625) (line 32) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 625) (line 32) (column 10) (len 1)))))
    (reference r23 (scope relative) (span (offset 647) (line 33) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 647) (line 33) (column 10) (len 1)))))
    (reference r24 (scope relative) (span (offset 660) (line 33) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 660) (line 33) (column 23) (len 5)))))
    (reference r25 (scope relative) (span (offset 703) (line 36) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 703) (line 36) (column 10) (len 1)))))
    (reference r26 (scope relative) (span (offset 725) (line 37) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 725) (line 37) (column 10) (len 1)))))
    (reference r27 (scope relative) (span (offset 738) (line 37) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 738) (line 37) (column 23) (len 5)))))
    (reference r28 (scope relative) (span (offset 777) (line 40) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 777) (line 40) (column 10) (len 1)))))
    (reference r29 (scope relative) (span (offset 799) (line 41) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 799) (line 41) (column 10) (len 1)))))
    (reference r30 (scope relative) (span (offset 812) (line 41) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 812) (line 41) (column 23) (len 5)))))
  )
  (root (package (visibility none) (name "ExtendedOwners") (body brace (metadata-def (name "Q") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (action-usage (keyword action) (name "a") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (extended-def (prefix-keywords ((ref r1))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r2)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (state-usage (name "s") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (extended-def (prefix-keywords ((ref r4))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r5)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (calc-usage (name "c") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (extended-def (prefix-keywords ((ref r7))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r8)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "check") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (extended-def (prefix-keywords ((ref r10))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r11)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (requirement-usage (name "r") (short-name none) (typing none) (abstract false) (variation false) none (subsets none) (references none) (body brace (extended-def (prefix-keywords ((ref r13))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r14)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (concern-usage (name "issue") (short-name none) (visibility none) (abstract false) (individual false) (definition false) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (extended-def (prefix-keywords ((ref r16))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r17)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (case-usage (declaration (name "c") (short-name none) (abstract false) (type none) (subsets none)) (body brace (extended-def (prefix-keywords ((ref r19))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r20)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "a") (short-name none) (type none) (subsets none) (redefines none) (body brace (extended-def (prefix-keywords ((ref r22))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r23)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (verification-case-usage (declaration (name "v") (short-name none) (abstract false) (type none) (subsets none)) (body brace (extended-def (prefix-keywords ((ref r25))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r26)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (use-case-usage (declaration (name "u") (short-name none) (abstract false) (type none) (subsets none)) (variation false) (body brace (extended-def (prefix-keywords ((ref r28))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r29)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r30)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))))))
)
~~~
