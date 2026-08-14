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
            part storageTank : StorageTank;
            flow of fuel : Fuel from storageTank.fuelOutPort.fuel to pump.fuelInPort.fuel {}
            part pump : Pump {
                perform action pumpFuel : PumpFuel {
                    in fuelIn = fuelInPort.fuel;
                    out fuelOut = fuelOutPort.fuel;
                }
            }
            flow of fuel : Fuel from pump.fuelOutPort.fuel to vehicle.fuelInPort.fuel;
            part vehicle : Vehicle {
                flow from fuelInPort.fuel to fuelTank.fuel {}
                part fuelTank : FuelTank {
                    attribute volumeMax : Real;
                    attribute fuelLevel : Real = fuel.volume / volumeMax;
                    item fuel : Fuel {
                        attribute volume : Real;
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
    (reference r3 (scope relative) (span (offset 280) (line 15) (column 23) (len 8)) (segments (segment 0 (token "FuelPort") (name "FuelPort") (separator none) (span (offset 280) (line 15) (column 23) (len 8)))))
    (reference r4 (scope relative) (span (offset 312) (line 16) (column 23) (len 8)) (segments (segment 0 (token "FuelPort") (name "FuelPort") (separator none) (span (offset 312) (line 16) (column 23) (len 8)))))
    (reference r5 (scope relative) (span (offset 376) (line 20) (column 23) (len 8)) (segments (segment 0 (token "FuelPort") (name "FuelPort") (separator none) (span (offset 376) (line 20) (column 23) (len 8)))))
    (reference r6 (scope relative) (span (offset 437) (line 24) (column 23) (len 8)) (segments (segment 0 (token "FuelPort") (name "FuelPort") (separator none) (span (offset 437) (line 24) (column 23) (len 8)))))
    (reference r7 (scope relative) (span (offset 497) (line 28) (column 23) (len 8)) (segments (segment 0 (token "FuelPort") (name "FuelPort") (separator none) (span (offset 497) (line 28) (column 23) (len 8)))))
    (reference r8 (scope relative) (span (offset 553) (line 32) (column 16) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 553) (line 32) (column 16) (len 4)))))
    (reference r9 (scope relative) (span (offset 576) (line 33) (column 18) (len 4)) (segments (segment 0 (token "Fuel") (name "Fuel") (separator none) (span (offset 576) (line 33) (column 18) (len 4)))))
  )
  (root (package (name "3d-Function-based Behavior-item") (body (import (target (span (span (offset 60) (line 2) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 95) (line 3) (column 16) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 106) (line 3) (column 27) (len 3))) (separator (span (offset 106) (line 3) (column 27) (len 2))) (marker (span (offset 108) (line 3) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 126) (line 4) (column 16) (len 9))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 132) (line 4) (column 22) (len 3))) (separator (span (offset 132) (line 4) (column 22) (len 2))) (marker (span (offset 134) (line 4) (column 24) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body (item-def) (port-def (name "FuelPort") (specializes none) (body (item-usage))) (part-def (name "Pump") (body (port-usage (declaration-name "fuelInPort") (direction none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (declaration-name "fuelOutPort") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "StorageTank") (body (port-usage (declaration-name "fuelOutPort") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "FuelTank") (body (port-usage (declaration-name "fuelInPort") (direction none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Vehicle") (body (port-usage (declaration-name "fuelInPort") (direction none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (action-def (name "PumpFuel") (specializes none) (body (in-out (direction in) (reference false) (declaration "fuelIn") (type (ref r8)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 541) (line 32) (column 4) (len 17))) (in-out (direction out) (reference false) (declaration "fuelOut") (type (ref r9)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 562) (line 33) (column 4) (len 19))))))) (package (name "Usages") (body (part-usage))))))
)
~~~
