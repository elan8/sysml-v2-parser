# META
~~~sexpr
(snapshot (type semantic) (description "SysML's keyword-less DefaultReferenceUsage = ( isEnd ?= 'end' )? RefPrefix UsageDeclaration (SysML BNF 630; reference SysML.xtext:630-633) is the one production that spells `end` beside a RefPrefix, so `end derived x : T;`, `end in x : T;` and `end constant x : T;` are legal and their prefixes are retained with spans. The keyworded forms stay an exclusive choice -- UnextendedUsagePrefix = EndUsagePrefix | BasicUsagePrefix (298) -- so `end derived part p : T;` is reported, as is any modifier written before `end`. This is the spelling that makes validateFeatureEndNoDirection and validateFeatureEndNotDerivedAbstractCompositeOrPortion reachable from textual notation at all (spec42 Gaps 59/67)."))
~~~
# SOURCE
~~~sysml
package EndRefPrefix {
    connection def LegalKeywordless {
        end derived derivedEnd : T;
        end in directedEnd : T;
        end out outboundEnd : T;
        end abstract abstractEnd : T;
        end constant constantEnd : T;
        end plainEnd : T;
    }
    connection def RejectedKeyworded {
        end derived part rejectedPart : T;
        end accepted : T;
    }
    connection def RejectedBeforeEnd {
        derived end rejectedOrder : T;
        end acceptedToo : T;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "end_ref_prefix.md"
    (diagnostics
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 317) (line 11) (column 9) (len 43)) (message "`end part` cannot carry the restriction modifier `derived`: `end` and the prefix keywords are exclusive alternatives of one choice (SysML BNF 298, KerML BNF 584)"))
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 431) (line 15) (column 9) (len 39)) (message "the restriction modifier `derived` cannot precede `end`: every production that spells both writes `end` first"))
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
    (reference r0 (scope relative) (span (offset 94) (line 3) (column 34) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 94) (line 3) (column 34) (len 1)))))
    (reference r1 (scope relative) (span (offset 126) (line 4) (column 30) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 126) (line 4) (column 30) (len 1)))))
    (reference r2 (scope relative) (span (offset 159) (line 5) (column 31) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 159) (line 5) (column 31) (len 1)))))
    (reference r3 (scope relative) (span (offset 197) (line 6) (column 36) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 197) (line 6) (column 36) (len 1)))))
    (reference r4 (scope relative) (span (offset 235) (line 7) (column 36) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 235) (line 7) (column 36) (len 1)))))
    (reference r5 (scope relative) (span (offset 261) (line 8) (column 24) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 261) (line 8) (column 24) (len 1)))))
    (reference r6 (scope relative) (span (offset 375) (line 12) (column 24) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 375) (line 12) (column 24) (len 1)))))
    (reference r7 (scope relative) (span (offset 488) (line 16) (column 27) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 488) (line 16) (column 27) (len 1)))))
  )
  (root (package (name "EndRefPrefix") (body brace (connection-def (name "LegalKeywordless") (modifiers) (role ordinary) (specializes none) (body brace (end (prefix (direction none) (derived true) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "derivedEnd") (span (offset 81) (line 3) (column 21) (len 10)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction in) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "directedEnd") (span (offset 112) (line 4) (column 16) (len 11)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction out) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "outboundEnd") (span (offset 145) (line 5) (column 17) (len 11)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance abstract)) (introducer bare) (short-name none) (identity (declaration (name "abstractEnd") (span (offset 183) (line 6) (column 22) (len 11)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant true) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "constantEnd") (span (offset 221) (line 7) (column 22) (len 11)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "plainEnd") (span (offset 250) (line 8) (column 13) (len 8)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (references none) (multiplicity none) (redefines none) (crosses none)))) (connection-def (name "RejectedKeyworded") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "end_feature_invalid_prefix") (found "end derived part rejectedPart : T;") (span (offset 317) (line 11) (column 9) (len 43))) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "accepted") (span (offset 364) (line 12) (column 13) (len 8)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (references none) (multiplicity none) (redefines none) (crosses none)))) (connection-def (name "RejectedBeforeEnd") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "end_feature_invalid_prefix") (found "derived end rejectedOrder : T;") (span (offset 431) (line 15) (column 9) (len 39))) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "acceptedToo") (span (offset 474) (line 16) (column 13) (len 11)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (references none) (multiplicity none) (redefines none) (crosses none)))))))
)
~~~
