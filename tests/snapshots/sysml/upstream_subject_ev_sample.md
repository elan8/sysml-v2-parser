# META
~~~sexpr
(snapshot (type semantic) (description "Exact SubjectUsage form and analysis context from State Space Representation Examples/EVSample.sysml:282: anonymous :>> redefinition followed by :> subsetting."))
~~~
# SOURCE
~~~sysml
package EVSample {
    analysis largeEVAnalysis : VehicleAnalysis {
        subject :>> vehicle :> vehicle_large;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_subject_ev_sample.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package EVSample {
    analysis largeEVAnalysis : VehicleAnalysis {
        subject :> vehicle_large :>> vehicle;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 50) (line 2) (column 32) (len 15)) (segments (segment 0 (token "VehicleAnalysis") (name "VehicleAnalysis") (separator none) (span (offset 50) (line 2) (column 32) (len 15)))))
    (reference r1 (scope relative) (span (offset 99) (line 3) (column 32) (len 13)) (segments (segment 0 (token "vehicle_large") (name "vehicle_large") (separator none) (span (offset 99) (line 3) (column 32) (len 13)))))
    (reference r2 (scope relative) (span (offset 88) (line 3) (column 21) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 88) (line 3) (column 21) (len 7)))))
  )
  (root (package (name "EVSample") (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "largeEVAnalysis") (short-name none) (type (ref r0)) (subsets none) (redefines none) (body brace (subject (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
