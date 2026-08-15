# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 29 (Expressions): MassRollup2"))
~~~
# SOURCE
~~~sysml
package MassRollup2 {
	private import NumericalFunctions::*;
	
	part def MassedThing {
		attribute simpleMass :> ISQ::mass; 
		attribute totalMass :> ISQ::mass default simpleMass;
	}
	
	part compositeThing : MassedThing {
		part subcomponents: MassedThing[*];		
		attribute :>> totalMass default
			simpleMass + sum(subcomponents.totalMass); 
	}
	
	part filteredMassThing :> compositeThing {
		attribute minMass :> ISQ::mass;		
		attribute :>> totalMass =
			simpleMass + sum(subcomponents.totalMass.?{in p:>ISQ::mass; p >= minMass});
	}

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "29_mass_rollup2.md"
    (diagnostics
      (diagnostic (code "recovered_part_usage_body_element") (severity error) (category parseerror) (span (offset 430) (line 17) (column 3) (len 106)) (message "unexpected token in part usage body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package MassRollup2 {
    private import NumericalFunctions::*;
    part def MassedThing {
        attribute simpleMass :> ISQ::mass;
        attribute totalMass :> ISQ::mass default simpleMass;
    }
    part compositeThing : MassedThing {
        part subcomponents : MassedThing[*];
        attribute :>> totalMass default simpleMass + sum(subcomponents.totalMass);
    }
    part filteredMassThing :> compositeThing {
        attribute minMass :> ISQ::mass;
        attribute :>> totalMass =
			simpleMass + sum(subcomponents.totalMass.?{in p:>ISQ::mass; p >= minMass});
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 38) (line 2) (column 17) (len 18)) (segments (segment 0 (token "NumericalFunctions") (name "NumericalFunctions") (separator none) (span (offset 38) (line 2) (column 17) (len 18)))))
    (reference r1 (scope relative) (span (offset 113) (line 5) (column 27) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 113) (line 5) (column 27) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 118) (line 5) (column 32) (len 4)))))
    (reference r2 (scope relative) (span (offset 150) (line 6) (column 26) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 150) (line 6) (column 26) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 155) (line 6) (column 31) (len 4)))))
    (reference r3 (scope relative) (span (offset 168) (line 6) (column 44) (len 10)) (segments (segment 0 (token "simpleMass") (name "simpleMass") (separator none) (span (offset 168) (line 6) (column 44) (len 10)))))
  )
  (root (package (name "MassRollup2") (body (import (target (span (span (offset 38) (line 2) (column 17) (len 21))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 56) (line 2) (column 35) (len 3))) (separator (span (offset 56) (line 2) (column 35) (len 2))) (marker (span (offset 58) (line 2) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "MassedThing") (body (attribute-usage (declaration-name "simpleMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "totalMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 168) (line 6) (column 44) (len 10)) (ref r3))))) (body semicolon)))) (part-usage) (part-usage))))
)
~~~
