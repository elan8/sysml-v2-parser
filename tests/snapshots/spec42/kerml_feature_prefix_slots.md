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
        inout feature replacementValues : Anything[*] nonunique :>> values;
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
    (reference r0 (scope relative) (span (offset 952) (line 36) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 952) (line 36) (column 10) (len 3)))))
    (reference r1 (scope relative) (span (offset 977) (line 37) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 977) (line 37) (column 10) (len 3)))))
  )
  (root (package (name "KermlFeaturePrefixSlots") (body brace (kerml-classifier (keyword type) (abstract false) (name "DirectedBasicPrefix") (specializes none) (body brace (kerml-feature (name "q") (relationships) (value none) (body semicolon)) (kerml-feature (name "o") (relationships) (value none) (body semicolon)) (kerml-feature (name "p") (relationships) (value none) (body semicolon)) (kerml-feature (name "s") (relationships) (value none) (body semicolon)) (kerml-feature (name "r") (relationships) (value none) (body semicolon)) (kerml-feature (name "t") (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "SpecializationTail") (specializes none) (body brace (kerml-feature (name "replacementValues") (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "DirectedKinds") (specializes none) (body brace (kerml-feature (name "e") (relationships) (value none) (body semicolon)) (kerml-feature (name "b") (relationships) (value none) (body semicolon)) (kerml-feature (name "st") (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "UndirectedSlots") (specializes none) (body brace (kerml-feature (name "d") (relationships) (value none) (body semicolon)) (kerml-feature (name "c") (relationships) (value none) (body semicolon)) (kerml-feature (name "po") (relationships) (value none) (body semicolon)) (kerml-feature (name "v") (relationships) (value none) (body semicolon)) (kerml-feature (name "k") (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "EndCrossFeature") (specializes none) (body brace (kerml-feature (name "constrainedLink") (relationships) (value none) (body semicolon)) (kerml-feature (name "transferSource") (relationships) (value none) (body semicolon)) (kerml-feature (name "constEnd") (relationships) (value none) (body semicolon)) (kerml-feature (name "plain") (relationships) (value none) (body semicolon)))) (metadata-def (name "Tag") (abstract false) (specializes none) (body semicolon)) (metadata-def (name "Tag2") (abstract false) (specializes none) (body semicolon)) (kerml-classifier (keyword type) (abstract false) (name "MetadataTail") (specializes none) (body brace (kerml-feature (name "z3") (relationships) (value none) (body semicolon)) (kerml-feature (name "z4") (relationships) (value none) (body semicolon)) (metadata-keyword-usage (type (ref r0)) (body none)) (kerml-feature (name "z1") (relationships) (value none) (body semicolon)) (metadata-keyword-usage (type (ref r1)) (body semicolon)))) (calc-def (name "CalcScope") (modifiers) (body brace (calc-usage))))))
)
~~~
