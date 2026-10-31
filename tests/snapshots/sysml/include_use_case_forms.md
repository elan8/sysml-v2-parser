# META
~~~sexpr
(snapshot (type semantic) (description "IncludeUseCaseUsage takes an OwnedReferenceSubsetting, which may be a feature chain, or `use case` with a full Identification. Both used to fall through to a stray `include` expression followed by a separate member."))
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
~~~sexpr
(stable-idempotent)
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
  )
  (root (use-case-def (name "Host") (modifiers) (body brace (include (target (ref r0))) (include (target (ref r1))) (include (target (ref r2))) (include (target (use-case (short-name "SN") (name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3))))))) (include (target (use-case (short-name "S2") (name "named") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4))))))) (include (target (use-case (short-name none) (name "bare") (typing none)))))))
)
~~~
