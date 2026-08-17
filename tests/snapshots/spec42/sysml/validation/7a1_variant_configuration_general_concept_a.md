# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (07-Variant Configuration): 7a1-Variant Configuration - General Concept-a"))
~~~
# SOURCE
~~~sysml
package '7a1-Variant Configuration - General Concept-a' {
	
	action doX;
	action doY;
	
	part part1;
	part part2;
	part part3 {
		port p1;
	}
	part part4;
	part part5 {
		port p2;
		variation perform action doXorY {
			variant perform doX;
			variant perform doY;
		}
	}
	part part6;
	
	abstract part def SubsystemA {
		abstract part :>> part3[0..1];
	}
	
	abstract part def SubsystemB {
		abstract part :>> part5[1];		
	}
	
	part anyVehicleConfig {
		
		variation part subsystemA : SubsystemA {
			variant part subsystem1 : SubsystemA {
				part :>> part1[1];
				part :>> part2[1];
			}
			variant part subsystem2 : SubsystemA {
				part :>> part2[1];
				part :>> part3[1];
			}
		}

		variation part subsystemB : SubsystemB {
			variant part subsystem3 : SubsystemB {
				part :>> part4[1];
				part :>> part5[1];
			}
			variant part subsystem4 : SubsystemB {
				part :>> part5[1];
				part :>> part6[1];
			}
		}
		
		connect [0..1] subsystemA.part3.p1 to [1] subsystemB.part5.p2;
		
		assert constraint {
			subsystemA != subsystemA::subsystem2 | 
			subsystemB == subsystemB::subsystem3
		}
		
	}
	
	part vehicleConfigA :> anyVehicleConfig {		
		part :>> subsystemA = subsystemA::subsystem1;
		part :>> subsystemB = subsystemB::subsystem3 {
			part :>> part5 {
				perform action :>> doXorY = doX;
			}
		}
	}
	
	part VehicleConfigB :> anyVehicleConfig {
		part :>> subsystemA = subsystemA::subsystem2;
		part :>> subsystemB = subsystemB::subsystem4 {
			part :>> part5 {
				perform action :>> doXorY = doY;
			}
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "7a1_variant_configuration_general_concept_a.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '7a1-Variant Configuration - General Concept-a' {
    action doX;
    action doY;
    part part1;
    part part2;
    part part3 {
        port p1;
    }
    part part4;
    part part5 {
        port p2;
        variation perform action doXorY {
            variant perform doX;
            variant perform doY;
        }
    }
    part part6;
    abstract part def SubsystemA {
        abstract part  :>> part3[0..1];
    }
    abstract part def SubsystemB {
        abstract part  :>> part5[1];
    }
    part anyVehicleConfig {
        variation part subsystemA : SubsystemA {
            variant part subsystem1 : SubsystemA {
                part  :>> part1[1];
                part  :>> part2[1];
            }
            variant part subsystem2 : SubsystemA {
                part  :>> part2[1];
                part  :>> part3[1];
            }
        }
        variation part subsystemB : SubsystemB {
            variant part subsystem3 : SubsystemB {
                part  :>> part4[1];
                part  :>> part5[1];
            }
            variant part subsystem4 : SubsystemB {
                part  :>> part5[1];
                part  :>> part6[1];
            }
        }
        connect [0..1] subsystemA.part3.p1 to [1] subsystemB.part5.p2;
        assert constraint {
            subsystemA != subsystemA::subsystem2 | subsystemB == subsystemB::subsystem3;
        }
    }
    part vehicleConfigA :> anyVehicleConfig {
        part  :>> subsystemA = subsystemA::subsystem1;
        part  :>> subsystemB = subsystemB::subsystem3 {
            part  :>> part5 {
                perform action  :>> doXorY = doX;
            }
        }
    }
    part VehicleConfigB :> anyVehicleConfig {
        part  :>> subsystemA = subsystemA::subsystem2;
        part  :>> subsystemB = subsystemB::subsystem4 {
            part  :>> part5 {
                perform action  :>> doXorY = doY;
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "7a1-Variant Configuration - General Concept-a") (body brace (action-usage (name "doX") (short-name none)) (action-usage (name "doY") (short-name none)) (part-usage (declaration-name "part1") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "part2") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "part3") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (port-usage))) (part-usage (declaration-name "part4") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "part5") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (port-usage) (perform))) (part-usage (declaration-name "part6") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-def (name "SubsystemA") (body brace (part-usage))) (part-def (name "SubsystemB") (body brace (part-usage))) (part-usage (declaration-name "anyVehicleConfig") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage) (part-usage) (connect) (assert-constraint))) (part-usage (declaration-name "vehicleConfigA") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage) (part-usage))) (part-usage (declaration-name "VehicleConfigB") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage) (part-usage))))))
)
~~~
