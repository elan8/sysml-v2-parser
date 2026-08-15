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
                    part  :>> clutch : ManualClutch {
                        port  :>> clutchPort : ManualClutchPort;
                    }
                }
                variant part automaticTransmission : AutomaticTransmission {
                    part  :>> clutch : AutomaticClutch {
                        port  :>> clutchPort : AutomaticClutchPort;
                    }
                }
            }
            assert constraint 'engine-transmission selection constraint' {
                (engineChoice == engineChoice::'4cylEngine' && transmissionChoice == transmissionChoice::manualTransmission) xor (engineChoice == engineChoice::'6cylEngine' && transmissionChoice == transmissionChoice::automaticTransmission);
            }
            part  :>> rearAxleAssembly {
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
    (reference r4 (scope relative) (span (offset 1748) (line 72) (column 32) (len 10)) (segments (segment 0 (token "ClutchPort") (name "ClutchPort") (separator none) (span (offset 1748) (line 72) (column 32) (len 10)))))
    (reference r5 (scope relative) (span (offset 1794) (line 73) (column 35) (len 10)) (segments (segment 0 (token "ClutchPort") (name "ClutchPort") (separator none) (span (offset 1794) (line 73) (column 35) (len 10)))))
  )
  (root (package (name "7b-Variant Configurations") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 71) (line 2) (column 34) (len 3))) (separator (span (offset 71) (line 2) (column 34) (len 2))) (marker (span (offset 73) (line 2) (column 36) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 92) (line 3) (column 17) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 103) (line 3) (column 28) (len 3))) (separator (span (offset 103) (line 3) (column 28) (len 2))) (marker (span (offset 105) (line 3) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 124) (line 4) (column 17) (len 21))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 142) (line 4) (column 35) (len 3))) (separator (span (offset 142) (line 4) (column 35) (len 2))) (marker (span (offset 144) (line 4) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 163) (line 5) (column 17) (len 24))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (package (name "RequirementsModel") (body brace (requirement-def (name "EnginePerformanceRequirement") (body semicolon)) (requirement-usage) (requirement-usage))) (package (name "DesignModel") (body brace (part-def (name "Vehicle") (body semicolon)) (part-def (name "Engine") (body semicolon)) (part-def (name "Transmission") (body semicolon)) (part-def (name "Clutch") (body semicolon)) (part-def (name "Driveshaft") (body semicolon)) (part-def (name "RearAxleAssembly") (body semicolon)) (part-def (name "Wheel") (body semicolon)) (port-def (name "FuelCmdPort") (specializes none) (body semicolon)) (port-def (name "ClutchPort") (specializes none) (body semicolon)) (port-def (name "ShaftPort_b") (specializes none) (body semicolon)) (port-def (name "ShaftPort_c") (specializes none) (body semicolon)) (port-def (name "ShaftPort_d") (specializes none) (body semicolon)) (port-def (name "VehicleToRoadPort") (specializes none) (body semicolon)) (port-def (name "WheelToRoadPort") (specializes none) (body semicolon)) (part-usage))) (package (name "VariantDefinitions") (body brace (part-def (name "4CylEngine") (body semicolon)) (part-def (name "6CylEngine") (body semicolon)) (part-def (name "ManualTransmission") (body semicolon)) (part-def (name "AutomaticTransmission") (body semicolon)) (part-def (name "ManualClutch") (body semicolon)) (part-def (name "AutomaticClutch") (body semicolon)) (port-def (name "ManualClutchPort") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r4)))) (body semicolon)) (port-def (name "AutomaticClutchPort") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (body semicolon)) (part-def (name "NarrowRimWheel") (body semicolon)) (part-def (name "WideRimWheel") (body semicolon)))) (package (name "VariabilityModel") (body brace (part-usage) (part-usage))))))
)
~~~
