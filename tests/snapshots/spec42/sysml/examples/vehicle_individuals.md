# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Vehicle): VehicleIndividuals"))
~~~
# SOURCE
~~~sysml
package VehicleIndividuals {
	private import VehicleUsages::*;
	private import Time::DateTime;
	private import SI::kg;
	
	package IndividualDefinitions {

		individual part def Vehicle1 :> Vehicle {
			doc
			/*
			 * This is an individual Vehicle with a mass of 1800 kg.
			 */
			
			attribute redefines mass = 1800 [kg];
		}
		
		individual part def Vehicle2 :> Vehicle {
			doc
			/*
			 * This is an individual Vehicle with a mass of 1700 kg.
			 */
		
			attribute redefines mass = 1700 [kg];
		}
		
		individual part def AxleAssembly1 :> AxleAssembly;
		
		individual part def Wheel1 :> Wheel;
		individual part def Wheel2 :> Wheel;
	}
	
	package IndividualSnapshots {
		public import IndividualDefinitions::*;
		private import Occurrences::HappensJustBefore;
	
		attribute t0: DateTime;
		attribute t1: DateTime;
		
		individual part vehicle1 : Vehicle1 {
    		snapshot vehicle1_t0 {
    			doc
    			/*
    			 * This is a snapshot of Vehicle1 at time t0;
    			 */
    		
    			attribute :>> localClock.currentTime = t0;
    		}
    		
    		succession : HappensJustBefore first vehicle1_t0 then vehicle1_t0_t1;
    		
    		timeslice vehicle1_t0_t1 {
    			doc
    			/*
    			 * This is a time slice of Vehicle1 starting at snapshot vehicle1_t0 
    			 * (time t0) and ending at time t1.
    			 */
    		
    			snapshot :>> done {
    				attribute :>> localClock.currentTime = t1;
    			}
    		}
		}	
	}
	
	package IndividualConfigurations {
		public import IndividualSnapshots::*;
	
		individual part vehicle1_C2: Vehicle1 :> vehicle_C2, vehicle1 {
			doc
			/*
			 * This asserts that for some portion of its lifetime, Vehicle1 conforms
			 * to the configuration vehicle_C2;
			 */
			
    		snapshot vehicle1_C2_t0 :> vehicle1_t0 {
    			doc
    			/*
    			 * This is a snapshot of Vehicle1 in configuration vehicle1_C2 at time t0.
    			 */
    		
    			individual axleAssembly1_t0: AxleAssembly1 :>> frontAxleAssembly {
    				doc
    				/*
    				 * frontAxleAssembly is a feature of vehicle1_C2.
    				 */
    			
    				individual leftFrontWheel_t0: Wheel1 :>> leftFrontWheel {
    					doc
    					/*
    					 * This asserts that Wheel1 is the leftFrontWheel of vehicle_C2_t0
    					 * (leftFrontWheel is a feature of vehicle_C2::frontAxleAssembly).
    					 */
    				}
    			}
    		}
		
    		snapshot vehicle1_C2_t1 :> vehicle1_t0_t1.done {
    			doc
    			/*
    			 * This is a snapshot of Vehicle1 in configuration vehicle_C2 at time t1.
    			 */
    		
    			individual axleAssembly1_t1: AxleAssembly1 :>> frontAxleAssembly {
    				individual rightFrontWheel_t1: Wheel1 :>> rightFrontWheel {
    					doc
    					/*
    					 * This asserts that Wheel1 is the rightFrontWheel of vehicle_C2_t1.
    					 */
    				}
    			}
    		}	
	       
        }
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicle_individuals.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package VehicleIndividuals {
    private import VehicleUsages::*;
    private import Time::DateTime;
    private import SI::kg;
    package IndividualDefinitions {
        individual part def Vehicle1 :> Vehicle {
            doc
            /*
			 * This is an individual Vehicle with a mass of 1800 kg.
			 */
            attribute redefines mass = 1800[kg];
        }
        individual part def Vehicle2 :> Vehicle {
            doc
            /*
			 * This is an individual Vehicle with a mass of 1700 kg.
			 */
            attribute redefines mass = 1700[kg];
        }
        individual part def AxleAssembly1 :> AxleAssembly;
        individual part def Wheel1 :> Wheel;
        individual part def Wheel2 :> Wheel;
    }
    package IndividualSnapshots {
        public import IndividualDefinitions::*;
        private import Occurrences::HappensJustBefore;
        attribute t0 : DateTime;
        attribute t1 : DateTime;
        individual part vehicle1 : Vehicle1 {
            snapshot vehicle1_t0 {
                doc
                /*
    			 * This is a snapshot of Vehicle1 at time t0;
    			 */
                attribute :>> localClock.currentTime = t0;
            }
            succession : HappensJustBefore first vehicle1_t0 then vehicle1_t0_t1;
            timeslice vehicle1_t0_t1 {
                doc
                /*
    			 * This is a time slice of Vehicle1 starting at snapshot vehicle1_t0 
    			 * (time t0) and ending at time t1.
    			 */
                snapshot :>> done {
                    attribute :>> localClock.currentTime = t1;
                }
            }
        }
    }
    package IndividualConfigurations {
        public import IndividualSnapshots::*;
        individual part vehicle1_C2 : Vehicle1 :> vehicle_C2, vehicle1 {
            doc
            /*
			 * This asserts that for some portion of its lifetime, Vehicle1 conforms
			 * to the configuration vehicle_C2;
			 */
            snapshot vehicle1_C2_t0 :> vehicle1_t0 {
                doc
                /*
    			 * This is a snapshot of Vehicle1 in configuration vehicle1_C2 at time t0.
    			 */
                individual axleAssembly1_t0 : AxleAssembly1 :>> frontAxleAssembly {
                    doc
                    /*
    				 * frontAxleAssembly is a feature of vehicle1_C2.
    				 */
                    individual leftFrontWheel_t0 : Wheel1 :>> leftFrontWheel {
                        doc
                        /*
    					 * This asserts that Wheel1 is the leftFrontWheel of vehicle_C2_t0
    					 * (leftFrontWheel is a feature of vehicle_C2::frontAxleAssembly).
    					 */
                    }
                }
            }
            snapshot vehicle1_C2_t1 :> vehicle1_t0_t1.done {
                doc
                /*
    			 * This is a snapshot of Vehicle1 in configuration vehicle_C2 at time t1.
    			 */
                individual axleAssembly1_t1 : AxleAssembly1 :>> frontAxleAssembly {
                    individual rightFrontWheel_t1 : Wheel1 :>> rightFrontWheel {
                        doc
                        /*
    					 * This asserts that Wheel1 is the rightFrontWheel of vehicle_C2_t1.
    					 */
                    }
                }
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 45) (line 2) (column 17) (len 13)) (segments (segment 0 (token "VehicleUsages") (name "VehicleUsages") (separator none) (span (offset 45) (line 2) (column 17) (len 13)))))
    (reference r1 (scope relative) (span (offset 79) (line 3) (column 17) (len 14)) (segments (segment 0 (token "Time") (name "Time") (separator none) (span (offset 79) (line 3) (column 17) (len 4))) (segment 1 (token "DateTime") (name "DateTime") (separator colon-colon) (span (offset 85) (line 3) (column 23) (len 8)))))
    (reference r2 (scope relative) (span (offset 111) (line 4) (column 17) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 111) (line 4) (column 17) (len 2))) (segment 1 (token "kg") (name "kg") (separator colon-colon) (span (offset 115) (line 4) (column 21) (len 2)))))
    (reference r3 (scope relative) (span (offset 306) (line 14) (column 24) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 306) (line 14) (column 24) (len 4)))))
    (reference r4 (scope relative) (span (offset 319) (line 14) (column 37) (len 2)) (segments (segment 0 (token "kg") (name "kg") (separator none) (span (offset 319) (line 14) (column 37) (len 2)))))
    (reference r5 (scope relative) (span (offset 481) (line 23) (column 24) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 481) (line 23) (column 24) (len 4)))))
    (reference r6 (scope relative) (span (offset 494) (line 23) (column 37) (len 2)) (segments (segment 0 (token "kg") (name "kg") (separator none) (span (offset 494) (line 23) (column 37) (len 2)))))
    (reference r7 (scope relative) (span (offset 692) (line 33) (column 17) (len 21)) (segments (segment 0 (token "IndividualDefinitions") (name "IndividualDefinitions") (separator none) (span (offset 692) (line 33) (column 17) (len 21)))))
    (reference r8 (scope relative) (span (offset 735) (line 34) (column 18) (len 30)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 735) (line 34) (column 18) (len 11))) (segment 1 (token "HappensJustBefore") (name "HappensJustBefore") (separator colon-colon) (span (offset 748) (line 34) (column 31) (len 17)))))
    (reference r9 (scope relative) (span (offset 853) (line 39) (column 30) (len 8)) (segments (segment 0 (token "Vehicle1") (name "Vehicle1") (separator none) (span (offset 853) (line 39) (column 30) (len 8)))))
    (reference r10 (scope relative) (span (offset 1006) (line 46) (column 22) (len 22)) (segments (segment 0 (token "localClock") (name "localClock") (separator none) (span (offset 1006) (line 46) (column 22) (len 10))) (segment 1 (token "currentTime") (name "currentTime") (separator dot) (span (offset 1017) (line 46) (column 33) (len 11)))))
    (reference r11 (scope relative) (span (offset 1031) (line 46) (column 47) (len 2)) (segments (segment 0 (token "t0") (name "t0") (separator none) (span (offset 1031) (line 46) (column 47) (len 2)))))
    (reference r12 (scope relative) (span (offset 1374) (line 59) (column 23) (len 22)) (segments (segment 0 (token "localClock") (name "localClock") (separator none) (span (offset 1374) (line 59) (column 23) (len 10))) (segment 1 (token "currentTime") (name "currentTime") (separator dot) (span (offset 1385) (line 59) (column 34) (len 11)))))
    (reference r13 (scope relative) (span (offset 1399) (line 59) (column 48) (len 2)) (segments (segment 0 (token "t1") (name "t1") (separator none) (span (offset 1399) (line 59) (column 48) (len 2)))))
    (reference r14 (scope relative) (span (offset 1482) (line 66) (column 17) (len 19)) (segments (segment 0 (token "IndividualSnapshots") (name "IndividualSnapshots") (separator none) (span (offset 1482) (line 66) (column 17) (len 19)))))
    (reference r15 (scope relative) (span (offset 1539) (line 68) (column 32) (len 8)) (segments (segment 0 (token "Vehicle1") (name "Vehicle1") (separator none) (span (offset 1539) (line 68) (column 32) (len 8)))))
    (reference r16 (scope relative) (span (offset 1551) (line 68) (column 44) (len 10)) (segments (segment 0 (token "vehicle_C2") (name "vehicle_C2") (separator none) (span (offset 1551) (line 68) (column 44) (len 10)))))
    (reference r17 (scope relative) (span (offset 1563) (line 68) (column 56) (len 8)) (segments (segment 0 (token "vehicle1") (name "vehicle1") (separator none) (span (offset 1563) (line 68) (column 56) (len 8)))))
  )
  (root (package (name "VehicleIndividuals") (body brace (import (target (span (span (offset 45) (line 2) (column 17) (len 16))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 58) (line 2) (column 30) (len 3))) (separator (span (offset 58) (line 2) (column 30) (len 2))) (marker (span (offset 60) (line 2) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 79) (line 3) (column 17) (len 14))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 111) (line 4) (column 17) (len 6))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (package (name "IndividualDefinitions") (body brace (part-def (name "Vehicle1") (modifiers individual) (body brace (doc (name none) (locale none) (body (span (offset 211) (line 10) (column 6) (len 65)) (normalized "This is an individual Vehicle with a mass of 1800 kg.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 313) (line 14) (column 31) (len 9)) (bracket (base (expression (span (offset 313) (line 14) (column 31) (len 4)) (integer 1800))) (operands (sequence-list (element first (expression (span (offset 319) (line 14) (column 37) (len 2)) (ref r4)))))))))) (body semicolon)))) (part-def (name "Vehicle2") (modifiers individual) (body brace (doc (name none) (locale none) (body (span (offset 387) (line 19) (column 6) (len 65)) (normalized "This is an individual Vehicle with a mass of 1700 kg.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 488) (line 23) (column 31) (len 9)) (bracket (base (expression (span (offset 488) (line 23) (column 31) (len 4)) (integer 1700))) (operands (sequence-list (element first (expression (span (offset 494) (line 23) (column 37) (len 2)) (ref r6)))))))))) (body semicolon)))) (part-def (name "AxleAssembly1") (modifiers individual) (body semicolon)) (part-def (name "Wheel1") (modifiers individual) (body semicolon)) (part-def (name "Wheel2") (modifiers individual) (body semicolon)))) (package (name "IndividualSnapshots") (body brace (import (target (span (span (offset 692) (line 33) (column 17) (len 24))) (all none) (ref r7) (shape (namespace (wildcard-suffix (span (span (offset 713) (line 33) (column 38) (len 3))) (separator (span (offset 713) (line 33) (column 38) (len 2))) (marker (span (offset 715) (line 33) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 735) (line 34) (column 18) (len 30))) (all none) (ref r8) (shape (membership (recursive-suffix none))))) (attribute-usage) (attribute-usage) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration-name "vehicle1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "vehicle1_t0") (short-name none) (target none) (body brace (doc (name none) (locale none) (body (span (offset 913) (line 42) (column 10) (len 62)) (normalized "This is a snapshot of Vehicle1 at time t0;\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1031) (line 46) (column 47) (len 2)) (ref r11))))) (body semicolon)))) (succession-usage) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion timeslice) (extensions)) (declaration "vehicle1_t0_t1") (short-name none) (target none) (body brace (doc (name none) (locale none) (body (span (offset 1186) (line 53) (column 10) (len 129)) (normalized "This is a time slice of Vehicle1 starting at snapshot vehicle1_t0 \n(time t0) and ending at time t1.\n"))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "") (short-name none) (target none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1399) (line 59) (column 48) (len 2)) (ref r13))))) (body semicolon)))))))))) (package (name "IndividualConfigurations") (body brace (import (target (span (span (offset 1482) (line 66) (column 17) (len 22))) (all none) (ref r14) (shape (namespace (wildcard-suffix (span (span (offset 1501) (line 66) (column 36) (len 3))) (separator (span (offset 1501) (line 66) (column 36) (len 2))) (marker (span (offset 1503) (line 66) (column 38) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration-name "vehicle1_C2") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r16) (ref r17))) (value none))) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1586) (line 70) (column 6) (len 120)) (normalized "This asserts that for some portion of its lifetime, Vehicle1 conforms\nto the configuration vehicle_C2;\n"))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "vehicle1_C2_t0") (short-name none) (target none) (body brace (doc (name none) (locale none) (body (span (offset 1780) (line 77) (column 10) (len 91)) (normalized "This is a snapshot of Vehicle1 in configuration vehicle1_C2 at time t0.\n"))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "axleAssembly1_t0") (short-name none) (target none) (body brace (doc (name none) (locale none) (body (span (offset 1977) (line 83) (column 11) (len 68)) (normalized "frontAxleAssembly is a feature of vehicle1_C2.\n"))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "leftFrontWheel_t0") (short-name none) (target none) (body brace (doc (name none) (locale none) (body (span (offset 2146) (line 89) (column 12) (len 163)) (normalized "This asserts that Wheel1 is the leftFrontWheel of vehicle_C2_t0\n(leftFrontWheel is a feature of vehicle_C2::frontAxleAssembly).\n"))))))))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "vehicle1_C2_t1") (short-name none) (target none) (body brace (doc (name none) (locale none) (body (span (offset 2417) (line 99) (column 10) (len 90)) (normalized "This is a snapshot of Vehicle1 in configuration vehicle_C2 at time t1.\n"))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "axleAssembly1_t1") (short-name none) (target none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration "rightFrontWheel_t1") (short-name none) (target none) (body brace (doc (name none) (locale none) (body (span (offset 2683) (line 106) (column 12) (len 89)) (normalized "This asserts that Wheel1 is the rightFrontWheel of vehicle_C2_t1.\n"))))))))))))))))
)
~~~
