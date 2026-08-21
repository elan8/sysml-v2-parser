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
        derived var feature annotatedElement : Element[1..*] ordered :>> annotatedElement;
        member feature 'in' : FeatureDirectionKind[1];
    }
    datatype UnitBoundedReal :> Real {
        inv unitBound {
            -1.0 <= that & that <= 1.0;
        }
    }
    assoc all HappensBefore specializes HappensLink, Without {
        end feature source : Occurrence[1] :> participant;
        feature all spaceShotOf : Occurrence[0..*] :> spaceSliceOf inverse of spaceShots;
        inv {
            isClosed == true;
        }
    }
    assoc struct LinkObject specializes Link, Object intersects Link, Object {
        doc
        /* A LinkObject is both a Link and an Object. */
    }
    abstract class Occurrence specializes Anything disjoint from DataValue {
        portion feature[1] :>> spaceBoundary;
    }
    abstract step performances : Performance[0..*] nonunique :> occurrences {
        doc
        /* performances is the base feature of all performances. */
    }
    abstract expr evaluations : Evaluation[0..*] nonunique :> performances;
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
    (reference r1 (scope relative) (span (offset 283) (line 6) (column 33) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 283) (line 6) (column 33) (len 4)))))
    (reference r2 (scope relative) (span (offset 389) (line 9) (column 41) (len 11)) (segments (segment 0 (token "HappensLink") (name "HappensLink") (separator none) (span (offset 389) (line 9) (column 41) (len 11)))))
    (reference r3 (scope relative) (span (offset 402) (line 9) (column 54) (len 7)) (segments (segment 0 (token "Without") (name "Without") (separator none) (span (offset 402) (line 9) (column 54) (len 7)))))
    (reference r4 (scope relative) (span (offset 557) (line 11) (column 83) (len 10)) (segments (segment 0 (token "spaceShots") (name "spaceShots") (separator none) (span (offset 557) (line 11) (column 83) (len 10)))))
    (reference r5 (scope relative) (span (offset 648) (line 14) (column 41) (len 4)) (segments (segment 0 (token "Link") (name "Link") (separator none) (span (offset 648) (line 14) (column 41) (len 4)))))
    (reference r6 (scope relative) (span (offset 654) (line 14) (column 47) (len 6)) (segments (segment 0 (token "Object") (name "Object") (separator none) (span (offset 654) (line 14) (column 47) (len 6)))))
    (reference r7 (scope relative) (span (offset 796) (line 17) (column 43) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 796) (line 17) (column 43) (len 8)))))
    (reference r8 (scope relative) (span (offset 1174) (line 24) (column 45) (len 5)) (segments (segment 0 (token "Clock") (name "Clock") (separator none) (span (offset 1174) (line 24) (column 45) (len 5)))))
    (reference r9 (scope relative) (span (offset 1181) (line 24) (column 52) (len 4)) (segments (segment 0 (token "Life") (name "Life") (separator none) (span (offset 1181) (line 24) (column 52) (len 4)))))
  )
  (root (library-package (name "Occurrences") (standard true) (body brace (kerml-classifier (keyword metaclass) (abstract false) (name "AnnotatingElement") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body brace (kerml-feature (name "annotatedElement") (relationships) (value none) (body semicolon)) (kerml-feature (name "in") (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword datatype) (abstract false) (name "UnitBoundedReal") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1)))) (body brace (invariant))) (kerml-classifier (keyword assoc) (abstract false) (name "HappensBefore") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r2) (ref r3)))) (body brace (kerml-feature (name "source") (relationships) (value none) (body semicolon)) (kerml-feature (name "spaceShotOf") (relationships (inverse-of (ref r4))) (value none) (body semicolon)) (invariant))) (kerml-classifier (keyword assoc struct) (abstract false) (name "LinkObject") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5) (ref r6)))) (body brace (doc (name none) (locale none) (body (span (offset 701) (line 15) (column 15) (len 44)) (normalized "A LinkObject is both a Link and an Object. "))))) (kerml-classifier (keyword class) (abstract true) (name "Occurrence") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r7)))) (body brace (kerml-feature (name none) (relationships) (value none) (body semicolon)))) (kerml-feature (name "performances") (relationships) (value none) (body brace (doc (name none) (locale none) (body (span (offset 986) (line 21) (column 15) (len 55)) (normalized "performances is the base feature of all performances. "))))) (kerml-feature (name "evaluations") (relationships) (value none) (body semicolon)) (kerml-classifier (keyword struct) (abstract false) (name "UniversalClockLife") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r8) (ref r9)))) (body brace (kerml-feature (name "universalClock") (relationships) (value none) (body semicolon)))))))
)
~~~
