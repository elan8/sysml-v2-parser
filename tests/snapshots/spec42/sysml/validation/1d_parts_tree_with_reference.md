# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (01-Parts Tree): 1d-Parts Tree with Reference"))
~~~
# SOURCE
~~~sysml
package '1d-Parts Tree with Reference' {
	
	package Definitions {
		part def Vehicle;
		part def Trailer;
		part def TrailerHitch;
		part def HitchBall;
		part def TrailerCoupler;
	}
	
	package Usages {
		private import Definitions::*;
		
		part vehicle_trailer_system {
			
			part vehicle1_c1: Vehicle {
				ref hitchBall : HitchBall {
					/*
					 * 'vehicle1_c1'::'hitchBall' is a reference property that
					 * references a hitch ball that is not part of this vehicle. 
					 * If 'vehicle1_c1' is removed or destroyed, this does not
					 * effect the hitchBall referenced here.
					 */
				}
			}
			
			bind vehicle1_c1.hitchBall = trailerHitch.hitchBall {
				/*
				 * This is a binding connector between the 'hitchBall' in 'vehicle1_c1'
				 * and the 'hitchBall' in 'trailerHitch'.
				 */			
			}
			
			part trailerHitch: TrailerHitch {				
				part hitchBall: HitchBall;
				part trailerCoupler: TrailerCoupler;
			}
			
			part trailer1: Trailer {
				ref trailerCoupler : TrailerCoupler = trailerHitch.trailerCoupler {
					/*
					 * This is a shorthand for a binding connector between the
					 * 'trailerCoupler' here and the 'trailerCoupler' in 'trailerHitch'.
					 * The binding connector is now contained within the 'trailer1'
					 * part, though, rather than being at the system level. 
					 */
				}
			}
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "1d_parts_tree_with_reference.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '1d-Parts Tree with Reference' {
    package Definitions {
        part def Vehicle;
        part def Trailer;
        part def TrailerHitch;
        part def HitchBall;
        part def TrailerCoupler;
    }
    package Usages {
        private import Definitions::*;
        part vehicle_trailer_system {
            part vehicle1_c1 : Vehicle {
                ref hitchBall : HitchBall {}
            }
            bind vehicle1_c1.hitchBall = trailerHitch.hitchBall {}
            part trailerHitch : TrailerHitch {
                part hitchBall : HitchBall;
                part trailerCoupler : TrailerCoupler;
            }
            part trailer1 : Trailer {
                ref trailerCoupler : TrailerCoupler = trailerHitch.trailerCoupler {}
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 220) (line 12) (column 18) (len 11)) (segments (segment 0 (token "Definitions") (name "Definitions") (separator none) (span (offset 220) (line 12) (column 18) (len 11)))))
  )
  (root (package (name "1d-Parts Tree with Reference") (body brace (package (name "Definitions") (body brace (part-def (name "Vehicle") (body semicolon)) (part-def (name "Trailer") (body semicolon)) (part-def (name "TrailerHitch") (body semicolon)) (part-def (name "HitchBall") (body semicolon)) (part-def (name "TrailerCoupler") (body semicolon)))) (package (name "Usages") (body brace (import (target (span (span (offset 220) (line 12) (column 18) (len 14))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 231) (line 12) (column 29) (len 3))) (separator (span (offset 231) (line 12) (column 29) (len 2))) (marker (span (offset 233) (line 12) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (declaration-name "vehicle_trailer_system") (typing none) (body brace (part-usage) (bind) (part-usage) (part-usage))))))))
)
~~~
