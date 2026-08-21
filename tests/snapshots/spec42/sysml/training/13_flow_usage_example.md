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
        flow of Fuel from tankAssy.fuelTankPort.fuelSupply to eng.engineFuelPort.fuelSupply;
        flow of Fuel from eng.engineFuelPort.fuelReturn to tankAssy.fuelTankPort.fuelReturn;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 47) (line 2) (column 17) (len 14)) (segments (segment 0 (token "'Port Example'") (name "Port Example") (separator none) (span (offset 47) (line 2) (column 17) (len 14)))))
    (reference r1 (scope relative) (span (offset 105) (line 6) (column 17) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 105) (line 6) (column 17) (len 7)))))
    (reference r2 (scope relative) (span (offset 133) (line 7) (column 19) (len 16)) (segments (segment 0 (token "FuelTankAssembly") (name "FuelTankAssembly") (separator none) (span (offset 133) (line 7) (column 19) (len 16)))))
    (reference r3 (scope relative) (span (offset 164) (line 8) (column 14) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 164) (line 8) (column 14) (len 6)))))
    (reference r4 (scope relative) (span (offset 185) (line 10) (column 11) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 185) (line 10) (column 11) (len 4)))))
    (reference r5 (scope relative) (span (offset 199) (line 11) (column 10) (len 32)) (segments (segment 0 (token "tankAssy") (name "tankAssy") (separator none) (span (offset 199) (line 11) (column 10) (len 8))) (segment 1 (token "fuelTankPort") (name "fuelTankPort") (separator dot) (span (offset 208) (line 11) (column 19) (len 12))) (segment 2 (token "fuelSupply") (name "fuelSupply") (separator dot) (span (offset 221) (line 11) (column 32) (len 10)))))
    (reference r6 (scope relative) (span (offset 238) (line 12) (column 7) (len 29)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 238) (line 12) (column 7) (len 3))) (segment 1 (token "engineFuelPort") (name "engineFuelPort") (separator dot) (span (offset 242) (line 12) (column 11) (len 14))) (segment 2 (token "fuelSupply") (name "fuelSupply") (separator dot) (span (offset 257) (line 12) (column 26) (len 10)))))
    (reference r7 (scope relative) (span (offset 283) (line 14) (column 11) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 283) (line 14) (column 11) (len 4)))))
    (reference r8 (scope relative) (span (offset 297) (line 15) (column 10) (len 29)) (segments (segment 0 (token "eng") (name "eng") (separator none) (span (offset 297) (line 15) (column 10) (len 3))) (segment 1 (token "engineFuelPort") (name "engineFuelPort") (separator dot) (span (offset 301) (line 15) (column 14) (len 14))) (segment 2 (token "fuelReturn") (name "fuelReturn") (separator dot) (span (offset 316) (line 15) (column 29) (len 10)))))
    (reference r9 (scope relative) (span (offset 333) (line 16) (column 7) (len 32)) (segments (segment 0 (token "tankAssy") (name "tankAssy") (separator none) (span (offset 333) (line 16) (column 7) (len 8))) (segment 1 (token "fuelTankPort") (name "fuelTankPort") (separator dot) (span (offset 342) (line 16) (column 16) (len 12))) (segment 2 (token "fuelReturn") (name "fuelReturn") (separator dot) (span (offset 355) (line 16) (column 29) (len 10)))))
  )
  (root (package (name "Flow Usage Example") (body brace (import (target (span (span (offset 47) (line 2) (column 17) (len 17))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 61) (line 2) (column 31) (len 3))) (separator (span (offset 61) (line 2) (column 31) (len 2))) (marker (span (offset 63) (line 2) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "tankAssy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload (name none) (type (ref r4)) (conjugated false) (multiplicity none)) (endpoints (from (connector-end (multiplicity none) (target (ref r5)) (references none))) (to (connector-end (multiplicity none) (target (ref r6)) (references none)))))) (body (body semicolon))) (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload (name none) (type (ref r7)) (conjugated false) (multiplicity none)) (endpoints (from (connector-end (multiplicity none) (target (ref r8)) (references none))) (to (connector-end (multiplicity none) (target (ref r9)) (references none)))))) (body (body semicolon))))))))
)
~~~
