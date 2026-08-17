# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (01-Parts Tree): 1a-Parts Tree"))
~~~
# SOURCE
~~~sysml
package '1a-Parts Tree' {
	private import SI::kg;
	
	package Definitions {	
		part def Vehicle {
			attribute mass :> ISQ::mass {
			doc
			/*
			 * The 'mass' attribute property is declared here to be a 
			 * specialization (subset) of the general 'mass' quantity 
			 * from the 'ISQ' (International System of Quantities) 
			 * library model.
			 */
			}
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
		private import Definitions::* {
			/*
			 * A "private" private import makes the imported names private to the
			 * imported package.
			 */
		}
	
		part vehicle1: Vehicle {
			/*
			 * 'vehicle1' is a package-owned part of type Vehicle.
			 */
			 
			attribute mass redefines Vehicle::mass = 1750 [kg] {
				/*
				 * This redefines the 'mass' attribute property from 'Vehicle' to 
				 * give it a fixed attribute.
				 */
			}
			
			part frontAxleAssembly: AxleAssembly {
				/*
				 * 'frontAxleAssembly' is a nested part of part 'vehicle1'.
				 * It is a composite part of the containing part.
				 * 
				 * (And similarly for 'rearAxleAssembly'.)
				 */
			
				part frontAxle: Axle;
				
				part frontWheel: Wheel[2] ordered {
					/*
					 * 'frontWheel' is a nested part of type 'Wheel' with
					 * multiplicity "2". This means that this axle assembly
					 * must have exactly two wheels. However, there is still
					 * only one 'frontWheel' part. The part is "ordered",
					 * so that the first wheel can be distinguished from the
					 * second.
					 */
				}
			}
			
			part rearAxleAssembly: AxleAssembly {
				part rearAxle: Axle;
				part rearWheel: Wheel[2] ordered;
			}
			
		}
	
		part vehicle1_c1: Vehicle {
			/*
			 * 'vehicle1_c1' is a modified copy of 'vehicle1'. There is no
			 * connection between this copy and the original version in the
			 * model.
			 */			
			
			attribute mass redefines Vehicle::mass = 2000 [kg] {
				/*
				 * The mass attribute has been modified.
				 */
			}
	
			part frontAxleAssembly: AxleAssembly {
				
				part frontAxle: FrontAxle {
					/*
					 * The part 'frontAxle' has been modified to have type 'FrontAxle'.
					 */
				}
				
				part frontWheel: Wheel[2] ordered {
					/*
					 * The parts 'frontWheel_1' and 'frontWheel_2' have been added
					 * as subsets of 'frontWheel'. These are separate parts from
					 * 'frontWheel', but essentially provide alternate names for
					 * each of the two wheels, as given by their defining expressions.
					 */
				}
				part frontWheel_1 subsets frontWheel = frontWheel#(1);
				part frontWheel_2 subsets frontWheel = frontWheel#(2);
			}
			
			part rearAxleAssembly: AxleAssembly {
				/*
				 * 'rearAxleAssembly' has also been modified to add subsetting parts
				 * for 'rearWheel'.
				 */
						
				part rearAxle: Axle;
				
				part rearWheel: Wheel[2] ordered;
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
  (document "1a_parts_tree.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '1a-Parts Tree' {
    private import SI::kg;
    package Definitions {
        part def Vehicle {
            attribute mass :> ISQ::mass {
                doc
                /*
			 * The 'mass' attribute property is declared here to be a 
			 * specialization (subset) of the general 'mass' quantity 
			 * from the 'ISQ' (International System of Quantities) 
			 * library model.
			 */
            }
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
        private import Definitions::* {}
        part vehicle1 : Vehicle {
            attribute mass :>> Vehicle::mass = 1750 [kg] {
            }
            part frontAxleAssembly : AxleAssembly {
                part frontAxle : Axle;
                part frontWheel : Wheel[2] ordered {}
            }
            part rearAxleAssembly : AxleAssembly {
                part rearAxle : Axle;
                part rearWheel : Wheel[2] ordered;
            }
        }
        part vehicle1_c1 : Vehicle {
            attribute mass :>> Vehicle::mass = 2000 [kg] {
            }
            part frontAxleAssembly : AxleAssembly {
                part frontAxle : FrontAxle {}
                part frontWheel : Wheel[2] ordered {}
                part frontWheel_1 :> frontWheel = frontWheel#(1);
                part frontWheel_2 :> frontWheel = frontWheel#(2);
            }
            part rearAxleAssembly : AxleAssembly {
                part rearAxle : Axle;
                part rearWheel : Wheel[2] ordered;
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
    (reference r0 (scope relative) (span (offset 42) (line 2) (column 17) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 42) (line 2) (column 17) (len 2))) (segment 1 (token "kg") (name "kg") (separator colon-colon) (span (offset 46) (line 2) (column 21) (len 2)))))
    (reference r1 (scope relative) (span (offset 118) (line 6) (column 22) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 118) (line 6) (column 22) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 123) (line 6) (column 27) (len 4)))))
    (reference r2 (scope relative) (span (offset 432) (line 18) (column 22) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 432) (line 18) (column 22) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 437) (line 18) (column 27) (len 4)))))
    (reference r3 (scope relative) (span (offset 508) (line 21) (column 29) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 508) (line 21) (column 29) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 522) (line 21) (column 43) (len 4)))))
    (reference r4 (scope relative) (span (offset 592) (line 27) (column 18) (len 11)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 592) (line 27) (column 18) (len 11)))))
    (reference r5 (scope relative) (span (offset 742) (line 34) (column 18) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 742) (line 34) (column 18) (len 7)))))
    (reference r6 (scope relative) (span (offset 856) (line 39) (column 29) (len 13)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 856) (line 39) (column 29) (len 7))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 865) (line 39) (column 38) (len 4)))))
    (reference r7 (scope relative) (span (offset 1808) (line 75) (column 21) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 1808) (line 75) (column 21) (len 7)))))
    (reference r8 (scope relative) (span (offset 2012) (line 82) (column 29) (len 13)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 2012) (line 82) (column 29) (len 7))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 2021) (line 82) (column 38) (len 4)))))
  )
  (root (package (name "1a-Parts Tree") (body brace (import (target (span (span (offset 42) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (package (name "Definitions") (body brace (part-def (name "Vehicle") (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (element-count 1))))) (part-def (name "AxleAssembly") (body semicolon)) (part-def (name "Axle") (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "FrontAxle") (body brace (attribute-usage (declaration-name "steeringAngle") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Wheel") (body semicolon)))) (package (name "Usages") (body brace (import (target (span (span (offset 592) (line 27) (column 18) (len 14))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 603) (line 27) (column 29) (len 3))) (separator (span (offset 603) (line 27) (column 29) (len 2))) (marker (span (offset 605) (line 27) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (declaration-name "vehicle1") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 872) (line 39) (column 45) (len 9)) (literal-with-unit (value (expression (span (offset 872) (line 39) (column 45) (len 4)) (integer 1750))) (unit (expression (span (offset 878) (line 39) (column 51) (len 2)) (bracket (expression (span (offset 878) (line 39) (column 51) (len 2)) (unit "kg")))))))))) (body brace (element-count 0))) (part-usage) (part-usage))) (part-usage (declaration-name "vehicle1_c1") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2028) (line 82) (column 45) (len 9)) (literal-with-unit (value (expression (span (offset 2028) (line 82) (column 45) (len 4)) (integer 2000))) (unit (expression (span (offset 2034) (line 82) (column 51) (len 2)) (bracket (expression (span (offset 2034) (line 82) (column 51) (len 2)) (unit "kg")))))))))) (body brace (element-count 0))) (part-usage) (part-usage))))))))
)
~~~
