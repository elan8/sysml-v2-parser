# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 10 (Ports): Port Example"))
~~~
# SOURCE
~~~sysml
package 'Port Example' {
	
	attribute def Temp;
	
	part def Fuel;
	
	port def FuelOutPort {
		attribute temperature : Temp;
		out item fuelSupply : Fuel;
		in item fuelReturn : Fuel;
	}
	
	port def FuelInPort {
		attribute temperature : Temp;
		in item fuelSupply : Fuel;
		out item fuelReturn : Fuel;
	}
	
	part def FuelTankAssembly {
		port fuelTankPort : FuelOutPort;
	}
	
	part def Engine {
		port engineFuelPort : FuelInPort;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "10_port_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Port Example' {
    attribute def Temp;
    part def Fuel;
    port def FuelOutPort {
        attribute temperature : Temp;
        out item fuelSupply : Fuel;
        in item fuelReturn : Fuel;
    }
    port def FuelInPort {
        attribute temperature : Temp;
        in item fuelSupply : Fuel;
        out item fuelReturn : Fuel;
    }
    part def FuelTankAssembly {
        port fuelTankPort : FuelOutPort;
    }
    part def Engine {
        port engineFuelPort : FuelInPort;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 118) (line 8) (column 27) (len 4)) (segments (segment 0 (token "Temp") (name "Temp") (separator none) (span (offset 118) (line 8) (column 27) (len 4)))))
    (reference r1 (scope relative) (span (offset 237) (line 14) (column 27) (len 4)) (segments (segment 0 (token "Temp") (name "Temp") (separator none) (span (offset 237) (line 14) (column 27) (len 4)))))
    (reference r2 (scope relative) (span (offset 358) (line 20) (column 23) (len 11)) (segments (segment 0 (token "FuelOutPort") (name "FuelOutPort") (separator none) (span (offset 358) (line 20) (column 23) (len 11)))))
    (reference r3 (scope relative) (span (offset 419) (line 24) (column 25) (len 10)) (segments (segment 0 (token "FuelInPort") (name "FuelInPort") (separator none) (span (offset 419) (line 24) (column 25) (len 10)))))
  )
  (root (package (name "Port Example") (body (attribute-def) (part-def (name "Fuel") (body semicolon)) (port-def (name "FuelOutPort") (specializes none) (body (attribute-usage (declaration-name "temperature") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (item-usage) (item-usage))) (port-def (name "FuelInPort") (specializes none) (body (attribute-usage (declaration-name "temperature") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (item-usage) (item-usage))) (part-def (name "FuelTankAssembly") (body (port-usage (declaration-name "fuelTankPort") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Engine") (body (port-usage (declaration-name "engineFuelPort") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
