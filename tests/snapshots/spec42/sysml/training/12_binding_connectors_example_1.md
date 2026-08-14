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
      (diagnostic (code "recovered_port_body_element") (severity error) (category parseerror) (span (offset 236) (line 11) (column 5) (len 35)) (message "unexpected token in port body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 236) (line 11) (column 5) (len 35)) (message "suppressed 1 cascading recovered diagnostic after earlier recovery errors"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 439) (line 19) (column 5) (len 55)) (message "unexpected keyword `out` in part usage body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 530) (line 24) (column 5) (len 55)) (message "unexpected keyword `out` in part usage body"))
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
            port  :>> fuelTankPort {
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
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 57) (line 2) (column 17) (len 14)) (segments (segment 0 (token "'Port Example'") (name "Port Example") (separator none) (span (offset 57) (line 2) (column 17) (len 14)))))
  )
  (root (package (name "Binding Connectors Example-1") (body (import (target (span (span (offset 57) (line 2) (column 17) (len 17))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 71) (line 2) (column 31) (len 3))) (separator (span (offset 71) (line 2) (column 31) (len 2))) (marker (span (offset 73) (line 2) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle") (body semicolon)) (part-def (name "FuelPump") (body semicolon)) (part-def (name "FuelTank") (body semicolon)) (part-usage))))
)
~~~
