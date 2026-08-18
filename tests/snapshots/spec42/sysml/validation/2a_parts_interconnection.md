# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (02-Parts Interconnection): 2a-Parts Interconnection"))
~~~
# SOURCE
~~~sysml
package '2a-Parts Interconnection' {
	public import Definitions::*;
	public import Usages::*;

	package Definitions {		
		// Port Definitions
		
		port def FuelCmdPort;
		
		port def DrivePwrPort;
		port def ClutchPort;
		
		port def ShaftPort_a;
		port def ShaftPort_b;
		port def ShaftPort_c;
		port def ShaftPort_d;
		
		port def DiffPort;
		port def AxlePort;
		port def AxleToWheelPort;
		port def WheelToAxlePort;
		port def WheelToRoadPort;
		
		port def VehicleToRoadPort {
			/*
			 * A port definition can have nested ports.
			 */
		 
			port wheelToRoadPort: WheelToRoadPort[2];
		}
	
		// Blocks
	
		part def VehicleA { 
			port fuelCmdPort: FuelCmdPort;
			port vehicleToRoadPort: VehicleToRoadPort;
		}
		
		part def AxleAssembly;		
		part def RearAxleAssembly :> AxleAssembly { 
			port shaftPort_d: ShaftPort_d;
		}
		
		part def Axle;
		part def RearAxle :> Axle;
		
		part def HalfAxle { 
			port axleToDiffPort: AxlePort;
			port axleToWheelPort: AxleToWheelPort;
		}
		
		part def Engine { 
			port fuelCmdPort: FuelCmdPort;
			port drivePwrPort: DrivePwrPort;
		}
	
		part def Transmission { 
			port clutchPort: ClutchPort;
			port shaftPort_a: ShaftPort_a;
		}
		
		part def Driveshaft { 
			port shaftPort_b: ShaftPort_b;
			port shaftPort_c: ShaftPort_c;
		}	
		
		part def Differential {
			/*
			 * Ports do not have to be defined on part defs.
			 * They can be added directly to their usages.
			 */
		}
		part def Wheel;
		
		// Interface Definitions
		
		interface def EngineToTransmissionInterface {
			/*
			 * The ends of an interface definition are always ports.
			 */
		
			end drivePwrPort: DrivePwrPort;
			end clutchPort: ClutchPort;
		}
		
		interface def DriveshaftInterface {
			end shaftPort_a: ShaftPort_a;
			end shaftPort_d: ShaftPort_d;
			
			ref driveshaft: Driveshaft {
				/*
				 * 'driveshaft' is a reference to the driveshaft that will
				 * act as the "interface medium" for this interface.
				 */
			}
			
			connect shaftPort_a to driveshaft.shaftPort_b {
				/*
				 * The two ends of 'DriveShaftInterface' are always connected
				 * via the referenced 'driveshaft'.
				 */
			}
			connect driveshaft.shaftPort_c to shaftPort_d;
		}
		
	}
	
	package Usages {
	
		part vehicle1_c1: VehicleA {
						
			bind fuelCmdPort = engine.fuelCmdPort;
			
			part engine: Engine;
			
			interface :EngineToTransmissionInterface
				connect engine.drivePwrPort to transmission.clutchPort {
				/*
				 * A usage of an interface definition connects two ports relative to 
				 * a containing context.
				 */
			}
				
			part transmission: Transmission;
			
			part driveshaft: Driveshaft {
				/*
				 * This 'driveshaft' is the part of 'vehicle1_c1' that will act as the
				 * interface medium in the following 'DriveshaftInterface' usage.
				 */
			}
			
			interface :DriveshaftInterface
				connect transmission.shaftPort_a to rearAxleAssembly.shaftPort_d {
					ref :>> driveshaft = vehicle1_c1.driveshaft {
						/*
						 * The reference property from 'DriveshaftInterface' is redefined
						 * in order to bind it to the appropriate part of 'vehicle1_c1'.
						 */
					}
				}
	
			part rearAxleAssembly: RearAxleAssembly {
				bind shaftPort_d = differential.shaftPort_d;
				
				part differential: Differential {
					port shaftPort_d: ShaftPort_d {
						/*
						 * If the part def has no ports, then they can be defined directly in
						 * a usage of the part def.
						 */
					}
					port leftDiffPort: DiffPort;
					port rightDiffPort: DiffPort;
				}
				
				interface differential.leftDiffPort to rearAxle.leftHalfAxle.axleToDiffPort {
					/*
					 * A connection can be to a port that is arbitrarily deeply nested, on either end. 
					 */
				}
				interface differential.rightDiffPort to rearAxle.rightHalfAxle.axleToDiffPort;
		
				part rearAxle: RearAxle {
					part leftHalfAxle: HalfAxle;
					part rightHalfAxle: HalfAxle;
				}
				
				connect rearAxle.leftHalfAxle.axleToWheelPort to leftWheel.wheelToAxlePort;
				connect rearAxle.rightHalfAxle.axleToWheelPort to rightWheel.wheelToAxlePort;
	
				part rearWheel: Wheel[2] ordered;
				
				/* The two rear wheels of 'rearAxleAssembly' must be given
				 * their own names in order to be referenced in connections.
				 * 
				 * (":>" is a shorthand here for "subsets".)
				 */
				part leftWheel :> rearWheel = rearWheel#(1) {
					port wheelToAxlePort: WheelToAxlePort;
					port wheelToRoadPort: WheelToRoadPort;
				}
				
				part rightWheel :> rearWheel = rearWheel#(2) {
					port wheelToAxlePort: WheelToAxlePort;
					port wheelToRoadPort: WheelToRoadPort;
				}
				
			}
			
			bind rearAxleAssembly.leftWheel.wheelToRoadPort = 
				 vehicleToRoadPort.leftWheelToRoadPort;
				 
			bind rearAxleAssembly.rightWheel.wheelToRoadPort = 
				 vehicleToRoadPort.rightWheelToRoadPort;
				
			port vehicleToRoadPort redefines VehicleA::vehicleToRoadPort {
				port leftWheelToRoadPort :> wheelToRoadPort = wheelToRoadPort#(1);
				port rightWheelToRoadPort :> wheelToRoadPort = wheelToRoadPort#(2);
			}
			
		}
	
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "2a_parts_interconnection.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '2a-Parts Interconnection' {
    public import Definitions::*;
    public import Usages::*;
    package Definitions {
        port def FuelCmdPort;
        port def DrivePwrPort;
        port def ClutchPort;
        port def ShaftPort_a;
        port def ShaftPort_b;
        port def ShaftPort_c;
        port def ShaftPort_d;
        port def DiffPort;
        port def AxlePort;
        port def AxleToWheelPort;
        port def WheelToAxlePort;
        port def WheelToRoadPort;
        port def VehicleToRoadPort {
            port wheelToRoadPort : WheelToRoadPort[2];
        }
        part def VehicleA {
            port fuelCmdPort : FuelCmdPort;
            port vehicleToRoadPort : VehicleToRoadPort;
        }
        part def AxleAssembly;
        part def RearAxleAssembly :> AxleAssembly {
            port shaftPort_d : ShaftPort_d;
        }
        part def Axle;
        part def RearAxle :> Axle;
        part def HalfAxle {
            port axleToDiffPort : AxlePort;
            port axleToWheelPort : AxleToWheelPort;
        }
        part def Engine {
            port fuelCmdPort : FuelCmdPort;
            port drivePwrPort : DrivePwrPort;
        }
        part def Transmission {
            port clutchPort : ClutchPort;
            port shaftPort_a : ShaftPort_a;
        }
        part def Driveshaft {
            port shaftPort_b : ShaftPort_b;
            port shaftPort_c : ShaftPort_c;
        }
        part def Differential {
        }
        part def Wheel;
        interface def EngineToTransmissionInterface {
            end drivePwrPort : DrivePwrPort;
            end clutchPort : ClutchPort;
        }
        interface def DriveshaftInterface {
            end shaftPort_a : ShaftPort_a;
            end shaftPort_d : ShaftPort_d;
            ref driveshaft : Driveshaft {}
            connect shaftPort_a to driveshaft.shaftPort_b {}
            connect driveshaft.shaftPort_c to shaftPort_d;
        }
    }
    package Usages {
        part vehicle1_c1 : VehicleA {
            bind fuelCmdPort = engine.fuelCmdPort;
            part engine : Engine;
            interface : EngineToTransmissionInterface connect engine.drivePwrPort to transmission.clutchPort {}
            part transmission : Transmission;
            part driveshaft : Driveshaft {}
            interface : DriveshaftInterface connect transmission.shaftPort_a to rearAxleAssembly.shaftPort_d {
                ref :>> driveshaft = vehicle1_c1.driveshaft {}
            }
            part rearAxleAssembly : RearAxleAssembly {
                bind shaftPort_d = differential.shaftPort_d;
                part differential : Differential {
                    port shaftPort_d : ShaftPort_d {}
                    port leftDiffPort : DiffPort;
                    port rightDiffPort : DiffPort;
                }
                interface differential.leftDiffPort to rearAxle.leftHalfAxle.axleToDiffPort {}
                interface differential.rightDiffPort to rearAxle.rightHalfAxle.axleToDiffPort;
                part rearAxle : RearAxle {
                    part leftHalfAxle : HalfAxle;
                    part rightHalfAxle : HalfAxle;
                }
                connect rearAxle.leftHalfAxle.axleToWheelPort to leftWheel.wheelToAxlePort;
                connect rearAxle.rightHalfAxle.axleToWheelPort to rightWheel.wheelToAxlePort;
                part rearWheel : Wheel[2] ordered;
                part leftWheel :> rearWheel = rearWheel#(1) {
                    port wheelToAxlePort : WheelToAxlePort;
                    port wheelToRoadPort : WheelToRoadPort;
                }
                part rightWheel :> rearWheel = rearWheel#(2) {
                    port wheelToAxlePort : WheelToAxlePort;
                    port wheelToRoadPort : WheelToRoadPort;
                }
            }
            bind rearAxleAssembly.leftWheel.wheelToRoadPort = vehicleToRoadPort.leftWheelToRoadPort;
            bind rearAxleAssembly.rightWheel.wheelToRoadPort = vehicleToRoadPort.rightWheelToRoadPort;
            port vehicleToRoadPort :>> VehicleA::vehicleToRoadPort {
                port leftWheelToRoadPort :> wheelToRoadPort = wheelToRoadPort#(1);
                port rightWheelToRoadPort :> wheelToRoadPort = wheelToRoadPort#(2);
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 52) (line 2) (column 16) (len 11)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 52) (line 2) (column 16) (len 11)))))
    (reference r1 (scope relative) (span (offset 83) (line 3) (column 16) (len 6)) (segments (segment 0 (token "Usages") (name "Usages") (separator none) (span (offset 83) (line 3) (column 16) (len 6)))))
    (reference r2 (scope relative) (span (offset 571) (line 29) (column 26) (len 15)) (segments (segment 0 (token "WheelToRoadPort") (name "WheelToRoadPort") (separator none) (span (offset 571) (line 29) (column 26) (len 15)))))
    (reference r3 (scope relative) (span (offset 655) (line 35) (column 22) (len 11)) (segments (segment 0 (token "FuelCmdPort") (name "FuelCmdPort") (separator none) (span (offset 655) (line 35) (column 22) (len 11)))))
    (reference r4 (scope relative) (span (offset 695) (line 36) (column 28) (len 17)) (segments (segment 0 (token "VehicleToRoadPort") (name "VehicleToRoadPort") (separator none) (span (offset 695) (line 36) (column 28) (len 17)))))
    (reference r5 (scope relative) (span (offset 816) (line 41) (column 22) (len 11)) (segments (segment 0 (token "ShaftPort_d") (name "ShaftPort_d") (separator none) (span (offset 816) (line 41) (column 22) (len 11)))))
    (reference r6 (scope relative) (span (offset 932) (line 48) (column 25) (len 8)) (segments (segment 0 (token "AxlePort") (name "AxlePort") (separator none) (span (offset 932) (line 48) (column 25) (len 8)))))
    (reference r7 (scope relative) (span (offset 967) (line 49) (column 26) (len 15)) (segments (segment 0 (token "AxleToWheelPort") (name "AxleToWheelPort") (separator none) (span (offset 967) (line 49) (column 26) (len 15)))))
    (reference r8 (scope relative) (span (offset 1033) (line 53) (column 22) (len 11)) (segments (segment 0 (token "FuelCmdPort") (name "FuelCmdPort") (separator none) (span (offset 1033) (line 53) (column 22) (len 11)))))
    (reference r9 (scope relative) (span (offset 1068) (line 54) (column 23) (len 12)) (segments (segment 0 (token "DrivePwrPort") (name "DrivePwrPort") (separator none) (span (offset 1068) (line 54) (column 23) (len 12)))))
    (reference r10 (scope relative) (span (offset 1135) (line 58) (column 21) (len 10)) (segments (segment 0 (token "ClutchPort") (name "ClutchPort") (separator none) (span (offset 1135) (line 58) (column 21) (len 10)))))
    (reference r11 (scope relative) (span (offset 1168) (line 59) (column 22) (len 11)) (segments (segment 0 (token "ShaftPort_a") (name "ShaftPort_a") (separator none) (span (offset 1168) (line 59) (column 22) (len 11)))))
    (reference r12 (scope relative) (span (offset 1234) (line 63) (column 22) (len 11)) (segments (segment 0 (token "ShaftPort_b") (name "ShaftPort_b") (separator none) (span (offset 1234) (line 63) (column 22) (len 11)))))
    (reference r13 (scope relative) (span (offset 1268) (line 64) (column 22) (len 11)) (segments (segment 0 (token "ShaftPort_c") (name "ShaftPort_c") (separator none) (span (offset 1268) (line 64) (column 22) (len 11)))))
    (reference r14 (scope relative) (span (offset 1630) (line 82) (column 22) (len 12)) (segments (segment 0 (token "DrivePwrPort") (name "DrivePwrPort") (separator none) (span (offset 1630) (line 82) (column 22) (len 12)))))
    (reference r15 (scope relative) (span (offset 1663) (line 83) (column 20) (len 10)) (segments (segment 0 (token "ClutchPort") (name "ClutchPort") (separator none) (span (offset 1663) (line 83) (column 20) (len 10)))))
    (reference r16 (scope relative) (span (offset 1740) (line 87) (column 21) (len 11)) (segments (segment 0 (token "ShaftPort_a") (name "ShaftPort_a") (separator none) (span (offset 1740) (line 87) (column 21) (len 11)))))
    (reference r17 (scope relative) (span (offset 1773) (line 88) (column 21) (len 11)) (segments (segment 0 (token "ShaftPort_d") (name "ShaftPort_d") (separator none) (span (offset 1773) (line 88) (column 21) (len 11)))))
    (reference r18 (scope relative) (span (offset 1809) (line 90) (column 20) (len 10)) (segments (segment 0 (token "Driveshaft") (name "Driveshaft") (separator none) (span (offset 1809) (line 90) (column 20) (len 10)))))
    (reference r19 (scope relative) (span (offset 2245) (line 110) (column 21) (len 8)) (segments (segment 0 (token "VehicleA") (name "VehicleA") (separator none) (span (offset 2245) (line 110) (column 21) (len 8)))))
    (reference r20 (scope relative) (span (offset 2325) (line 114) (column 17) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 2325) (line 114) (column 17) (len 6)))))
    (reference r21 (scope relative) (span (offset 2592) (line 124) (column 23) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 2592) (line 124) (column 23) (len 12)))))
    (reference r22 (scope relative) (span (offset 2630) (line 126) (column 21) (len 10)) (segments (segment 0 (token "Driveshaft") (name "Driveshaft") (separator none) (span (offset 2630) (line 126) (column 21) (len 10)))))
    (reference r23 (scope relative) (span (offset 3171) (line 143) (column 27) (len 16)) (segments (segment 0 (token "RearAxleAssembly") (name "RearAxleAssembly") (separator none) (span (offset 3171) (line 143) (column 27) (len 16)))))
    (reference r24 (scope relative) (span (offset 3267) (line 146) (column 24) (len 12)) (segments (segment 0 (token "Differential") (name "Differential") (separator none) (span (offset 3267) (line 146) (column 24) (len 12)))))
    (reference r25 (scope relative) (span (offset 3305) (line 147) (column 24) (len 11)) (segments (segment 0 (token "ShaftPort_d") (name "ShaftPort_d") (separator none) (span (offset 3305) (line 147) (column 24) (len 11)))))
    (reference r26 (scope relative) (span (offset 3479) (line 153) (column 25) (len 8)) (segments (segment 0 (token "DiffPort") (name "DiffPort") (separator none) (span (offset 3479) (line 153) (column 25) (len 8)))))
    (reference r27 (scope relative) (span (offset 3514) (line 154) (column 26) (len 8)) (segments (segment 0 (token "DiffPort") (name "DiffPort") (separator none) (span (offset 3514) (line 154) (column 26) (len 8)))))
    (reference r28 (scope relative) (span (offset 3834) (line 164) (column 20) (len 8)) (segments (segment 0 (token "RearAxle") (name "RearAxle") (separator none) (span (offset 3834) (line 164) (column 20) (len 8)))))
    (reference r29 (scope relative) (span (offset 3869) (line 165) (column 25) (len 8)) (segments (segment 0 (token "HalfAxle") (name "HalfAxle") (separator none) (span (offset 3869) (line 165) (column 25) (len 8)))))
    (reference r30 (scope relative) (span (offset 3904) (line 166) (column 26) (len 8)) (segments (segment 0 (token "HalfAxle") (name "HalfAxle") (separator none) (span (offset 3904) (line 166) (column 26) (len 8)))))
    (reference r31 (scope relative) (span (offset 4109) (line 172) (column 21) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 4109) (line 172) (column 21) (len 5)))))
    (reference r32 (scope relative) (span (offset 4347) (line 179) (column 23) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 4347) (line 179) (column 23) (len 9)))))
    (reference r33 (scope relative) (span (offset 4359) (line 179) (column 35) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 4359) (line 179) (column 35) (len 9)))))
    (reference r34 (scope relative) (span (offset 4402) (line 180) (column 28) (len 15)) (segments (segment 0 (token "WheelToAxlePort") (name "WheelToAxlePort") (separator none) (span (offset 4402) (line 180) (column 28) (len 15)))))
    (reference r35 (scope relative) (span (offset 4446) (line 181) (column 28) (len 15)) (segments (segment 0 (token "WheelToRoadPort") (name "WheelToRoadPort") (separator none) (span (offset 4446) (line 181) (column 28) (len 15)))))
    (reference r36 (scope relative) (span (offset 4497) (line 184) (column 24) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 4497) (line 184) (column 24) (len 9)))))
    (reference r37 (scope relative) (span (offset 4509) (line 184) (column 36) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 4509) (line 184) (column 36) (len 9)))))
    (reference r38 (scope relative) (span (offset 4552) (line 185) (column 28) (len 15)) (segments (segment 0 (token "WheelToAxlePort") (name "WheelToAxlePort") (separator none) (span (offset 4552) (line 185) (column 28) (len 15)))))
    (reference r39 (scope relative) (span (offset 4596) (line 186) (column 28) (len 15)) (segments (segment 0 (token "WheelToRoadPort") (name "WheelToRoadPort") (separator none) (span (offset 4596) (line 186) (column 28) (len 15)))))
    (reference r40 (scope relative) (span (offset 4878) (line 197) (column 37) (len 27)) (segments (segment 0 (token "VehicleA") (name "VehicleA") (separator none) (span (offset 4878) (line 197) (column 37) (len 8))) (segment 1 (token "vehicleToRoadPort") (name "vehicleToRoadPort") (separator colon-colon) (span (offset 4888) (line 197) (column 47) (len 17)))))
    (reference r41 (scope relative) (span (offset 4940) (line 198) (column 33) (len 15)) (segments (segment 0 (token "wheelToRoadPort") (name "wheelToRoadPort") (separator none) (span (offset 4940) (line 198) (column 33) (len 15)))))
    (reference r42 (scope relative) (span (offset 4958) (line 198) (column 51) (len 15)) (segments (segment 0 (token "wheelToRoadPort") (name "wheelToRoadPort") (separator none) (span (offset 4958) (line 198) (column 51) (len 15)))))
    (reference r43 (scope relative) (span (offset 5012) (line 199) (column 34) (len 15)) (segments (segment 0 (token "wheelToRoadPort") (name "wheelToRoadPort") (separator none) (span (offset 5012) (line 199) (column 34) (len 15)))))
    (reference r44 (scope relative) (span (offset 5030) (line 199) (column 52) (len 15)) (segments (segment 0 (token "wheelToRoadPort") (name "wheelToRoadPort") (separator none) (span (offset 5030) (line 199) (column 52) (len 15)))))
  )
  (root (package (name "2a-Parts Interconnection") (body brace (import (target (span (span (offset 52) (line 2) (column 16) (len 14))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 63) (line 2) (column 27) (len 3))) (separator (span (offset 63) (line 2) (column 27) (len 2))) (marker (span (offset 65) (line 2) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 83) (line 3) (column 16) (len 9))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 89) (line 3) (column 22) (len 3))) (separator (span (offset 89) (line 3) (column 22) (len 2))) (marker (span (offset 91) (line 3) (column 24) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (port-def (name "FuelCmdPort") (specializes none) (body semicolon)) (port-def (name "DrivePwrPort") (specializes none) (body semicolon)) (port-def (name "ClutchPort") (specializes none) (body semicolon)) (port-def (name "ShaftPort_a") (specializes none) (body semicolon)) (port-def (name "ShaftPort_b") (specializes none) (body semicolon)) (port-def (name "ShaftPort_c") (specializes none) (body semicolon)) (port-def (name "ShaftPort_d") (specializes none) (body semicolon)) (port-def (name "DiffPort") (specializes none) (body semicolon)) (port-def (name "AxlePort") (specializes none) (body semicolon)) (port-def (name "AxleToWheelPort") (specializes none) (body semicolon)) (port-def (name "WheelToAxlePort") (specializes none) (body semicolon)) (port-def (name "WheelToRoadPort") (specializes none) (body semicolon)) (port-def (name "VehicleToRoadPort") (specializes none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheelToRoadPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity (lower (expression (span (offset 587) (line 29) (column 42) (len 1)) (integer 2))) (upper (expression (span (offset 587) (line 29) (column 42) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "VehicleA") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelCmdPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleToRoadPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "AxleAssembly") (body semicolon)) (part-def (name "RearAxleAssembly") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "shaftPort_d") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Axle") (body semicolon)) (part-def (name "RearAxle") (body semicolon)) (part-def (name "HalfAxle") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "axleToDiffPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "axleToWheelPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Engine") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelCmdPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "drivePwrPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Transmission") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "clutchPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "shaftPort_a") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Driveshaft") (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "shaftPort_b") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "shaftPort_c") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Differential") (body brace)) (part-def (name "Wheel") (body semicolon)) (interface-def (name "EngineToTransmissionInterface") (modifiers) (specializes none) (body brace (end (short-name none) (identity (declaration (name "drivePwrPort") (span (offset 1616) (line 82) (column 8) (len 12)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (references none) (redefines none) (crosses none)) (end (short-name none) (identity (declaration (name "clutchPort") (span (offset 1651) (line 83) (column 8) (len 10)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (references none) (redefines none) (crosses none)))) (interface-def (name "DriveshaftInterface") (modifiers) (specializes none) (body brace (end (short-name none) (identity (declaration (name "shaftPort_a") (span (offset 1727) (line 87) (column 8) (len 11)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (references none) (redefines none) (crosses none)) (end (short-name none) (identity (declaration (name "shaftPort_d") (span (offset 1760) (line 88) (column 8) (len 11)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (references none) (redefines none) (crosses none)) (ref (name "driveshaft") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (redefines none) (subsets none) (body brace)) (connect (body brace)) (connect (body semicolon)))))) (package (name "Usages") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (bind) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (interface-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "driveshaft") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace)) (interface-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxleAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (bind) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "differential") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "shaftPort_d") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "leftDiffPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rightDiffPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (interface-usage) (interface-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r28)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "leftHalfAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rightHalfAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r30)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (connect) (connect) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r31)))) (multiplicity (lower (expression (span (offset 4115) (line 172) (column 27) (len 1)) (integer 2))) (upper (expression (span (offset 4115) (line 172) (column 27) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered true) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "leftWheel") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r32))) (value (expression (span (offset 4359) (line 179) (column 35) (len 13)) (index (base (expression (span (offset 4359) (line 179) (column 35) (len 9)) (ref r33))) (index (expression (span (offset 4370) (line 179) (column 46) (len 1)) (integer 1)))))))) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheelToAxlePort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r34)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheelToRoadPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r35)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rightWheel") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r36))) (value (expression (span (offset 4509) (line 184) (column 36) (len 13)) (index (base (expression (span (offset 4509) (line 184) (column 36) (len 9)) (ref r37))) (index (expression (span (offset 4520) (line 184) (column 47) (len 1)) (integer 2)))))))) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheelToAxlePort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r38)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheelToRoadPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r39)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (bind) (bind) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleToRoadPort") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r40)))) (references none) (crosses none) (intersects none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "leftWheelToRoadPort") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r41))) (value (expression (span (offset 4958) (line 198) (column 51) (len 19)) (index (base (expression (span (offset 4958) (line 198) (column 51) (len 15)) (ref r42))) (index (expression (span (offset 4975) (line 198) (column 68) (len 1)) (integer 1)))))))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rightWheelToRoadPort") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r43))) (value (expression (span (offset 5030) (line 199) (column 52) (len 19)) (index (base (expression (span (offset 5030) (line 199) (column 52) (len 15)) (ref r44))) (index (expression (span (offset 5047) (line 199) (column 69) (len 1)) (integer 2)))))))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))))))
)
~~~
