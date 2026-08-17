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
    (reference r3 (scope relative) (span (offset 179) (line 9) (column 21) (len 11)) (segments (segment 0 (token "MassedThing") (name "MassedThing") (separator none) (span (offset 179) (line 9) (column 21) (len 11)))))
    (reference r4 (scope relative) (span (offset 215) (line 10) (column 23) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 215) (line 10) (column 23) (len 9)))))
    (reference r5 (scope relative) (span (offset 227) (line 10) (column 35) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 227) (line 10) (column 35) (len 4)))))
    (reference r6 (scope relative) (span (offset 261) (line 13) (column 24) (len 11)) (segments (segment 0 (token "MassedThing") (name "MassedThing") (separator none) (span (offset 261) (line 13) (column 24) (len 11)))))
    (reference r7 (scope relative) (span (offset 338) (line 16) (column 23) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 338) (line 16) (column 23) (len 9)))))
    (reference r8 (scope relative) (span (offset 359) (line 17) (column 4) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 359) (line 17) (column 4) (len 4)))))
    (reference r9 (scope relative) (span (offset 366) (line 17) (column 11) (len 3)) (segments (segment 0 (token "sum") (name "sum") (separator none) (span (offset 366) (line 17) (column 11) (len 3)))))
    (reference r10 (scope relative) (span (offset 370) (line 17) (column 15) (len 13)) (segments (segment 0 (token "subcomponents") (name "subcomponents") (separator none) (span (offset 370) (line 17) (column 15) (len 13)))))
    (reference r11 (scope relative) (span (offset 384) (line 17) (column 29) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 384) (line 17) (column 29) (len 9)))))
    (reference r12 (scope relative) (span (offset 478) (line 21) (column 33) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 478) (line 21) (column 33) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 483) (line 21) (column 38) (len 4)))))
  )
  (root (package (name "MassRollup") (body brace (import (target (span (span (offset 37) (line 2) (column 17) (len 21))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 55) (line 2) (column 35) (len 3))) (separator (span (offset 55) (line 2) (column 35) (len 2))) (marker (span (offset 57) (line 2) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "MassedThing") (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "totalMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (declaration-name "simpleThing") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 227) (line 10) (column 35) (len 4)) (ref r5))))) (body semicolon)))) (part-usage (declaration-name "compositeThing") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 359) (line 17) (column 4) (len 35)) (binary (operator "+") (left (expression (span (offset 359) (line 17) (column 4) (len 4)) (ref r8))) (right (expression (span (offset 366) (line 17) (column 11) (len 28)) (invocation (callee (expression (span (offset 366) (line 17) (column 11) (len 3)) (ref r9))) (arguments (argument (parameter none) (value (expression (span (offset 370) (line 17) (column 15) (len 23)) (member-access (base (expression (span (offset 370) (line 17) (column 15) (len 13)) (ref r10))) (separator dot) (member (ref r11))))))))))))))) (body semicolon)))) (part-usage (declaration-name "filteredMassThing") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (attribute-usage (declaration-name "minMass") (direction none) (derived false) (usage-prefix abstract) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r12)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (malformed (code "recovered_part_usage_body_element") (found "attribute redefines totalMass =") (span (offset 494) (line 23) (column 3) (len 107))))))))
)
~~~
