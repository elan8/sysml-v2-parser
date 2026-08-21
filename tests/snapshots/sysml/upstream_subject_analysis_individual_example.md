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
  (references
    (reference r0 (scope relative) (span (offset 84) (line 2) (column 49) (len 21)) (segments (segment 0 (token "FuelEconomyAnalysis_1") (name "FuelEconomyAnalysis_1") (separator none) (span (offset 84) (line 2) (column 49) (len 21)))))
  )
  (root (package (name "AnalysisIndividualExample") (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (name "fuelEconomyAnalysis_1") (type (ref r0)) (subsets none) (redefines none)))))
)
~~~
