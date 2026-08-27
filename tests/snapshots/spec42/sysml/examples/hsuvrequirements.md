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
  (references
    (reference r0 (scope relative) (span (offset 43) (line 2) (column 17) (len 12)) (segments (segment 0 (token "Requirements") (name "Requirements") (separator none) (span (offset 43) (line 2) (column 17) (len 12)))))
  )
  (root (package (name "HSUVRequirements") (body brace (import (target (span (span (offset 43) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 55) (line 2) (column 29) (len 3))) (separator (span (offset 55) (line 2) (column 29) (len 2))) (marker (span (offset 57) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (requirement-usage (name "Load") (multiplicity none)) (requirement-usage (name "EcoFriendliness") (multiplicity none)) (requirement-usage (name "Performance") (multiplicity none)) (requirement-usage (name "Ergonomics") (multiplicity none)) (requirement-usage (name "HybridSUVSpec") (multiplicity none)))))
)
~~~
