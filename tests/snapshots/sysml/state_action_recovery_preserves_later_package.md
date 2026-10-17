# META
~~~sexpr
(snapshot (type recovery) (description "Migrated recovery control: malformed members of a state entry action report action-body diagnostics, retain all malformed slices, and preserve the later valid package without a top-level cascade."))
~~~
# SOURCE
~~~sysml
package Broken {
  part def BatteryLevelComputer {
    exhibit state BatteryLevelComputerStates {
      in ref maxBatCap = batteryCapacity;
    }
  }
  state def BatteryLevelComputerStates {
    entry; then x;
    state x {
      entry act { batCap; maxBatCap; computedColor; }
    }
  }
}
package Later {
  part def Good;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "state_action_recovery_preserves_later_package.md"
    (diagnostics
      (diagnostic (code "invalid_bare_identifier_in_action_body") (severity error) (category parseerror) (span (offset 242) (line 10) (column 19) (len 8)) (message "bare identifier `batCap` is not a valid action body member"))
      (diagnostic (code "invalid_bare_identifier_in_action_body") (severity error) (category parseerror) (span (offset 250) (line 10) (column 27) (len 11)) (message "bare identifier `maxBatCap` is not a valid action body member"))
      (diagnostic (code "invalid_bare_identifier_in_action_body") (severity error) (category parseerror) (span (offset 261) (line 10) (column 38) (len 15)) (message "bare identifier `computedColor` is not a valid action body member"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Broken {
    part def BatteryLevelComputer {
        exhibit state BatteryLevelComputerStates {
            in ref maxBatCap = batteryCapacity;
        }
    }
    state def BatteryLevelComputerStates {
        entry;
        then x;
        state x {
            entry act {
                batCap;
                maxBatCap;
                computedColor;
            }
        }
    }
}

package Later {
    part def Good;
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 207) (line 8) (column 17) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 207) (line 8) (column 17) (len 1)))))
    (reference r1 (scope relative) (span (offset 236) (line 10) (column 13) (len 3)) (segments (segment 0 (token "act") (name "act") (separator none) (span (offset 236) (line 10) (column 13) (len 3)))))
  )
  (root (package (name "Broken") (body brace (part-def (name "BatteryLevelComputer") (modifiers) (body brace (exhibit (declaration "BatteryLevelComputerStates") (short-name none) (state none)))) (state-def (name "BatteryLevelComputerStates") (modifiers) (body brace (entry (action empty) (body semicolon)) (then (state (ref r0))) (state-usage (name "x") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (entry (action (reference (action (ref r1)) (redefines none)) (value none)) (body brace (malformed (code "invalid_bare_identifier_in_action_body") (found "batCap;") (span (offset 242) (line 10) (column 19) (len 8))) (malformed (code "invalid_bare_identifier_in_action_body") (found "maxBatCap;") (span (offset 250) (line 10) (column 27) (len 11))) (malformed (code "invalid_bare_identifier_in_action_body") (found "computedColor;") (span (offset 261) (line 10) (column 38) (len 15))))))))))) (package (name "Later") (body brace (part-def (name "Good") (modifiers) (body semicolon)))))
)
~~~
