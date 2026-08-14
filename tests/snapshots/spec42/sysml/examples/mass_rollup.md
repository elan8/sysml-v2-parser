# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Mass Roll-up): MassRollup"))
~~~
# SOURCE
~~~sysml
package MassRollup {
	private import NumericalFunctions::*;
	
	part def MassedThing {
		attribute mass :> ISQ::mass; 
		attribute totalMass :> ISQ::mass;
	}
	
	part simpleThing : MassedThing {
		attribute redefines totalMass = mass;
	}
	
	part compositeThing : MassedThing {
		part subcomponents: MassedThing[*];
		
		attribute redefines totalMass default
			mass + sum(subcomponents.totalMass); 
	}
	
	part filteredMassThing :> compositeThing {
		abstract attribute minMass :> ISQ::mass;
		
		attribute redefines totalMass =
			mass + sum(subcomponents.totalMass.?{in p :> ISQ::mass; p > minMass});
	}

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "mass_rollup.md"
    (diagnostics
      (diagnostic (code "recovered_part_usage_body_element") (severity error) (category parseerror) (span (offset 494) (line 23) (column 3) (len 107)) (message "unexpected token in part usage body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package MassRollup {
    private import NumericalFunctions::*;
    part def MassedThing {
        attribute mass :> ISQ::mass;
        attribute totalMass :> ISQ::mass;
    }
    part simpleThing : MassedThing {
        attribute :>> totalMass = mass;
    }
    part compositeThing : MassedThing {
        part subcomponents : MassedThing[*];
        attribute :>> totalMass default mass + sum(subcomponents.totalMass);
    }
    part filteredMassThing :> compositeThing {
        abstract attribute minMass :> ISQ::mass;
        attribute redefines totalMass =
			mass + sum(subcomponents.totalMass.?{in p :> ISQ::mass; p > minMass});
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 37) (line 2) (column 17) (len 18)) (segments (segment 0 (token "NumericalFunctions") (name "NumericalFunctions") (separator none) (span (offset 37) (line 2) (column 17) (len 18)))))
    (reference r1 (scope relative) (span (offset 106) (line 5) (column 21) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 106) (line 5) (column 21) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 111) (line 5) (column 26) (len 4)))))
    (reference r2 (scope relative) (span (offset 143) (line 6) (column 26) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 143) (line 6) (column 26) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 148) (line 6) (column 31) (len 4)))))
  )
  (root (package (name "MassRollup") (body (import (target (span (span (offset 37) (line 2) (column 17) (len 21))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 55) (line 2) (column 35) (len 3))) (separator (span (offset 55) (line 2) (column 35) (len 2))) (marker (span (offset 57) (line 2) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "MassedThing") (body (attribute-usage (declaration-name "mass") (direction none) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "totalMass") (direction none) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage) (part-usage) (part-usage))))
)
~~~
