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
    (reference r4 (scope relative) (span (offset 442) (line 20) (column 16) (len 8)) (segments (segment 0 (token "FuelTank") (name "FuelTank") (separator none) (span (offset 442) (line 20) (column 16) (len 8)))))
  )
  (root (package (name "Binding Connectors Example-2") (body brace (import (target (span (span (offset 57) (line 2) (column 17) (len 17))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 71) (line 2) (column 31) (len 3))) (separator (span (offset 71) (line 2) (column 31) (len 2))) (marker (span (offset 73) (line 2) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle") (body semicolon)) (part-def (name "FuelPump") (body semicolon)) (part-def (name "FuelTank") (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "tank") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "pump") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "pumpOut")) (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "pumpIn")))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "tank") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuelOut")) (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuelIn")))))))))))
)
~~~
