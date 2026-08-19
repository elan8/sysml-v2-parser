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
            vehicle.fuelMass > 0 [kg];
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
    (reference r2 (scope relative) (span (offset 375) (line 15) (column 22) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 375) (line 15) (column 22) (len 9)))))
    (reference r3 (scope relative) (span (offset 408) (line 16) (column 23) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 408) (line 16) (column 23) (len 9)))))
    (reference r4 (scope relative) (span (offset 445) (line 17) (column 27) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 445) (line 17) (column 27) (len 9)))))
    (reference r5 (scope relative) (span (offset 661) (line 23) (column 21) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 661) (line 23) (column 21) (len 7)))))
    (reference r6 (scope relative) (span (offset 1025) (line 35) (column 23) (len 10)) (segments (segment 0 (token "ClutchPort") (name "ClutchPort") (separator none) (span (offset 1025) (line 35) (column 23) (len 10)))))
    (reference r7 (scope relative) (span (offset 1198) (line 40) (column 27) (len 14)) (segments (segment 0 (token "GenerateTorque") (name "GenerateTorque") (separator none) (span (offset 1198) (line 40) (column 27) (len 14)))))
  )
  (root (package (name "Requirement Definitions") (body brace (import (target (span (span (offset 52) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 55) (line 2) (column 20) (len 3))) (separator (span (offset 55) (line 2) (column 20) (len 2))) (marker (span (offset 57) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 76) (line 3) (column 17) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 78) (line 3) (column 19) (len 3))) (separator (span (offset 78) (line 3) (column 19) (len 2))) (marker (span (offset 80) (line 3) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (requirement-def (name "MassLimitationRequirement") (body brace (doc (name none) (locale none) (body (span (offset 137) (line 6) (column 9) (len 67)) (normalized "The actual mass shall be less than or equal to the required mass. "))) (attribute-usage) (attribute-usage) (require-constraint))) (part-def (name "Vehicle") (body brace (attribute-usage (declaration-name "dryMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "fuelMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "fuelFullMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (requirement-def (name "VehicleMassLimitationRequirement") (body brace (doc (name none) (locale none) (body (span (offset 556) (line 21) (column 9) (len 79)) (normalized "The total mass of a vehicle shall be less than or equal to the required mass. "))) (subject (name "vehicle") (short-name none) (type (ref r5)) (redefines none) (value none)) (attribute-usage) (require-constraint))) (port-def (name "ClutchPort") (specializes none) (body semicolon)) (action-def (name "GenerateTorque") (specializes none) (body semicolon)) (requirement-def (name "DrivePowerInterface") (body brace (doc (name none) (locale none) (body (span (offset 906) (line 34) (column 9) (len 94)) (normalized "The engine shall transfer its generated torque to the transmission via the clutch interface. "))) (subject (name "clutchPort") (short-name none) (type (ref r6)) (redefines none) (value none)))) (requirement-def (name "TorqueGeneration") (body brace (doc (name none) (locale none) (body (span (offset 1093) (line 39) (column 9) (len 76)) (normalized "The engine shall generate torque as a function of RPM as shown in Table 1. "))) (subject (name "generateTorque") (short-name none) (type (ref r7)) (redefines none) (value none)))))))
)
~~~
