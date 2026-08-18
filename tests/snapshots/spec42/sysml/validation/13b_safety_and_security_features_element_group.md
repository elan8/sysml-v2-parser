# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (13-Model Containment): 13b-Safety and Security Features Element Group"))
~~~
# SOURCE
~~~sysml
package '13b-Safety and Security Features Element Group' {
	
	part vehicle1_c1 {
		part interior {
			part alarm;
			part seatBelt[2];
			part frontSeat[2];
			part driverAirBag;
		}
		part bodyAssy {
			part body;
			part bumper;
			part keylessEntry;
		}
	}
	
	package 'Safety Features' {
		/* Parts that contribute to safety. */
		
		public import vehicle1_c1::interior::seatBelt;
		public import vehicle1_c1::interior::driverAirBag;
		public import vehicle1_c1::bodyAssy::bumper;		
	}
	
	package 'Security Features' {
		/* Parts that contribute to security. */
		
		public import vehicle1_c1::interior::alarm;
		public import vehicle1_c1::bodyAssy::keylessEntry;
	}
	
	package 'Safety & Security Features' {
		/* Parts that contribute to safety AND
		 * parts that contribute to security.
		 */
		 
		public import 'Safety Features'::*;
		public import 'Security Features'::*;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "13b_safety_and_security_features_element_group.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '13b-Safety and Security Features Element Group' {
    part vehicle1_c1 {
        part interior {
            part alarm;
            part seatBelt[2];
            part frontSeat[2];
            part driverAirBag;
        }
        part bodyAssy {
            part body;
            part bumper;
            part keylessEntry;
        }
    }
    package 'Safety Features' {
        public import vehicle1_c1::interior::seatBelt;
        public import vehicle1_c1::interior::driverAirBag;
        public import vehicle1_c1::bodyAssy::bumper;
    }
    package 'Security Features' {
        public import vehicle1_c1::interior::alarm;
        public import vehicle1_c1::bodyAssy::keylessEntry;
    }
    package 'Safety & Security Features' {
        public import 'Safety Features'::*;
        public import 'Security Features'::*;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 351) (line 20) (column 17) (len 31)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 351) (line 20) (column 17) (len 11))) (segment 1 (token "interior") (name "interior") (separator colon-colon) (span (offset 364) (line 20) (column 30) (len 8))) (segment 2 (token "seatBelt") (name "seatBelt") (separator colon-colon) (span (offset 374) (line 20) (column 40) (len 8)))))
    (reference r1 (scope relative) (span (offset 400) (line 21) (column 17) (len 35)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 400) (line 21) (column 17) (len 11))) (segment 1 (token "interior") (name "interior") (separator colon-colon) (span (offset 413) (line 21) (column 30) (len 8))) (segment 2 (token "driverAirBag") (name "driverAirBag") (separator colon-colon) (span (offset 423) (line 21) (column 40) (len 12)))))
    (reference r2 (scope relative) (span (offset 453) (line 22) (column 17) (len 29)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 453) (line 22) (column 17) (len 11))) (segment 1 (token "bodyAssy") (name "bodyAssy") (separator colon-colon) (span (offset 466) (line 22) (column 30) (len 8))) (segment 2 (token "bumper") (name "bumper") (separator colon-colon) (span (offset 476) (line 22) (column 40) (len 6)))))
    (reference r3 (scope relative) (span (offset 584) (line 28) (column 17) (len 28)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 584) (line 28) (column 17) (len 11))) (segment 1 (token "interior") (name "interior") (separator colon-colon) (span (offset 597) (line 28) (column 30) (len 8))) (segment 2 (token "alarm") (name "alarm") (separator colon-colon) (span (offset 607) (line 28) (column 40) (len 5)))))
    (reference r4 (scope relative) (span (offset 630) (line 29) (column 17) (len 35)) (segments (segment 0 (token "vehicle1_c1") (name "vehicle1_c1") (separator none) (span (offset 630) (line 29) (column 17) (len 11))) (segment 1 (token "bodyAssy") (name "bodyAssy") (separator colon-colon) (span (offset 643) (line 29) (column 30) (len 8))) (segment 2 (token "keylessEntry") (name "keylessEntry") (separator colon-colon) (span (offset 653) (line 29) (column 40) (len 12)))))
    (reference r5 (scope relative) (span (offset 819) (line 37) (column 17) (len 17)) (segments (segment 0 (token "'Safety Features'") (name "Safety Features") (separator none) (span (offset 819) (line 37) (column 17) (len 17)))))
    (reference r6 (scope relative) (span (offset 857) (line 38) (column 17) (len 19)) (segments (segment 0 (token "'Security Features'") (name "Security Features") (separator none) (span (offset 857) (line 38) (column 17) (len 19)))))
  )
  (root (package (name "13b-Safety and Security Features Element Group") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle1_c1") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "interior") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "alarm") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "seatBelt") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 131) (line 6) (column 18) (len 1)) (integer 2))) (upper (expression (span (offset 131) (line 6) (column 18) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontSeat") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 153) (line 7) (column 19) (len 1)) (integer 2))) (upper (expression (span (offset 153) (line 7) (column 19) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "driverAirBag") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bodyAssy") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "body") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bumper") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "keylessEntry") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))))) (package (name "Safety Features") (body brace (import (target (span (span (offset 351) (line 20) (column 17) (len 31))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 400) (line 21) (column 17) (len 35))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 453) (line 22) (column 17) (len 29))) (all none) (ref r2) (shape (membership (recursive-suffix none))))))) (package (name "Security Features") (body brace (import (target (span (span (offset 584) (line 28) (column 17) (len 28))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 630) (line 29) (column 17) (len 35))) (all none) (ref r4) (shape (membership (recursive-suffix none))))))) (package (name "Safety & Security Features") (body brace (import (target (span (span (offset 819) (line 37) (column 17) (len 20))) (all none) (ref r5) (shape (namespace (wildcard-suffix (span (span (offset 836) (line 37) (column 34) (len 3))) (separator (span (offset 836) (line 37) (column 34) (len 2))) (marker (span (offset 838) (line 37) (column 36) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 857) (line 38) (column 17) (len 22))) (all none) (ref r6) (shape (namespace (wildcard-suffix (span (span (offset 876) (line 38) (column 36) (len 3))) (separator (span (offset 876) (line 38) (column 36) (len 2))) (marker (span (offset 878) (line 38) (column 38) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))))))))
)
~~~
