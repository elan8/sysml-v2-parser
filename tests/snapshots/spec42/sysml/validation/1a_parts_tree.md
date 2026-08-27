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
        private import Definitions::* {
            /*
			 * A "private" private import makes the imported names private to the
			 * imported package.
			 */
        }
        part vehicle1 : Vehicle {
            /*
			 * 'vehicle1' is a package-owned part of type Vehicle.
			 */
            attribute mass redefines Vehicle::mass = 1750[kg] {
                /*
				 * This redefines the 'mass' attribute property from 'Vehicle' to 
				 * give it a fixed attribute.
				 */
            }
            part frontAxleAssembly : AxleAssembly {
                /*
				 * 'frontAxleAssembly' is a nested part of part 'vehicle1'.
				 * It is a composite part of the containing part.
				 * 
				 * (And similarly for 'rearAxleAssembly'.)
				 */
                part frontAxle : Axle;
                part frontWheel : Wheel[2] ordered {
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
            part rearAxleAssembly : AxleAssembly {
                part rearAxle : Axle;
                part rearWheel : Wheel[2] ordered;
            }
        }
        part vehicle1_c1 : Vehicle {
            /*
			 * 'vehicle1_c1' is a modified copy of 'vehicle1'. There is no
			 * connection between this copy and the original version in the
			 * model.
			 */
            attribute mass redefines Vehicle::mass = 2000[kg] {
                /*
				 * The mass attribute has been modified.
				 */
            }
            part frontAxleAssembly : AxleAssembly {
                part frontAxle : FrontAxle {
                    /*
					 * The part 'frontAxle' has been modified to have type 'FrontAxle'.
					 */
                }
                part frontWheel : Wheel[2] ordered {
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
            part rearAxleAssembly : AxleAssembly {
                /*
				 * 'rearAxleAssembly' has also been modified to add subsetting parts
				 * for 'rearWheel'.
				 */
                part rearAxle : Axle;
                part rearWheel : Wheel[2] ordered;
                part rearWheel_1 subsets rearWheel = rearWheel#(1);
                part rearWheel_2 subsets rearWheel = rearWheel#(2);
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
    (reference r7 (scope relative) (span (offset 878) (line 39) (column 51) (len 2)) (segments (segment 0 (token "kg") (name "kg") (separator none) (span (offset 878) (line 39) (column 51) (len 2)))))
    (reference r8 (scope relative) (span (offset 1040) (line 46) (column 28) (len 12)) (segments (segment 0 (token "AxleAssembly") (name "AxleAssembly") (separator none) (span (offset 1040) (line 46) (column 28) (len 12)))))
    (reference r9 (scope relative) (span (offset 1267) (line 54) (column 21) (len 4)) (segments (segment 0 (token "Axle") (name "Axle") (separator none) (span (offset 1267) (line 54) (column 21) (len 4)))))
    (reference r10 (scope relative) (span (offset 1299) (line 56) (column 22) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 1299) (line 56) (column 22) (len 5)))))
    (reference r11 (scope relative) (span (offset 1695) (line 68) (column 27) (len 12)) (segments (segment 0 (token "AxleAssembly") (name "AxleAssembly") (separator none) (span (offset 1695) (line 68) (column 27) (len 12)))))
    (reference r12 (scope relative) (span (offset 1729) (line 69) (column 20) (len 4)) (segments (segment 0 (token "Axle") (name "Axle") (separator none) (span (offset 1729) (line 69) (column 20) (len 4)))))
    (reference r13 (scope relative) (span (offset 1755) (line 70) (column 21) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 1755) (line 70) (column 21) (len 5)))))
    (reference r14 (scope relative) (span (offset 1808) (line 75) (column 21) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 1808) (line 75) (column 21) (len 7)))))
    (reference r15 (scope relative) (span (offset 2012) (line 82) (column 29) (len 13)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 2012) (line 82) (column 29) (len 7))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 2021) (line 82) (column 38) (len 4)))))
    (reference r16 (scope relative) (span (offset 2034) (line 82) (column 51) (len 2)) (segments (segment 0 (token "kg") (name "kg") (separator none) (span (offset 2034) (line 82) (column 51) (len 2)))))
    (reference r17 (scope relative) (span (offset 2134) (line 88) (column 28) (len 12)) (segments (segment 0 (token "AxleAssembly") (name "AxleAssembly") (separator none) (span (offset 2134) (line 88) (column 28) (len 12)))))
    (reference r18 (scope relative) (span (offset 2174) (line 90) (column 21) (len 9)) (segments (segment 0 (token "FrontAxle") (name "FrontAxle") (separator none) (span (offset 2174) (line 90) (column 21) (len 9)))))
    (reference r19 (scope relative) (span (offset 2308) (line 96) (column 22) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 2308) (line 96) (column 22) (len 5)))))
    (reference r20 (scope relative) (span (offset 2652) (line 104) (column 31) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 2652) (line 104) (column 31) (len 10)))))
    (reference r21 (scope relative) (span (offset 2665) (line 104) (column 44) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 2665) (line 104) (column 44) (len 10)))))
    (reference r22 (scope relative) (span (offset 2711) (line 105) (column 31) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 2711) (line 105) (column 31) (len 10)))))
    (reference r23 (scope relative) (span (offset 2724) (line 105) (column 44) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 2724) (line 105) (column 44) (len 10)))))
    (reference r24 (scope relative) (span (offset 2775) (line 108) (column 27) (len 12)) (segments (segment 0 (token "AxleAssembly") (name "AxleAssembly") (separator none) (span (offset 2775) (line 108) (column 27) (len 12)))))
    (reference r25 (scope relative) (span (offset 2928) (line 114) (column 20) (len 4)) (segments (segment 0 (token "Axle") (name "Axle") (separator none) (span (offset 2928) (line 114) (column 20) (len 4)))))
    (reference r26 (scope relative) (span (offset 2959) (line 116) (column 21) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 2959) (line 116) (column 21) (len 5)))))
    (reference r27 (scope relative) (span (offset 3006) (line 117) (column 30) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 3006) (line 117) (column 30) (len 9)))))
    (reference r28 (scope relative) (span (offset 3018) (line 117) (column 42) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 3018) (line 117) (column 42) (len 9)))))
    (reference r29 (scope relative) (span (offset 3062) (line 118) (column 30) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 3062) (line 118) (column 30) (len 9)))))
    (reference r30 (scope relative) (span (offset 3074) (line 118) (column 42) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 3074) (line 118) (column 42) (len 9)))))
  )
  (root (package (name "1a-Parts Tree") (body brace (import (target (span (span (offset 42) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (package (name "Definitions") (body brace (part-def (name "Vehicle") (modifiers) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 142) (line 8) (column 6) (len 209)) (normalized "The 'mass' attribute property is declared here to be a \nspecialization (subset) of the general 'mass' quantity \nfrom the 'ISQ' (International System of Quantities) \nlibrary model.\n"))))))) (part-def (name "AxleAssembly") (modifiers) (body semicolon)) (part-def (name "Axle") (modifiers) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "FrontAxle") (modifiers) (body brace (attribute-usage (declaration-name "steeringAngle") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Wheel") (modifiers) (body semicolon)))) (package (name "Usages") (body brace (import (target (span (span (offset 592) (line 27) (column 18) (len 14))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 603) (line 27) (column 29) (len 3))) (separator (span (offset 603) (line 27) (column 29) (len 2))) (marker (span (offset 605) (line 27) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 757) (line 35) (column 6) (len 63)) (normalized "'vehicle1' is a package-owned part of type Vehicle.\n"))) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 872) (line 39) (column 45) (len 9)) (bracket (base (expression (span (offset 872) (line 39) (column 45) (len 4)) (integer 1750))) (operands (sequence-list (element first (expression (span (offset 878) (line 39) (column 51) (len 2)) (ref r7)))))))))) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 890) (line 40) (column 7) (len 111)) (normalized "This redefines the 'mass' attribute property from 'Vehicle' to \ngive it a fixed attribute.\n"))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxleAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1061) (line 47) (column 7) (len 179)) (normalized "'frontAxleAssembly' is a nested part of part 'vehicle1'.\nIt is a composite part of the containing part.\n\n(And similarly for 'rearAxleAssembly'.)\n"))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity (lower (expression (span (offset 1305) (line 56) (column 28) (len 1)) (integer 2))) (upper (expression (span (offset 1305) (line 56) (column 28) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1325) (line 57) (column 8) (len 326)) (normalized "'frontWheel' is a nested part of type 'Wheel' with\nmultiplicity \"2\". This means that this axle assembly\nmust have exactly two wheels. However, there is still\nonly one 'frontWheel' part. The part is \"ordered\",\nso that the first wheel can be distinguished from the\nsecond.\n"))))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxleAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity (lower (expression (span (offset 1761) (line 70) (column 27) (len 1)) (integer 2))) (upper (expression (span (offset 1761) (line 70) (column 27) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1823) (line 76) (column 6) (len 151)) (normalized "'vehicle1_c1' is a modified copy of 'vehicle1'. There is no\nconnection between this copy and the original version in the\nmodel.\n"))) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2028) (line 82) (column 45) (len 9)) (bracket (base (expression (span (offset 2028) (line 82) (column 45) (len 4)) (integer 2000))) (operands (sequence-list (element first (expression (span (offset 2034) (line 82) (column 51) (len 2)) (ref r16)))))))))) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2046) (line 83) (column 7) (len 51)) (normalized "The mass attribute has been modified.\n"))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxleAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2193) (line 91) (column 8) (len 80)) (normalized "The part 'frontAxle' has been modified to have type 'FrontAxle'.\n"))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity (lower (expression (span (offset 2314) (line 96) (column 28) (len 1)) (integer 2))) (upper (expression (span (offset 2314) (line 96) (column 28) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2334) (line 97) (column 8) (len 279)) (normalized "The parts 'frontWheel_1' and 'frontWheel_2' have been added\nas subsets of 'frontWheel'. These are separate parts from\n'frontWheel', but essentially provide alternate names for\neach of the two wheels, as given by their defining expressions.\n"))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontWheel_1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r20))) (value (expression (span (offset 2665) (line 104) (column 44) (len 14)) (index (base (expression (span (offset 2665) (line 104) (column 44) (len 10)) (ref r21))) (operands (sequence-list (element first (expression (span (offset 2677) (line 104) (column 56) (len 1)) (integer 1)))))))))) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontWheel_2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r22))) (value (expression (span (offset 2724) (line 105) (column 44) (len 14)) (index (base (expression (span (offset 2724) (line 105) (column 44) (len 10)) (ref r23))) (operands (sequence-list (element first (expression (span (offset 2736) (line 105) (column 56) (len 1)) (integer 2)))))))))) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxleAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2796) (line 109) (column 7) (len 103)) (normalized "'rearAxleAssembly' has also been modified to add subsetting parts\nfor 'rearWheel'.\n"))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity (lower (expression (span (offset 2965) (line 116) (column 27) (len 1)) (integer 2))) (upper (expression (span (offset 2965) (line 116) (column 27) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearWheel_1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r27))) (value (expression (span (offset 3018) (line 117) (column 42) (len 13)) (index (base (expression (span (offset 3018) (line 117) (column 42) (len 9)) (ref r28))) (operands (sequence-list (element first (expression (span (offset 3029) (line 117) (column 53) (len 1)) (integer 1)))))))))) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearWheel_2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r29))) (value (expression (span (offset 3074) (line 118) (column 42) (len 13)) (index (base (expression (span (offset 3074) (line 118) (column 42) (len 9)) (ref r30))) (operands (sequence-list (element first (expression (span (offset 3085) (line 118) (column 53) (len 1)) (integer 2)))))))))) (redefines none) (value none) (body semicolon)))))))))))
)
~~~
