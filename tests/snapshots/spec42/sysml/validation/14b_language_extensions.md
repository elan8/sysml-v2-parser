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
  )
  (root (package (name "14b-Language-Extensions") (body brace (package (name "LibraryModel") (body brace (part-def (name "ECU") (body semicolon)))) (package (name "UserModel") (body brace (package (name "Definitions") (body brace (import (target (span (span (offset 155) (line 12) (column 19) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 167) (line 12) (column 31) (len 3))) (separator (span (offset 167) (line 12) (column 31) (len 2))) (marker (span (offset 169) (line 12) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "VehicleControlUnit") (body semicolon)) (part-def (name "EngineControlUnit") (body semicolon)) (part-def (name "Vehicle") (body semicolon)) (part-def (name "Engine") (body semicolon)) (part-def (name "CanBus") (body semicolon)) (port-def (name "BusIF") (specializes none) (body semicolon)))) (package (name "Usages") (body brace (import (target (span (span (offset 385) (line 25) (column 19) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 396) (line 25) (column 30) (len 3))) (separator (span (offset 396) (line 25) (column 30) (len 2))) (marker (span (offset 398) (line 25) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (declaration-name "vehicle1") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (body brace (part-usage) (connect) (part-usage) (connect) (part-usage))))))))))
)
~~~
