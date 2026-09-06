# META
~~~sexpr
(snapshot (type recovery) (description "`verify requirement <feature-chain>;` opens the `RequirementVerificationUsage` alternative that needs a `ConstraintUsageDeclaration`, so a bare reference is invalid (SysML textual BNF 8.2.2.24). Recovery is one targeted `verify_requirement_expects_declaration` node per occurrence, bounded at the next member's `verify` keyword, carrying a `verify <chain>;` fix; the trailing typed `verify rss.recoverFromStall;` and the constraint siblings keep their arena identities."))
~~~
# SOURCE
~~~sysml
package VerifyRequirementKeywordRecovery {
    requirement def R {
        requirement recoverFromStall : R;
        requirement returnToDock : R;
    }
    requirement rss : R;
    verification def V {
        objective {
            require constraint before;
            verify requirement rss.recoverFromStall;
            verify requirement rss.returnToDock;
            verify rss.recoverFromStall;
            require constraint after;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "verify_requirement_keyword_bare_reference_recovery.md"
    (diagnostics
      (diagnostic (code "verify_requirement_expects_declaration") (severity error) (category parseerror) (span (offset 274) (line 10) (column 13) (len 53)) (message "`verify requirement` opens an owned requirement usage, which needs a name and/or a `: Type` / `:>>` / `= <ref>` clause; `rss.recoverFromStall` is only a reference"))
      (diagnostic (code "verify_requirement_expects_declaration") (severity error) (category parseerror) (span (offset 327) (line 11) (column 13) (len 49)) (message "`verify requirement` opens an owned requirement usage, which needs a name and/or a `: Type` / `:>>` / `= <ref>` clause; `rss.returnToDock` is only a reference"))
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
    (reference r0 (scope relative) (span (offset 383) (line 12) (column 20) (len 20)) (segments (segment 0 (token "rss") (name "rss") (separator none) (span (offset 383) (line 12) (column 20) (len 3))) (segment 1 (token "recoverFromStall") (name "recoverFromStall") (separator dot) (span (offset 387) (line 12) (column 24) (len 16)))))
  )
  (root (package (name "VerifyRequirementKeywordRecovery") (body brace (requirement-def (name "R") (modifiers) (body brace (requirement-usage (name "recoverFromStall") (multiplicity none)) (requirement-usage (name "returnToDock") (multiplicity none)))) (requirement-usage (name "rss") (multiplicity none)) (verification-case-def (name "V") (modifiers) (body brace (objective (visibility none) (name none) (type none) (body brace (require-constraint (kind require) (constraint-keyword true) (name "before") (target none) (typing none) (body semicolon)) (malformed (code "verify_requirement_expects_declaration") (found "verify requirement rss.recoverFromStall;") (span (offset 274) (line 10) (column 13) (len 53))) (malformed (code "verify_requirement_expects_declaration") (found "verify requirement rss.returnToDock;") (span (offset 327) (line 11) (column 13) (len 49))) (verify (explicit-requirement false) (requirement none) (target (ref r0)) (redefines none)) (require-constraint (kind require) (constraint-keyword true) (name "after") (target none) (typing none) (body semicolon)))))))))
)
~~~
