# META
~~~sexpr
(snapshot (type recovery) (description "Recovery around a prefixed port usage (planning/port-usage-prefix-matrix.md §10). Malformed content sits immediately before every one of FIRST(OccurrenceUsagePrefix), so a starter table missing one of them shows up as a swallowed sibling; it also sits between each pair of prefix slots. Invalid orderings, both mutually exclusive pairs authored together, a repeated portion kind, every repeated independent singleton, a complete prefix with no `port`, a `port` with no declaration, `port def` behind a usage-only prefix, and incomplete and malformed extension keywords each become one recovery node retaining the exact authored span -- never a valid unprefixed port usage. A malformed prefix precedes the named, anonymous and `:>>` declaration shapes in turn. Prefix words inside a quoted name, a string literal and both comment forms are lexical content, not members. A valid declaration follows every case, and one case is followed by three."))
~~~
# SOURCE
~~~sysml
package PortPrefixRecovery {
    metadata def Tag;
    port def PowerPort;
    part def MalformedBeforeEachStarter {
        %%%;
        in port afterDirectionIn : PowerPort;
        %%%;
        out port afterDirectionOut : PowerPort;
        %%%;
        inout port afterDirectionInOut : PowerPort;
        %%%;
        derived port afterDerived : PowerPort;
        %%%;
        abstract port afterAbstract : PowerPort;
        %%%;
        variation port afterVariation : PowerPort;
        %%%;
        constant port afterConstant : PowerPort;
        %%%;
        ref port afterReference : PowerPort;
        %%%;
        individual port afterIndividual : PowerPort;
        %%%;
        snapshot port afterSnapshot;
        %%%;
        timeslice port afterTimeslice;
        %%%;
        #Tag port afterExtensionKeyword : PowerPort;
        %%%;
        port afterKindKeyword : PowerPort;
        port lastSiblingSurvives : PowerPort;
    }
    part def MalformedBetweenPrefixSlots {
        in %%% derived port betweenDirectionAndDerived : PowerPort;
        derived %%% abstract port betweenDerivedAndVariance : PowerPort;
        abstract %%% constant port betweenVarianceAndConstant : PowerPort;
        constant %%% ref port betweenConstantAndRef : PowerPort;
        ref %%% individual port betweenRefAndIndividual : PowerPort;
        individual %%% snapshot port betweenIndividualAndPortion : PowerPort;
        snapshot %%% #Tag port betweenPortionAndExtension : PowerPort;
        port validAfterBetween : PowerPort;
    }
    part def InvalidOrdering {
        ref derived port reversedRefDerived : PowerPort;
        individual ref port reversedIndividualRef : PowerPort;
        snapshot individual port reversedPortionIndividual : PowerPort;
        port individual reversedKindFirst;
        port validAfterInvalidOrdering : PowerPort;
    }
    part def ExclusiveAlternatives {
        in out port twoDirections : PowerPort;
        abstract variation port abstractAndVariation : PowerPort;
        snapshot timeslice port twoPortionKinds : PowerPort;
        port validAfterExclusive : PowerPort;
    }
    part def RepeatedModifiers {
        abstract abstract port twoAbstracts : PowerPort;
        variation variation port twoVariations : PowerPort;
        snapshot snapshot port twoSnapshots : PowerPort;
        ref ref port twoRefs : PowerPort;
        individual individual port twoIndividuals : PowerPort;
        derived derived port twoDeriveds : PowerPort;
        constant constant port twoConstants : PowerPort;
        port validAfterRepeated : PowerPort;
    }
    part def IncompleteProduction {
        in derived ref;
        individual snapshot;
        port;
        ref port def RefusedDefinition;
        port validAfterIncomplete : PowerPort;
    }
    part def MalformedExtensionKeyword {
        # port incompleteExtension : PowerPort;
        #;
        #Tag:: port malformedRelative : PowerPort;
        #$:: port malformedAbsolute : PowerPort;
        port validAfterMalformedKeyword : PowerPort;
    }
    part def MalformedPrefixBeforeEachShape {
        ref derived port namedAfterFailure : PowerPort;
        port validAfterNamed : PowerPort;
        ref derived port : PowerPort;
        port validAfterAnonymous : PowerPort;
        ref derived port :>> validAfterNamed;
        port validAfterRedefinesOnly : PowerPort;
    }
    part def PrefixLikeWordsInLexicalContent {
        port 'in derived ref port' : PowerPort;
        attribute quoted = "ref individual snapshot port notAMember;";
        // derived abstract port commentedOut;
        /* individual timeslice port alsoCommentedOut; */
        port validAfterLexicalContent : PowerPort;
    }
    part def SeveralValidSiblingsAfterOneFailure {
        ref derived port malformedThenThree : PowerPort;
        port firstSurvivor : PowerPort;
        ref port secondSurvivor : PowerPort;
        #Tag port thirdSurvivor : PowerPort;
    }
    part def NestedBodies {
        ref individual port outer : PowerPort {
            snapshot port inner {
                port deepest : PowerPort;
            }
        }
        port validAfterNested : PowerPort;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "port_usage_prefix_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 125) (line 5) (column 9) (len 13)) (message "unexpected token in part definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 125) (line 5) (column 9) (len 13)) (message "suppressed 36 cascading recovered diagnostics after earlier recovery errors"))
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 2848) (line 74) (column 9) (len 48)) (message "incomplete parser support for metadata syntax in part definition body"))
      (diagnostic (code "malformed_annotation_head") (severity error) (category parseerror) (span (offset 2896) (line 75) (column 9) (len 11)) (message "malformed metadata reference in part definition body"))
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 2907) (line 76) (column 9) (len 51)) (message "incomplete parser support for metadata syntax in part definition body"))
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 2958) (line 77) (column 9) (len 49)) (message "incomplete parser support for metadata syntax in part definition body"))
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 3112) (line 81) (column 9) (len 56)) (message "unexpected token in part definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 3112) (line 81) (column 9) (len 56)) (message "suppressed 3 cascading recovered diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package PortPrefixRecovery {
    metadata def Tag;
    port def PowerPort;
    part def MalformedBeforeEachStarter {
        %%%;
        in port afterDirectionIn : PowerPort;
        %%%;
        out port afterDirectionOut : PowerPort;
        %%%;
        inout port afterDirectionInOut : PowerPort;
        %%%;
        derived port afterDerived : PowerPort;
        %%%;
        abstract port afterAbstract : PowerPort;
        %%%;
        variation port afterVariation : PowerPort;
        %%%;
        constant port afterConstant : PowerPort;
        %%%;
        ref port afterReference : PowerPort;
        %%%;
        individual port afterIndividual : PowerPort;
        %%%;
        snapshot port afterSnapshot;
        %%%;
        timeslice port afterTimeslice;
        %%%;
        #Tag port afterExtensionKeyword : PowerPort;
        %%%;
        port afterKindKeyword : PowerPort;
        port lastSiblingSurvives : PowerPort;
    }
    part def MalformedBetweenPrefixSlots {
        in %%% derived port betweenDirectionAndDerived : PowerPort;
        derived %%% abstract port betweenDerivedAndVariance : PowerPort;
        abstract %%% constant port betweenVarianceAndConstant : PowerPort;
        constant %%% ref port betweenConstantAndRef : PowerPort;
        ref %%% individual port betweenRefAndIndividual : PowerPort;
        individual %%% snapshot port betweenIndividualAndPortion : PowerPort;
        snapshot %%% #Tag port betweenPortionAndExtension : PowerPort;
        port validAfterBetween : PowerPort;
    }
    part def InvalidOrdering {
        ref derived port reversedRefDerived : PowerPort;
        individual ref port reversedIndividualRef : PowerPort;
        snapshot individual port reversedPortionIndividual : PowerPort;
        port individual reversedKindFirst;
        port validAfterInvalidOrdering : PowerPort;
    }
    part def ExclusiveAlternatives {
        in out port twoDirections : PowerPort;
        abstract variation port abstractAndVariation : PowerPort;
        snapshot timeslice port twoPortionKinds : PowerPort;
        port validAfterExclusive : PowerPort;
    }
    part def RepeatedModifiers {
        abstract abstract port twoAbstracts : PowerPort;
        variation variation port twoVariations : PowerPort;
        snapshot snapshot port twoSnapshots : PowerPort;
        ref ref port twoRefs : PowerPort;
        individual individual port twoIndividuals : PowerPort;
        derived derived port twoDeriveds : PowerPort;
        constant constant port twoConstants : PowerPort;
        port validAfterRepeated : PowerPort;
    }
    part def IncompleteProduction {
        in derived ref;
        individual snapshot;
        port;
        ref port def RefusedDefinition;
        port validAfterIncomplete : PowerPort;
    }
    part def MalformedExtensionKeyword {
        # port incompleteExtension : PowerPort;
        #;
        #Tag:: port malformedRelative : PowerPort;
        #$:: port malformedAbsolute : PowerPort;
        port validAfterMalformedKeyword : PowerPort;
    }
    part def MalformedPrefixBeforeEachShape {
        ref derived port namedAfterFailure : PowerPort;
        port validAfterNamed : PowerPort;
        ref derived port : PowerPort;
        port validAfterAnonymous : PowerPort;
        ref derived port :>> validAfterNamed;
        port validAfterRedefinesOnly : PowerPort;
    }
    part def PrefixLikeWordsInLexicalContent {
        port 'in derived ref port' : PowerPort;
        attribute quoted = "ref individual snapshot port notAMember;";
        port validAfterLexicalContent : PowerPort;
    }
    part def SeveralValidSiblingsAfterOneFailure {
        ref derived port malformedThenThree : PowerPort;
        port firstSurvivor : PowerPort;
        ref port secondSurvivor : PowerPort;
        #Tag port thirdSurvivor : PowerPort;
    }
    part def NestedBodies {
        ref individual port outer : PowerPort {
            snapshot port inner {
                port deepest : PowerPort;
            }
        }
        port validAfterNested : PowerPort;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 165) (line 6) (column 36) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 165) (line 6) (column 36) (len 9)))))
    (reference r1 (scope relative) (span (offset 226) (line 8) (column 38) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 226) (line 8) (column 38) (len 9)))))
    (reference r2 (scope relative) (span (offset 291) (line 10) (column 42) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 291) (line 10) (column 42) (len 9)))))
    (reference r3 (scope relative) (span (offset 351) (line 12) (column 37) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 351) (line 12) (column 37) (len 9)))))
    (reference r4 (scope relative) (span (offset 413) (line 14) (column 39) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 413) (line 14) (column 39) (len 9)))))
    (reference r5 (scope relative) (span (offset 477) (line 16) (column 41) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 477) (line 16) (column 41) (len 9)))))
    (reference r6 (scope relative) (span (offset 539) (line 18) (column 39) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 539) (line 18) (column 39) (len 9)))))
    (reference r7 (scope relative) (span (offset 597) (line 20) (column 35) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 597) (line 20) (column 35) (len 9)))))
    (reference r8 (scope relative) (span (offset 663) (line 22) (column 43) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 663) (line 22) (column 43) (len 9)))))
    (reference r9 (scope relative) (span (offset 798) (line 28) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 798) (line 28) (column 10) (len 3)))))
    (reference r10 (scope relative) (span (offset 831) (line 28) (column 43) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 831) (line 28) (column 43) (len 9)))))
    (reference r11 (scope relative) (span (offset 887) (line 30) (column 33) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 887) (line 30) (column 33) (len 9)))))
    (reference r12 (scope relative) (span (offset 933) (line 31) (column 36) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 933) (line 31) (column 36) (len 9)))))
    (reference r13 (scope relative) (span (offset 1525) (line 41) (column 34) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 1525) (line 41) (column 34) (len 9)))))
    (reference r14 (scope relative) (span (offset 1849) (line 48) (column 42) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 1849) (line 48) (column 42) (len 9)))))
    (reference r15 (scope relative) (span (offset 2112) (line 54) (column 36) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 2112) (line 54) (column 36) (len 9)))))
    (reference r16 (scope relative) (span (offset 2586) (line 64) (column 35) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 2586) (line 64) (column 35) (len 9)))))
    (reference r17 (scope relative) (span (offset 2782) (line 71) (column 37) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 2782) (line 71) (column 37) (len 9)))))
    (reference r18 (scope relative) (span (offset 3041) (line 78) (column 43) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 3041) (line 78) (column 43) (len 9)))))
    (reference r19 (scope relative) (span (offset 3191) (line 82) (column 32) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 3191) (line 82) (column 32) (len 9)))))
    (reference r20 (scope relative) (span (offset 3275) (line 84) (column 36) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 3275) (line 84) (column 36) (len 9)))))
    (reference r21 (scope relative) (span (offset 3371) (line 86) (column 40) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 3371) (line 86) (column 40) (len 9)))))
    (reference r22 (scope relative) (span (offset 3472) (line 89) (column 38) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 3472) (line 89) (column 38) (len 9)))))
    (reference r23 (scope relative) (span (offset 3699) (line 93) (column 41) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 3699) (line 93) (column 41) (len 9)))))
    (reference r24 (scope relative) (span (offset 3853) (line 97) (column 30) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 3853) (line 97) (column 30) (len 9)))))
    (reference r25 (scope relative) (span (offset 3898) (line 98) (column 35) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 3898) (line 98) (column 35) (len 9)))))
    (reference r26 (scope relative) (span (offset 3918) (line 99) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 3918) (line 99) (column 10) (len 3)))))
    (reference r27 (scope relative) (span (offset 3943) (line 99) (column 35) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 3943) (line 99) (column 35) (len 9)))))
    (reference r28 (scope relative) (span (offset 4024) (line 102) (column 37) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 4024) (line 102) (column 37) (len 9)))))
    (reference r29 (scope relative) (span (offset 4101) (line 104) (column 32) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 4101) (line 104) (column 32) (len 9)))))
    (reference r30 (scope relative) (span (offset 4168) (line 107) (column 33) (len 9)) (segments (segment 0 (token "PowerPort") (name "PowerPort") (separator none) (span (offset 4168) (line 107) (column 33) (len 9)))))
  )
  (root (package (name "PortPrefixRecovery") (body brace (metadata-def (name "Tag") (abstract false) (specializes none) (body semicolon)) (port-def (name "PowerPort") (specializes none) (body semicolon)) (part-def (name "MalformedBeforeEachStarter") (body brace (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 125) (line 5) (column 9) (len 13))) (port-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterDirectionIn") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 184) (line 7) (column 9) (len 13))) (port-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterDirectionOut") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 245) (line 9) (column 9) (len 13))) (port-usage (prefix (direction inout) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterDirectionInOut") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 310) (line 11) (column 9) (len 13))) (port-usage (prefix (direction none) (derived true) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterDerived") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 370) (line 13) (column 9) (len 13))) (port-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterAbstract") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 432) (line 15) (column 9) (len 13))) (port-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterVariation") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 496) (line 17) (column 9) (len 13))) (port-usage (prefix (direction none) (derived false) (variance none) (constant true) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterConstant") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 558) (line 19) (column 9) (len 13))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "afterReference") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 616) (line 21) (column 9) (len 13))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration-name "afterIndividual") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 682) (line 23) (column 9) (len 13))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration-name "afterSnapshot") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 732) (line 25) (column 9) (len 13))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration-name "afterTimeslice") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 784) (line 27) (column 9) (len 13))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r9))) (declaration-name "afterExtensionKeyword") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 850) (line 29) (column 9) (len 13))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "afterKindKeyword") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lastSiblingSurvives") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "MalformedBetweenPrefixSlots") (body brace (malformed (code "recovered_part_def_body_element") (found "in %%% derived port betweenDirectionAndDerived : PowerPort;") (span (offset 1001) (line 34) (column 9) (len 68))) (malformed (code "recovered_part_def_body_element") (found "derived %%% abstract port betweenDerivedAndVariance : PowerP") (span (offset 1069) (line 35) (column 9) (len 73))) (malformed (code "recovered_part_def_body_element") (found "abstract %%% constant port betweenVarianceAndConstant : Powe") (span (offset 1142) (line 36) (column 9) (len 75))) (malformed (code "recovered_part_def_body_element") (found "constant %%% ref port betweenConstantAndRef : PowerPort;") (span (offset 1217) (line 37) (column 9) (len 65))) (malformed (code "recovered_part_def_body_element") (found "ref %%% individual port betweenRefAndIndividual : PowerPort;") (span (offset 1282) (line 38) (column 9) (len 69))) (malformed (code "recovered_part_def_body_element") (found "individual %%% snapshot port betweenIndividualAndPortion : P") (span (offset 1351) (line 39) (column 9) (len 78))) (malformed (code "recovered_part_def_body_element") (found "snapshot %%% #Tag port betweenPortionAndExtension : PowerPor") (span (offset 1429) (line 40) (column 9) (len 71))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterBetween") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "InvalidOrdering") (body brace (malformed (code "recovered_part_def_body_element") (found "ref derived port reversedRefDerived : PowerPort;") (span (offset 1581) (line 44) (column 9) (len 57))) (malformed (code "recovered_part_def_body_element") (found "individual ref port reversedIndividualRef : PowerPort;") (span (offset 1638) (line 45) (column 9) (len 63))) (malformed (code "recovered_part_def_body_element") (found "snapshot individual port reversedPortionIndividual : PowerPo") (span (offset 1701) (line 46) (column 9) (len 72))) (malformed (code "recovered_part_def_body_element") (found "port individual reversedKindFirst;") (span (offset 1773) (line 47) (column 9) (len 43))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterInvalidOrdering") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "ExclusiveAlternatives") (body brace (malformed (code "recovered_part_def_body_element") (found "in out port twoDirections : PowerPort;") (span (offset 1911) (line 51) (column 9) (len 47))) (malformed (code "recovered_part_def_body_element") (found "abstract variation port abstractAndVariation : PowerPort;") (span (offset 1958) (line 52) (column 9) (len 66))) (malformed (code "recovered_part_def_body_element") (found "snapshot timeslice port twoPortionKinds : PowerPort;") (span (offset 2024) (line 53) (column 9) (len 61))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterExclusive") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "RepeatedModifiers") (body brace (malformed (code "recovered_part_def_body_element") (found "abstract abstract port twoAbstracts : PowerPort;") (span (offset 2170) (line 57) (column 9) (len 57))) (malformed (code "recovered_part_def_body_element") (found "variation variation port twoVariations : PowerPort;") (span (offset 2227) (line 58) (column 9) (len 60))) (malformed (code "recovered_part_def_body_element") (found "snapshot snapshot port twoSnapshots : PowerPort;") (span (offset 2287) (line 59) (column 9) (len 57))) (malformed (code "recovered_part_def_body_element") (found "ref ref port twoRefs : PowerPort;") (span (offset 2344) (line 60) (column 9) (len 42))) (malformed (code "recovered_part_def_body_element") (found "individual individual port twoIndividuals : PowerPort;") (span (offset 2386) (line 61) (column 9) (len 63))) (malformed (code "recovered_part_def_body_element") (found "derived derived port twoDeriveds : PowerPort;") (span (offset 2449) (line 62) (column 9) (len 54))) (malformed (code "recovered_part_def_body_element") (found "constant constant port twoConstants : PowerPort;") (span (offset 2503) (line 63) (column 9) (len 57))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterRepeated") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "IncompleteProduction") (body brace (malformed (code "recovered_part_def_body_element") (found "in derived ref;") (span (offset 2647) (line 67) (column 9) (len 24))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion snapshot) (extensions)) (declaration "") (short-name none) (target none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "port;") (span (offset 2700) (line 69) (column 9) (len 14))) (malformed (code "recovered_part_def_body_element") (found "ref port def RefusedDefinition;") (span (offset 2714) (line 70) (column 9) (len 40))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterIncomplete") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "MalformedExtensionKeyword") (body brace (malformed (code "unsupported_annotation_syntax") (found "# port incompleteExtension : PowerPort;") (span (offset 2848) (line 74) (column 9) (len 48))) (malformed (code "malformed_annotation_head") (found "#;") (span (offset 2896) (line 75) (column 9) (len 11))) (malformed (code "unsupported_annotation_syntax") (found "#Tag:: port malformedRelative : PowerPort;") (span (offset 2907) (line 76) (column 9) (len 51))) (malformed (code "unsupported_annotation_syntax") (found "#$:: port malformedAbsolute : PowerPort;") (span (offset 2958) (line 77) (column 9) (len 49))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterMalformedKeyword") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "MalformedPrefixBeforeEachShape") (body brace (malformed (code "recovered_part_def_body_element") (found "ref derived port namedAfterFailure : PowerPort;") (span (offset 3112) (line 81) (column 9) (len 56))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterNamed") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "ref derived port : PowerPort;") (span (offset 3210) (line 83) (column 9) (len 38))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterAnonymous") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "ref derived port :>> validAfterNamed;") (span (offset 3294) (line 85) (column 9) (len 46))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterRedefinesOnly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "PrefixLikeWordsInLexicalContent") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "in derived ref port") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "quoted") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3510) (line 90) (column 28) (len 42)) (string "ref individual snapshot port notAMember;"))))) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterLexicalContent") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "SeveralValidSiblingsAfterOneFailure") (body brace (malformed (code "recovered_part_def_body_element") (found "ref derived port malformedThenThree : PowerPort;") (span (offset 3775) (line 96) (column 9) (len 57))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "firstSurvivor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "secondSurvivor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r26))) (declaration-name "thirdSurvivor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "NestedBodies") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration-name "outer") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r28)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration-name "inner") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "deepest") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "validAfterNested") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r30)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
