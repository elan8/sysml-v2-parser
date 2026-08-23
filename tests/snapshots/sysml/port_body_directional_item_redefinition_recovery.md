# META
~~~sexpr
(snapshot (type recovery) (description "SysML ItemUsage is OccurrenceUsagePrefix `item` Usage (SysML-textual-bnf.kebnf 564, 616; Pilot SysML.xtext 917): a reserved `redefines` begins the anonymous usage's FeatureSpecializationPart, while a malformed first redefinition recovers before the following directioned anonymous item usage."))
~~~
# SOURCE
~~~sysml
package P {
    port p {
        out item redefines ;
        in item redefines fuelReturn;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "port_body_directional_item_redefinition_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_port_body_element") (severity error) (category parseerror) (span (offset 33) (line 3) (column 9) (len 29)) (message "unexpected token in port body"))
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
    (reference r0 (scope relative) (span (offset 80) (line 4) (column 27) (len 10)) (segments (segment 0 (token "fuelReturn") (name "fuelReturn") (separator none) (span (offset 80) (line 4) (column 27) (len 10)))))
  )
  (root (package (name "P") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (malformed (code "recovered_port_body_element") (found "out item redefines ;") (span (offset 33) (line 3) (column 9) (len 29))) (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (value none) (body semicolon)))))))
)
~~~
