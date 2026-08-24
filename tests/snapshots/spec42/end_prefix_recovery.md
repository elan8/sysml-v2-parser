# META
~~~sexpr
(snapshot (type semantic) (description "The pin makes a direction or BasicFeaturePrefix restriction beside `end` unauthorable: FeaturePrefix = EndFeaturePrefix | BasicFeaturePrefix (KerML BNF 573-588; Pilot KerML.xtext 510-526) is a choice, where EndFeaturePrefix spells only `const`? `end` and the direction/derived/abstract/composite/portion/var slots live only in BasicFeaturePrefix. Both orders are reported as `end_feature_invalid_prefix`, naming the offending keyword, with exact malformed spans; `const end`, later feature siblings, and an `abstract struct` classifier remain typed (spec42 Gaps 59/67)."))
~~~
# SOURCE
~~~sysml
package EndPrefixRecovery {
    connection def DirectionBeforeEnd {
        in end port rejected : Port;
        end accepted : Port;
    }
    connection def DirectionAfterEnd {
        end in port rejected : Port;
        end accepted : Port;
    }
    type KermlDirectionBeforeEnd {
        in end feature rejected : T;
        end feature accepted : T;
    }
    type KermlLegalPrefixes {
        const end feature constEnd;
        in feature directed : T;
    }
    type KermlRestrictionBeforeEnd {
        derived end feature derivedRejected : T;
        end derived feature derivedRejectedAfterEnd : T;
        abstract end feature abstractRejected : T;
        end abstract feature abstractRejectedAfterEnd : T;
        composite end feature compositeRejected : T;
        end composite feature compositeRejectedAfterEnd : T;
        portion end feature portionRejected : T;
        end portion feature portionRejectedAfterEnd : T;
        var end feature varRejected : T;
        end var feature varRejectedAfterEnd : T;
        const end feature constEnd;
        feature later : T;
        abstract struct NestedClassifier {
            feature nested;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "end_prefix_recovery.md"
    (diagnostics
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 76) (line 3) (column 9) (len 37)) (message "the direction `in` cannot precede `end`: every production that spells both writes `end` first"))
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 187) (line 7) (column 9) (len 37)) (message "`end port` cannot carry the direction `in`: `end` and the prefix keywords are exclusive alternatives of one choice (SysML BNF 298, KerML BNF 584)"))
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 294) (line 11) (column 9) (len 37)) (message "the direction `in` cannot precede `end`: every production that spells both writes `end` first"))
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 513) (line 19) (column 9) (len 49)) (message "the restriction modifier `derived` cannot precede `end`: every production that spells both writes `end` first"))
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 562) (line 20) (column 9) (len 57)) (message "`end feature` cannot carry the restriction modifier `derived`: `end` and the prefix keywords are exclusive alternatives of one choice (SysML BNF 298, KerML BNF 584)"))
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 619) (line 21) (column 9) (len 51)) (message "the restriction modifier `abstract` cannot precede `end`: every production that spells both writes `end` first"))
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 670) (line 22) (column 9) (len 59)) (message "`end feature` cannot carry the restriction modifier `abstract`: `end` and the prefix keywords are exclusive alternatives of one choice (SysML BNF 298, KerML BNF 584)"))
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 729) (line 23) (column 9) (len 53)) (message "the restriction modifier `composite` cannot precede `end`: every production that spells both writes `end` first"))
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 782) (line 24) (column 9) (len 61)) (message "`end feature` cannot carry the restriction modifier `composite`: `end` and the prefix keywords are exclusive alternatives of one choice (SysML BNF 298, KerML BNF 584)"))
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 843) (line 25) (column 9) (len 49)) (message "the restriction modifier `portion` cannot precede `end`: every production that spells both writes `end` first"))
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 892) (line 26) (column 9) (len 57)) (message "`end feature` cannot carry the restriction modifier `portion`: `end` and the prefix keywords are exclusive alternatives of one choice (SysML BNF 298, KerML BNF 584)"))
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 949) (line 27) (column 9) (len 41)) (message "the restriction modifier `var` cannot precede `end`: every production that spells both writes `end` first"))
      (diagnostic (code "end_feature_invalid_prefix") (severity error) (category parseerror) (span (offset 990) (line 28) (column 9) (len 49)) (message "`end feature` cannot carry the restriction modifier `var`: `end` and the prefix keywords are exclusive alternatives of one choice (SysML BNF 298, KerML BNF 584)"))
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
    (reference r0 (scope relative) (span (offset 128) (line 4) (column 24) (len 4)) (segments (segment 0 (token "Port") (name "Port") (separator none) (span (offset 128) (line 4) (column 24) (len 4)))))
    (reference r1 (scope relative) (span (offset 239) (line 8) (column 24) (len 4)) (segments (segment 0 (token "Port") (name "Port") (separator none) (span (offset 239) (line 8) (column 24) (len 4)))))
    (reference r2 (scope relative) (span (offset 354) (line 12) (column 32) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 354) (line 12) (column 32) (len 1)))))
    (reference r3 (scope relative) (span (offset 459) (line 16) (column 31) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 459) (line 16) (column 31) (len 1)))))
    (reference r4 (scope relative) (span (offset 1091) (line 30) (column 25) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 1091) (line 30) (column 25) (len 1)))))
  )
  (root (package (name "EndPrefixRecovery") (body brace (connection-def (name "DirectionBeforeEnd") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "end_feature_invalid_prefix") (found "in end port rejected : Port;") (span (offset 76) (line 3) (column 9) (len 37))) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "accepted") (span (offset 117) (line 4) (column 13) (len 8)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (references none) (multiplicity none) (redefines none) (crosses none)))) (connection-def (name "DirectionAfterEnd") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "end_feature_invalid_prefix") (found "end in port rejected : Port;") (span (offset 187) (line 7) (column 9) (len 37))) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "accepted") (span (offset 228) (line 8) (column 13) (len 8)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (multiplicity none) (redefines none) (crosses none)))) (kerml-classifier (keyword type) (abstract false) (name "KermlDirectionBeforeEnd") (specializes none) (body brace (malformed (code "end_feature_invalid_prefix") (found "in end feature rejected : T;") (span (offset 294) (line 11) (column 9) (len 37))) (kerml-feature (prefix (head end) (constant false) (cross none) (metadata)) (kind feature) (member false) (all false) (name "accepted") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "KermlLegalPrefixes") (specializes none) (body brace (kerml-feature (prefix (head end) (constant true) (cross none) (metadata)) (kind feature) (member false) (all false) (name "constEnd") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "directed") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "KermlRestrictionBeforeEnd") (specializes none) (body brace (malformed (code "end_feature_invalid_prefix") (found "derived end feature derivedRejected : T;") (span (offset 513) (line 19) (column 9) (len 49))) (malformed (code "end_feature_invalid_prefix") (found "end derived feature derivedRejectedAfterEnd : T;") (span (offset 562) (line 20) (column 9) (len 57))) (malformed (code "end_feature_invalid_prefix") (found "abstract end feature abstractRejected : T;") (span (offset 619) (line 21) (column 9) (len 51))) (malformed (code "end_feature_invalid_prefix") (found "end abstract feature abstractRejectedAfterEnd : T;") (span (offset 670) (line 22) (column 9) (len 59))) (malformed (code "end_feature_invalid_prefix") (found "composite end feature compositeRejected : T;") (span (offset 729) (line 23) (column 9) (len 53))) (malformed (code "end_feature_invalid_prefix") (found "end composite feature compositeRejectedAfterEnd : T;") (span (offset 782) (line 24) (column 9) (len 61))) (malformed (code "end_feature_invalid_prefix") (found "portion end feature portionRejected : T;") (span (offset 843) (line 25) (column 9) (len 49))) (malformed (code "end_feature_invalid_prefix") (found "end portion feature portionRejectedAfterEnd : T;") (span (offset 892) (line 26) (column 9) (len 57))) (malformed (code "end_feature_invalid_prefix") (found "var end feature varRejected : T;") (span (offset 949) (line 27) (column 9) (len 41))) (malformed (code "end_feature_invalid_prefix") (found "end var feature varRejectedAfterEnd : T;") (span (offset 990) (line 28) (column 9) (len 49))) (kerml-feature (prefix (head end) (constant true) (cross none) (metadata)) (kind feature) (member false) (all false) (name "constEnd") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "later") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)) (kerml-classifier (keyword struct) (abstract true) (name "NestedClassifier") (specializes none) (body brace (kerml-feature (prefix (head basic) (direction none) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind feature) (member false) (all false) (name "nested") (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (relationships) (value none) (body semicolon)))))))))
)
~~~
