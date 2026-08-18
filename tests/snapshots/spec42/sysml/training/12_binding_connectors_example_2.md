# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 12 (Binding Connectors): Binding Connectors Example-2"))
~~~
# SOURCE
~~~sysml
package 'Binding Connectors Example-2' {
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
			
			part pump : FuelPump {
				out item pumpOut : Fuel = fuelTankPort.fuelSupply;
				in item pumpIn : Fuel;
			}
			
			part tank : FuelTank {
				out item fuelOut : Fuel;
				in item fuelIn : Fuel = fuelTankPort.fuelReturn;
			}
		}
	} 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "12_binding_connectors_example_2.md"
    (diagnostics
      (diagnostic (code "recovered_port_body_element") (severity error) (category parseerror) (span (offset 236) (line 11) (column 5) (len 35)) (message "unexpected token in port body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 236) (line 11) (column 5) (len 35)) (message "suppressed 1 cascading recovered diagnostic after earlier recovery errors"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Binding Connectors Example-2' {
    private import 'Port Example'::*;
    part def Vehicle;
    part def FuelPump;
    part def FuelTank;
    part vehicle : Vehicle {
        part tank : FuelTankAssembly {
            port  :>> fuelTankPort {
                out item redefines fuelSupply;
                in item redefines fuelReturn;
            }
            part pump : FuelPump {
                out item pumpOut : Fuel = fuelTankPort.fuelSupply;
                in item pumpIn : Fuel;
            }
            part tank : FuelTank {
                out item fuelOut : Fuel;
                in item fuelIn : Fuel = fuelTankPort.fuelReturn;
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
    (reference r3 (scope relative) (span (offset 325) (line 15) (column 16) (len 8)) (segments (segment 0 (token "FuelPump") (name "FuelPump") (separator none) (span (offset 325) (line 15) (column 16) (len 8)))))
    (reference r4 (scope relative) (span (offset 359) (line 16) (column 24) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 359) (line 16) (column 24) (len 4)))))
    (reference r5 (scope relative) (span (offset 366) (line 16) (column 31) (len 12)) (segments (segment 0 (token "fuelTankPort") (name "fuelTankPort") (separator none) (span (offset 366) (line 16) (column 31) (len 12)))))
    (reference r6 (scope relative) (span (offset 379) (line 16) (column 44) (len 10)) (segments (segment 0 (token "fuelSupply") (name "fuelSupply") (separator none) (span (offset 379) (line 16) (column 44) (len 10)))))
    (reference r7 (scope relative) (span (offset 412) (line 17) (column 22) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 412) (line 17) (column 22) (len 4)))))
    (reference r8 (scope relative) (span (offset 442) (line 20) (column 16) (len 8)) (segments (segment 0 (token "FuelTank") (name "FuelTank") (separator none) (span (offset 442) (line 20) (column 16) (len 8)))))
    (reference r9 (scope relative) (span (offset 476) (line 21) (column 24) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 476) (line 21) (column 24) (len 4)))))
    (reference r10 (scope relative) (span (offset 503) (line 22) (column 22) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 503) (line 22) (column 22) (len 4)))))
    (reference r11 (scope relative) (span (offset 510) (line 22) (column 29) (len 12)) (segments (segment 0 (token "fuelTankPort") (name "fuelTankPort") (separator none) (span (offset 510) (line 22) (column 29) (len 12)))))
    (reference r12 (scope relative) (span (offset 523) (line 22) (column 42) (len 10)) (segments (segment 0 (token "fuelReturn") (name "fuelReturn") (separator none) (span (offset 523) (line 22) (column 42) (len 10)))))
  )
  (root (package (name "Binding Connectors Example-2") (body brace (import (target (span (span (offset 57) (line 2) (column 17) (len 17))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 71) (line 2) (column 31) (len 3))) (separator (span (offset 71) (line 2) (column 31) (len 2))) (marker (span (offset 73) (line 2) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle") (body semicolon)) (part-def (name "FuelPump") (body semicolon)) (part-def (name "FuelTank") (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "tank") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "pump") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "pumpOut") (short-name none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 366) (line 16) (column 31) (len 23)) (member-access (base (expression (span (offset 366) (line 16) (column 31) (len 12)) (ref r5))) (separator dot) (member (ref r6))))))) (body semicolon)) (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "pumpIn") (short-name none) (type (ref r7)) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "tank") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuelOut") (short-name none) (type (ref r9)) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuelIn") (short-name none) (type (ref r10)) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 510) (line 22) (column 29) (len 23)) (member-access (base (expression (span (offset 510) (line 22) (column 29) (len 12)) (ref r11))) (separator dot) (member (ref r12))))))) (body semicolon)))))))))))
)
~~~
