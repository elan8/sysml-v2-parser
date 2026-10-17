# META
~~~sexpr
(snapshot (type semantic) (description "Malformed short-name identifications produce recovery diagnostics while following valid short-name-only verification and exhibit-state declarations remain typed and format in place."))
~~~
# SOURCE
~~~sysml
package ShortNameRecovery {
    verification <> : V;
    verification <Retained> : V;
    part def Owner {
        exhibit state <> : S;
        exhibit state <RetainedState> : S;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "short_name_only_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 32) (line 2) (column 5) (len 20)) (message "unexpected token in package body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 32) (line 2) (column 5) (len 20)) (message "suppressed 1 cascading recovered diagnostic after earlier recovery errors"))
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
    (reference r0 (scope relative) (span (offset 83) (line 3) (column 31) (len 1)) (segments (segment 0 (token "V") (name "V") (separator none) (span (offset 83) (line 3) (column 31) (len 1)))))
  )
  (root (package (name "ShortNameRecovery") (body brace (malformed (code "recovered_package_body_element") (found "verification <> : V;") (span (offset 32) (line 2) (column 5) (len 20))) (verification-case-usage (declaration (name none) (short-name "Retained") (abstract false) (type (ref r0)) (subsets none)) (body semicolon)) (part-def (name "Owner") (modifiers) (body brace (malformed (code "recovered_part_def_body_element") (found "exhibit state <> : S;") (span (offset 115) (line 5) (column 9) (len 30))) (exhibit (declaration none) (short-name "RetainedState") (state none)))))))
)
~~~
