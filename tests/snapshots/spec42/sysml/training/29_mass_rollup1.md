# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 29 (Expressions): MassRollup1"))
~~~
# SOURCE
~~~sysml
package MassRollup1 {
	private import NumericalFunctions::*;
	
	part def MassedThing {
		attribute simpleMass :> ISQ::mass; 
		attribute totalMass :> ISQ::mass;
	}
	
	part simpleThing : MassedThing {
		attribute :>> totalMass = simpleMass;
	}
	
	part compositeThing : MassedThing {
		part subcomponents: MassedThing[*];		
		attribute :>> totalMass =
			simpleMass + sum(subcomponents.totalMass); 
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "29_mass_rollup1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package MassRollup1 {
    private import NumericalFunctions::*;
    part def MassedThing {
        attribute simpleMass :> ISQ::mass;
        attribute totalMass :> ISQ::mass;
    }
    part simpleThing : MassedThing {
        attribute :>> totalMass = simpleMass;
    }
    part compositeThing : MassedThing {
        part subcomponents : MassedThing[*];
        attribute :>> totalMass = simpleMass + sum(subcomponents.totalMass);
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
    (reference r3 (scope relative) (span (offset 186) (line 9) (column 21) (len 11)) (segments (segment 0 (token "MassedThing") (name "MassedThing") (separator none) (span (offset 186) (line 9) (column 21) (len 11)))))
    (reference r4 (scope relative) (span (offset 216) (line 10) (column 17) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 216) (line 10) (column 17) (len 9)))))
    (reference r5 (scope relative) (span (offset 228) (line 10) (column 29) (len 10)) (segments (segment 0 (token "simpleMass") (name "simpleMass") (separator none) (span (offset 228) (line 10) (column 29) (len 10)))))
    (reference r6 (scope relative) (span (offset 268) (line 13) (column 24) (len 11)) (segments (segment 0 (token "MassedThing") (name "MassedThing") (separator none) (span (offset 268) (line 13) (column 24) (len 11)))))
    (reference r7 (scope relative) (span (offset 304) (line 14) (column 23) (len 11)) (segments (segment 0 (token "MassedThing") (name "MassedThing") (separator none) (span (offset 304) (line 14) (column 23) (len 11)))))
    (reference r8 (scope relative) (span (offset 338) (line 15) (column 17) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 338) (line 15) (column 17) (len 9)))))
    (reference r9 (scope relative) (span (offset 353) (line 16) (column 4) (len 10)) (segments (segment 0 (token "simpleMass") (name "simpleMass") (separator none) (span (offset 353) (line 16) (column 4) (len 10)))))
    (reference r10 (scope relative) (span (offset 366) (line 16) (column 17) (len 3)) (segments (segment 0 (token "sum") (name "sum") (separator none) (span (offset 366) (line 16) (column 17) (len 3)))))
    (reference r11 (scope relative) (span (offset 370) (line 16) (column 21) (len 13)) (segments (segment 0 (token "subcomponents") (name "subcomponents") (separator none) (span (offset 370) (line 16) (column 21) (len 13)))))
    (reference r12 (scope relative) (span (offset 384) (line 16) (column 35) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 384) (line 16) (column 35) (len 9)))))
  )
  (root (package (name "MassRollup1") (body brace (import (target (span (span (offset 38) (line 2) (column 17) (len 21))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 56) (line 2) (column 35) (len 3))) (separator (span (offset 56) (line 2) (column 35) (len 2))) (marker (span (offset 58) (line 2) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "MassedThing") (body brace (attribute-usage (declaration-name "simpleMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "totalMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "simpleThing") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 228) (line 10) (column 29) (len 10)) (ref r5))))) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "compositeThing") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "subcomponents") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 353) (line 16) (column 4) (len 41)) (binary (operator "+") (left (expression (span (offset 353) (line 16) (column 4) (len 10)) (ref r9))) (right (expression (span (offset 366) (line 16) (column 17) (len 28)) (invocation (callee (expression (span (offset 366) (line 16) (column 17) (len 3)) (ref r10))) (arguments (argument (parameter none) (value (expression (span (offset 370) (line 16) (column 21) (len 23)) (member-access (base (expression (span (offset 370) (line 16) (column 21) (len 13)) (ref r11))) (separator dot) (member (ref r12))))))))))))))) (body semicolon)))))))
)
~~~
