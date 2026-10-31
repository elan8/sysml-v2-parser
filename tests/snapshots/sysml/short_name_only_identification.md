# META
~~~sexpr
(snapshot (type semantic) (description "Identification permits short names independently of regular names. Typed and standalone case, analysis, verification, concern, viewpoint, enum, allocation, metadata, state, and exhibit-state declarations retain their short names through formatting and reparsing, alongside named controls."))
~~~
# SOURCE
~~~sysml
package ShortNameOnly {
    case <C> : CaseType;
    case <COnly>;
    analysis <A> : AnalysisType;
    analysis <AOnly>;
    verification <V> : VerificationType;
    verification <VOnly>;
    concern <R> : ConcernType;
    concern <ROnly>;
    concern def <RD>;
    viewpoint <VP> : ViewpointType;
    viewpoint <VPOnly>;
    enum <E> : EnumType;
    enum <EOnly>;
    allocation <AL> : AllocationType;
    allocation <ALOnly>;
    metadata <M> : MetadataType;
    metadata <MOnly> : MetadataType;
    state <S> : StateType;
    state <SOnly>;
    verification <'named-short'> named : VerificationType;
    verification ordinary : VerificationType;
    verification <'body-only'> {
        objective <'objective-only'> ;
    }
    state <'inherited-state'> :>> inherited;
    part def Owner {
        exhibit state <ES> : StateType;
        exhibit state <ESOnly>;
        exhibit state <'named-exhibit'> named : StateType;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "short_name_only_identification.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ShortNameOnly {
    case <C> : CaseType;
    case <COnly>;
    analysis <A> : AnalysisType;
    analysis <AOnly>;
    verification <V> : VerificationType;
    verification <VOnly>;
    concern <R> : ConcernType;
    concern <ROnly>;
    concern def <RD>;
    viewpoint <VP> : ViewpointType;
    viewpoint <VPOnly>;
    enum <E> : EnumType;
    enum <EOnly>;
    allocation <AL> : AllocationType;
    allocation <ALOnly>;
    metadata <M> : MetadataType;
    metadata <MOnly> : MetadataType;
    state <S> : StateType;
    state <SOnly>;
    verification <'named-short'> named : VerificationType;
    verification ordinary : VerificationType;
    verification <'body-only'> {
        objective <'objective-only'>;
    }
    state <'inherited-state'> :>> inherited;
    part def Owner {
        exhibit state <ES> : StateType;
        exhibit state <ESOnly>;
        exhibit state <'named-exhibit'> named : StateType;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 39) (line 2) (column 16) (len 8)) (segments (segment 0 (token "CaseType") (name "CaseType") (separator none) (span (offset 39) (line 2) (column 16) (len 8)))))
    (reference r1 (scope relative) (span (offset 86) (line 4) (column 20) (len 12)) (segments (segment 0 (token "AnalysisType") (name "AnalysisType") (separator none) (span (offset 86) (line 4) (column 20) (len 12)))))
    (reference r2 (scope relative) (span (offset 145) (line 6) (column 24) (len 16)) (segments (segment 0 (token "VerificationType") (name "VerificationType") (separator none) (span (offset 145) (line 6) (column 24) (len 16)))))
    (reference r3 (scope relative) (span (offset 207) (line 8) (column 19) (len 11)) (segments (segment 0 (token "ConcernType") (name "ConcernType") (separator none) (span (offset 207) (line 8) (column 19) (len 11)))))
    (reference r4 (scope relative) (span (offset 284) (line 11) (column 22) (len 13)) (segments (segment 0 (token "ViewpointType") (name "ViewpointType") (separator none) (span (offset 284) (line 11) (column 22) (len 13)))))
    (reference r5 (scope relative) (span (offset 338) (line 13) (column 16) (len 8)) (segments (segment 0 (token "EnumType") (name "EnumType") (separator none) (span (offset 338) (line 13) (column 16) (len 8)))))
    (reference r6 (scope relative) (span (offset 388) (line 15) (column 23) (len 14)) (segments (segment 0 (token "AllocationType") (name "AllocationType") (separator none) (span (offset 388) (line 15) (column 23) (len 14)))))
    (reference r7 (scope relative) (span (offset 448) (line 17) (column 20) (len 12)) (segments (segment 0 (token "MetadataType") (name "MetadataType") (separator none) (span (offset 448) (line 17) (column 20) (len 12)))))
    (reference r8 (scope relative) (span (offset 485) (line 18) (column 24) (len 12)) (segments (segment 0 (token "MetadataType") (name "MetadataType") (separator none) (span (offset 485) (line 18) (column 24) (len 12)))))
    (reference r9 (scope relative) (span (offset 515) (line 19) (column 17) (len 9)) (segments (segment 0 (token "StateType") (name "StateType") (separator none) (span (offset 515) (line 19) (column 17) (len 9)))))
    (reference r10 (scope relative) (span (offset 586) (line 21) (column 42) (len 16)) (segments (segment 0 (token "VerificationType") (name "VerificationType") (separator none) (span (offset 586) (line 21) (column 42) (len 16)))))
    (reference r11 (scope relative) (span (offset 632) (line 22) (column 29) (len 16)) (segments (segment 0 (token "VerificationType") (name "VerificationType") (separator none) (span (offset 632) (line 22) (column 29) (len 16)))))
    (reference r12 (scope relative) (span (offset 762) (line 26) (column 35) (len 9)) (segments (segment 0 (token "inherited") (name "inherited") (separator none) (span (offset 762) (line 26) (column 35) (len 9)))))
  )
  (root (package (visibility none) (name "ShortNameOnly") (body brace (case-usage (declaration (name none) (short-name "C") (abstract false) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none)) (body semicolon)) (case-usage (declaration (name none) (short-name "COnly") (abstract false) (type none) (subsets none)) (body semicolon)) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name none) (short-name "A") (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (body semicolon)) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name none) (short-name "AOnly") (type none) (subsets none) (redefines none) (body semicolon)) (verification-case-usage (declaration (name none) (short-name "V") (abstract false) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none)) (body semicolon)) (verification-case-usage (declaration (name none) (short-name "VOnly") (abstract false) (type none) (subsets none)) (body semicolon)) (concern-usage (name none) (short-name "R") (visibility none) (abstract false) (individual false) (definition false) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (concern-usage (name none) (short-name "ROnly") (visibility none) (abstract false) (individual false) (definition false) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (concern-usage (name none) (short-name "RD") (visibility none) (abstract false) (individual false) (definition true) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (viewpoint-usage (name none) (short-name "VP") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (body semicolon)) (viewpoint-usage (name none) (short-name "VPOnly") (typing none) (body semicolon)) (enumeration-usage (name none) (short-name "E") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (body semicolon)) (enumeration-usage (name none) (short-name "EOnly") (typing none) (body semicolon)) (allocation-usage (name none) (short-name "AL") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (ends) (body semicolon)) (allocation-usage (name none) (short-name "ALOnly") (typing none) (ends) (body semicolon)) (metadata-usage (declaration-name none) (short-name "M") (type (ref r7)) (about) (body semicolon)) (metadata-usage (declaration-name none) (short-name "MOnly") (type (ref r8)) (about) (body semicolon)) (state-usage (name none) (short-name "S") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (state-usage (name none) (short-name "SOnly") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (verification-case-usage (declaration (name "named") (short-name "named-short") (abstract false) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (subsets none)) (body semicolon)) (verification-case-usage (declaration (name "ordinary") (short-name none) (abstract false) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (subsets none)) (body semicolon)) (verification-case-usage (declaration (name none) (short-name "body-only") (abstract false) (type none) (subsets none)) (body brace (objective (visibility none) (name none) (type none) (body semicolon)))) (state-usage (name none) (short-name "inherited-state") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (body semicolon)) (part-def (name "Owner") (modifiers) (body brace (exhibit (declaration none) (short-name "ES") (state none)) (exhibit (declaration none) (short-name "ESOnly") (state none)) (exhibit (declaration "named") (short-name "named-exhibit") (state none)))))))
)
~~~
