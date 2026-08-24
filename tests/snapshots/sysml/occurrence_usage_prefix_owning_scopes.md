# META
~~~sexpr
(snapshot (type semantic) (description "The shared OccurrenceUsagePrefix in every scope this parser dispatches a migrated family from. `OccurrenceUsage`, `IndividualUsage`, `PortionUsage` and `ItemUsage` reach a body through StructureUsageMember; `SatisfyRequirementUsage` reaches it through BehaviorUsageMember, so both are ordinary members of every definition and usage body. Each scope carries a materially different prefix so a scope that silently dropped the prefix, or that never dispatched a prefixed member at all, shows up here rather than in a corpus run."))
~~~
# SOURCE
~~~sysml
package OccurrencePrefixScopes {
    metadata def Tag;
    ref individual snapshot occurrence atPackageScope;
    #Tag satisfy PackageRequirement by subjectPart;
    part def InPartDef {
        derived ref individual timeslice occurrence inPartDef;
        abstract #Tag satisfy PartDefRequirement;
    }
    part inPartUsage {
        in constant ref occurrence inPartUsageOccurrence;
        variation #Tag satisfy PartUsageRequirement;
    }
    occurrence inOccurrenceBody {
        out derived ref individual occurrence nested;
        constant #Tag satisfy OccurrenceRequirement;
        ref individual item nestedItem;
    }
    action def InActionDef {
        in abstract ref individual snapshot occurrence inActionDef;
        derived ref item inActionDefItem;
    }
    action inActionUsage {
        out variation ref timeslice occurrence inActionUsage;
    }
    item def InItemDef {
        constant ref individual occurrence inItemBody;
        abstract ref item inItemBodyItem;
    }
    connection def InConnectionDef {
        derived constant ref occurrence inConnectionDef;
    }
    view def InViewDef {
        ref individual snapshot satisfy ViewDefRequirement;
    }
    view inViewUsage {
        derived abstract satisfy ViewRequirement by subjectPart;
    }
    requirement def InRequirementDef {
        in ref individual timeslice assert not satisfy NestedRequirement by subjectPart;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "occurrence_usage_prefix_owning_scopes.md"
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
    (reference r0 (scope relative) (span (offset 115) (line 4) (column 6) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 115) (line 4) (column 6) (len 3)))))
    (reference r1 (scope relative) (span (offset 127) (line 4) (column 18) (len 18)) (segments (segment 0 (token "PackageRequirement") (name "PackageRequirement") (separator none) (span (offset 127) (line 4) (column 18) (len 18)))))
    (reference r2 (scope relative) (span (offset 149) (line 4) (column 40) (len 11)) (segments (segment 0 (token "subjectPart") (name "subjectPart") (separator none) (span (offset 149) (line 4) (column 40) (len 11)))))
    (reference r3 (scope relative) (span (offset 268) (line 7) (column 19) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 268) (line 7) (column 19) (len 3)))))
    (reference r4 (scope relative) (span (offset 280) (line 7) (column 31) (len 18)) (segments (segment 0 (token "PartDefRequirement") (name "PartDefRequirement") (separator none) (span (offset 280) (line 7) (column 31) (len 18)))))
    (reference r5 (scope relative) (span (offset 406) (line 11) (column 20) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 406) (line 11) (column 20) (len 3)))))
    (reference r6 (scope relative) (span (offset 418) (line 11) (column 32) (len 20)) (segments (segment 0 (token "PartUsageRequirement") (name "PartUsageRequirement") (separator none) (span (offset 418) (line 11) (column 32) (len 20)))))
    (reference r7 (scope relative) (span (offset 552) (line 15) (column 19) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 552) (line 15) (column 19) (len 3)))))
    (reference r8 (scope relative) (span (offset 564) (line 15) (column 31) (len 21)) (segments (segment 0 (token "OccurrenceRequirement") (name "OccurrenceRequirement") (separator none) (span (offset 564) (line 15) (column 31) (len 21)))))
    (reference r9 (scope relative) (span (offset 1166) (line 33) (column 41) (len 18)) (segments (segment 0 (token "ViewDefRequirement") (name "ViewDefRequirement") (separator none) (span (offset 1166) (line 33) (column 41) (len 18)))))
    (reference r10 (scope relative) (span (offset 1248) (line 36) (column 34) (len 15)) (segments (segment 0 (token "ViewRequirement") (name "ViewRequirement") (separator none) (span (offset 1248) (line 36) (column 34) (len 15)))))
    (reference r11 (scope relative) (span (offset 1267) (line 36) (column 53) (len 11)) (segments (segment 0 (token "subjectPart") (name "subjectPart") (separator none) (span (offset 1267) (line 36) (column 53) (len 11)))))
    (reference r12 (scope relative) (span (offset 1380) (line 39) (column 56) (len 17)) (segments (segment 0 (token "NestedRequirement") (name "NestedRequirement") (separator none) (span (offset 1380) (line 39) (column 56) (len 17)))))
    (reference r13 (scope relative) (span (offset 1401) (line 39) (column 77) (len 11)) (segments (segment 0 (token "subjectPart") (name "subjectPart") (separator none) (span (offset 1401) (line 39) (column 77) (len 11)))))
  )
  (root (package (name "OccurrencePrefixScopes") (body brace (metadata-def (name "Tag") (abstract false) (specializes none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion snapshot) (extensions)) (declaration "atPackageScope") (short-name none) (target none) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r0))) (visibility none) (assert false) (negated false) (requirement (reference (ref r1))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r2)) (body semicolon)) (part-def (name "InPartDef") (modifiers) (body brace (occurrence (prefix (direction none) (derived true) (variance none) (constant false) (reference true) (individual true) (portion timeslice) (extensions)) (declaration "inPartDef") (short-name none) (target none) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions (ref r3))) (visibility none) (assert false) (negated false) (requirement (reference (ref r4))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "inPartUsage") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction in) (derived false) (variance none) (constant true) (reference true) (individual false) (portion none) (extensions)) (declaration "inPartUsageOccurrence") (short-name none) (target none) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions (ref r5))) (visibility none) (assert false) (negated false) (requirement (reference (ref r6))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "inOccurrenceBody") (short-name none) (target none) (body brace (occurrence (prefix (direction out) (derived true) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration "nested") (short-name none) (target none) (body semicolon)) (satisfy (prefix (direction none) (derived false) (variance none) (constant true) (reference false) (individual false) (portion none) (extensions (ref r7))) (visibility none) (assert false) (negated false) (requirement (reference (ref r8))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration "nestedItem") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (action-def (name "InActionDef") (modifiers) (specializes none) (body brace (occurrence-usage (prefix (direction in) (derived false) (variance abstract) (constant false) (reference true) (individual true) (portion snapshot) (extensions))) (item-usage (prefix (direction none) (derived true) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration "inActionDefItem") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (action-usage (name "inActionUsage") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (occurrence (prefix (direction out) (derived false) (variance variation) (constant false) (reference true) (individual false) (portion timeslice) (extensions)) (declaration "inActionUsage") (short-name none) (target none) (body semicolon)))) (item-def (name "InItemDef") (modifiers) (individual false) (specializes none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant true) (reference true) (individual true) (portion none) (extensions)) (declaration "inItemBody") (short-name none) (target none) (body semicolon)) (ref (name "inItemBodyItem") (short-name none) (prefix (direction none) (derived false) (usage-prefix abstract) (constant false)) (extensions) (kind item) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)))) (connection-def (name "InConnectionDef") (modifiers) (role ordinary) (specializes none) (body brace (occurrence (prefix (direction none) (derived true) (variance none) (constant true) (reference true) (individual false) (portion none) (extensions)) (declaration "inConnectionDef") (short-name none) (target none) (body semicolon)))) (view-def (name "InViewDef") (short-name none) (modifiers) (specializes none) (body brace (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion snapshot) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r9))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)))) (view (name "inViewUsage") (short-name none) (type none) (body brace (satisfy (prefix (direction none) (derived true) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r10))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r11)) (body semicolon)))) (requirement-def (name "InRequirementDef") (modifiers) (body brace (satisfy (prefix (direction in) (derived false) (variance none) (constant false) (reference true) (individual true) (portion timeslice) (extensions)) (visibility none) (assert true) (negated true) (requirement (reference (ref r12))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r13)) (body semicolon)))))))
)
~~~
