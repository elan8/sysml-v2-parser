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
    (reference r1 (scope relative) (span (offset 157) (line 9) (column 25) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 157) (line 9) (column 25) (len 4)))))
    (reference r2 (scope relative) (span (offset 186) (line 10) (column 24) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 186) (line 10) (column 24) (len 4)))))
    (reference r3 (scope relative) (span (offset 240) (line 14) (column 23) (len 8)) (segments (segment 0 (token "FuelPort") (name "FuelPort") (separator none) (span (offset 240) (line 14) (column 23) (len 8)))))
    (reference r4 (scope relative) (span (offset 299) (line 18) (column 26) (len 8)) (segments (segment 0 (token "FuelPort") (name "FuelPort") (separator none) (span (offset 299) (line 18) (column 26) (len 8)))))
  )
  (root (package (name "Port Conjugation Example") (body brace (attribute-def (declaration-name "Temp") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (part-def (name "Fuel") (body semicolon)) (port-def (name "FuelPort") (specializes none) (body brace (attribute-usage (declaration-name "temperature") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuelSupply") (short-name none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuelReturn") (short-name none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "FuelTank") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelTankPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Engine") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engineFuelPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
