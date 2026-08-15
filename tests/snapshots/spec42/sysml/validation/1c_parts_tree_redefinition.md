# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (01-Parts Tree): 1c-Parts Tree Redefinition"))
~~~
# SOURCE
~~~sysml
package '1c-Parts Tree Redefinition' {
	private import SI::kg;
	
	package Definitions {	
		part def Vehicle {
			attribute mass :> ISQ::mass;
		}		
		part def AxleAssembly;		
		part def Axle { 
			attribute mass :> ISQ::mass;
		}	
		part def FrontAxle :> Axle { 
			attribute steeringAngle: ScalarValues::Real;
		}	
		part def Wheel;	
	}
		
	package Usages {
		private import Definitions::*;
		
		part vehicle1: Vehicle {
			attribute mass redefines Vehicle::mass default = 1750 [kg] {
			doc
			/*
			 * The mass attribute is redefined to give it a default value.
			 */
			}
					
			part frontAxleAssembly: AxleAssembly {
				part frontAxle: Axle;			
				part frontWheel: Wheel[2] ordered;
			}		
			part rearAxleAssembly: AxleAssembly {
				part rearAxle: Axle;
				part rearWheel: Wheel[2] ordered;
			}		
		}
	
		part vehicle1_c1 :> vehicle1 {
			/*
			 * 'vehicle1_c1' is a specialization of 'vehicle1' (technically 
			 * a subset). It inherits all the parts of 'vehicle1' and
			 * only needs to specify additional or redefined parts.
			 */
		
			attribute mass redefines vehicle1::mass = 2000 [kg] {
				/*
				 * The mass is further redefined to override the default value
				 * with a bound value for 'vehicle_c1'.
				 */
			}
					
			part frontAxleAssembly_c1 redefines frontAxleAssembly {
				part frontAxle_c1: FrontAxle redefines frontAxle {
					/*
					 * 'frontAxle_c1' redefines 'frontAxleAssembly'::'frontAxle'
					 * to give it a new name and the specialized type
					 * 'FrontAxle'.
					 */
				}
				
				/*
				 * 'frontWheel' is inherited from 'vehicle1'::'frontAxleAssembly',
				 * allowing it to be used in the following part declarations.
				 */
				
				part frontWheel_1 subsets frontWheel = frontWheel#(1);
				part frontWheel_2 subsets frontWheel = frontWheel#(2);
			}
				
			part rearAxleAssembly_c1 redefines rearAxleAssembly {
				part rearAxle_c1 redefines rearAxle {
					/*
					 * 'rearAxle_c1' redefines 'rearAxleAssembly'::'rearAxle'
					 * to give it a new name. It inherits the type 'Axle'
					 * from the redefined part.
					 */
				}
						
				part rearWheel_1 subsets rearWheel = rearWheel#(1);
				part rearWheel_2 subsets rearWheel = rearWheel#(2);
			}		
		}
		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "1c_parts_tree_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '1c-Parts Tree Redefinition' {
    private import SI::kg;
    package Definitions {
        part def Vehicle {
            attribute mass :> ISQ::mass;
        }
        part def AxleAssembly;
        part def Axle {
            attribute mass :> ISQ::mass;
        }
        part def FrontAxle :> Axle {
            attribute steeringAngle : ScalarValues::Real;
        }
        part def Wheel;
    }
    package Usages {
        private import Definitions::*;
        part vehicle1 : Vehicle {
            attribute mass :>> Vehicle::mass default = 1750 [kg] {
                doc
                /*
			 * The mass attribute is redefined to give it a default value.
			 */
            }
            part frontAxleAssembly : AxleAssembly {
                part frontAxle : Axle;
                part frontWheel : Wheel[2] ordered;
            }
            part rearAxleAssembly : AxleAssembly {
                part rearAxle : Axle;
                part rearWheel : Wheel[2] ordered;
            }
        }
        part vehicle1_c1 :> vehicle1 {
            attribute mass :>> vehicle1::mass = 2000 [kg] {
            }
            part frontAxleAssembly_c1 :>> frontAxleAssembly {
                part frontAxle_c1 : FrontAxle :>> frontAxle {
                }
                part frontWheel_1 :> frontWheel = frontWheel#(1);
                part frontWheel_2 :> frontWheel = frontWheel#(2);
            }
            part rearAxleAssembly_c1 :>> rearAxleAssembly {
                part rearAxle_c1 :>> rearAxle {
                }
                part rearWheel_1 :> rearWheel = rearWheel#(1);
                part rearWheel_2 :> rearWheel = rearWheel#(2);
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 55) (line 2) (column 17) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 55) (line 2) (column 17) (len 2))) (segment 1 (token "kg") (name "kg") (separator colon-colon) (span (offset 59) (line 2) (column 21) (len 2)))))
    (reference r1 (scope relative) (span (offset 131) (line 6) (column 22) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 131) (line 6) (column 22) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 136) (line 6) (column 27) (len 4)))))
    (reference r2 (scope relative) (span (offset 215) (line 10) (column 22) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 215) (line 10) (column 22) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 220) (line 10) (column 27) (len 4)))))
    (reference r3 (scope relative) (span (offset 291) (line 13) (column 29) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 291) (line 13) (column 29) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 305) (line 13) (column 43) (len 4)))))
    (reference r4 (scope relative) (span (offset 376) (line 19) (column 18) (len 11)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 376) (line 19) (column 18) (len 11)))))
  )
  (root (package (name "1c-Parts Tree Redefinition") (body (import (target (span (span (offset 55) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (package (name "Definitions") (body (part-def (name "Vehicle") (body (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "AxleAssembly") (body semicolon)) (part-def (name "Axle") (body (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "FrontAxle") (body (attribute-usage (declaration-name "steeringAngle") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Wheel") (body semicolon)))) (package (name "Usages") (body (import (target (span (span (offset 376) (line 19) (column 18) (len 14))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 387) (line 19) (column 29) (len 3))) (separator (span (offset 387) (line 19) (column 29) (len 2))) (marker (span (offset 389) (line 19) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage) (part-usage))))))
)
~~~
