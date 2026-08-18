# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (03-Function-based Behavior): 3e-Function-based Behavior-item"))
~~~
# SOURCE
~~~sysml
package '3e-Function-based Behavior-item' {
	public import Definitions::*;
	
	package Definitions {
		
		item def VehicleAssembly;
		item def AssembledVehicle :> VehicleAssembly;
		
		part def Vehicle :> AssembledVehicle;		
		part def Transmission;
		part def Engine;		
		
	}
	
	package Usages {
		
		part AssemblyLine {
		
			perform action 'assemble vehicle' {
				
				action 'assemble transmission into vehicle' {
					in item 'vehicle assy without transmission or engine' : VehicleAssembly;					
					in item transmission : Transmission {
						/* Note: A part can be treated as an item. */
					}
					
					out item 'vehicle assy without engine' : VehicleAssembly = 'vehicle assy without transmission or engine' {						
						part transmission : Transmission = 'assemble transmission into vehicle'.transmission {
							/* Note: An item can become a part of something else. */
						}
					}
				}
				
				flow 'assemble transmission into vehicle'.'vehicle assy without engine' 
				    to 'assemble engine into vehicle'.'vehicle assy without engine';
				
				action 'assemble engine into vehicle' {
					in item 'vehicle assy without engine' : VehicleAssembly {
						part transmission : Transmission;
					}
					in item engine : Engine;
					
					out item assembledVehicle : AssembledVehicle = 'vehicle assy without engine' {
						part engine : Engine = 'assemble engine into vehicle'.engine;
					}
				}
			}
			
			bind 'assemble vehicle'.'assemble engine into vehicle'.assembledVehicle = vehicle;
			
			part vehicle : Vehicle {
				/*
				 * Note: An in item one context can become a part in an other.
				 */
			
				part transmission: Transmission;
				part engine: Engine;
				
				perform action providePower;
			}
			
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "3e_function_based_behavior_item.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '3e-Function-based Behavior-item' {
    public import Definitions::*;
    package Definitions {
        item def VehicleAssembly;
        item def AssembledVehicle :> VehicleAssembly;
        part def Vehicle :> AssembledVehicle;
        part def Transmission;
        part def Engine;
    }
    package Usages {
        part AssemblyLine {
            perform action 'assemble vehicle' {
                action 'assemble transmission into vehicle' {
                    in item 'vehicle assy without transmission or engine' : VehicleAssembly;
                    in item transmission : Transmission {
                    }
                    out item 'vehicle assy without engine' : VehicleAssembly = 'vehicle assy without transmission or engine' {
                        part transmission : Transmission = 'assemble transmission into vehicle'.transmission {}
                    }
                }
                flow from 'assemble transmission into vehicle'.'vehicle assy without engine' to 'assemble engine into vehicle'.'vehicle assy without engine';
                action 'assemble engine into vehicle' {
                    in item 'vehicle assy without engine' : VehicleAssembly {
                        part transmission : Transmission;
                    }
                    in item engine : Engine;
                    out item assembledVehicle : AssembledVehicle = 'vehicle assy without engine' {
                        part engine : Engine = 'assemble engine into vehicle'.engine;
                    }
                }
            }
            bind 'assemble vehicle'.'assemble engine into vehicle'.assembledVehicle = vehicle;
            part vehicle : Vehicle {
                part transmission : Transmission;
                part engine : Engine;
                perform action providePower;
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 59) (line 2) (column 16) (len 11)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 59) (line 2) (column 16) (len 11)))))
    (reference r1 (scope relative) (span (offset 162) (line 7) (column 32) (len 15)) (segments (segment 0 (token "VehicleAssembly") (name "VehicleAssembly") (separator none) (span (offset 162) (line 7) (column 32) (len 15)))))
    (reference r2 (scope relative) (span (offset 1537) (line 51) (column 19) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 1537) (line 51) (column 19) (len 7)))))
    (reference r3 (scope relative) (span (offset 1656) (line 56) (column 24) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 1656) (line 56) (column 24) (len 12)))))
    (reference r4 (scope relative) (span (offset 1687) (line 57) (column 18) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1687) (line 57) (column 18) (len 6)))))
  )
  (root (package (name "3e-Function-based Behavior-item") (body brace (import (target (span (span (offset 59) (line 2) (column 16) (len 14))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 70) (line 2) (column 27) (len 3))) (separator (span (offset 70) (line 2) (column 27) (len 2))) (marker (span (offset 72) (line 2) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (item-def (name "VehicleAssembly") (individual false) (specializes none) (body semicolon)) (item-def (name "AssembledVehicle") (individual false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1)))) (body semicolon)) (part-def (name "Vehicle") (body semicolon)) (part-def (name "Transmission") (body semicolon)) (part-def (name "Engine") (body semicolon)))) (package (name "Usages") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "AssemblyLine") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (perform) (bind) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (perform))))))))))
)
~~~
