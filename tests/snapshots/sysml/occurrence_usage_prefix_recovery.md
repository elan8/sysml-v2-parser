# META
~~~sexpr
(snapshot (type recovery) (description "Recovery around the shared OccurrenceUsagePrefix. Malformed content sits immediately before every one of the prefix's FIRST tokens, so a starter table missing one of them shows up as a swallowed sibling; it also sits between two prefix components, and inside a prefixed member of each migrated family followed by several valid siblings. Invalid orderings, both mutually exclusive pairs authored together, a repeated portion kind, a prefix with no usage after it, and a malformed extension keyword each become one recovery node retaining the exact authored span -- never a valid unprefixed usage. Prefix words inside a quoted name, a string literal and both comment forms are lexical content, not members. A valid declaration follows every case."))
~~~
# SOURCE
~~~sysml
package OccurrencePrefixRecovery {
    part def MalformedBeforeEachStarter {
        %%%;
        in occurrence afterDirection;
        %%%;
        derived occurrence afterDerived;
        %%%;
        abstract occurrence afterAbstract;
        %%%;
        variation occurrence afterVariation;
        %%%;
        constant occurrence afterConstant;
        %%%;
        ref occurrence afterReference;
        %%%;
        individual afterIndividual;
        %%%;
        snapshot afterSnapshot;
        %%%;
        timeslice afterTimeslice;
        %%%;
        #Tag occurrence afterExtensionKeyword;
        %%%;
        ref individual snapshot satisfy AfterPrefixedSatisfy;
        occurrence lastSiblingSurvives;
    }
    part def MalformedBetweenPrefixComponents {
        in %%% derived occurrence between;
        occurrence validAfterBetween;
    }
    part def InvalidOrdering {
        ref abstract occurrence reordered;
        occurrence individual reordered2;
        occurrence validAfterInvalidOrdering;
    }
    part def DuplicateModifiers {
        abstract variation occurrence bothVariance;
        in out occurrence bothDirections;
        snapshot timeslice bothPortions;
        snapshot snapshot repeatedPortion;
        occurrence validAfterDuplicates;
    }
    part def MissingUsageAfterPrefix {
        in derived constant;
        occurrence validAfterMissingUsage;
    }
    part def MalformedExtensionKeyword {
        #Tag::;
        #;
        occurrence validAfterMalformedKeyword;
    }
    part def MalformedPrefixedFamilyMembers {
        derived ref individual snapshot occurrence %%%;
        occurrence firstSiblingSurvives;
        item secondSiblingSurvives;
        satisfy ThirdSiblingSurvives;
        in ref individual assert not satisfy %%%;
        occurrence fourthSiblingSurvives;
    }
    part def PrefixLikeWordsInLexicalContent {
        occurrence 'individual snapshot ref';
        attribute quoted = "ref individual snapshot occurrence notAMember;";
        // derived abstract occurrence commentedOut;
        /* individual timeslice occurrence alsoCommentedOut; */
        occurrence validAfterLexicalContent;
    }
    part def NestedBodies {
        ref individual occurrence outer {
            snapshot inner {
                occurrence deepest;
            }
        }
        occurrence validAfterNested;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "occurrence_usage_prefix_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 85) (line 3) (column 9) (len 13)) (message "unexpected token in part definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 85) (line 3) (column 9) (len 13)) (message "suppressed 18 cascading recovered diagnostics after earlier recovery errors"))
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 1454) (line 48) (column 9) (len 16)) (message "incomplete parser support for metadata syntax in part definition body"))
      (diagnostic (code "malformed_annotation_head") (severity error) (category parseerror) (span (offset 1470) (line 49) (column 9) (len 11)) (message "malformed metadata reference in part definition body"))
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 1580) (line 53) (column 9) (len 56)) (message "unexpected token in part definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 1580) (line 53) (column 9) (len 56)) (message "suppressed 1 cascading recovered diagnostic after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package OccurrencePrefixRecovery {
    part def MalformedBeforeEachStarter {
        %%%;
        in occurrence afterDirection;
        %%%;
        derived occurrence afterDerived;
        %%%;
        abstract occurrence afterAbstract;
        %%%;
        variation occurrence afterVariation;
        %%%;
        constant occurrence afterConstant;
        %%%;
        ref occurrence afterReference;
        %%%;
        individual afterIndividual;
        %%%;
        snapshot afterSnapshot;
        %%%;
        timeslice afterTimeslice;
        %%%;
        #Tag occurrence afterExtensionKeyword;
        %%%;
        ref individual snapshot satisfy AfterPrefixedSatisfy;
        occurrence lastSiblingSurvives;
    }
    part def MalformedBetweenPrefixComponents {
        in %%% derived occurrence between;
        occurrence validAfterBetween;
    }
    part def InvalidOrdering {
        ref abstract occurrence reordered;
        occurrence individual reordered2;
        occurrence validAfterInvalidOrdering;
    }
    part def DuplicateModifiers {
        abstract variation occurrence bothVariance;
        in out occurrence bothDirections;
        snapshot timeslice bothPortions;
        snapshot snapshot repeatedPortion;
        occurrence validAfterDuplicates;
    }
    part def MissingUsageAfterPrefix {
        in derived constant;
        occurrence validAfterMissingUsage;
    }
    part def MalformedExtensionKeyword {
        #Tag::;
        #;
        occurrence validAfterMalformedKeyword;
    }
    part def MalformedPrefixedFamilyMembers {
        derived ref individual snapshot occurrence %%%;
        occurrence firstSiblingSurvives;
        item secondSiblingSurvives;
        satisfy ThirdSiblingSurvives;
        in ref individual assert not satisfy %%%;
        occurrence fourthSiblingSurvives;
    }
    part def PrefixLikeWordsInLexicalContent {
        occurrence 'individual snapshot ref';
        attribute quoted = "ref individual snapshot occurrence notAMember;";
        occurrence validAfterLexicalContent;
    }
    part def NestedBodies {
        ref individual occurrence outer {
            snapshot inner {
                occurrence deepest;
            }
        }
        occurrence validAfterNested;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 567) (line 22) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 567) (line 22) (column 10) (len 3)))))
    (reference r1 (scope relative) (span (offset 658) (line 24) (column 41) (len 20)) (segments (segment 0 (token "AfterPrefixedSatisfy") (name "AfterPrefixedSatisfy") (separator none) (span (offset 658) (line 24) (column 41) (len 20)))))
    (reference r2 (scope relative) (span (offset 1721) (line 56) (column 17) (len 20)) (segments (segment 0 (token "ThirdSiblingSurvives") (name "ThirdSiblingSurvives") (separator none) (span (offset 1721) (line 56) (column 17) (len 20)))))
  )
  (root (package (name "OccurrencePrefixRecovery") (body brace (part-def (name "MalformedBeforeEachStarter") (body brace (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 85) (line 3) (column 9) (len 13))) (occurrence (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "afterDirection") (short-name none) (target none)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 136) (line 5) (column 9) (len 13))) (occurrence (prefix (direction none) (derived true) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "afterDerived") (short-name none) (target none)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 190) (line 7) (column 9) (len 13))) (occurrence (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "afterAbstract") (short-name none) (target none)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 246) (line 9) (column 9) (len 13))) (occurrence (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "afterVariation") (short-name none) (target none)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 304) (line 11) (column 9) (len 13))) (occurrence (prefix (direction none) (derived false) (variance none) (constant true) (reference false) (individual false) (portion none) (extensions)) (declaration "afterConstant") (short-name none) (target none)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 360) (line 13) (column 9) (len 13))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration "afterReference") (short-name none) (target none)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 412) (line 15) (column 9) (len 13))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "afterIndividual") (short-name none) (target none)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 461) (line 17) (column 9) (len 13))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "afterSnapshot") (short-name none) (target none)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 506) (line 19) (column 9) (len 13))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "afterTimeslice") (short-name none) (target none)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 553) (line 21) (column 9) (len 13))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r0))) (declaration "afterExtensionKeyword") (short-name none) (target none)) (malformed (code "recovered_part_def_body_element") (found "%%%;") (span (offset 613) (line 23) (column 9) (len 13))) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion snapshot) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r1))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "lastSiblingSurvives") (short-name none) (target none)))) (part-def (name "MalformedBetweenPrefixComponents") (body brace (malformed (code "recovered_part_def_body_element") (found "in %%% derived occurrence between;") (span (offset 782) (line 28) (column 9) (len 43))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "validAfterBetween") (short-name none) (target none)))) (part-def (name "InvalidOrdering") (body brace (malformed (code "recovered_part_def_body_element") (found "ref abstract occurrence reordered;") (span (offset 900) (line 32) (column 9) (len 43))) (malformed (code "recovered_part_def_body_element") (found "occurrence individual reordered2;") (span (offset 943) (line 33) (column 9) (len 42))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "validAfterInvalidOrdering") (short-name none) (target none)))) (part-def (name "DuplicateModifiers") (body brace (malformed (code "recovered_part_def_body_element") (found "abstract variation occurrence bothVariance;") (span (offset 1071) (line 37) (column 9) (len 52))) (malformed (code "recovered_part_def_body_element") (found "in out occurrence bothDirections;") (span (offset 1123) (line 38) (column 9) (len 42))) (malformed (code "recovered_part_def_body_element") (found "snapshot timeslice bothPortions;") (span (offset 1165) (line 39) (column 9) (len 41))) (malformed (code "recovered_part_def_body_element") (found "snapshot snapshot repeatedPortion;") (span (offset 1206) (line 40) (column 9) (len 43))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "validAfterDuplicates") (short-name none) (target none)))) (part-def (name "MissingUsageAfterPrefix") (body brace (malformed (code "recovered_part_def_body_element") (found "in derived constant;") (span (offset 1335) (line 44) (column 9) (len 29))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "validAfterMissingUsage") (short-name none) (target none)))) (part-def (name "MalformedExtensionKeyword") (body brace (malformed (code "unsupported_annotation_syntax") (found "#Tag::;") (span (offset 1454) (line 48) (column 9) (len 16))) (malformed (code "malformed_annotation_head") (found "#;") (span (offset 1470) (line 49) (column 9) (len 11))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "validAfterMalformedKeyword") (short-name none) (target none)))) (part-def (name "MalformedPrefixedFamilyMembers") (body brace (malformed (code "recovered_part_def_body_element") (found "derived ref individual snapshot occurrence %%%;") (span (offset 1580) (line 53) (column 9) (len 56))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "firstSiblingSurvives") (short-name none) (target none)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "secondSiblingSurvives")) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r2))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)) (malformed (code "recovered_part_def_body_element") (found "in ref individual assert not satisfy %%%;") (span (offset 1751) (line 57) (column 9) (len 50))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fourthSiblingSurvives") (short-name none) (target none)))) (part-def (name "PrefixLikeWordsInLexicalContent") (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "individual snapshot ref") (short-name none) (target none)) (attribute-usage (declaration-name "quoted") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1961) (line 62) (column 28) (len 48)) (string "ref individual snapshot occurrence notAMember;"))))) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "validAfterLexicalContent") (short-name none) (target none)))) (part-def (name "NestedBodies") (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration "outer") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "validAfterNested") (short-name none) (target none)))))))
)
~~~
