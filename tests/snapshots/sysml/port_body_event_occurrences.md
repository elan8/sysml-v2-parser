# META
~~~sexpr
(snapshot (type recovery) (description "Port bodies retain both EventOccurrenceUsage alternatives as typed occurrence members, while a malformed event recovers before a later valid sibling."))
~~~
# SOURCE
~~~sysml
package P {
    port p : PortType {
        event occurrence received : Signal;
        event sender.sourceEvent;
        event occurrence = ;
        item afterRecovery : ItemType;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "port_body_event_occurrences.md"
    (diagnostics
      (diagnostic (code "recovered_port_body_element") (severity error) (category parseerror) (span (offset 122) (line 5) (column 9) (len 29)) (message "unexpected token in port body"))
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
    (reference r0 (scope relative) (span (offset 25) (line 2) (column 14) (len 8)) (segments (segment 0 (token "PortType") (name "PortType") (separator none) (span (offset 25) (line 2) (column 14) (len 8)))))
    (reference r1 (scope relative) (span (offset 94) (line 4) (column 15) (len 18)) (segments (segment 0 (token "sender") (name "sender") (separator none) (span (offset 94) (line 4) (column 15) (len 6))) (segment 1 (token "sourceEvent") (name "sourceEvent") (separator dot) (span (offset 101) (line 4) (column 22) (len 11)))))
    (reference r2 (scope relative) (span (offset 172) (line 6) (column 30) (len 8)) (segments (segment 0 (token "ItemType") (name "ItemType") (separator none) (span (offset 172) (line 6) (column 30) (len 8)))))
  )
  (root (package (name "P") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration "received") (short-name none) (target none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (event true) (declaration none) (short-name none) (target (ref r1)) (body semicolon)) (malformed (code "recovered_port_body_element") (found "event occurrence = ;") (span (offset 122) (line 5) (column 9) (len 29))) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "afterRecovery") (short-name none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))
)
~~~
