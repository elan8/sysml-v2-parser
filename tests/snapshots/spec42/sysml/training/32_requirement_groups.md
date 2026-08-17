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
  (references
    (reference r0 (scope relative) (span (offset 47) (line 2) (column 17) (len 25)) (segments (segment 0 (token "'Requirement Definitions'") (name "Requirement Definitions") (separator none) (span (offset 47) (line 2) (column 17) (len 25)))))
    (reference r1 (scope relative) (span (offset 93) (line 3) (column 17) (len 20)) (segments (segment 0 (token "'Requirement Usages'") (name "Requirement Usages") (separator none) (span (offset 93) (line 3) (column 17) (len 20)))))
    (reference r2 (scope relative) (span (offset 158) (line 6) (column 20) (len 10)) (segments (segment 0 (token "ClutchPort") (name "ClutchPort") (separator none) (span (offset 158) (line 6) (column 20) (len 10)))))
    (reference r3 (scope relative) (span (offset 203) (line 7) (column 34) (len 14)) (segments (segment 0 (token "GenerateTorque") (name "GenerateTorque") (separator none) (span (offset 203) (line 7) (column 34) (len 14)))))
  )
  (root (package (name "Requirement Groups") (body brace (import (target (span (span (offset 47) (line 2) (column 17) (len 28))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 72) (line 2) (column 42) (len 3))) (separator (span (offset 72) (line 2) (column 42) (len 2))) (marker (span (offset 74) (line 2) (column 44) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 93) (line 3) (column 17) (len 23))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 113) (line 3) (column 37) (len 3))) (separator (span (offset 113) (line 3) (column 37) (len 2))) (marker (span (offset 115) (line 3) (column 39) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Engine") (body brace (port-usage (declaration-name "clutchPort") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (declaration "generateTorque") (action none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (body semicolon)))) (requirement-usage (name "vehicleSpecification") (multiplicity none)) (requirement-usage (name "engineSpecification") (multiplicity none)))))
)
~~~
