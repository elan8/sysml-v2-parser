# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 12 (Binding Connectors): Binding Connectors Example-1"))
~~~
# SOURCE
~~~sysml
package 'Binding Connectors Example-1' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	part def FuelPump;
	part def FuelTank;
	
	part vehicle : Vehicle {	
		part tank : FuelTankAssembly {
			port redefines fuelTankPort {
				out item redefines fuelSupply;
				in item redefines fuelReturn;
			}
			
			bind fuelTankPort.fuelSupply = pump.pumpOut;
			bind fuelTankPort.fuelReturn = tank.fuelIn;
			
			part pump : FuelPump {
				out item pumpOut : Fuel;
				in item pumpIn : Fuel;
			}
			
			part tank : FuelTank {
				out item fuelOut : Fuel;
				in item fuelIn : Fuel;
			}
		}
	} 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "12_binding_connectors_example_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Binding Connectors Example-1' {
    private import 'Port Example'::*;
    part def Vehicle;
    part def FuelPump;
    part def FuelTank;
    part vehicle : Vehicle {
        part tank : FuelTankAssembly {
            port :>> fuelTankPort {
                out item :>> fuelSupply;
                in item :>> fuelReturn;
            }
            bind fuelTankPort.fuelSupply = pump.pumpOut;
            bind fuelTankPort.fuelReturn = tank.fuelIn;
            part pump : FuelPump {
                out item pumpOut : Fuel;
                in item pumpIn : Fuel;
            }
            part tank : FuelTank {
                out item fuelOut : Fuel;
                in item fuelIn : Fuel;
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 57) (line 2) (column 17) (len 14)) (segments (segment 0 (token "'Port Example'") (name "Port Example") (separator none) (span (offset 57) (line 2) (column 17) (len 14)))))
    (reference r1 (scope relative) (span (offset 155) (line 8) (column 17) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 155) (line 8) (column 17) (len 7)))))
    (reference r2 (scope relative) (span (offset 180) (line 9) (column 15) (len 16)) (segments (segment 0 (token "FuelTankAssembly") (name "FuelTankAssembly") (separator none) (span (offset 180) (line 9) (column 15) (len 16)))))
    (reference r3 (scope relative) (span (offset 217) (line 10) (column 19) (len 12)) (segments (segment 0 (token "fuelTankPort") (name "fuelTankPort") (separator none) (span (offset 217) (line 10) (column 19) (len 12)))))
    (reference r4 (scope relative) (span (offset 255) (line 11) (column 24) (len 10)) (segments (segment 0 (token "fuelSupply") (name "fuelSupply") (separator none) (span (offset 255) (line 11) (column 24) (len 10)))))
    (reference r5 (scope relative) (span (offset 289) (line 12) (column 23) (len 10)) (segments (segment 0 (token "fuelReturn") (name "fuelReturn") (separator none) (span (offset 289) (line 12) (column 23) (len 10)))))
    (reference r6 (scope relative) (span (offset 424) (line 18) (column 16) (len 8)) (segments (segment 0 (token "FuelPump") (name "FuelPump") (separator none) (span (offset 424) (line 18) (column 16) (len 8)))))
    (reference r7 (scope relative) (span (offset 458) (line 19) (column 24) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 458) (line 19) (column 24) (len 4)))))
    (reference r8 (scope relative) (span (offset 485) (line 20) (column 22) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 485) (line 20) (column 22) (len 4)))))
    (reference r9 (scope relative) (span (offset 515) (line 23) (column 16) (len 8)) (segments (segment 0 (token "FuelTank") (name "FuelTank") (separator none) (span (offset 515) (line 23) (column 16) (len 8)))))
    (reference r10 (scope relative) (span (offset 549) (line 24) (column 24) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 549) (line 24) (column 24) (len 4)))))
    (reference r11 (scope relative) (span (offset 576) (line 25) (column 22) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 576) (line 25) (column 22) (len 4)))))
  )
  (root (package (name "Binding Connectors Example-1") (body brace (import (target (span (span (offset 57) (line 2) (column 17) (len 17))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 71) (line 2) (column 31) (len 3))) (separator (span (offset 71) (line 2) (column 31) (len 2))) (marker (span (offset 73) (line 2) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle") (modifiers) (body semicolon)) (part-def (name "FuelPump") (modifiers) (body semicolon)) (part-def (name "FuelTank") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "tank") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value none) (body brace (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (value none) (body semicolon)) (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (value none) (body semicolon)))) (bind) (bind) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "pump") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "pumpOut") (short-name none) (type (ref r7)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "pumpIn") (short-name none) (type (ref r8)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "tank") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuelOut") (short-name none) (type (ref r10)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuelIn") (short-name none) (type (ref r11)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))))))
)
~~~
