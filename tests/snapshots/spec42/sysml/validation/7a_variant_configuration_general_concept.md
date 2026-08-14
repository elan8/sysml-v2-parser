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
  )
  (root (package (name "7a-Variant Configuration - General Concept") (body (part-def (name "Vehicle") (body semicolon)) (part-usage) (part-usage) (part-usage) (part-usage) (part-usage) (part-usage) (part-usage) (part-usage) (part-usage))))
)
~~~
