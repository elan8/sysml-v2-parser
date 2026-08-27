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
  (references
    (reference r0 (scope relative) (span (offset 73) (line 2) (column 39) (len 10)) (segments (segment 0 (token "TradeStudy") (name "TradeStudy") (separator none) (span (offset 73) (line 2) (column 39) (len 10)))))
  )
  (root (package (name "RationaleMetadataExample") (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "engineTradeOffAnalysis") (type (ref r0)) (subsets none) (redefines none)))))
)
~~~
