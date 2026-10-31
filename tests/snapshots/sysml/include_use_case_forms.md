# META
~~~sexpr
(snapshot (type semantic) (description "IncludeUseCaseUsage takes an OwnedReferenceSubsetting, which may be a feature chain, or `use case` with a full UsageDeclaration, whose specialization and multiplicity are its own. Both used to fall through to a stray `include` expression followed by a separate member."))
~~~
# SOURCE
~~~sysml
use case def Host {
    include plain;
    include system.uc1;
    include Lib::uc2;
    include use case <'SN'> : UCD;
    include use case <'S2'> named : UCD;
    include use case bare;
    include use case specialized :> plain [1..5];
    include use case <'S3'> tight:>Lib::uc2[2];
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "include_use_case_forms.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
use case def Host {
    include plain;
    include system.uc1;
    include Lib::uc2;
    include use case <'SN'> : UCD;
    include use case <'S2'> named : UCD;
    include use case bare;
    include use case specialized[1..5] :> plain;
    include use case <'S3'> tight[2] :> Lib::uc2;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 32) (line 2) (column 13) (len 5)) (segments (segment 0 (token "plain") (name "plain") (separator none) (span (offset 32) (line 2) (column 13) (len 5)))))
    (reference r1 (scope relative) (span (offset 51) (line 3) (column 13) (len 10)) (segments (segment 0 (token "system") (name "system") (separator none) (span (offset 51) (line 3) (column 13) (len 6))) (segment 1 (token "uc1") (name "uc1") (separator dot) (span (offset 58) (line 3) (column 20) (len 3)))))
    (reference r2 (scope relative) (span (offset 75) (line 4) (column 13) (len 8)) (segments (segment 0 (token "Lib") (name "Lib") (separator none) (span (offset 75) (line 4) (column 13) (len 3))) (segment 1 (token "uc2") (name "uc2") (separator colon-colon) (span (offset 80) (line 4) (column 18) (len 3)))))
    (reference r3 (scope relative) (span (offset 115) (line 5) (column 31) (len 3)) (segments (segment 0 (token "UCD") (name "UCD") (separator none) (span (offset 115) (line 5) (column 31) (len 3)))))
    (reference r4 (scope relative) (span (offset 156) (line 6) (column 37) (len 3)) (segments (segment 0 (token "UCD") (name "UCD") (separator none) (span (offset 156) (line 6) (column 37) (len 3)))))
    (reference r5 (scope relative) (span (offset 224) (line 8) (column 37) (len 5)) (segments (segment 0 (token "plain") (name "plain") (separator none) (span (offset 224) (line 8) (column 37) (len 5)))))
    (reference r6 (scope relative) (span (offset 273) (line 9) (column 36) (len 8)) (segments (segment 0 (token "Lib") (name "Lib") (separator none) (span (offset 273) (line 9) (column 36) (len 3))) (segment 1 (token "uc2") (name "uc2") (separator colon-colon) (span (offset 278) (line 9) (column 41) (len 3)))))
  )
  (root (use-case-def (name "Host") (modifiers) (body brace (include (target (ref r0))) (include (target (ref r1))) (include (target (ref r2))) (include (target (use-case (name none) (short-name "SN") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)))) (include (target (use-case (name "named") (short-name "S2") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)))) (include (target (use-case (name "bare") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)))) (include (target (use-case (name "specialized") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 231) (line 8) (column 44) (len 1)) (integer 1))) (upper (expression (span (offset 234) (line 8) (column 47) (len 1)) (integer 5)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r5))) (value none)) (redefines none) (references none) (crosses none) (intersects none)))) (include (target (use-case (name "tight") (short-name "S3") (typing none) (multiplicity (lower none) (upper (expression (span (offset 282) (line 9) (column 45) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r6))) (value none)) (redefines none) (references none) (crosses none) (intersects none)))))))
)
~~~
