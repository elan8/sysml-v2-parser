# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (09-Verification): 9-Verification-simplified"))
~~~
# SOURCE
~~~sysml
package '9-Verification-simplified' {
	private import VerificationCases::*;
	private import Definitions::*;
	
	package Definitions {
	
		requirement def <'2'> MassRequirement {
			attribute massActual :> ISQ::mass;
			attribute massReqd :> ISQ::mass;
			
			doc /* The actual mass shall be less than or equal to the required mass limit. */
			
			require constraint { massActual <= massReqd }
		}
		
		part def Vehicle {
			attribute mass :> ISQ::mass;
		}
		
		part def MassVerificationSystem;
		part def Scale;
		part def TestOperator;
		
		individual def TestVehicle1 :> Vehicle;
		individual def TestVehicle2 :> Vehicle;

		individual def TestSystem :> MassVerificationSystem;
	
		verification def MassTest {
			objective massVerificationObjective {
				verify requirement massRequirement : MassRequirement;
			}
		}
				
	}
	
	package Usages {
	
		requirement <'2.1'> vehicleMassRequirement : MassRequirement {
			subject vehicle : Vehicle;
			doc /* The vehicle mass shall be less than or equal to 2500 kg. */
			
			:>> massActual = vehicle.mass;		
			:>> massReqd = 2500 [SI::kg];
		}
		
		part vehicle1_c2 : Vehicle {
			// ...
		}
		
		verification vehicleMassTest : MassTest {
			subject testVehicle : Vehicle;
			objective vehicleMassVerificationObjective {
				// The subject of the verify is automatically bound to 'testVehicle' here.
				verify vehicleMassRequirement :>> massRequirement;
			}
			
			action collectData {
				in part testVehicle : Vehicle = vehicleMassTest.testVehicle;
				out massMeasured :> ISQ::mass;
			}
			
			action processData {
				in massMeasured :> ISQ::mass = collectData.massMeasured;
				out massProcessed :> ISQ::mass;
			}
			
			action evaluateData {
				in massProcessed :> ISQ::mass = processData.massProcessed;
				out verdict : VerdictKind = 
					// Check that 'testVehicle' statisfies 'vehicleMassRequirement' if its mass equals 'massProcessed'.
					PassIf(vehicleMassRequirement(vehicle = new testVehicle(mass = massProcessed)));
			}
			
			return verdict : VerdictKind = evaluateData.verdict;
		}
		
		part massVerificationSystem : MassVerificationSystem {
			perform vehicleMassTest {
				in part :>> testVehicle = vehicleUnderTest;
			}
			
			ref part vehicleUnderTest : Vehicle;
			
			part testOperator : TestOperator;
			
			part scale : Scale {
				perform vehicleMassTest.collectData {
					in part :>> testVehicle;
					
					// In reality, this would be some more involved process.
					measurement = testVehicle.mass;
					
					out :>> massMeasured = measurement;
				}
			}
		}
		
		individual testSystem : TestSystem :> massVerificationSystem {
			timeslice test1 {
				ref individual :>> vehicleUnderTest : TestVehicle1 :> vehicle1_c2 {
					:>> mass = 2500 [SI::kg];
				}
			}
			
			then timeslice test2 {
				ref individual :>> vehicleUnderTest : TestVehicle2 :> vehicle1_c2 {
					:>> mass = 2500 [SI::kg];
				}
			}
		}
		
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "9_verification_simplified.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '9-Verification-simplified' {
    private import VerificationCases::*;
    private import Definitions::*;
    package Definitions {
        requirement def <'2'> MassRequirement {
            attribute massActual :> ISQ::mass;
            attribute massReqd :> ISQ::mass;
            doc
            /* The actual mass shall be less than or equal to the required mass limit. */
            require constraint {
                massActual <= massReqd;
            }
        }
        part def Vehicle {
            attribute mass :> ISQ::mass;
        }
        part def MassVerificationSystem;
        part def Scale;
        part def TestOperator;
        individual def TestVehicle1 :> Vehicle;
        individual def TestVehicle2 :> Vehicle;
        individual def TestSystem :> MassVerificationSystem;
        verification def MassTest {
            objective massVerificationObjective  {
                verify requirement massRequirement : MassRequirement;
            }
        }
    }
    package Usages {
        requirement <'2.1'> vehicleMassRequirement : MassRequirement {
            subject vehicle : Vehicle;
            doc
            /* The vehicle mass shall be less than or equal to 2500 kg. */
            :>> massActual = vehicle.mass;
            :>> massReqd = 2500[SI::kg];
        }
        part vehicle1_c2 : Vehicle {}
        verification vehicleMassTest : MassTest {
            subject testVehicle : Vehicle;
            objective vehicleMassVerificationObjective  {
                verify vehicleMassRequirement :>> massRequirement;
            }
            action collectData {
                in part testVehicle : Vehicle = vehicleMassTest.testVehicle;
                out massMeasured :> ISQ::mass;
            }
            action processData {
                in massMeasured :> ISQ::mass = collectData.massMeasured;
                out massProcessed :> ISQ::mass;
            }
            action evaluateData {
                in massProcessed :> ISQ::mass = processData.massProcessed;
                out verdict : VerdictKind = PassIf(vehicleMassRequirement(vehicle = new testVehicle(mass = massProcessed)));
            }
            return verdict : VerdictKind = evaluateData.verdict;
        }
        part massVerificationSystem : MassVerificationSystem {
            perform vehicleMassTest {
                in part :>> testVehicle = vehicleUnderTest;
            }
            ref part vehicleUnderTest : Vehicle;
            part testOperator : TestOperator;
            part scale : Scale {
                perform vehicleMassTest.collectData {
                    in part :>> testVehicle;
                    measurement = testVehicle.mass;
                    out :>> massMeasured = measurement;
                }
            }
        }
        individual testSystem : TestSystem :> massVerificationSystem {
            timeslice test1 {
                ref individual : TestVehicle1 :> vehicle1_c2 :>> vehicleUnderTest {
                    attribute :>> mass = 2500[SI::kg];
                }
            }
            then timeslice test2 {
                ref individual : TestVehicle2 :> vehicle1_c2 :>> vehicleUnderTest {
                    attribute :>> mass = 2500[SI::kg];
                }
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 17)) (segments (segment 0 (token "VerificationCases") (name "VerificationCases") (separator none) (span (offset 54) (line 2) (column 17) (len 17)))))
    (reference r1 (scope relative) (span (offset 92) (line 3) (column 17) (len 11)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 92) (line 3) (column 17) (len 11)))))
    (reference r2 (scope relative) (span (offset 442) (line 17) (column 22) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 442) (line 17) (column 22) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 447) (line 17) (column 27) (len 4)))))
    (reference r3 (scope relative) (span (offset 1117) (line 47) (column 22) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 1117) (line 47) (column 22) (len 7)))))
    (reference r4 (scope relative) (span (offset 2092) (line 78) (column 33) (len 22)) (segments (segment 0 (token "MassVerificationSystem") (name "MassVerificationSystem") (separator none) (span (offset 2092) (line 78) (column 33) (len 22)))))
    (reference r5 (scope relative) (span (offset 2128) (line 79) (column 12) (len 15)) (segments (segment 0 (token "vehicleMassTest") (name "vehicleMassTest") (separator none) (span (offset 2128) (line 79) (column 12) (len 15)))))
    (reference r6 (scope relative) (span (offset 2162) (line 80) (column 17) (len 11)) (segments (segment 0 (token "testVehicle") (name "testVehicle") (separator none) (span (offset 2162) (line 80) (column 17) (len 11)))))
    (reference r7 (scope relative) (span (offset 2176) (line 80) (column 31) (len 16)) (segments (segment 0 (token "vehicleUnderTest") (name "vehicleUnderTest") (separator none) (span (offset 2176) (line 80) (column 31) (len 16)))))
    (reference r8 (scope relative) (span (offset 2234) (line 83) (column 32) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 2234) (line 83) (column 32) (len 7)))))
    (reference r9 (scope relative) (span (offset 2270) (line 85) (column 24) (len 12)) (segments (segment 0 (token "TestOperator") (name "TestOperator") (separator none) (span (offset 2270) (line 85) (column 24) (len 12)))))
    (reference r10 (scope relative) (span (offset 2304) (line 87) (column 17) (len 5)) (segments (segment 0 (token "Scale") (name "Scale") (separator none) (span (offset 2304) (line 87) (column 17) (len 5)))))
    (reference r11 (scope relative) (span (offset 2324) (line 88) (column 13) (len 27)) (segments (segment 0 (token "vehicleMassTest") (name "vehicleMassTest") (separator none) (span (offset 2324) (line 88) (column 13) (len 15))) (segment 1 (token "collectData") (name "collectData") (separator dot) (span (offset 2340) (line 88) (column 29) (len 11)))))
    (reference r12 (scope relative) (span (offset 2371) (line 89) (column 18) (len 11)) (segments (segment 0 (token "testVehicle") (name "testVehicle") (separator none) (span (offset 2371) (line 89) (column 18) (len 11)))))
    (reference r13 (scope relative) (span (offset 2721) (line 102) (column 10) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 2721) (line 102) (column 10) (len 4)))))
    (reference r14 (scope relative) (span (offset 2734) (line 102) (column 23) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 2734) (line 102) (column 23) (len 2))) (segment 1 (token "kg") (name "kg") (separator colon-colon) (span (offset 2738) (line 102) (column 27) (len 2)))))
    (reference r15 (scope relative) (span (offset 2865) (line 108) (column 10) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 2865) (line 108) (column 10) (len 4)))))
    (reference r16 (scope relative) (span (offset 2878) (line 108) (column 23) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 2878) (line 108) (column 23) (len 2))) (segment 1 (token "kg") (name "kg") (separator colon-colon) (span (offset 2882) (line 108) (column 27) (len 2)))))
  )
  (root (package (name "9-Verification-simplified") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 71) (line 2) (column 34) (len 3))) (separator (span (offset 71) (line 2) (column 34) (len 2))) (marker (span (offset 73) (line 2) (column 36) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 92) (line 3) (column 17) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 103) (line 3) (column 28) (len 3))) (separator (span (offset 103) (line 3) (column 28) (len 2))) (marker (span (offset 105) (line 3) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (requirement-def (name "MassRequirement") (modifiers) (body brace (attribute-usage) (attribute-usage) (doc (name none) (locale none) (body (span (offset 264) (line 11) (column 10) (len 73)) (normalized "The actual mass shall be less than or equal to the required mass limit. "))) (require-constraint))) (part-def (name "Vehicle") (modifiers) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "MassVerificationSystem") (modifiers) (body semicolon)) (part-def (name "Scale") (modifiers) (body semicolon)) (part-def (name "TestOperator") (modifiers) (body semicolon)) (individual-def (modifiers)) (individual-def (modifiers)) (individual-def (modifiers)) (verification-case-def (name "MassTest") (modifiers) (body brace (objective))))) (package (name "Usages") (body brace (requirement-usage (name "vehicleMassRequirement") (multiplicity none)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c2") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace)) (verification-case-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "massVerificationSystem") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform (target (reference (action (ref r5)) (redefines none))) (value none) (body brace (part-usage (then false) (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2176) (line 80) (column 31) (len 16)) (ref r7))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "vehicleUnderTest") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "testOperator") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "scale") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform (target (reference (action (ref r11)) (redefines none))) (value none) (body brace (part-usage (then false) (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (value none) (body semicolon)) (action) (action))))))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "testSystem") (short-name none) (target none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "test1") (short-name none) (target none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration "") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2728) (line 102) (column 17) (len 13)) (bracket (base (expression (span (offset 2728) (line 102) (column 17) (len 4)) (integer 2500))) (operands (sequence-list (element first (expression (span (offset 2734) (line 102) (column 23) (len 6)) (ref r14)))))))))) (body semicolon)))))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "test2") (short-name none) (target none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual true) (portion none) (extensions)) (declaration "") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2872) (line 108) (column 17) (len 13)) (bracket (base (expression (span (offset 2872) (line 108) (column 17) (len 4)) (integer 2500))) (operands (sequence-list (element first (expression (span (offset 2878) (line 108) (column 23) (len 6)) (ref r16)))))))))) (body semicolon)))))))))))))
)
~~~
