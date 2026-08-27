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
        interface : FuelInterface connect supplierPort ::> tankAssy.fuelTankPort to consumerPort ::> eng.engineFuelPort;
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
    (reference r3 (scope relative) (span (offset 201) (line 10) (column 8) (len 23)) (segments (segment 0 (token "supplierPort") (name "supplierPort") (separator none) (span (offset 201) (line 10) (column 8) (len 12))) (segment 1 (token "fuelSupply") (name "fuelSupply") (separator dot) (span (offset 214) (line 10) (column 21) (len 10)))))
    (reference r4 (scope relative) (span (offset 228) (line 10) (column 35) (len 23)) (segments (segment 0 (token "consumerPort") (name "consumerPort") (separator none) (span (offset 228) (line 10) (column 35) (len 12))) (segment 1 (token "fuelSupply") (name "fuelSupply") (separator dot) (span (offset 241) (line 10) (column 48) (len 10)))))
    (reference r5 (scope relative) (span (offset 263) (line 11) (column 8) (len 23)) (segments (segment 0 (token "consumerPort") (name "consumerPort") (separator none) (span (offset 263) (line 11) (column 8) (len 12))) (segment 1 (token "fuelReturn") (name "fuelReturn") (separator dot) (span (offset 276) (line 11) (column 21) (len 10)))))
    (reference r6 (scope relative) (span (offset 290) (line 11) (column 35) (len 23)) (segments (segment 0 (token "supplierPort") (name "supplierPort") (separator none) (span (offset 290) (line 11) (column 35) (len 12))) (segment 1 (token "fuelReturn") (name "fuelReturn") (separator dot) (span (offset 303) (line 11) (column 48) (len 10)))))
    (reference r7 (scope relative) (span (offset 336) (line 14) (column 17) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 336) (line 14) (column 17) (len 7)))))
    (reference r8 (scope relative) (span (offset 365) (line 15) (column 19) (len 16)) (segments (segment 0 (token "FuelTankAssembly") (name "FuelTankAssembly") (separator none) (span (offset 365) (line 15) (column 19) (len 16)))))
    (reference r9 (scope relative) (span (offset 398) (line 16) (column 14) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 398) (line 16) (column 14) (len 6)))))
    (reference r10 (scope relative) (span (offset 466) (line 19) (column 21) (len 21)) (segments (segment 0 (token "tankAssy") (name "tankAssy") (separator none) (span (offset 466) (line 19) (column 21) (len 8))) (segment 1 (token "fuelTankPort") (name "fuelTankPort") (separator dot) (span (offset 475) (line 19) (column 30) (len 12)))))
    (reference r11 (scope relative) (span (offset 512) (line 20) (column 21) (len 18)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 512) (line 20) (column 21) (len 3))) (segment 1 (token "engineFuelPort") (name "engineFuelPort") (separator dot) (span (offset 516) (line 20) (column 25) (len 14)))))
  )
  (root (package (name "Flow Interface Example") (body brace (import (target (span (span (offset 51) (line 2) (column 17) (len 17))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 65) (line 2) (column 31) (len 3))) (separator (span (offset 65) (line 2) (column 31) (len 2))) (marker (span (offset 67) (line 2) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle") (modifiers) (body semicolon)) (interface-def (name "FuelInterface") (modifiers) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "supplierPort") (span (offset 130) (line 7) (column 7) (len 12)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "consumerPort") (span (offset 164) (line 8) (column 7) (len 12)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (references none) (multiplicity none) (redefines none) (crosses none)) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r3)) (references none))) (to (connector-end (multiplicity none) (target (ref r4)) (references none))))) (body (body semicolon))) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r5)) (references none))) (to (connector-end (multiplicity none) (target (ref r6)) (references none))))) (body (body semicolon))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "tankAssy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (interface-usage (form typed-connect) (part (binary (from (interface-end (multiplicity none) (target (named (name "supplierPort") (references symbol) (target (ref r10)))))) (to (interface-end (multiplicity none) (target (named (name "consumerPort") (references symbol) (target (ref r11)))))))) (body semicolon)))))))
)
~~~
