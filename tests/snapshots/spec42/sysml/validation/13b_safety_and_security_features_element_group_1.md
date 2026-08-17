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
    (reference r3 (scope relative) (span (offset 835) (line 35) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 835) (line 35) (column 17) (len 7)))))
    (reference r4 (scope relative) (span (offset 963) (line 41) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 963) (line 41) (column 17) (len 7)))))
    (reference r5 (scope relative) (span (offset 1113) (line 47) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1113) (line 47) (column 17) (len 7)))))
    (reference r6 (scope relative) (span (offset 1276) (line 53) (column 17) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1276) (line 53) (column 17) (len 7)))))
  )
  (root (package (name "13b-Safety and Security Features Element Group-1") (body brace (import (target (span (span (offset 77) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 89) (line 2) (column 29) (len 3))) (separator (span (offset 89) (line 2) (column 29) (len 2))) (marker (span (offset 91) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 110) (line 3) (column 17) (len 24))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 131) (line 3) (column 38) (len 3))) (separator (span (offset 131) (line 3) (column 38) (len 2))) (marker (span (offset 133) (line 3) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 152) (line 4) (column 17) (len 12))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 161) (line 4) (column 26) (len 3))) (separator (span (offset 161) (line 4) (column 26) (len 2))) (marker (span (offset 163) (line 4) (column 28) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "AnnotationDefinitions") (body brace (metadata-def) (metadata-def))) (package (name "PartsTree") (body brace (part-usage (declaration-name "vehicle") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body brace (part-usage) (part-usage) (part-usage))))) (package (name "Safety Features") (body brace (import (target (span (span (offset 835) (line 35) (column 17) (len 11))) (all none) (ref r3) (shape (membership (recursive-suffix (span (span (offset 842) (line 35) (column 24) (len 4))) (separator (span (offset 842) (line 35) (column 24) (len 2))) (marker (span (offset 844) (line 35) (column 26) (len 2)))))))) (filter))) (package (name "Security Features") (body brace (import (target (span (span (offset 963) (line 41) (column 17) (len 11))) (all none) (ref r4) (shape (membership (recursive-suffix (span (span (offset 970) (line 41) (column 24) (len 4))) (separator (span (offset 970) (line 41) (column 24) (len 2))) (marker (span (offset 972) (line 41) (column 26) (len 2)))))))) (filter))) (package (name "Safety & Security Features") (body brace (import (target (span (span (offset 1113) (line 47) (column 17) (len 11))) (all none) (ref r5) (shape (membership (recursive-suffix (span (span (offset 1120) (line 47) (column 24) (len 4))) (separator (span (offset 1120) (line 47) (column 24) (len 2))) (marker (span (offset 1122) (line 47) (column 26) (len 2)))))))) (filter))) (package (name "Mandatory Safety Features") (body brace (import (target (span (span (offset 1276) (line 53) (column 17) (len 11))) (all none) (ref r6) (shape (membership (recursive-suffix (span (span (offset 1283) (line 53) (column 24) (len 4))) (separator (span (offset 1283) (line 53) (column 24) (len 2))) (marker (span (offset 1285) (line 53) (column 26) (len 2)))))))) (filter))))))
)
~~~
