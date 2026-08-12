# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 11 (Interfaces): Interface Example"))
~~~
# SOURCE
~~~sysml
package 'Interface Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	interface def FuelInterface {
		end supplierPort : FuelOutPort;
		end consumerPort : FuelInPort;
	}
	
	part vehicle : Vehicle {	
		part tankAssy : FuelTankAssembly;		
		part eng : Engine;
		
		interface : FuelInterface connect 
			supplierPort ::> tankAssy.fuelTankPort to 
			consumerPort ::> eng.engineFuelPort;
	} 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "11_interface_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Interface Example' {
    private import 'Port Example'::*;
    part def Vehicle;
    interface def FuelInterface {
        end supplierPort : FuelOutPort;
        end consumerPort : FuelInPort;
    }
    part vehicle : Vehicle {
        part tankAssy : FuelTankAssembly;
        part eng : Engine;
        interface : FuelInterface connect tankAssy.fuelTankPort to eng.engineFuelPort;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 46) (line 2) (column 17) (len 14)) (segments (segment 0 (token "'Port Example'") (name "Port Example") (separator none) (span (offset 46) (line 2) (column 17) (len 14)))))
  )
  (root (package (name "Interface Example") (body (import (target (span (span (offset 46) (line 2) (column 17) (len 17))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 60) (line 2) (column 31) (len 3))) (separator (span (offset 60) (line 2) (column 31) (len 2))) (marker (span (offset 62) (line 2) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle") (body semicolon)) (interface-def) (part-usage))))
)
~~~
