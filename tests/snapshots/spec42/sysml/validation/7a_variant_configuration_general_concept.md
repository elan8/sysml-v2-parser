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
            subsystemA != subsystemA::subsystem2 | subsystemB == subsystemB::subsystem3;
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
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 192) (line 12) (column 35) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 192) (line 12) (column 35) (len 7)))))
    (reference r1 (scope relative) (span (offset 714) (line 43) (column 25) (len 16)) (segments (segment 0 (token "anyVehicleConfig") (name "anyVehicleConfig") (separator none) (span (offset 714) (line 43) (column 25) (len 16)))))
    (reference r2 (scope relative) (span (offset 746) (line 44) (column 12) (len 10)) (segments (segment 0 (token "subsystemA") (name "subsystemA") (separator none) (span (offset 746) (line 44) (column 12) (len 10)))))
    (reference r3 (scope relative) (span (offset 759) (line 44) (column 25) (len 22)) (segments (segment 0 (token "subsystemA") (name "subsystemA") (separator none) (span (offset 759) (line 44) (column 25) (len 10))) (segment 1 (token "subsystem1") (name "subsystem1") (separator colon-colon) (span (offset 771) (line 44) (column 37) (len 10)))))
    (reference r4 (scope relative) (span (offset 794) (line 45) (column 12) (len 10)) (segments (segment 0 (token "subsystemB") (name "subsystemB") (separator none) (span (offset 794) (line 45) (column 12) (len 10)))))
    (reference r5 (scope relative) (span (offset 807) (line 45) (column 25) (len 22)) (segments (segment 0 (token "subsystemB") (name "subsystemB") (separator none) (span (offset 807) (line 45) (column 25) (len 10))) (segment 1 (token "subsystem3") (name "subsystem3") (separator colon-colon) (span (offset 819) (line 45) (column 37) (len 10)))))
    (reference r6 (scope relative) (span (offset 860) (line 48) (column 25) (len 16)) (segments (segment 0 (token "anyVehicleConfig") (name "anyVehicleConfig") (separator none) (span (offset 860) (line 48) (column 25) (len 16)))))
    (reference r7 (scope relative) (span (offset 890) (line 49) (column 12) (len 10)) (segments (segment 0 (token "subsystemA") (name "subsystemA") (separator none) (span (offset 890) (line 49) (column 12) (len 10)))))
    (reference r8 (scope relative) (span (offset 903) (line 49) (column 25) (len 22)) (segments (segment 0 (token "subsystemA") (name "subsystemA") (separator none) (span (offset 903) (line 49) (column 25) (len 10))) (segment 1 (token "subsystem2") (name "subsystem2") (separator colon-colon) (span (offset 915) (line 49) (column 37) (len 10)))))
    (reference r9 (scope relative) (span (offset 938) (line 50) (column 12) (len 10)) (segments (segment 0 (token "subsystemB") (name "subsystemB") (separator none) (span (offset 938) (line 50) (column 12) (len 10)))))
    (reference r10 (scope relative) (span (offset 951) (line 50) (column 25) (len 22)) (segments (segment 0 (token "subsystemB") (name "subsystemB") (separator none) (span (offset 951) (line 50) (column 25) (len 10))) (segment 1 (token "subsystem3") (name "subsystem3") (separator colon-colon) (span (offset 963) (line 50) (column 37) (len 10)))))
  )
  (root (package (name "7a-Variant Configuration - General Concept") (body brace (part-def (name "Vehicle") (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part3") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part4") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part5") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "part6") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "anyVehicleConfig") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "subsystemA") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (variant-usage) (variant-usage))) (part-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "subsystemB") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (variant-usage) (variant-usage))) (assert-constraint))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleConfigA") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r1))) (value none))) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 759) (line 44) (column 25) (len 22)) (ref r3))))) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 807) (line 45) (column 25) (len 22)) (ref r5))))) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "VehicleConfigB") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r6))) (value none))) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 903) (line 49) (column 25) (len 22)) (ref r8))))) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 951) (line 50) (column 25) (len 22)) (ref r10))))) (body semicolon)))))))
)
~~~
