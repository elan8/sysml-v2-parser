# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): CalculationTest"))
~~~
# SOURCE
~~~sysml
package CalculationExample {
	private import ISQ::*;
	private import NumericalFunctions::*;
	
	part def VehiclePart {
		attribute m : MassValue;
	}
	
	part def Vehicle :> VehiclePart;
	
	part vehicle : Vehicle {		
		part eng : VehiclePart;		
		part trans : VehiclePart;
		attribute ::> m = ms.totalMass;
	}
	
	calc def MassSum {
		in partMasses : MassValue[0..*];
		return totalMass : MassValue = sum(partMasses);
	}
	
	calc ms: MassSum {
		in partMasses = (vehicle.eng.m, vehicle.trans.m);
		return totalMass;
	}
	
	part vehicles[*] = (vehicle, vehicle);
	attribute masses1[*] = (vehicles as VehiclePart).m;
	attribute masses2[*] = (vehicles as vehicle).m;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "calculation_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package CalculationExample {
    private import ISQ::*;
    private import NumericalFunctions::*;
    part def VehiclePart {
        attribute m : MassValue;
    }
    part def Vehicle :> VehiclePart;
    part vehicle : Vehicle {
        part eng : VehiclePart;
        part trans : VehiclePart;
        attribute ::> m = ms.totalMass;
    }
    calc def MassSum {
        in partMasses : MassValue[0..*];
        return totalMass : MassValue = sum(partMasses);
    }
    calc def ms : MassSum {
        in partMasses = (vehicle.eng.m, vehicle.trans.m);
        return totalMass;
    }
    part vehicles[*] = (vehicle, vehicle);
    attribute def masses1[*] = (vehicles as VehiclePart).m;
    attribute def masses2[*] = (vehicles as vehicle).m;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 45) (line 2) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 45) (line 2) (column 17) (len 3)))))
    (reference r1 (scope relative) (span (offset 69) (line 3) (column 17) (len 18)) (segments (segment 0 (token "NumericalFunctions") (name "NumericalFunctions") (separator none) (span (offset 69) (line 3) (column 17) (len 18)))))
    (reference r2 (scope relative) (span (offset 134) (line 6) (column 17) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 134) (line 6) (column 17) (len 9)))))
    (reference r3 (scope relative) (span (offset 202) (line 11) (column 17) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 202) (line 11) (column 17) (len 7)))))
    (reference r4 (scope relative) (span (offset 227) (line 12) (column 14) (len 11)) (segments (segment 0 (token "VehiclePart") (name "VehiclePart") (separator none) (span (offset 227) (line 12) (column 14) (len 11)))))
    (reference r5 (scope relative) (span (offset 257) (line 13) (column 16) (len 11)) (segments (segment 0 (token "VehiclePart") (name "VehiclePart") (separator none) (span (offset 257) (line 13) (column 16) (len 11)))))
    (reference r6 (scope relative) (span (offset 286) (line 14) (column 17) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 286) (line 14) (column 17) (len 1)))))
    (reference r7 (scope relative) (span (offset 290) (line 14) (column 21) (len 2)) (segments (segment 0 (token "ms") (name "ms") (separator none) (span (offset 290) (line 14) (column 21) (len 2)))))
    (reference r8 (scope relative) (span (offset 293) (line 14) (column 24) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 293) (line 14) (column 24) (len 9)))))
    (reference r9 (scope relative) (span (offset 537) (line 27) (column 22) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 537) (line 27) (column 22) (len 7)))))
    (reference r10 (scope relative) (span (offset 546) (line 27) (column 31) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 546) (line 27) (column 31) (len 7)))))
    (reference r11 (scope relative) (span (offset 581) (line 28) (column 26) (len 8)) (segments (segment 0 (token "vehicles") (name "vehicles") (separator none) (span (offset 581) (line 28) (column 26) (len 8)))))
    (reference r12 (scope relative) (span (offset 593) (line 28) (column 38) (len 11)) (segments (segment 0 (token "VehiclePart") (name "VehiclePart") (separator none) (span (offset 593) (line 28) (column 38) (len 11)))))
    (reference r13 (scope relative) (span (offset 606) (line 28) (column 51) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 606) (line 28) (column 51) (len 1)))))
    (reference r14 (scope relative) (span (offset 634) (line 29) (column 26) (len 8)) (segments (segment 0 (token "vehicles") (name "vehicles") (separator none) (span (offset 634) (line 29) (column 26) (len 8)))))
    (reference r15 (scope relative) (span (offset 646) (line 29) (column 38) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 646) (line 29) (column 38) (len 7)))))
    (reference r16 (scope relative) (span (offset 655) (line 29) (column 47) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 655) (line 29) (column 47) (len 1)))))
  )
  (root (package (name "CalculationExample") (body brace (import (target (span (span (offset 45) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 48) (line 2) (column 20) (len 3))) (separator (span (offset 48) (line 2) (column 20) (len 2))) (marker (span (offset 50) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 69) (line 3) (column 17) (len 21))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 87) (line 3) (column 35) (len 3))) (separator (span (offset 87) (line 3) (column 35) (len 2))) (marker (span (offset 89) (line 3) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "VehiclePart") (body brace (attribute-usage (declaration-name "m") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Vehicle") (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "eng") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "trans") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r6)))) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 290) (line 14) (column 21) (len 12)) (member-access (base (expression (span (offset 290) (line 14) (column 21) (len 2)) (ref r7))) (separator dot) (member (ref r8))))))) (body semicolon)))) (calc-def (name "MassSum") (body brace (in-out-declaration) (return-declaration (name "totalMass") (short-name none)))) (calc-def (name "ms") (body brace (in-out-declaration) (return-declaration (name "totalMass") (short-name none)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicles") (short-name none) (typing none) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 536) (line 27) (column 21) (len 18)) (tuple (expression (span (offset 537) (line 27) (column 22) (len 7)) (ref r9)) (expression (span (offset 546) (line 27) (column 31) (len 7)) (ref r10))))))) (body semicolon)) (attribute-def (declaration-name "masses1") (short-name none) (typing none) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique false)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 580) (line 28) (column 25) (len 27)) (member-access (base (expression (span (offset 580) (line 28) (column 25) (len 25)) (parenthesized (expression (span (offset 581) (line 28) (column 26) (len 23)) (type-check (kind as) (operand (expression (span (offset 581) (line 28) (column 26) (len 8)) (ref r11))) (type (ref r12))))))) (separator dot) (member (ref r13))))))) (body semicolon)) (attribute-def (declaration-name "masses2") (short-name none) (typing none) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique false)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 633) (line 29) (column 25) (len 23)) (member-access (base (expression (span (offset 633) (line 29) (column 25) (len 21)) (parenthesized (expression (span (offset 634) (line 29) (column 26) (len 19)) (type-check (kind as) (operand (expression (span (offset 634) (line 29) (column 26) (len 8)) (ref r14))) (type (ref r15))))))) (separator dot) (member (ref r16))))))) (body semicolon)))))
)
~~~
