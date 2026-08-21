# META
~~~sexpr
(snapshot (type semantic) (description "The canonical anonymous flow shorthand (flow from a to b;) and its succession flow sibling parse with no declared name instead of silently taking the from keyword as the flow's name, while genuinely named flows keep theirs (spec42 Gap 47)."))
~~~
# SOURCE
~~~sysml
package AnonymousFlowShorthand {
    action def Shoot {
        flow from focus.image to shoot.image;
        succession flow from focus.image to shoot.image;
        succession flow lightFlow from bulb.light to lens.light;
        flow of Exposure from focus.xrsl to shoot.xsf;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "anonymous_flow_shorthand.md"
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
    (reference r0 (scope relative) (span (offset 74) (line 3) (column 19) (len 11)) (segments (segment 0 (token "focus") (name "focus") (separator none) (span (offset 74) (line 3) (column 19) (len 5))) (segment 1 (token "image") (name "image") (separator dot) (span (offset 80) (line 3) (column 25) (len 5)))))
    (reference r1 (scope relative) (span (offset 89) (line 3) (column 34) (len 11)) (segments (segment 0 (token "shoot") (name "shoot") (separator none) (span (offset 89) (line 3) (column 34) (len 5))) (segment 1 (token "image") (name "image") (separator dot) (span (offset 95) (line 3) (column 40) (len 5)))))
    (reference r2 (scope relative) (span (offset 131) (line 4) (column 30) (len 11)) (segments (segment 0 (token "focus") (name "focus") (separator none) (span (offset 131) (line 4) (column 30) (len 5))) (segment 1 (token "image") (name "image") (separator dot) (span (offset 137) (line 4) (column 36) (len 5)))))
    (reference r3 (scope relative) (span (offset 146) (line 4) (column 45) (len 11)) (segments (segment 0 (token "shoot") (name "shoot") (separator none) (span (offset 146) (line 4) (column 45) (len 5))) (segment 1 (token "image") (name "image") (separator dot) (span (offset 152) (line 4) (column 51) (len 5)))))
    (reference r4 (scope relative) (span (offset 198) (line 5) (column 40) (len 10)) (segments (segment 0 (token "bulb") (name "bulb") (separator none) (span (offset 198) (line 5) (column 40) (len 4))) (segment 1 (token "light") (name "light") (separator dot) (span (offset 203) (line 5) (column 45) (len 5)))))
    (reference r5 (scope relative) (span (offset 212) (line 5) (column 54) (len 10)) (segments (segment 0 (token "lens") (name "lens") (separator none) (span (offset 212) (line 5) (column 54) (len 4))) (segment 1 (token "light") (name "light") (separator dot) (span (offset 217) (line 5) (column 59) (len 5)))))
    (reference r6 (scope relative) (span (offset 240) (line 6) (column 17) (len 8)) (segments (segment 0 (token "Exposure") (name "Exposure") (separator none) (span (offset 240) (line 6) (column 17) (len 8)))))
    (reference r7 (scope relative) (span (offset 254) (line 6) (column 31) (len 10)) (segments (segment 0 (token "focus") (name "focus") (separator none) (span (offset 254) (line 6) (column 31) (len 5))) (segment 1 (token "xrsl") (name "xrsl") (separator dot) (span (offset 260) (line 6) (column 37) (len 4)))))
    (reference r8 (scope relative) (span (offset 268) (line 6) (column 45) (len 9)) (segments (segment 0 (token "shoot") (name "shoot") (separator none) (span (offset 268) (line 6) (column 45) (len 5))) (segment 1 (token "xsf") (name "xsf") (separator dot) (span (offset 274) (line 6) (column 51) (len 3)))))
  )
  (root (package (name "AnonymousFlowShorthand") (body brace (action-def (name "Shoot") (modifiers) (specializes none) (body brace (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r0)) (references none))) (to (connector-end (multiplicity none) (target (ref r1)) (references none))))) (body (body semicolon))) (flow-usage (kind succession-flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r2)) (references none))) (to (connector-end (multiplicity none) (target (ref r3)) (references none))))) (body (body semicolon))) (flow-usage (kind succession-flow) (visibility none) (declaration (declared (name "lightFlow") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints (from (connector-end (multiplicity none) (target (ref r4)) (references none))) (to (connector-end (multiplicity none) (target (ref r5)) (references none)))))) (body (body semicolon))) (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload (name none) (type (ref r6)) (conjugated false) (multiplicity none)) (endpoints (from (connector-end (multiplicity none) (target (ref r7)) (references none))) (to (connector-end (multiplicity none) (target (ref r8)) (references none)))))) (body (body semicolon))))))))
)
~~~
