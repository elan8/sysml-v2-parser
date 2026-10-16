# META
~~~sexpr
(snapshot (type malformed) (description "SysML DefinitionDeclaration rejects KerML disjoining headers while preserving valid siblings, comments and quoted names."))
~~~
# SOURCE
~~~sysml
package P {
    abstract action def A;
    abstract action def B disjoint from A;
    action def C :> A disjoint /* header clause */ from B;
    part def D disjoint from E;
    part def E;
    occurrence def F disjoint from E;
    verification def G disjoint from E;
    analysis def H disjoint from E;
    use case def I disjoint from E;
    action def 'disjoint from A';
    action def Fine {
        doc /* disjoint from is documentation here. */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "definition_disjoining_rejected.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 43) (line 3) (column 5) (len 38)) (message "unexpected token in package body"))
      (diagnostic (code "missing_body_or_semicolon") (severity error) (category parseerror) (span (offset 86) (line 4) (column 5) (len 54)) (message "expected ';' or '{' after action definition header"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 86) (line 4) (column 5) (len 54)) (message "suppressed 1 cascading missing_body_or_semicolon diagnostic after earlier recovery errors"))
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 193) (line 7) (column 5) (len 33)) (message "unexpected token in package body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 193) (line 7) (column 5) (len 33)) (message "suppressed 3 cascading recovered diagnostics after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    abstract action def A;
    abstract action def B disjoint from A;
    action def C :> A disjoint /* header clause */ from B;
    part def D disjoint from E;
    part def E;
    occurrence def F disjoint from E;
    verification def G disjoint from E;
    analysis def H disjoint from E;
    use case def I disjoint from E;
    action def 'disjoint from A';
    action def Fine {
        doc
        /* disjoint from is documentation here. */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
  )
  (root (package (name "P") (body brace (action-def (name "A") (modifiers (abstract (span (offset 16) (line 2) (column 5) (len 8)))) (specializes none) (body semicolon)) (malformed (code "recovered_package_body_element") (found "abstract action def B disjoint from A;") (span (offset 43) (line 3) (column 5) (len 38))) (malformed (code "missing_body_or_semicolon") (found "action def C :> A disjoint /* header clause */ from B;") (span (offset 86) (line 4) (column 5) (len 54))) (malformed (code "missing_body_or_semicolon") (found "part def D disjoint from E;") (span (offset 145) (line 5) (column 5) (len 27))) (part-def (name "E") (modifiers) (body semicolon)) (malformed (code "recovered_package_body_element") (found "occurrence def F disjoint from E;") (span (offset 193) (line 7) (column 5) (len 33))) (malformed (code "recovered_package_body_element") (found "verification def G disjoint from E;") (span (offset 231) (line 8) (column 5) (len 35))) (malformed (code "recovered_package_body_element") (found "analysis def H disjoint from E;") (span (offset 271) (line 9) (column 5) (len 31))) (malformed (code "recovered_package_body_element") (found "use case def I disjoint from E;") (span (offset 307) (line 10) (column 5) (len 31))) (action-def (name "disjoint from A") (modifiers) (specializes none) (body semicolon)) (action-def (name "Fine") (modifiers) (specializes none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 409) (line 13) (column 15) (len 38)) (normalized "disjoint from is documentation here. "))))))))
)
~~~
