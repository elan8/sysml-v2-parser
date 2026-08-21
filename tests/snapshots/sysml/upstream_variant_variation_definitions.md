# META
~~~sexpr
(snapshot (type semantic) (description "Exact VariantUsage context from training/36. Variability/Variation Definitions.sysml:26-32: typed attribute variants in a variation attribute definition and untyped quoted variant references."))
~~~
# SOURCE
~~~sysml
package 'Variation Definitions' {
    variation attribute def DiameterChoices :> Diameter {
        variant attribute diameterSmall = 70[mm];
        variant attribute diameterLarge = 100[mm];
    }
    variation part def EngineChoices :> Engine {
        variant '4cylEngine';
        variant '6cylEngine';
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_variant_variation_definitions.md"
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
    (reference r0 (scope relative) (span (offset 81) (line 2) (column 48) (len 8)) (segments (segment 0 (token "Diameter") (name "Diameter") (separator none) (span (offset 81) (line 2) (column 48) (len 8)))))
    (reference r1 (scope relative) (span (offset 137) (line 3) (column 46) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 137) (line 3) (column 46) (len 2)))))
    (reference r2 (scope relative) (span (offset 188) (line 4) (column 47) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 188) (line 4) (column 47) (len 2)))))
    (reference r3 (scope relative) (span (offset 264) (line 7) (column 17) (len 12)) (segments (segment 0 (token "'4cylEngine'") (name "4cylEngine") (separator none) (span (offset 264) (line 7) (column 17) (len 12)))))
    (reference r4 (scope relative) (span (offset 294) (line 8) (column 17) (len 12)) (segments (segment 0 (token "'6cylEngine'") (name "6cylEngine") (separator none) (span (offset 294) (line 8) (column 17) (len 12)))))
  )
  (root (package (name "Variation Definitions") (body brace (attribute-def (declaration-name "DiameterChoices") (short-name none) (modifiers (variation (span (offset 38) (line 2) (column 5) (len 9)))) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (variant-usage (target none) (usage (attribute-usage (declaration-name "diameterSmall") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 134) (line 3) (column 43) (len 6)) (bracket (base (expression (span (offset 134) (line 3) (column 43) (len 2)) (integer 70))) (operands (sequence-list (element first (expression (span (offset 137) (line 3) (column 46) (len 2)) (ref r1)))))))))) (body semicolon))) (body absent)) (variant-usage (target none) (usage (attribute-usage (declaration-name "diameterLarge") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 184) (line 4) (column 43) (len 7)) (bracket (base (expression (span (offset 184) (line 4) (column 43) (len 3)) (integer 100))) (operands (sequence-list (element first (expression (span (offset 188) (line 4) (column 47) (len 2)) (ref r2)))))))))) (body semicolon))) (body absent)))) (part-def (name "EngineChoices") (modifiers (variation (span (offset 203) (line 6) (column 5) (len 9)))) (body brace (variant-usage (target (ref r3)) (usage none) (body absent)) (variant-usage (target (ref r4)) (usage none) (body absent)))))))
)
~~~
