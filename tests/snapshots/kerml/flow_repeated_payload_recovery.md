# META
~~~sexpr
(snapshot (type recovery) (description "A malformed repeated KerML flow payload clause recovers as one exact malformed member without leaking its first payload reference or consuming the following valid flow sibling. KerML textual BNF 1303-1334; Pilot KerML.xtext 995-1035."))
~~~
# SOURCE
~~~sysml
package FlowPayloadRecovery {
    behavior Transfers {
        flow of Thing of ;
        flow of Retained from source to target;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "flow_repeated_payload_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_calc_body_element") (severity error) (category parseerror) (span (offset 63) (line 3) (column 9) (len 27)) (message "unexpected token in calc body"))
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
    (reference r0 (scope relative) (span (offset 98) (line 4) (column 17) (len 8)) (segments (segment 0 (token "Retained") (name "Retained") (separator none) (span (offset 98) (line 4) (column 17) (len 8)))))
    (reference r1 (scope relative) (span (offset 112) (line 4) (column 31) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 112) (line 4) (column 31) (len 6)))))
    (reference r2 (scope relative) (span (offset 122) (line 4) (column 41) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 122) (line 4) (column 41) (len 6)))))
  )
  (root (package (name "FlowPayloadRecovery") (body brace (kerml-classifier (keyword behavior) (abstract false) (name "Transfers") (specializes none) (conjugates none) (body brace (malformed (code "recovered_calc_body_element") (found "flow of Thing of ;") (span (offset 63) (line 3) (column 9) (len 27))) (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payloads (payload (of (span (offset 95) (line 4) (column 14) (len 2))) (feature (name none) (type (ref r0)) (conjugated false) (multiplicity none)))) (endpoints (from (connector-end (multiplicity none) (target (ref r1)) (references none))) (to (connector-end (multiplicity none) (target (ref r2)) (references none)))))) (body (body semicolon))))))))
)
~~~
