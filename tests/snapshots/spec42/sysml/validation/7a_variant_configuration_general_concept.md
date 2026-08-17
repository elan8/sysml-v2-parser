# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (07-Variant Configuration): 7a-Variant Configuration - General Concept"))
~~~
# SOURCE
~~~sysml
package '7a-Variant Configuration - General Concept' {
	
	part def Vehicle;
	
	part part1;
	part part2;
	part part3;
	part part4;
	part part5;
	part part6;
	
	abstract part anyVehicleConfig : Vehicle {
		
		variation part subsystemA {
			variant part subsystem1 {
				part :>> part1;
				part :>> part2;
			}
			variant part subsystem2 {
				part :>> part2;
				part :>> part3;
			}
		}

		variation part subsystemB {
			variant part subsystem3 {
				part :>> part4;
				part :>> part5;
			}
			variant part subsystem4 {
				part :>> part5;
				part :>> part6;
			}
		}
		
		assert constraint {
			subsystemA != subsystemA::subsystem2 | 
			subsystemB == subsystemB::subsystem3
		}
		
	}
	
	part vehicleConfigA :> anyVehicleConfig {		
		part :>> subsystemA = subsystemA::subsystem1;
		part :>> subsystemB = subsystemB::subsystem3;
	}
	
	part VehicleConfigB :> anyVehicleConfig {
		part :>> subsystemA = subsystemA::subsystem2;
		part :>> subsystemB = subsystemB::subsystem3;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "7a_variant_configuration_general_concept.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '7a-Variant Configuration - General Concept' {
    part def Vehicle;
    part part1;
    part part2;
    part part3;
    part part4;
    part part5;
    part part6;
    abstract part anyVehicleConfig : Vehicle {
        variation part subsystemA {
            variant part subsystem1 {
                part  :>> part1;
                part  :>> part2;
            }
            variant part subsystem2 {
                part  :>> part2;
                part  :>> part3;
            }
        }
        variation part subsystemB {
            variant part subsystem3 {
                part  :>> part4;
                part  :>> part5;
            }
            variant part subsystem4 {
                part  :>> part5;
                part  :>> part6;
            }
        }
        assert constraint {
            subsystemA != subsystemA::subsystem2 | subsystemB == subsystemB::subsystem3;
        }
    }
    part vehicleConfigA :> anyVehicleConfig {
        part  :>> subsystemA = subsystemA::subsystem1;
        part  :>> subsystemB = subsystemB::subsystem3;
    }
    part VehicleConfigB :> anyVehicleConfig {
        part  :>> subsystemA = subsystemA::subsystem2;
        part  :>> subsystemB = subsystemB::subsystem3;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 192) (line 12) (column 35) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 192) (line 12) (column 35) (len 7)))))
  )
  (root (package (name "7a-Variant Configuration - General Concept") (body brace (part-def (name "Vehicle") (body semicolon)) (part-usage (declaration-name "part1") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "part2") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "part3") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "part4") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "part5") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "part6") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "anyVehicleConfig") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage) (part-usage) (assert-constraint))) (part-usage (declaration-name "vehicleConfigA") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage) (part-usage))) (part-usage (declaration-name "VehicleConfigB") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage) (part-usage))))))
)
~~~
