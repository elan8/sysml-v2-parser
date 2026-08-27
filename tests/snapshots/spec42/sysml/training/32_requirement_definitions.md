# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 32 (Requirements): Requirement Definitions"))
~~~
# SOURCE
~~~sysml
package 'Requirement Definitions' {
	private import ISQ::*;
	private import SI::*;

	requirement def MassLimitationRequirement {
		doc /* The actual mass shall be less than or equal to the required mass. */
		
		attribute massActual: MassValue;
		attribute massReqd: MassValue;
		
		require constraint { massActual <= massReqd }
	}
	
	part def Vehicle {
		attribute dryMass: MassValue;
		attribute fuelMass: MassValue;
		attribute fuelFullMass: MassValue;
	}
	
	requirement def <'1'> VehicleMassLimitationRequirement :> MassLimitationRequirement {
		doc /* The total mass of a vehicle shall be less than or equal to the required mass. */
		
		subject vehicle : Vehicle;
		
		attribute redefines massActual = vehicle.dryMass + vehicle.fuelMass;
		
		assume constraint { vehicle.fuelMass > 0[kg] }
	}
	
	port def ClutchPort;
	action def GenerateTorque;
	
	requirement def <'2'> DrivePowerInterface {
		doc /* The engine shall transfer its generated torque to the transmission via the clutch interface. */
		subject clutchPort: ClutchPort;
	}
		
	requirement def <'3'> TorqueGeneration {
		doc /* The engine shall generate torque as a function of RPM as shown in Table 1. */
		subject generateTorque: GenerateTorque;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "32_requirement_definitions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Requirement Definitions' {
    private import ISQ::*;
    private import SI::*;
    requirement def MassLimitationRequirement {
        doc
        /* The actual mass shall be less than or equal to the required mass. */
        attribute massActual : MassValue;
        attribute massReqd : MassValue;
        require constraint {
            massActual <= massReqd;
        }
    }
    part def Vehicle {
        attribute dryMass : MassValue;
        attribute fuelMass : MassValue;
        attribute fuelFullMass : MassValue;
    }
    requirement def <'1'> VehicleMassLimitationRequirement :> MassLimitationRequirement {
        doc
        /* The total mass of a vehicle shall be less than or equal to the required mass. */
        subject vehicle : Vehicle;
        :>> massActual = vehicle.dryMass + vehicle.fuelMass;
        assume constraint {
            vehicle.fuelMass > 0[kg];
        }
    }
    port def ClutchPort;
    action def GenerateTorque;
    requirement def <'2'> DrivePowerInterface {
        doc
        /* The engine shall transfer its generated torque to the transmission via the clutch interface. */
        subject clutchPort : ClutchPort;
    }
    requirement def <'3'> TorqueGeneration {
        doc
        /* The engine shall generate torque as a function of RPM as shown in Table 1. */
        subject generateTorque : GenerateTorque;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 52) (line 2) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 52) (line 2) (column 17) (len 3)))))
    (reference r1 (scope relative) (span (offset 76) (line 3) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 76) (line 3) (column 17) (len 2)))))
    (reference r2 (scope relative) (span (offset 304) (line 11) (column 24) (len 10)) (segments (segment 0 (token "massActual") (name "massActual") (separator none) (span (offset 304) (line 11) (column 24) (len 10)))))
    (reference r3 (scope relative) (span (offset 318) (line 11) (column 38) (len 8)) (segments (segment 0 (token "massReqd") (name "massReqd") (separator none) (span (offset 318) (line 11) (column 38) (len 8)))))
    (reference r4 (scope relative) (span (offset 375) (line 15) (column 22) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 375) (line 15) (column 22) (len 9)))))
    (reference r5 (scope relative) (span (offset 408) (line 16) (column 23) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 408) (line 16) (column 23) (len 9)))))
    (reference r6 (scope relative) (span (offset 445) (line 17) (column 27) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 445) (line 17) (column 27) (len 9)))))
    (reference r7 (scope relative) (span (offset 661) (line 23) (column 21) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 661) (line 23) (column 21) (len 7)))))
    (reference r8 (scope relative) (span (offset 769) (line 27) (column 23) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 769) (line 27) (column 23) (len 7)))))
    (reference r9 (scope relative) (span (offset 777) (line 27) (column 31) (len 8)) (segments (segment 0 (token "fuelMass") (name "fuelMass") (separator none) (span (offset 777) (line 27) (column 31) (len 8)))))
    (reference r10 (scope relative) (span (offset 790) (line 27) (column 44) (len 2)) (segments (segment 0 (token "kg") (name "kg") (separator none) (span (offset 790) (line 27) (column 44) (len 2)))))
    (reference r11 (scope relative) (span (offset 1025) (line 35) (column 23) (len 10)) (segments (segment 0 (token "ClutchPort") (name "ClutchPort") (separator none) (span (offset 1025) (line 35) (column 23) (len 10)))))
    (reference r12 (scope relative) (span (offset 1198) (line 40) (column 27) (len 14)) (segments (segment 0 (token "GenerateTorque") (name "GenerateTorque") (separator none) (span (offset 1198) (line 40) (column 27) (len 14)))))
  )
  (root (package (name "Requirement Definitions") (body brace (import (target (span (span (offset 52) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 55) (line 2) (column 20) (len 3))) (separator (span (offset 55) (line 2) (column 20) (len 2))) (marker (span (offset 57) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 76) (line 3) (column 17) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 78) (line 3) (column 19) (len 3))) (separator (span (offset 78) (line 3) (column 19) (len 2))) (marker (span (offset 80) (line 3) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (requirement-def (name "MassLimitationRequirement") (modifiers) (body brace (doc (name none) (locale none) (body (span (offset 137) (line 6) (column 9) (len 67)) (normalized "The actual mass shall be less than or equal to the required mass. "))) (attribute-usage) (attribute-usage) (require-constraint (kind require) (constraint-keyword true) (name none) (target none) (typing none) (body brace (expression (span (offset 304) (line 11) (column 24) (len 22)) (binary (operator "<=") (left (expression (span (offset 304) (line 11) (column 24) (len 10)) (ref r2))) (right (expression (span (offset 318) (line 11) (column 38) (len 8)) (ref r3))))))))) (part-def (name "Vehicle") (modifiers) (body brace (attribute-usage (declaration-name "dryMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "fuelMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "fuelFullMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (requirement-def (name "VehicleMassLimitationRequirement") (modifiers) (body brace (doc (name none) (locale none) (body (span (offset 556) (line 21) (column 9) (len 79)) (normalized "The total mass of a vehicle shall be less than or equal to the required mass. "))) (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage) (require-constraint (kind assume) (constraint-keyword true) (name none) (target none) (typing none) (body brace (expression (span (offset 769) (line 27) (column 23) (len 24)) (binary (operator ">") (left (expression (span (offset 769) (line 27) (column 23) (len 16)) (member-access (base (expression (span (offset 769) (line 27) (column 23) (len 7)) (ref r8))) (separator dot) (member (ref r9))))) (right (expression (span (offset 788) (line 27) (column 42) (len 5)) (bracket (base (expression (span (offset 788) (line 27) (column 42) (len 1)) (integer 0))) (operands (sequence-list (element first (expression (span (offset 790) (line 27) (column 44) (len 2)) (ref r10)))))))))))))) (port-def (name "ClutchPort") (modifiers) (specializes none) (body semicolon)) (action-def (name "GenerateTorque") (modifiers) (specializes none) (body semicolon)) (requirement-def (name "DrivePowerInterface") (modifiers) (body brace (doc (name none) (locale none) (body (span (offset 906) (line 34) (column 9) (len 94)) (normalized "The engine shall transfer its generated torque to the transmission via the clutch interface. "))) (subject (name "clutchPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (requirement-def (name "TorqueGeneration") (modifiers) (body brace (doc (name none) (locale none) (body (span (offset 1093) (line 39) (column 9) (len 76)) (normalized "The engine shall generate torque as a function of RPM as shown in Table 1. "))) (subject (name "generateTorque") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
