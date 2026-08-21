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
        attribute :>> totalMass = mass + sum(subcomponents.totalMass.?{ in p :> ISQ::mass; p > minMass });
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
    (reference r7 (scope relative) (span (offset 297) (line 14) (column 23) (len 11)) (segments (segment 0 (token "MassedThing") (name "MassedThing") (separator none) (span (offset 297) (line 14) (column 23) (len 11)))))
    (reference r8 (scope relative) (span (offset 338) (line 16) (column 23) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 338) (line 16) (column 23) (len 9)))))
    (reference r9 (scope relative) (span (offset 359) (line 17) (column 4) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 359) (line 17) (column 4) (len 4)))))
    (reference r10 (scope relative) (span (offset 366) (line 17) (column 11) (len 3)) (segments (segment 0 (token "sum") (name "sum") (separator none) (span (offset 366) (line 17) (column 11) (len 3)))))
    (reference r11 (scope relative) (span (offset 370) (line 17) (column 15) (len 13)) (segments (segment 0 (token "subcomponents") (name "subcomponents") (separator none) (span (offset 370) (line 17) (column 15) (len 13)))))
    (reference r12 (scope relative) (span (offset 384) (line 17) (column 29) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 384) (line 17) (column 29) (len 9)))))
    (reference r13 (scope relative) (span (offset 429) (line 20) (column 28) (len 14)) (segments (segment 0 (token "compositeThing") (name "compositeThing") (separator none) (span (offset 429) (line 20) (column 28) (len 14)))))
    (reference r14 (scope relative) (span (offset 478) (line 21) (column 33) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 478) (line 21) (column 33) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 483) (line 21) (column 38) (len 4)))))
    (reference r15 (scope relative) (span (offset 514) (line 23) (column 23) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 514) (line 23) (column 23) (len 9)))))
    (reference r16 (scope relative) (span (offset 529) (line 24) (column 4) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 529) (line 24) (column 4) (len 4)))))
    (reference r17 (scope relative) (span (offset 536) (line 24) (column 11) (len 3)) (segments (segment 0 (token "sum") (name "sum") (separator none) (span (offset 536) (line 24) (column 11) (len 3)))))
    (reference r18 (scope relative) (span (offset 540) (line 24) (column 15) (len 13)) (segments (segment 0 (token "subcomponents") (name "subcomponents") (separator none) (span (offset 540) (line 24) (column 15) (len 13)))))
    (reference r19 (scope relative) (span (offset 554) (line 24) (column 29) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 554) (line 24) (column 29) (len 9)))))
    (reference r20 (scope relative) (span (offset 574) (line 24) (column 49) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 574) (line 24) (column 49) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 579) (line 24) (column 54) (len 4)))))
    (reference r21 (scope relative) (span (offset 585) (line 24) (column 60) (len 1)) (segments (segment 0 (token "p") (name "p") (separator none) (span (offset 585) (line 24) (column 60) (len 1)))))
    (reference r22 (scope relative) (span (offset 589) (line 24) (column 64) (len 7)) (segments (segment 0 (token "minMass") (name "minMass") (separator none) (span (offset 589) (line 24) (column 64) (len 7)))))
  )
  (root (package (name "MassRollup") (body brace (import (target (span (span (offset 37) (line 2) (column 17) (len 21))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 55) (line 2) (column 35) (len 3))) (separator (span (offset 55) (line 2) (column 35) (len 2))) (marker (span (offset 57) (line 2) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "MassedThing") (modifiers) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "totalMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "simpleThing") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 227) (line 10) (column 35) (len 4)) (ref r5))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "compositeThing") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "subcomponents") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r8)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 359) (line 17) (column 4) (len 35)) (binary (operator "+") (left (expression (span (offset 359) (line 17) (column 4) (len 4)) (ref r9))) (right (expression (span (offset 366) (line 17) (column 11) (len 28)) (invocation (callee (expression (span (offset 366) (line 17) (column 11) (len 3)) (ref r10))) (arguments (argument (parameter none) (value (expression (span (offset 370) (line 17) (column 15) (len 23)) (member-access (base (expression (span (offset 370) (line 17) (column 15) (len 13)) (ref r11))) (separator dot) (member (ref r12))))))))))))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "filteredMassThing") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r13))) (value none))) (redefines none) (value none) (body brace (attribute-usage (declaration-name "minMass") (direction none) (derived false) (usage-prefix abstract) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r14)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 529) (line 24) (column 4) (len 69)) (binary (operator "+") (left (expression (span (offset 529) (line 24) (column 4) (len 4)) (ref r16))) (right (expression (span (offset 536) (line 24) (column 11) (len 62)) (invocation (callee (expression (span (offset 536) (line 24) (column 11) (len 3)) (ref r17))) (arguments (argument (parameter none) (value (expression (span (offset 540) (line 24) (column 15) (len 57)) (collection-op (operator "select") (base (expression (span (offset 540) (line 24) (column 15) (len 23)) (member-access (base (expression (span (offset 540) (line 24) (column 15) (len 13)) (ref r18))) (separator dot) (member (ref r19))))) (arguments) (brace-body (body (span (offset 565) (line 24) (column 40) (len 32)) (open-brace (span (offset 565) (line 24) (column 40) (len 1))) (parameters (parameter (span (offset 566) (line 24) (column 41) (len 18)) (direction in (span (offset 566) (line 24) (column 41) (len 2))) (reference-keyword none) (declaration (name "p") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r20))) (value none)) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 583) (line 24) (column 58) (len 1)))))) (result (expression (span (offset 585) (line 24) (column 60) (len 11)) (binary (operator ">") (left (expression (span (offset 585) (line 24) (column 60) (len 1)) (ref r21))) (right (expression (span (offset 589) (line 24) (column 64) (len 7)) (ref r22)))))) (close-brace (span (offset 596) (line 24) (column 71) (len 1)))))))))))))))))) (body semicolon)))))))
)
~~~
