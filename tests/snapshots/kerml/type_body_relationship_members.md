# META
~~~sexpr
(snapshot (type semantic) (description "KerML type bodies structure connector, binding, succession, end-cross-feature, nested classifier, import, attribute, and keyword-less binding members that were previously captured opaquely without diagnostics."))
~~~
# SOURCE
~~~sysml
standard library package Occurrences {
    private import SequenceFunctions::*;
    assoc all Within specializes HappensDuring, InsideOf {
        connector :HappensDuring from [1] self to [1] this;
        connector :Without
            from [0..*] separateOccurrenceToo references elements.notIntersection
            to [1] separateOccurrence references intersection;
        binding [1] startShot = [1] endShot;
        binding oSelf of sourceOccurrence.portionOfLife = targetOccurrence.portionOfLife;
        end happensDuring [1..*] feature longerOccurrence: Occurrence redefines targetOccurrence;
        end [1] feature transferSource references source;
        private succession all [*] trigger then [*] guard;
        private succession triggerAfter [taNum] first [0..1] transitionLinkSource then [*] trigger.endShot;
        feature self: Anything[1] subsets things chains things.that;
        feature withoutOccurrences: Occurrence[0..*] unions successors, predecessors
            inverse of withoutOccurrences;
        portion redefines portionOfLife = (that as Occurrence).portionOfLife;
        private instantNum: Natural[1] = if isInstant? 1 else 0;
        private thisClock : Clock :>> self;
        struct StructuredSurface specializes StructuredSpaceObject, Surface {
            doc /* A nested classifier. */
        }
    }
    function dot { in v : CartesianVectorValue[1]; in w : CartesianVectorValue[1];
        return x: Real[1] =
            (1..v.dimension)->collect{in i : Positive; v#(i) * w#(i)}->reduce RealFunctions::'+';
    }
    function sum0 { in collection: ScalarValue[0..*]; in zero: ScalarValue;
        return : ScalarValue = collection->reduce '+' ?? zero;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "type_body_relationship_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Occurrences {
    private import SequenceFunctions::*;
    assoc all Within specializes HappensDuring, InsideOf {
        connector :HappensDuring from [1] self to [1] this;
        connector :Without from [0..*] separateOccurrenceToo references elements.notIntersection to [1] separateOccurrence references intersection;
        binding [1] startShot = [1] endShot;
        binding oSelf of sourceOccurrence.portionOfLife = targetOccurrence.portionOfLife;
        end happensDuring [1..*] feature longerOccurrence : Occurrence redefines targetOccurrence;
        end [1] feature transferSource references source;
        private succession all [*] trigger then [*] guard;
        private succession triggerAfter[taNum] first [0..1] transitionLinkSource then [*] trigger.endShot;
        feature self : Anything[1] subsets things chains things.that;
        feature withoutOccurrences : Occurrence[0..*] unions successors, predecessors inverse of withoutOccurrences;
        portion redefines portionOfLife = (that as Occurrence).portionOfLife;
        private instantNum : Natural[1] = if isInstant ? 1 else 0;
        private thisClock : Clock :>> self;
        struct StructuredSurface specializes StructuredSpaceObject, Surface {
            doc
            /* A nested classifier. */
        }
    }
    function dot {
        in v : CartesianVectorValue[1];
        in w : CartesianVectorValue[1];
        return x : Real[1] = (1 .. v.dimension)->collect { in i : Positive; v#(i) * w#(i) }->reduce(RealFunctions::'+');
    }
    function sum0 {
        in collection : ScalarValue[0..*];
        in zero : ScalarValue;
        return : ScalarValue = collection->reduce('+') ?? zero;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 58) (line 2) (column 20) (len 17)) (segments (segment 0 (token "SequenceFunctions") (name "SequenceFunctions") (separator none) (span (offset 58) (line 2) (column 20) (len 17)))))
    (reference r1 (scope relative) (span (offset 113) (line 3) (column 34) (len 13)) (segments (segment 0 (token "HappensDuring") (name "HappensDuring") (separator none) (span (offset 113) (line 3) (column 34) (len 13)))))
    (reference r2 (scope relative) (span (offset 128) (line 3) (column 49) (len 8)) (segments (segment 0 (token "InsideOf") (name "InsideOf") (separator none) (span (offset 128) (line 3) (column 49) (len 8)))))
    (reference r3 (scope relative) (span (offset 565) (line 10) (column 60) (len 10)) (segments (segment 0 (token "Occurrence") (name "Occurrence") (separator none) (span (offset 565) (line 10) (column 60) (len 10)))))
    (reference r4 (scope relative) (span (offset 586) (line 10) (column 81) (len 16)) (segments (segment 0 (token "targetOccurrence") (name "targetOccurrence") (separator none) (span (offset 586) (line 10) (column 81) (len 16)))))
    (reference r5 (scope relative) (span (offset 654) (line 11) (column 51) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 654) (line 11) (column 51) (len 6)))))
    (reference r6 (scope relative) (span (offset 851) (line 14) (column 23) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 851) (line 14) (column 23) (len 8)))))
    (reference r7 (scope relative) (span (offset 871) (line 14) (column 43) (len 6)) (segments (segment 0 (token "things") (name "things") (separator none) (span (offset 871) (line 14) (column 43) (len 6)))))
    (reference r8 (scope relative) (span (offset 885) (line 14) (column 57) (len 11)) (segments (segment 0 (token "things") (name "things") (separator none) (span (offset 885) (line 14) (column 57) (len 6))) (segment 1 (token "that") (name "that") (separator dot) (span (offset 892) (line 14) (column 64) (len 4)))))
    (reference r9 (scope relative) (span (offset 934) (line 15) (column 37) (len 10)) (segments (segment 0 (token "Occurrence") (name "Occurrence") (separator none) (span (offset 934) (line 15) (column 37) (len 10)))))
    (reference r10 (scope relative) (span (offset 958) (line 15) (column 61) (len 10)) (segments (segment 0 (token "successors") (name "successors") (separator none) (span (offset 958) (line 15) (column 61) (len 10)))))
    (reference r11 (scope relative) (span (offset 970) (line 15) (column 73) (len 12)) (segments (segment 0 (token "predecessors") (name "predecessors") (separator none) (span (offset 970) (line 15) (column 73) (len 12)))))
    (reference r12 (scope relative) (span (offset 1006) (line 16) (column 24) (len 18)) (segments (segment 0 (token "withoutOccurrences") (name "withoutOccurrences") (separator none) (span (offset 1006) (line 16) (column 24) (len 18)))))
    (reference r13 (scope relative) (span (offset 1052) (line 17) (column 27) (len 13)) (segments (segment 0 (token "portionOfLife") (name "portionOfLife") (separator none) (span (offset 1052) (line 17) (column 27) (len 13)))))
    (reference r14 (scope relative) (span (offset 1069) (line 17) (column 44) (len 4)) (segments (segment 0 (token "that") (name "that") (separator none) (span (offset 1069) (line 17) (column 44) (len 4)))))
    (reference r15 (scope relative) (span (offset 1077) (line 17) (column 52) (len 10)) (segments (segment 0 (token "Occurrence") (name "Occurrence") (separator none) (span (offset 1077) (line 17) (column 52) (len 10)))))
    (reference r16 (scope relative) (span (offset 1089) (line 17) (column 64) (len 13)) (segments (segment 0 (token "portionOfLife") (name "portionOfLife") (separator none) (span (offset 1089) (line 17) (column 64) (len 13)))))
    (reference r17 (scope relative) (span (offset 1132) (line 18) (column 29) (len 7)) (segments (segment 0 (token "Natural") (name "Natural") (separator none) (span (offset 1132) (line 18) (column 29) (len 7)))))
    (reference r18 (scope relative) (span (offset 1148) (line 18) (column 45) (len 9)) (segments (segment 0 (token "isInstant") (name "isInstant") (separator none) (span (offset 1148) (line 18) (column 45) (len 9)))))
    (reference r19 (scope relative) (span (offset 1197) (line 19) (column 29) (len 5)) (segments (segment 0 (token "Clock") (name "Clock") (separator none) (span (offset 1197) (line 19) (column 29) (len 5)))))
    (reference r20 (scope relative) (span (offset 1207) (line 19) (column 39) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 1207) (line 19) (column 39) (len 4)))))
    (reference r21 (scope relative) (span (offset 1258) (line 20) (column 46) (len 21)) (segments (segment 0 (token "StructuredSpaceObject") (name "StructuredSpaceObject") (separator none) (span (offset 1258) (line 20) (column 46) (len 21)))))
    (reference r22 (scope relative) (span (offset 1281) (line 20) (column 69) (len 7)) (segments (segment 0 (token "Surface") (name "Surface") (separator none) (span (offset 1281) (line 20) (column 69) (len 7)))))
    (reference r23 (scope relative) (span (offset 1376) (line 24) (column 27) (len 20)) (segments (segment 0 (token "CartesianVectorValue") (name "CartesianVectorValue") (separator none) (span (offset 1376) (line 24) (column 27) (len 20)))))
    (reference r24 (scope relative) (span (offset 1408) (line 24) (column 59) (len 20)) (segments (segment 0 (token "CartesianVectorValue") (name "CartesianVectorValue") (separator none) (span (offset 1408) (line 24) (column 59) (len 20)))))
    (reference r25 (scope relative) (span (offset 1600) (line 28) (column 36) (len 11)) (segments (segment 0 (token "ScalarValue") (name "ScalarValue") (separator none) (span (offset 1600) (line 28) (column 36) (len 11)))))
    (reference r26 (scope relative) (span (offset 1628) (line 28) (column 64) (len 11)) (segments (segment 0 (token "ScalarValue") (name "ScalarValue") (separator none) (span (offset 1628) (line 28) (column 64) (len 11)))))
  )
  (root (library-package (name "Occurrences") (standard true) (body brace (import (target (span (span (offset 58) (line 2) (column 20) (len 20))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 75) (line 2) (column 37) (len 3))) (separator (span (offset 75) (line 2) (column 37) (len 2))) (marker (span (offset 77) (line 2) (column 39) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (kerml-classifier (keyword assoc) (abstract false) (name "Within") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1) (ref r2)))) (body brace (connector) (connector) (binding (name none) (body semicolon)) (binding (name "oSelf") (body semicolon)) (kerml-feature (prefix (head end) (constant false) (cross present) (metadata)) (kind feature) (member false) (all false) (name "longerOccurrence") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head end) (constant false) (cross present) (metadata)) (kind feature) (member false) (all false) (name "transferSource") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r5)))) (crosses none) (relationships) (value none) (body semicolon)) (succession) (succession) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "self") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity (lower (expression (span (offset 860) (line 14) (column 32) (len 1)) (integer 1))) (upper (expression (span (offset 860) (line 14) (column 32) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r7)))) (redefines none) (references none) (crosses none) (relationships (chains (ref r8))) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "withoutOccurrences") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity (lower (expression (span (offset 945) (line 15) (column 48) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships (type-relationship (keyword unions) (targets (ref r10) (ref r11))) (inverse-of (ref r12))) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion portion) (variability none) (metadata)) (kind none) (member false) (all false) (name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (references none) (crosses none) (relationships) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1068) (line 17) (column 43) (len 34)) (member-access (base (expression (span (offset 1068) (line 17) (column 43) (len 20)) (sequence (sequence-list (element first (expression (span (offset 1069) (line 17) (column 44) (len 18)) (type-check (kind as) (operand (expression (span (offset 1069) (line 17) (column 44) (len 4)) (ref r14))) (type (ref r15))))))))) (separator dot) (member (ref r16))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "instantNum") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity (lower (expression (span (offset 1140) (line 18) (column 37) (len 1)) (integer 1))) (upper (expression (span (offset 1140) (line 18) (column 37) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1145) (line 18) (column 42) (len 22)) (conditional (test (expression (span (offset 1148) (line 18) (column 45) (len 9)) (ref r18))) (then (expression (span (offset 1159) (line 18) (column 56) (len 1)) (integer 1))) (else (expression (span (offset 1166) (line 18) (column 63) (len 1)) (integer 0)))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "thisClock") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (kerml-classifier (keyword struct) (abstract false) (name "StructuredSurface") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r21) (ref r22)))) (body brace (doc (name none) (locale none) (body (span (offset 1309) (line 21) (column 19) (len 22)) (normalized "A nested classifier. "))))))) (kerml-classifier (keyword function) (abstract false) (name "dot") (specializes none) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "v") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (multiplicity (lower (expression (span (offset 1397) (line 24) (column 48) (len 1)) (integer 1))) (upper (expression (span (offset 1397) (line 24) (column 48) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "w") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity (lower (expression (span (offset 1429) (line 24) (column 80) (len 1)) (integer 1))) (upper (expression (span (offset 1429) (line 24) (column 80) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (return-declaration (name "x") (short-name none)))) (kerml-classifier (keyword function) (abstract false) (name "sum0") (specializes none) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "collection") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (multiplicity (lower (expression (span (offset 1612) (line 28) (column 48) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "zero") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (return-declaration (name none) (short-name none)))))))
)
~~~
