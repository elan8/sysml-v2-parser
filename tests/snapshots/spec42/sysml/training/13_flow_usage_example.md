# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 13 (Flows): Flow Usage Example"))
~~~
# SOURCE
~~~sysml
package 'Flow Usage Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	part vehicle : Vehicle {
		part tankAssy : FuelTankAssembly;
		part eng : Engine;
		
		flow of Fuel
		  from tankAssy.fuelTankPort.fuelSupply
			to eng.engineFuelPort.fuelSupply;
			
		flow of Fuel
		  from eng.engineFuelPort.fuelReturn
			to tankAssy.fuelTankPort.fuelReturn;
	} 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "13_flow_usage_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Flow Usage Example' {
    private import 'Port Example'::*;
    part def Vehicle;
    part vehicle : Vehicle {
        part tankAssy : FuelTankAssembly;
        part eng : Engine;
        flow  of Fuel from tankAssy.fuelTankPort.fuelSupply to eng.engineFuelPort.fuelSupply;
        flow  of Fuel from eng.engineFuelPort.fuelReturn to tankAssy.fuelTankPort.fuelReturn;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 47) (line 2) (column 17) (len 14)) (segments (segment 0 (token "'Port Example'") (name "Port Example") (separator none) (span (offset 47) (line 2) (column 17) (len 14)))))
  )
  (root (package (name "Flow Usage Example") (body (import (target (span (span (offset 47) (line 2) (column 17) (len 17))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 61) (line 2) (column 31) (len 3))) (separator (span (offset 61) (line 2) (column 31) (len 2))) (marker (span (offset 63) (line 2) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle") (body semicolon)) (part-usage))))
)
~~~
