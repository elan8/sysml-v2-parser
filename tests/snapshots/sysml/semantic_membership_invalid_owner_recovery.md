# META
~~~sexpr
(snapshot (type recovery) (description "Malformed expose, verify, and render memberships recover at their owning body boundary without leaking speculative references or consuming the following valid typed membership and declaration sibling."))
~~~
# SOURCE
~~~sysml
package MembershipOwnerRecovery {
    expose leaked::;
    expose retained;
    part def InvalidOwner {
        verify requirement leaked : ;
        verify requirement retained : RequirementType;
        render leaked : ;
        render retained : RenderingType;
        attribute after;
    }
    part def Later;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "semantic_membership_invalid_owner_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 38) (line 2) (column 5) (len 21)) (message "unexpected token in package body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 38) (line 2) (column 5) (len 21)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
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
    (reference r0 (scope relative) (span (offset 66) (line 3) (column 12) (len 8)) (segments (segment 0 (token "retained") (name "retained") (separator none) (span (offset 66) (line 3) (column 12) (len 8)))))
    (reference r1 (scope relative) (span (offset 180) (line 6) (column 39) (len 15)) (segments (segment 0 (token "RequirementType") (name "RequirementType") (separator none) (span (offset 180) (line 6) (column 39) (len 15)))))
    (reference r2 (scope relative) (span (offset 249) (line 8) (column 27) (len 13)) (segments (segment 0 (token "RenderingType") (name "RenderingType") (separator none) (span (offset 249) (line 8) (column 27) (len 13)))))
  )
  (root (package (name "MembershipOwnerRecovery") (body brace (malformed (code "recovered_package_body_element") (found "expose leaked::;") (span (offset 38) (line 2) (column 5) (len 21))) (expose (target (span (span (offset 66) (line 3) (column 12) (len 8))) (all none) (ref r0) (shape (membership (recursive-suffix none)))) (body semicolon)) (part-def (name "InvalidOwner") (modifiers) (body brace (malformed (code "recovered_part_def_body_element") (found "verify requirement leaked : ;") (span (offset 112) (line 5) (column 9) (len 38))) (verify (explicit-requirement true) (requirement (name "retained") (type (ref r1)) (body semicolon)) (target none) (redefines none)) (malformed (code "recovered_part_def_body_element") (found "render leaked : ;") (span (offset 205) (line 7) (column 9) (len 26))) (view-rendering (name "retained") (type (ref r2)) (body semicolon)) (attribute-usage (declaration-name "after") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Later") (modifiers) (body semicolon)))))
)
~~~
