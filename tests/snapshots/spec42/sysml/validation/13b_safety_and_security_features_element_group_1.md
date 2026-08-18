# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (13-Model Containment): 13b-Safety and Security Features Element Group-1"))
~~~
# SOURCE
~~~sysml
package '13b-Safety and Security Features Element Group-1' {
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
		public import vehicle::**;
		filter @Safety;
	}
	
	package 'Security Features' {
		/* Parts that contribute to security. */		
		public import vehicle::**;
		filter @Security;
	}
	
	package 'Safety & Security Features' {
		/* Parts that contribute to safety OR security. */		 
		public import vehicle::**;
		filter @Safety or @Security;
	}
	
	package 'Mandatory Safety Features' {
		/* Parts that contribute to safety AND are mandatory. */
		public import vehicle::**;
		filter @Safety and Safety::isMandatory;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "13b_safety_and_security_features_element_group_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '13b-Safety and Security Features Element Group-1' {
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
        public import vehicle::**;
        filter @Safety;
    }
    package 'Security Features' {
        public import vehicle::**;
        filter @Security;
    }
    package 'Safety & Security Features' {
        public import vehicle::**;
        filter @Safety || @Security;
    }
    package 'Mandatory Safety Features' {
        public import vehicle::**;
        filter @Safety && Safety::isMandatory;
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
    (reference r3 (scope relative) (span (offset 369) (line 16) (column 18) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 369) (line 16) (column 18) (len 8)))))
    (reference r4 (scope relative) (span (offset 403) (line 17) (column 24) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 403) (line 17) (column 24) (len 6)))))
    (reference r5 (scope relative) (span (offset 479) (line 19) (column 25) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 479) (line 19) (column 25) (len 6)))))
    (reference r6 (scope relative) (span (offset 566) (line 23) (column 19) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 566) (line 23) (column 19) (len 6)))))
    (reference r7 (scope relative) (span (offset 619) (line 24) (column 25) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 619) (line 24) (column 25) (len 8)))))
    (reference r8 (scope relative) (span (offset 703) (line 28) (column 30) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 703) (line 28) (column 30) (len 6)))))
    (reference r9 (scope relative) (span (offset 835) (line 35) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 835) (line 35) (column 17) (len 7)))))
    (reference r10 (scope relative) (span (offset 963) (line 41) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 963) (line 41) (column 17) (len 7)))))
    (reference r11 (scope relative) (span (offset 1113) (line 47) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1113) (line 47) (column 17) (len 7)))))
    (reference r12 (scope relative) (span (offset 1276) (line 53) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1276) (line 53) (column 17) (len 7)))))
  )
  (root (package (name "13b-Safety and Security Features Element Group-1") (body brace (import (target (span (span (offset 77) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 89) (line 2) (column 29) (len 3))) (separator (span (offset 89) (line 2) (column 29) (len 2))) (marker (span (offset 91) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 110) (line 3) (column 17) (len 24))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 131) (line 3) (column 38) (len 3))) (separator (span (offset 131) (line 3) (column 38) (len 2))) (marker (span (offset 133) (line 3) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 152) (line 4) (column 17) (len 12))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 161) (line 4) (column 26) (len 3))) (separator (span (offset 161) (line 4) (column 26) (len 2))) (marker (span (offset 163) (line 4) (column 28) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "AnnotationDefinitions") (body brace (metadata-def) (metadata-def))) (package (name "PartsTree") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "interior") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "alarm") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r3)) (about) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "seatBelt") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 398) (line 17) (column 19) (len 1)) (integer 2))) (upper (expression (span (offset 398) (line 17) (column 19) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r4)) (about) (body brace (element-count 1))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontSeat") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 451) (line 18) (column 20) (len 1)) (integer 2))) (upper (expression (span (offset 451) (line 18) (column 20) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "driverAirBag") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r5)) (about) (body brace (element-count 1))))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bodyAssy") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "body") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bumper") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r6)) (about) (body brace (element-count 1))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "keylessEntry") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r7)) (about) (body semicolon)))))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheelAssy") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheel") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 670) (line 27) (column 16) (len 1)) (integer 2))) (upper (expression (span (offset 670) (line 27) (column 16) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "antilockBrakes") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 698) (line 28) (column 25) (len 1)) (integer 2))) (upper (expression (span (offset 698) (line 28) (column 25) (len 1)) (integer 2)))) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r8)) (about) (body brace (element-count 1))))))))))) (package (name "Safety Features") (body brace (import (target (span (span (offset 835) (line 35) (column 17) (len 11))) (all none) (ref r9) (shape (membership (recursive-suffix (span (span (offset 842) (line 35) (column 24) (len 4))) (separator (span (offset 842) (line 35) (column 24) (len 2))) (marker (span (offset 844) (line 35) (column 26) (len 2)))))))) (filter))) (package (name "Security Features") (body brace (import (target (span (span (offset 963) (line 41) (column 17) (len 11))) (all none) (ref r10) (shape (membership (recursive-suffix (span (span (offset 970) (line 41) (column 24) (len 4))) (separator (span (offset 970) (line 41) (column 24) (len 2))) (marker (span (offset 972) (line 41) (column 26) (len 2)))))))) (filter))) (package (name "Safety & Security Features") (body brace (import (target (span (span (offset 1113) (line 47) (column 17) (len 11))) (all none) (ref r11) (shape (membership (recursive-suffix (span (span (offset 1120) (line 47) (column 24) (len 4))) (separator (span (offset 1120) (line 47) (column 24) (len 2))) (marker (span (offset 1122) (line 47) (column 26) (len 2)))))))) (filter))) (package (name "Mandatory Safety Features") (body brace (import (target (span (span (offset 1276) (line 53) (column 17) (len 11))) (all none) (ref r12) (shape (membership (recursive-suffix (span (span (offset 1283) (line 53) (column 24) (len 4))) (separator (span (offset 1283) (line 53) (column 24) (len 2))) (marker (span (offset 1285) (line 53) (column 26) (len 2)))))))) (filter))))))
)
~~~
