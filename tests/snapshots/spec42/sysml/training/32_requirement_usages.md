# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 32 (Requirements): Requirement Usages"))
~~~
# SOURCE
~~~sysml
package 'Requirement Usages' {
	private import SI::*;
	private import 'Requirement Definitions'::*;
	
	requirement <'1.1'> fullVehicleMassLimit : VehicleMassLimitationRequirement {
		subject vehicle : Vehicle;
		attribute :>> massReqd = 2000[kg];
		
		assume constraint {
			doc /* Full tank is full. */
			vehicle.fuelMass == vehicle.fuelFullMass
		}
	}
	
	requirement <'1.2'> emptyVehicleMassLimit : VehicleMassLimitationRequirement {
		subject vehicle : Vehicle;
		attribute :>> massReqd = 1500[kg];
		
		assume constraint {
			doc /* Full tank is empty. */
			vehicle.fuelMass == 0[kg]
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "32_requirement_usages.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Requirement Usages' {
    private import SI::*;
    private import 'Requirement Definitions'::*;
    requirement <'1.1'> fullVehicleMassLimit : VehicleMassLimitationRequirement {
        subject vehicle : Vehicle;
        :>> massReqd = 2000[kg];
        assume constraint {
            doc
            /* Full tank is full. */
            vehicle.fuelMass == vehicle.fuelFullMass
        }
    }
    requirement <'1.2'> emptyVehicleMassLimit : VehicleMassLimitationRequirement {
        subject vehicle : Vehicle;
        :>> massReqd = 1500[kg];
        assume constraint {
            doc
            /* Full tank is empty. */
            vehicle.fuelMass == 0[kg]
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 47) (line 2) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 47) (line 2) (column 17) (len 2)))))
    (reference r1 (scope relative) (span (offset 70) (line 3) (column 17) (len 25)) (segments (segment 0 (token "'Requirement Definitions'") (name "Requirement Definitions") (separator none) (span (offset 70) (line 3) (column 17) (len 25)))))
    (reference r2 (scope relative) (span (offset 146) (line 5) (column 45) (len 32)) (segments (segment 0 (token "VehicleMassLimitationRequirement") (name "VehicleMassLimitationRequirement") (separator none) (span (offset 146) (line 5) (column 45) (len 32)))))
    (reference r3 (scope relative) (span (offset 201) (line 6) (column 21) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 201) (line 6) (column 21) (len 7)))))
    (reference r4 (scope relative) (span (offset 307) (line 11) (column 4) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 307) (line 11) (column 4) (len 7)))))
    (reference r5 (scope relative) (span (offset 315) (line 11) (column 12) (len 8)) (segments (segment 0 (token "fuelMass") (name "fuelMass") (separator none) (span (offset 315) (line 11) (column 12) (len 8)))))
    (reference r6 (scope relative) (span (offset 327) (line 11) (column 24) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 327) (line 11) (column 24) (len 7)))))
    (reference r7 (scope relative) (span (offset 335) (line 11) (column 32) (len 12)) (segments (segment 0 (token "fuelFullMass") (name "fuelFullMass") (separator none) (span (offset 335) (line 11) (column 32) (len 12)))))
    (reference r8 (scope relative) (span (offset 402) (line 15) (column 46) (len 32)) (segments (segment 0 (token "VehicleMassLimitationRequirement") (name "VehicleMassLimitationRequirement") (separator none) (span (offset 402) (line 15) (column 46) (len 32)))))
    (reference r9 (scope relative) (span (offset 457) (line 16) (column 21) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 457) (line 16) (column 21) (len 7)))))
    (reference r10 (scope relative) (span (offset 564) (line 21) (column 4) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 564) (line 21) (column 4) (len 7)))))
    (reference r11 (scope relative) (span (offset 572) (line 21) (column 12) (len 8)) (segments (segment 0 (token "fuelMass") (name "fuelMass") (separator none) (span (offset 572) (line 21) (column 12) (len 8)))))
    (reference r12 (scope relative) (span (offset 586) (line 21) (column 26) (len 2)) (segments (segment 0 (token "kg") (name "kg") (separator none) (span (offset 586) (line 21) (column 26) (len 2)))))
  )
  (root (package (name "Requirement Usages") (body brace (import (target (span (span (offset 47) (line 2) (column 17) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 49) (line 2) (column 19) (len 3))) (separator (span (offset 49) (line 2) (column 19) (len 2))) (marker (span (offset 51) (line 2) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 70) (line 3) (column 17) (len 28))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 95) (line 3) (column 42) (len 3))) (separator (span (offset 95) (line 3) (column 42) (len 2))) (marker (span (offset 97) (line 3) (column 44) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (requirement-usage (name "fullVehicleMassLimit") (short-name "1.1") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (abstract false) (variation false) none (subsets none) (references none) (body brace (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage) (require-constraint (kind assume) (constraint-keyword true) (name none) (target none) (specializations) (value none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 281) (line 10) (column 10) (len 20)) (normalized "Full tank is full. "))) (expression (span (offset 307) (line 11) (column 4) (len 40)) (binary (operator "==") (left (expression (span (offset 307) (line 11) (column 4) (len 16)) (member-access (base (expression (span (offset 307) (line 11) (column 4) (len 7)) (ref r4))) (separator dot) (member (ref r5))))) (right (expression (span (offset 327) (line 11) (column 24) (len 20)) (member-access (base (expression (span (offset 327) (line 11) (column 24) (len 7)) (ref r6))) (separator dot) (member (ref r7))))))))))) (requirement-usage (name "emptyVehicleMassLimit") (short-name "1.2") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (abstract false) (variation false) none (subsets none) (references none) (body brace (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage) (require-constraint (kind assume) (constraint-keyword true) (name none) (target none) (specializations) (value none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 537) (line 20) (column 10) (len 21)) (normalized "Full tank is empty. "))) (expression (span (offset 564) (line 21) (column 4) (len 25)) (binary (operator "==") (left (expression (span (offset 564) (line 21) (column 4) (len 16)) (member-access (base (expression (span (offset 564) (line 21) (column 4) (len 7)) (ref r10))) (separator dot) (member (ref r11))))) (right (expression (span (offset 584) (line 21) (column 24) (len 5)) (bracket (base (expression (span (offset 584) (line 21) (column 24) (len 1)) (integer 0))) (operands (sequence-list (element first (expression (span (offset 586) (line 21) (column 26) (len 2)) (ref r12)))))))))))))))))
)
~~~
