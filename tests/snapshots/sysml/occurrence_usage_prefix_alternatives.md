# META
~~~sexpr
(snapshot (type semantic) (description "Every slot of the shared OccurrenceUsagePrefix, on all three families that carry it. RefPrefix contributes a direction (in/out/inout, one slot), `derived`, the abstract/variation alternative and `constant`; BasicUsagePrefix adds `ref`; OccurrenceUsagePrefix adds `individual`, the snapshot/timeslice alternative and an ordered run of UsageExtensionKeywords. Each modifier appears alone, then in the full legal order and in materially different combinations, on an occurrence usage, an item usage and a satisfy usage. The two keyword-less spellings (IndividualUsage and PortionUsage) appear beside the `occurrence`-keyword one, and both body forms appear. `MemberPrefix` visibility belongs to the membership rather than to the prefix, so it is shown separately and before it."))
~~~
# SOURCE
~~~sysml
package OccurrencePrefixAlternatives {
    metadata def Tag;
    package Tags {
        metadata def 'safety critical';
    }
    part def NoPrefix {
        occurrence plain;
        occurrence braced {
        }
    }
    part def SingleModifier {
        in occurrence directedIn;
        out occurrence directedOut;
        inout occurrence directedInOut;
        derived occurrence isDerived;
        abstract occurrence isAbstract;
        variation occurrence isVariation;
        constant occurrence isConstant;
        ref occurrence isReference;
        individual occurrence isIndividual;
        snapshot isSnapshot;
        timeslice isTimeslice;
    }
    part def Combinations {
        in derived abstract constant ref individual snapshot occurrence everySlot;
        out variation ref timeslice occurrence mixed;
        inout constant individual occurrence directedIndividual;
        derived ref occurrence derivedReference;
        individual snapshot portionOfIndividual;
        ref individual bareIndividual;
        abstract ref individual timeslice bareTimeslice;
        individual occurrence withBody {
        }
        event occurrence anEvent;
        then timeslice aSuccessionPortion;
    }
    part def ExtensionKeywords {
        #Tag occurrence oneKeyword;
        #Tag #Tags::'safety critical' occurrence twoKeywords;
        #$::OccurrencePrefixAlternatives::Tag occurrence absoluteKeyword;
        abstract ref #Tag occurrence prefixedThenKeyword;
    }
    part def ItemFamily {
        in derived abstract constant ref individual snapshot item everySlotItem;
        #Tag item taggedItem;
        ref individual item :>> driver : NoPrefix;
    }
    part def SatisfyFamily {
        satisfy R1;
        ref satisfy R2;
        in derived abstract constant ref individual snapshot assert not satisfy R3 by p;
        #Tag satisfy R4 by p {
        }
        private satisfy R5;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "occurrence_usage_prefix_alternatives.md"
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
    (reference r0 (scope relative) (span (offset 1266) (line 38) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1266) (line 38) (column 10) (len 3)))))
    (reference r1 (scope relative) (span (offset 1302) (line 39) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1302) (line 39) (column 10) (len 3)))))
    (reference r2 (scope relative) (span (offset 1307) (line 39) (column 15) (len 23)) (segments (segment 0 (token "Tags") (name "Tags") (separator none) (span (offset 1307) (line 39) (column 15) (len 4))) (segment 1 (token "'safety critical'") (name "safety critical") (separator colon-colon) (span (offset 1313) (line 39) (column 21) (len 17)))))
    (reference r3 (scope absolute) (span (offset 1364) (line 40) (column 10) (len 36)) (segments (segment 0 (token "OccurrencePrefixAlternatives") (name "OccurrencePrefixAlternatives") (separator none) (span (offset 1367) (line 40) (column 13) (len 28))) (segment 1 (token "Tag") (name "Tag") (separator colon-colon) (span (offset 1397) (line 40) (column 43) (len 3)))))
    (reference r4 (scope relative) (span (offset 1451) (line 41) (column 23) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1451) (line 41) (column 23) (len 3)))))
    (reference r5 (scope relative) (span (offset 1609) (line 45) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1609) (line 45) (column 10) (len 3)))))
    (reference r6 (scope relative) (span (offset 1732) (line 49) (column 17) (len 2)) (segments (segment 0 (token "R1") (name "R1") (separator none) (span (offset 1732) (line 49) (column 17) (len 2)))))
    (reference r7 (scope relative) (span (offset 1756) (line 50) (column 21) (len 2)) (segments (segment 0 (token "R2") (name "R2") (separator none) (span (offset 1756) (line 50) (column 21) (len 2)))))
    (reference r8 (scope relative) (span (offset 1840) (line 51) (column 81) (len 2)) (segments (segment 0 (token "R3") (name "R3") (separator none) (span (offset 1840) (line 51) (column 81) (len 2)))))
    (reference r9 (scope relative) (span (offset 1846) (line 51) (column 87) (len 1)) (segments (segment 0 (token "p") (name "p") (separator none) (span (offset 1846) (line 51) (column 87) (len 1)))))
    (reference r10 (scope relative) (span (offset 1858) (line 52) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 1858) (line 52) (column 10) (len 3)))))
    (reference r11 (scope relative) (span (offset 1870) (line 52) (column 22) (len 2)) (segments (segment 0 (token "R4") (name "R4") (separator none) (span (offset 1870) (line 52) (column 22) (len 2)))))
    (reference r12 (scope relative) (span (offset 1876) (line 52) (column 28) (len 1)) (segments (segment 0 (token "p") (name "p") (separator none) (span (offset 1876) (line 52) (column 28) (len 1)))))
    (reference r13 (scope relative) (span (offset 1914) (line 54) (column 25) (len 2)) (segments (segment 0 (token "R5") (name "R5") (separator none) (span (offset 1914) (line 54) (column 25) (len 2)))))
  )
  (root (package (name "OccurrencePrefixAlternatives") (body brace (metadata-def) (package (name "Tags") (body brace (metadata-def))) (part-def (name "NoPrefix") (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "plain") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "braced") (short-name none) (target none)))) (part-def (name "SingleModifier") (body brace (occurrence (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "directedIn") (short-name none) (target none)) (occurrence (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "directedOut") (short-name none) (target none)) (occurrence (prefix (direction inout) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "directedInOut") (short-name none) (target none)) (occurrence (prefix (direction none) (derived true) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "isDerived") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "isAbstract") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "isVariation") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant true) (reference false) (individual false) (portion none) (extensions)) (declaration "isConstant") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration "isReference") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "isIndividual") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "isSnapshot") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "isTimeslice") (short-name none) (target none)))) (part-def (name "Combinations") (body brace (occurrence (prefix (direction in) (derived true) (variance abstract) (constant true) (reference true) (individual true) (portion snapshot) (extensions)) (declaration "everySlot") (short-name none) (target none)) (occurrence (prefix (direction out) (derived false) (variance variation) (constant false) (reference true) (individual false) (portion timeslice) (extensions)) (declaration "mixed") (short-name none) (target none)) (occurrence (prefix (direction inout) (derived false) (variance none) (constant true) (reference false) (individual true) (portion none) (extensions)) (declaration "directedIndividual") (short-name none) (target none)) (occurrence (prefix (direction none) (derived true) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration "derivedReference") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion snapshot) (extensions)) (declaration "portionOfIndividual") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration "bareIndividual") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance abstract) (constant false) (reference true) (individual true) (portion timeslice) (extensions)) (declaration "bareTimeslice") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "withBody") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "anEvent") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "aSuccessionPortion") (short-name none) (target none)))) (part-def (name "ExtensionKeywords") (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r0))) (declaration "oneKeyword") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r1) (ref r2))) (declaration "twoKeywords") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r3))) (declaration "absoluteKeyword") (short-name none) (target none)) (occurrence (prefix (direction none) (derived false) (variance abstract) (constant false) (reference true) (individual false) (portion none) (extensions (ref r4))) (declaration "prefixedThenKeyword") (short-name none) (target none)))) (part-def (name "ItemFamily") (body brace (item-usage (prefix (direction in) (derived true) (variance abstract) (constant true) (reference true) (individual true) (portion snapshot) (extensions)) (declaration "everySlotItem")) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r5))) (declaration "taggedItem")) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration "")))) (part-def (name "SatisfyFamily") (body brace (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r6))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r7))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)) (satisfy (prefix (direction in) (derived true) (variance abstract) (constant true) (reference true) (individual true) (portion snapshot) (extensions)) (visibility none) (assert true) (negated true) (requirement (reference (ref r8))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r9)) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r10))) (visibility none) (assert false) (negated false) (requirement (reference (ref r11))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r12)) (body brace)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility private) (assert false) (negated false) (requirement (reference (ref r13))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)))))))
)
~~~
