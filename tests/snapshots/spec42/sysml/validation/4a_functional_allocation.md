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
    (reference r5 (scope relative) (span (offset 1172) (line 36) (column 12) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 1172) (line 36) (column 12) (len 6)))))
    (reference r6 (scope relative) (span (offset 1193) (line 37) (column 13) (len 11)) (segments (segment 0 (token "fuelCmdPort") (name "fuelCmdPort") (separator none) (span (offset 1193) (line 37) (column 13) (len 11)))))
    (reference r7 (scope relative) (span (offset 1506) (line 50) (column 13) (len 12)) (segments (segment 0 (token "drivePwrPort") (name "drivePwrPort") (separator none) (span (offset 1506) (line 50) (column 13) (len 12)))))
    (reference r8 (scope relative) (span (offset 1574) (line 55) (column 12) (len 12)) (segments (segment 0 (token "transmission") (name "transmission") (separator none) (span (offset 1574) (line 55) (column 12) (len 12)))))
    (reference r9 (scope relative) (span (offset 1601) (line 56) (column 13) (len 10)) (segments (segment 0 (token "clutchPort") (name "clutchPort") (separator none) (span (offset 1601) (line 56) (column 13) (len 10)))))
    (reference r10 (scope relative) (span (offset 1835) (line 65) (column 13) (len 11)) (segments (segment 0 (token "shaftPort_a") (name "shaftPort_a") (separator none) (span (offset 1835) (line 65) (column 13) (len 11)))))
    (reference r11 (scope relative) (span (offset 1908) (line 70) (column 12) (len 10)) (segments (segment 0 (token "driveshaft") (name "driveshaft") (separator none) (span (offset 1908) (line 70) (column 12) (len 10)))))
    (reference r12 (scope relative) (span (offset 1933) (line 71) (column 13) (len 11)) (segments (segment 0 (token "shaftPort_b") (name "shaftPort_b") (separator none) (span (offset 1933) (line 71) (column 13) (len 11)))))
    (reference r13 (scope relative) (span (offset 2171) (line 80) (column 13) (len 11)) (segments (segment 0 (token "shaftPort_c") (name "shaftPort_c") (separator none) (span (offset 2171) (line 80) (column 13) (len 11)))))
    (reference r14 (scope relative) (span (offset 2245) (line 85) (column 12) (len 16)) (segments (segment 0 (token "rearAxleAssembly") (name "rearAxleAssembly") (separator none) (span (offset 2245) (line 85) (column 12) (len 16)))))
    (reference r15 (scope relative) (span (offset 2276) (line 86) (column 13) (len 11)) (segments (segment 0 (token "shaftPort_d") (name "shaftPort_d") (separator none) (span (offset 2276) (line 86) (column 13) (len 11)))))
    (reference r16 (scope relative) (span (offset 2610) (line 96) (column 13) (len 8)) (segments (segment 0 (token "rearAxle") (name "rearAxle") (separator none) (span (offset 2610) (line 96) (column 13) (len 8)))))
    (reference r17 (scope relative) (span (offset 2634) (line 97) (column 14) (len 12)) (segments (segment 0 (token "leftHalfAxle") (name "leftHalfAxle") (separator none) (span (offset 2634) (line 97) (column 14) (len 12)))))
    (reference r18 (scope relative) (span (offset 2663) (line 98) (column 15) (len 15)) (segments (segment 0 (token "axleToWheelPort") (name "axleToWheelPort") (separator none) (span (offset 2663) (line 98) (column 15) (len 15)))))
    (reference r19 (scope relative) (span (offset 2738) (line 102) (column 14) (len 13)) (segments (segment 0 (token "rightHalfAxle") (name "rightHalfAxle") (separator none) (span (offset 2738) (line 102) (column 14) (len 13)))))
    (reference r20 (scope relative) (span (offset 2768) (line 103) (column 15) (len 15)) (segments (segment 0 (token "axleToWheelPort") (name "axleToWheelPort") (separator none) (span (offset 2768) (line 103) (column 15) (len 15)))))
  )
  (root (package (name "4a-Functional Allocation") (body brace (import (target (span (span (offset 53) (line 2) (column 17) (len 29))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 79) (line 2) (column 43) (len 3))) (separator (span (offset 79) (line 2) (column 43) (len 2))) (marker (span (offset 81) (line 2) (column 45) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 100) (line 3) (column 17) (len 33))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 130) (line 3) (column 47) (len 3))) (separator (span (offset 130) (line 3) (column 47) (len 2))) (marker (span (offset 132) (line 3) (column 49) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 151) (line 4) (column 17) (len 50))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 198) (line 4) (column 64) (len 3))) (separator (span (offset 198) (line 4) (column 64) (len 2))) (marker (span (offset 200) (line 4) (column 66) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c1_functional_allocation") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r3))) (value none))) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))) (perform) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))) (perform) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))) (perform) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))) (perform) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r14)))) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))) (perform) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r18)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19)))) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (intersects none) (value none) (body brace (in-out-declaration))))))))))))))
)
~~~
