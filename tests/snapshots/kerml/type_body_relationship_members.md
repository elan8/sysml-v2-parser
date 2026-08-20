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
        end happensDuring [1..*] feature longerOccurrence : Occurrence :>> targetOccurrence;
        end [1] feature transferSource ::> source;
        private succession all [*] trigger then [*] guard;
        private succession triggerAfter[taNum] first [0..1] transitionLinkSource then [*] trigger.endShot;
        feature self : Anything[1] :> things chains things.that;
        feature withoutOccurrences : Occurrence[0..*] unions successors, predecessors inverse of withoutOccurrences;
        portion :>> portionOfLife = (that as Occurrence).portionOfLife;
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
    (reference r3 (scope relative) (span (offset 1258) (line 20) (column 46) (len 21)) (segments (segment 0 (token "StructuredSpaceObject") (name "StructuredSpaceObject") (separator none) (span (offset 1258) (line 20) (column 46) (len 21)))))
    (reference r4 (scope relative) (span (offset 1281) (line 20) (column 69) (len 7)) (segments (segment 0 (token "Surface") (name "Surface") (separator none) (span (offset 1281) (line 20) (column 69) (len 7)))))
  )
  (root (library-package (name "Occurrences") (standard true) (body brace (import (target (span (span (offset 58) (line 2) (column 20) (len 20))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 75) (line 2) (column 37) (len 3))) (separator (span (offset 75) (line 2) (column 37) (len 2))) (marker (span (offset 77) (line 2) (column 39) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (kerml-classifier (keyword assoc) (abstract false) (name "Within") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1) (ref r2)))) (body brace (connector) (connector) (binding (name none) (body semicolon)) (binding (name "oSelf") (body semicolon)) (kerml-feature) (kerml-feature) (succession) (succession) (kerml-feature) (kerml-feature) (kerml-feature) (default-reference-usage) (default-reference-usage) (kerml-classifier (keyword struct) (abstract false) (name "StructuredSurface") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r3) (ref r4)))) (body brace (doc (name none) (locale none) (body (span (offset 1309) (line 21) (column 19) (len 22)) (normalized "A nested classifier. "))))))) (kerml-classifier (keyword function) (abstract false) (name "dot") (specializes none) (body brace (in-out-declaration) (in-out-declaration) (return-declaration (name "x") (short-name none)))) (kerml-classifier (keyword function) (abstract false) (name "sum0") (specializes none) (body brace (in-out-declaration) (in-out-declaration) (return-declaration (name none) (short-name none)))))))
)
~~~
