# META
~~~sexpr
(snapshot (type recovery) (description "A malformed action accept payload becomes explicit recovery without consuming a later valid bare action accept. SysML textual BNF 898-909 and 1002-1020; Pilot ActionBody 1361-1368 and AcceptNode 1450-1451."))
~~~
# SOURCE
~~~sysml
package ActionAcceptPayloadRecovery {
    action def Owner {
        action malformed accept : Signal;
        then action retained accept Deliver via consumer;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "action_accept_payload_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 86) (line 3) (column 26) (len 25)) (message "unexpected token in action body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package ActionAcceptPayloadRecovery {
    action def Owner {
        action malformed
        accept : Signal;
        then action retained accept Deliver via consumer;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 139) (line 4) (column 37) (len 7)) (segments (segment 0 (token "Deliver") (name "Deliver") (separator none) (span (offset 139) (line 4) (column 37) (len 7)))))
    (reference r1 (scope relative) (span (offset 151) (line 4) (column 49) (len 8)) (segments (segment 0 (token "consumer") (name "consumer") (separator none) (span (offset 151) (line 4) (column 49) (len 8)))))
  )
  (root (package (name "ActionAcceptPayloadRecovery") (body brace (action-def (name "Owner") (modifiers) (specializes none) (body brace (action-usage (keyword action) (name "malformed") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body absent)) (malformed (code "recovered_action_body_element") (found "accept : Signal;") (span (offset 86) (line 3) (column 26) (len 25))) (then-action (action-usage (keyword action) (name "retained") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (accept (shorthand (expression (span (offset 139) (line 4) (column 37) (len 7)) (ref r0)) (via (expression (span (offset 151) (line 4) (column 49) (len 8)) (ref r1))))) (body semicolon))))))))
)
~~~
