# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Requirements): HSUVRequirements"))
~~~
# SOURCE
~~~sysml
package HSUVRequirements {
	private import Requirements::*;
	
	requirement <'UR1.1'> Load: FunctionalRequirementCheck {
		// The following requirements are composite sub-requirements.
		requirement Passengers;
		requirement FuelCapacity;
		requirement Cargo;
	}
	
	requirement <'UR1.2'> EcoFriendliness: PerformanceRequirementCheck {
		requirement <'URI1.2.1'> Emissions: PerformanceRequirementCheck {
			/* The car shall meet 2010 Kyoto Accord emissions standards. */
		}
	}
	
	requirement <'UR1.3'> Performance: PerformanceRequirementCheck {
		requirement Acceleration;
		requirement <'UR1.3.1'> FuelEconomy: PerformanceRequirementCheck {
			/* User shall obtain fuel economy better than that provided by
			 * 95% of cars built in 2004.
			 */
		}
		requirement Braking;
		requirement Range;
		requirement Power;
	}
	
	requirement <'UR1.4'> Ergonomics;
	
	// Syntactically, should this be explicitly marked as a "group"?
	requirement HybridSUVSpec {		
		// The following requirements are required by reference.
		require Load;
		require EcoFriendliness;
		require Performance;
		require Ergonomics;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "hsuvrequirements.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package HSUVRequirements {
    private import Requirements::*;
    requirement <'UR1.1'> Load : FunctionalRequirementCheck {
        requirement Passengers;
        requirement FuelCapacity;
        requirement Cargo;
    }
    requirement <'UR1.2'> EcoFriendliness : PerformanceRequirementCheck {
        requirement <'URI1.2.1'> Emissions : PerformanceRequirementCheck {
            /* The car shall meet 2010 Kyoto Accord emissions standards. */
        }
    }
    requirement <'UR1.3'> Performance : PerformanceRequirementCheck {
        requirement Acceleration;
        requirement <'UR1.3.1'> FuelEconomy : PerformanceRequirementCheck {
            /* User shall obtain fuel economy better than that provided by
			 * 95% of cars built in 2004.
			 */
        }
        requirement Braking;
        requirement Range;
        requirement Power;
    }
    requirement <'UR1.4'> Ergonomics;
    requirement HybridSUVSpec {
        require Load;
        require EcoFriendliness;
        require Performance;
        require Ergonomics;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 43) (line 2) (column 17) (len 12)) (segments (segment 0 (token "Requirements") (name "Requirements") (separator none) (span (offset 43) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 91) (line 4) (column 30) (len 26)) (segments (segment 0 (token "FunctionalRequirementCheck") (name "FunctionalRequirementCheck") (separator none) (span (offset 91) (line 4) (column 30) (len 26)))))
    (reference r2 (scope relative) (span (offset 304) (line 11) (column 41) (len 27)) (segments (segment 0 (token "PerformanceRequirementCheck") (name "PerformanceRequirementCheck") (separator none) (span (offset 304) (line 11) (column 41) (len 27)))))
    (reference r3 (scope relative) (span (offset 372) (line 12) (column 39) (len 27)) (segments (segment 0 (token "PerformanceRequirementCheck") (name "PerformanceRequirementCheck") (separator none) (span (offset 372) (line 12) (column 39) (len 27)))))
    (reference r4 (scope relative) (span (offset 514) (line 17) (column 37) (len 27)) (segments (segment 0 (token "PerformanceRequirementCheck") (name "PerformanceRequirementCheck") (separator none) (span (offset 514) (line 17) (column 37) (len 27)))))
    (reference r5 (scope relative) (span (offset 611) (line 19) (column 40) (len 27)) (segments (segment 0 (token "PerformanceRequirementCheck") (name "PerformanceRequirementCheck") (separator none) (span (offset 611) (line 19) (column 40) (len 27)))))
    (reference r6 (scope relative) (span (offset 1024) (line 34) (column 11) (len 4)) (segments (segment 0 (token "Load") (name "Load") (separator none) (span (offset 1024) (line 34) (column 11) (len 4)))))
    (reference r7 (scope relative) (span (offset 1040) (line 35) (column 11) (len 15)) (segments (segment 0 (token "EcoFriendliness") (name "EcoFriendliness") (separator none) (span (offset 1040) (line 35) (column 11) (len 15)))))
    (reference r8 (scope relative) (span (offset 1067) (line 36) (column 11) (len 11)) (segments (segment 0 (token "Performance") (name "Performance") (separator none) (span (offset 1067) (line 36) (column 11) (len 11)))))
    (reference r9 (scope relative) (span (offset 1090) (line 37) (column 11) (len 10)) (segments (segment 0 (token "Ergonomics") (name "Ergonomics") (separator none) (span (offset 1090) (line 37) (column 11) (len 10)))))
  )
  (root (package (visibility none) (name "HSUVRequirements") (body brace (import (target (span (span (offset 43) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 55) (line 2) (column 29) (len 3))) (separator (span (offset 55) (line 2) (column 29) (len 2))) (marker (span (offset 57) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (requirement-usage (name "Load") (short-name "UR1.1") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (abstract false) (variation false) none (subsets none) (references none) (body brace (requirement-usage (name "Passengers") (short-name none) (typing none) (abstract false) (variation false) none (subsets none) (references none) (body semicolon)) (requirement-usage (name "FuelCapacity") (short-name none) (typing none) (abstract false) (variation false) none (subsets none) (references none) (body semicolon)) (requirement-usage (name "Cargo") (short-name none) (typing none) (abstract false) (variation false) none (subsets none) (references none) (body semicolon)))) (requirement-usage (name "EcoFriendliness") (short-name "UR1.2") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (abstract false) (variation false) none (subsets none) (references none) (body brace (requirement-usage (name "Emissions") (short-name "URI1.2.1") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (abstract false) (variation false) none (subsets none) (references none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 407) (line 13) (column 6) (len 59)) (normalized "The car shall meet 2010 Kyoto Accord emissions standards. "))))))) (requirement-usage (name "Performance") (short-name "UR1.3") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (abstract false) (variation false) none (subsets none) (references none) (body brace (requirement-usage (name "Acceleration") (short-name none) (typing none) (abstract false) (variation false) none (subsets none) (references none) (body semicolon)) (requirement-usage (name "FuelEconomy") (short-name "UR1.3.1") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (abstract false) (variation false) none (subsets none) (references none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 646) (line 20) (column 6) (len 98)) (normalized "User shall obtain fuel economy better than that provided by\n95% of cars built in 2004.\n"))))) (requirement-usage (name "Braking") (short-name none) (typing none) (abstract false) (variation false) none (subsets none) (references none) (body semicolon)) (requirement-usage (name "Range") (short-name none) (typing none) (abstract false) (variation false) none (subsets none) (references none) (body semicolon)) (requirement-usage (name "Power") (short-name none) (typing none) (abstract false) (variation false) none (subsets none) (references none) (body semicolon)))) (requirement-usage (name "Ergonomics") (short-name "UR1.4") (typing none) (abstract false) (variation false) none (subsets none) (references none) (body semicolon)) (requirement-usage (name "HybridSUVSpec") (short-name none) (typing none) (abstract false) (variation false) none (subsets none) (references none) (body brace (require-constraint (kind require) (constraint-keyword false) (name none) (target (ref r6)) (specializations) (value none) (body semicolon)) (require-constraint (kind require) (constraint-keyword false) (name none) (target (ref r7)) (specializations) (value none) (body semicolon)) (require-constraint (kind require) (constraint-keyword false) (name none) (target (ref r8)) (specializations) (value none) (body semicolon)) (require-constraint (kind require) (constraint-keyword false) (name none) (target (ref r9)) (specializations) (value none) (body semicolon)))))))
)
~~~
