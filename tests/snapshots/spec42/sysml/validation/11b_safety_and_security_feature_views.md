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
    package ViewDefinitions {
        public import AnnotationDefinitions::*;
        view def SafetyFeatureView {
            filter @Safety;
            render asTreeDiagram;
        }
        view def SafetyOrSecurityFeatureView {
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
    (reference r2 (scope relative) (span (offset 337) (line 13) (column 17) (len 21)) (segments (segment 0 (token "AnnotationDefinitions") (name "AnnotationDefinitions") (separator none) (span (offset 337) (line 13) (column 17) (len 21)))))
    (reference r3 (scope relative) (span (offset 837) (line 34) (column 17) (len 21)) (segments (segment 0 (token "AnnotationDefinitions") (name "AnnotationDefinitions") (separator none) (span (offset 837) (line 34) (column 17) (len 21)))))
    (reference r4 (scope relative) (span (offset 1162) (line 48) (column 18) (len 15)) (segments (segment 0 (token "ViewDefinitions") (name "ViewDefinitions") (separator none) (span (offset 1162) (line 48) (column 18) (len 15)))))
    (reference r5 (scope relative) (span (offset 1199) (line 49) (column 18) (len 18)) (segments (segment 0 (token "PartsTree") (name "PartsTree") (separator none) (span (offset 1199) (line 49) (column 18) (len 9))) (segment 1 (token "vehicle") (name "vehicle") (separator colon-colon) (span (offset 1210) (line 49) (column 29) (len 7)))))
    (reference r6 (scope relative) (span (offset 1256) (line 51) (column 35) (len 17)) (segments (segment 0 (token "SafetyFeatureView") (name "SafetyFeatureView") (separator none) (span (offset 1256) (line 51) (column 35) (len 17)))))
    (reference r7 (scope relative) (span (offset 1286) (line 52) (column 11) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1286) (line 52) (column 11) (len 7)))))
    (reference r8 (scope relative) (span (offset 1386) (line 56) (column 14) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1386) (line 56) (column 14) (len 7)))))
    (reference r9 (scope relative) (span (offset 1503) (line 61) (column 11) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 1503) (line 61) (column 11) (len 7)))))
    (reference r10 (scope relative) (span (offset 1516) (line 61) (column 24) (len 6)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 1516) (line 61) (column 24) (len 6)))))
    (reference r11 (scope relative) (span (offset 1527) (line 61) (column 35) (len 19)) (segments (segment 0 (token "Safety") (name "Safety") (separator none) (span (offset 1527) (line 61) (column 35) (len 6))) (segment 1 (token "isMandatory") (name "isMandatory") (separator colon-colon) (span (offset 1535) (line 61) (column 43) (len 11)))))
  )
  (root (import (target (span (span (offset 15) (line 1) (column 16) (len 8))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 20) (line 1) (column 21) (len 3))) (separator (span (offset 20) (line 1) (column 21) (len 2))) (marker (span (offset 22) (line 1) (column 23) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "11b-Safety and Security Feaure Views") (body (import (target (span (span (offset 153) (line 3) (column 17) (len 15))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 165) (line 3) (column 29) (len 3))) (separator (span (offset 165) (line 3) (column 29) (len 2))) (marker (span (offset 167) (line 3) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "AnnotationDefinitions") (body (metadata-def) (metadata-def))) (package (name "PartsTree") (body (import (target (span (span (offset 337) (line 13) (column 17) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 358) (line 13) (column 38) (len 3))) (separator (span (offset 358) (line 13) (column 38) (len 2))) (marker (span (offset 360) (line 13) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage))) (package (name "ViewDefinitions") (body (import (target (span (span (offset 837) (line 34) (column 17) (len 24))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 858) (line 34) (column 38) (len 3))) (separator (span (offset 858) (line 34) (column 38) (len 2))) (marker (span (offset 860) (line 34) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (view-def) (view-def))) (package (name "Views") (body (import (target (span (span (offset 1162) (line 48) (column 18) (len 18))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 1177) (line 48) (column 33) (len 3))) (separator (span (offset 1177) (line 48) (column 33) (len 2))) (marker (span (offset 1179) (line 48) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1199) (line 49) (column 18) (len 18))) (all none) (ref r5) (shape (membership (recursive-suffix none))))) (view (name "vehicleSafetyFeatureView") (type (ref r6)) (body (expose (target (span (span (offset 1286) (line 52) (column 11) (len 7))) (all none) (ref r7) (shape (membership (recursive-suffix none))))))) (view (name "vehicleMandatorySafetyFeatureView") (type none) (body (expose (target (span (span (offset 1386) (line 56) (column 14) (len 14))) (all none) (ref r8) (shape (namespace (wildcard-suffix (span (span (offset 1393) (line 56) (column 21) (len 3))) (separator (span (offset 1393) (line 56) (column 21) (len 2))) (marker (span (offset 1395) (line 56) (column 23) (len 1)))) (recursive-suffix (span (span (offset 1396) (line 56) (column 24) (len 4))) (separator (span (offset 1396) (line 56) (column 24) (len 2))) (marker (span (offset 1398) (line 56) (column 26) (len 2)))) (combined-recursive-suffix-span (span (offset 1393) (line 56) (column 21) (len 7))))))) (filter))) (view (name "vehicleMandatorySafetyFeatureViewStandalone") (type none) (body (expose (target (span (span (offset 1503) (line 61) (column 11) (len 44))) (all none) (ref r9) (shape (filter (recursive-suffix (span (span (offset 1510) (line 61) (column 18) (len 4))) (separator (span (offset 1510) (line 61) (column 18) (len 2))) (marker (span (offset 1512) (line 61) (column 20) (len 2)))) (members (filter-member (span (span (offset 1514) (line 61) (column 22) (len 33))) (open (span (offset 1514) (line 61) (column 22) (len 1))) (expression (expression (span (offset 1515) (line 61) (column 23) (len 31)) (binary (operator "&&") (left (expression (span (offset 1515) (line 61) (column 23) (len 7)) (classification (metaclass (ref r10))))) (right (expression (span (offset 1527) (line 61) (column 35) (len 19)) (ref r11)))))) (close (span (offset 1546) (line 61) (column 54) (len 1))))))))) (view-rendering))))))))
)
~~~
