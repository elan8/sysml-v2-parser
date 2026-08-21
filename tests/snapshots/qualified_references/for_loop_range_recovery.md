# META
~~~sexpr
(snapshot (type recovery) (description "For-loop ranges retain typed qualified feature chains, malformed ranges recover as explicit body errors without publishing speculative references, and valid following siblings keep deterministic reference identities."))
~~~
# SOURCE
~~~sysml
package LoopRecovery {
    action def Traverse {
        for item in Domain::fleet.activeMembers {
            action visit;
        }
        for orphan in Ghost::leaked + {
            action swallowed;
        }
        action later : Later::Type;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "for_loop_range_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 143) (line 6) (column 9) (len 80)) (message "unexpected token in action body"))
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
    (reference r0 (scope relative) (span (offset 69) (line 3) (column 21) (len 13)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 69) (line 3) (column 21) (len 6))) (segment 1 (token "fleet") (name "fleet") (separator colon-colon) (span (offset 77) (line 3) (column 29) (len 5)))))
    (reference r1 (scope relative) (span (offset 83) (line 3) (column 35) (len 13)) (segments (segment 0 (token "activeMembers") (name "activeMembers") (separator none) (span (offset 83) (line 3) (column 35) (len 13)))))
    (reference r2 (scope relative) (span (offset 238) (line 9) (column 24) (len 11)) (segments (segment 0 (token "Later") (name "Later") (separator none) (span (offset 238) (line 9) (column 24) (len 5))) (segment 1 (token "Type") (name "Type") (separator colon-colon) (span (offset 245) (line 9) (column 31) (len 4)))))
  )
  (root (package (name "LoopRecovery") (body brace (action-def (name "Traverse") (modifiers) (specializes none) (body brace (for-loop (prefix (action-node-prefix (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (action-declaration none))) (variable (for-variable (name "item") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (in (expression (span (offset 69) (line 3) (column 21) (len 27)) (member-access (base (expression (span (offset 69) (line 3) (column 21) (len 13)) (ref r0))) (separator dot) (member (ref r1))))) (body-parameter (action-declaration none) (body brace (action-usage (name "visit") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))))) (malformed (code "recovered_action_body_element") (found "for orphan in Ghost::leaked + {") (span (offset 143) (line 6) (column 9) (len 80))) (action-usage (name "later") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))))
)
~~~
