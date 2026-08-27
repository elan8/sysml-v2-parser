# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (03-Function-based Behavior): 3d-Function-based Behavior-item"))
~~~
# SOURCE
~~~sysml
package '3d-Function-based Behavior-item' {
	private import ScalarValues::Real;
	public import Definitions::*;
	public import Usages::*;
	
	package Definitions {
		
		item def Fuel;
		
		port def FuelPort {
			out item fuel: Fuel;
		}
				
		part def Pump {
			port fuelInPort : ~FuelPort;
			port fuelOutPort : FuelPort;
		}
		
		part def StorageTank {
			port fuelOutPort : FuelPort;
		}
		
		part def FuelTank {
			port fuelInPort : ~FuelPort;
		}
		
		part def Vehicle {
			port fuelInPort : ~FuelPort;
		}
		
		action def PumpFuel {
			in fuelIn : Fuel;
			out fuelOut : Fuel;
		}
		
	}
	
	package Usages {
		
		part context {
			
			/* Storage Element */
			part storageTank : StorageTank;
			
			flow of  fuel : Fuel
				from storageTank.fuelOutPort.fuel to pump.fuelInPort.fuel {
				/*
				 * Note: Explicitly notating that the flow is "of fuel : Fuel" is optional.
				 */					
			}
			
			part pump : Pump {
				perform action pumpFuel : PumpFuel {
					in fuelIn = fuelInPort.fuel;
					out fuelOut = fuelOutPort.fuel;
				}
			}
			
			flow of fuel : Fuel
				from pump.fuelOutPort.fuel to vehicle.fuelInPort.fuel;
			
			part vehicle : Vehicle {
				flow fuelInPort.fuel to fuelTank.fuel {
					/* 
					 * Note: The semantics of flowing to a "stored item" is tentative.
					 */					
				}
				
				/* Storage Element */
				part fuelTank : FuelTank {
					attribute volumeMax : Real;
					attribute fuelLevel : Real = fuel.volume / volumeMax;
					
					 /* Stored Item */
					item fuel : Fuel {
						attribute volume : Real;
						/* isConserved = true */
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
  (document "3d_function_based_behavior_item.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '3d-Function-based Behavior-item' {
    private import ScalarValues::Real;
    public import Definitions::*;
    public import Usages::*;
    package Definitions {
        item def Fuel;
        port def FuelPort {
            out item fuel : Fuel;
        }
        part def Pump {
            port fuelInPort : ~FuelPort;
            port fuelOutPort : FuelPort;
        }
        part def StorageTank {
            port fuelOutPort : FuelPort;
        }
        part def FuelTank {
            port fuelInPort : ~FuelPort;
        }
        part def Vehicle {
            port fuelInPort : ~FuelPort;
        }
        action def PumpFuel {
            in fuelIn : Fuel;
            out fuelOut : Fuel;
        }
    }
    package Usages {
        part context {
            /* Storage Element */
            part storageTank : StorageTank;
            flow of fuel : Fuel from storageTank.fuelOutPort.fuel to pump.fuelInPort.fuel {
                /*
				 * Note: Explicitly notating that the flow is "of fuel : Fuel" is optional.
				 */
            }
            part pump : Pump {
                perform action pumpFuel : PumpFuel {
                    in fuelIn = fuelInPort.fuel;
                    out fuelOut = fuelOutPort.fuel;
                }
            }
            flow of fuel : Fuel from pump.fuelOutPort.fuel to vehicle.fuelInPort.fuel;
            part vehicle : Vehicle {
                flow from fuelInPort.fuel to fuelTank.fuel {
                    /* 
					 * Note: The semantics of flowing to a "stored item" is tentative.
					 */
                }
                /* Storage Element */
                part fuelTank : FuelTank {
                    attribute volumeMax : Real;
                    attribute fuelLevel : Real = fuel.volume / volumeMax;
                    /* Stored Item */
                    item fuel : Fuel {
                        attribute volume : Real;
                        /* isConserved = true */
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
    (reference r0 (scope relative) (span (offset 60) (line 2) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 60) (line 2) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 74) (line 2) (column 31) (len 4)))))
    (reference r1 (scope relative) (span (offset 95) (line 3) (column 16) (len 11)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 95) (line 3) (column 16) (len 11)))))
    (reference r2 (scope relative) (span (offset 126) (line 4) (column 16) (len 6)) (segments (segment 0 (token "Usages") (name "Usages") (separator none) (span (offset 126) (line 4) (column 16) (len 6)))))
    (reference r3 (scope relative) (span (offset 225) (line 11) (column 19) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 225) (line 11) (column 19) (len 4)))))
    (reference r4 (scope relative) (span (offset 280) (line 15) (column 23) (len 8)) (segments (segment 0 (token "FuelPort") (name "FuelPort") (separator none) (span (offset 280) (line 15) (column 23) (len 8)))))
    (reference r5 (scope relative) (span (offset 312) (line 16) (column 23) (len 8)) (segments (segment 0 (token "FuelPort") (name "FuelPort") (separator none) (span (offset 312) (line 16) (column 23) (len 8)))))
    (reference r6 (scope relative) (span (offset 376) (line 20) (column 23) (len 8)) (segments (segment 0 (token "FuelPort") (name "FuelPort") (separator none) (span (offset 376) (line 20) (column 23) (len 8)))))
    (reference r7 (scope relative) (span (offset 437) (line 24) (column 23) (len 8)) (segments (segment 0 (token "FuelPort") (name "FuelPort") (separator none) (span (offset 437) (line 24) (column 23) (len 8)))))
    (reference r8 (scope relative) (span (offset 497) (line 28) (column 23) (len 8)) (segments (segment 0 (token "FuelPort") (name "FuelPort") (separator none) (span (offset 497) (line 28) (column 23) (len 8)))))
    (reference r9 (scope relative) (span (offset 553) (line 32) (column 16) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 553) (line 32) (column 16) (len 4)))))
    (reference r10 (scope relative) (span (offset 576) (line 33) (column 18) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 576) (line 33) (column 18) (len 4)))))
    (reference r11 (scope relative) (span (offset 683) (line 43) (column 23) (len 11)) (segments (segment 0 (token "StorageTank") (name "StorageTank") (separator none) (span (offset 683) (line 43) (column 23) (len 11)))))
    (reference r12 (scope relative) (span (offset 719) (line 45) (column 20) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 719) (line 45) (column 20) (len 4)))))
    (reference r13 (scope relative) (span (offset 733) (line 46) (column 10) (len 28)) (segments (segment 0 (token "storageTank") (name "storageTank") (separator none) (span (offset 733) (line 46) (column 10) (len 11))) (segment 1 (token "fuelOutPort") (name "fuelOutPort") (separator dot) (span (offset 745) (line 46) (column 22) (len 11))) (segment 2 (token "fuel") (name "fuel") (separator dot) (span (offset 757) (line 46) (column 34) (len 4)))))
    (reference r14 (scope relative) (span (offset 765) (line 46) (column 42) (len 20)) (segments (segment 0 (token "pump") (name "pump") (separator none) (span (offset 765) (line 46) (column 42) (len 4))) (segment 1 (token "fuelInPort") (name "fuelInPort") (separator dot) (span (offset 770) (line 46) (column 47) (len 10))) (segment 2 (token "fuel") (name "fuel") (separator dot) (span (offset 781) (line 46) (column 58) (len 4)))))
    (reference r15 (scope relative) (span (offset 912) (line 52) (column 16) (len 4)) (segments (segment 0 (token "Pump") (name "Pump") (separator none) (span (offset 912) (line 52) (column 16) (len 4)))))
    (reference r16 (scope relative) (span (offset 949) (line 53) (column 31) (len 8)) (segments (segment 0 (token "PumpFuel") (name "PumpFuel") (separator none) (span (offset 949) (line 53) (column 31) (len 8)))))
    (reference r17 (scope relative) (span (offset 968) (line 54) (column 9) (len 6)) (segments (segment 0 (token "fuelIn") (name "fuelIn") (separator none) (span (offset 968) (line 54) (column 9) (len 6)))))
    (reference r18 (scope relative) (span (offset 977) (line 54) (column 18) (len 15)) (segments (segment 0 (token "fuelInPort") (name "fuelInPort") (separator none) (span (offset 977) (line 54) (column 18) (len 10))) (segment 1 (token "fuel") (name "fuel") (separator dot) (span (offset 988) (line 54) (column 29) (len 4)))))
    (reference r19 (scope relative) (span (offset 1003) (line 55) (column 10) (len 7)) (segments (segment 0 (token "fuelOut") (name "fuelOut") (separator none) (span (offset 1003) (line 55) (column 10) (len 7)))))
    (reference r20 (scope relative) (span (offset 1013) (line 55) (column 20) (len 16)) (segments (segment 0 (token "fuelOutPort") (name "fuelOutPort") (separator none) (span (offset 1013) (line 55) (column 20) (len 11))) (segment 1 (token "fuel") (name "fuel") (separator dot) (span (offset 1025) (line 55) (column 32) (len 4)))))
    (reference r21 (scope relative) (span (offset 1064) (line 59) (column 19) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 1064) (line 59) (column 19) (len 4)))))
    (reference r22 (scope relative) (span (offset 1078) (line 60) (column 10) (len 21)) (segments (segment 0 (token "pump") (name "pump") (separator none) (span (offset 1078) (line 60) (column 10) (len 4))) (segment 1 (token "fuelOutPort") (name "fuelOutPort") (separator dot) (span (offset 1083) (line 60) (column 15) (len 11))) (segment 2 (token "fuel") (name "fuel") (separator dot) (span (offset 1095) (line 60) (column 27) (len 4)))))
    (reference r23 (scope relative) (span (offset 1103) (line 60) (column 35) (len 23)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1103) (line 60) (column 35) (len 7))) (segment 1 (token "fuelInPort") (name "fuelInPort") (separator dot) (span (offset 1111) (line 60) (column 43) (len 10))) (segment 2 (token "fuel") (name "fuel") (separator dot) (span (offset 1122) (line 60) (column 54) (len 4)))))
    (reference r24 (scope relative) (span (offset 1150) (line 62) (column 19) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 1150) (line 62) (column 19) (len 7)))))
    (reference r25 (scope relative) (span (offset 1169) (line 63) (column 10) (len 15)) (segments (segment 0 (token "fuelInPort") (name "fuelInPort") (separator none) (span (offset 1169) (line 63) (column 10) (len 10))) (segment 1 (token "fuel") (name "fuel") (separator dot) (span (offset 1180) (line 63) (column 21) (len 4)))))
    (reference r26 (scope relative) (span (offset 1188) (line 63) (column 29) (len 13)) (segments (segment 0 (token "fuelTank") (name "fuelTank") (separator none) (span (offset 1188) (line 63) (column 29) (len 8))) (segment 1 (token "fuel") (name "fuel") (separator dot) (span (offset 1197) (line 63) (column 38) (len 4)))))
    (reference r27 (scope relative) (span (offset 1356) (line 70) (column 21) (len 8)) (segments (segment 0 (token "FuelTank") (name "FuelTank") (separator none) (span (offset 1356) (line 70) (column 21) (len 8)))))
    (reference r28 (scope relative) (span (offset 1394) (line 71) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1394) (line 71) (column 28) (len 4)))))
    (reference r29 (scope relative) (span (offset 1427) (line 72) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1427) (line 72) (column 28) (len 4)))))
    (reference r30 (scope relative) (span (offset 1434) (line 72) (column 35) (len 4)) (segments (segment 0 (token "fuel") (name "fuel") (separator none) (span (offset 1434) (line 72) (column 35) (len 4)))))
    (reference r31 (scope relative) (span (offset 1439) (line 72) (column 40) (len 6)) (segments (segment 0 (token "volume") (name "volume") (separator none) (span (offset 1439) (line 72) (column 40) (len 6)))))
    (reference r32 (scope relative) (span (offset 1448) (line 72) (column 49) (len 9)) (segments (segment 0 (token "volumeMax") (name "volumeMax") (separator none) (span (offset 1448) (line 72) (column 49) (len 9)))))
    (reference r33 (scope relative) (span (offset 1506) (line 75) (column 18) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 1506) (line 75) (column 18) (len 4)))))
    (reference r34 (scope relative) (span (offset 1538) (line 76) (column 26) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1538) (line 76) (column 26) (len 4)))))
  )
  (root (package (name "3d-Function-based Behavior-item") (body brace (import (target (span (span (offset 60) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 95) (line 3) (column 16) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 106) (line 3) (column 27) (len 3))) (separator (span (offset 106) (line 3) (column 27) (len 2))) (marker (span (offset 108) (line 3) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 126) (line 4) (column 16) (len 9))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 132) (line 4) (column 22) (len 3))) (separator (span (offset 132) (line 4) (column 22) (len 2))) (marker (span (offset 134) (line 4) (column 24) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (item-def (name "Fuel") (modifiers) (individual false) (specializes none) (body semicolon)) (port-def (name "FuelPort") (modifiers) (specializes none) (body brace (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuel") (short-name none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "Pump") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelInPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelOutPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "StorageTank") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelOutPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "FuelTank") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelInPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Vehicle") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelInPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (action-def (name "PumpFuel") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "fuelIn") (subsets none) (type (ref r9)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 541) (line 32) (column 4) (len 17))) (in-out (direction out) (kind none) (reference false) (declaration "fuelOut") (subsets none) (type (ref r10)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 562) (line 33) (column 4) (len 19))))))) (package (name "Usages") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "context") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 641) (line 42) (column 6) (len 17)) (normalized "Storage Element "))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "storageTank") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload (name "fuel") (type (ref r12)) (conjugated false) (multiplicity none)) (endpoints (from (connector-end (multiplicity none) (target (ref r13)) (references none))) (to (connector-end (multiplicity none) (target (ref r14)) (references none)))))) (body (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 794) (line 47) (column 7) (len 86)) (normalized "Note: Explicitly notating that the flow is \"of fuel : Fuel\" is optional.\n")))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "pump") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform (target (action (name "pumpFuel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body brace (binding (direction in) (target (ref r17)) (value (expression (span (offset 977) (line 54) (column 18) (len 15)) (ref r18)))) (binding (direction out) (target (ref r19)) (value (expression (span (offset 1013) (line 55) (column 20) (len 16)) (ref r20)))))))) (flow-usage (kind flow) (visibility none) (declaration (declared (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload (name "fuel") (type (ref r21)) (conjugated false) (multiplicity none)) (endpoints (from (connector-end (multiplicity none) (target (ref r22)) (references none))) (to (connector-end (multiplicity none) (target (ref r23)) (references none)))))) (body (body semicolon))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r25)) (references none))) (to (connector-end (multiplicity none) (target (ref r26)) (references none))))) (body (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1211) (line 64) (column 8) (len 80)) (normalized "Note: The semantics of flowing to a \"stored item\" is tentative.\n")))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1316) (line 69) (column 7) (len 17)) (normalized "Storage Element "))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelTank") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "volumeMax") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r28)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "fuelLevel") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1434) (line 72) (column 35) (len 23)) (binary (operator "/") (left (expression (span (offset 1434) (line 72) (column 35) (len 11)) (member-access (base (expression (span (offset 1434) (line 72) (column 35) (len 4)) (ref r30))) (separator dot) (member (ref r31))))) (right (expression (span (offset 1448) (line 72) (column 49) (len 9)) (ref r32)))))))) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1473) (line 74) (column 9) (len 13)) (normalized "Stored Item "))) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuel") (short-name none) (type (ref r33)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "volume") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r34)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1552) (line 77) (column 9) (len 20)) (normalized "isConserved = true "))))))))))))))))
)
~~~
