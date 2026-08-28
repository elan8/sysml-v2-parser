# META
~~~sexpr
(snapshot (type recovery) (description "A malformed `verify` feature-chain target is one explicit requirement-body recovery node and does not leak a speculative reference or consume the following valid verify member. A trailing `.` after a name recovers at the member boundary; the later typed `verify rss.recoverFromStall;` and the constraint siblings keep their arena identities (SysML textual BNF 8.2.2.24)."))
~~~
# SOURCE
~~~sysml
package VerifyFeatureChainRecovery {
    requirement def R {
        requirement recoverFromStall : R;
    }
    requirement rss : R;
    verification def V {
        objective {
            require constraint before;
            verify rss.;
            verify rss.recoverFromStall;
            require constraint after;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "verify_requirement_feature_chain_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_requirement_body_element") (severity error) (category parseerror) (span (offset 230) (line 9) (column 13) (len 25)) (message "unexpected token in requirement body"))
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
    (reference r0 (scope relative) (span (offset 262) (line 10) (column 20) (len 20)) (segments (segment 0 (token "rss") (name "rss") (separator none) (span (offset 262) (line 10) (column 20) (len 3))) (segment 1 (token "recoverFromStall") (name "recoverFromStall") (separator dot) (span (offset 266) (line 10) (column 24) (len 16)))))
  )
  (root (package (name "VerifyFeatureChainRecovery") (body brace (requirement-def (name "R") (modifiers) (body brace (requirement-usage (name "recoverFromStall") (multiplicity none)))) (requirement-usage (name "rss") (multiplicity none)) (verification-case-def (name "V") (modifiers) (body brace (objective (visibility none) (name none) (type none) (body brace (require-constraint (kind require) (constraint-keyword true) (name "before") (target none) (typing none) (body semicolon)) (malformed (code "recovered_requirement_body_element") (found "verify rss.;") (span (offset 230) (line 9) (column 13) (len 25))) (verify (explicit-requirement false) (requirement none) (target (ref r0)) (redefines none)) (require-constraint (kind require) (constraint-keyword true) (name "after") (target none) (typing none) (body semicolon)))))))))
)
~~~
