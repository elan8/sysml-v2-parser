# META
~~~sexpr
(snapshot (type semantic) (description "Exact VariantUsage contexts from Variability Examples/VehicleVariabilityModel.sysml:72-92: typed attribute variants in AttributeBody and typed port variants in the nested PortBody."))
~~~
# SOURCE
~~~sysml
package VehicleVariabilityModel {
    package PartsTree {
        variation attribute def DiameterChoices :> Diameter {
            variant attribute diameterSmall;
            variant attribute diameterLarge;
        }
        variation part def EngineChoices :> Engine {
            variant '4cylEngine';
            variant '6cylEngine' {
                variation port :>> autoPort {
                    variant port autoPort1;
                    variant port autoPort2;
                }
            }
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_variant_vehicle_variability_model.md"
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
    (reference r0 (scope relative) (span (offset 109) (line 3) (column 52) (len 8)) (segments (segment 0 (token "Diameter") (name "Diameter") (separator none) (span (offset 109) (line 3) (column 52) (len 8)))))
    (reference r1 (scope relative) (span (offset 293) (line 8) (column 21) (len 12)) (segments (segment 0 (token "'4cylEngine'") (name "4cylEngine") (separator none) (span (offset 293) (line 8) (column 21) (len 12)))))
    (reference r2 (scope relative) (span (offset 327) (line 9) (column 21) (len 12)) (segments (segment 0 (token "'6cylEngine'") (name "6cylEngine") (separator none) (span (offset 327) (line 9) (column 21) (len 12)))))
    (reference r3 (scope relative) (span (offset 377) (line 10) (column 36) (len 8)) (segments (segment 0 (token "autoPort") (name "autoPort") (separator none) (span (offset 377) (line 10) (column 36) (len 8)))))
  )
  (root (package (name "VehicleVariabilityModel") (body brace (package (name "PartsTree") (body brace (attribute-def (declaration-name "DiameterChoices") (short-name none) (modifiers (variation (span (offset 66) (line 3) (column 9) (len 9)))) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (variant-usage (target none) (usage (attribute-usage (declaration-name "diameterSmall") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (body absent)) (variant-usage (target none) (usage (attribute-usage (declaration-name "diameterLarge") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (body absent)))) (part-def (name "EngineChoices") (modifiers (variation (span (offset 228) (line 7) (column 9) (len 9)))) (body brace (variant-usage (target (ref r1)) (usage none) (body absent)) (variant-usage (target (ref r2)) (usage none) (body brace (port-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value none) (body brace (variant-usage (target none) (usage (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "autoPort1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (body absent)) (variant-usage (target none) (usage (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "autoPort2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon))) (body absent)))))))))))))
)
~~~
