# META
~~~sexpr
(snapshot (type provenance) (description "A KerML flow preserves each authored `of PayloadFeature` clause as an ordered, source-backed clause, making the violating side of validateFlowPayloadFeature observable without text reconstruction; the single-clause sibling retains the ordinary shape. KerML textual BNF 1303-1334; Pilot KerML.xtext 995-1035."))
~~~
# SOURCE
~~~sysml
package FlowPayloadClauses {
    behavior Transfers {
        flow of Thing of Other from source to target;
        flow of Single from source to target;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "flow_repeated_payload.md"
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
    (reference r0 (scope relative) (span (offset 70) (line 3) (column 17) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 70) (line 3) (column 17) (len 5)))))
    (reference r1 (scope relative) (span (offset 79) (line 3) (column 26) (len 5)) (segments (segment 0 (token "Other") (name "Other") (separator none) (span (offset 79) (line 3) (column 26) (len 5)))))
    (reference r2 (scope relative) (span (offset 90) (line 3) (column 37) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 90) (line 3) (column 37) (len 6)))))
    (reference r3 (scope relative) (span (offset 100) (line 3) (column 47) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 100) (line 3) (column 47) (len 6)))))
    (reference r4 (scope relative) (span (offset 124) (line 4) (column 17) (len 6)) (segments (segment 0 (token "Single") (name "Single") (separator none) (span (offset 124) (line 4) (column 17) (len 6)))))
    (reference r5 (scope relative) (span (offset 136) (line 4) (column 29) (len 6)) (segments (segment 0 (token "source") (name "source") (separator none) (span (offset 136) (line 4) (column 29) (len 6)))))
    (reference r6 (scope relative) (span (offset 146) (line 4) (column 39) (len 6)) (segments (segment 0 (token "target") (name "target") (separator none) (span (offset 146) (line 4) (column 39) (len 6)))))
  )
  (root (package (name "FlowPayloadClauses") (body brace (kerml-classifier (keyword behavior) (abstract false) (name "Transfers") (specializes none) (conjugates none) (body brace (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payloads (payload (of (span (offset 67) (line 3) (column 14) (len 2))) (feature (name none) (type (ref r0)) (conjugated false) (multiplicity none))) (payload (of (span (offset 76) (line 3) (column 23) (len 2))) (feature (name none) (type (ref r1)) (conjugated false) (multiplicity none)))) (endpoints (from (connector-end (multiplicity none) (target (ref r2)) (references none))) (to (connector-end (multiplicity none) (target (ref r3)) (references none)))))) (body (body semicolon))) (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payloads (payload (of (span (offset 121) (line 4) (column 14) (len 2))) (feature (name none) (type (ref r4)) (conjugated false) (multiplicity none)))) (endpoints (from (connector-end (multiplicity none) (target (ref r5)) (references none))) (to (connector-end (multiplicity none) (target (ref r6)) (references none)))))) (body (body semicolon))))))))
)
~~~
