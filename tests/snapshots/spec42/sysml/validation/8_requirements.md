# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (08-Requirements): 8-Requirements"))
~~~
# SOURCE
~~~sysml
package '8-Requirements' {
	private import ScalarValues::Real;
	private import ISQ::*;
	private import SI::*;
	public import 'Vehicle Usages'::*;
	public import 'Vehicle Requirements'::*;
	
	package 'Vehicle Definitions' {
		part def Vehicle {
			attribute mass: MassValue;
			attribute fuelLevel: Real;
			attribute fuelTankCapacity: Real;
		}
		
		part def Engine {
			port drivePwrPort: DrivePwrPort;
			perform action 'generate torque': 'Generate Torque';
		}
		
		part def Transmission {
			port clutchPort: ClutchPort;
		}
		
		port def DrivePwrPort;
		port def ClutchPort;
		
		interface def EngineToTransmissionInterface {
			end drivePwrPort: DrivePwrPort;
			end clutchPort: ClutchPort;
		}
	
		action def 'Generate Torque';
	}
	
	package 'Vehicle Usages' {
		public import 'Vehicle Definitions'::*;
		
		action 'provide power' {
			action 'generate torque' { /* ... */ }
			//...
		}
		
		part vehicle1_c1: Vehicle {
			attribute :>> mass = 2000 [kg];
			perform 'provide power';
				
			part engine_v1: Engine {
				port :>> drivePwrPort;
				perform 'provide power'.'generate torque' :>> 'generate torque';
			}
			
			part transmission: Transmission {
				port :>> clutchPort;
			}
			
			interface engineToTransmission: EngineToTransmissionInterface
				connect engine_v1.drivePwrPort to transmission.clutchPort;
		}
		
		part vehicle1_c2: Vehicle {
			attribute :>> mass = 2500 [kg];
		}
	}
	
	package 'Vehicle Requirements' {	
		public import 'Vehicle Definitions'::*;
	
		requirement def <'1'> MassLimitationRequirement {
			/*
			 * The optional requirement ID  of this requirement ('1') is given after the keyword "id" (using name syntax).
			 * Every requirement is parameterized by a "subject". The "subject" of this requirement is implicitly "Anything".
			 */
		
			// The requirement text is given by the documentation in the requirement def body.
			doc /* The actual mass shall be less than or equal to the required mass. */
			
			attribute massActual: MassValue;
			attribute massReqd: MassValue;
			
			require constraint {
				/*
				 * A constraint can be used to formalize a requirement.
				 */
				 massActual <= massReqd 
			 }
		}
		
		requirement def <'2'> ReliabilityRequirement;
		
		requirement <'1.1'> vehicleMass1: MassLimitationRequirement {
			doc /* The vehicle mass shall be less than or equal to 2000 kg when the fuel tank is full. */
			
			subject vehicle : Vehicle {
				/*
				 * The subject of this requirement is redefined to be a "Vehicle".
				 */
			}
			
			attribute :>> massActual: MassValue = vehicle.mass {
				/*
				 * This redefinition binds the vehicle mass to the actual mass.
				 */
			}
			
			attribute :>> massReqd = 2000 [kg] {
				/*
				 * This redefinition sets the required mass to 2000 kg.
				 */
			}
			
			assume constraint fuelConstraint {
				/*
				 * A constraint can also be used to specify an assumption.
				 */
			
				doc /* full fuel tank */
				vehicle.fuelLevel >= vehicle.fuelTankCapacity
			}
		}
			
		requirement <'2.1'> vehicleMass2: MassLimitationRequirement {
			doc /* The vehicle mass shall be less than or equal to 2500 kg when the fuel tank is empty. */
			
			subject vehicle : Vehicle;
			
			attribute :>> massActual: MassValue = vehicle.mass;
			attribute :>> massReqd = 2500 [kg];
		
			assume constraint fuelConstraint {
				doc /* empty fuel tank */
				vehicle.fuelLevel == 0.0
			}
		}
		
		requirement <'2.2'> vehicleReliability2: ReliabilityRequirement {
			subject vehicle : Vehicle;
		}
			
		requirement <'3.1'> drivePowerInterface {
			doc /* The engine shall transfer its generated torque to the transmission via the clutch interface. */
			subject drivePwrPort: DrivePwrPort;
		}
		
		requirement <'3.2'> torqueGeneration {
			doc /* The engine shall generate torque as a function of RPM as shown in Table 1. */
			subject generateTorque: 'Generate Torque';
		}
			
	}
	
	part 'vehicle1_c1 Specification Context' {
		private import 'vehicle1-c1 Specification'::*;
		private import 'engine-v1 Specification'::*;
		
		requirement 'vehicle1-c1 Specification' {
		doc
		/*
		 * This models a "requirement group" as a requirement that references other requirements.
		 */
		
			subject vehicle : Vehicle;
			requirement references vehicleMass1 {
				/*
				 * This is a reference to a requirement defined outside the group.
				 * By default, the subject of the requirement is bound to that of the group.
				 */				
			}
			// ...
		}
		
		requirement 'engine-v1 Specification' {
			subject engine : Engine;
			/* 
			 * Here the subjects of the referenced requirements are defined to be specific properties of the
			 * subject of the group.
			 */
			require torqueGeneration {
				in :>> generateTorque = engine.'generate torque';
			}
			require drivePowerInterface {
				in :>> drivePwrPort = engine.drivePwrPort; 
			}
		}
		
		satisfy 'vehicle1-c1 Specification' by vehicle1_c1 {
			/*
			 * This asserts that if the assumptions of 'vehicle1-c1 Specification' are true with 'vehicle_c1' as
			 * the subject, then the required constraints are also true.
			 */
		}
		satisfy 'engine-v1 Specification' by vehicle1_c1.engine_v1;
	}
	
	part 'vehicle1_c2 Specification Context' {
		private import 'vehicle1-c2 Specification'::*;
		
		requirement 'vehicle1-c2 Specification' {
			subject vehicle : Vehicle;
			require vehicleMass2;
			require vehicleReliability2;
		}
		
		satisfy 'vehicle1-c2 Specification' by vehicle1_c2;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "8_requirements.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '8-Requirements' {
    private import ScalarValues::Real;
    private import ISQ::*;
    private import SI::*;
    public import 'Vehicle Usages'::*;
    public import 'Vehicle Requirements'::*;
    package 'Vehicle Definitions' {
        part def Vehicle {
            attribute mass : MassValue;
            attribute fuelLevel : Real;
            attribute fuelTankCapacity : Real;
        }
        part def Engine {
            port drivePwrPort : DrivePwrPort;
            perform action 'generate torque' : 'Generate Torque';
        }
        part def Transmission {
            port clutchPort : ClutchPort;
        }
        port def DrivePwrPort;
        port def ClutchPort;
        interface def EngineToTransmissionInterface {
            end drivePwrPort : DrivePwrPort;
            end clutchPort : ClutchPort;
        }
        action def 'Generate Torque';
    }
    package 'Vehicle Usages' {
        public import 'Vehicle Definitions'::*;
        action 'provide power' {
            action 'generate torque' {
                /* ... */
            }
        }
        part vehicle1_c1 : Vehicle {
            attribute :>> mass = 2000[kg];
            perform 'provide power';
            part engine_v1 : Engine {
                port :>> drivePwrPort;
                perform 'provide power'.'generate torque' :>> 'generate torque';
            }
            part transmission : Transmission {
                port :>> clutchPort;
            }
            interface engineToTransmission : EngineToTransmissionInterface connect engine_v1.drivePwrPort to transmission.clutchPort;
        }
        part vehicle1_c2 : Vehicle {
            attribute :>> mass = 2500[kg];
        }
    }
    package 'Vehicle Requirements' {
        public import 'Vehicle Definitions'::*;
        requirement def <'1'> MassLimitationRequirement {
            /*
			 * The optional requirement ID  of this requirement ('1') is given after the keyword "id" (using name syntax).
			 * Every requirement is parameterized by a "subject". The "subject" of this requirement is implicitly "Anything".
			 */
            doc
            /* The actual mass shall be less than or equal to the required mass. */
            attribute massActual : MassValue;
            attribute massReqd : MassValue;
            require constraint {
                /*
				 * A constraint can be used to formalize a requirement.
				 */
                massActual <= massReqd;
            }
        }
        requirement def <'2'> ReliabilityRequirement;
        requirement <'1.1'> vehicleMass1 : MassLimitationRequirement {
            doc
            /* The vehicle mass shall be less than or equal to 2000 kg when the fuel tank is full. */
            subject vehicle : Vehicle {
                /*
				 * The subject of this requirement is redefined to be a "Vehicle".
				 */
            }
            :>> massActual : MassValue = vehicle.mass {
                /*
				 * This redefinition binds the vehicle mass to the actual mass.
				 */
            }
            :>> massReqd = 2000[kg] {
                /*
				 * This redefinition sets the required mass to 2000 kg.
				 */
            }
            assume constraint fuelConstraint {
                /*
				 * A constraint can also be used to specify an assumption.
				 */
                doc
                /* full fuel tank */
                vehicle.fuelLevel >= vehicle.fuelTankCapacity;
            }
        }
        requirement <'2.1'> vehicleMass2 : MassLimitationRequirement {
            doc
            /* The vehicle mass shall be less than or equal to 2500 kg when the fuel tank is empty. */
            subject vehicle : Vehicle;
            :>> massActual : MassValue = vehicle.mass;
            :>> massReqd = 2500[kg];
            assume constraint fuelConstraint {
                doc
                /* empty fuel tank */
                vehicle.fuelLevel == 0.0;
            }
        }
        requirement <'2.2'> vehicleReliability2 : ReliabilityRequirement {
            subject vehicle : Vehicle;
        }
        requirement <'3.1'> drivePowerInterface {
            doc
            /* The engine shall transfer its generated torque to the transmission via the clutch interface. */
            subject drivePwrPort : DrivePwrPort;
        }
        requirement <'3.2'> torqueGeneration {
            doc
            /* The engine shall generate torque as a function of RPM as shown in Table 1. */
            subject generateTorque : 'Generate Torque';
        }
    }
    part 'vehicle1_c1 Specification Context' {
        private import 'vehicle1-c1 Specification'::*;
        private import 'engine-v1 Specification'::*;
        requirement 'vehicle1-c1 Specification' {
            doc
            /*
		 * This models a "requirement group" as a requirement that references other requirements.
		 */
            subject vehicle : Vehicle;
            requirement  references vehicleMass1 {
                /*
				 * This is a reference to a requirement defined outside the group.
				 * By default, the subject of the requirement is bound to that of the group.
				 */
            }
        }
        requirement 'engine-v1 Specification' {
            subject engine : Engine;
            /* 
			 * Here the subjects of the referenced requirements are defined to be specific properties of the
			 * subject of the group.
			 */
            require torqueGeneration {
                in :>> generateTorque = engine.'generate torque';
            }
            require drivePowerInterface {
                in :>> drivePwrPort = engine.drivePwrPort;
            }
        }
        satisfy 'vehicle1-c1 Specification' by vehicle1_c1 {
            /*
			 * This asserts that if the assumptions of 'vehicle1-c1 Specification' are true with 'vehicle_c1' as
			 * the subject, then the required constraints are also true.
			 */
        }
        satisfy 'engine-v1 Specification' by vehicle1_c1.engine_v1;
    }
    part 'vehicle1_c2 Specification Context' {
        private import 'vehicle1-c2 Specification'::*;
        requirement 'vehicle1-c2 Specification' {
            subject vehicle : Vehicle;
            require vehicleMass2;
            require vehicleReliability2;
        }
        satisfy 'vehicle1-c2 Specification' by vehicle1_c2;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 43) (line 2) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 43) (line 2) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 57) (line 2) (column 31) (len 4)))))
    (reference r1 (scope relative) (span (offset 79) (line 3) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 79) (line 3) (column 17) (len 3)))))
    (reference r2 (scope relative) (span (offset 103) (line 4) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 103) (line 4) (column 17) (len 2)))))
    (reference r3 (scope relative) (span (offset 125) (line 5) (column 16) (len 16)) (segments (segment 0 (token "'Vehicle Usages'") (name "Vehicle Usages") (separator none) (span (offset 125) (line 5) (column 16) (len 16)))))
    (reference r4 (scope relative) (span (offset 161) (line 6) (column 16) (len 22)) (segments (segment 0 (token "'Vehicle Requirements'") (name "Vehicle Requirements") (separator none) (span (offset 161) (line 6) (column 16) (len 22)))))
    (reference r5 (scope relative) (span (offset 263) (line 10) (column 20) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 263) (line 10) (column 20) (len 9)))))
    (reference r6 (scope relative) (span (offset 298) (line 11) (column 25) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 298) (line 11) (column 25) (len 4)))))
    (reference r7 (scope relative) (span (offset 335) (line 12) (column 32) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 335) (line 12) (column 32) (len 4)))))
    (reference r8 (scope relative) (span (offset 390) (line 16) (column 23) (len 12)) (segments (segment 0 (token "DrivePwrPort") (name "DrivePwrPort") (separator none) (span (offset 390) (line 16) (column 23) (len 12)))))
    (reference r9 (scope relative) (span (offset 441) (line 17) (column 38) (len 17)) (segments (segment 0 (token "'Generate Torque'") (name "Generate Torque") (separator none) (span (offset 441) (line 17) (column 38) (len 17)))))
    (reference r10 (scope relative) (span (offset 513) (line 21) (column 21) (len 10)) (segments (segment 0 (token "ClutchPort") (name "ClutchPort") (separator none) (span (offset 513) (line 21) (column 21) (len 10)))))
    (reference r11 (scope relative) (span (offset 652) (line 28) (column 22) (len 12)) (segments (segment 0 (token "DrivePwrPort") (name "DrivePwrPort") (separator none) (span (offset 652) (line 28) (column 22) (len 12)))))
    (reference r12 (scope relative) (span (offset 685) (line 29) (column 20) (len 10)) (segments (segment 0 (token "ClutchPort") (name "ClutchPort") (separator none) (span (offset 685) (line 29) (column 20) (len 10)))))
    (reference r13 (scope relative) (span (offset 784) (line 36) (column 17) (len 21)) (segments (segment 0 (token "'Vehicle Definitions'") (name "Vehicle Definitions") (separator none) (span (offset 784) (line 36) (column 17) (len 21)))))
    (reference r14 (scope relative) (span (offset 918) (line 43) (column 21) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 918) (line 43) (column 21) (len 7)))))
    (reference r15 (scope relative) (span (offset 945) (line 44) (column 18) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 945) (line 44) (column 18) (len 4)))))
    (reference r16 (scope relative) (span (offset 958) (line 44) (column 31) (len 2)) (segments (segment 0 (token "kg") (name "kg") (separator none) (span (offset 958) (line 44) (column 31) (len 2)))))
    (reference r17 (scope relative) (span (offset 974) (line 45) (column 12) (len 15)) (segments (segment 0 (token "'provide power'") (name "provide power") (separator none) (span (offset 974) (line 45) (column 12) (len 15)))))
    (reference r18 (scope relative) (span (offset 1015) (line 47) (column 20) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1015) (line 47) (column 20) (len 6)))))
    (reference r19 (scope relative) (span (offset 1037) (line 48) (column 14) (len 12)) (segments (segment 0 (token "drivePwrPort") (name "drivePwrPort") (separator none) (span (offset 1037) (line 48) (column 14) (len 12)))))
    (reference r20 (scope relative) (span (offset 1063) (line 49) (column 13) (len 33)) (segments (segment 0 (token "'provide power'") (name "provide power") (separator none) (span (offset 1063) (line 49) (column 13) (len 15))) (segment 1 (token "'generate torque'") (name "generate torque") (separator dot) (span (offset 1079) (line 49) (column 29) (len 17)))))
    (reference r21 (scope relative) (span (offset 1101) (line 49) (column 51) (len 17)) (segments (segment 0 (token "'generate torque'") (name "generate torque") (separator none) (span (offset 1101) (line 49) (column 51) (len 17)))))
    (reference r22 (scope relative) (span (offset 1151) (line 52) (column 23) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 1151) (line 52) (column 23) (len 12)))))
    (reference r23 (scope relative) (span (offset 1179) (line 53) (column 14) (len 10)) (segments (segment 0 (token "clutchPort") (name "clutchPort") (separator none) (span (offset 1179) (line 53) (column 14) (len 10)))))
    (reference r24 (scope relative) (span (offset 1277) (line 57) (column 13) (len 22)) (segments (segment 0 (token "engine_v1") (name "engine_v1") (separator none) (span (offset 1277) (line 57) (column 13) (len 9))) (segment 1 (token "drivePwrPort") (name "drivePwrPort") (separator dot) (span (offset 1287) (line 57) (column 23) (len 12)))))
    (reference r25 (scope relative) (span (offset 1303) (line 57) (column 39) (len 23)) (segments (segment 0 (token "transmission") (name "transmission") (separator none) (span (offset 1303) (line 57) (column 39) (len 12))) (segment 1 (token "clutchPort") (name "clutchPort") (separator dot) (span (offset 1316) (line 57) (column 52) (len 10)))))
    (reference r26 (scope relative) (span (offset 1355) (line 60) (column 21) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 1355) (line 60) (column 21) (len 7)))))
    (reference r27 (scope relative) (span (offset 1382) (line 61) (column 18) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 1382) (line 61) (column 18) (len 4)))))
    (reference r28 (scope relative) (span (offset 1395) (line 61) (column 31) (len 2)) (segments (segment 0 (token "kg") (name "kg") (separator none) (span (offset 1395) (line 61) (column 31) (len 2)))))
    (reference r29 (scope relative) (span (offset 1460) (line 66) (column 17) (len 21)) (segments (segment 0 (token "'Vehicle Definitions'") (name "Vehicle Definitions") (separator none) (span (offset 1460) (line 66) (column 17) (len 21)))))
    (reference r30 (scope relative) (span (offset 4860) (line 185) (column 11) (len 27)) (segments (segment 0 (token "'vehicle1-c1 Specification'") (name "vehicle1-c1 Specification") (separator none) (span (offset 4860) (line 185) (column 11) (len 27)))))
    (reference r31 (scope relative) (span (offset 4891) (line 185) (column 42) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 4891) (line 185) (column 42) (len 11)))))
    (reference r32 (scope relative) (span (offset 5100) (line 191) (column 11) (len 25)) (segments (segment 0 (token "'engine-v1 Specification'") (name "engine-v1 Specification") (separator none) (span (offset 5100) (line 191) (column 11) (len 25)))))
    (reference r33 (scope relative) (span (offset 5129) (line 191) (column 40) (len 21)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 5129) (line 191) (column 40) (len 11))) (segment 1 (token "engine_v1") (name "engine_v1") (separator dot) (span (offset 5141) (line 191) (column 52) (len 9)))))
    (reference r34 (scope relative) (span (offset 5401) (line 203) (column 11) (len 27)) (segments (segment 0 (token "'vehicle1-c2 Specification'") (name "vehicle1-c2 Specification") (separator none) (span (offset 5401) (line 203) (column 11) (len 27)))))
    (reference r35 (scope relative) (span (offset 5432) (line 203) (column 42) (len 11)) (segments (segment 0 (token "vehicle1_c2") (name "vehicle1_c2") (separator none) (span (offset 5432) (line 203) (column 42) (len 11)))))
  )
  (root (package (name "8-Requirements") (body brace (import (target (span (span (offset 43) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 79) (line 3) (column 17) (len 6))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 82) (line 3) (column 20) (len 3))) (separator (span (offset 82) (line 3) (column 20) (len 2))) (marker (span (offset 84) (line 3) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 103) (line 4) (column 17) (len 5))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 105) (line 4) (column 19) (len 3))) (separator (span (offset 105) (line 4) (column 19) (len 2))) (marker (span (offset 107) (line 4) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 125) (line 5) (column 16) (len 19))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 141) (line 5) (column 32) (len 3))) (separator (span (offset 141) (line 5) (column 32) (len 2))) (marker (span (offset 143) (line 5) (column 34) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 161) (line 6) (column 16) (len 25))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 183) (line 6) (column 38) (len 3))) (separator (span (offset 183) (line 6) (column 38) (len 2))) (marker (span (offset 185) (line 6) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Vehicle Definitions") (body brace (part-def (name "Vehicle") (modifiers) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "fuelLevel") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "fuelTankCapacity") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Engine") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "drivePwrPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (target (action (name "generate torque") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body semicolon)))) (part-def (name "Transmission") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "clutchPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (port-def (name "DrivePwrPort") (modifiers) (specializes none) (body semicolon)) (port-def (name "ClutchPort") (modifiers) (specializes none) (body semicolon)) (interface-def (name "EngineToTransmissionInterface") (modifiers) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "drivePwrPort") (span (offset 638) (line 28) (column 8) (len 12)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "clutchPort") (span (offset 673) (line 29) (column 8) (len 10)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)))) (action-def (name "Generate Torque") (modifiers) (specializes none) (body semicolon)))) (package (name "Vehicle Usages") (body brace (import (target (span (span (offset 784) (line 36) (column 17) (len 24))) (all none) (ref r13) (shape (namespace (wildcard-suffix (span (span (offset 805) (line 36) (column 38) (len 3))) (separator (span (offset 805) (line 36) (column 38) (len 2))) (marker (span (offset 807) (line 36) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (action-usage (name "provide power") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (action-usage (name "generate torque") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 872) (line 39) (column 33) (len 5)) (normalized "... "))))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 952) (line 44) (column 25) (len 9)) (bracket (base (expression (span (offset 952) (line 44) (column 25) (len 4)) (integer 2000))) (operands (sequence-list (element first (expression (span (offset 958) (line 44) (column 31) (len 2)) (ref r16)))))))))) (body semicolon)) (perform (target (reference (action (ref r17)) (redefines none))) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine_v1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (perform (target (reference (action (ref r20)) (redefines (relationship (kind redefines) (implied false) (targets (ref r21)))))) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r23)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (interface-usage (form typed-connect) (part (binary (from (interface-end (multiplicity none) (target (ref r24)))) (to (interface-end (multiplicity none) (target (ref r25)))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c2") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r27)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1389) (line 61) (column 25) (len 9)) (bracket (base (expression (span (offset 1389) (line 61) (column 25) (len 4)) (integer 2500))) (operands (sequence-list (element first (expression (span (offset 1395) (line 61) (column 31) (len 2)) (ref r28)))))))))) (body semicolon)))))) (package (name "Vehicle Requirements") (body brace (import (target (span (span (offset 1460) (line 66) (column 17) (len 24))) (all none) (ref r29) (shape (namespace (wildcard-suffix (span (span (offset 1481) (line 66) (column 38) (len 3))) (separator (span (offset 1481) (line 66) (column 38) (len 2))) (marker (span (offset 1483) (line 66) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (requirement-def (name "MassLimitationRequirement") (modifiers) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1545) (line 69) (column 6) (len 236)) (normalized "The optional requirement ID  of this requirement ('1') is given after the keyword \"id\" (using name syntax).\nEvery requirement is parameterized by a \"subject\". The \"subject\" of this requirement is implicitly \"Anything\".\n"))) (doc (name none) (locale none) (body (span (offset 1882) (line 75) (column 10) (len 67)) (normalized "The actual mass shall be less than or equal to the required mass. "))) (attribute-usage) (attribute-usage) (require-constraint))) (requirement-def (name "ReliabilityRequirement") (modifiers) (body semicolon)) (requirement-usage (name "vehicleMass1") (multiplicity none)) (requirement-usage (name "vehicleMass2") (multiplicity none)) (requirement-usage (name "vehicleReliability2") (multiplicity none)) (requirement-usage (name "drivePowerInterface") (multiplicity none)) (requirement-usage (name "torqueGeneration") (multiplicity none)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c1 Specification Context") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (import) (import) (requirement-usage) (requirement-usage) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r30))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r31)) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 4910) (line 186) (column 6) (len 173)) (normalized "This asserts that if the assumptions of 'vehicle1-c1 Specification' are true with 'vehicle_c1' as\nthe subject, then the required constraints are also true.\n"))))) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r32))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r33)) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c2 Specification Context") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (import) (requirement-usage) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r34))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r35)) (body semicolon)))))))
)
~~~
