# META
~~~sexpr
(snapshot (type semantic) (description "A port body redefines an inherited feature with the prefix `:>>` form, without repeating a kind keyword."))
~~~
# SOURCE
~~~sysml
package Power {
    port def DevicePower {
        attribute maxCurrent : Real;
    }

    part def Microcontroller {
        port pwr : DevicePower {
            :>> maxCurrent = 0.02 [A];
        }
    }

    port def LimitedPower :> DevicePower {
        :>> maxCurrent = 0.5 [A];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "port_body_prefix_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Power {
    port def DevicePower {
        attribute maxCurrent : Real;
    }
    part def Microcontroller {
        port pwr : DevicePower {
            attribute :>> maxCurrent = 0.02 [A];
        }
    }
    port def LimitedPower :> DevicePower {
        attribute :>> maxCurrent = 0.5 [A];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 74) (line 3) (column 32) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 74) (line 3) (column 32) (len 4)))))
    (reference r1 (scope relative) (span (offset 137) (line 7) (column 20) (len 11)) (segments (segment 0 (token "DevicePower") (name "DevicePower") (separator none) (span (offset 137) (line 7) (column 20) (len 11)))))
    (reference r2 (scope relative) (span (offset 236) (line 12) (column 30) (len 11)) (segments (segment 0 (token "DevicePower") (name "DevicePower") (separator none) (span (offset 236) (line 12) (column 30) (len 11)))))
    (reference r3 (scope relative) (span (offset 262) (line 13) (column 13) (len 10)) (segments (segment 0 (token "maxCurrent") (name "maxCurrent") (separator none) (span (offset 262) (line 13) (column 13) (len 10)))))
  )
  (root (package (name "Power") (body (port-def (name "DevicePower") (specializes none) (body (attribute-usage (declaration-name "maxCurrent") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Microcontroller") (body (port-usage (declaration-name "pwr") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (element-count 1))))) (port-def (name "LimitedPower") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r2)))) (body (attribute-usage (declaration-name none) (direction none) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 275) (line 13) (column 26) (len 7)) (literal-with-unit (value (expression (span (offset 275) (line 13) (column 26) (len 3)) (real "0.5"))) (unit (expression (span (offset 280) (line 13) (column 31) (len 1)) (bracket (expression (span (offset 280) (line 13) (column 31) (len 1)) (unit "A")))))))))) (body semicolon)))))))
)
~~~
