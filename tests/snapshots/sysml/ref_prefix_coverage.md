# META
~~~sexpr
(snapshot (type semantic) (description "BNF RefPrefix -- direction, derived, abstract or variation, constant -- is accepted ahead of any usage keyword and survives emission in the one order the grammar allows. Ref declarations project the whole chain, so a dropped keyword is visible in the AST and not only in the emitted text; item usages still project as bare markers, so the emitted form is what covers those."))
~~~
# SOURCE
~~~sysml
package RefPrefixCoverage {
    action def A {
        abstract ref :>> trailerHitch;
        derived ref action deferred : ActionUsage :> Metadata::metadataItems;
        derived abstract constant ref action everything : T;
        in ref inbound : T;
    }
    metadata def M {
        derived ref item receiverArgument : Expression[0..1] :> Metadata::metadataItems;
        derived ref item 'action' : ActionUsage[0..*] ordered subsets step, usage subsets Metadata::metadataItems;
        derived item ownedActorParameter :>> ownedMemberParameter : PartUsage[1];
        derived abstract constant ref item everything : T;
    }
    part def D {
        constant item constantItem : T;
        variation item variationItem : T;
        derived abstract constant ref attribute everyAttribute : T;
        derived abstract constant ref part everyPart : T;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ref_prefix_coverage.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package RefPrefixCoverage {
    action def A {
        abstract ref :>> trailerHitch;
        derived ref action deferred : ActionUsage :> Metadata::metadataItems;
        derived abstract constant ref action everything : T;
        in ref inbound : T;
    }
    metadata def M {
        derived ref item receiverArgument : Expression[0..1] :> Metadata::metadataItems;
        derived ref item 'action' : ActionUsage[0..*] ordered :> step, usage, Metadata::metadataItems;
        derived item ownedActorParameter :>> ownedMemberParameter : PartUsage[1];
        derived abstract constant ref item everything : T;
    }
    part def D {
        constant item constantItem : T;
        variation item variationItem : T;
        derived abstract constant ref attribute everyAttribute : T;
        derived abstract constant ref part everyPart : T;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 72) (line 3) (column 26) (len 12)) (segments (segment 0 (token "trailerHitch") (name "trailerHitch") (separator none) (span (offset 72) (line 3) (column 26) (len 12)))))
    (reference r1 (scope relative) (span (offset 124) (line 4) (column 39) (len 11)) (segments (segment 0 (token "ActionUsage") (name "ActionUsage") (separator none) (span (offset 124) (line 4) (column 39) (len 11)))))
    (reference r2 (scope relative) (span (offset 139) (line 4) (column 54) (len 23)) (segments (segment 0 (token "Metadata") (name "Metadata") (separator none) (span (offset 139) (line 4) (column 54) (len 8))) (segment 1 (token "metadataItems") (name "metadataItems") (separator colon-colon) (span (offset 149) (line 4) (column 64) (len 13)))))
    (reference r3 (scope relative) (span (offset 222) (line 5) (column 59) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 222) (line 5) (column 59) (len 1)))))
    (reference r4 (scope relative) (span (offset 250) (line 6) (column 26) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 250) (line 6) (column 26) (len 1)))))
    (reference r5 (scope relative) (span (offset 795) (line 17) (column 66) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 795) (line 17) (column 66) (len 1)))))
  )
  (root (package (name "RefPrefixCoverage") (body brace (action-def (name "A") (specializes none) (body brace (ref (name "") (short-name none) (prefix (direction none) (derived false) (usage-prefix abstract) (constant false)) (kind none) (typing none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (subsets none) (body semicolon)) (ref (name "deferred") (short-name none) (prefix (direction none) (derived true) (usage-prefix none) (constant false)) (kind action) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (redefines none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (body semicolon)) (ref (name "everything") (short-name none) (prefix (direction none) (derived true) (usage-prefix abstract) (constant true)) (kind action) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (redefines none) (subsets none) (body semicolon)) (in-out (direction in) (reference true) (declaration "inbound") (subsets none) (type (ref r4)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 233) (line 6) (column 9) (len 19))))) (metadata-def) (part-def (name "D") (body brace (item-usage (prefix (direction none) (derived false) (variance none) (constant true) (reference false) (individual false) (portion none) (extensions)) (declaration "constantItem")) (item-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "variationItem")) (attribute-usage (declaration-name "everyAttribute") (direction none) (derived true) (usage-prefix abstract) (constant true) (reference true) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage))))))
)
~~~
