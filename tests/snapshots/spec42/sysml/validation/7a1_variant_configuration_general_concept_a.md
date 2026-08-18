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
            subsystemA != subsystemA::subsystem2 | subsystemB == subsystemB::subsystem3;
        }
    }
    part vehicleConfigA :> anyVehicleConfig {
        part :>> subsystemA = subsystemA::subsystem1;
        part :>> subsystemB = subsystemB::subsystem3 {
            part :>> part5 {
                perform action  :>> doXorY = doX;
            }
        }
    }
    part VehicleConfigB :> anyVehicleConfig {
        part :>> subsystemA = subsystemA::subsystem2;
        part :>> subsystemB = subsystemB::subsystem4 {
            part :>> part5 {
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
    (reference r0 (scope relative) (span (offset 338) (line 22) (column 21) (len 5)) (segments (segment 0 (token "part3") (name "part3") (separator none) (span (offset 338) (line 22) (column 21) (len 5)))))
    (reference r1 (scope relative) (span (offset 408) (line 26) (column 21) (len 5)) (segments (segment 0 (token "part5") (name "part5") (separator none) (span (offset 408) (line 26) (column 21) (len 5)))))
    (reference r2 (scope relative) (span (offset 483) (line 31) (column 31) (len 10)) (segments (segment 0 (token "SubsystemA") (name "SubsystemA") (separator none) (span (offset 483) (line 31) (column 31) (len 10)))))
    (reference r3 (scope relative) (span (offset 717) (line 42) (column 31) (len 10)) (segments (segment 0 (token "SubsystemB") (name "SubsystemB") (separator none) (span (offset 717) (line 42) (column 31) (len 10)))))
    (reference r4 (scope relative) (span (offset 1132) (line 62) (column 25) (len 16)) (segments (segment 0 (token "anyVehicleConfig") (name "anyVehicleConfig") (separator none) (span (offset 1132) (line 62) (column 25) (len 16)))))
    (reference r5 (scope relative) (span (offset 1164) (line 63) (column 12) (len 10)) (segments (segment 0 (token "subsystemA") (name "subsystemA") (separator none) (span (offset 1164) (line 63) (column 12) (len 10)))))
    (reference r6 (scope relative) (span (offset 1177) (line 63) (column 25) (len 22)) (segments (segment 0 (token "subsystemA") (name "subsystemA") (separator none) (span (offset 1177) (line 63) (column 25) (len 10))) (segment 1 (token "subsystem1") (name "subsystem1") (separator colon-colon) (span (offset 1189) (line 63) (column 37) (len 10)))))
    (reference r7 (scope relative) (span (offset 1212) (line 64) (column 12) (len 10)) (segments (segment 0 (token "subsystemB") (name "subsystemB") (separator none) (span (offset 1212) (line 64) (column 12) (len 10)))))
    (reference r8 (scope relative) (span (offset 1225) (line 64) (column 25) (len 22)) (segments (segment 0 (token "subsystemB") (name "subsystemB") (separator none) (span (offset 1225) (line 64) (column 25) (len 10))) (segment 1 (token "subsystem3") (name "subsystem3") (separator colon-colon) (span (offset 1237) (line 64) (column 37) (len 10)))))
    (reference r9 (scope relative) (span (offset 1262) (line 65) (column 13) (len 5)) (segments (segment 0 (token "part5") (name "part5") (separator none) (span (offset 1262) (line 65) (column 13) (len 5)))))
    (reference r10 (scope relative) (span (offset 1345) (line 71) (column 25) (len 16)) (segments (segment 0 (token "anyVehicleConfig") (name "anyVehicleConfig") (separator none) (span (offset 1345) (line 71) (column 25) (len 16)))))
    (reference r11 (scope relative) (span (offset 1375) (line 72) (column 12) (len 10)) (segments (segment 0 (token "subsystemA") (name "subsystemA") (separator none) (span (offset 1375) (line 72) (column 12) (len 10)))))
    (reference r12 (scope relative) (span (offset 1388) (line 72) (column 25) (len 22)) (segments (segment 0 (token "subsystemA") (name "subsystemA") (separator none) (span (offset 1388) (line 72) (column 25) (len 10))) (segment 1 (token "subsystem2") (name "subsystem2") (separator colon-colon) (span (offset 1400) (line 72) (column 37) (len 10)))))
    (reference r13 (scope relative) (span (offset 1423) (line 73) (column 12) (len 10)) (segments (segment 0 (token "subsystemB") (name "subsystemB") (separator none) (span (offset 1423) (line 73) (column 12) (len 10)))))
    (reference r14 (scope relative) (span (offset 1436) (line 73) (column 25) (len 22)) (segments (segment 0 (token "subsystemB") (name "subsystemB") (separator none) (span (offset 1436) (line 73) (column 25) (len 10))) (segment 1 (token "subsystem4") (name "subsystem4") (separator colon-colon) (span (offset 1448) (line 73) (column 37) (len 10)))))
    (reference r15 (scope relative) (span (offset 1473) (line 74) (column 13) (len 5)) (segments (segment 0 (token "part5") (name "part5") (separator none) (span (offset 1473) (line 74) (column 13) (len 5)))))
  )
  (root (package (name "7a1-Variant Configuration - General Concept-a") (body brace (action-usage (name "doX") (short-name none)) (action-usage (name "doY") (short-name none)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part3") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part4") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part5") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage) (perform))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part6") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-def (name "SubsystemA") (body brace (part-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 344) (line 22) (column 27) (len 1)) (integer 0))) (upper (expression (span (offset 347) (line 22) (column 30) (len 1)) (integer 1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r0)))) (value none) (body semicolon)))) (part-def (name "SubsystemB") (body brace (part-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity (lower (expression (span (offset 414) (line 26) (column 27) (len 1)) (integer 1))) (upper (expression (span (offset 414) (line 26) (column 27) (len 1)) (integer 1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r1)))) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "anyVehicleConfig") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "subsystemA") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (variant-usage) (variant-usage))) (part-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "subsystemB") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (variant-usage) (variant-usage))) (connect) (assert-constraint))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleConfigA") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r4))) (value none))) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1177) (line 63) (column 25) (len 22)) (ref r6))))) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1225) (line 64) (column 25) (len 22)) (ref r8))))) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (value none) (body brace (perform))))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "VehicleConfigB") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r10))) (value none))) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1388) (line 72) (column 25) (len 22)) (ref r12))))) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1436) (line 73) (column 25) (len 22)) (ref r14))))) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (value none) (body brace (perform))))))))))
)
~~~
