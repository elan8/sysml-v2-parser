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
        attribute vin redefines serialNumber;
        part carParts : CarPart[*] redefines subcomponents;
        part engine :> simpleThing, carParts {}
        part transmission :> simpleThing, carParts {}
    }
    private import SI::*;
    part c :> car {
        attribute :>> mass = 1000[kg];
        part redefines engine {
            attribute :>> mass = 100[kg];
        }
        part redefines transmission {
            attribute :>> mass = 50[kg];
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
    (reference r4 (scope relative) (span (offset 189) (line 9) (column 23) (len 14)) (segments (segment 0 (token "compositeThing") (name "compositeThing") (separator none) (span (offset 189) (line 9) (column 23) (len 14)))))
    (reference r5 (scope relative) (span (offset 233) (line 10) (column 27) (len 12)) (segments (segment 0 (token "serialNumber") (name "serialNumber") (separator none) (span (offset 233) (line 10) (column 27) (len 12)))))
    (reference r6 (scope relative) (span (offset 267) (line 12) (column 18) (len 7)) (segments (segment 0 (token "CarPart") (name "CarPart") (separator none) (span (offset 267) (line 12) (column 18) (len 7)))))
    (reference r7 (scope relative) (span (offset 288) (line 12) (column 39) (len 13)) (segments (segment 0 (token "subcomponents") (name "subcomponents") (separator none) (span (offset 288) (line 12) (column 39) (len 13)))))
    (reference r8 (scope relative) (span (offset 323) (line 14) (column 18) (len 11)) (segments (segment 0 (token "simpleThing") (name "simpleThing") (separator none) (span (offset 323) (line 14) (column 18) (len 11)))))
    (reference r9 (scope relative) (span (offset 336) (line 14) (column 31) (len 8)) (segments (segment 0 (token "carParts") (name "carParts") (separator none) (span (offset 336) (line 14) (column 31) (len 8)))))
    (reference r10 (scope relative) (span (offset 386) (line 18) (column 24) (len 11)) (segments (segment 0 (token "simpleThing") (name "simpleThing") (separator none) (span (offset 386) (line 18) (column 24) (len 11)))))
    (reference r11 (scope relative) (span (offset 399) (line 18) (column 37) (len 8)) (segments (segment 0 (token "carParts") (name "carParts") (separator none) (span (offset 399) (line 18) (column 37) (len 8)))))
    (reference r12 (scope relative) (span (offset 461) (line 24) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 461) (line 24) (column 17) (len 2)))))
    (reference r13 (scope relative) (span (offset 480) (line 25) (column 12) (len 3)) (segments (segment 0 (token "car") (name "car") (separator none) (span (offset 480) (line 25) (column 12) (len 3)))))
    (reference r14 (scope relative) (span (offset 498) (line 26) (column 13) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 498) (line 26) (column 13) (len 4)))))
    (reference r15 (scope relative) (span (offset 511) (line 26) (column 26) (len 2)) (segments (segment 0 (token "kg") (name "kg") (separator none) (span (offset 511) (line 26) (column 26) (len 2)))))
    (reference r16 (scope relative) (span (offset 533) (line 27) (column 18) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 533) (line 27) (column 18) (len 6)))))
    (reference r17 (scope relative) (span (offset 555) (line 28) (column 14) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 555) (line 28) (column 14) (len 4)))))
    (reference r18 (scope relative) (span (offset 567) (line 28) (column 26) (len 2)) (segments (segment 0 (token "kg") (name "kg") (separator none) (span (offset 567) (line 28) (column 26) (len 2)))))
    (reference r19 (scope relative) (span (offset 596) (line 31) (column 18) (len 12)) (segments (segment 0 (token "transmission") (name "transmission") (separator none) (span (offset 596) (line 31) (column 18) (len 12)))))
    (reference r20 (scope relative) (span (offset 624) (line 32) (column 14) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 624) (line 32) (column 14) (len 4)))))
    (reference r21 (scope relative) (span (offset 635) (line 32) (column 25) (len 2)) (segments (segment 0 (token "kg") (name "kg") (separator none) (span (offset 635) (line 32) (column 25) (len 2)))))
  )
  (root (package (name "VehicleMasses") (body brace (import (target (span (span (offset 40) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 52) (line 2) (column 29) (len 3))) (separator (span (offset 52) (line 2) (column 29) (len 2))) (marker (span (offset 54) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 73) (line 3) (column 17) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 83) (line 3) (column 27) (len 3))) (separator (span (offset 83) (line 3) (column 27) (len 2))) (marker (span (offset 85) (line 3) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "CarPart") (modifiers) (body brace (attribute-usage (declaration-name "serialNumber") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "car") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r4))) (value none))) (redefines none) (value none) (body brace (attribute-usage (declaration-name "vin") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "carParts") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r8) (ref r9))) (value none))) (redefines none) (value none) (body brace)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r10) (ref r11))) (value none))) (redefines none) (value none) (body brace)))) (import (target (span (span (offset 461) (line 24) (column 17) (len 5))) (all none) (ref r12) (shape (namespace (wildcard-suffix (span (span (offset 463) (line 24) (column 19) (len 3))) (separator (span (offset 463) (line 24) (column 19) (len 2))) (marker (span (offset 465) (line 24) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r13))) (value none))) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r14)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 505) (line 26) (column 20) (len 9)) (bracket (base (expression (span (offset 505) (line 26) (column 20) (len 4)) (integer 1000))) (operands (sequence-list (element first (expression (span (offset 511) (line 26) (column 26) (len 2)) (ref r15)))))))))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 562) (line 28) (column 21) (len 8)) (bracket (base (expression (span (offset 562) (line 28) (column 21) (len 3)) (integer 100))) (operands (sequence-list (element first (expression (span (offset 567) (line 28) (column 26) (len 2)) (ref r18)))))))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19)))) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 631) (line 32) (column 21) (len 7)) (bracket (base (expression (span (offset 631) (line 32) (column 21) (len 2)) (integer 50))) (operands (sequence-list (element first (expression (span (offset 635) (line 32) (column 25) (len 2)) (ref r21)))))))))) (body semicolon)))))))))
)
~~~
