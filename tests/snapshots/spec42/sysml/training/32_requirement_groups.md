# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 32 (Requirements): Requirement Groups"))
~~~
# SOURCE
~~~sysml
package 'Requirement Groups' {
	private import 'Requirement Definitions'::*;
	private import 'Requirement Usages'::*;
	
	part def Engine {
		port clutchPort: ClutchPort;
		perform action generateTorque: GenerateTorque;
	}
	
	requirement vehicleSpecification {
		doc /* Overall vehicle requirements group */
		
		subject vehicle : Vehicle;
		
		require fullVehicleMassLimit;
		require emptyVehicleMassLimit;
	}
	
	requirement engineSpecification {
		doc /* Engine power requirements group */
		
		subject engine : Engine;
		
		requirement drivePowerInterface : DrivePowerInterface {
			subject = engine.clutchPort;
		}
		
		requirement torqueGeneration : TorqueGeneration {
			subject = engine.generateTorque;	
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "32_requirement_groups.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Requirement Groups' {
    private import 'Requirement Definitions'::*;
    private import 'Requirement Usages'::*;
    part def Engine {
        port clutchPort : ClutchPort;
        perform action generateTorque : GenerateTorque;
    }
    requirement vehicleSpecification {
        doc
        /* Overall vehicle requirements group */
        subject vehicle : Vehicle;
        require fullVehicleMassLimit;
        require emptyVehicleMassLimit;
    }
    requirement engineSpecification {
        doc
        /* Engine power requirements group */
        subject engine : Engine;
        requirement drivePowerInterface : DrivePowerInterface {
            subject = engine.clutchPort;
        }
        requirement torqueGeneration : TorqueGeneration {
            subject = engine.generateTorque;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 47) (line 2) (column 17) (len 25)) (segments (segment 0 (token "'Requirement Definitions'") (name "Requirement Definitions") (separator none) (span (offset 47) (line 2) (column 17) (len 25)))))
    (reference r1 (scope relative) (span (offset 93) (line 3) (column 17) (len 20)) (segments (segment 0 (token "'Requirement Usages'") (name "Requirement Usages") (separator none) (span (offset 93) (line 3) (column 17) (len 20)))))
    (reference r2 (scope relative) (span (offset 158) (line 6) (column 20) (len 10)) (segments (segment 0 (token "ClutchPort") (name "ClutchPort") (separator none) (span (offset 158) (line 6) (column 20) (len 10)))))
    (reference r3 (scope relative) (span (offset 203) (line 7) (column 34) (len 14)) (segments (segment 0 (token "GenerateTorque") (name "GenerateTorque") (separator none) (span (offset 203) (line 7) (column 34) (len 14)))))
    (reference r4 (scope relative) (span (offset 330) (line 13) (column 21) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 330) (line 13) (column 21) (len 7)))))
    (reference r5 (scope relative) (span (offset 352) (line 15) (column 11) (len 20)) (segments (segment 0 (token "fullVehicleMassLimit") (name "fullVehicleMassLimit") (separator none) (span (offset 352) (line 15) (column 11) (len 20)))))
    (reference r6 (scope relative) (span (offset 384) (line 16) (column 11) (len 21)) (segments (segment 0 (token "emptyVehicleMassLimit") (name "emptyVehicleMassLimit") (separator none) (span (offset 384) (line 16) (column 11) (len 21)))))
    (reference r7 (scope relative) (span (offset 513) (line 22) (column 20) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 513) (line 22) (column 20) (len 6)))))
    (reference r8 (scope relative) (span (offset 560) (line 24) (column 37) (len 19)) (segments (segment 0 (token "DrivePowerInterface") (name "DrivePowerInterface") (separator none) (span (offset 560) (line 24) (column 37) (len 19)))))
    (reference r9 (scope relative) (span (offset 595) (line 25) (column 14) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 595) (line 25) (column 14) (len 6)))))
    (reference r10 (scope relative) (span (offset 602) (line 25) (column 21) (len 10)) (segments (segment 0 (token "clutchPort") (name "clutchPort") (separator none) (span (offset 602) (line 25) (column 21) (len 10)))))
    (reference r11 (scope relative) (span (offset 654) (line 28) (column 34) (len 16)) (segments (segment 0 (token "TorqueGeneration") (name "TorqueGeneration") (separator none) (span (offset 654) (line 28) (column 34) (len 16)))))
    (reference r12 (scope relative) (span (offset 686) (line 29) (column 14) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 686) (line 29) (column 14) (len 6)))))
    (reference r13 (scope relative) (span (offset 693) (line 29) (column 21) (len 14)) (segments (segment 0 (token "generateTorque") (name "generateTorque") (separator none) (span (offset 693) (line 29) (column 21) (len 14)))))
  )
  (root (package (visibility none) (name "Requirement Groups") (body brace (import (target (span (span (offset 47) (line 2) (column 17) (len 28))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 72) (line 2) (column 42) (len 3))) (separator (span (offset 72) (line 2) (column 42) (len 2))) (marker (span (offset 74) (line 2) (column 44) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 93) (line 3) (column 17) (len 23))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 113) (line 3) (column 37) (len 3))) (separator (span (offset 113) (line 3) (column 37) (len 2))) (marker (span (offset 115) (line 3) (column 39) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Engine") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "clutchPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (target (action (name "generateTorque") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body semicolon)))) (requirement-usage (name "vehicleSpecification") (short-name none) (typing none) (abstract false) (variation false) none (subsets none) (references none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 268) (line 11) (column 9) (len 36)) (normalized "Overall vehicle requirements group "))) (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (require-constraint (kind require) (constraint-keyword false) (name none) (target (ref r5)) (specializations) (value none) (body semicolon)) (require-constraint (kind require) (constraint-keyword false) (name none) (target (ref r6)) (specializations) (value none) (body semicolon)))) (requirement-usage (name "engineSpecification") (short-name none) (typing none) (abstract false) (variation false) none (subsets none) (references none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 455) (line 20) (column 9) (len 33)) (normalized "Engine power requirements group "))) (subject (name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (requirement-usage (name "drivePowerInterface") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (abstract false) (variation false) none (subsets none) (references none) (body brace (subject (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 595) (line 25) (column 14) (len 17)) (member-access (base (expression (span (offset 595) (line 25) (column 14) (len 6)) (ref r9))) (separator dot) (member (ref r10))))))) (body semicolon)))) (requirement-usage (name "torqueGeneration") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (abstract false) (variation false) none (subsets none) (references none) (body brace (subject (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 686) (line 29) (column 14) (len 21)) (member-access (base (expression (span (offset 686) (line 29) (column 14) (len 6)) (ref r12))) (separator dot) (member (ref r13))))))) (body semicolon)))))))))
)
~~~
