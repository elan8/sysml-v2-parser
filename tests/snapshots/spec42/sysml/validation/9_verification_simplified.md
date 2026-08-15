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
            :>> massReqd = 2500 ['SI::kg'];
        }
        part vehicle1_c2 : Vehicle {
        }
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
                in part  :>> testVehicle = vehicleUnderTest;
            }
            ref part vehicleUnderTest : Vehicle;
            part testOperator : TestOperator;
            part scale : Scale {
                perform vehicleMassTest.collectData {
                    in part  :>> testVehicle;
                    measurement = testVehicle.mass;
                    out :>> massMeasured = measurement;
                }
            }
        }
        individual testSystem : TestSystem :> massVerificationSystem {
            timeslice test1 {
                ref individual  : TestVehicle1 :> vehicle1_c2 :>> vehicleUnderTest {
                    attribute :>> mass = 2500 ['SI::kg'];
                }
            }
            then timeslice test2 {
                ref individual  : TestVehicle2 :> vehicle1_c2 :>> vehicleUnderTest {
                    attribute :>> mass = 2500 ['SI::kg'];
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
  )
  (root (package (name "9-Verification-simplified") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 71) (line 2) (column 34) (len 3))) (separator (span (offset 71) (line 2) (column 34) (len 2))) (marker (span (offset 73) (line 2) (column 36) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 92) (line 3) (column 17) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 103) (line 3) (column 28) (len 3))) (separator (span (offset 103) (line 3) (column 28) (len 2))) (marker (span (offset 105) (line 3) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (requirement-def (name "MassRequirement") (body brace (attribute-usage) (attribute-usage) (doc) (require-constraint))) (part-def (name "Vehicle") (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "MassVerificationSystem") (body semicolon)) (part-def (name "Scale") (body semicolon)) (part-def (name "TestOperator") (body semicolon)) (individual-def) (individual-def) (individual-def) (verification-case-def (name "MassTest") (body brace (objective))))) (package (name "Usages") (body brace (requirement-usage) (part-usage) (verification-case-usage) (part-usage) (occurrence (portion none) (declaration "testSystem") (target none)))))))
)
~~~
