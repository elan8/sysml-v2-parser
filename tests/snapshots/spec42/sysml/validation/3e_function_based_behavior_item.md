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
                        /* Note: A part can be treated as an item. */
                    }
                    out item 'vehicle assy without engine' : VehicleAssembly = 'vehicle assy without transmission or engine' {
                        part transmission : Transmission = 'assemble transmission into vehicle'.transmission {
                            /* Note: An item can become a part of something else. */
                        }
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
                /*
				 * Note: An in item one context can become a part in an other.
				 */
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
    (reference r2 (scope relative) (span (offset 479) (line 22) (column 62) (len 15)) (segments (segment 0 (token "VehicleAssembly") (name "VehicleAssembly") (separator none) (span (offset 479) (line 22) (column 62) (len 15)))))
    (reference r3 (scope relative) (span (offset 529) (line 23) (column 29) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 529) (line 23) (column 29) (len 12)))))
    (reference r4 (scope relative) (span (offset 655) (line 27) (column 47) (len 15)) (segments (segment 0 (token "VehicleAssembly") (name "VehicleAssembly") (separator none) (span (offset 655) (line 27) (column 47) (len 15)))))
    (reference r5 (scope relative) (span (offset 673) (line 27) (column 65) (len 45)) (segments (segment 0 (token "'vehicle assy without transmission or engine'") (name "vehicle assy without transmission or engine") (separator none) (span (offset 673) (line 27) (column 65) (len 45)))))
    (reference r6 (scope relative) (span (offset 753) (line 28) (column 27) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 753) (line 28) (column 27) (len 12)))))
    (reference r7 (scope relative) (span (offset 768) (line 28) (column 42) (len 36)) (segments (segment 0 (token "'assemble transmission into vehicle'") (name "assemble transmission into vehicle") (separator none) (span (offset 768) (line 28) (column 42) (len 36)))))
    (reference r8 (scope relative) (span (offset 805) (line 28) (column 79) (len 12)) (segments (segment 0 (token "transmission") (name "transmission") (separator none) (span (offset 805) (line 28) (column 79) (len 12)))))
    (reference r9 (scope relative) (span (offset 1154) (line 38) (column 46) (len 15)) (segments (segment 0 (token "VehicleAssembly") (name "VehicleAssembly") (separator none) (span (offset 1154) (line 38) (column 46) (len 15)))))
    (reference r10 (scope relative) (span (offset 1198) (line 39) (column 27) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 1198) (line 39) (column 27) (len 12)))))
    (reference r11 (scope relative) (span (offset 1241) (line 41) (column 23) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1241) (line 41) (column 23) (len 6)))))
    (reference r12 (scope relative) (span (offset 1288) (line 43) (column 34) (len 16)) (segments (segment 0 (token "AssembledVehicle") (name "AssembledVehicle") (separator none) (span (offset 1288) (line 43) (column 34) (len 16)))))
    (reference r13 (scope relative) (span (offset 1307) (line 43) (column 53) (len 29)) (segments (segment 0 (token "'vehicle assy without engine'") (name "vehicle assy without engine") (separator none) (span (offset 1307) (line 43) (column 53) (len 29)))))
    (reference r14 (scope relative) (span (offset 1359) (line 44) (column 21) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1359) (line 44) (column 21) (len 6)))))
    (reference r15 (scope relative) (span (offset 1368) (line 44) (column 30) (len 30)) (segments (segment 0 (token "'assemble engine into vehicle'") (name "assemble engine into vehicle") (separator none) (span (offset 1368) (line 44) (column 30) (len 30)))))
    (reference r16 (scope relative) (span (offset 1399) (line 44) (column 61) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 1399) (line 44) (column 61) (len 6)))))
    (reference r17 (scope relative) (span (offset 1537) (line 51) (column 19) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 1537) (line 51) (column 19) (len 7)))))
    (reference r18 (scope relative) (span (offset 1656) (line 56) (column 24) (len 12)) (segments (segment 0 (token "Transmission") (name "Transmission") (separator none) (span (offset 1656) (line 56) (column 24) (len 12)))))
    (reference r19 (scope relative) (span (offset 1687) (line 57) (column 18) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 1687) (line 57) (column 18) (len 6)))))
  )
  (root (package (name "3e-Function-based Behavior-item") (body brace (import (target (span (span (offset 59) (line 2) (column 16) (len 14))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 70) (line 2) (column 27) (len 3))) (separator (span (offset 70) (line 2) (column 27) (len 2))) (marker (span (offset 72) (line 2) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Definitions") (body brace (item-def (name "VehicleAssembly") (modifiers) (individual false) (specializes none) (body semicolon)) (item-def (name "AssembledVehicle") (modifiers) (individual false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1)))) (body semicolon)) (part-def (name "Vehicle") (modifiers) (body semicolon)) (part-def (name "Transmission") (modifiers) (body semicolon)) (part-def (name "Engine") (modifiers) (body semicolon)))) (package (name "Usages") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "AssemblyLine") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform (target (action (name "assemble vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body brace (action-usage (keyword action) (name "assemble transmission into vehicle") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "vehicle assy without transmission or engine") (short-name none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "transmission") (short-name none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 552) (line 24) (column 9) (len 41)) (normalized "Note: A part can be treated as an item. "))))) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "vehicle assy without engine") (short-name none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 673) (line 27) (column 65) (len 45)) (ref r5))))) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 768) (line 28) (column 42) (len 49)) (member-access (base (expression (span (offset 768) (line 28) (column 42) (len 36)) (ref r7))) (separator dot) (member (ref r8))))))) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 829) (line 29) (column 10) (len 52)) (normalized "Note: An item can become a part of something else. "))))))))) (action) (action-usage (keyword action) (name "assemble engine into vehicle") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "vehicle assy without engine") (short-name none) (type (ref r9)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "engine") (short-name none) (type (ref r11)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "assembledVehicle") (short-name none) (type (ref r12)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1307) (line 43) (column 53) (len 29)) (ref r13))))) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1368) (line 44) (column 30) (len 37)) (member-access (base (expression (span (offset 1368) (line 44) (column 30) (len 30)) (ref r15))) (separator dot) (member (ref r16))))))) (body semicolon)))))))) (bind) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1553) (line 52) (column 7) (len 73)) (normalized "Note: An in item one context can become a part in an other.\n"))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (perform (target (action (name "providePower") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body semicolon)))))))))))
)
~~~
