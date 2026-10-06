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
  (references
    (reference r0 (scope relative) (span (offset 86) (line 4) (column 20) (len 12)) (segments (segment 0 (token "AnalysisType") (name "AnalysisType") (separator none) (span (offset 86) (line 4) (column 20) (len 12)))))
    (reference r1 (scope relative) (span (offset 207) (line 8) (column 19) (len 11)) (segments (segment 0 (token "ConcernType") (name "ConcernType") (separator none) (span (offset 207) (line 8) (column 19) (len 11)))))
    (reference r2 (scope relative) (span (offset 448) (line 17) (column 20) (len 12)) (segments (segment 0 (token "MetadataType") (name "MetadataType") (separator none) (span (offset 448) (line 17) (column 20) (len 12)))))
    (reference r3 (scope relative) (span (offset 485) (line 18) (column 24) (len 12)) (segments (segment 0 (token "MetadataType") (name "MetadataType") (separator none) (span (offset 485) (line 18) (column 24) (len 12)))))
    (reference r4 (scope relative) (span (offset 515) (line 19) (column 17) (len 9)) (segments (segment 0 (token "StateType") (name "StateType") (separator none) (span (offset 515) (line 19) (column 17) (len 9)))))
    (reference r5 (scope relative) (span (offset 762) (line 26) (column 35) (len 9)) (segments (segment 0 (token "inherited") (name "inherited") (separator none) (span (offset 762) (line 26) (column 35) (len 9)))))
  )
  (root (package (name "ShortNameOnly") (body brace (case-usage (name none) (short-name "C")) (case-usage (name none) (short-name "COnly")) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name none) (short-name "A") (type (ref r0)) (subsets none) (redefines none)) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name none) (short-name "AOnly") (type none) (subsets none) (redefines none)) (verification-case-usage (name none) (short-name "V")) (verification-case-usage (name none) (short-name "VOnly")) (concern-usage (name none) (short-name "R") (visibility none) (abstract false) (individual false) (definition false) (type (ref r1)) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (concern-usage (name none) (short-name "ROnly") (visibility none) (abstract false) (individual false) (definition false) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (concern-usage (name none) (short-name "RD") (visibility none) (abstract false) (individual false) (definition true) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (viewpoint-usage (name none) (short-name "VP")) (viewpoint-usage (name none) (short-name "VPOnly")) (enumeration-usage (name none) (short-name "E")) (enumeration-usage (name none) (short-name "EOnly")) (allocation-usage (name none) (short-name "AL")) (allocation-usage (name none) (short-name "ALOnly")) (metadata-usage (declaration-name none) (short-name "M") (type (ref r2)) (about) (body semicolon)) (metadata-usage (declaration-name none) (short-name "MOnly") (type (ref r3)) (about) (body semicolon)) (state-usage (name none) (short-name "S") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (state-usage (name none) (short-name "SOnly") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (verification-case-usage (name "named") (short-name "named-short")) (verification-case-usage (name "ordinary") (short-name none)) (verification-case-usage (name none) (short-name "body-only")) (state-usage (name none) (short-name "inherited-state") (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (body semicolon)) (part-def (name "Owner") (modifiers) (body brace (exhibit (declaration none) (short-name "ES") (state none)) (exhibit (declaration none) (short-name "ESOnly") (state none)) (exhibit (declaration "named") (short-name "named-exhibit") (state none)))))))
)
~~~
