# META
~~~sexpr
(snapshot (type semantic) (description "Exact SubjectUsage context from Individuals Examples/AnalysisIndividualExample.sysml:80: an individual analysis subject with typing, :> subsetting, and a typed DefinitionBody."))
~~~
# SOURCE
~~~sysml
package AnalysisIndividualExample {
    individual analysis fuelEconomyAnalysis_1 : FuelEconomyAnalysis_1 {
        subject vehicle : Vehicle_1 :> vehicle_c1 {
            individual part :>> engine : Engine_1 {
                attribute :>> peakPower = 200[hp];
                attribute :>> fuelEfficiency = 0.4;
            }
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_subject_analysis_individual_example.md"
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 84) (line 2) (column 49) (len 21)) (segments (segment 0 (token "FuelEconomyAnalysis_1") (name "FuelEconomyAnalysis_1") (separator none) (span (offset 84) (line 2) (column 49) (len 21)))))
    (reference r1 (scope relative) (span (offset 134) (line 3) (column 27) (len 9)) (segments (segment 0 (token "Vehicle_1") (name "Vehicle_1") (separator none) (span (offset 134) (line 3) (column 27) (len 9)))))
    (reference r2 (scope relative) (span (offset 147) (line 3) (column 40) (len 10)) (segments (segment 0 (token "vehicle_c1") (name "vehicle_c1") (separator none) (span (offset 147) (line 3) (column 40) (len 10)))))
    (reference r3 (scope relative) (span (offset 201) (line 4) (column 42) (len 8)) (segments (segment 0 (token "Engine_1") (name "Engine_1") (separator none) (span (offset 201) (line 4) (column 42) (len 8)))))
    (reference r4 (scope relative) (span (offset 192) (line 4) (column 33) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 192) (line 4) (column 33) (len 6)))))
    (reference r5 (scope relative) (span (offset 242) (line 5) (column 31) (len 9)) (segments (segment 0 (token "peakPower") (name "peakPower") (separator none) (span (offset 242) (line 5) (column 31) (len 9)))))
    (reference r6 (scope relative) (span (offset 258) (line 5) (column 47) (len 2)) (segments (segment 0 (token "hp") (name "hp") (separator none) (span (offset 258) (line 5) (column 47) (len 2)))))
    (reference r7 (scope relative) (span (offset 293) (line 6) (column 31) (len 14)) (segments (segment 0 (token "fuelEfficiency") (name "fuelEfficiency") (separator none) (span (offset 293) (line 6) (column 31) (len 14)))))
  )
  (root (package (visibility none) (name "AnalysisIndividualExample") (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (name "fuelEconomyAnalysis_1") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (body brace (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 254) (line 5) (column 43) (len 7)) (bracket (base (expression (span (offset 254) (line 5) (column 43) (len 3)) (integer 200))) (operands (sequence-list (element first (expression (span (offset 258) (line 5) (column 47) (len 2)) (ref r6)))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 310) (line 6) (column 48) (len 3)) (real "0.4"))))) (body semicolon)))))))))))
)
~~~
