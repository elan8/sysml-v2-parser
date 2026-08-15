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
      (diagnostic (code "missing_semicolon") (severity error) (category parseerror) (span (offset 77) (line 5) (column 9) (len 100)) (message "missing semicolon before next declaration"))
    )
  )
)
~~~
# FORMAT
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
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "P") (body brace (part-usage))))
)
~~~
