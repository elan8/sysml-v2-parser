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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 77) (line 3) (column 22) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 77) (line 3) (column 22) (len 1)))))
    (reference r1 (scope relative) (span (offset 115) (line 4) (column 36) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 115) (line 4) (column 36) (len 1)))))
    (reference r2 (scope relative) (span (offset 176) (line 5) (column 59) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 176) (line 5) (column 59) (len 3)))))
    (reference r3 (scope relative) (span (offset 200) (line 5) (column 83) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 200) (line 5) (column 83) (len 1)))))
    (reference r4 (scope relative) (span (offset 281) (line 7) (column 47) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 281) (line 7) (column 47) (len 1)))))
    (reference r5 (scope relative) (span (offset 354) (line 10) (column 42) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 354) (line 10) (column 42) (len 1)))))
    (reference r6 (scope relative) (span (offset 429) (line 13) (column 41) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 429) (line 13) (column 41) (len 1)))))
    (reference r7 (scope relative) (span (offset 462) (line 14) (column 31) (len 4)) (segments (segment 0 (token "Case") (name "Case") (separator none) (span (offset 462) (line 14) (column 31) (len 4)))))
    (reference r8 (scope relative) (span (offset 514) (line 15) (column 47) (len 16)) (segments (segment 0 (token "VerificationCase") (name "VerificationCase") (separator none) (span (offset 514) (line 15) (column 47) (len 16)))))
  )
  (root (package (name "AnalysisCaseUsagePrefix") (body brace (metadata-def (name "Tag") (abstract false) (specializes none) (body semicolon)) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "plain") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (body semicolon)) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (name "packageAnalysis") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (body semicolon)) (analysis-case-usage (prefix (direction in) (derived true) (variance abstract) (constant true) (reference true) (individual true) (portion snapshot) (extensions (ref r2))) (name "allSlots") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (body semicolon)) (part-def (name "InPartDefinition") (modifiers) (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (name "partDefinitionAnalysis") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inPartUsage") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (name "partUsageAnalysis") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (body semicolon)))) (case-def (name "InCaseBody") (modifiers) (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (name "caseBodyAnalysis") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (body semicolon)) (ref (name "legacyCase") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind case) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)) (ref (name "legacyVerification") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind verification) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)))))))
)
~~~
