# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (13-Model Containment): 13b-Safety and Security Features Element Group-2"))
~~~
# SOURCE
~~~sysml
package '13b-Safety and Security Features Element Group-2' {
	private import ScalarValues::*;
	private import AnnotationDefinitions::*;
	private import PartsTree::*;
	
	package AnnotationDefinitions {
		metadata def Safety {
			attribute isMandatory : Boolean;
		}
		metadata def Security;
	}
	
	package PartsTree {
		part vehicle {
			part interior {
				part alarm {@Security;}
				part seatBelt[2] {@Safety{isMandatory = true;}}
				part frontSeat[2];
				part driverAirBag {@Safety{isMandatory = false;}}
			}
			part bodyAssy {
				part body;
				part bumper {@Safety{isMandatory = true;}}
				part keylessEntry {@Security;}
			}
			part wheelAssy {
				part wheel[2];
				part antilockBrakes[2] {@Safety{isMandatory = false;}}
			}
		}
	}
	
	package 'Safety Features' {
		/* Parts that contribute to safety. */		
		public import vehicle::**[@Safety];
	}
	
	package 'Security Features' {
		/* Parts that contribute to security. */		
		public import vehicle::**[@Security];
	}
	
	package 'Safety & Security Features' {
		/* Parts that contribute to safety OR security. */		 
		public import vehicle::**[@Safety or @Security];
	}
	
	package 'Mandatory Saftey Features' {
		/* Parts that contribute to safety AND are mandatory. */
		public import vehicle::**[@Safety and Safety::isMandatory];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "13b_safety_and_security_features_element_group_2.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '13b-Safety and Security Features Element Group-2' {
    private import ScalarValues::*;
    private import AnnotationDefinitions::*;
    private import PartsTree::*;
    package AnnotationDefinitions {
        metadata def Safety {
            attribute isMandatory : Boolean;
        }
        metadata def Security;
    }
    package PartsTree {
        part vehicle {
            part interior {
                part alarm {
                    @Security;
                }
                part seatBelt[2] {
                    @Safety {
                        attribute isMandatory = true;
                    }
                }
                part frontSeat[2];
                part driverAirBag {
                    @Safety {
                        attribute isMandatory = false;
                    }
                }
            }
            part bodyAssy {
                part body;
                part bumper {
                    @Safety {
                        attribute isMandatory = true;
                    }
                }
                part keylessEntry {
                    @Security;
                }
            }
            part wheelAssy {
                part wheel[2];
                part antilockBrakes[2] {
                    @Safety {
                        attribute isMandatory = false;
                    }
                }
            }
        }
    }
    package 'Safety Features' {
        public import vehicle::** [@Safety];
    }
    package 'Security Features' {
        public import vehicle::** [@Security];
    }
    package 'Safety & Security Features' {
        public import vehicle::** [@Safety || @Security];
    }
    package 'Mandatory Saftey Features' {
        public import vehicle::** [@Safety && Safety::isMandatory];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 77) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 77) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 110) (line 3) (column 17) (len 21)) (segments (segment 0 (token "AnnotationDefinitions") (name "AnnotationDefinitions") (separator none) (span (offset 110) (line 3) (column 17) (len 21)))))
    (reference r2 (scope relative) (span (offset 152) (line 4) (column 17) (len 9)) (segments (segment 0 (token "PartsTree") (name "PartsTree") (separator none) (span (offset 152) (line 4) (column 17) (len 9)))))
    (reference r3 (scope relative) (span (offset 252) (line 8) (column 28) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 252) (line 8) (column 28) (len 7)))))
    (reference r4 (scope relative) (span (offset 369) (line 16) (column 18) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 369) (line 16) (column 18) (len 8)))))
    (reference r5 (scope relative) (span (offset 403) (line 17) (column 24) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 403) (line 17) (column 24) (len 6)))))
    (reference r6 (scope relative) (span (offset 479) (line 19) (column 25) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 479) (line 19) (column 25) (len 6)))))
    (reference r7 (scope relative) (span (offset 566) (line 23) (column 19) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 566) (line 23) (column 19) (len 6)))))
    (reference r8 (scope relative) (span (offset 619) (line 24) (column 25) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 619) (line 24) (column 25) (len 8)))))
    (reference r9 (scope relative) (span (offset 703) (line 28) (column 30) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 703) (line 28) (column 30) (len 6)))))
    (reference r10 (scope relative) (span (offset 835) (line 35) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 835) (line 35) (column 17) (len 7)))))
    (reference r11 (scope relative) (span (offset 848) (line 35) (column 30) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 848) (line 35) (column 30) (len 6)))))
    (reference r12 (scope relative) (span (offset 954) (line 40) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 954) (line 40) (column 17) (len 7)))))
    (reference r13 (scope relative) (span (offset 967) (line 40) (column 30) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 967) (line 40) (column 30) (len 8)))))
    (reference r14 (scope relative) (span (offset 1095) (line 45) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1095) (line 45) (column 17) (len 7)))))
    (reference r15 (scope relative) (span (offset 1108) (line 45) (column 30) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 1108) (line 45) (column 30) (len 6)))))
    (reference r16 (scope relative) (span (offset 1119) (line 45) (column 41) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 1119) (line 45) (column 41) (len 8)))))
    (reference r17 (scope relative) (span (offset 1249) (line 50) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1249) (line 50) (column 17) (len 7)))))
    (reference r18 (scope relative) (span (offset 1262) (line 50) (column 30) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 1262) (line 50) (column 30) (len 6)))))
    (reference r19 (scope relative) (span (offset 1273) (line 50) (column 41) (len 19)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 1273) (line 50) (column 41) (len 6))) (segment 1 (token "isMandatory") (name "isMandatory") (separator colon-colon) (span (offset 1281) (line 50) (column 49) (len 11)))))
  )
  (root (package (name "13b-Safety and Security Features Element Group-2") (body brace (import (target (span (span (offset 77) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 89) (line 2) (column 29) (len 3))) (separator (span (offset 89) (line 2) (column 29) (len 2))) (marker (span (offset 91) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 110) (line 3) (column 17) (len 24))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 131) (line 3) (column 38) (len 3))) (separator (span (offset 131) (line 3) (column 38) (len 2))) (marker (span (offset 133) (line 3) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 152) (line 4) (column 17) (len 12))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 161) (line 4) (column 26) (len 3))) (separator (span (offset 161) (line 4) (column 26) (len 2))) (marker (span (offset 163) (line 4) (column 28) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "AnnotationDefinitions") (body brace (metadata-def (name "Safety") (abstract false) (specializes none) (body brace (attribute-usage (declaration-name "isMandatory") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (metadata-def (name "Security") (abstract false) (specializes none) (body semicolon)))) (package (name "PartsTree") (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "interior") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "alarm") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r4)) (about) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "seatBelt") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 398) (line 17) (column 19) (len 1)) (integer 2))) (upper (expression (span (offset 398) (line 17) (column 19) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r5)) (about) (body brace (attribute-usage (declaration-name "isMandatory") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 424) (line 17) (column 45) (len 4)) (boolean true))))) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontSeat") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 451) (line 18) (column 20) (len 1)) (integer 2))) (upper (expression (span (offset 451) (line 18) (column 20) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "driverAirBag") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r6)) (about) (body brace (attribute-usage (declaration-name "isMandatory") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 500) (line 19) (column 46) (len 5)) (boolean false))))) (body semicolon)))))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bodyAssy") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "body") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bumper") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r7)) (about) (body brace (attribute-usage (declaration-name "isMandatory") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 587) (line 23) (column 40) (len 4)) (boolean true))))) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "keylessEntry") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r8)) (about) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheelAssy") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheel") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 670) (line 27) (column 16) (len 1)) (integer 2))) (upper (expression (span (offset 670) (line 27) (column 16) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "antilockBrakes") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 698) (line 28) (column 25) (len 1)) (integer 2))) (upper (expression (span (offset 698) (line 28) (column 25) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r9)) (about) (body brace (attribute-usage (declaration-name "isMandatory") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 724) (line 28) (column 51) (len 5)) (boolean false))))) (body semicolon)))))))))))) (package (name "Safety Features") (body brace (import (target (span (span (offset 835) (line 35) (column 17) (len 20))) (all none) (ref r10) (shape (filter (recursive-suffix (span (span (offset 842) (line 35) (column 24) (len 4))) (separator (span (offset 842) (line 35) (column 24) (len 2))) (marker (span (offset 844) (line 35) (column 26) (len 2)))) (members (filter-member (span (span (offset 846) (line 35) (column 28) (len 9))) (open (span (offset 846) (line 35) (column 28) (len 1))) (expression (expression (span (offset 847) (line 35) (column 29) (len 7)) (classification (metaclass (ref r11))))) (close (span (offset 854) (line 35) (column 36) (len 1))))))))))) (package (name "Security Features") (body brace (import (target (span (span (offset 954) (line 40) (column 17) (len 22))) (all none) (ref r12) (shape (filter (recursive-suffix (span (span (offset 961) (line 40) (column 24) (len 4))) (separator (span (offset 961) (line 40) (column 24) (len 2))) (marker (span (offset 963) (line 40) (column 26) (len 2)))) (members (filter-member (span (span (offset 965) (line 40) (column 28) (len 11))) (open (span (offset 965) (line 40) (column 28) (len 1))) (expression (expression (span (offset 966) (line 40) (column 29) (len 9)) (classification (metaclass (ref r13))))) (close (span (offset 975) (line 40) (column 38) (len 1))))))))))) (package (name "Safety & Security Features") (body brace (import (target (span (span (offset 1095) (line 45) (column 17) (len 33))) (all none) (ref r14) (shape (filter (recursive-suffix (span (span (offset 1102) (line 45) (column 24) (len 4))) (separator (span (offset 1102) (line 45) (column 24) (len 2))) (marker (span (offset 1104) (line 45) (column 26) (len 2)))) (members (filter-member (span (span (offset 1106) (line 45) (column 28) (len 22))) (open (span (offset 1106) (line 45) (column 28) (len 1))) (expression (expression (span (offset 1107) (line 45) (column 29) (len 20)) (binary (operator "||") (left (expression (span (offset 1107) (line 45) (column 29) (len 7)) (classification (metaclass (ref r15))))) (right (expression (span (offset 1118) (line 45) (column 40) (len 9)) (classification (metaclass (ref r16)))))))) (close (span (offset 1127) (line 45) (column 49) (len 1))))))))))) (package (name "Mandatory Saftey Features") (body brace (import (target (span (span (offset 1249) (line 50) (column 17) (len 44))) (all none) (ref r17) (shape (filter (recursive-suffix (span (span (offset 1256) (line 50) (column 24) (len 4))) (separator (span (offset 1256) (line 50) (column 24) (len 2))) (marker (span (offset 1258) (line 50) (column 26) (len 2)))) (members (filter-member (span (span (offset 1260) (line 50) (column 28) (len 33))) (open (span (offset 1260) (line 50) (column 28) (len 1))) (expression (expression (span (offset 1261) (line 50) (column 29) (len 31)) (binary (operator "&&") (left (expression (span (offset 1261) (line 50) (column 29) (len 7)) (classification (metaclass (ref r18))))) (right (expression (span (offset 1273) (line 50) (column 41) (len 19)) (ref r19)))))) (close (span (offset 1292) (line 50) (column 60) (len 1))))))))))))))
)
~~~
