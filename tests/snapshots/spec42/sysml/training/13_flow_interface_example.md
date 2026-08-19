# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 13 (Flows): Flow Interface Example"))
~~~
# SOURCE
~~~sysml
package 'Flow Interface Example' {
	private import 'Port Example'::*;
	
	part def Vehicle;
	
	interface def FuelInterface {
		end supplierPort : FuelOutPort;
		end consumerPort : FuelInPort;
		
		flow supplierPort.fuelSupply to consumerPort.fuelSupply;			
		flow consumerPort.fuelReturn to supplierPort.fuelReturn;
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
  (document "13_flow_interface_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Flow Interface Example' {
    private import 'Port Example'::*;
    part def Vehicle;
    interface def FuelInterface {
        end supplierPort : FuelOutPort;
        end consumerPort : FuelInPort;
        flow from supplierPort.fuelSupply to consumerPort.fuelSupply;
        flow from consumerPort.fuelReturn to supplierPort.fuelReturn;
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
    (reference r0 (scope relative) (span (offset 51) (line 2) (column 17) (len 14)) (segments (segment 0 (token "'Port Example'") (name "Port Example") (separator none) (span (offset 51) (line 2) (column 17) (len 14)))))
    (reference r1 (scope relative) (span (offset 145) (line 7) (column 22) (len 11)) (segments (segment 0 (token "FuelOutPort") (name "FuelOutPort") (separator none) (span (offset 145) (line 7) (column 22) (len 11)))))
    (reference r2 (scope relative) (span (offset 179) (line 8) (column 22) (len 10)) (segments (segment 0 (token "FuelInPort") (name "FuelInPort") (separator none) (span (offset 179) (line 8) (column 22) (len 10)))))
    (reference r3 (scope relative) (span (offset 336) (line 14) (column 17) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 336) (line 14) (column 17) (len 7)))))
    (reference r4 (scope relative) (span (offset 365) (line 15) (column 19) (len 16)) (segments (segment 0 (token "FuelTankAssembly") (name "FuelTankAssembly") (separator none) (span (offset 365) (line 15) (column 19) (len 16)))))
    (reference r5 (scope relative) (span (offset 398) (line 16) (column 14) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 398) (line 16) (column 14) (len 6)))))
  )
  (root (package (name "Flow Interface Example") (body brace (import (target (span (span (offset 51) (line 2) (column 17) (len 17))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 65) (line 2) (column 31) (len 3))) (separator (span (offset 65) (line 2) (column 31) (len 2))) (marker (span (offset 67) (line 2) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle") (body semicolon)) (interface-def (name "FuelInterface") (modifiers) (specializes none) (body brace (end (short-name none) (identity (declaration (name "supplierPort") (span (offset 130) (line 7) (column 7) (len 12)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (redefines none) (crosses none)) (end (short-name none) (identity (declaration (name "consumerPort") (span (offset 164) (line 8) (column 7) (len 12)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (references none) (redefines none) (crosses none)) (flow-usage) (flow-usage))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "tankAssy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (interface-usage))))))
)
~~~
