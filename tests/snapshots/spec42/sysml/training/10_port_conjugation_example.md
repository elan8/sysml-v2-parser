# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 10 (Ports): Port Conjugation Example"))
~~~
# SOURCE
~~~sysml
package 'Port Conjugation Example' {
	
	attribute def Temp;
	
	part def Fuel;
	
	port def FuelPort {
		attribute temperature : Temp;
		out item fuelSupply : Fuel;
		in item fuelReturn : Fuel;
	}
	
	part def FuelTank {
		port fuelTankPort : FuelPort;
	}
	
	part def Engine {
		port engineFuelPort : ~FuelPort;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "10_port_conjugation_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Port Conjugation Example' {
    attribute def Temp;
    part def Fuel;
    port def FuelPort {
        attribute temperature : Temp;
        out item fuelSupply : Fuel;
        in item fuelReturn : Fuel;
    }
    part def FuelTank {
        port fuelTankPort : FuelPort;
    }
    part def Engine {
        port engineFuelPort : ~FuelPort;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 127) (line 8) (column 27) (len 4)) (segments (segment 0 (token "Temp") (name "Temp") (separator none) (span (offset 127) (line 8) (column 27) (len 4)))))
    (reference r1 (scope relative) (span (offset 240) (line 14) (column 23) (len 8)) (segments (segment 0 (token "FuelPort") (name "FuelPort") (separator none) (span (offset 240) (line 14) (column 23) (len 8)))))
    (reference r2 (scope relative) (span (offset 299) (line 18) (column 26) (len 8)) (segments (segment 0 (token "FuelPort") (name "FuelPort") (separator none) (span (offset 299) (line 18) (column 26) (len 8)))))
  )
  (root (package (name "Port Conjugation Example") (body brace (attribute-def (name "Temp") (multiplicity none)) (part-def (name "Fuel") (body semicolon)) (port-def (name "FuelPort") (specializes none) (body brace (attribute-usage (declaration-name "temperature") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (item-usage) (item-usage))) (part-def (name "FuelTank") (body brace (port-usage (declaration-name "fuelTankPort") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Engine") (body brace (port-usage (declaration-name "engineFuelPort") (direction none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
