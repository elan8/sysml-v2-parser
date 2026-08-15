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
    (reference r1 (scope relative) (span (offset 140) (line 7) (column 22) (len 11)) (segments (segment 0 (token "FuelOutPort") (name "FuelOutPort") (separator none) (span (offset 140) (line 7) (column 22) (len 11)))))
    (reference r2 (scope relative) (span (offset 174) (line 8) (column 22) (len 10)) (segments (segment 0 (token "FuelInPort") (name "FuelInPort") (separator none) (span (offset 174) (line 8) (column 22) (len 10)))))
  )
  (root (package (name "Interface Example") (body brace (import (target (span (span (offset 46) (line 2) (column 17) (len 17))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 60) (line 2) (column 31) (len 3))) (separator (span (offset 60) (line 2) (column 31) (len 2))) (marker (span (offset 62) (line 2) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle") (body semicolon)) (interface-def (name "FuelInterface") (specializes none) (body brace (end (identity (declaration (name "supplierPort") (span (offset 125) (line 7) (column 7) (len 12)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (redefines none) (crosses none)) (end (identity (declaration (name "consumerPort") (span (offset 159) (line 8) (column 7) (len 12)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (references none) (redefines none) (crosses none)))) (part-usage))))
)
~~~
