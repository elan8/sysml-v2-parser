# META
~~~sexpr
(snapshot (type semantic) (description "Every scope this parser dispatches SatisfyRequirementUsage from, holding the same usage. SatisfyRequirementUsage is a BehaviorUsageElement, so it reaches package bodies, part definition and usage bodies, occurrence bodies, view definition and view usage bodies, and requirement bodies -- including the RequirementBody a satisfy usage owns itself. The view usage body used to own a separate viewpoint-only node that could represent neither the prefixes, the `by` clause, the inline declaration, nor a requirement-body member; it holds the one production here."))
~~~
# SOURCE
~~~sysml
package SatisfyOwningScopes {
    requirement def Spec;
    part target;
    satisfy Spec by target;
    assert not satisfy Spec by target;
    part def InPartDefinition {
        satisfy Spec by target;
    }
    part inPartUsage {
        satisfy Spec by target;
    }
    occurrence inOccurrence {
        satisfy Spec by target;
    }
    view def InViewDefinition {
        satisfy Spec by target;
    }
    view inViewUsage : InViewDefinition {
        satisfy Spec;
        satisfy Spec by target;
        satisfy requirement declared : Spec by target {
            doc
            /* a view-body satisfy owns a RequirementBody like every other one */
        }
    }
    requirement def InRequirementDefinition {
        satisfy Spec by target;
    }
    part nestedSatisfyBody {
        satisfy Spec by target {
            satisfy Spec by target;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "satisfy_requirement_usage_owning_scopes.md"
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
    (reference r0 (scope relative) (span (offset 85) (line 4) (column 13) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 85) (line 4) (column 13) (len 4)))))
    (reference r1 (scope relative) (span (offset 93) (line 4) (column 21) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 93) (line 4) (column 21) (len 6)))))
    (reference r2 (scope relative) (span (offset 124) (line 5) (column 24) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 124) (line 5) (column 24) (len 4)))))
    (reference r3 (scope relative) (span (offset 132) (line 5) (column 32) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 132) (line 5) (column 32) (len 6)))))
    (reference r4 (scope relative) (span (offset 188) (line 7) (column 17) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 188) (line 7) (column 17) (len 4)))))
    (reference r5 (scope relative) (span (offset 196) (line 7) (column 25) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 196) (line 7) (column 25) (len 6)))))
    (reference r6 (scope relative) (span (offset 249) (line 10) (column 17) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 249) (line 10) (column 17) (len 4)))))
    (reference r7 (scope relative) (span (offset 257) (line 10) (column 25) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 257) (line 10) (column 25) (len 6)))))
    (reference r8 (scope relative) (span (offset 432) (line 18) (column 24) (len 16)) (segments (segment 0 (token "InViewDefinition") (name "InViewDefinition") (separator none) (span (offset 432) (line 18) (column 24) (len 16)))))
    (reference r9 (scope relative) (span (offset 467) (line 19) (column 17) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 467) (line 19) (column 17) (len 4)))))
    (reference r10 (scope relative) (span (offset 489) (line 20) (column 17) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 489) (line 20) (column 17) (len 4)))))
    (reference r11 (scope relative) (span (offset 497) (line 20) (column 25) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 497) (line 20) (column 25) (len 6)))))
    (reference r12 (scope relative) (span (offset 544) (line 21) (column 40) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 544) (line 21) (column 40) (len 4)))))
    (reference r13 (scope relative) (span (offset 552) (line 21) (column 48) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 552) (line 21) (column 48) (len 6)))))
    (reference r14 (scope relative) (span (offset 737) (line 27) (column 17) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 737) (line 27) (column 17) (len 4)))))
    (reference r15 (scope relative) (span (offset 745) (line 27) (column 25) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 745) (line 27) (column 25) (len 6)))))
    (reference r16 (scope relative) (span (offset 804) (line 30) (column 17) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 804) (line 30) (column 17) (len 4)))))
    (reference r17 (scope relative) (span (offset 812) (line 30) (column 25) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 812) (line 30) (column 25) (len 6)))))
    (reference r18 (scope relative) (span (offset 841) (line 31) (column 21) (len 4)) (segments (segment 0 (token "Spec") (name "Spec") (separator none) (span (offset 841) (line 31) (column 21) (len 4)))))
    (reference r19 (scope relative) (span (offset 849) (line 31) (column 29) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 849) (line 31) (column 29) (len 6)))))
  )
  (root (package (name "SatisfyOwningScopes") (body brace (requirement-def (name "Spec") (body semicolon)) (part-usage (declaration-name "target") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r0))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r1)) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert true) (negated true) (requirement (reference (ref r2))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r3)) (body semicolon)) (part-def (name "InPartDefinition") (body brace (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r4))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r5)) (body semicolon)))) (part-usage (declaration-name "inPartUsage") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r6))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r7)) (body semicolon)))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "inOccurrence") (short-name none) (target none)) (view-def) (view (name "inViewUsage") (short-name none) (type (ref r8)) (body brace (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r9))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r10))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r11)) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (declaration (name "declared") (short-name none))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r13)) (body brace (doc))))) (requirement-def (name "InRequirementDefinition") (body brace (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r14))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r15)) (body semicolon)))) (part-usage (declaration-name "nestedSatisfyBody") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r16))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r17)) (body brace (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r18))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r19)) (body semicolon)))))))))
)
~~~
