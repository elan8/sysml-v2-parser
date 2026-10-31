# META
~~~sexpr
(snapshot (type recovery) (description "A malformed prefixed analysis usage recovers as one body member without consuming a following valid `ref analysis` usage or an ordinary later case usage. This protects the complete-prefix first-refusal seam while making malformed input explicit rather than silently treating `ref` as an expression."))
~~~
# SOURCE
~~~sysml
package AnalysisCaseUsagePrefixRecovery {
    ref analysis : ;
    ref analysis recovered : A;
    case following : C;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "analysis_case_usage_prefix_recovery.md"
    (diagnostics
      (diagnostic (code "missing_type_reference") (severity error) (category parseerror) (span (offset 46) (line 2) (column 5) (len 16)) (message "expected reference type after ':'"))
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
    (reference r0 (scope relative) (span (offset 92) (line 3) (column 30) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 92) (line 3) (column 30) (len 1)))))
    (reference r1 (scope relative) (span (offset 116) (line 4) (column 22) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 116) (line 4) (column 22) (len 1)))))
  )
  (root (package (visibility none) (name "AnalysisCaseUsagePrefixRecovery") (body brace (malformed (code "missing_type_reference") (found "ref analysis : ;") (span (offset 46) (line 2) (column 5) (len 16))) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (name "recovered") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (body semicolon)) (case-usage (declaration (name "following") (short-name none) (abstract false) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none)) (body semicolon)))))
)
~~~
