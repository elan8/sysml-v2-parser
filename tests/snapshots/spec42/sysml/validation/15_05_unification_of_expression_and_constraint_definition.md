# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (15-Properties-Values-Expressions): 15_05-Unification of Expression and Constraint Definition"))
~~~
# SOURCE
~~~sysml
package '15_05-Unification of Expression and Constraint Definition' {
	private import '15_03-Value Expression'::*;
	private import ControlFunctions::forAll;
	private import SI::*;
	
	constraint def DiscBrakeConstraint {
		in wheelAssy : WheelAssy[4];
		
		wheelAssy->forAll {in ref w: WheelAssy; 
			2 * w.discBrakeAssy.radius < w.wheel.outerDiameter
		}
	}
	
	constraint def DiscBrakeFitConstraint_Alt {
		in discBrakeAssy : DiscBrakeAssy[1];
		in wheel : Wheel[1];	
			
		2 * discBrakeAssy.radius < wheel.outerDiameter
	}
	
	part def Vehicle_2 {
		attribute mass : MassValue[1] = 1200 [kg];
		attribute length : LengthValue[1] = 4.82 [m];
		
		part wheelAssy : WheelAssy[4];
		
		constraint discBrakeConstraint : DiscBrakeConstraint {
			doc
			/*
			 * This constraint is computed, but not asserted. This means a tool can identify 
			 * when it is violated without the model being inconsistent.
			 */
			in wheelAssy = Vehicle_2::wheelAssy;
		}
	}
	
	part def WheelAssy {
		part wheel : Wheel[1];
		part discBrakeAssy : DiscBrakeAssy[1];
		
		assert constraint discBrakeFitConstraint_Alt: DiscBrakeFitConstraint_Alt {
			doc
			/*
			 * This constraint is asserted to be true, which means that the model
			 * is inconsistent if it the constraint is violated.
			 */
		
			in discBrakeAssy = WheelAssy::discBrakeAssy;
			in wheel = WheelAssy::wheel;
		}
	}
	
	part def DiscBrakeAssy {
		attribute radius : LengthValue[1] = 95 [mm];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_05_unification_of_expression_and_constraint_definition.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_05-Unification of Expression and Constraint Definition' {
    private import '15_03-Value Expression'::*;
    private import ControlFunctions::forAll;
    private import SI::*;
    constraint def DiscBrakeConstraint {
        in wheelAssy : WheelAssy[4];
        wheelAssy->forAll { in ref w : WheelAssy; 2 * w.discBrakeAssy.radius < w.wheel.outerDiameter };
    }
    constraint def DiscBrakeFitConstraint_Alt {
        in discBrakeAssy : DiscBrakeAssy[1];
        in wheel : Wheel[1];
        2 * discBrakeAssy.radius < wheel.outerDiameter;
    }
    part def Vehicle_2 {
        attribute mass : MassValue[1] = 1200[kg];
        attribute length : LengthValue[1] = 4.82[m];
        part wheelAssy : WheelAssy[4];
        constraint discBrakeConstraint : DiscBrakeConstraint {
            doc
            /*
			 * This constraint is computed, but not asserted. This means a tool can identify 
			 * when it is violated without the model being inconsistent.
			 */
            in wheelAssy = Vehicle_2::wheelAssy;
        }
    }
    part def WheelAssy {
        part wheel : Wheel[1];
        part discBrakeAssy : DiscBrakeAssy[1];
        assert constraint discBrakeFitConstraint_Alt : DiscBrakeFitConstraint_Alt {
            doc
            /*
			 * This constraint is asserted to be true, which means that the model
			 * is inconsistent if it the constraint is violated.
			 */
            in discBrakeAssy = WheelAssy::discBrakeAssy;
            in wheel = WheelAssy::wheel;
        }
    }
    part def DiscBrakeAssy {
        attribute radius : LengthValue[1] = 95[mm];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 86) (line 2) (column 17) (len 24)) (segments (segment 0 (token "'15_03-Value Expression'") (name "15_03-Value Expression") (separator none) (span (offset 86) (line 2) (column 17) (len 24)))))
    (reference r1 (scope relative) (span (offset 131) (line 3) (column 17) (len 24)) (segments (segment 0 (token "ControlFunctions") (name "ControlFunctions") (separator none) (span (offset 131) (line 3) (column 17) (len 16))) (segment 1 (token "forAll") (name "forAll") (separator colon-colon) (span (offset 149) (line 3) (column 35) (len 6)))))
    (reference r2 (scope relative) (span (offset 173) (line 4) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 173) (line 4) (column 17) (len 2)))))
    (reference r3 (scope relative) (span (offset 256) (line 9) (column 3) (len 9)) (segments (segment 0 (token "wheelAssy") (name "wheelAssy") (separator none) (span (offset 256) (line 9) (column 3) (len 9)))))
    (reference r4 (scope relative) (span (offset 285) (line 9) (column 32) (len 9)) (segments (segment 0 (token "WheelAssy") (name "WheelAssy") (separator none) (span (offset 285) (line 9) (column 32) (len 9)))))
    (reference r5 (scope relative) (span (offset 304) (line 10) (column 8) (len 1)) (segments (segment 0 (token "w") (name "w") (separator none) (span (offset 304) (line 10) (column 8) (len 1)))))
    (reference r6 (scope relative) (span (offset 306) (line 10) (column 10) (len 13)) (segments (segment 0 (token "discBrakeAssy") (name "discBrakeAssy") (separator none) (span (offset 306) (line 10) (column 10) (len 13)))))
    (reference r7 (scope relative) (span (offset 320) (line 10) (column 24) (len 6)) (segments (segment 0 (token "radius") (name "radius") (separator none) (span (offset 320) (line 10) (column 24) (len 6)))))
    (reference r8 (scope relative) (span (offset 329) (line 10) (column 33) (len 1)) (segments (segment 0 (token "w") (name "w") (separator none) (span (offset 329) (line 10) (column 33) (len 1)))))
    (reference r9 (scope relative) (span (offset 331) (line 10) (column 35) (len 5)) (segments (segment 0 (token "wheel") (name "wheel") (separator none) (span (offset 331) (line 10) (column 35) (len 5)))))
    (reference r10 (scope relative) (span (offset 337) (line 10) (column 41) (len 13)) (segments (segment 0 (token "outerDiameter") (name "outerDiameter") (separator none) (span (offset 337) (line 10) (column 41) (len 13)))))
    (reference r11 (scope relative) (span (offset 478) (line 18) (column 7) (len 13)) (segments (segment 0 (token "discBrakeAssy") (name "discBrakeAssy") (separator none) (span (offset 478) (line 18) (column 7) (len 13)))))
    (reference r12 (scope relative) (span (offset 492) (line 18) (column 21) (len 6)) (segments (segment 0 (token "radius") (name "radius") (separator none) (span (offset 492) (line 18) (column 21) (len 6)))))
    (reference r13 (scope relative) (span (offset 501) (line 18) (column 30) (len 5)) (segments (segment 0 (token "wheel") (name "wheel") (separator none) (span (offset 501) (line 18) (column 30) (len 5)))))
    (reference r14 (scope relative) (span (offset 507) (line 18) (column 36) (len 13)) (segments (segment 0 (token "outerDiameter") (name "outerDiameter") (separator none) (span (offset 507) (line 18) (column 36) (len 13)))))
    (reference r15 (scope relative) (span (offset 567) (line 22) (column 20) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 567) (line 22) (column 20) (len 9)))))
    (reference r16 (scope relative) (span (offset 588) (line 22) (column 41) (len 2)) (segments (segment 0 (token "kg") (name "kg") (separator none) (span (offset 588) (line 22) (column 41) (len 2)))))
    (reference r17 (scope relative) (span (offset 614) (line 23) (column 22) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 614) (line 23) (column 22) (len 11)))))
    (reference r18 (scope relative) (span (offset 637) (line 23) (column 45) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 637) (line 23) (column 45) (len 1)))))
    (reference r19 (scope relative) (span (offset 663) (line 25) (column 20) (len 9)) (segments (segment 0 (token "WheelAssy") (name "WheelAssy") (separator none) (span (offset 663) (line 25) (column 20) (len 9)))))
    (reference r20 (scope relative) (span (offset 715) (line 27) (column 36) (len 19)) (segments (segment 0 (token "DiscBrakeConstraint") (name "DiscBrakeConstraint") (separator none) (span (offset 715) (line 27) (column 36) (len 19)))))
    (reference r21 (scope relative) (span (offset 992) (line 38) (column 16) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 992) (line 38) (column 16) (len 5)))))
    (reference r22 (scope relative) (span (offset 1025) (line 39) (column 24) (len 13)) (segments (segment 0 (token "DiscBrakeAssy") (name "DiscBrakeAssy") (separator none) (span (offset 1025) (line 39) (column 24) (len 13)))))
    (reference r23 (scope relative) (span (offset 1411) (line 54) (column 22) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 1411) (line 54) (column 22) (len 11)))))
    (reference r24 (scope relative) (span (offset 1432) (line 54) (column 43) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 1432) (line 54) (column 43) (len 2)))))
  )
  (root (package (name "15_05-Unification of Expression and Constraint Definition") (body brace (import (target (span (span (offset 86) (line 2) (column 17) (len 27))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 110) (line 2) (column 41) (len 3))) (separator (span (offset 110) (line 2) (column 41) (len 2))) (marker (span (offset 112) (line 2) (column 43) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 131) (line 3) (column 17) (len 24))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 173) (line 4) (column 17) (len 5))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 175) (line 4) (column 19) (len 3))) (separator (span (offset 175) (line 4) (column 19) (len 2))) (marker (span (offset 177) (line 4) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (constraint-def (name "DiscBrakeConstraint") (modifiers) (specializes none) (body brace (in-out-declaration) (expression (span (offset 256) (line 9) (column 3) (len 98)) (collection-op (operator "forAll") (base (expression (span (offset 256) (line 9) (column 3) (len 9)) (ref r3))) (arguments) (brace-body (body (span (offset 274) (line 9) (column 21) (len 80)) (open-brace (span (offset 274) (line 9) (column 21) (len 1))) (parameters (parameter (span (offset 275) (line 9) (column 22) (len 20)) (direction in (span (offset 275) (line 9) (column 22) (len 2))) (reference-keyword (span (offset 278) (line 9) (column 25) (len 3))) (declaration (name "w") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 294) (line 9) (column 41) (len 1)))))) (result (expression (span (offset 300) (line 10) (column 4) (len 50)) (binary (operator "<") (left (expression (span (offset 300) (line 10) (column 4) (len 26)) (binary (operator "*") (left (expression (span (offset 300) (line 10) (column 4) (len 1)) (integer 2))) (right (expression (span (offset 304) (line 10) (column 8) (len 22)) (member-access (base (expression (span (offset 304) (line 10) (column 8) (len 15)) (member-access (base (expression (span (offset 304) (line 10) (column 8) (len 1)) (ref r5))) (separator dot) (member (ref r6))))) (separator dot) (member (ref r7)))))))) (right (expression (span (offset 329) (line 10) (column 33) (len 21)) (member-access (base (expression (span (offset 329) (line 10) (column 33) (len 7)) (member-access (base (expression (span (offset 329) (line 10) (column 33) (len 1)) (ref r8))) (separator dot) (member (ref r9))))) (separator dot) (member (ref r10)))))))) (close-brace (span (offset 353) (line 11) (column 3) (len 1))))))))) (constraint-def (name "DiscBrakeFitConstraint_Alt") (modifiers) (specializes none) (body brace (in-out-declaration) (in-out-declaration) (expression (span (offset 474) (line 18) (column 3) (len 46)) (binary (operator "<") (left (expression (span (offset 474) (line 18) (column 3) (len 24)) (binary (operator "*") (left (expression (span (offset 474) (line 18) (column 3) (len 1)) (integer 2))) (right (expression (span (offset 478) (line 18) (column 7) (len 20)) (member-access (base (expression (span (offset 478) (line 18) (column 7) (len 13)) (ref r11))) (separator dot) (member (ref r12)))))))) (right (expression (span (offset 501) (line 18) (column 30) (len 19)) (member-access (base (expression (span (offset 501) (line 18) (column 30) (len 5)) (ref r13))) (separator dot) (member (ref r14))))))))) (part-def (name "Vehicle_2") (modifiers) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 582) (line 22) (column 35) (len 9)) (bracket (base (expression (span (offset 582) (line 22) (column 35) (len 4)) (integer 1200))) (operands (sequence-list (element first (expression (span (offset 588) (line 22) (column 41) (len 2)) (ref r16)))))))))) (body semicolon)) (attribute-usage (declaration-name "length") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 631) (line 23) (column 39) (len 8)) (bracket (base (expression (span (offset 631) (line 23) (column 39) (len 4)) (real "4.82"))) (operands (sequence-list (element first (expression (span (offset 637) (line 23) (column 45) (len 1)) (ref r18)))))))))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheelAssy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity (lower (expression (span (offset 673) (line 25) (column 30) (len 1)) (integer 4))) (upper (expression (span (offset 673) (line 25) (column 30) (len 1)) (integer 4)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "discBrakeConstraint") (short-name none) (type (ref r20)) (multiplicity none) (subsets none) (redefines none) (body brace (doc (name none) (locale none) (body (span (offset 749) (line 29) (column 6) (len 154)) (normalized "This constraint is computed, but not asserted. This means a tool can identify \nwhen it is violated without the model being inconsistent.\n"))) (in-out-declaration))))) (part-def (name "WheelAssy") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity (lower (expression (span (offset 998) (line 38) (column 22) (len 1)) (integer 1))) (upper (expression (span (offset 998) (line 38) (column 22) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "discBrakeAssy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity (lower (expression (span (offset 1039) (line 39) (column 38) (len 1)) (integer 1))) (upper (expression (span (offset 1039) (line 39) (column 38) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (assert-constraint))) (part-def (name "DiscBrakeAssy") (modifiers) (body brace (attribute-usage (declaration-name "radius") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1428) (line 54) (column 39) (len 7)) (bracket (base (expression (span (offset 1428) (line 54) (column 39) (len 2)) (integer 95))) (operands (sequence-list (element first (expression (span (offset 1432) (line 54) (column 43) (len 2)) (ref r24)))))))))) (body semicolon)))))))
)
~~~
