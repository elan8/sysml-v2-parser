# META
~~~sexpr
(snapshot (type semantic) (description "KerML classifier declarations cover metaclass/datatype/assoc/struct/class families with all-sufficiency, compound assoc struct, disjoint-from/unions/intersects clauses, feature members with prefixes and inverse-of, kinded step/expr features, and named and anonymous invariants."))
~~~
# SOURCE
~~~sysml
standard library package Occurrences {
    metaclass AnnotatingElement specializes Element {
        derived var feature annotatedElement : Element[1..*] ordered redefines annotatedElement;
        member feature 'in' : FeatureDirectionKind[1];
    }
    datatype UnitBoundedReal :> Real {
        inv unitBound { -1.0 <= that & that <= 1.0 }
    }
    assoc all HappensBefore specializes HappensLink, Without {
        end feature source: Occurrence[1] subsets participant;
        feature all spaceShotOf: Occurrence[0..*] subsets spaceSliceOf inverse of spaceShots;
        inv { isClosed == true }
    }
    assoc struct LinkObject specializes Link, Object intersects Link, Object {
        doc /* A LinkObject is both a Link and an Object. */
    }
    abstract class Occurrence specializes Anything disjoint from DataValue {
        portion feature redefines spaceBoundary [1];
    }
    abstract step performances: Performance[0..*] nonunique subsets occurrences {
        doc /* performances is the base feature of all performances. */
    }
    abstract expr evaluations: Evaluation[0..*] nonunique subsets performances;
    private struct UniversalClockLife[1] :> Clock, Life {
        feature universalClock : UniversalClockLife[1];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "classifier_declarations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Occurrences {
    metaclass AnnotatingElement specializes Element {
        derived var feature annotatedElement : Element[1..*] ordered redefines annotatedElement;
        member feature 'in' : FeatureDirectionKind[1];
    }
    datatype UnitBoundedReal :> Real {
        inv unitBound {
            -1.0 <= that & that <= 1.0;
        }
    }
    assoc all HappensBefore specializes HappensLink, Without {
        end feature source : Occurrence[1] subsets participant;
        feature all spaceShotOf : Occurrence[0..*] subsets spaceSliceOf inverse of spaceShots;
        inv {
            isClosed == true;
        }
    }
    assoc struct LinkObject specializes Link, Object intersects Link, Object {
        doc
        /* A LinkObject is both a Link and an Object. */
    }
    abstract class Occurrence specializes Anything disjoint from DataValue {
        portion feature[1] redefines spaceBoundary;
    }
    abstract step performances : Performance[0..*] nonunique subsets occurrences {
        doc
        /* performances is the base feature of all performances. */
    }
    abstract expr evaluations : Evaluation[0..*] nonunique subsets performances;
    private struct UniversalClockLife[1] :> Clock, Life {
        feature universalClock : UniversalClockLife[1];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 83) (line 2) (column 45) (len 7)) (segments (segment 0 (token "Element") (name "Element") (separator none) (span (offset 83) (line 2) (column 45) (len 7)))))
    (reference r1 (scope relative) (span (offset 140) (line 3) (column 48) (len 7)) (segments (segment 0 (token "Element") (name "Element") (separator none) (span (offset 140) (line 3) (column 48) (len 7)))))
    (reference r2 (scope relative) (span (offset 172) (line 3) (column 80) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 172) (line 3) (column 80) (len 16)))))
    (reference r3 (scope relative) (span (offset 220) (line 4) (column 31) (len 20)) (segments (segment 0 (token "FeatureDirectionKind") (name "FeatureDirectionKind") (separator none) (span (offset 220) (line 4) (column 31) (len 20)))))
    (reference r4 (scope relative) (span (offset 283) (line 6) (column 33) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 283) (line 6) (column 33) (len 4)))))
    (reference r5 (scope relative) (span (offset 389) (line 9) (column 41) (len 11)) (segments (segment 0 (token "HappensLink") (name "HappensLink") (separator none) (span (offset 389) (line 9) (column 41) (len 11)))))
    (reference r6 (scope relative) (span (offset 402) (line 9) (column 54) (len 7)) (segments (segment 0 (token "Without") (name "Without") (separator none) (span (offset 402) (line 9) (column 54) (len 7)))))
    (reference r7 (scope relative) (span (offset 440) (line 10) (column 29) (len 10)) (segments (segment 0 (token "Occurrence") (name "Occurrence") (separator none) (span (offset 440) (line 10) (column 29) (len 10)))))
    (reference r8 (scope relative) (span (offset 462) (line 10) (column 51) (len 11)) (segments (segment 0 (token "participant") (name "participant") (separator none) (span (offset 462) (line 10) (column 51) (len 11)))))
    (reference r9 (scope relative) (span (offset 508) (line 11) (column 34) (len 10)) (segments (segment 0 (token "Occurrence") (name "Occurrence") (separator none) (span (offset 508) (line 11) (column 34) (len 10)))))
    (reference r10 (scope relative) (span (offset 533) (line 11) (column 59) (len 12)) (segments (segment 0 (token "spaceSliceOf") (name "spaceSliceOf") (separator none) (span (offset 533) (line 11) (column 59) (len 12)))))
    (reference r11 (scope relative) (span (offset 557) (line 11) (column 83) (len 10)) (segments (segment 0 (token "spaceShots") (name "spaceShots") (separator none) (span (offset 557) (line 11) (column 83) (len 10)))))
    (reference r12 (scope relative) (span (offset 648) (line 14) (column 41) (len 4)) (segments (segment 0 (token "Link") (name "Link") (separator none) (span (offset 648) (line 14) (column 41) (len 4)))))
    (reference r13 (scope relative) (span (offset 654) (line 14) (column 47) (len 6)) (segments (segment 0 (token "Object") (name "Object") (separator none) (span (offset 654) (line 14) (column 47) (len 6)))))
    (reference r14 (scope relative) (span (offset 796) (line 17) (column 43) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 796) (line 17) (column 43) (len 8)))))
    (reference r15 (scope relative) (span (offset 865) (line 18) (column 35) (len 13)) (segments (segment 0 (token "spaceBoundary") (name "spaceBoundary") (separator none) (span (offset 865) (line 18) (column 35) (len 13)))))
    (reference r16 (scope relative) (span (offset 922) (line 20) (column 33) (len 11)) (segments (segment 0 (token "Performance") (name "Performance") (separator none) (span (offset 922) (line 20) (column 33) (len 11)))))
    (reference r17 (scope relative) (span (offset 958) (line 20) (column 69) (len 11)) (segments (segment 0 (token "occurrences") (name "occurrences") (separator none) (span (offset 958) (line 20) (column 69) (len 11)))))
    (reference r18 (scope relative) (span (offset 1081) (line 23) (column 32) (len 10)) (segments (segment 0 (token "Evaluation") (name "Evaluation") (separator none) (span (offset 1081) (line 23) (column 32) (len 10)))))
    (reference r19 (scope relative) (span (offset 1116) (line 23) (column 67) (len 12)) (segments (segment 0 (token "performances") (name "performances") (separator none) (span (offset 1116) (line 23) (column 67) (len 12)))))
    (reference r20 (scope relative) (span (offset 1174) (line 24) (column 45) (len 5)) (segments (segment 0 (token "Clock") (name "Clock") (separator none) (span (offset 1174) (line 24) (column 45) (len 5)))))
    (reference r21 (scope relative) (span (offset 1181) (line 24) (column 52) (len 4)) (segments (segment 0 (token "Life") (name "Life") (separator none) (span (offset 1181) (line 24) (column 52) (len 4)))))
    (reference r22 (scope relative) (span (offset 1221) (line 25) (column 34) (len 18)) (segments (segment 0 (token "UniversalClockLife") (name "UniversalClockLife") (separator none) (span (offset 1221) (line 25) (column 34) (len 18)))))
  )
  (root (library-package (name "Occurrences") (standard true) (body brace (kerml-classifier (keyword metaclass) (abstract false) (name "AnnotatingElement") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body brace (kerml-feature (prefix (head basic) (direction none) (derived true) (abstract false) (portion none) (variability var) (metadata)) (kind feature) (member false) (all false) (name "annotatedElement") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity (lower (expression (span (offset 148) (line 3) (column 56) (len 1)) (integer 1))) (upper unbounded)) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member true) (all false) (name "in") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity (lower (expression (span (offset 241) (line 4) (column 52) (len 1)) (integer 1))) (upper (expression (span (offset 241) (line 4) (column 52) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword datatype) (abstract false) (name "UnitBoundedReal") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r4)))) (body brace (invariant))) (kerml-classifier (keyword assoc) (abstract false) (name "HappensBefore") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5) (ref r6)))) (body brace (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind feature) (member false) (all false) (name "source") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity (lower (expression (span (offset 451) (line 10) (column 40) (len 1)) (integer 1))) (upper (expression (span (offset 451) (line 10) (column 40) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r8)))) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all true) (name "spaceShotOf") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity (lower (expression (span (offset 519) (line 11) (column 45) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r10)))) (redefines none) (references none) (crosses none) (relationships (inverse-of (ref r11))) (value none) (body semicolon)) (invariant))) (kerml-classifier (keyword assoc struct) (abstract false) (name "LinkObject") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r12) (ref r13)))) (body brace (doc (name none) (locale none) (body (span (offset 701) (line 15) (column 15) (len 44)) (normalized "A LinkObject is both a Link and an Object. "))))) (kerml-classifier (keyword class) (abstract true) (name "Occurrence") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r14)))) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion portion) (variability none) (metadata)) (kind feature) (member false) (all false) (name none) (typing none) (multiplicity (lower (expression (span (offset 880) (line 18) (column 50) (len 1)) (integer 1))) (upper (expression (span (offset 880) (line 18) (column 50) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract true) (portion none) (variability none) (metadata)) (kind step) (member false) (all false) (name "performances") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity (lower (expression (span (offset 934) (line 20) (column 45) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (subsets (relationship (kind subsets) (implied false) (targets (ref r17)))) (redefines none) (references none) (crosses none) (relationships) (value none) (body brace (doc (name none) (locale none) (body (span (offset 986) (line 21) (column 15) (len 55)) (normalized "performances is the base feature of all performances. "))))) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract true) (portion none) (variability none) (metadata)) (kind expr) (member false) (all false) (name "evaluations") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity (lower (expression (span (offset 1092) (line 23) (column 43) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (subsets (relationship (kind subsets) (implied false) (targets (ref r19)))) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-classifier (keyword struct) (abstract false) (name "UniversalClockLife") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r20) (ref r21)))) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "universalClock") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity (lower (expression (span (offset 1240) (line 25) (column 53) (len 1)) (integer 1))) (upper (expression (span (offset 1240) (line 25) (column 53) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))))))
)
~~~
