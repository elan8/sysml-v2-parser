# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 29 (Expressions): Car Mass Rollup Example 2"))
~~~
# SOURCE
~~~sysml
package 'Car Mass Rollup 1' {
	private import ScalarValues::*;
	private import MassRollup2::*;
	
	part def CarPart :> MassedThing {			
		attribute serialNumber: String;
	}
	
	part car: CarPart :> compositeThing {	
		attribute vin :>> serialNumber;
		
		part carParts: CarPart[*] :>> subcomponents;
		
		part engine :> carParts {
			//...
		}
		
		part transmission :> carParts {
			//...
		}
	}

	// Example usage
	
	private import SI::kg;
	part c :> car {
		attribute :>> simpleMass = 1000[kg];
		part :>> engine {
			attribute :>> simpleMass = 100[kg];
		}
		
		part redefines transmission {
			attribute :>> simpleMass = 50[kg];
		}	
	}
	
	// c::totalMass --> 1150.0[kg]
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "29_car_mass_rollup_example_2.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Car Mass Rollup 1' {
    private import ScalarValues::*;
    private import MassRollup2::*;
    part def CarPart :> MassedThing {
        attribute serialNumber : String;
    }
    part car : CarPart :> compositeThing {
        attribute vin :>> serialNumber;
        part carParts : CarPart[*] :>> subcomponents;
        part engine :> carParts {
        }
        part transmission :> carParts {
        }
    }
    private import SI::kg;
    part c :> car {
        attribute :>> simpleMass = 1000 [kg];
        part  :>> engine {
            attribute :>> simpleMass = 100 [kg];
        }
        part  :>> transmission {
            attribute :>> simpleMass = 50 [kg];
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 46) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 46) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 79) (line 3) (column 17) (len 11)) (segments (segment 0 (token "MassRollup2") (name "MassRollup2") (separator none) (span (offset 79) (line 3) (column 17) (len 11)))))
    (reference r2 (scope relative) (span (offset 161) (line 6) (column 27) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 161) (line 6) (column 27) (len 6)))))
    (reference r3 (scope relative) (span (offset 432) (line 25) (column 17) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 432) (line 25) (column 17) (len 2))) (segment 1 (token "kg") (name "kg") (separator colon-colon) (span (offset 436) (line 25) (column 21) (len 2)))))
  )
  (root (package (name "Car Mass Rollup 1") (body brace (import (target (span (span (offset 46) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 58) (line 2) (column 29) (len 3))) (separator (span (offset 58) (line 2) (column 29) (len 2))) (marker (span (offset 60) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 79) (line 3) (column 17) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 90) (line 3) (column 28) (len 3))) (separator (span (offset 90) (line 3) (column 28) (len 2))) (marker (span (offset 92) (line 3) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "CarPart") (body brace (attribute-usage (declaration-name "serialNumber") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage) (import (target (span (span (offset 432) (line 25) (column 17) (len 6))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (part-usage))))
)
~~~
