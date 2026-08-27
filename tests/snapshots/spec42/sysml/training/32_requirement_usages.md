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
            vehicle.fuelMass == vehicle.fuelFullMass;
        }
    }
    requirement <'1.2'> emptyVehicleMassLimit : VehicleMassLimitationRequirement {
        subject vehicle : Vehicle;
        :>> massReqd = 1500[kg];
        assume constraint {
            doc
            /* Full tank is empty. */
            vehicle.fuelMass == 0[kg];
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 47) (line 2) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 47) (line 2) (column 17) (len 2)))))
    (reference r1 (scope relative) (span (offset 70) (line 3) (column 17) (len 25)) (segments (segment 0 (token "'Requirement Definitions'") (name "Requirement Definitions") (separator none) (span (offset 70) (line 3) (column 17) (len 25)))))
  )
  (root (package (name "Requirement Usages") (body brace (import (target (span (span (offset 47) (line 2) (column 17) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 49) (line 2) (column 19) (len 3))) (separator (span (offset 49) (line 2) (column 19) (len 2))) (marker (span (offset 51) (line 2) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 70) (line 3) (column 17) (len 28))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 95) (line 3) (column 42) (len 3))) (separator (span (offset 95) (line 3) (column 42) (len 2))) (marker (span (offset 97) (line 3) (column 44) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (requirement-usage (name "fullVehicleMassLimit") (multiplicity none)) (requirement-usage (name "emptyVehicleMassLimit") (multiplicity none)))))
)
~~~
