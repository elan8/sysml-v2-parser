# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (04-Functional Allocation): 4a-Functional Allocation"))
~~~
# SOURCE
~~~sysml
package '4a-Functional Allocation' {
	private import '2a-Parts Interconnection'::*;
	private import '3a-Function-based Behavior-1'::*;
	private import '3a-Function-based Behavior-1'::'provide power'::*;
		
	part vehicle1_c1_functional_allocation :> vehicle1_c1 {
		// Note: The definitions of the port types in '2a-Parts Interconnection' do not include 
		// flow properties.
		port :>> fuelCmdPort {
			in fuelCmd: FuelCmd;
		}

		perform 'provide power' {
		doc
		/*
		 * This allocates the action '3a-Function-based Behavior-1'::'provide power' as an enacted 
		 * performance of 'vehicle_c1_functional_allocation'.
		 */
		
			// This assigns the fuelCmdPort to provide the input to 'provide power'.
			in fuelCmd = fuelCmdPort.fuelCmd;
		}
		
		//*
		// The above is semantically equivalent to:
		
		ref action 'provide power' (in fuelCmd = fuelCmdPort::fuelCmd) 
		   :> '3a-Function-based Behavior'::'provide power', performedActions;		
			
		// For a composite enacted performance within the vehicle, replace the above with:
		
		action 'provide power' (in fuelCmd = fuelCmdPort::fuelCmd) 
		   :> '3a-Function-based Behavior'::'provide power';
		*/
		
		part :>> engine {
			port :>> fuelCmdPort {
				in fuelCmd: FuelCmd;
			}
			
			perform 'provide power'.'generate torque' {
				/*
				 *  This allocates one of the sub-steps of 'provide power' to a sub-part of vehicle_c1. 
				 */

				in fuelCmd = fuelCmdPort.fuelCmd;
				out engineTorque = drivePwrPort.engineTorque;
			}
			
			port :>> drivePwrPort {
				out engineTorque: Torque;
			}
		}
		
		part :>> transmission {
			port :>> clutchPort {
				in attribute engineTorque: Torque;
			}
			
			perform 'provide power'.'amplify torque' {
				in engineTorque = clutchPort.engineTorque; 
				out transmissionTorque = shaftPort_a.transmissionTorque;
			}

			port :>> shaftPort_a {
				out transmissionTorque: Torque;
			}
		}
		
		part :>> driveshaft {
			port :>> shaftPort_b {
				in transmissionTorque: Torque;
			}

			perform 'provide power'.'transfer torque' {
				in transmissionTorque = shaftPort_b.transmissionTorque; 
				out driveshaftTorque = shaftPort_c.driveshaftTorque;
			}

			port :>> shaftPort_c {
				out driveshaftTorque: Torque;
			}			
		}
		
		part :>> rearAxleAssembly {
			port :>> shaftPort_d {
				in driveshaftTorque: Torque;
			}
				
			perform 'provide power'.'distribute torque' {
				in driveshaftTorque = shaftPort_d.driveshaftTorque; 
				out wheelTorque1 = rearAxle.leftHalfAxle.axleToWheelPort.wheelTorque; 
				out wheelTorque2 = rearAxle.rightHalfAxle.axleToWheelPort.wheelTorque;
			}
			
			part :>> rearAxle {
				part :>> leftHalfAxle {
					port :>> axleToWheelPort {
						out wheelTorque: Torque;
					}
				}
				part :>> rightHalfAxle {
					port :>> axleToWheelPort {
						out wheelTorque: Torque;
					}
				}
			}
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "4a_functional_allocation.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '4a-Functional Allocation' {
    private import '2a-Parts Interconnection'::*;
    private import '3a-Function-based Behavior-1'::*;
    private import '3a-Function-based Behavior-1'::'provide power'::*;
    part vehicle1_c1_functional_allocation :> vehicle1_c1 {
        port :>> fuelCmdPort {
            in fuelCmd : FuelCmd;
        }
        perform 'provide power' {
            doc
            /*
		 * This allocates the action '3a-Function-based Behavior-1'::'provide power' as an enacted 
		 * performance of 'vehicle_c1_functional_allocation'.
		 */
            in fuelCmd = fuelCmdPort.fuelCmd;
        }
        part :>> engine {
            port :>> fuelCmdPort {
                in fuelCmd : FuelCmd;
            }
            perform 'provide power'.'generate torque' {
                /*
				 *  This allocates one of the sub-steps of 'provide power' to a sub-part of vehicle_c1. 
				 */
                in fuelCmd = fuelCmdPort.fuelCmd;
                out engineTorque = drivePwrPort.engineTorque;
            }
            port :>> drivePwrPort {
                out engineTorque : Torque;
            }
        }
        part :>> transmission {
            port :>> clutchPort {
                in engineTorque : Torque;
            }
            perform 'provide power'.'amplify torque' {
                in engineTorque = clutchPort.engineTorque;
                out transmissionTorque = shaftPort_a.transmissionTorque;
            }
            port :>> shaftPort_a {
                out transmissionTorque : Torque;
            }
        }
        part :>> driveshaft {
            port :>> shaftPort_b {
                in transmissionTorque : Torque;
            }
            perform 'provide power'.'transfer torque' {
                in transmissionTorque = shaftPort_b.transmissionTorque;
                out driveshaftTorque = shaftPort_c.driveshaftTorque;
            }
            port :>> shaftPort_c {
                out driveshaftTorque : Torque;
            }
        }
        part :>> rearAxleAssembly {
            port :>> shaftPort_d {
                in driveshaftTorque : Torque;
            }
            perform 'provide power'.'distribute torque' {
                in driveshaftTorque = shaftPort_d.driveshaftTorque;
                out wheelTorque1 = rearAxle.leftHalfAxle.axleToWheelPort.wheelTorque;
                out wheelTorque2 = rearAxle.rightHalfAxle.axleToWheelPort.wheelTorque;
            }
            part :>> rearAxle {
                part :>> leftHalfAxle {
                    port :>> axleToWheelPort {
                        out wheelTorque : Torque;
                    }
                }
                part :>> rightHalfAxle {
                    port :>> axleToWheelPort {
                        out wheelTorque : Torque;
                    }
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
    (reference r0 (scope relative) (span (offset 53) (line 2) (column 17) (len 26)) (segments (segment 0 (token "'2a-Parts Interconnection'") (name "2a-Parts Interconnection") (separator none) (span (offset 53) (line 2) (column 17) (len 26)))))
    (reference r1 (scope relative) (span (offset 100) (line 3) (column 17) (len 30)) (segments (segment 0 (token "'3a-Function-based Behavior-1'") (name "3a-Function-based Behavior-1") (separator none) (span (offset 100) (line 3) (column 17) (len 30)))))
    (reference r2 (scope relative) (span (offset 151) (line 4) (column 17) (len 47)) (segments (segment 0 (token "'3a-Function-based Behavior-1'") (name "3a-Function-based Behavior-1") (separator none) (span (offset 151) (line 4) (column 17) (len 30))) (segment 1 (token "'provide power'") (name "provide power") (separator colon-colon) (span (offset 183) (line 4) (column 49) (len 15)))))
    (reference r3 (scope relative) (span (offset 249) (line 6) (column 44) (len 11)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 249) (line 6) (column 44) (len 11)))))
    (reference r4 (scope relative) (span (offset 387) (line 9) (column 12) (len 11)) (segments (segment 0 (token "fuelCmdPort") (name "fuelCmdPort") (separator none) (span (offset 387) (line 9) (column 12) (len 11)))))
    (reference r5 (scope relative) (span (offset 440) (line 13) (column 11) (len 15)) (segments (segment 0 (token "'provide power'") (name "provide power") (separator none) (span (offset 440) (line 13) (column 11) (len 15)))))
    (reference r6 (scope relative) (span (offset 710) (line 21) (column 7) (len 7)) (segments (segment 0 (token "fuelCmd") (name "fuelCmd") (separator none) (span (offset 710) (line 21) (column 7) (len 7)))))
    (reference r7 (scope relative) (span (offset 720) (line 21) (column 17) (len 19)) (segments (segment 0 (token "fuelCmdPort") (name "fuelCmdPort") (separator none) (span (offset 720) (line 21) (column 17) (len 11))) (segment 1 (token "fuelCmd") (name "fuelCmd") (separator dot) (span (offset 732) (line 21) (column 29) (len 7)))))
    (reference r8 (scope relative) (span (offset 1172) (line 36) (column 12) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 1172) (line 36) (column 12) (len 6)))))
    (reference r9 (scope relative) (span (offset 1193) (line 37) (column 13) (len 11)) (segments (segment 0 (token "fuelCmdPort") (name "fuelCmdPort") (separator none) (span (offset 1193) (line 37) (column 13) (len 11)))))
    (reference r10 (scope relative) (span (offset 1252) (line 41) (column 12) (len 33)) (segments (segment 0 (token "'provide power'") (name "provide power") (separator none) (span (offset 1252) (line 41) (column 12) (len 15))) (segment 1 (token "'generate torque'") (name "generate torque") (separator dot) (span (offset 1268) (line 41) (column 28) (len 17)))))
    (reference r11 (scope relative) (span (offset 1404) (line 46) (column 8) (len 7)) (segments (segment 0 (token "fuelCmd") (name "fuelCmd") (separator none) (span (offset 1404) (line 46) (column 8) (len 7)))))
    (reference r12 (scope relative) (span (offset 1414) (line 46) (column 18) (len 19)) (segments (segment 0 (token "fuelCmdPort") (name "fuelCmdPort") (separator none) (span (offset 1414) (line 46) (column 18) (len 11))) (segment 1 (token "fuelCmd") (name "fuelCmd") (separator dot) (span (offset 1426) (line 46) (column 30) (len 7)))))
    (reference r13 (scope relative) (span (offset 1443) (line 47) (column 9) (len 12)) (segments (segment 0 (token "engineTorque") (name "engineTorque") (separator none) (span (offset 1443) (line 47) (column 9) (len 12)))))
    (reference r14 (scope relative) (span (offset 1458) (line 47) (column 24) (len 25)) (segments (segment 0 (token "drivePwrPort") (name "drivePwrPort") (separator none) (span (offset 1458) (line 47) (column 24) (len 12))) (segment 1 (token "engineTorque") (name "engineTorque") (separator dot) (span (offset 1471) (line 47) (column 37) (len 12)))))
    (reference r15 (scope relative) (span (offset 1506) (line 50) (column 13) (len 12)) (segments (segment 0 (token "drivePwrPort") (name "drivePwrPort") (separator none) (span (offset 1506) (line 50) (column 13) (len 12)))))
    (reference r16 (scope relative) (span (offset 1574) (line 55) (column 12) (len 12)) (segments (segment 0 (token "transmission") (name "transmission") (separator none) (span (offset 1574) (line 55) (column 12) (len 12)))))
    (reference r17 (scope relative) (span (offset 1601) (line 56) (column 13) (len 10)) (segments (segment 0 (token "clutchPort") (name "clutchPort") (separator none) (span (offset 1601) (line 56) (column 13) (len 10)))))
    (reference r18 (scope relative) (span (offset 1673) (line 60) (column 12) (len 32)) (segments (segment 0 (token "'provide power'") (name "provide power") (separator none) (span (offset 1673) (line 60) (column 12) (len 15))) (segment 1 (token "'amplify torque'") (name "amplify torque") (separator dot) (span (offset 1689) (line 60) (column 28) (len 16)))))
    (reference r19 (scope relative) (span (offset 1715) (line 61) (column 8) (len 12)) (segments (segment 0 (token "engineTorque") (name "engineTorque") (separator none) (span (offset 1715) (line 61) (column 8) (len 12)))))
    (reference r20 (scope relative) (span (offset 1730) (line 61) (column 23) (len 23)) (segments (segment 0 (token "clutchPort") (name "clutchPort") (separator none) (span (offset 1730) (line 61) (column 23) (len 10))) (segment 1 (token "engineTorque") (name "engineTorque") (separator dot) (span (offset 1741) (line 61) (column 34) (len 12)))))
    (reference r21 (scope relative) (span (offset 1764) (line 62) (column 9) (len 18)) (segments (segment 0 (token "transmissionTorque") (name "transmissionTorque") (separator none) (span (offset 1764) (line 62) (column 9) (len 18)))))
    (reference r22 (scope relative) (span (offset 1785) (line 62) (column 30) (len 30)) (segments (segment 0 (token "shaftPort_a") (name "shaftPort_a") (separator none) (span (offset 1785) (line 62) (column 30) (len 11))) (segment 1 (token "transmissionTorque") (name "transmissionTorque") (separator dot) (span (offset 1797) (line 62) (column 42) (len 18)))))
    (reference r23 (scope relative) (span (offset 1835) (line 65) (column 13) (len 11)) (segments (segment 0 (token "shaftPort_a") (name "shaftPort_a") (separator none) (span (offset 1835) (line 65) (column 13) (len 11)))))
    (reference r24 (scope relative) (span (offset 1908) (line 70) (column 12) (len 10)) (segments (segment 0 (token "driveshaft") (name "driveshaft") (separator none) (span (offset 1908) (line 70) (column 12) (len 10)))))
    (reference r25 (scope relative) (span (offset 1933) (line 71) (column 13) (len 11)) (segments (segment 0 (token "shaftPort_b") (name "shaftPort_b") (separator none) (span (offset 1933) (line 71) (column 13) (len 11)))))
    (reference r26 (scope relative) (span (offset 1999) (line 75) (column 12) (len 33)) (segments (segment 0 (token "'provide power'") (name "provide power") (separator none) (span (offset 1999) (line 75) (column 12) (len 15))) (segment 1 (token "'transfer torque'") (name "transfer torque") (separator dot) (span (offset 2015) (line 75) (column 28) (len 17)))))
    (reference r27 (scope relative) (span (offset 2042) (line 76) (column 8) (len 18)) (segments (segment 0 (token "transmissionTorque") (name "transmissionTorque") (separator none) (span (offset 2042) (line 76) (column 8) (len 18)))))
    (reference r28 (scope relative) (span (offset 2063) (line 76) (column 29) (len 30)) (segments (segment 0 (token "shaftPort_b") (name "shaftPort_b") (separator none) (span (offset 2063) (line 76) (column 29) (len 11))) (segment 1 (token "transmissionTorque") (name "transmissionTorque") (separator dot) (span (offset 2075) (line 76) (column 41) (len 18)))))
    (reference r29 (scope relative) (span (offset 2104) (line 77) (column 9) (len 16)) (segments (segment 0 (token "driveshaftTorque") (name "driveshaftTorque") (separator none) (span (offset 2104) (line 77) (column 9) (len 16)))))
    (reference r30 (scope relative) (span (offset 2123) (line 77) (column 28) (len 28)) (segments (segment 0 (token "shaftPort_c") (name "shaftPort_c") (separator none) (span (offset 2123) (line 77) (column 28) (len 11))) (segment 1 (token "driveshaftTorque") (name "driveshaftTorque") (separator dot) (span (offset 2135) (line 77) (column 40) (len 16)))))
    (reference r31 (scope relative) (span (offset 2171) (line 80) (column 13) (len 11)) (segments (segment 0 (token "shaftPort_c") (name "shaftPort_c") (separator none) (span (offset 2171) (line 80) (column 13) (len 11)))))
    (reference r32 (scope relative) (span (offset 2245) (line 85) (column 12) (len 16)) (segments (segment 0 (token "rearAxleAssembly") (name "rearAxleAssembly") (separator none) (span (offset 2245) (line 85) (column 12) (len 16)))))
    (reference r33 (scope relative) (span (offset 2276) (line 86) (column 13) (len 11)) (segments (segment 0 (token "shaftPort_d") (name "shaftPort_d") (separator none) (span (offset 2276) (line 86) (column 13) (len 11)))))
    (reference r34 (scope relative) (span (offset 2344) (line 90) (column 12) (len 35)) (segments (segment 0 (token "'provide power'") (name "provide power") (separator none) (span (offset 2344) (line 90) (column 12) (len 15))) (segment 1 (token "'distribute torque'") (name "distribute torque") (separator dot) (span (offset 2360) (line 90) (column 28) (len 19)))))
    (reference r35 (scope relative) (span (offset 2389) (line 91) (column 8) (len 16)) (segments (segment 0 (token "driveshaftTorque") (name "driveshaftTorque") (separator none) (span (offset 2389) (line 91) (column 8) (len 16)))))
    (reference r36 (scope relative) (span (offset 2408) (line 91) (column 27) (len 28)) (segments (segment 0 (token "shaftPort_d") (name "shaftPort_d") (separator none) (span (offset 2408) (line 91) (column 27) (len 11))) (segment 1 (token "driveshaftTorque") (name "driveshaftTorque") (separator dot) (span (offset 2420) (line 91) (column 39) (len 16)))))
    (reference r37 (scope relative) (span (offset 2447) (line 92) (column 9) (len 12)) (segments (segment 0 (token "wheelTorque1") (name "wheelTorque1") (separator none) (span (offset 2447) (line 92) (column 9) (len 12)))))
    (reference r38 (scope relative) (span (offset 2462) (line 92) (column 24) (len 49)) (segments (segment 0 (token "rearAxle") (name "rearAxle") (separator none) (span (offset 2462) (line 92) (column 24) (len 8))) (segment 1 (token "leftHalfAxle") (name "leftHalfAxle") (separator dot) (span (offset 2471) (line 92) (column 33) (len 12))) (segment 2 (token "axleToWheelPort") (name "axleToWheelPort") (separator dot) (span (offset 2484) (line 92) (column 46) (len 15))) (segment 3 (token "wheelTorque") (name "wheelTorque") (separator dot) (span (offset 2500) (line 92) (column 62) (len 11)))))
    (reference r39 (scope relative) (span (offset 2522) (line 93) (column 9) (len 12)) (segments (segment 0 (token "wheelTorque2") (name "wheelTorque2") (separator none) (span (offset 2522) (line 93) (column 9) (len 12)))))
    (reference r40 (scope relative) (span (offset 2537) (line 93) (column 24) (len 50)) (segments (segment 0 (token "rearAxle") (name "rearAxle") (separator none) (span (offset 2537) (line 93) (column 24) (len 8))) (segment 1 (token "rightHalfAxle") (name "rightHalfAxle") (separator dot) (span (offset 2546) (line 93) (column 33) (len 13))) (segment 2 (token "axleToWheelPort") (name "axleToWheelPort") (separator dot) (span (offset 2560) (line 93) (column 47) (len 15))) (segment 3 (token "wheelTorque") (name "wheelTorque") (separator dot) (span (offset 2576) (line 93) (column 63) (len 11)))))
    (reference r41 (scope relative) (span (offset 2610) (line 96) (column 13) (len 8)) (segments (segment 0 (token "rearAxle") (name "rearAxle") (separator none) (span (offset 2610) (line 96) (column 13) (len 8)))))
    (reference r42 (scope relative) (span (offset 2634) (line 97) (column 14) (len 12)) (segments (segment 0 (token "leftHalfAxle") (name "leftHalfAxle") (separator none) (span (offset 2634) (line 97) (column 14) (len 12)))))
    (reference r43 (scope relative) (span (offset 2663) (line 98) (column 15) (len 15)) (segments (segment 0 (token "axleToWheelPort") (name "axleToWheelPort") (separator none) (span (offset 2663) (line 98) (column 15) (len 15)))))
    (reference r44 (scope relative) (span (offset 2738) (line 102) (column 14) (len 13)) (segments (segment 0 (token "rightHalfAxle") (name "rightHalfAxle") (separator none) (span (offset 2738) (line 102) (column 14) (len 13)))))
    (reference r45 (scope relative) (span (offset 2768) (line 103) (column 15) (len 15)) (segments (segment 0 (token "axleToWheelPort") (name "axleToWheelPort") (separator none) (span (offset 2768) (line 103) (column 15) (len 15)))))
  )
  (root (package (name "4a-Functional Allocation") (body brace (import (target (span (span (offset 53) (line 2) (column 17) (len 29))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 79) (line 2) (column 43) (len 3))) (separator (span (offset 79) (line 2) (column 43) (len 2))) (marker (span (offset 81) (line 2) (column 45) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 100) (line 3) (column 17) (len 33))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 130) (line 3) (column 47) (len 3))) (separator (span (offset 130) (line 3) (column 47) (len 2))) (marker (span (offset 132) (line 3) (column 49) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 151) (line 4) (column 17) (len 50))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 198) (line 4) (column 64) (len 3))) (separator (span (offset 198) (line 4) (column 64) (len 2))) (marker (span (offset 200) (line 4) (column 66) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c1_functional_allocation") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r3))) (value none))) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))) (perform (target (reference (action (ref r5)) (redefines none))) (value none) (body brace (doc (name none) (locale none) (body (span (offset 468) (line 15) (column 5) (len 154)) (normalized "This allocates the action '3a-Function-based Behavior-1'::'provide power' as an enacted \nperformance of 'vehicle_c1_functional_allocation'.\n"))) (binding (direction in) (target (ref r6)) (value (expression (span (offset 720) (line 21) (column 17) (len 19)) (ref r7)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))) (perform (target (reference (action (ref r10)) (redefines none))) (value none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1294) (line 42) (column 7) (len 99)) (normalized " This allocates one of the sub-steps of 'provide power' to a sub-part of vehicle_c1. \n"))) (binding (direction in) (target (ref r11)) (value (expression (span (offset 1414) (line 46) (column 18) (len 19)) (ref r12)))) (binding (direction out) (target (ref r13)) (value (expression (span (offset 1458) (line 47) (column 24) (len 25)) (ref r14)))))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))) (perform (target (reference (action (ref r18)) (redefines none))) (value none) (body brace (binding (direction in) (target (ref r19)) (value (expression (span (offset 1730) (line 61) (column 23) (len 23)) (ref r20)))) (binding (direction out) (target (ref r21)) (value (expression (span (offset 1785) (line 62) (column 30) (len 30)) (ref r22)))))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r23)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r24)))) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r25)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))) (perform (target (reference (action (ref r26)) (redefines none))) (value none) (body brace (binding (direction in) (target (ref r27)) (value (expression (span (offset 2063) (line 76) (column 29) (len 30)) (ref r28)))) (binding (direction out) (target (ref r29)) (value (expression (span (offset 2123) (line 77) (column 28) (len 28)) (ref r30)))))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r31)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r32)))) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r33)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))) (perform (target (reference (action (ref r34)) (redefines none))) (value none) (body brace (binding (direction in) (target (ref r35)) (value (expression (span (offset 2408) (line 91) (column 27) (len 28)) (ref r36)))) (binding (direction out) (target (ref r37)) (value (expression (span (offset 2462) (line 92) (column 24) (len 49)) (ref r38)))) (binding (direction out) (target (ref r39)) (value (expression (span (offset 2537) (line 93) (column 24) (len 50)) (ref r40)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r41)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r42)))) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r43)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r44)))) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r45)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))))))))))))))
)
~~~
