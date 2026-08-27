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
        attribute :>> totalMass = simpleMass + sum(subcomponents.totalMass.?{ in p :> ISQ::mass; p >= minMass });
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
    (reference r4 (scope relative) (span (offset 208) (line 9) (column 24) (len 11)) (segments (segment 0 (token "MassedThing") (name "MassedThing") (separator none) (span (offset 208) (line 9) (column 24) (len 11)))))
    (reference r5 (scope relative) (span (offset 244) (line 10) (column 23) (len 11)) (segments (segment 0 (token "MassedThing") (name "MassedThing") (separator none) (span (offset 244) (line 10) (column 23) (len 11)))))
    (reference r6 (scope relative) (span (offset 278) (line 11) (column 17) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 278) (line 11) (column 17) (len 9)))))
    (reference r7 (scope relative) (span (offset 299) (line 12) (column 4) (len 10)) (segments (segment 0 (token "simpleMass") (name "simpleMass") (separator none) (span (offset 299) (line 12) (column 4) (len 10)))))
    (reference r8 (scope relative) (span (offset 312) (line 12) (column 17) (len 3)) (segments (segment 0 (token "sum") (name "sum") (separator none) (span (offset 312) (line 12) (column 17) (len 3)))))
    (reference r9 (scope relative) (span (offset 316) (line 12) (column 21) (len 13)) (segments (segment 0 (token "subcomponents") (name "subcomponents") (separator none) (span (offset 316) (line 12) (column 21) (len 13)))))
    (reference r10 (scope relative) (span (offset 330) (line 12) (column 35) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 330) (line 12) (column 35) (len 9)))))
    (reference r11 (scope relative) (span (offset 375) (line 15) (column 28) (len 14)) (segments (segment 0 (token "compositeThing") (name "compositeThing") (separator none) (span (offset 375) (line 15) (column 28) (len 14)))))
    (reference r12 (scope relative) (span (offset 415) (line 16) (column 24) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 415) (line 16) (column 24) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 420) (line 16) (column 29) (len 4)))))
    (reference r13 (scope relative) (span (offset 444) (line 17) (column 17) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 444) (line 17) (column 17) (len 9)))))
    (reference r14 (scope relative) (span (offset 459) (line 18) (column 4) (len 10)) (segments (segment 0 (token "simpleMass") (name "simpleMass") (separator none) (span (offset 459) (line 18) (column 4) (len 10)))))
    (reference r15 (scope relative) (span (offset 472) (line 18) (column 17) (len 3)) (segments (segment 0 (token "sum") (name "sum") (separator none) (span (offset 472) (line 18) (column 17) (len 3)))))
    (reference r16 (scope relative) (span (offset 476) (line 18) (column 21) (len 13)) (segments (segment 0 (token "subcomponents") (name "subcomponents") (separator none) (span (offset 476) (line 18) (column 21) (len 13)))))
    (reference r17 (scope relative) (span (offset 490) (line 18) (column 35) (len 9)) (segments (segment 0 (token "totalMass") (name "totalMass") (separator none) (span (offset 490) (line 18) (column 35) (len 9)))))
    (reference r18 (scope relative) (span (offset 508) (line 18) (column 53) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 508) (line 18) (column 53) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 513) (line 18) (column 58) (len 4)))))
    (reference r19 (scope relative) (span (offset 519) (line 18) (column 64) (len 1)) (segments (segment 0 (token "p") (name "p") (separator none) (span (offset 519) (line 18) (column 64) (len 1)))))
    (reference r20 (scope relative) (span (offset 524) (line 18) (column 69) (len 7)) (segments (segment 0 (token "minMass") (name "minMass") (separator none) (span (offset 524) (line 18) (column 69) (len 7)))))
  )
  (root (package (name "MassRollup2") (body brace (import (target (span (span (offset 38) (line 2) (column 17) (len 21))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 56) (line 2) (column 35) (len 3))) (separator (span (offset 56) (line 2) (column 35) (len 2))) (marker (span (offset 58) (line 2) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "MassedThing") (modifiers) (body brace (attribute-usage (declaration-name "simpleMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "totalMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 168) (line 6) (column 44) (len 10)) (ref r3))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "compositeThing") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "subcomponents") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 299) (line 12) (column 4) (len 41)) (binary (operator "+") (left (expression (span (offset 299) (line 12) (column 4) (len 10)) (ref r7))) (right (expression (span (offset 312) (line 12) (column 17) (len 28)) (invocation (callee (expression (span (offset 312) (line 12) (column 17) (len 3)) (ref r8))) (arguments (argument (parameter none) (value (expression (span (offset 316) (line 12) (column 21) (len 23)) (member-access (base (expression (span (offset 316) (line 12) (column 21) (len 13)) (ref r9))) (separator dot) (member (ref r10))))))))))))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "filteredMassThing") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r11))) (value none))) (redefines none) (value none) (body brace (attribute-usage (declaration-name "minMass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r12)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 459) (line 18) (column 4) (len 74)) (binary (operator "+") (left (expression (span (offset 459) (line 18) (column 4) (len 10)) (ref r14))) (right (expression (span (offset 472) (line 18) (column 17) (len 61)) (invocation (callee (expression (span (offset 472) (line 18) (column 17) (len 3)) (ref r15))) (arguments (argument (parameter none) (value (expression (span (offset 476) (line 18) (column 21) (len 56)) (collection-op (operator "select") (base (expression (span (offset 476) (line 18) (column 21) (len 23)) (member-access (base (expression (span (offset 476) (line 18) (column 21) (len 13)) (ref r16))) (separator dot) (member (ref r17))))) (arguments) (brace-body (body (span (offset 501) (line 18) (column 46) (len 31)) (open-brace (span (offset 501) (line 18) (column 46) (len 1))) (parameters (parameter (span (offset 502) (line 18) (column 47) (len 16)) (direction in (span (offset 502) (line 18) (column 47) (len 2))) (reference-keyword none) (declaration (name "p") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r18))) (value none)) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 517) (line 18) (column 62) (len 1)))))) (result (expression (span (offset 519) (line 18) (column 64) (len 12)) (binary (operator ">=") (left (expression (span (offset 519) (line 18) (column 64) (len 1)) (ref r19))) (right (expression (span (offset 524) (line 18) (column 69) (len 7)) (ref r20)))))) (close-brace (span (offset 531) (line 18) (column 76) (len 1)))))))))))))))))) (body semicolon)))))))
)
~~~
