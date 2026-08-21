# META
~~~sexpr
(snapshot (type semantic) (description "A materially different OccurrenceUsagePrefix on a part usage in every scope this parser dispatches PartUsage from -- package/namespace/root, part def body, part usage body, item def body, metadata def body, connection def body, occurrence body, use case def body, calc def body, constraint def body, KerML type body, action def body, action usage body, perform body and variant member -- with `#Tag part sameSyntax : Engine;` repeated so identical syntax can be compared across scopes. Every one projects, and the copies of the repeated member are byte-identical apart from their spans and reference identities: a part usage means the same thing wherever it is written, and its projection does not depend on the body that owns it. The calc, constraint and KerML type bodies share one container (CalcDefBody), which had no PartUsage arm at all: `part p;` there fell through to the bare-expression fallback and was shredded into `'part';` and `p;` with no diagnostic, which is why a KerML type body appears here even though `part` is a SysML keyword -- preserving the member is strictly better than silently rewriting it."))
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
        part bareInCalcDef : Engine;
        #Tag part sameSyntax : Engine;
    }
    constraint def ConstraintScope {
        part inConstraintDef : Engine;
    }
    struct KermlTypeScope {
        part inKermlTypeBody : Engine;
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
    (reference r9 (scope relative) (span (offset 506) (line 17) (column 31) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 506) (line 17) (column 31) (len 6)))))
    (reference r10 (scope relative) (span (offset 523) (line 18) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 523) (line 18) (column 10) (len 3)))))
    (reference r11 (scope relative) (span (offset 545) (line 18) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 545) (line 18) (column 32) (len 6)))))
    (reference r12 (scope relative) (span (offset 626) (line 21) (column 35) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 626) (line 21) (column 35) (len 6)))))
    (reference r13 (scope relative) (span (offset 712) (line 24) (column 36) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 712) (line 24) (column 36) (len 6)))))
    (reference r14 (scope relative) (span (offset 729) (line 25) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 729) (line 25) (column 10) (len 3)))))
    (reference r15 (scope relative) (span (offset 751) (line 25) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 751) (line 25) (column 32) (len 6)))))
    (reference r16 (scope relative) (span (offset 845) (line 28) (column 48) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 845) (line 28) (column 48) (len 6)))))
    (reference r17 (scope relative) (span (offset 862) (line 29) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 862) (line 29) (column 10) (len 3)))))
    (reference r18 (scope relative) (span (offset 884) (line 29) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 884) (line 29) (column 32) (len 6)))))
    (reference r19 (scope relative) (span (offset 962) (line 32) (column 33) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 962) (line 32) (column 33) (len 6)))))
    (reference r20 (scope relative) (span (offset 979) (line 33) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 979) (line 33) (column 10) (len 3)))))
    (reference r21 (scope relative) (span (offset 1001) (line 33) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1001) (line 33) (column 32) (len 6)))))
    (reference r22 (scope relative) (span (offset 1068) (line 36) (column 29) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1068) (line 36) (column 29) (len 6)))))
    (reference r23 (scope relative) (span (offset 1105) (line 37) (column 30) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1105) (line 37) (column 30) (len 6)))))
    (reference r24 (scope relative) (span (offset 1122) (line 38) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1122) (line 38) (column 10) (len 3)))))
    (reference r25 (scope relative) (span (offset 1144) (line 38) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1144) (line 38) (column 32) (len 6)))))
    (reference r26 (scope relative) (span (offset 1226) (line 41) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1226) (line 41) (column 32) (len 6)))))
    (reference r27 (scope relative) (span (offset 1299) (line 44) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1299) (line 44) (column 32) (len 6)))))
    (reference r28 (scope relative) (span (offset 1373) (line 47) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1373) (line 47) (column 32) (len 6)))))
    (reference r29 (scope relative) (span (offset 1390) (line 48) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1390) (line 48) (column 10) (len 3)))))
    (reference r30 (scope relative) (span (offset 1412) (line 48) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1412) (line 48) (column 32) (len 6)))))
    (reference r31 (scope relative) (span (offset 1489) (line 51) (column 34) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1489) (line 51) (column 34) (len 6)))))
    (reference r32 (scope relative) (span (offset 1506) (line 52) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1506) (line 52) (column 10) (len 3)))))
    (reference r33 (scope relative) (span (offset 1528) (line 52) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1528) (line 52) (column 32) (len 6)))))
    (reference r34 (scope relative) (span (offset 1642) (line 56) (column 38) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1642) (line 56) (column 38) (len 6)))))
    (reference r35 (scope relative) (span (offset 1741) (line 60) (column 38) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1741) (line 60) (column 38) (len 6)))))
  )
  (root (package (name "PartPrefixOwningScopes") (body brace (metadata-def (name "Tag") (abstract false) (specializes none) (body semicolon)) (part-def (name "Engine") (modifiers) (body semicolon)) (part-def (name "Wheel") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "packageScope") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r1))) (declaration-name "packageTagged") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration-name "packageSnapshot") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-def (name "DefinitionScope") (modifiers) (body brace (part-usage (then false) (prefix (direction in) (derived true) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inDefinition") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r4))) (declaration-name "sameSyntax") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "usageScope") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion timeslice) (extensions)) (declaration-name "inUsage") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r7))) (declaration-name "sameSyntax") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (item-def (name "ItemScope") (modifiers) (individual false) (specializes none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inItemBody") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r10))) (declaration-name "sameSyntax") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (metadata-def (name "MetadataScope") (abstract false) (specializes none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inMetadataBody") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (connection-def (name "ConnectionScope") (modifiers) (role ordinary) (specializes none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inConnectionDef") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r14))) (declaration-name "sameSyntax") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "occurrenceScope") (short-name none) (target none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration-name "inOccurrenceBody") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r17))) (declaration-name "sameSyntax") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (use-case-def (name "UseCaseScope") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inUseCaseDef") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r20))) (declaration-name "sameSyntax") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (calc-def (name "CalcScope") (modifiers) (body brace (part-usage (then false) (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inCalcDef") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bareInCalcDef") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r24))) (declaration-name "sameSyntax") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (constraint-def (name "ConstraintScope") (modifiers) (specializes none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inConstraintDef") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (kerml-classifier (keyword struct) (abstract false) (name "KermlTypeScope") (specializes none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inKermlTypeBody") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (action-def (name "ActionScope") (modifiers) (specializes none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inActionDef") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r28)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r29))) (declaration-name "sameSyntax") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r30)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (action-usage (name "actionUsageScope") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inActionUsage") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r31)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r32))) (declaration-name "sameSyntax") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r33)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "PerformScope") (modifiers) (body brace (perform (declaration "inPerform") (action none) (typing none) (subsets none) (redefines none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "inPerformBody") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r34)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))) (part-def (name "VariantScope") (modifiers (variation (span (offset 1670) (line 59) (column 5) (len 9)))) (body brace (variant-usage (target none) (usage (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "variantMember") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r35)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon))) (body absent)))))))
)
~~~
