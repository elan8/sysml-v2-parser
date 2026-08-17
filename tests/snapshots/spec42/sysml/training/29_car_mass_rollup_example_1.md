# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 29 (Expressions): Car Mass Rollup Example 1"))
~~~
# SOURCE
~~~sysml
package 'Car Mass Rollup Example 1' {
	private import ScalarValues::*;
	private import MassRollup1::*;
	
	part def CarPart :> MassedThing {			
		attribute serialNumber: String;
	}
	
	part car: CarPart :> compositeThing {	
		attribute vin :>> serialNumber;
		
		part carParts: CarPart[*] :>> subcomponents;
		
		part engine :> simpleThing, carParts {
			//...
		}
		
		part transmission :> simpleThing, carParts {
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
  (document "29_car_mass_rollup_example_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Car Mass Rollup Example 1' {
    private import ScalarValues::*;
    private import MassRollup1::*;
    part def CarPart :> MassedThing {
        attribute serialNumber : String;
    }
    part car : CarPart :> compositeThing {
        attribute vin :>> serialNumber;
        part carParts : CarPart[*] :>> subcomponents;
        part engine :> simpleThing, carParts {
        }
        part transmission :> simpleThing, carParts {
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
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 54) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 87) (line 3) (column 17) (len 11)) (segments (segment 0 (token "MassRollup1") (name "MassRollup1") (separator none) (span (offset 87) (line 3) (column 17) (len 11)))))
    (reference r2 (scope relative) (span (offset 169) (line 6) (column 27) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 169) (line 6) (column 27) (len 6)))))
    (reference r3 (scope relative) (span (offset 193) (line 9) (column 12) (len 7)) (segments (segment 0 (token "CarPart") (name "CarPart") (separator none) (span (offset 193) (line 9) (column 12) (len 7)))))
    (reference r4 (scope relative) (span (offset 242) (line 10) (column 21) (len 12)) (segments (segment 0 (token "serialNumber") (name "serialNumber") (separator none) (span (offset 242) (line 10) (column 21) (len 12)))))
    (reference r5 (scope relative) (span (offset 466) (line 25) (column 17) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 466) (line 25) (column 17) (len 2))) (segment 1 (token "kg") (name "kg") (separator colon-colon) (span (offset 470) (line 25) (column 21) (len 2)))))
    (reference r6 (scope relative) (span (offset 507) (line 27) (column 17) (len 10)) (segments (segment 0 (token "simpleMass") (name "simpleMass") (separator none) (span (offset 507) (line 27) (column 17) (len 10)))))
  )
  (root (package (name "Car Mass Rollup Example 1") (body brace (import (target (span (span (offset 54) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 66) (line 2) (column 29) (len 3))) (separator (span (offset 66) (line 2) (column 29) (len 2))) (marker (span (offset 68) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 87) (line 3) (column 17) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 98) (line 3) (column 28) (len 3))) (separator (span (offset 98) (line 3) (column 28) (len 2))) (marker (span (offset 100) (line 3) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "CarPart") (body brace (attribute-usage (declaration-name "serialNumber") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (declaration-name "car") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (body brace (attribute-usage (declaration-name "vin") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage) (part-usage) (part-usage))) (import (target (span (span (offset 466) (line 25) (column 17) (len 6))) (all none) (ref r5) (shape (membership (recursive-suffix none))))) (part-usage (declaration-name "c") (typing none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 520) (line 27) (column 30) (len 8)) (literal-with-unit (value (expression (span (offset 520) (line 27) (column 30) (len 4)) (integer 1000))) (unit (expression (span (offset 525) (line 27) (column 35) (len 2)) (bracket (expression (span (offset 525) (line 27) (column 35) (len 2)) (unit "kg")))))))))) (body semicolon)) (part-usage) (part-usage))))))
)
~~~
