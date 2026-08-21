# META
~~~sexpr
(snapshot (type semantic) (description "Exact SubjectUsage form and analysis context from Vehicle Example/SysML v2 Spec Annex A SimpleVehicleModel.sysml:1166: multiplicity immediately before a :> subsetting clause."))
~~~
# SOURCE
~~~sysml
package SimpleVehicleModel {
    analysis engineTradeOffAnalysis : TradeStudy {
        subject vehicleAlternatives[2]:>vehicle_b;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_subject_simple_vehicle_model.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package SimpleVehicleModel {
    analysis engineTradeOffAnalysis : TradeStudy {
        subject vehicleAlternatives[2] :> vehicle_b;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 67) (line 2) (column 39) (len 10)) (segments (segment 0 (token "TradeStudy") (name "TradeStudy") (separator none) (span (offset 67) (line 2) (column 39) (len 10)))))
  )
  (root (package (name "SimpleVehicleModel") (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "engineTradeOffAnalysis") (type (ref r0)) (subsets none) (redefines none)))))
)
~~~
