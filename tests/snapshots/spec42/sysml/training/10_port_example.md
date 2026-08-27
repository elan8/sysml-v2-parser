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
    (reference r1 (scope relative) (span (offset 148) (line 9) (column 25) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 148) (line 9) (column 25) (len 4)))))
    (reference r2 (scope relative) (span (offset 177) (line 10) (column 24) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 177) (line 10) (column 24) (len 4)))))
    (reference r3 (scope relative) (span (offset 237) (line 14) (column 27) (len 4)) (segments (segment 0 (token "Temp") (name "Temp") (separator none) (span (offset 237) (line 14) (column 27) (len 4)))))
    (reference r4 (scope relative) (span (offset 266) (line 15) (column 24) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 266) (line 15) (column 24) (len 4)))))
    (reference r5 (scope relative) (span (offset 296) (line 16) (column 25) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 296) (line 16) (column 25) (len 4)))))
    (reference r6 (scope relative) (span (offset 358) (line 20) (column 23) (len 11)) (segments (segment 0 (token "FuelOutPort") (name "FuelOutPort") (separator none) (span (offset 358) (line 20) (column 23) (len 11)))))
    (reference r7 (scope relative) (span (offset 419) (line 24) (column 25) (len 10)) (segments (segment 0 (token "FuelInPort") (name "FuelInPort") (separator none) (span (offset 419) (line 24) (column 25) (len 10)))))
  )
  (root (package (name "Port Example") (body brace (attribute-def (declaration-name "Temp") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (part-def (name "Fuel") (modifiers) (body semicolon)) (port-def (name "FuelOutPort") (modifiers) (specializes none) (body brace (attribute-usage (declaration-name "temperature") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuelSupply") (short-name none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuelReturn") (short-name none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (port-def (name "FuelInPort") (modifiers) (specializes none) (body brace (attribute-usage (declaration-name "temperature") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuelSupply") (short-name none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuelReturn") (short-name none) (type (ref r5)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "FuelTankAssembly") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelTankPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Engine") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engineFuelPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
