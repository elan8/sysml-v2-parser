# META
~~~sexpr
(snapshot (type semantic) (description "The same KerML feature prefix in every scope that dispatches a Feature, so one node reached from seven scopes projects and re-emits identically wherever it appears. This is the property a merge has to earn: before it, the directed spelling reached only the three calc-body scopes and the undirected one reached all seven, purely because a second node had been added at one dispatch site. `derived var feature` and `in feature` are written side by side in each scope; identical syntax must give identical projection across all of them."))
~~~
# SOURCE
~~~sysml
package KermlFeaturePrefixOwningScopes {
    derived var feature namespaceLevel;
    in feature namespaceDirected;
    struct TypeBodyScope {
        derived var feature inType;
        in feature inTypeDirected;
    }
    function CalcBodyScope {
        derived var feature inFunction;
        in feature inFunctionDirected;
    }
    calc def CalcDefScope {
        derived var feature inCalcDef;
        in feature inCalcDefDirected;
    }
    struct RecursiveMemberScope {
        feature outer {
            derived var feature inNestedFeature;
            in feature inNestedFeatureDirected;
        }
    }
    attribute def AttributeBodyScope {
        derived var feature inAttribute;
        in feature inAttributeDirected;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml_feature_prefix_owning_scopes.md"
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
  )
  (root (package (name "KermlFeaturePrefixOwningScopes") (body brace (kerml-feature (name "namespaceLevel") (relationships) (value none) (body semicolon)) (kerml-feature (name "namespaceDirected") (relationships) (value none) (body semicolon)) (kerml-classifier (keyword struct) (abstract false) (name "TypeBodyScope") (specializes none) (body brace (kerml-feature (name "inType") (relationships) (value none) (body semicolon)) (kerml-feature (name "inTypeDirected") (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword function) (abstract false) (name "CalcBodyScope") (specializes none) (body brace (kerml-feature (name "inFunction") (relationships) (value none) (body semicolon)) (kerml-feature (name "inFunctionDirected") (relationships) (value none) (body semicolon)))) (calc-def (name "CalcDefScope") (modifiers) (body brace (kerml-feature (name "inCalcDef") (relationships) (value none) (body semicolon)) (kerml-feature (name "inCalcDefDirected") (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword struct) (abstract false) (name "RecursiveMemberScope") (specializes none) (body brace (kerml-feature (name "outer") (relationships) (value none) (body brace (kerml-feature (name "inNestedFeature") (relationships) (value none) (body semicolon)) (kerml-feature (name "inNestedFeatureDirected") (relationships) (value none) (body semicolon)))))) (attribute-def (declaration-name "AttributeBodyScope") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (kerml-feature (name "inAttribute") (relationships) (value none) (body semicolon)) (kerml-feature (name "inAttributeDirected") (relationships) (value none) (body semicolon)))))))
)
~~~
