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
  )
  (root (package (name "CaseAnalysisSubsettingExample") (body brace (analysis-case-usage) (case-usage))))
)
~~~
