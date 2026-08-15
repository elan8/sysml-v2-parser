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
    (reference r3 (scope relative) (span (offset 835) (line 35) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 835) (line 35) (column 17) (len 7)))))
    (reference r4 (scope relative) (span (offset 848) (line 35) (column 30) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 848) (line 35) (column 30) (len 6)))))
    (reference r5 (scope relative) (span (offset 954) (line 40) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 954) (line 40) (column 17) (len 7)))))
    (reference r6 (scope relative) (span (offset 967) (line 40) (column 30) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 967) (line 40) (column 30) (len 8)))))
    (reference r7 (scope relative) (span (offset 1095) (line 45) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1095) (line 45) (column 17) (len 7)))))
    (reference r8 (scope relative) (span (offset 1108) (line 45) (column 30) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 1108) (line 45) (column 30) (len 6)))))
    (reference r9 (scope relative) (span (offset 1119) (line 45) (column 41) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 1119) (line 45) (column 41) (len 8)))))
    (reference r10 (scope relative) (span (offset 1249) (line 50) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1249) (line 50) (column 17) (len 7)))))
    (reference r11 (scope relative) (span (offset 1262) (line 50) (column 30) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 1262) (line 50) (column 30) (len 6)))))
    (reference r12 (scope relative) (span (offset 1273) (line 50) (column 41) (len 19)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 1273) (line 50) (column 41) (len 6))) (segment 1 (token "isMandatory") (name "isMandatory") (separator colon-colon) (span (offset 1281) (line 50) (column 49) (len 11)))))
  )
  (root (package (name "13b-Safety and Security Features Element Group-2") (body brace (import (target (span (span (offset 77) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 89) (line 2) (column 29) (len 3))) (separator (span (offset 89) (line 2) (column 29) (len 2))) (marker (span (offset 91) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 110) (line 3) (column 17) (len 24))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 131) (line 3) (column 38) (len 3))) (separator (span (offset 131) (line 3) (column 38) (len 2))) (marker (span (offset 133) (line 3) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 152) (line 4) (column 17) (len 12))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 161) (line 4) (column 26) (len 3))) (separator (span (offset 161) (line 4) (column 26) (len 2))) (marker (span (offset 163) (line 4) (column 28) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "AnnotationDefinitions") (body brace (metadata-def) (metadata-def))) (package (name "PartsTree") (body brace (part-usage))) (package (name "Safety Features") (body brace (import (target (span (span (offset 835) (line 35) (column 17) (len 20))) (all none) (ref r3) (shape (filter (recursive-suffix (span (span (offset 842) (line 35) (column 24) (len 4))) (separator (span (offset 842) (line 35) (column 24) (len 2))) (marker (span (offset 844) (line 35) (column 26) (len 2)))) (members (filter-member (span (span (offset 846) (line 35) (column 28) (len 9))) (open (span (offset 846) (line 35) (column 28) (len 1))) (expression (expression (span (offset 847) (line 35) (column 29) (len 7)) (classification (metaclass (ref r4))))) (close (span (offset 854) (line 35) (column 36) (len 1))))))))))) (package (name "Security Features") (body brace (import (target (span (span (offset 954) (line 40) (column 17) (len 22))) (all none) (ref r5) (shape (filter (recursive-suffix (span (span (offset 961) (line 40) (column 24) (len 4))) (separator (span (offset 961) (line 40) (column 24) (len 2))) (marker (span (offset 963) (line 40) (column 26) (len 2)))) (members (filter-member (span (span (offset 965) (line 40) (column 28) (len 11))) (open (span (offset 965) (line 40) (column 28) (len 1))) (expression (expression (span (offset 966) (line 40) (column 29) (len 9)) (classification (metaclass (ref r6))))) (close (span (offset 975) (line 40) (column 38) (len 1))))))))))) (package (name "Safety & Security Features") (body brace (import (target (span (span (offset 1095) (line 45) (column 17) (len 33))) (all none) (ref r7) (shape (filter (recursive-suffix (span (span (offset 1102) (line 45) (column 24) (len 4))) (separator (span (offset 1102) (line 45) (column 24) (len 2))) (marker (span (offset 1104) (line 45) (column 26) (len 2)))) (members (filter-member (span (span (offset 1106) (line 45) (column 28) (len 22))) (open (span (offset 1106) (line 45) (column 28) (len 1))) (expression (expression (span (offset 1107) (line 45) (column 29) (len 20)) (binary (operator "||") (left (expression (span (offset 1107) (line 45) (column 29) (len 7)) (classification (metaclass (ref r8))))) (right (expression (span (offset 1118) (line 45) (column 40) (len 9)) (classification (metaclass (ref r9)))))))) (close (span (offset 1127) (line 45) (column 49) (len 1))))))))))) (package (name "Mandatory Saftey Features") (body brace (import (target (span (span (offset 1249) (line 50) (column 17) (len 44))) (all none) (ref r10) (shape (filter (recursive-suffix (span (span (offset 1256) (line 50) (column 24) (len 4))) (separator (span (offset 1256) (line 50) (column 24) (len 2))) (marker (span (offset 1258) (line 50) (column 26) (len 2)))) (members (filter-member (span (span (offset 1260) (line 50) (column 28) (len 33))) (open (span (offset 1260) (line 50) (column 28) (len 1))) (expression (expression (span (offset 1261) (line 50) (column 29) (len 31)) (binary (operator "&&") (left (expression (span (offset 1261) (line 50) (column 29) (len 7)) (classification (metaclass (ref r11))))) (right (expression (span (offset 1273) (line 50) (column 41) (len 19)) (ref r12)))))) (close (span (offset 1292) (line 50) (column 60) (len 1))))))))))))))
)
~~~
