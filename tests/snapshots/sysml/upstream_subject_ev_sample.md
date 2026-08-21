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
  (references
    (reference r0 (scope relative) (span (offset 50) (line 2) (column 32) (len 15)) (segments (segment 0 (token "VehicleAnalysis") (name "VehicleAnalysis") (separator none) (span (offset 50) (line 2) (column 32) (len 15)))))
  )
  (root (package (name "EVSample") (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "largeEVAnalysis") (type (ref r0)) (subsets none) (redefines none)))))
)
~~~
