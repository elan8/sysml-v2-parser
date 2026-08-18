# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Mass Roll-up): Vehicles"))
~~~
# SOURCE
~~~sysml
package VehicleMasses {
	private import ScalarValues::*;
	private import MassRollup::*;
	
	part def CarPart :> MassedThing {			
		attribute serialNumber: String;
	}
	
	part car: CarPart :> compositeThing {	
		attribute vin redefines serialNumber;
		
		part carParts: CarPart[*] redefines subcomponents;
		
		part engine :> simpleThing, carParts {
			//...
		}
		
		part transmission :> simpleThing, carParts {
			//...
		}
	}

	// Example usage
	private import SI::*;	
	part c :> car {
		redefines mass = 1000 [kg];
		part redefines engine {
			redefines mass = 100 [kg];
		}
		
		part redefines transmission {
			redefines mass = 50 [kg];
		}	
	}
	
	// c.totalMass --> 1150.0 [kg]
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicles.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package VehicleMasses {
    private import ScalarValues::*;
    private import MassRollup::*;
    part def CarPart :> MassedThing {
        attribute serialNumber : String;
    }
    part car : CarPart :> compositeThing {
        attribute vin :>> serialNumber;
        part carParts : CarPart[*] :>> subcomponents;
        part engine :> simpleThing, carParts {}
        part transmission :> simpleThing, carParts {}
    }
    private import SI::*;
    part c :> car {
        attribute :>> mass = 1000 [kg];
        part :>> engine {
            attribute :>> mass = 100 [kg];
        }
        part :>> transmission {
            attribute :>> mass = 50 [kg];
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 40) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 40) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 73) (line 3) (column 17) (len 10)) (segments (segment 0 (token "MassRollup") (name "MassRollup") (separator none) (span (offset 73) (line 3) (column 17) (len 10)))))
    (reference r2 (scope relative) (span (offset 154) (line 6) (column 27) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 154) (line 6) (column 27) (len 6)))))
    (reference r3 (scope relative) (span (offset 178) (line 9) (column 12) (len 7)) (segments (segment 0 (token "CarPart") (name "CarPart") (separator none) (span (offset 178) (line 9) (column 12) (len 7)))))
    (reference r4 (scope relative) (span (offset 233) (line 10) (column 27) (len 12)) (segments (segment 0 (token "serialNumber") (name "serialNumber") (separator none) (span (offset 233) (line 10) (column 27) (len 12)))))
    (reference r5 (scope relative) (span (offset 461) (line 24) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 461) (line 24) (column 17) (len 2)))))
    (reference r6 (scope relative) (span (offset 498) (line 26) (column 13) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 498) (line 26) (column 13) (len 4)))))
  )
  (root (package (name "VehicleMasses") (body brace (import (target (span (span (offset 40) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 52) (line 2) (column 29) (len 3))) (separator (span (offset 52) (line 2) (column 29) (len 2))) (marker (span (offset 54) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 73) (line 3) (column 17) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 83) (line 3) (column 27) (len 3))) (separator (span (offset 83) (line 3) (column 27) (len 2))) (marker (span (offset 85) (line 3) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "CarPart") (body brace (attribute-usage (declaration-name "serialNumber") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (declaration-name "car") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (attribute-usage (declaration-name "vin") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage) (part-usage) (part-usage))) (import (target (span (span (offset 461) (line 24) (column 17) (len 5))) (all none) (ref r5) (shape (namespace (wildcard-suffix (span (span (offset 463) (line 24) (column 19) (len 3))) (separator (span (offset 463) (line 24) (column 19) (len 2))) (marker (span (offset 465) (line 24) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (declaration-name "c") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 505) (line 26) (column 20) (len 9)) (literal-with-unit (value (expression (span (offset 505) (line 26) (column 20) (len 4)) (integer 1000))) (unit (expression (span (offset 511) (line 26) (column 26) (len 2)) (bracket (expression (span (offset 511) (line 26) (column 26) (len 2)) (unit "kg")))))))))) (body semicolon)) (part-usage) (part-usage))))))
)
~~~
