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
    (reference r12 (scope relative) (span (offset 912) (line 52) (column 16) (len 4)) (segments (segment 0 (token "Pump") (name "Pump") (separator none) (span (offset 912) (line 52) (column 16) (len 4)))))
    (reference r13 (scope relative) (span (offset 1150) (line 62) (column 19) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 1150) (line 62) (column 19) (len 7)))))
    (reference r14 (scope relative) (span (offset 1356) (line 70) (column 21) (len 8)) (segments (segment 0 (token "FuelTank") (name "FuelTank") (separator none) (span (offset 1356) (line 70) (column 21) (len 8)))))
    (reference r15 (scope relative) (span (offset 1394) (line 71) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1394) (line 71) (column 28) (len 4)))))
    (reference r16 (scope relative) (span (offset 1427) (line 72) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1427) (line 72) (column 28) (len 4)))))
    (reference r17 (scope relative) (span (offset 1434) (line 72) (column 35) (len 4)) (segments (segment 0 (token "fuel") (name "fuel") (separator none) (span (offset 1434) (line 72) (column 35) (len 4)))))
    (reference r18 (scope relative) (span (offset 1439) (line 72) (column 40) (len 6)) (segments (segment 0 (token "volume") (name "volume") (separator none) (span (offset 1439) (line 72) (column 40) (len 6)))))
    (reference r19 (scope relative) (span (offset 1448) (line 72) (column 49) (len 9)) (segments (segment 0 (token "volumeMax") (name "volumeMax") (separator none) (span (offset 1448) (line 72) (column 49) (len 9)))))
    (reference r20 (scope relative) (span (offset 1506) (line 75) (column 18) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 1506) (line 75) (column 18) (len 4)))))
    (reference r21 (scope relative) (span (offset 1538) (line 76) (column 26) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1538) (line 76) (column 26) (len 4)))))
  )
  (root (package (name "3d-Function-based Behavior-item") (body brace (import (target (span (span (offset 60) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 95) (line 3) (column 16) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 106) (line 3) (column 27) (len 3))) (separator (span (offset 106) (line 3) (column 27) (len 2))) (marker (span (offset 108) (line 3) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 126) (line 4) (column 16) (len 9))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 132) (line 4) (column 22) (len 3))) (separator (span (offset 132) (line 4) (column 22) (len 2))) (marker (span (offset 134) (line 4) (column 24) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (item-def (name "Fuel") (modifiers) (individual false) (specializes none) (body semicolon)) (port-def (name "FuelPort") (modifiers) (specializes none) (body brace (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuel") (short-name none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "Pump") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelInPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelOutPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "StorageTank") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelOutPort") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "FuelTank") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelInPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Vehicle") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelInPort") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (action-def (name "PumpFuel") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "fuelIn") (subsets none) (type (ref r9)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 541) (line 32) (column 4) (len 17))) (in-out (direction out) (reference false) (declaration "fuelOut") (subsets none) (type (ref r10)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 562) (line 33) (column 4) (len 19))))))) (package (name "Usages") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "context") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 641) (line 42) (column 6) (len 17)) (normalized "Storage Element "))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "storageTank") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (flow-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "pump") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform))) (flow-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (flow-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1316) (line 69) (column 7) (len 17)) (normalized "Storage Element "))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "fuelTank") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "volumeMax") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "fuelLevel") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1434) (line 72) (column 35) (len 23)) (binary (operator "/") (left (expression (span (offset 1434) (line 72) (column 35) (len 11)) (member-access (base (expression (span (offset 1434) (line 72) (column 35) (len 4)) (ref r17))) (separator dot) (member (ref r18))))) (right (expression (span (offset 1448) (line 72) (column 49) (len 9)) (ref r19)))))))) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1473) (line 74) (column 9) (len 13)) (normalized "Stored Item "))) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "fuel") (short-name none) (type (ref r20)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "volume") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1552) (line 77) (column 9) (len 20)) (normalized "isConserved = true "))))))))))))))))
)
~~~
