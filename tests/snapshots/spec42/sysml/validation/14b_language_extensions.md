# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (14-Language Extensions): 14b-Language Extensions"))
~~~
# SOURCE
~~~sysml
package '14b-Language-Extensions' {
	
	package LibraryModel {
		
		part def ECU;
		
	}
	
	package UserModel {
		
		package Definitions {
			private import LibraryModel::*;
			
			part def VehicleControlUnit :> ECU;
			part def EngineControlUnit :> ECU;
			
			part def Vehicle;
			part def Engine;
			part def CanBus;
			
			port def BusIF;
		}
		
		package Usages {
			private import Definitions::*;
			
			part vehicle1: Vehicle {
				part vehicleControlUnit : VehicleControlUnit {
					port busIF: ~BusIF;
				}
				
				connect vehicleControlUnit.busIF to canBus.vehicleControlIF;
				
				part canBus: CanBus {
					port vehicleControlIF: BusIF;
					port engineControlIF: BusIF;
					port sensorIF: BusIF;					
				}
				
				connect engine.engineControlUnit.busIF to canBus.engineControlIF;
				
				part engine: Engine {
					part engineControlUnit: EngineControlUnit {
						port busIF: ~BusIF;
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
  (document "14b_language_extensions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '14b-Language-Extensions' {
    package LibraryModel {
        part def ECU;
    }
    package UserModel {
        package Definitions {
            private import LibraryModel::*;
            part def VehicleControlUnit :> ECU;
            part def EngineControlUnit :> ECU;
            part def Vehicle;
            part def Engine;
            part def CanBus;
            port def BusIF;
        }
        package Usages {
            private import Definitions::*;
            part vehicle1 : Vehicle {
                part vehicleControlUnit : VehicleControlUnit {
                    port busIF : ~BusIF;
                }
                connect vehicleControlUnit.busIF to canBus.vehicleControlIF;
                part canBus : CanBus {
                    port vehicleControlIF : BusIF;
                    port engineControlIF : BusIF;
                    port sensorIF : BusIF;
                }
                connect engine.engineControlUnit.busIF to canBus.engineControlIF;
                part engine : Engine {
                    part engineControlUnit : EngineControlUnit {
                        port busIF : ~BusIF;
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
    (reference r0 (scope relative) (span (offset 155) (line 12) (column 19) (len 12)) (segments (segment 0 (token "LibraryModel") (name "LibraryModel") (separator none) (span (offset 155) (line 12) (column 19) (len 12)))))
    (reference r1 (scope relative) (span (offset 385) (line 25) (column 19) (len 11)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 385) (line 25) (column 19) (len 11)))))
    (reference r2 (scope relative) (span (offset 423) (line 27) (column 19) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 423) (line 27) (column 19) (len 7)))))
    (reference r3 (scope relative) (span (offset 463) (line 28) (column 31) (len 18)) (segments (segment 0 (token "VehicleControlUnit") (name "VehicleControlUnit") (separator none) (span (offset 463) (line 28) (column 31) (len 18)))))
    (reference r4 (scope relative) (span (offset 502) (line 29) (column 19) (len 5)) (segments (segment 0 (token "BusIF") (name "BusIF") (separator none) (span (offset 502) (line 29) (column 19) (len 5)))))
    (reference r5 (scope relative) (span (offset 607) (line 34) (column 18) (len 6)) (segments (segment 0 (token "CanBus") (name "CanBus") (separator none) (span (offset 607) (line 34) (column 18) (len 6)))))
    (reference r6 (scope relative) (span (offset 644) (line 35) (column 29) (len 5)) (segments (segment 0 (token "BusIF") (name "BusIF") (separator none) (span (offset 644) (line 35) (column 29) (len 5)))))
    (reference r7 (scope relative) (span (offset 678) (line 36) (column 28) (len 5)) (segments (segment 0 (token "BusIF") (name "BusIF") (separator none) (span (offset 678) (line 36) (column 28) (len 5)))))
    (reference r8 (scope relative) (span (offset 705) (line 37) (column 21) (len 5)) (segments (segment 0 (token "BusIF") (name "BusIF") (separator none) (span (offset 705) (line 37) (column 21) (len 5)))))
    (reference r9 (scope relative) (span (offset 820) (line 42) (column 18) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 820) (line 42) (column 18) (len 6)))))
    (reference r10 (scope relative) (span (offset 858) (line 43) (column 30) (len 17)) (segments (segment 0 (token "EngineControlUnit") (name "EngineControlUnit") (separator none) (span (offset 858) (line 43) (column 30) (len 17)))))
    (reference r11 (scope relative) (span (offset 897) (line 44) (column 20) (len 5)) (segments (segment 0 (token "BusIF") (name "BusIF") (separator none) (span (offset 897) (line 44) (column 20) (len 5)))))
  )
  (root (package (name "14b-Language-Extensions") (body brace (package (name "LibraryModel") (body brace (part-def (name "ECU") (body semicolon)))) (package (name "UserModel") (body brace (package (name "Definitions") (body brace (import (target (span (span (offset 155) (line 12) (column 19) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 167) (line 12) (column 31) (len 3))) (separator (span (offset 167) (line 12) (column 31) (len 2))) (marker (span (offset 169) (line 12) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "VehicleControlUnit") (body semicolon)) (part-def (name "EngineControlUnit") (body semicolon)) (part-def (name "Vehicle") (body semicolon)) (part-def (name "Engine") (body semicolon)) (part-def (name "CanBus") (body semicolon)) (port-def (name "BusIF") (specializes none) (body semicolon)))) (package (name "Usages") (body brace (import (target (span (span (offset 385) (line 25) (column 19) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 396) (line 25) (column 30) (len 3))) (separator (span (offset 396) (line 25) (column 30) (len 2))) (marker (span (offset 398) (line 25) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleControlUnit") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "busIF") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connect) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "canBus") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicleControlIF") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engineControlIF") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "sensorIF") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connect) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engineControlUnit") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "busIF") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))))))))))
)
~~~
