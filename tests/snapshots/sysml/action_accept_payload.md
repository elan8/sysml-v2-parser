# META
~~~sexpr
(snapshot (type semantic) (description "Direct and inline action accepts retain the typed-payload and bare source-backed payload-reference alternatives, including accept-owned via targets. SysML textual BNF 954-965 and 1002-1020; Pilot SysML.xtext 1438-1451."))
~~~
# SOURCE
~~~sysml
package ActionAcceptPayload {
    action def Owner {
        accept payload : Signal via inputPort;
        accept Signals::Deliver via consumer.port;
        then action receive accept Event via consumer.port;
        then action typed accept payload : Signal via inputPort;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "action_accept_payload.md"
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
  (references
    (reference r0 (scope relative) (span (offset 78) (line 3) (column 26) (len 6)) (segments (segment 0 (token "Signal") (name "Signal") (separator none) (span (offset 78) (line 3) (column 26) (len 6)))))
    (reference r1 (scope relative) (span (offset 89) (line 3) (column 37) (len 9)) (segments (segment 0 (token "inputPort") (name "inputPort") (separator none) (span (offset 89) (line 3) (column 37) (len 9)))))
    (reference r2 (scope relative) (span (offset 115) (line 4) (column 16) (len 16)) (segments (segment 0 (token "Signals") (name "Signals") (separator none) (span (offset 115) (line 4) (column 16) (len 7))) (segment 1 (token "Deliver") (name "Deliver") (separator colon-colon) (span (offset 124) (line 4) (column 25) (len 7)))))
    (reference r3 (scope relative) (span (offset 136) (line 4) (column 37) (len 8)) (segments (segment 0 (token "consumer") (name "consumer") (separator none) (span (offset 136) (line 4) (column 37) (len 8)))))
    (reference r4 (scope relative) (span (offset 145) (line 4) (column 46) (len 4)) (segments (segment 0 (token "port") (name "port") (separator none) (span (offset 145) (line 4) (column 46) (len 4)))))
    (reference r5 (scope relative) (span (offset 186) (line 5) (column 36) (len 5)) (segments (segment 0 (token "Event") (name "Event") (separator none) (span (offset 186) (line 5) (column 36) (len 5)))))
    (reference r6 (scope relative) (span (offset 196) (line 5) (column 46) (len 8)) (segments (segment 0 (token "consumer") (name "consumer") (separator none) (span (offset 196) (line 5) (column 46) (len 8)))))
    (reference r7 (scope relative) (span (offset 205) (line 5) (column 55) (len 4)) (segments (segment 0 (token "port") (name "port") (separator none) (span (offset 205) (line 5) (column 55) (len 4)))))
    (reference r8 (scope relative) (span (offset 254) (line 6) (column 44) (len 6)) (segments (segment 0 (token "Signal") (name "Signal") (separator none) (span (offset 254) (line 6) (column 44) (len 6)))))
    (reference r9 (scope relative) (span (offset 265) (line 6) (column 55) (len 9)) (segments (segment 0 (token "inputPort") (name "inputPort") (separator none) (span (offset 265) (line 6) (column 55) (len 9)))))
  )
  (root (package (name "ActionAcceptPayload") (body brace (action-def (name "Owner") (modifiers) (specializes none) (body brace (action-usage (keyword accept) (name none) (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (accept (payload (name "payload") (type (ref r0)) (via (expression (span (offset 89) (line 3) (column 37) (len 9)) (ref r1))))) (body semicolon)) (action-usage (keyword accept) (name none) (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (accept (shorthand (expression (span (offset 115) (line 4) (column 16) (len 16)) (ref r2)) (via (expression (span (offset 136) (line 4) (column 37) (len 13)) (member-access (base (expression (span (offset 136) (line 4) (column 37) (len 8)) (ref r3))) (separator dot) (member (ref r4))))))) (body semicolon)) (then-action (action-usage (keyword action) (name "receive") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (accept (shorthand (expression (span (offset 186) (line 5) (column 36) (len 5)) (ref r5)) (via (expression (span (offset 196) (line 5) (column 46) (len 13)) (member-access (base (expression (span (offset 196) (line 5) (column 46) (len 8)) (ref r6))) (separator dot) (member (ref r7))))))) (body semicolon))) (then-action (action-usage (keyword action) (name "typed") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (accept (payload (name "payload") (type (ref r8)) (via (expression (span (offset 265) (line 6) (column 55) (len 9)) (ref r9))))) (body semicolon))))))))
)
~~~
