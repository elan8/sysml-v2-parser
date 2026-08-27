# META
~~~sexpr
(snapshot (type semantic) (description "Flow and message declarations distinguish a complete UsageDeclaration (including leading specialization and FeatureValue clauses) from the endpoint-only alternative. Direct interaction-body message redefinitions and the semantic projection retain their authored declaration, FeatureValue, payload, and typed endpoint pair rather than inventing an empty declaration."))
~~~
# SOURCE
~~~sysml
package FlowMessageDeclarationAlternatives {
    flow :> inheritedFlow;
    flow :>> redefinedFlow;
    flow shaped : Signal = source.value of payload : Payload from source.out to target.in;
    flow source.out to target.in;
    message sent : Message = source.value of payload : Payload from source.event to target.event;
    occurrence DirectInteraction {
        message :>> inheritedMessage = participant.sentMessage;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "flow_message_declaration_alternatives.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package FlowMessageDeclarationAlternatives {
    flow :> inheritedFlow;
    flow :>> redefinedFlow;
    flow shaped : Signal = source.value of payload : Payload from source.out to target.in;
    flow from source.out to target.in;
    message sent : Message = source.value of payload : Payload from source.event to target.event;
    occurrence DirectInteraction {
        message :>> inheritedMessage = participant.sentMessage;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 57) (line 2) (column 13) (len 13)) (segments (segment 0 (token "inheritedFlow") (name "inheritedFlow") (separator none) (span (offset 57) (line 2) (column 13) (len 13)))))
    (reference r1 (scope relative) (span (offset 85) (line 3) (column 14) (len 13)) (segments (segment 0 (token "redefinedFlow") (name "redefinedFlow") (separator none) (span (offset 85) (line 3) (column 14) (len 13)))))
    (reference r2 (scope relative) (span (offset 118) (line 4) (column 19) (len 6)) (segments (segment 0 (token "Signal") (name "Signal") (separator none) (span (offset 118) (line 4) (column 19) (len 6)))))
    (reference r3 (scope relative) (span (offset 127) (line 4) (column 28) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 127) (line 4) (column 28) (len 6)))))
    (reference r4 (scope relative) (span (offset 134) (line 4) (column 35) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 134) (line 4) (column 35) (len 5)))))
    (reference r5 (scope relative) (span (offset 153) (line 4) (column 54) (len 7)) (segments (segment 0 (token "Payload") (name "Payload") (separator none) (span (offset 153) (line 4) (column 54) (len 7)))))
    (reference r6 (scope relative) (span (offset 166) (line 4) (column 67) (len 10)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 166) (line 4) (column 67) (len 6))) (segment 1 (token "out") (name "out") (separator dot) (span (offset 173) (line 4) (column 74) (len 3)))))
    (reference r7 (scope relative) (span (offset 180) (line 4) (column 81) (len 9)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 180) (line 4) (column 81) (len 6))) (segment 1 (token "in") (name "in") (separator dot) (span (offset 187) (line 4) (column 88) (len 2)))))
    (reference r8 (scope relative) (span (offset 200) (line 5) (column 10) (len 10)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 200) (line 5) (column 10) (len 6))) (segment 1 (token "out") (name "out") (separator dot) (span (offset 207) (line 5) (column 17) (len 3)))))
    (reference r9 (scope relative) (span (offset 214) (line 5) (column 24) (len 9)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 214) (line 5) (column 24) (len 6))) (segment 1 (token "in") (name "in") (separator dot) (span (offset 221) (line 5) (column 31) (len 2)))))
    (reference r10 (scope relative) (span (offset 244) (line 6) (column 20) (len 7)) (segments (segment 0 (token "Message") (name "Message") (separator none) (span (offset 244) (line 6) (column 20) (len 7)))))
    (reference r11 (scope relative) (span (offset 254) (line 6) (column 30) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 254) (line 6) (column 30) (len 6)))))
    (reference r12 (scope relative) (span (offset 261) (line 6) (column 37) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 261) (line 6) (column 37) (len 5)))))
    (reference r13 (scope relative) (span (offset 280) (line 6) (column 56) (len 7)) (segments (segment 0 (token "Payload") (name "Payload") (separator none) (span (offset 280) (line 6) (column 56) (len 7)))))
    (reference r14 (scope relative) (span (offset 293) (line 6) (column 69) (len 12)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 293) (line 6) (column 69) (len 6))) (segment 1 (token "event") (name "event") (separator dot) (span (offset 300) (line 6) (column 76) (len 5)))))
    (reference r15 (scope relative) (span (offset 309) (line 6) (column 85) (len 12)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 309) (line 6) (column 85) (len 6))) (segment 1 (token "event") (name "event") (separator dot) (span (offset 316) (line 6) (column 92) (len 5)))))
    (reference r16 (scope relative) (span (offset 378) (line 8) (column 21) (len 16)) (segments (segment 0 (token "inheritedMessage") (name "inheritedMessage") (separator none) (span (offset 378) (line 8) (column 21) (len 16)))))
    (reference r17 (scope relative) (span (offset 397) (line 8) (column 40) (len 11)) (segments (segment 0 (token "participant") (name "participant") (separator none) (span (offset 397) (line 8) (column 40) (len 11)))))
    (reference r18 (scope relative) (span (offset 409) (line 8) (column 52) (len 11)) (segments (segment 0 (token "sentMessage") (name "sentMessage") (separator none) (span (offset 409) (line 8) (column 52) (len 11)))))
  )
  (root (package (name "FlowMessageDeclarationAlternatives") (body brace (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r0))) (value none)) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payloads) (endpoints none))) (body (body semicolon))) (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (references none) (crosses none) (intersects none)) (value none) (payloads) (endpoints none))) (body (body semicolon))) (flow-usage (kind flow) (visibility none) (declaration (declared (name "shaped") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 127) (line 4) (column 28) (len 12)) (member-access (base (expression (span (offset 127) (line 4) (column 28) (len 6)) (ref r3))) (separator dot) (member (ref r4))))))) (payloads (payload (of (span (offset 140) (line 4) (column 41) (len 2))) (feature (name "payload") (type (ref r5)) (conjugated false) (multiplicity none)))) (endpoints (from (connector-end (multiplicity none) (target (ref r6)) (references none))) (to (connector-end (multiplicity none) (target (ref r7)) (references none)))))) (body (body semicolon))) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r8)) (references none))) (to (connector-end (multiplicity none) (target (ref r9)) (references none))))) (body (body semicolon))) (flow-usage (kind message) (visibility none) (declaration (declared (name "sent") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 254) (line 6) (column 30) (len 12)) (member-access (base (expression (span (offset 254) (line 6) (column 30) (len 6)) (ref r11))) (separator dot) (member (ref r12))))))) (payloads (payload (of (span (offset 267) (line 6) (column 43) (len 2))) (feature (name "payload") (type (ref r13)) (conjugated false) (multiplicity none)))) (endpoints (from (connector-end (multiplicity none) (target (ref r14)) (references none))) (to (connector-end (multiplicity none) (target (ref r15)) (references none)))))) (body (body semicolon))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "DirectInteraction") (short-name none) (target none) (body brace (flow-usage (kind message) (visibility none) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (references none) (crosses none) (intersects none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 397) (line 8) (column 40) (len 23)) (member-access (base (expression (span (offset 397) (line 8) (column 40) (len 11)) (ref r17))) (separator dot) (member (ref r18))))))) (payloads) (endpoints none))) (body (body semicolon))))))))
)
~~~
