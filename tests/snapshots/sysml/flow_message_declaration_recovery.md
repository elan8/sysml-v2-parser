# META
~~~sexpr
(snapshot (type recovery) (description "A malformed leading flow specialization is recovered as one action-body member. The following endpoint-only flow and declared message remain typed, proving the transactional declaration/endpoint dispatch does not leak speculative references or swallow later siblings."))
~~~
# SOURCE
~~~sysml
package FlowMessageDeclarationRecovery {
    action def A {
        flow :>> ;
        flow source.out to target.in;
        message sent : Message from source.event to target.event;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "flow_message_declaration_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 68) (line 3) (column 9) (len 19)) (message "unexpected token in action body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package FlowMessageDeclarationRecovery {
    action def A {
        flow :>> ;
        flow from source.out to target.in;
        message sent : Message from source.event to target.event;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 92) (line 4) (column 14) (len 10)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 92) (line 4) (column 14) (len 6))) (segment 1 (token "out") (name "out") (separator dot) (span (offset 99) (line 4) (column 21) (len 3)))))
    (reference r1 (scope relative) (span (offset 106) (line 4) (column 28) (len 9)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 106) (line 4) (column 28) (len 6))) (segment 1 (token "in") (name "in") (separator dot) (span (offset 113) (line 4) (column 35) (len 2)))))
    (reference r2 (scope relative) (span (offset 140) (line 5) (column 24) (len 7)) (segments (segment 0 (token "Message") (name "Message") (separator none) (span (offset 140) (line 5) (column 24) (len 7)))))
    (reference r3 (scope relative) (span (offset 153) (line 5) (column 37) (len 12)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 153) (line 5) (column 37) (len 6))) (segment 1 (token "event") (name "event") (separator dot) (span (offset 160) (line 5) (column 44) (len 5)))))
    (reference r4 (scope relative) (span (offset 169) (line 5) (column 53) (len 12)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 169) (line 5) (column 53) (len 6))) (segment 1 (token "event") (name "event") (separator dot) (span (offset 176) (line 5) (column 60) (len 5)))))
  )
  (root (package (name "FlowMessageDeclarationRecovery") (body brace (action-def (name "A") (modifiers) (specializes none) (body brace (malformed (code "recovered_action_body_element") (found "flow :>> ;") (span (offset 68) (line 3) (column 9) (len 19))) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r0)) (references none))) (to (connector-end (multiplicity none) (target (ref r1)) (references none))))) (body (body semicolon))) (flow-usage (kind message) (visibility none) (declaration (declared (name "sent") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payloads) (endpoints (from (connector-end (multiplicity none) (target (ref r3)) (references none))) (to (connector-end (multiplicity none) (target (ref r4)) (references none)))))) (body (body semicolon))))))))
)
~~~
