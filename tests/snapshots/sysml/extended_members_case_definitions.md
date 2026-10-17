# META
~~~sexpr
(snapshot (type semantic) (description "Issue 200 case 1: extended members case definitions preserve typed ExtendedDefinition and ExtendedUsage members through their grammar-owned bodies."))
~~~
# SOURCE
~~~sysml
package ExtendedOwners {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    requirement def R {
        #q def Inner;
        #q instance : Inner;
    }
    concern def Concern {
        #q def Inner;
        #q instance : Inner;
    }
    case def Case {
        #q def Inner;
        #q instance : Inner;
    }
    analysis def Analysis {
        #q def Inner;
        #q instance : Inner;
    }
    verification def Verification {
        #q def Inner;
        #q instance : Inner;
    }
    use case def UseCase {
        #q def Inner;
        #q instance : Inner;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "extended_members_case_definitions.md"
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
    (reference r1 (scope relative) (span (offset 115) (line 4) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 115) (line 4) (column 10) (len 1)))))
    (reference r2 (scope relative) (span (offset 137) (line 5) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 137) (line 5) (column 10) (len 1)))))
    (reference r3 (scope relative) (span (offset 150) (line 5) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 150) (line 5) (column 23) (len 5)))))
    (reference r4 (scope relative) (span (offset 198) (line 8) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 198) (line 8) (column 10) (len 1)))))
    (reference r5 (scope relative) (span (offset 220) (line 9) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 220) (line 9) (column 10) (len 1)))))
    (reference r6 (scope relative) (span (offset 233) (line 9) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 233) (line 9) (column 23) (len 5)))))
    (reference r7 (scope relative) (span (offset 275) (line 12) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 275) (line 12) (column 10) (len 1)))))
    (reference r8 (scope relative) (span (offset 297) (line 13) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 297) (line 13) (column 10) (len 1)))))
    (reference r9 (scope relative) (span (offset 310) (line 13) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 310) (line 13) (column 23) (len 5)))))
    (reference r10 (scope relative) (span (offset 360) (line 16) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 360) (line 16) (column 10) (len 1)))))
    (reference r11 (scope relative) (span (offset 382) (line 17) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 382) (line 17) (column 10) (len 1)))))
    (reference r12 (scope relative) (span (offset 395) (line 17) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 395) (line 17) (column 23) (len 5)))))
    (reference r13 (scope relative) (span (offset 453) (line 20) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 453) (line 20) (column 10) (len 1)))))
    (reference r14 (scope relative) (span (offset 475) (line 21) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 475) (line 21) (column 10) (len 1)))))
    (reference r15 (scope relative) (span (offset 488) (line 21) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 488) (line 21) (column 23) (len 5)))))
    (reference r16 (scope relative) (span (offset 537) (line 24) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 537) (line 24) (column 10) (len 1)))))
    (reference r17 (scope relative) (span (offset 559) (line 25) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 559) (line 25) (column 10) (len 1)))))
    (reference r18 (scope relative) (span (offset 572) (line 25) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 572) (line 25) (column 23) (len 5)))))
  )
  (root (package (name "ExtendedOwners") (body brace (metadata-def (name "Q") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (requirement-def (name "R") (modifiers) (body brace (extended-def (prefix-keywords ((ref r1))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r2)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (concern-usage (name "Concern") (short-name none) (visibility none) (abstract false) (individual false) (definition true) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (extended-def (prefix-keywords ((ref r4))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r5)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (case-def (name "Case") (modifiers) (body brace (extended-def (prefix-keywords ((ref r7))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r8)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (analysis-case-def (name "Analysis") (modifiers) (body brace (extended-def (prefix-keywords ((ref r10))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r11)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (verification-case-def (name "Verification") (modifiers) (body brace (extended-def (prefix-keywords ((ref r13))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r14)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (use-case-def (name "UseCase") (modifiers) (body brace (extended-def (prefix-keywords ((ref r16))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r17)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))))))
)
~~~
