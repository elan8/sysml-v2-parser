# META
~~~sexpr
(snapshot (type semantic) (description "AnalysisCaseUsage owns the complete OccurrenceUsagePrefix required by SysML textual BNF 1533-1535 and pinned Pilot SysML.xtext 2236. The four body scopes that own analysis usages each use `ref analysis`, whose competing `ref` starter previously reached a generic reference/expression path before the analysis usage; a no-prefix counterpart remains ordinary. A full prefix also proves the source-backed slot ordering, while the case-body `ref case` and `ref verification` counterparts remain typed RefDecls rather than being recategorized by the analysis guard."))
~~~
# SOURCE
~~~sysml
package AnalysisCaseUsagePrefix {
    metadata def Tag;
    analysis plain : A;
    ref analysis packageAnalysis : A;
    in derived abstract constant ref individual snapshot #Tag analysis allSlots : A;
    part def InPartDefinition {
        ref analysis partDefinitionAnalysis : A;
    }
    part inPartUsage {
        ref analysis partUsageAnalysis : A;
    }
    case def InCaseBody {
        ref analysis caseBodyAnalysis : A;
        ref case legacyCase : Case;
        ref verification legacyVerification : VerificationCase;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "analysis_case_usage_prefix.md"
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
    (reference r0 (scope relative) (span (offset 77) (line 3) (column 22) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 77) (line 3) (column 22) (len 1)))))
    (reference r1 (scope relative) (span (offset 115) (line 4) (column 36) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 115) (line 4) (column 36) (len 1)))))
    (reference r2 (scope relative) (span (offset 176) (line 5) (column 59) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 176) (line 5) (column 59) (len 3)))))
    (reference r3 (scope relative) (span (offset 200) (line 5) (column 83) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 200) (line 5) (column 83) (len 1)))))
    (reference r4 (scope relative) (span (offset 281) (line 7) (column 47) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 281) (line 7) (column 47) (len 1)))))
    (reference r5 (scope relative) (span (offset 354) (line 10) (column 42) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 354) (line 10) (column 42) (len 1)))))
  )
  (root (package (name "AnalysisCaseUsagePrefix") (body brace (metadata-def (name "Tag") (abstract false) (specializes none) (body semicolon)) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "plain") (type (ref r0)) (subsets none) (redefines none)) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (name "packageAnalysis") (type (ref r1)) (subsets none) (redefines none)) (analysis-case-usage (prefix (direction in) (derived true) (variance abstract) (constant true) (reference true) (individual true) (portion snapshot) (extensions (ref r2))) (name "allSlots") (type (ref r3)) (subsets none) (redefines none)) (part-def (name "InPartDefinition") (modifiers) (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (name "partDefinitionAnalysis") (type (ref r4)) (subsets none) (redefines none)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inPartUsage") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (name "partUsageAnalysis") (type (ref r5)) (subsets none) (redefines none)))) (case-def (modifiers)))))
)
~~~
