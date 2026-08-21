# META
~~~sexpr
(snapshot (type semantic) (description "An InterfaceUsage body retains both source-backed PerformActionUsageDeclaration alternatives beside its existing FlowUsage member (SysML textual BNF 724-759, 374-390; pinned Pilot SysML.xtext 1109-1144)."))
~~~
# SOURCE
~~~sysml
package InterfaceUsageBodyPerform {
    part host {
        interface link connect left.port to right.port {
            flow transmitted from left.port to right.port;
            perform action dispatch : ActionType;
            perform workflow.execute :>> replacement;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "interface_usage_body_perform.md"
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
    (reference r0 (scope relative) (span (offset 83) (line 3) (column 32) (len 9)) (segments (segment 0 (token "left") (name "left") (separator none) (span (offset 83) (line 3) (column 32) (len 4))) (segment 1 (token "port") (name "port") (separator dot) (span (offset 88) (line 3) (column 37) (len 4)))))
    (reference r1 (scope relative) (span (offset 96) (line 3) (column 45) (len 10)) (segments (segment 0 (token "right") (name "right") (separator none) (span (offset 96) (line 3) (column 45) (len 5))) (segment 1 (token "port") (name "port") (separator dot) (span (offset 102) (line 3) (column 51) (len 4)))))
    (reference r2 (scope relative) (span (offset 143) (line 4) (column 35) (len 9)) (segments (segment 0 (token "left") (name "left") (separator none) (span (offset 143) (line 4) (column 35) (len 4))) (segment 1 (token "port") (name "port") (separator dot) (span (offset 148) (line 4) (column 40) (len 4)))))
    (reference r3 (scope relative) (span (offset 156) (line 4) (column 48) (len 10)) (segments (segment 0 (token "right") (name "right") (separator none) (span (offset 156) (line 4) (column 48) (len 5))) (segment 1 (token "port") (name "port") (separator dot) (span (offset 162) (line 4) (column 54) (len 4)))))
    (reference r4 (scope relative) (span (offset 206) (line 5) (column 39) (len 10)) (segments (segment 0 (token "ActionType") (name "ActionType") (separator none) (span (offset 206) (line 5) (column 39) (len 10)))))
    (reference r5 (scope relative) (span (offset 238) (line 6) (column 21) (len 16)) (segments (segment 0 (token "workflow") (name "workflow") (separator none) (span (offset 238) (line 6) (column 21) (len 8))) (segment 1 (token "execute") (name "execute") (separator dot) (span (offset 247) (line 6) (column 30) (len 7)))))
    (reference r6 (scope relative) (span (offset 259) (line 6) (column 42) (len 11)) (segments (segment 0 (token "replacement") (name "replacement") (separator none) (span (offset 259) (line 6) (column 42) (len 11)))))
  )
  (root (package (name "InterfaceUsageBodyPerform") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "host") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (interface-usage (form typed-connect) (part (binary (from (interface-end (multiplicity none) (target (ref r0)))) (to (interface-end (multiplicity none) (target (ref r1)))))) (body brace (flow-usage (kind flow) (visibility none) (declaration (declared (name "transmitted") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints (from (connector-end (multiplicity none) (target (ref r2)) (references none))) (to (connector-end (multiplicity none) (target (ref r3)) (references none)))))) (body (body semicolon))) (perform (target (action (name "dispatch") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body semicolon)) (perform (target (reference (action (ref r5)) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))))) (value none) (body semicolon)))))))))
)
~~~
