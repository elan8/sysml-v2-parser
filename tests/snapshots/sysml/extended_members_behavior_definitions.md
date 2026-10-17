# META
~~~sexpr
(snapshot (type semantic) (description "Issue 200 case 1: extended members behavior definitions preserve typed ExtendedDefinition and ExtendedUsage members through their grammar-owned bodies."))
~~~
# SOURCE
~~~sysml
package ExtendedOwners {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    action def A {
        #q def Inner;
        #q instance : Inner;
    }
    state def S {
        #q def Inner;
        #q instance : Inner;
    }
    calc def C {
        #q def Inner;
        #q instance : Inner;
    }
    constraint def Check {
        #q def Inner;
        #q instance : Inner;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "extended_members_behavior_definitions.md"
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
    (reference r1 (scope relative) (span (offset 110) (line 4) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 110) (line 4) (column 10) (len 1)))))
    (reference r2 (scope relative) (span (offset 132) (line 5) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 132) (line 5) (column 10) (len 1)))))
    (reference r3 (scope relative) (span (offset 145) (line 5) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 145) (line 5) (column 23) (len 5)))))
    (reference r4 (scope relative) (span (offset 185) (line 8) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 185) (line 8) (column 10) (len 1)))))
    (reference r5 (scope relative) (span (offset 207) (line 9) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 207) (line 9) (column 10) (len 1)))))
    (reference r6 (scope relative) (span (offset 220) (line 9) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 220) (line 9) (column 23) (len 5)))))
    (reference r7 (scope relative) (span (offset 259) (line 12) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 259) (line 12) (column 10) (len 1)))))
    (reference r8 (scope relative) (span (offset 281) (line 13) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 281) (line 13) (column 10) (len 1)))))
    (reference r9 (scope relative) (span (offset 294) (line 13) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 294) (line 13) (column 23) (len 5)))))
    (reference r10 (scope relative) (span (offset 343) (line 16) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 343) (line 16) (column 10) (len 1)))))
    (reference r11 (scope relative) (span (offset 365) (line 17) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 365) (line 17) (column 10) (len 1)))))
    (reference r12 (scope relative) (span (offset 378) (line 17) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 378) (line 17) (column 23) (len 5)))))
  )
  (root (package (name "ExtendedOwners") (body brace (metadata-def (name "Q") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (action-def (name "A") (modifiers) (specializes none) (body brace (extended-def (prefix-keywords ((ref r1))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r2)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (state-def (name "S") (modifiers) (body brace (extended-def (prefix-keywords ((ref r4))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r5)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (calc-def (name "C") (modifiers) (body brace (extended-def (prefix-keywords ((ref r7))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r8)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (constraint-def (name "Check") (modifiers) (specializes none) (body brace (extended-def (prefix-keywords ((ref r10))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r11)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))))))
)
~~~
