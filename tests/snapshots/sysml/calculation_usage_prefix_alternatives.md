# META
~~~sexpr
(snapshot (type semantic) (description "Every slot of the shared OccurrenceUsagePrefix on a CalculationUsage (CalculationUsage = OccurrenceUsagePrefix 'calc' ActionUsageDeclaration CalculationBody, SysML BNF 1388), and every authored type of its Typings (TypedBy ( ',' FeatureTyping )*). Each modifier appears alone, then in legal combinations, including `#Tag` extension keywords, in every scope that owns a calculation usage: package, calc def body, action def body, part def body and part usage body. A `#Tag`-prefixed calculation in a calc body stays one prefixed usage rather than a metadata member plus a calculation. MemberPrefix visibility belongs to the membership and precedes the prefix."))
~~~
# SOURCE
~~~sysml
package CalculationPrefixAlternatives {
    metadata def Tag;
    metadata def Other;
    calc def C1;
    calc def C2;
    calc typedTwice : C1, C2;
    individual calc individualCalc;
    snapshot calc snapshotCalc;
    timeslice calc timesliceCalc;
    abstract calc abstractCalc : C1;
    variation calc variationCalc;
    derived calc derivedCalc;
    constant calc constantCalc;
    ref calc referenceCalc;
    #Tag calc taggedCalc;
    private ref individual snapshot #Tag #Other calc combinedCalc : C1, C2;
    calc def Host {
        in calc directedIn : C1;
        individual calc individualMember;
        #Tag calc taggedMember;
        ref individual calc referenceMember : C1, C2;
        calc typedMember : C1, C2;
    }
    action def Act {
        individual calc individualAction;
        calc typedAction : C1, C2;
    }
    part def Holder {
        timeslice calc timesliceMember;
    }
    part holder {
        individual calc usageMember;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "calculation_usage_prefix_alternatives.md"
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
    (reference r0 (scope relative) (span (offset 142) (line 6) (column 23) (len 2)) (segments (segment 0 (token "C1") (name "C1") (separator none) (span (offset 142) (line 6) (column 23) (len 2)))))
    (reference r1 (scope relative) (span (offset 146) (line 6) (column 27) (len 2)) (segments (segment 0 (token "C2") (name "C2") (separator none) (span (offset 146) (line 6) (column 27) (len 2)))))
    (reference r2 (scope relative) (span (offset 285) (line 10) (column 34) (len 2)) (segments (segment 0 (token "C1") (name "C1") (separator none) (span (offset 285) (line 10) (column 34) (len 2)))))
    (reference r3 (scope relative) (span (offset 418) (line 15) (column 6) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 418) (line 15) (column 6) (len 3)))))
    (reference r4 (scope relative) (span (offset 476) (line 16) (column 38) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 476) (line 16) (column 38) (len 3)))))
    (reference r5 (scope relative) (span (offset 481) (line 16) (column 43) (len 5)) (segments (segment 0 (token "Other") (name "Other") (separator none) (span (offset 481) (line 16) (column 43) (len 5)))))
    (reference r6 (scope relative) (span (offset 507) (line 16) (column 69) (len 2)) (segments (segment 0 (token "C1") (name "C1") (separator none) (span (offset 507) (line 16) (column 69) (len 2)))))
    (reference r7 (scope relative) (span (offset 511) (line 16) (column 73) (len 2)) (segments (segment 0 (token "C2") (name "C2") (separator none) (span (offset 511) (line 16) (column 73) (len 2)))))
    (reference r8 (scope relative) (span (offset 564) (line 18) (column 30) (len 2)) (segments (segment 0 (token "C1") (name "C1") (separator none) (span (offset 564) (line 18) (column 30) (len 2)))))
    (reference r9 (scope relative) (span (offset 619) (line 20) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 619) (line 20) (column 10) (len 3)))))
    (reference r10 (scope relative) (span (offset 688) (line 21) (column 47) (len 2)) (segments (segment 0 (token "C1") (name "C1") (separator none) (span (offset 688) (line 21) (column 47) (len 2)))))
    (reference r11 (scope relative) (span (offset 692) (line 21) (column 51) (len 2)) (segments (segment 0 (token "C2") (name "C2") (separator none) (span (offset 692) (line 21) (column 51) (len 2)))))
    (reference r12 (scope relative) (span (offset 723) (line 22) (column 28) (len 2)) (segments (segment 0 (token "C1") (name "C1") (separator none) (span (offset 723) (line 22) (column 28) (len 2)))))
    (reference r13 (scope relative) (span (offset 727) (line 22) (column 32) (len 2)) (segments (segment 0 (token "C2") (name "C2") (separator none) (span (offset 727) (line 22) (column 32) (len 2)))))
    (reference r14 (scope relative) (span (offset 827) (line 26) (column 28) (len 2)) (segments (segment 0 (token "C1") (name "C1") (separator none) (span (offset 827) (line 26) (column 28) (len 2)))))
    (reference r15 (scope relative) (span (offset 831) (line 26) (column 32) (len 2)) (segments (segment 0 (token "C2") (name "C2") (separator none) (span (offset 831) (line 26) (column 32) (len 2)))))
  )
  (root (package (name "CalculationPrefixAlternatives") (body brace (metadata-def (name "Tag") (abstract false) (specializes none) (body semicolon)) (metadata-def (name "Other") (abstract false) (specializes none) (body semicolon)) (calc-def (name "C1") (modifiers) (body semicolon)) (calc-def (name "C2") (modifiers) (body semicolon)) (calc-usage (name "typedTwice") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0) (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "individualCalc") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "snapshotCalc") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "timesliceCalc") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "abstractCalc") (short-name none) (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "variationCalc") (short-name none) (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "derivedCalc") (short-name none) (prefix (direction none) (derived true) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "constantCalc") (short-name none) (prefix (direction none) (derived false) (variance none) (constant true) (reference false) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "referenceCalc") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "taggedCalc") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r3))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "combinedCalc") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion snapshot) (extensions (ref r4) (ref r5))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6) (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-def (name "Host") (modifiers) (body brace (calc-usage (name "directedIn") (short-name none) (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "individualMember") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "taggedMember") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r9))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "referenceMember") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10) (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "typedMember") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12) (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (action-def (name "Act") (modifiers) (specializes none) (body brace (calc-usage (name "individualAction") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (calc-usage (name "typedAction") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14) (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "Holder") (modifiers) (body brace (calc-usage))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "holder") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (calc-usage))))))
)
~~~
