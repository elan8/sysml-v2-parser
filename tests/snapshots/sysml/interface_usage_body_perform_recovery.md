# META
~~~sexpr
(snapshot (type recovery) (description "A malformed perform reference recovers within an InterfaceUsage body without leaking its partial references or consuming the following typed perform member (SysML textual BNF 724-759, 374-390; pinned Pilot SysML.xtext 1109-1144)."))
~~~
# SOURCE
~~~sysml
package InterfaceUsageBodyPerformRecovery {
    part host {
        interface link connect left.port to right.port {
            perform invalid :>> dangling.;
            perform retained.perform;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "interface_usage_body_perform_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_interface_usage_body_element") (severity error) (category parseerror) (span (offset 129) (line 4) (column 13) (len 43)) (message "unexpected token in interface usage body"))
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
    (reference r0 (scope relative) (span (offset 91) (line 3) (column 32) (len 9)) (segments (segment 0 (token "left") (name "left") (separator none) (span (offset 91) (line 3) (column 32) (len 4))) (segment 1 (token "port") (name "port") (separator dot) (span (offset 96) (line 3) (column 37) (len 4)))))
    (reference r1 (scope relative) (span (offset 104) (line 3) (column 45) (len 10)) (segments (segment 0 (token "right") (name "right") (separator none) (span (offset 104) (line 3) (column 45) (len 5))) (segment 1 (token "port") (name "port") (separator dot) (span (offset 110) (line 3) (column 51) (len 4)))))
    (reference r2 (scope relative) (span (offset 180) (line 5) (column 21) (len 16)) (segments (segment 0 (token "retained") (name "retained") (separator none) (span (offset 180) (line 5) (column 21) (len 8))) (segment 1 (token "perform") (name "perform") (separator dot) (span (offset 189) (line 5) (column 30) (len 7)))))
  )
  (root (package (name "InterfaceUsageBodyPerformRecovery") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "host") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (interface-usage (form typed-connect) (part (binary (from (interface-end (multiplicity none) (target (ref r0)))) (to (interface-end (multiplicity none) (target (ref r1)))))) (body brace (malformed (code "recovered_interface_usage_body_element") (found "perform invalid :>> dangling.;") (span (offset 129) (line 4) (column 13) (len 43))) (perform (target (reference (action (ref r2)) (redefines none))) (value none) (body semicolon)))))))))
)
~~~
