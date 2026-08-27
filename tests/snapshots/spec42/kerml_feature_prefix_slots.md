# META
~~~sexpr
(snapshot (type semantic) (description "Every slot of KerML's BasicFeaturePrefix (BNF 577) is authorable in front of every feature-kind keyword, because Feature (562), Step (863), Expression (895) and BooleanExpression (908) spell one shared FeaturePrefix (584). Before the merge a directed feature had its own node modelling only direction + abstract, so `in derived`, `in composite`, `in var` and `in portion` all reached recovery; `in calc` is not a Feature at all but SysML's CalculationUsage (SysML BNF 1355), and routes to calc-usage."))
~~~
# SOURCE
~~~sysml
package KermlFeaturePrefixSlots {
    type DirectedBasicPrefix {
        in derived feature q;
        in composite feature o;
        in var feature p;
        in portion feature s;
        out abstract feature r;
        inout feature t;
    }
    type SpecializationTail {
        inout feature replacementValues : Anything redefines values [*] nonunique;
    }
    type DirectedKinds {
        in expr e;
        in bool b;
        in step st;
    }
    type UndirectedSlots {
        derived feature d;
        composite feature c;
        portion feature po;
        var feature v;
        const feature k;
    }
    type EndCrossFeature {
        end guardedLink [0..1] feature constrainedLink;
        end [1] feature transferSource;
        const end feature constEnd;
        end plain;
    }
    metadata def Tag;
    metadata def Tag2;
    type MetadataTail {
        derived #Tag feature z3;
        in var #Tag #Tag2 feature z4;
        #Tag feature z1;
        #Tag;
    }
    calc def CalcScope {
        in calc nested;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml_feature_prefix_slots.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package KermlFeaturePrefixSlots {
    type DirectedBasicPrefix {
        in derived feature q;
        in composite feature o;
        in var feature p;
        in portion feature s;
        out abstract feature r;
        inout feature t;
    }
    type SpecializationTail {
        inout feature replacementValues : Anything redefines values[*] nonunique;
    }
    type DirectedKinds {
        in expr e;
        in bool b;
        in step st;
    }
    type UndirectedSlots {
        derived feature d;
        composite feature c;
        portion feature po;
        var feature v;
        const feature k;
    }
    type EndCrossFeature {
        end guardedLink [0..1] feature constrainedLink;
        end [1] feature transferSource;
        const end feature constEnd;
        end plain;
    }
    metadata def Tag;
    metadata def Tag2;
    type MetadataTail {
        derived #Tag feature z3;
        in var #Tag #Tag2 feature z4;
        #Tag
        feature z1;
        #Tag;
    }
    calc def CalcScope {
        in calc nested;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 318) (line 11) (column 43) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 318) (line 11) (column 43) (len 8)))))
    (reference r1 (scope relative) (span (offset 337) (line 11) (column 62) (len 6)) (segments (segment 0 (token "values") (name "values") (separator none) (span (offset 337) (line 11) (column 62) (len 6)))))
    (reference r2 (scope relative) (span (offset 889) (line 34) (column 18) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 889) (line 34) (column 18) (len 3)))))
    (reference r3 (scope relative) (span (offset 921) (line 35) (column 17) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 921) (line 35) (column 17) (len 3)))))
    (reference r4 (scope relative) (span (offset 926) (line 35) (column 22) (len 4)) (segments (segment 0 (token "Tag2") (name "Tag2") (separator none) (span (offset 926) (line 35) (column 22) (len 4)))))
    (reference r5 (scope relative) (span (offset 952) (line 36) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 952) (line 36) (column 10) (len 3)))))
    (reference r6 (scope relative) (span (offset 977) (line 37) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 977) (line 37) (column 10) (len 3)))))
  )
  (root (package (name "KermlFeaturePrefixSlots") (body brace (kerml-classifier (keyword type) (abstract false) (name "DirectedBasicPrefix") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction in) (derived true) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "q") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion composite) (variability none) (metadata)) (kind feature) (member false) (all false) (name "o") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability var) (metadata)) (kind feature) (member false) (all false) (name "p") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion portion) (variability none) (metadata)) (kind feature) (member false) (all false) (name "s") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction out) (derived false) (abstract true) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "r") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction inout) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "t") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "SpecializationTail") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction inout) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "replacementValues") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (redefinition (relationship (kind redefines) (implied false) (targets (ref r1))))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "DirectedKinds") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind expr) (member false) (all false) (name "e") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind bool) (member false) (all false) (name "b") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind step) (member false) (all false) (name "st") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "UndirectedSlots") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived true) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "d") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion composite) (variability none) (metadata)) (kind feature) (member false) (all false) (name "c") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion portion) (variability none) (metadata)) (kind feature) (member false) (all false) (name "po") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability var) (metadata)) (kind feature) (member false) (all false) (name "v") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability const) (metadata)) (kind feature) (member false) (all false) (name "k") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "EndCrossFeature") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head end) (constant false) (cross present) (metadata)) (kind feature) (member false) (all false) (name "constrainedLink") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head end) (constant false) (cross present) (metadata)) (kind feature) (member false) (all false) (name "transferSource") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head end) (constant true) (cross none) (metadata)) (kind feature) (member false) (all false) (name "constEnd") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind none) (member false) (all false) (name "plain") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)))) (metadata-def (name "Tag") (abstract false) (specializes none) (body semicolon)) (metadata-def (name "Tag2") (abstract false) (specializes none) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "MetadataTail") (specializes none) (conjugates none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived true) (abstract false) (portion none) (variability none) (metadata (ref r2))) (kind feature) (member false) (all false) (name "z3") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability var) (metadata (ref r3) (ref r4))) (kind feature) (member false) (all false) (name "z4") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (metadata-keyword-usage (type (ref r5)) (body none)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "z1") (specializations) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (metadata-keyword-usage (type (ref r6)) (body semicolon)))) (calc-def (name "CalcScope") (modifiers) (body brace (calc-usage))))))
)
~~~
