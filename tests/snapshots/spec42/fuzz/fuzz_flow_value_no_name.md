# META
~~~sexpr
(snapshot (type semantic) (description "Fuzz: flow usage with value and typing but no name preserves value in formatting"))
~~~
# SOURCE
~~~sysml
package P {
    part vehicle : Vehicle {
        part eng : Engine;

        flow = FuelFlow of Fuel
            from tank.fuelSupply
                to eng.engineFuelPort;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_flow_value_no_name.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    part vehicle : Vehicle {
        part eng : Engine;
        flow = FuelFlow of Fuel from tank.fuelSupply to eng.engineFuelPort;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 31) (line 2) (column 20) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 31) (line 2) (column 20) (len 7)))))
    (reference r1 (scope relative) (span (offset 60) (line 3) (column 20) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 60) (line 3) (column 20) (len 6)))))
    (reference r2 (scope relative) (span (offset 84) (line 5) (column 16) (len 8)) (segments (segment 0 (token "FuelFlow") (name "FuelFlow") (separator none) (span (offset 84) (line 5) (column 16) (len 8)))))
    (reference r3 (scope relative) (span (offset 96) (line 5) (column 28) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 96) (line 5) (column 28) (len 4)))))
    (reference r4 (scope relative) (span (offset 118) (line 6) (column 18) (len 15)) (segments (segment 0 (token "tank") (name "tank") (separator none) (span (offset 118) (line 6) (column 18) (len 4))) (segment 1 (token "fuelSupply") (name "fuelSupply") (separator dot) (span (offset 123) (line 6) (column 23) (len 10)))))
    (reference r5 (scope relative) (span (offset 153) (line 7) (column 20) (len 18)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 153) (line 7) (column 20) (len 3))) (segment 1 (token "engineFuelPort") (name "engineFuelPort") (separator dot) (span (offset 157) (line 7) (column 24) (len 14)))))
  )
  (root (package (name "P") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (flow-usage (kind flow) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 84) (line 5) (column 16) (len 8)) (ref r2))))) (payload (name none) (type (ref r3)) (conjugated false) (multiplicity none)) (endpoints (from (connector-end (multiplicity none) (target (ref r4)) (references none))) (to (connector-end (multiplicity none) (target (ref r5)) (references none)))))) (body (body semicolon))))))))
)
~~~
