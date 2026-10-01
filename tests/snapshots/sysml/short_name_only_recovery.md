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
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 32) (line 2) (column 5) (len 20)) (message "the spec-valid extended-library declaration production is retained but not structurally implemented"))
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 115) (line 5) (column 9) (len 30)) (message "unexpected token in part definition body"))
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
  )
  (root (package (name "ShortNameRecovery") (body brace (extended-library-declaration) (verification-case-usage (name none) (short-name "Retained")) (part-def (name "Owner") (modifiers) (body brace (malformed (code "recovered_part_def_body_element") (found "exhibit state <> : S;") (span (offset 115) (line 5) (column 9) (len 30))) (exhibit (declaration none) (short-name "RetainedState") (state none)))))))
)
~~~
