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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 67) (line 2) (column 39) (len 10)) (segments (segment 0 (token "TradeStudy") (name "TradeStudy") (separator none) (span (offset 67) (line 2) (column 39) (len 10)))))
    (reference r1 (scope relative) (span (offset 120) (line 3) (column 41) (len 9)) (segments (segment 0 (token "vehicle_b") (name "vehicle_b") (separator none) (span (offset 120) (line 3) (column 41) (len 9)))))
  )
  (root (package (visibility none) (name "SimpleVehicleModel") (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "engineTradeOffAnalysis") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (body brace (subject (name "vehicleAlternatives") (short-name none) (typing none) (multiplicity (lower none) (upper (expression (span (offset 116) (line 3) (column 37) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
