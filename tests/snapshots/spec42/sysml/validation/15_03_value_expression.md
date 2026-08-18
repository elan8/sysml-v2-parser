# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (15-Properties-Values-Expressions): 15_03-Value Expression"))
~~~
# SOURCE
~~~sysml
package '15_03-Value Expression' {
    private import SI::*;
    private import USCustomaryUnits::*;

    part def Vehicle_1 {
        attribute mass: MassValue = 1200 [kg];
        attribute length: LengthValue = 4.82 [m];
        part leftFrontWheel : Wheel;
        part rightFrontWheel : Wheel;
    }

    part def Wheel {
    	attribute hubDiameter: LengthValue = 18 ['in'];
        attribute width: LengthValue = 245 [mm];
        attribute outerDiameter: LengthValue = (hubDiameter + 2 * tire.height) [mm] {
	        doc
	        /*
	         * This binds 'outDiameter' to the result of a computed attribute.
	         * There is no need to mark it as "derived".
	         */
        }
        part tire: Tire[1];
    }
    
    part def Tire {
    	attribute profileDepth: LengthValue default 6.0 [mm];
        constraint hasLegalProfileDepth {profileDepth >= 3.5 [mm]}
    	attribute height: LengthValue = 45 [mm];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_03_value_expression.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_03-Value Expression' {
    private import SI::*;
    private import USCustomaryUnits::*;
    part def Vehicle_1 {
        attribute mass : MassValue = 1200 [kg];
        attribute length : LengthValue = 4.82 [m];
        part leftFrontWheel : Wheel;
        part rightFrontWheel : Wheel;
    }
    part def Wheel {
        attribute hubDiameter : LengthValue = 18 ['in'];
        attribute width : LengthValue = 245 [mm];
        attribute outerDiameter : LengthValue = (hubDiameter + 2 * tire.height) [mm] {
            doc
            /*
	         * This binds 'outDiameter' to the result of a computed attribute.
	         * There is no need to mark it as "derived".
	         */
        }
        part tire : Tire[1];
    }
    part def Tire {
        attribute profileDepth : LengthValue default 6.0 [mm];
        constraint hasLegalProfileDepth {
            profileDepth >= 3.5 [mm];
        }
        attribute height : LengthValue = 45 [mm];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 54) (line 2) (column 20) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 54) (line 2) (column 20) (len 2)))))
    (reference r1 (scope relative) (span (offset 80) (line 3) (column 20) (len 16)) (segments (segment 0 (token "USCustomaryUnits") (name "USCustomaryUnits") (separator none) (span (offset 80) (line 3) (column 20) (len 16)))))
    (reference r2 (scope relative) (span (offset 151) (line 6) (column 25) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 151) (line 6) (column 25) (len 9)))))
    (reference r3 (scope relative) (span (offset 200) (line 7) (column 27) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 200) (line 7) (column 27) (len 11)))))
    (reference r4 (scope relative) (span (offset 254) (line 8) (column 31) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 254) (line 8) (column 31) (len 5)))))
    (reference r5 (scope relative) (span (offset 292) (line 9) (column 32) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 292) (line 9) (column 32) (len 5)))))
    (reference r6 (scope relative) (span (offset 355) (line 13) (column 29) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 355) (line 13) (column 29) (len 11)))))
    (reference r7 (scope relative) (span (offset 405) (line 14) (column 26) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 405) (line 14) (column 26) (len 11)))))
    (reference r8 (scope relative) (span (offset 462) (line 15) (column 34) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 462) (line 15) (column 34) (len 11)))))
    (reference r9 (scope relative) (span (offset 477) (line 15) (column 49) (len 11)) (segments (segment 0 (token "hubDiameter") (name "hubDiameter") (separator none) (span (offset 477) (line 15) (column 49) (len 11)))))
    (reference r10 (scope relative) (span (offset 495) (line 15) (column 67) (len 4)) (segments (segment 0 (token "tire") (name "tire") (separator none) (span (offset 495) (line 15) (column 67) (len 4)))))
    (reference r11 (scope relative) (span (offset 500) (line 15) (column 72) (len 6)) (segments (segment 0 (token "height") (name "height") (separator none) (span (offset 500) (line 15) (column 72) (len 6)))))
    (reference r12 (scope relative) (span (offset 712) (line 22) (column 20) (len 4)) (segments (segment 0 (token "Tire") (name "Tire") (separator none) (span (offset 712) (line 22) (column 20) (len 4)))))
    (reference r13 (scope relative) (span (offset 781) (line 26) (column 30) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 781) (line 26) (column 30) (len 11)))))
    (reference r14 (scope relative) (span (offset 901) (line 28) (column 24) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 901) (line 28) (column 24) (len 11)))))
  )
  (root (package (name "15_03-Value Expression") (body brace (import (target (span (span (offset 54) (line 2) (column 20) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 56) (line 2) (column 22) (len 3))) (separator (span (offset 56) (line 2) (column 22) (len 2))) (marker (span (offset 58) (line 2) (column 24) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 80) (line 3) (column 20) (len 19))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 96) (line 3) (column 36) (len 3))) (separator (span (offset 96) (line 3) (column 36) (len 2))) (marker (span (offset 98) (line 3) (column 38) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle_1") (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 163) (line 6) (column 37) (len 9)) (literal-with-unit (value (expression (span (offset 163) (line 6) (column 37) (len 4)) (integer 1200))) (unit (expression (span (offset 169) (line 6) (column 43) (len 2)) (bracket (expression (span (offset 169) (line 6) (column 43) (len 2)) (unit "kg")))))))))) (body semicolon)) (attribute-usage (declaration-name "length") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 214) (line 7) (column 41) (len 8)) (literal-with-unit (value (expression (span (offset 214) (line 7) (column 41) (len 4)) (real "4.82"))) (unit (expression (span (offset 220) (line 7) (column 47) (len 1)) (bracket (expression (span (offset 220) (line 7) (column 47) (len 1)) (unit "m")))))))))) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "leftFrontWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rightFrontWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "Wheel") (body brace (attribute-usage (declaration-name "hubDiameter") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 369) (line 13) (column 43) (len 9)) (literal-with-unit (value (expression (span (offset 369) (line 13) (column 43) (len 2)) (integer 18))) (unit (expression (span (offset 373) (line 13) (column 47) (len 4)) (bracket (expression (span (offset 373) (line 13) (column 47) (len 4)) (unit "in")))))))))) (body semicolon)) (attribute-usage (declaration-name "width") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 419) (line 14) (column 40) (len 8)) (literal-with-unit (value (expression (span (offset 419) (line 14) (column 40) (len 3)) (integer 245))) (unit (expression (span (offset 424) (line 14) (column 45) (len 2)) (bracket (expression (span (offset 424) (line 14) (column 45) (len 2)) (unit "mm")))))))))) (body semicolon)) (attribute-usage (declaration-name "outerDiameter") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 476) (line 15) (column 48) (len 36)) (literal-with-unit (value (expression (span (offset 476) (line 15) (column 48) (len 31)) (parenthesized (expression (span (offset 477) (line 15) (column 49) (len 29)) (binary (operator "+") (left (expression (span (offset 477) (line 15) (column 49) (len 11)) (ref r9))) (right (expression (span (offset 491) (line 15) (column 63) (len 15)) (binary (operator "*") (left (expression (span (offset 491) (line 15) (column 63) (len 1)) (integer 2))) (right (expression (span (offset 495) (line 15) (column 67) (len 11)) (member-access (base (expression (span (offset 495) (line 15) (column 67) (len 4)) (ref r10))) (separator dot) (member (ref r11))))))))))))) (unit (expression (span (offset 509) (line 15) (column 81) (len 2)) (bracket (expression (span (offset 509) (line 15) (column 81) (len 2)) (unit "mm")))))))))) (body brace (doc))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "tire") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity (lower (expression (span (offset 717) (line 22) (column 25) (len 1)) (integer 1))) (upper (expression (span (offset 717) (line 22) (column 25) (len 1)) (integer 1)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "Tire") (body brace (attribute-usage (declaration-name "profileDepth") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 801) (line 26) (column 50) (len 8)) (literal-with-unit (value (expression (span (offset 801) (line 26) (column 50) (len 3)) (real "6.0"))) (unit (expression (span (offset 806) (line 26) (column 55) (len 2)) (bracket (expression (span (offset 806) (line 26) (column 55) (len 2)) (unit "mm")))))))))) (body semicolon)) (constraint-usage) (attribute-usage (declaration-name "height") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 915) (line 28) (column 38) (len 7)) (literal-with-unit (value (expression (span (offset 915) (line 28) (column 38) (len 2)) (integer 45))) (unit (expression (span (offset 919) (line 28) (column 42) (len 2)) (bracket (expression (span (offset 919) (line 28) (column 42) (len 2)) (unit "mm")))))))))) (body semicolon)))))))
)
~~~
