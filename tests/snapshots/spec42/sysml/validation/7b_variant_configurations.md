# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (07-Variant Configuration): 7b-Variant Configurations"))
~~~
# SOURCE
~~~sysml
package '7b-Variant Configurations' {
	private import RequirementsModel::*;
	private import DesignModel::*;
	private import VariantDefinitions::*;
	private import ControlFunctions::forAll;
	
	package RequirementsModel {
		requirement def EnginePerformanceRequirement;
		requirement highPerformanceRequirement : EnginePerformanceRequirement;
		requirement normalPerformanceRequirement : EnginePerformanceRequirement;
	}
	
	package DesignModel {
		part def Vehicle;
		part def Engine;
		part def Transmission;
		part def Clutch;
		part def Driveshaft;
		part def RearAxleAssembly;
		part def Wheel;
		
		port def FuelCmdPort;
		port def ClutchPort;
		port def ShaftPort_b;
		port def ShaftPort_c;
		port def ShaftPort_d;
		port def VehicleToRoadPort;
		port def WheelToRoadPort;
		
		part vehicle : Vehicle {
			port fuelCmdPort;
			
			bind fuelCmdPort = engine.fuelCmdPort;
			
			part engine : Engine[1] {
				port fuelCmdPort : FuelCmdPort;
			}
			
			part transmission : Transmission[1] {
				part clutch: Clutch[1] {
					port clutchPort : ClutchPort;
				}
			}
			
			part driveshaft : Driveshaft[1] {
				port shaftPort_b : ShaftPort_b;
				port shaftPort_c : ShaftPort_c;
			}
			
			part rearAxleAssembly : RearAxleAssembly {
				part rearWheels : Wheel[2] {
					port wheelToRoadPort : WheelToRoadPort;
				}
			}
			
			port vehicleToRoadPort : VehicleToRoadPort {
				port wheelToRoadPort : WheelToRoadPort[2];
			}
		}
	}
	
	package VariantDefinitions {
		part def '4CylEngine' :> Engine;
		part def '6CylEngine' :> Engine;
		
		part def ManualTransmission :> Transmission;
		part def AutomaticTransmission :> Transmission;
		
		part def ManualClutch :> Clutch;
		part def AutomaticClutch :> Clutch;
		
		port def ManualClutchPort :> ClutchPort;
		port def AutomaticClutchPort :> ClutchPort;
		
		part def NarrowRimWheel :> Wheel;
		part def WideRimWheel :> Wheel;		
	}
	
	package VariabilityModel {
		part anyVehicleConfig :> vehicle {
			
			variation requirement engineRqtChoice : EnginePerformanceRequirement {
				variant highPerformanceRequirement;
				variant normalPerformanceRequirement;
			}
			
			variation part engineChoice :>> engine {
				variant part '4cylEngine' : '4CylEngine';
				variant part '6cylEngine' : '6CylEngine';
			}
			
			satisfy engineRqtChoice by engineChoice;
			
			assert constraint 'engine choice constraint' {
				if engineRqtChoice == engineRqtChoice::highPerformanceRequirement? 
					engineChoice == engineChoice::'6cylEngine' 
				else
					engineChoice == engineChoice::'4cylEngine'
			}
			
			variation part transmissionChoice :>> transmission {
				variant part manualTransmission : ManualTransmission {
					part :>> clutch : ManualClutch {
						port :>> clutchPort : ManualClutchPort;
					}
				}
				variant part automaticTransmission : AutomaticTransmission {
					part :>> clutch : AutomaticClutch {
						port :>> clutchPort : AutomaticClutchPort;
					}
				}
			}
			
			assert constraint 'engine-transmission selection constraint' {
				(engineChoice == engineChoice::'4cylEngine' and transmissionChoice == transmissionChoice::manualTransmission) xor
				(engineChoice == engineChoice::'6cylEngine' and transmissionChoice == transmissionChoice::automaticTransmission)
			}
			
			part :>> rearAxleAssembly {
				variation part rearWheelChoice :>> rearWheels {
					variant part narrowRimWheel : NarrowRimWheel;
					variant part wideRimWheel : WideRimWheel;
				}
			
    			assert constraint 'engine-wheel selection constraint' {
    				(engineChoice == engineChoice::'4cylEngine' and 
    					rearWheelChoice->forAll {in ref w; w == rearWheelChoice::narrowRimWheel}) xor
    				(engineChoice == engineChoice::'6cylEngine' and 
    					rearWheelChoice->forAll {in ref w; w == rearWheelChoice::wideRimWheel})
    			}
            }
			
		}
		
		variation part vehicleChoice :> anyVehicleConfig {
			variant part vehicle_c1;
			variant part vehicle_c2;
		}	
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "7b_variant_configurations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '7b-Variant Configurations' {
    private import RequirementsModel::*;
    private import DesignModel::*;
    private import VariantDefinitions::*;
    private import ControlFunctions::forAll;
    package RequirementsModel {
        requirement def EnginePerformanceRequirement;
        requirement highPerformanceRequirement : EnginePerformanceRequirement;
        requirement normalPerformanceRequirement : EnginePerformanceRequirement;
    }
    package DesignModel {
        part def Vehicle;
        part def Engine;
        part def Transmission;
        part def Clutch;
        part def Driveshaft;
        part def RearAxleAssembly;
        part def Wheel;
        port def FuelCmdPort;
        port def ClutchPort;
        port def ShaftPort_b;
        port def ShaftPort_c;
        port def ShaftPort_d;
        port def VehicleToRoadPort;
        port def WheelToRoadPort;
        part vehicle : Vehicle {
            port fuelCmdPort;
            bind fuelCmdPort = engine.fuelCmdPort;
            part engine : Engine[1] {
                port fuelCmdPort : FuelCmdPort;
            }
            part transmission : Transmission[1] {
                part clutch : Clutch[1] {
                    port clutchPort : ClutchPort;
                }
            }
            part driveshaft : Driveshaft[1] {
                port shaftPort_b : ShaftPort_b;
                port shaftPort_c : ShaftPort_c;
            }
            part rearAxleAssembly : RearAxleAssembly {
                part rearWheels : Wheel[2] {
                    port wheelToRoadPort : WheelToRoadPort;
                }
            }
            port vehicleToRoadPort : VehicleToRoadPort {
                port wheelToRoadPort : WheelToRoadPort[2];
            }
        }
    }
    package VariantDefinitions {
        part def '4CylEngine' :> Engine;
        part def '6CylEngine' :> Engine;
        part def ManualTransmission :> Transmission;
        part def AutomaticTransmission :> Transmission;
        part def ManualClutch :> Clutch;
        part def AutomaticClutch :> Clutch;
        port def ManualClutchPort :> ClutchPort;
        port def AutomaticClutchPort :> ClutchPort;
        part def NarrowRimWheel :> Wheel;
        part def WideRimWheel :> Wheel;
    }
    package VariabilityModel {
        part anyVehicleConfig :> vehicle {
            variation requirement engineRqtChoice : EnginePerformanceRequirement {
                variant highPerformanceRequirement;
                variant normalPerformanceRequirement;
            }
            variation part engineChoice :>> engine {
                variant part '4cylEngine' : '4CylEngine';
                variant part '6cylEngine' : '6CylEngine';
            }
            satisfy engineRqtChoice by engineChoice;
            assert constraint 'engine choice constraint' {
                if engineRqtChoice == engineRqtChoice::highPerformanceRequirement ? engineChoice == engineChoice::'6cylEngine' else engineChoice == engineChoice::'4cylEngine';
            }
            variation part transmissionChoice :>> transmission {
                variant part manualTransmission : ManualTransmission {
                    part :>> clutch : ManualClutch {
                        port  :>> clutchPort : ManualClutchPort;
                    }
                }
                variant part automaticTransmission : AutomaticTransmission {
                    part :>> clutch : AutomaticClutch {
                        port  :>> clutchPort : AutomaticClutchPort;
                    }
                }
            }
            assert constraint 'engine-transmission selection constraint' {
                (engineChoice == engineChoice::'4cylEngine' && transmissionChoice == transmissionChoice::manualTransmission) xor (engineChoice == engineChoice::'6cylEngine' && transmissionChoice == transmissionChoice::automaticTransmission);
            }
            part :>> rearAxleAssembly {
                variation part rearWheelChoice :>> rearWheels {
                    variant part narrowRimWheel : NarrowRimWheel;
                    variant part wideRimWheel : WideRimWheel;
                }
                assert constraint 'engine-wheel selection constraint' {
                    (engineChoice == engineChoice::'4cylEngine' && rearWheelChoice->forAll { in ref w; w == rearWheelChoice::narrowRimWheel }) xor (engineChoice == engineChoice::'6cylEngine' && rearWheelChoice->forAll { in ref w; w == rearWheelChoice::wideRimWheel });
                }
            }
        }
        variation part vehicleChoice :> anyVehicleConfig {
            variant part vehicle_c1;
            variant part vehicle_c2;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 17)) (segments (segment 0 (token "RequirementsModel") (name "RequirementsModel") (separator none) (span (offset 54) (line 2) (column 17) (len 17)))))
    (reference r1 (scope relative) (span (offset 92) (line 3) (column 17) (len 11)) (segments (segment 0 (token "DesignModel") (name "DesignModel") (separator none) (span (offset 92) (line 3) (column 17) (len 11)))))
    (reference r2 (scope relative) (span (offset 124) (line 4) (column 17) (len 18)) (segments (segment 0 (token "VariantDefinitions") (name "VariantDefinitions") (separator none) (span (offset 124) (line 4) (column 17) (len 18)))))
    (reference r3 (scope relative) (span (offset 163) (line 5) (column 17) (len 24)) (segments (segment 0 (token "ControlFunctions") (name "ControlFunctions") (separator none) (span (offset 163) (line 5) (column 17) (len 16))) (segment 1 (token "forAll") (name "forAll") (separator colon-colon) (span (offset 181) (line 5) (column 35) (len 6)))))
    (reference r4 (scope relative) (span (offset 797) (line 30) (column 18) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 797) (line 30) (column 18) (len 7)))))
    (reference r5 (scope relative) (span (offset 895) (line 35) (column 18) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 895) (line 35) (column 18) (len 6)))))
    (reference r6 (scope relative) (span (offset 975) (line 39) (column 24) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 975) (line 39) (column 24) (len 12)))))
    (reference r7 (scope relative) (span (offset 1010) (line 40) (column 18) (len 6)) (segments (segment 0 (token "Clutch") (name "Clutch") (separator none) (span (offset 1010) (line 40) (column 18) (len 6)))))
    (reference r8 (scope relative) (span (offset 1093) (line 45) (column 22) (len 10)) (segments (segment 0 (token "Driveshaft") (name "Driveshaft") (separator none) (span (offset 1093) (line 45) (column 22) (len 10)))))
    (reference r9 (scope relative) (span (offset 1217) (line 50) (column 28) (len 16)) (segments (segment 0 (token "RearAxleAssembly") (name "RearAxleAssembly") (separator none) (span (offset 1217) (line 50) (column 28) (len 16)))))
    (reference r10 (scope relative) (span (offset 1258) (line 51) (column 23) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 1258) (line 51) (column 23) (len 5)))))
    (reference r11 (scope relative) (span (offset 1748) (line 72) (column 32) (len 10)) (segments (segment 0 (token "ClutchPort") (name "ClutchPort") (separator none) (span (offset 1748) (line 72) (column 32) (len 10)))))
    (reference r12 (scope relative) (span (offset 1794) (line 73) (column 35) (len 10)) (segments (segment 0 (token "ClutchPort") (name "ClutchPort") (separator none) (span (offset 1794) (line 73) (column 35) (len 10)))))
    (reference r13 (scope relative) (span (offset 1941) (line 80) (column 28) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1941) (line 80) (column 28) (len 7)))))
    (reference r14 (scope relative) (span (offset 2155) (line 87) (column 36) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 2155) (line 87) (column 36) (len 6)))))
    (reference r15 (scope relative) (span (offset 2196) (line 88) (column 33) (len 12)) (segments (segment 0 (token "'4CylEngine'") (name "4CylEngine") (separator none) (span (offset 2196) (line 88) (column 33) (len 12)))))
    (reference r16 (scope relative) (span (offset 2242) (line 89) (column 33) (len 12)) (segments (segment 0 (token "'6CylEngine'") (name "6CylEngine") (separator none) (span (offset 2242) (line 89) (column 33) (len 12)))))
    (reference r17 (scope relative) (span (offset 2276) (line 92) (column 12) (len 15)) (segments (segment 0 (token "engineRqtChoice") (name "engineRqtChoice") (separator none) (span (offset 2276) (line 92) (column 12) (len 15)))))
    (reference r18 (scope relative) (span (offset 2295) (line 92) (column 31) (len 12)) (segments (segment 0 (token "engineChoice") (name "engineChoice") (separator none) (span (offset 2295) (line 92) (column 31) (len 12)))))
    (reference r19 (scope relative) (span (offset 2591) (line 101) (column 42) (len 12)) (segments (segment 0 (token "transmission") (name "transmission") (separator none) (span (offset 2591) (line 101) (column 42) (len 12)))))
    (reference r20 (scope relative) (span (offset 2644) (line 102) (column 39) (len 18)) (segments (segment 0 (token "ManualTransmission") (name "ManualTransmission") (separator none) (span (offset 2644) (line 102) (column 39) (len 18)))))
    (reference r21 (scope relative) (span (offset 2688) (line 103) (column 24) (len 12)) (segments (segment 0 (token "ManualClutch") (name "ManualClutch") (separator none) (span (offset 2688) (line 103) (column 24) (len 12)))))
    (reference r22 (scope relative) (span (offset 2679) (line 103) (column 15) (len 6)) (segments (segment 0 (token "clutch") (name "clutch") (separator none) (span (offset 2679) (line 103) (column 15) (len 6)))))
    (reference r23 (scope relative) (span (offset 2803) (line 107) (column 42) (len 21)) (segments (segment 0 (token "AutomaticTransmission") (name "AutomaticTransmission") (separator none) (span (offset 2803) (line 107) (column 42) (len 21)))))
    (reference r24 (scope relative) (span (offset 2850) (line 108) (column 24) (len 15)) (segments (segment 0 (token "AutomaticClutch") (name "AutomaticClutch") (separator none) (span (offset 2850) (line 108) (column 24) (len 15)))))
    (reference r25 (scope relative) (span (offset 2841) (line 108) (column 15) (len 6)) (segments (segment 0 (token "clutch") (name "clutch") (separator none) (span (offset 2841) (line 108) (column 15) (len 6)))))
    (reference r26 (scope relative) (span (offset 3261) (line 119) (column 13) (len 16)) (segments (segment 0 (token "rearAxleAssembly") (name "rearAxleAssembly") (separator none) (span (offset 3261) (line 119) (column 13) (len 16)))))
    (reference r27 (scope relative) (span (offset 3319) (line 120) (column 40) (len 10)) (segments (segment 0 (token "rearWheels") (name "rearWheels") (separator none) (span (offset 3319) (line 120) (column 40) (len 10)))))
    (reference r28 (scope relative) (span (offset 3367) (line 121) (column 36) (len 14)) (segments (segment 0 (token "NarrowRimWheel") (name "NarrowRimWheel") (separator none) (span (offset 3367) (line 121) (column 36) (len 14)))))
    (reference r29 (scope relative) (span (offset 3416) (line 122) (column 34) (len 12)) (segments (segment 0 (token "WideRimWheel") (name "WideRimWheel") (separator none) (span (offset 3416) (line 122) (column 34) (len 12)))))
    (reference r30 (scope relative) (span (offset 3853) (line 135) (column 35) (len 16)) (segments (segment 0 (token "anyVehicleConfig") (name "anyVehicleConfig") (separator none) (span (offset 3853) (line 135) (column 35) (len 16)))))
  )
  (root (package (name "7b-Variant Configurations") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 71) (line 2) (column 34) (len 3))) (separator (span (offset 71) (line 2) (column 34) (len 2))) (marker (span (offset 73) (line 2) (column 36) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 92) (line 3) (column 17) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 103) (line 3) (column 28) (len 3))) (separator (span (offset 103) (line 3) (column 28) (len 2))) (marker (span (offset 105) (line 3) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 124) (line 4) (column 17) (len 21))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 142) (line 4) (column 35) (len 3))) (separator (span (offset 142) (line 4) (column 35) (len 2))) (marker (span (offset 144) (line 4) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 163) (line 5) (column 17) (len 24))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (package (name "RequirementsModel") (body brace (requirement-def (name "EnginePerformanceRequirement") (body semicolon)) (requirement-usage (name "highPerformanceRequirement") (multiplicity none)) (requirement-usage (name "normalPerformanceRequirement") (multiplicity none)))) (package (name "DesignModel") (body brace (part-def (name "Vehicle") (body semicolon)) (part-def (name "Engine") (body semicolon)) (part-def (name "Transmission") (body semicolon)) (part-def (name "Clutch") (body semicolon)) (part-def (name "Driveshaft") (body semicolon)) (part-def (name "RearAxleAssembly") (body semicolon)) (part-def (name "Wheel") (body semicolon)) (port-def (name "FuelCmdPort") (specializes none) (body semicolon)) (port-def (name "ClutchPort") (specializes none) (body semicolon)) (port-def (name "ShaftPort_b") (specializes none) (body semicolon)) (port-def (name "ShaftPort_c") (specializes none) (body semicolon)) (port-def (name "ShaftPort_d") (specializes none) (body semicolon)) (port-def (name "VehicleToRoadPort") (specializes none) (body semicolon)) (port-def (name "WheelToRoadPort") (specializes none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage) (bind) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity (lower (expression (span (offset 902) (line 35) (column 25) (len 1)) (integer 1))) (upper (expression (span (offset 902) (line 35) (column 25) (len 1)) (integer 1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity (lower (expression (span (offset 988) (line 39) (column 37) (len 1)) (integer 1))) (upper (expression (span (offset 988) (line 39) (column 37) (len 1)) (integer 1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "clutch") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity (lower (expression (span (offset 1017) (line 40) (column 25) (len 1)) (integer 1))) (upper (expression (span (offset 1017) (line 40) (column 25) (len 1)) (integer 1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "driveshaft") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity (lower (expression (span (offset 1104) (line 45) (column 33) (len 1)) (integer 1))) (upper (expression (span (offset 1104) (line 45) (column 33) (len 1)) (integer 1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage) (port-usage))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxleAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearWheels") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity (lower (expression (span (offset 1264) (line 51) (column 29) (len 1)) (integer 2))) (upper (expression (span (offset 1264) (line 51) (column 29) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage))))) (port-usage))))) (package (name "VariantDefinitions") (body brace (part-def (name "4CylEngine") (body semicolon)) (part-def (name "6CylEngine") (body semicolon)) (part-def (name "ManualTransmission") (body semicolon)) (part-def (name "AutomaticTransmission") (body semicolon)) (part-def (name "ManualClutch") (body semicolon)) (part-def (name "AutomaticClutch") (body semicolon)) (port-def (name "ManualClutchPort") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r11)))) (body semicolon)) (port-def (name "AutomaticClutchPort") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r12)))) (body semicolon)) (part-def (name "NarrowRimWheel") (body semicolon)) (part-def (name "WideRimWheel") (body semicolon)))) (package (name "VariabilityModel") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "anyVehicleConfig") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r13))) (value none))) (redefines none) (value none) (body brace (requirement-usage) (part-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engineChoice") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r14)))) (value none) (body brace (variant-usage (target none) (usage (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "4cylEngine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon))) (body absent)) (variant-usage (target none) (usage (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "6cylEngine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon))) (body absent)))) (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r17))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by (ref r18)) (body semicolon)) (assert-constraint) (part-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmissionChoice") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19)))) (value none) (body brace (variant-usage (target none) (usage (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "manualTransmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (value none) (body brace (port-usage)))))) (body absent)) (variant-usage (target none) (usage (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "automaticTransmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r25)))) (value none) (body brace (port-usage)))))) (body absent)))) (assert-constraint) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r26)))) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearWheelChoice") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r27)))) (value none) (body brace (variant-usage (target none) (usage (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "narrowRimWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r28)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon))) (body absent)) (variant-usage (target none) (usage (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wideRimWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon))) (body absent)))) (assert-constraint))))) (part-usage (prefix (direction none) (derived false) (variance variation) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleChoice") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r30))) (value none))) (redefines none) (value none) (body brace (variant-usage (target none) (usage (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle_c1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon))) (body absent)) (variant-usage (target none) (usage (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle_c2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon))) (body absent)))))))))
)
~~~
