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
                part frontAxle_c1 : FrontAxle :>> frontAxle {}
                part frontWheel_1 :> frontWheel = frontWheel#(1);
                part frontWheel_2 :> frontWheel = frontWheel#(2);
            }
            part rearAxleAssembly_c1 :>> rearAxleAssembly {
                part rearAxle_c1 :>> rearAxle {}
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
    (reference r5 (scope relative) (span (offset 412) (line 21) (column 18) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 412) (line 21) (column 18) (len 7)))))
    (reference r6 (scope relative) (span (offset 450) (line 22) (column 29) (len 13)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 450) (line 22) (column 29) (len 7))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 459) (line 22) (column 38) (len 4)))))
    (reference r7 (scope relative) (span (offset 610) (line 29) (column 28) (len 12)) (segments (segment 0 (token "AxleAssembly") (name "AxleAssembly") (separator none) (span (offset 610) (line 29) (column 28) (len 12)))))
    (reference r8 (scope relative) (span (offset 645) (line 30) (column 21) (len 4)) (segments (segment 0 (token "Axle") (name "Axle") (separator none) (span (offset 645) (line 30) (column 21) (len 4)))))
    (reference r9 (scope relative) (span (offset 675) (line 31) (column 22) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 675) (line 31) (column 22) (len 5)))))
    (reference r10 (scope relative) (span (offset 726) (line 33) (column 27) (len 12)) (segments (segment 0 (token "AxleAssembly") (name "AxleAssembly") (separator none) (span (offset 726) (line 33) (column 27) (len 12)))))
    (reference r11 (scope relative) (span (offset 760) (line 34) (column 20) (len 4)) (segments (segment 0 (token "Axle") (name "Axle") (separator none) (span (offset 760) (line 34) (column 20) (len 4)))))
    (reference r12 (scope relative) (span (offset 786) (line 35) (column 21) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 786) (line 35) (column 21) (len 5)))))
    (reference r13 (scope relative) (span (offset 839) (line 39) (column 23) (len 8)) (segments (segment 0 (token "vehicle1") (name "vehicle1") (separator none) (span (offset 839) (line 39) (column 23) (len 8)))))
    (reference r14 (scope relative) (span (offset 1082) (line 46) (column 29) (len 14)) (segments (segment 0 (token "vehicle1") (name "vehicle1") (separator none) (span (offset 1082) (line 46) (column 29) (len 8))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 1092) (line 46) (column 39) (len 4)))))
    (reference r15 (scope relative) (span (offset 1287) (line 53) (column 40) (len 17)) (segments (segment 0 (token "frontAxleAssembly") (name "frontAxleAssembly") (separator none) (span (offset 1287) (line 53) (column 40) (len 17)))))
    (reference r16 (scope relative) (span (offset 1330) (line 54) (column 24) (len 9)) (segments (segment 0 (token "FrontAxle") (name "FrontAxle") (separator none) (span (offset 1330) (line 54) (column 24) (len 9)))))
    (reference r17 (scope relative) (span (offset 1350) (line 54) (column 44) (len 9)) (segments (segment 0 (token "frontAxle") (name "frontAxle") (separator none) (span (offset 1350) (line 54) (column 44) (len 9)))))
    (reference r18 (scope relative) (span (offset 1719) (line 67) (column 31) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 1719) (line 67) (column 31) (len 10)))))
    (reference r19 (scope relative) (span (offset 1732) (line 67) (column 44) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 1732) (line 67) (column 44) (len 10)))))
    (reference r20 (scope relative) (span (offset 1778) (line 68) (column 31) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 1778) (line 68) (column 31) (len 10)))))
    (reference r21 (scope relative) (span (offset 1791) (line 68) (column 44) (len 10)) (segments (segment 0 (token "frontWheel") (name "frontWheel") (separator none) (span (offset 1791) (line 68) (column 44) (len 10)))))
    (reference r22 (scope relative) (span (offset 1855) (line 71) (column 39) (len 16)) (segments (segment 0 (token "rearAxleAssembly") (name "rearAxleAssembly") (separator none) (span (offset 1855) (line 71) (column 39) (len 16)))))
    (reference r23 (scope relative) (span (offset 1905) (line 72) (column 32) (len 8)) (segments (segment 0 (token "rearAxle") (name "rearAxle") (separator none) (span (offset 1905) (line 72) (column 32) (len 8)))))
    (reference r24 (scope relative) (span (offset 2130) (line 80) (column 30) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 2130) (line 80) (column 30) (len 9)))))
    (reference r25 (scope relative) (span (offset 2142) (line 80) (column 42) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 2142) (line 80) (column 42) (len 9)))))
    (reference r26 (scope relative) (span (offset 2186) (line 81) (column 30) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 2186) (line 81) (column 30) (len 9)))))
    (reference r27 (scope relative) (span (offset 2198) (line 81) (column 42) (len 9)) (segments (segment 0 (token "rearWheel") (name "rearWheel") (separator none) (span (offset 2198) (line 81) (column 42) (len 9)))))
  )
  (root (package (name "1c-Parts Tree Redefinition") (body brace (import (target (span (span (offset 55) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (package (name "Definitions") (body brace (part-def (name "Vehicle") (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "AxleAssembly") (body semicolon)) (part-def (name "Axle") (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "FrontAxle") (body brace (attribute-usage (declaration-name "steeringAngle") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Wheel") (body semicolon)))) (package (name "Usages") (body brace (import (target (span (span (offset 376) (line 19) (column 18) (len 14))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 387) (line 19) (column 29) (len 3))) (separator (span (offset 387) (line 19) (column 29) (len 2))) (marker (span (offset 389) (line 19) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 474) (line 22) (column 53) (len 9)) (literal-with-unit (value (expression (span (offset 474) (line 22) (column 53) (len 4)) (integer 1750))) (unit (expression (span (offset 480) (line 22) (column 59) (len 2)) (bracket (expression (span (offset 480) (line 22) (column 59) (len 2)) (unit "kg")))))))))) (body brace (element-count 1))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxleAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity (lower (expression (span (offset 681) (line 31) (column 28) (len 1)) (integer 2))) (upper (expression (span (offset 681) (line 31) (column 28) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered true) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxleAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity (lower (expression (span (offset 792) (line 35) (column 27) (len 1)) (integer 2))) (upper (expression (span (offset 792) (line 35) (column 27) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered true) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r13))) (value none))) (redefines none) (value none) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r14)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1099) (line 46) (column 46) (len 9)) (literal-with-unit (value (expression (span (offset 1099) (line 46) (column 46) (len 4)) (integer 2000))) (unit (expression (span (offset 1105) (line 46) (column 52) (len 2)) (bracket (expression (span (offset 1105) (line 46) (column 52) (len 2)) (unit "kg")))))))))) (body brace (element-count 0))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxleAssembly_c1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxle_c1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (value none) (body brace)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontWheel_1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r18))) (value (expression (span (offset 1732) (line 67) (column 44) (len 14)) (index (base (expression (span (offset 1732) (line 67) (column 44) (len 10)) (ref r19))) (index (expression (span (offset 1744) (line 67) (column 56) (len 1)) (integer 1)))))))) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontWheel_2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r20))) (value (expression (span (offset 1791) (line 68) (column 44) (len 14)) (index (base (expression (span (offset 1791) (line 68) (column 44) (len 10)) (ref r21))) (index (expression (span (offset 1803) (line 68) (column 56) (len 1)) (integer 2)))))))) (redefines none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxleAssembly_c1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxle_c1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r23)))) (value none) (body brace)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearWheel_1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r24))) (value (expression (span (offset 2142) (line 80) (column 42) (len 13)) (index (base (expression (span (offset 2142) (line 80) (column 42) (len 9)) (ref r25))) (index (expression (span (offset 2153) (line 80) (column 53) (len 1)) (integer 1)))))))) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearWheel_2") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r26))) (value (expression (span (offset 2198) (line 81) (column 42) (len 13)) (index (base (expression (span (offset 2198) (line 81) (column 42) (len 9)) (ref r27))) (index (expression (span (offset 2209) (line 81) (column 53) (len 1)) (integer 2)))))))) (redefines none) (value none) (body semicolon)))))))))))
)
~~~
