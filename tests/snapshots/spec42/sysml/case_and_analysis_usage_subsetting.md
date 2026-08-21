# META
~~~sexpr
(snapshot (type semantic) (description "analysis/case usage subsetting and redefinition clauses (:>/:>>)"))
~~~
# SOURCE
~~~sysml
package CaseAnalysisSubsettingExample {
    analysis a : A1 :> baseAnalysis {
    }
    case c :>> baseCase;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "case_and_analysis_usage_subsetting.md"
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
    (reference r0 (scope relative) (span (offset 57) (line 2) (column 18) (len 2)) (segments (segment 0 (token "A1") (name "A1") (separator none) (span (offset 57) (line 2) (column 18) (len 2)))))
    (reference r1 (scope relative) (span (offset 63) (line 2) (column 24) (len 12)) (segments (segment 0 (token "baseAnalysis") (name "baseAnalysis") (separator none) (span (offset 63) (line 2) (column 24) (len 12)))))
  )
  (root (package (name "CaseAnalysisSubsettingExample") (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "a") (type (ref r0)) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none)) (case-usage))))
)
~~~
