# META
~~~sexpr
(snapshot (type semantic) (description "A materially different OccurrenceUsagePrefix on a part usage in every scope this parser dispatches PartUsage from -- package/namespace/root, part def body, part usage body, item def body, metadata def body, connection def body, occurrence body, use case def body, calc def body, action def body, action usage body, perform body and variant member -- with `#Tag part sameSyntax : Engine;` repeated so identical syntax can be compared across scopes: a part usage means the same thing wherever it is written, and the projection is identical in every scope that reaches it. Five owning nodes still project as contentless markers of their own -- item def, metadata def, an occurrence usage body, an action usage body and a variant member -- so nothing inside them appears here, part usage or otherwise. That is a gap in those families' projections, not in this one; see planning/part-usage-prefix-matrix.md §11."))
~~~
# SOURCE
~~~sysml
package PartPrefixOwningScopes {
    metadata def Tag;
    part def Engine;
    part def Wheel;
    ref part packageScope : Engine;
    #Tag part packageTagged : Engine;
    snapshot part packageSnapshot;
    part def DefinitionScope {
        in derived ref part inDefinition : Engine;
        #Tag part sameSyntax : Engine;
    }
    part usageScope : Engine {
        individual timeslice part inUsage;
        #Tag part sameSyntax : Engine;
    }
    item def ItemScope {
        ref part inItemBody : Engine;
        #Tag part sameSyntax : Engine;
    }
    metadata def MetadataScope {
        ref part inMetadataBody : Engine;
    }
    connection def ConnectionScope {
        ref part inConnectionDef : Engine;
        #Tag part sameSyntax : Engine;
    }
    occurrence occurrenceScope {
        ref individual part inOccurrenceBody : Engine;
        #Tag part sameSyntax : Engine;
    }
    use case def UseCaseScope {
        ref part inUseCaseDef : Engine;
        #Tag part sameSyntax : Engine;
    }
    calc def CalcScope {
        in part inCalcDef : Engine;
    }
    action def ActionScope {
        ref part inActionDef : Engine;
        #Tag part sameSyntax : Engine;
    }
    action actionUsageScope {
        ref part inActionUsage : Engine;
        #Tag part sameSyntax : Engine;
    }
    part def PerformScope {
        perform action inPerform {
            ref part inPerformBody : Engine;
        }
    }
    variation part def VariantScope {
        variant part variantMember : Engine;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "part_usage_prefix_owning_scopes.md"
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
    (reference r0 (scope relative) (span (offset 124) (line 5) (column 29) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 124) (line 5) (column 29) (len 6)))))
    (reference r1 (scope relative) (span (offset 137) (line 6) (column 6) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 137) (line 6) (column 6) (len 3)))))
    (reference r2 (scope relative) (span (offset 162) (line 6) (column 31) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 162) (line 6) (column 31) (len 6)))))
    (reference r3 (scope relative) (span (offset 279) (line 9) (column 44) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 279) (line 9) (column 44) (len 6)))))
    (reference r4 (scope relative) (span (offset 296) (line 10) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 296) (line 10) (column 10) (len 3)))))
    (reference r5 (scope relative) (span (offset 318) (line 10) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 318) (line 10) (column 32) (len 6)))))
    (reference r6 (scope relative) (span (offset 354) (line 12) (column 23) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 354) (line 12) (column 23) (len 6)))))
    (reference r7 (scope relative) (span (offset 415) (line 14) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 415) (line 14) (column 10) (len 3)))))
    (reference r8 (scope relative) (span (offset 437) (line 14) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 437) (line 14) (column 32) (len 6)))))
    (reference r9 (scope relative) (span (offset 712) (line 24) (column 36) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 712) (line 24) (column 36) (len 6)))))
    (reference r10 (scope relative) (span (offset 729) (line 25) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 729) (line 25) (column 10) (len 3)))))
    (reference r11 (scope relative) (span (offset 751) (line 25) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 751) (line 25) (column 32) (len 6)))))
    (reference r12 (scope relative) (span (offset 962) (line 32) (column 33) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 962) (line 32) (column 33) (len 6)))))
    (reference r13 (scope relative) (span (offset 979) (line 33) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 979) (line 33) (column 10) (len 3)))))
    (reference r14 (scope relative) (span (offset 1001) (line 33) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1001) (line 33) (column 32) (len 6)))))
    (reference r15 (scope relative) (span (offset 1068) (line 36) (column 29) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1068) (line 36) (column 29) (len 6)))))
    (reference r16 (scope relative) (span (offset 1142) (line 39) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1142) (line 39) (column 32) (len 6)))))
    (reference r17 (scope relative) (span (offset 1159) (line 40) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1159) (line 40) (column 10) (len 3)))))
    (reference r18 (scope relative) (span (offset 1181) (line 40) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1181) (line 40) (column 32) (len 6)))))
    (reference r19 (scope relative) (span (offset 1411) (line 48) (column 38) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1411) (line 48) (column 38) (len 6)))))
  )
  (root (package (name "PartPrefixOwningScopes") (body brace (metadata-def) (part-def (name "Engine") (body semicolon)) (part-def (name "Wheel") (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "packageScope") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r1))) (declaration-name "packageTagged") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration-name "packageSnapshot") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-def (name "DefinitionScope") (body brace (part-usage (prefix (direction in) (derived true) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inDefinition") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r4))) (declaration-name "sameSyntax") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "usageScope") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion timeslice) (extensions)) (declaration-name "inUsage") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r7))) (declaration-name "sameSyntax") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (item-def) (metadata-def) (connection-def (name "ConnectionScope") (modifiers) (role ordinary) (specializes none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inConnectionDef") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r10))) (declaration-name "sameSyntax") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "occurrenceScope") (short-name none) (target none)) (use-case-def (name "UseCaseScope") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inUseCaseDef") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r13))) (declaration-name "sameSyntax") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (calc-def (name "CalcScope") (body brace (part-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inCalcDef") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (action-def (name "ActionScope") (specializes none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inActionDef") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r17))) (declaration-name "sameSyntax") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (action-usage (name "actionUsageScope") (short-name none)) (part-def (name "PerformScope") (body brace (perform (declaration "inPerform") (action none) (typing none) (subsets none) (redefines none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inPerformBody") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))))) (part-def (name "VariantScope") (body brace (variant-usage))))))
)
~~~
