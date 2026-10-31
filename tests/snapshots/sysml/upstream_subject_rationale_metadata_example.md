# META
~~~sexpr
(snapshot (type semantic) (description "Exact SubjectUsage form and analysis context from Metadata Examples/RationaleMetadataExample.sysml:18: :> subsetting, multiplicity after it, and a typed feature value."))
~~~
# SOURCE
~~~sysml
package RationaleMetadataExample {
    analysis engineTradeOffAnalysis : TradeStudy {
        subject alternatives :> engine [2] = (engine4cyl, engine6cyl);
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_subject_rationale_metadata_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package RationaleMetadataExample {
    analysis engineTradeOffAnalysis : TradeStudy {
        subject alternatives[2] :> engine = (engine4cyl, engine6cyl);
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 73) (line 2) (column 39) (len 10)) (segments (segment 0 (token "TradeStudy") (name "TradeStudy") (separator none) (span (offset 73) (line 2) (column 39) (len 10)))))
    (reference r1 (scope relative) (span (offset 118) (line 3) (column 33) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 118) (line 3) (column 33) (len 6)))))
    (reference r2 (scope relative) (span (offset 132) (line 3) (column 47) (len 10)) (segments (segment 0 (token "engine4cyl") (name "engine4cyl") (separator none) (span (offset 132) (line 3) (column 47) (len 10)))))
    (reference r3 (scope relative) (span (offset 144) (line 3) (column 59) (len 10)) (segments (segment 0 (token "engine6cyl") (name "engine6cyl") (separator none) (span (offset 144) (line 3) (column 59) (len 10)))))
  )
  (root (package (visibility none) (name "RationaleMetadataExample") (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "engineTradeOffAnalysis") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (body brace (subject (name "alternatives") (short-name none) (typing none) (multiplicity (lower none) (upper (expression (span (offset 126) (line 3) (column 41) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 131) (line 3) (column 46) (len 24)) (sequence (sequence-list (element first (expression (span (offset 132) (line 3) (column 47) (len 10)) (ref r2))) (element comma (expression (span (offset 144) (line 3) (column 59) (len 10)) (ref r3))))))))) (body semicolon)))))))
)
~~~
