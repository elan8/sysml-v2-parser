# META
~~~sexpr
(snapshot (type recovery) (description "Recovery around a prefixed part usage (planning/part-usage-prefix-matrix.md §10). Malformed content sits immediately before every one of FIRST(OccurrenceUsagePrefix), so a starter table missing one of them shows up as a swallowed sibling; it also sits between each pair of prefix slots. Invalid orderings, both mutually exclusive pairs authored together, a repeated portion kind, every repeated independent singleton, a complete prefix with no `part`, a `part` with no declaration, and incomplete and malformed extension keywords each become one recovery node retaining the exact authored span -- never a valid unprefixed part usage. A malformed prefix precedes the named, anonymous and `:>>` declaration shapes in turn. Prefix words inside a quoted name, a string literal and both comment forms are lexical content, not members. A valid declaration follows every case, and one case is followed by three."))
~~~
# SOURCE
~~~sysml
package PartPrefixRecovery {
    metadata def Tag;
    part def Engine;
    part def MalformedBeforeEachStarter {
        %%%;
        in part afterDirectionIn : Engine;
        %%%;
        out part afterDirectionOut : Engine;
        %%%;
        inout part afterDirectionInOut : Engine;
        %%%;
        derived part afterDerived : Engine;
        %%%;
        abstract part afterAbstract : Engine;
        %%%;
        variation part afterVariation : Engine;
        %%%;
        constant part afterConstant : Engine;
        %%%;
        ref part afterReference : Engine;
        %%%;
        individual part afterIndividual : Engine;
        %%%;
        snapshot part afterSnapshot;
        %%%;
        timeslice part afterTimeslice;
        %%%;
        #Tag part afterExtensionKeyword : Engine;
        %%%;
        part afterKindKeyword : Engine;
        part lastSiblingSurvives : Engine;
    }
    part def MalformedBetweenPrefixSlots {
        in %%% derived part betweenDirectionAndDerived : Engine;
        derived %%% abstract part betweenDerivedAndVariance : Engine;
        abstract %%% constant part betweenVarianceAndConstant : Engine;
        constant %%% ref part betweenConstantAndRef : Engine;
        ref %%% individual part betweenRefAndIndividual : Engine;
        individual %%% snapshot part betweenIndividualAndPortion : Engine;
        snapshot %%% #Tag part betweenPortionAndExtension : Engine;
        part validAfterBetween : Engine;
    }
    part def InvalidOrdering {
        ref derived part reversedRefDerived : Engine;
        individual ref part reversedIndividualRef : Engine;
        snapshot individual part reversedPortionIndividual : Engine;
        part individual reversedKindFirst;
        part validAfterInvalidOrdering : Engine;
    }
    part def ExclusiveAlternatives {
        in out part twoDirections : Engine;
        abstract variation part abstractAndVariation : Engine;
        snapshot timeslice part twoPortionKinds : Engine;
        part validAfterExclusive : Engine;
    }
    part def RepeatedModifiers {
        abstract abstract part twoAbstracts : Engine;
        variation variation part twoVariations : Engine;
        snapshot snapshot part twoSnapshots : Engine;
        ref ref part twoRefs : Engine;
        individual individual part twoIndividuals : Engine;
        derived derived part twoDeriveds : Engine;
        constant constant part twoConstants : Engine;
        part validAfterRepeated : Engine;
    }
    part def IncompleteProduction {
        in derived ref;
        individual snapshot;
        part;
        part validAfterIncomplete : Engine;
    }
    part def MalformedExtensionKeyword {
        # part incompleteExtension : Engine;
        #;
        #Tag:: part malformedRelative : Engine;
        #$:: part malformedAbsolute : Engine;
        part validAfterMalformedKeyword : Engine;
    }
    part def MalformedPrefixBeforeEachShape {
        ref derived part namedAfterFailure : Engine;
        part validAfterNamed : Engine;
        ref derived part : Engine;
        part validAfterAnonymous : Engine;
        ref derived part :>> validAfterNamed;
        part validAfterRedefinesOnly : Engine;
    }
    part def PrefixLikeWordsInLexicalContent {
        part 'in derived ref part' : Engine;
        attribute quoted = "ref individual snapshot part notAMember;";
        // derived abstract part commentedOut;
        /* individual timeslice part alsoCommentedOut; */
        part validAfterLexicalContent : Engine;
    }
    part def SeveralValidSiblingsAfterOneFailure {
        ref derived part malformedThenThree : Engine;
        part firstSurvivor : Engine;
        ref part secondSurvivor : Engine;
        #Tag part thirdSurvivor : Engine;
    }
    part def NestedBodies {
        ref individual part outer : Engine {
            snapshot part inner {
                part deepest : Engine;
            }
        }
        part validAfterNested : Engine;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "part_usage_prefix_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 122) (line 5) (column 9) (len 13)) (message "unexpected token in part definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 122) (line 5) (column 9) (len 13)) (message "suppressed 35 cascading recovered diagnostics after earlier recovery errors"))
      (diagnostic (code "malformed_annotation_head") (severity error) (category parseerror) (span (offset 2739) (line 74) (column 9) (len 11)) (message "malformed metadata reference in part definition body"))
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 2946) (line 80) (column 9) (len 53)) (message "unexpected token in part definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 2946) (line 80) (column 9) (len 53)) (message "suppressed 3 cascading recovered diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package PartPrefixRecovery {
    metadata def Tag;
    part def Engine;
    part def MalformedBeforeEachStarter {
        %%%;
        in part afterDirectionIn : Engine;
        %%%;
        out part afterDirectionOut : Engine;
        %%%;
        inout part afterDirectionInOut : Engine;
        %%%;
        derived part afterDerived : Engine;
        %%%;
        abstract part afterAbstract : Engine;
        %%%;
        variation part afterVariation : Engine;
        %%%;
        constant part afterConstant : Engine;
        %%%;
        ref part afterReference : Engine;
        %%%;
        individual part afterIndividual : Engine;
        %%%;
        snapshot part afterSnapshot;
        %%%;
        timeslice part afterTimeslice;
        %%%;
        #Tag part afterExtensionKeyword : Engine;
        %%%;
        part afterKindKeyword : Engine;
        part lastSiblingSurvives : Engine;
    }
    part def MalformedBetweenPrefixSlots {
        in %%% derived part betweenDirectionAndDerived : Engine;
        derived %%% abstract part betweenDerivedAndVariance : Engine;
        abstract %%% constant part betweenVarianceAndConstant : Engine;
        constant %%% ref part betweenConstantAndRef : Engine;
        ref %%% individual part betweenRefAndIndividual : Engine;
        individual %%% snapshot part betweenIndividualAndPortion : Engine;
        snapshot %%% #Tag part betweenPortionAndExtension : Engine;
        part validAfterBetween : Engine;
    }
    part def InvalidOrdering {
        ref derived part reversedRefDerived : Engine;
        individual ref part reversedIndividualRef : Engine;
        snapshot individual part reversedPortionIndividual : Engine;
        part individual reversedKindFirst;
        part validAfterInvalidOrdering : Engine;
    }
    part def ExclusiveAlternatives {
        in out part twoDirections : Engine;
        abstract variation part abstractAndVariation : Engine;
        snapshot timeslice part twoPortionKinds : Engine;
        part validAfterExclusive : Engine;
    }
    part def RepeatedModifiers {
        abstract abstract part twoAbstracts : Engine;
        variation variation part twoVariations : Engine;
        snapshot snapshot part twoSnapshots : Engine;
        ref ref part twoRefs : Engine;
        individual individual part twoIndividuals : Engine;
        derived derived part twoDeriveds : Engine;
        constant constant part twoConstants : Engine;
        part validAfterRepeated : Engine;
    }
    part def IncompleteProduction {
        in derived ref;
        individual snapshot;
        part;
        part validAfterIncomplete : Engine;
    }
    part def MalformedExtensionKeyword {
        #'part'
        incompleteExtension : Engine;
        #;
        #Tag::'part'
        malformedRelative : Engine;
        #$::'part'
        malformedAbsolute : Engine;
        part validAfterMalformedKeyword : Engine;
    }
    part def MalformedPrefixBeforeEachShape {
        ref derived part namedAfterFailure : Engine;
        part validAfterNamed : Engine;
        ref derived part : Engine;
        part validAfterAnonymous : Engine;
        ref derived part :>> validAfterNamed;
        part validAfterRedefinesOnly : Engine;
    }
    part def PrefixLikeWordsInLexicalContent {
        part 'in derived ref part' : Engine;
        attribute quoted = "ref individual snapshot part notAMember;";
        part validAfterLexicalContent : Engine;
    }
    part def SeveralValidSiblingsAfterOneFailure {
        ref derived part malformedThenThree : Engine;
        part firstSurvivor : Engine;
        ref part secondSurvivor : Engine;
        #Tag part thirdSurvivor : Engine;
    }
    part def NestedBodies {
        ref individual part outer : Engine {
            snapshot part inner {
                part deepest : Engine;
            }
        }
        part validAfterNested : Engine;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 162) (line 6) (column 36) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 162) (line 6) (column 36) (len 6)))))
    (reference r1 (scope relative) (span (offset 220) (line 8) (column 38) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 220) (line 8) (column 38) (len 6)))))
    (reference r2 (scope relative) (span (offset 282) (line 10) (column 42) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 282) (line 10) (column 42) (len 6)))))
    (reference r3 (scope relative) (span (offset 339) (line 12) (column 37) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 339) (line 12) (column 37) (len 6)))))
    (reference r4 (scope relative) (span (offset 398) (line 14) (column 39) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 398) (line 14) (column 39) (len 6)))))
    (reference r5 (scope relative) (span (offset 459) (line 16) (column 41) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 459) (line 16) (column 41) (len 6)))))
    (reference r6 (scope relative) (span (offset 518) (line 18) (column 39) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 518) (line 18) (column 39) (len 6)))))
    (reference r7 (scope relative) (span (offset 573) (line 20) (column 35) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 573) (line 20) (column 35) (len 6)))))
    (reference r8 (scope relative) (span (offset 636) (line 22) (column 43) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 636) (line 22) (column 43) (len 6)))))
    (reference r9 (scope relative) (span (offset 768) (line 28) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 768) (line 28) (column 10) (len 3)))))
    (reference r10 (scope relative) (span (offset 801) (line 28) (column 43) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 801) (line 28) (column 43) (len 6)))))
    (reference r11 (scope relative) (span (offset 854) (line 30) (column 33) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 854) (line 30) (column 33) (len 6)))))
    (reference r12 (scope relative) (span (offset 897) (line 31) (column 36) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 897) (line 31) (column 36) (len 6)))))
    (reference r13 (scope relative) (span (offset 1465) (line 41) (column 34) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1465) (line 41) (column 34) (len 6)))))
    (reference r14 (scope relative) (span (offset 1777) (line 48) (column 42) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1777) (line 48) (column 42) (len 6)))))
    (reference r15 (scope relative) (span (offset 2028) (line 54) (column 36) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 2028) (line 54) (column 36) (len 6)))))
    (reference r16 (scope relative) (span (offset 2478) (line 64) (column 35) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 2478) (line 64) (column 35) (len 6)))))
    (reference r17 (scope relative) (span (offset 2631) (line 70) (column 37) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 2631) (line 70) (column 37) (len 6)))))
    (reference r18 (scope relative) (span (offset 2696) (line 73) (column 11) (len 4)) (segments (segment 0 (token "part") (name "part") (separator none) (span (offset 2696) (line 73) (column 11) (len 4)))))
    (reference r19 (scope relative) (span (offset 2751) (line 75) (column 10) (len 10)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 2751) (line 75) (column 10) (len 3))) (segment 1 (token "part") (name "part") (separator colon-colon) (span (offset 2757) (line 75) (column 16) (len 4)))))
    (reference r20 (scope absolute) (span (offset 2799) (line 76) (column 10) (len 8)) (segments (segment 0 (token "part") (name "part") (separator none) (span (offset 2803) (line 76) (column 14) (len 4)))))
    (reference r21 (scope relative) (span (offset 2878) (line 77) (column 43) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 2878) (line 77) (column 43) (len 6)))))
    (reference r22 (scope relative) (span (offset 3022) (line 81) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 3022) (line 81) (column 32) (len 6)))))
    (reference r23 (scope relative) (span (offset 3100) (line 83) (column 36) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 3100) (line 83) (column 36) (len 6)))))
    (reference r24 (scope relative) (span (offset 3193) (line 85) (column 40) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 3193) (line 85) (column 40) (len 6)))))
    (reference r25 (scope relative) (span (offset 3291) (line 88) (column 38) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 3291) (line 88) (column 38) (len 6)))))
    (reference r26 (scope relative) (span (offset 3515) (line 92) (column 41) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 3515) (line 92) (column 41) (len 6)))))
    (reference r27 (scope relative) (span (offset 3663) (line 96) (column 30) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 3663) (line 96) (column 30) (len 6)))))
    (reference r28 (scope relative) (span (offset 3705) (line 97) (column 35) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 3705) (line 97) (column 35) (len 6)))))
    (reference r29 (scope relative) (span (offset 3722) (line 98) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 3722) (line 98) (column 10) (len 3)))))
    (reference r30 (scope relative) (span (offset 3747) (line 98) (column 35) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 3747) (line 98) (column 35) (len 6)))))
    (reference r31 (scope relative) (span (offset 3825) (line 101) (column 37) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 3825) (line 101) (column 37) (len 6)))))
    (reference r32 (scope relative) (span (offset 3899) (line 103) (column 32) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 3899) (line 103) (column 32) (len 6)))))
    (reference r33 (scope relative) (span (offset 3963) (line 106) (column 33) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 3963) (line 106) (column 33) (len 6)))))
  )
  (root (package (name "PartPrefixRecovery") (body brace (metadata-def) (part-def (name "Engine") (body semicolon)) (part-def (name "MalformedBeforeEachStarter") (body brace (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 122) (line 5) (column 9) (len 13))) (part-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterDirectionIn") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 178) (line 7) (column 9) (len 13))) (part-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterDirectionOut") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 236) (line 9) (column 9) (len 13))) (part-usage (prefix (direction inout) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterDirectionInOut") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 298) (line 11) (column 9) (len 13))) (part-usage (prefix (direction none) (derived true) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterDerived") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 355) (line 13) (column 9) (len 13))) (part-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterAbstract") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 414) (line 15) (column 9) (len 13))) (part-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterVariation") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 475) (line 17) (column 9) (len 13))) (part-usage (prefix (direction none) (derived false) (variance none) (constant true) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterConstant") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 534) (line 19) (column 9) (len 13))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "afterReference") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 589) (line 21) (column 9) (len 13))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration-name "afterIndividual") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 652) (line 23) (column 9) (len 13))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration-name "afterSnapshot") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 702) (line 25) (column 9) (len 13))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration-name "afterTimeslice") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 754) (line 27) (column 9) (len 13))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r9))) (declaration-name "afterExtensionKeyword") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 817) (line 29) (column 9) (len 13))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterKindKeyword") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lastSiblingSurvives") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "MalformedBetweenPrefixSlots") (body brace (malformed (code "recovered_part_def_body_element") (found "in %%% derived part betweenDirectionAndDerived : Engine;") (span (offset 962) (line 34) (column 9) (len 65))) (malformed (code "recovered_part_def_body_element") (found "derived %%% abstract part betweenDerivedAndVariance : Engine") (span (offset 1027) (line 35) (column 9) (len 70))) (malformed (code "recovered_part_def_body_element") (found "abstract %%% constant part betweenVarianceAndConstant : Engi") (span (offset 1097) (line 36) (column 9) (len 72))) (malformed (code "recovered_part_def_body_element") (found "constant %%% ref part betweenConstantAndRef : Engine;") (span (offset 1169) (line 37) (column 9) (len 62))) (malformed (code "recovered_part_def_body_element") (found "ref %%% individual part betweenRefAndIndividual : Engine;") (span (offset 1231) (line 38) (column 9) (len 66))) (malformed (code "recovered_part_def_body_element") (found "individual %%% snapshot part betweenIndividualAndPortion : E") (span (offset 1297) (line 39) (column 9) (len 75))) (malformed (code "recovered_part_def_body_element") (found "snapshot %%% #Tag part betweenPortionAndExtension : Engine;") (span (offset 1372) (line 40) (column 9) (len 68))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterBetween") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "InvalidOrdering") (body brace (malformed (code "recovered_part_def_body_element") (found "ref derived part reversedRefDerived : Engine;") (span (offset 1518) (line 44) (column 9) (len 54))) (malformed (code "recovered_part_def_body_element") (found "individual ref part reversedIndividualRef : Engine;") (span (offset 1572) (line 45) (column 9) (len 60))) (malformed (code "recovered_part_def_body_element") (found "snapshot individual part reversedPortionIndividual : Engine;") (span (offset 1632) (line 46) (column 9) (len 69))) (malformed (code "recovered_part_def_body_element") (found "part individual reversedKindFirst;") (span (offset 1701) (line 47) (column 9) (len 43))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterInvalidOrdering") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "ExclusiveAlternatives") (body brace (malformed (code "recovered_part_def_body_element") (found "in out part twoDirections : Engine;") (span (offset 1836) (line 51) (column 9) (len 44))) (malformed (code "recovered_part_def_body_element") (found "abstract variation part abstractAndVariation : Engine;") (span (offset 1880) (line 52) (column 9) (len 63))) (malformed (code "recovered_part_def_body_element") (found "snapshot timeslice part twoPortionKinds : Engine;") (span (offset 1943) (line 53) (column 9) (len 58))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterExclusive") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "RepeatedModifiers") (body brace (malformed (code "recovered_part_def_body_element") (found "abstract abstract part twoAbstracts : Engine;") (span (offset 2083) (line 57) (column 9) (len 54))) (malformed (code "recovered_part_def_body_element") (found "variation variation part twoVariations : Engine;") (span (offset 2137) (line 58) (column 9) (len 57))) (malformed (code "recovered_part_def_body_element") (found "snapshot snapshot part twoSnapshots : Engine;") (span (offset 2194) (line 59) (column 9) (len 54))) (malformed (code "recovered_part_def_body_element") (found "ref ref part twoRefs : Engine;") (span (offset 2248) (line 60) (column 9) (len 39))) (malformed (code "recovered_part_def_body_element") (found "individual individual part twoIndividuals : Engine;") (span (offset 2287) (line 61) (column 9) (len 60))) (malformed (code "recovered_part_def_body_element") (found "derived derived part twoDeriveds : Engine;") (span (offset 2347) (line 62) (column 9) (len 51))) (malformed (code "recovered_part_def_body_element") (found "constant constant part twoConstants : Engine;") (span (offset 2398) (line 63) (column 9) (len 54))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterRepeated") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "IncompleteProduction") (body brace (malformed (code "recovered_part_def_body_element") (found "in derived ref;") (span (offset 2536) (line 67) (column 9) (len 24))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion snapshot) (extensions)) (declaration "") (short-name none) (target none)) (malformed (code "recovered_part_def_body_element") (found "part;") (span (offset 2589) (line 69) (column 9) (len 14))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterIncomplete") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "MalformedExtensionKeyword") (body brace (metadata-keyword-usage (type (ref r18)) (body none)) (default-reference-usage) (malformed (code "malformed_annotation_head") (found "#;") (span (offset 2739) (line 74) (column 9) (len 11))) (metadata-keyword-usage (type (ref r19)) (body none)) (default-reference-usage) (metadata-keyword-usage (type (ref r20)) (body none)) (default-reference-usage) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterMalformedKeyword") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "MalformedPrefixBeforeEachShape") (body brace (malformed (code "recovered_part_def_body_element") (found "ref derived part namedAfterFailure : Engine;") (span (offset 2946) (line 80) (column 9) (len 53))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterNamed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "ref derived part : Engine;") (span (offset 3038) (line 82) (column 9) (len 35))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterAnonymous") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "ref derived part :>> validAfterNamed;") (span (offset 3116) (line 84) (column 9) (len 46))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterRedefinesOnly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "PrefixLikeWordsInLexicalContent") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "in derived ref part") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (attribute-usage (declaration-name "quoted") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3326) (line 89) (column 28) (len 42)) (string "ref individual snapshot part notAMember;"))))) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterLexicalContent") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "SeveralValidSiblingsAfterOneFailure") (body brace (malformed (code "recovered_part_def_body_element") (found "ref derived part malformedThenThree : Engine;") (span (offset 3588) (line 95) (column 9) (len 54))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "firstSurvivor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "secondSurvivor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r28)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r29))) (declaration-name "thirdSurvivor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r30)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "NestedBodies") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration-name "outer") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r31)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration-name "inner") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "deepest") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r32)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterNested") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r33)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))))))
)
~~~
