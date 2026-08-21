# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (11-View and Viewpoint): 11b-Safety and Security Feature Views"))
~~~
# SOURCE
~~~sysml
private import Views::*; // private import library package, not internal Views package!
package '11b-Safety and Security Feaure Views' {
	private import ScalarValues::*;
	
	package AnnotationDefinitions {	
		metadata def Safety {
			attribute isMandatory : Boolean;
		}
		metadata def Security;
	}
	
	package PartsTree {
		public import AnnotationDefinitions::*;
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

	package ViewDefinitions {	
		public import AnnotationDefinitions::*;
		view def SafetyFeatureView {
			/* Parts that contribute to safety. */		
			filter @Safety;
			render asTreeDiagram;
		}
		
		view def SafetyOrSecurityFeatureView {
			/* Parts that contribute to safety OR security. */		 
			filter @Safety | @Security;
		}	
	}
	
	package Views {
		private import ViewDefinitions::*;
		private import PartsTree::vehicle;
		
		view vehicleSafetyFeatureView : SafetyFeatureView {
			expose vehicle;
		}
		
		view vehicleMandatorySafetyFeatureView :> vehicleSafetyFeatureView {
		    expose vehicle::*::**;
			filter Safety::isMandatory;
		}
		
		view vehicleMandatorySafetyFeatureViewStandalone {
			expose vehicle::**[@Safety and Safety::isMandatory];
			render asElementTable;
		}	
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "11b_safety_and_security_feature_views.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
private import Views::*;

package '11b-Safety and Security Feaure Views' {
    private import ScalarValues::*;
    package AnnotationDefinitions {
        metadata def Safety {
            attribute isMandatory : Boolean;
        }
        metadata def Security;
    }
    package PartsTree {
        public import AnnotationDefinitions::*;
        part vehicle {
            part interior {
                part alarm {
                    @Security;
                }
                part seatBelt[2] {
                    @Safety {
                        isMandatory = true;
                    }
                }
                part frontSeat[2];
                part driverAirBag {
                    @Safety {
                        isMandatory = false;
                    }
                }
            }
            part bodyAssy {
                part body;
                part bumper {
                    @Safety {
                        isMandatory = true;
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
                        isMandatory = false;
                    }
                }
            }
        }
    }
    package ViewDefinitions {
        public import AnnotationDefinitions::*;
        view def SafetyFeatureView {
            /* Parts that contribute to safety. */
            filter @Safety;
            render asTreeDiagram;
        }
        view def SafetyOrSecurityFeatureView {
            /* Parts that contribute to safety OR security. */
            filter @Safety | @Security;
        }
    }
    package Views {
        private import ViewDefinitions::*;
        private import PartsTree::vehicle;
        view vehicleSafetyFeatureView : SafetyFeatureView {
            expose vehicle;
        }
        view vehicleMandatorySafetyFeatureView :> vehicleSafetyFeatureView {
            expose vehicle::*::**;
            filter Safety::isMandatory;
        }
        view vehicleMandatorySafetyFeatureViewStandalone {
            expose vehicle::** [@Safety && Safety::isMandatory];
            render asElementTable;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 15) (line 1) (column 16) (len 5)) (segments (segment 0 (token "Views") (name "Views") (separator none) (span (offset 15) (line 1) (column 16) (len 5)))))
    (reference r1 (scope relative) (span (offset 153) (line 3) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 153) (line 3) (column 17) (len 12)))))
    (reference r2 (scope relative) (span (offset 257) (line 7) (column 28) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 257) (line 7) (column 28) (len 7)))))
    (reference r3 (scope relative) (span (offset 337) (line 13) (column 17) (len 21)) (segments (segment 0 (token "AnnotationDefinitions") (name "AnnotationDefinitions") (separator none) (span (offset 337) (line 13) (column 17) (len 21)))))
    (reference r4 (scope relative) (span (offset 416) (line 16) (column 18) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 416) (line 16) (column 18) (len 8)))))
    (reference r5 (scope relative) (span (offset 450) (line 17) (column 24) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 450) (line 17) (column 24) (len 6)))))
    (reference r6 (scope relative) (span (offset 457) (line 17) (column 31) (len 11)) (segments (segment 0 (token "isMandatory") (name "isMandatory") (separator none) (span (offset 457) (line 17) (column 31) (len 11)))))
    (reference r7 (scope relative) (span (offset 526) (line 19) (column 25) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 526) (line 19) (column 25) (len 6)))))
    (reference r8 (scope relative) (span (offset 533) (line 19) (column 32) (len 11)) (segments (segment 0 (token "isMandatory") (name "isMandatory") (separator none) (span (offset 533) (line 19) (column 32) (len 11)))))
    (reference r9 (scope relative) (span (offset 613) (line 23) (column 19) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 613) (line 23) (column 19) (len 6)))))
    (reference r10 (scope relative) (span (offset 620) (line 23) (column 26) (len 11)) (segments (segment 0 (token "isMandatory") (name "isMandatory") (separator none) (span (offset 620) (line 23) (column 26) (len 11)))))
    (reference r11 (scope relative) (span (offset 666) (line 24) (column 25) (len 8)) (segments (segment 0 (token "Security") (name "Security") (separator none) (span (offset 666) (line 24) (column 25) (len 8)))))
    (reference r12 (scope relative) (span (offset 750) (line 28) (column 30) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 750) (line 28) (column 30) (len 6)))))
    (reference r13 (scope relative) (span (offset 757) (line 28) (column 37) (len 11)) (segments (segment 0 (token "isMandatory") (name "isMandatory") (separator none) (span (offset 757) (line 28) (column 37) (len 11)))))
    (reference r14 (scope relative) (span (offset 837) (line 34) (column 17) (len 21)) (segments (segment 0 (token "AnnotationDefinitions") (name "AnnotationDefinitions") (separator none) (span (offset 837) (line 34) (column 17) (len 21)))))
    (reference r15 (scope relative) (span (offset 1162) (line 48) (column 18) (len 15)) (segments (segment 0 (token "ViewDefinitions") (name "ViewDefinitions") (separator none) (span (offset 1162) (line 48) (column 18) (len 15)))))
    (reference r16 (scope relative) (span (offset 1199) (line 49) (column 18) (len 18)) (segments (segment 0 (token "PartsTree") (name "PartsTree") (separator none) (span (offset 1199) (line 49) (column 18) (len 9))) (segment 1 (token "vehicle") (name "vehicle") (separator colon-colon) (span (offset 1210) (line 49) (column 29) (len 7)))))
    (reference r17 (scope relative) (span (offset 1256) (line 51) (column 35) (len 17)) (segments (segment 0 (token "SafetyFeatureView") (name "SafetyFeatureView") (separator none) (span (offset 1256) (line 51) (column 35) (len 17)))))
    (reference r18 (scope relative) (span (offset 1286) (line 52) (column 11) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1286) (line 52) (column 11) (len 7)))))
    (reference r19 (scope relative) (span (offset 1386) (line 56) (column 14) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1386) (line 56) (column 14) (len 7)))))
    (reference r20 (scope relative) (span (offset 1503) (line 61) (column 11) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1503) (line 61) (column 11) (len 7)))))
    (reference r21 (scope relative) (span (offset 1516) (line 61) (column 24) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 1516) (line 61) (column 24) (len 6)))))
    (reference r22 (scope relative) (span (offset 1527) (line 61) (column 35) (len 19)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 1527) (line 61) (column 35) (len 6))) (segment 1 (token "isMandatory") (name "isMandatory") (separator colon-colon) (span (offset 1535) (line 61) (column 43) (len 11)))))
  )
  (root (import (target (span (span (offset 15) (line 1) (column 16) (len 8))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 20) (line 1) (column 21) (len 3))) (separator (span (offset 20) (line 1) (column 21) (len 2))) (marker (span (offset 22) (line 1) (column 23) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "11b-Safety and Security Feaure Views") (body brace (import (target (span (span (offset 153) (line 3) (column 17) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 165) (line 3) (column 29) (len 3))) (separator (span (offset 165) (line 3) (column 29) (len 2))) (marker (span (offset 167) (line 3) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "AnnotationDefinitions") (body brace (metadata-def (name "Safety") (abstract false) (specializes none) (body brace (attribute-usage (declaration-name "isMandatory") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (metadata-def (name "Security") (abstract false) (specializes none) (body semicolon)))) (package (name "PartsTree") (body brace (import (target (span (span (offset 337) (line 13) (column 17) (len 24))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 358) (line 13) (column 38) (len 3))) (separator (span (offset 358) (line 13) (column 38) (len 2))) (marker (span (offset 360) (line 13) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "interior") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "alarm") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r4)) (about) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "seatBelt") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 445) (line 17) (column 19) (len 1)) (integer 2))) (upper (expression (span (offset 445) (line 17) (column 19) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r5)) (about) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r6)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 471) (line 17) (column 45) (len 4)) (boolean true))))) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontSeat") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 498) (line 18) (column 20) (len 1)) (integer 2))) (upper (expression (span (offset 498) (line 18) (column 20) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "driverAirBag") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r7)) (about) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r8)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 547) (line 19) (column 46) (len 5)) (boolean false))))) (body semicolon)))))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bodyAssy") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "body") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bumper") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r9)) (about) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r10)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 634) (line 23) (column 40) (len 4)) (boolean true))))) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "keylessEntry") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r11)) (about) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheelAssy") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheel") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 717) (line 27) (column 16) (len 1)) (integer 2))) (upper (expression (span (offset 717) (line 27) (column 16) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "antilockBrakes") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 745) (line 28) (column 25) (len 1)) (integer 2))) (upper (expression (span (offset 745) (line 28) (column 25) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r12)) (about) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r13)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 771) (line 28) (column 51) (len 5)) (boolean false))))) (body semicolon)))))))))))) (package (name "ViewDefinitions") (body brace (import (target (span (span (offset 837) (line 34) (column 17) (len 24))) (all none) (ref r14) (shape (namespace (wildcard-suffix (span (span (offset 858) (line 34) (column 38) (len 3))) (separator (span (offset 858) (line 34) (column 38) (len 2))) (marker (span (offset 860) (line 34) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (view-def (name "SafetyFeatureView") (short-name none) (modifiers) (specializes none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 899) (line 36) (column 6) (len 34)) (normalized "Parts that contribute to safety. "))) (filter) (view-rendering))) (view-def (name "SafetyOrSecurityFeatureView") (short-name none) (modifiers) (specializes none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1035) (line 42) (column 6) (len 46)) (normalized "Parts that contribute to safety OR security. "))) (filter))))) (package (name "Views") (body brace (import (target (span (span (offset 1162) (line 48) (column 18) (len 18))) (all none) (ref r15) (shape (namespace (wildcard-suffix (span (span (offset 1177) (line 48) (column 33) (len 3))) (separator (span (offset 1177) (line 48) (column 33) (len 2))) (marker (span (offset 1179) (line 48) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1199) (line 49) (column 18) (len 18))) (all none) (ref r16) (shape (membership (recursive-suffix none))))) (view (name "vehicleSafetyFeatureView") (short-name none) (type (ref r17)) (body brace (expose (target (span (span (offset 1286) (line 52) (column 11) (len 7))) (all none) (ref r18) (shape (membership (recursive-suffix none)))) (body semicolon)))) (view (name "vehicleMandatorySafetyFeatureView") (short-name none) (type none) (body brace (expose (target (span (span (offset 1386) (line 56) (column 14) (len 14))) (all none) (ref r19) (shape (namespace (wildcard-suffix (span (span (offset 1393) (line 56) (column 21) (len 3))) (separator (span (offset 1393) (line 56) (column 21) (len 2))) (marker (span (offset 1395) (line 56) (column 23) (len 1)))) (recursive-suffix (span (span (offset 1396) (line 56) (column 24) (len 4))) (separator (span (offset 1396) (line 56) (column 24) (len 2))) (marker (span (offset 1398) (line 56) (column 26) (len 2)))) (combined-recursive-suffix-span (span (offset 1393) (line 56) (column 21) (len 7)))))) (body semicolon)) (filter))) (view (name "vehicleMandatorySafetyFeatureViewStandalone") (short-name none) (type none) (body brace (expose (target (span (span (offset 1503) (line 61) (column 11) (len 44))) (all none) (ref r20) (shape (filter (recursive-suffix (span (span (offset 1510) (line 61) (column 18) (len 4))) (separator (span (offset 1510) (line 61) (column 18) (len 2))) (marker (span (offset 1512) (line 61) (column 20) (len 2)))) (members (filter-member (span (span (offset 1514) (line 61) (column 22) (len 33))) (open (span (offset 1514) (line 61) (column 22) (len 1))) (expression (expression (span (offset 1515) (line 61) (column 23) (len 31)) (binary (operator "&&") (left (expression (span (offset 1515) (line 61) (column 23) (len 7)) (classification (metaclass (ref r21))))) (right (expression (span (offset 1527) (line 61) (column 35) (len 19)) (ref r22)))))) (close (span (offset 1546) (line 61) (column 54) (len 1)))))))) (body semicolon)) (view-rendering))))))))
)
~~~
