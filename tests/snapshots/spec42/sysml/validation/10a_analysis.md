# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (10-Analysis and Trades): 10a-Analysis"))
~~~
# SOURCE
~~~sysml
package '10a-Analysis' {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	package VehicleDesignModel {
		part def Vehicle {
			mass : MassValue;
		}
		
		part vehicle {
			:>> mass : MassValue = sum((
				vehicle.engine.mass, 
				vehicle.transmission.mass, 
				vehicle.frontAxleAssembly.mass, 
				vehicle.rearAxleAssembly.mass
			));
			
			part engine {
				mass : MassValue;
			}
			
			part transmission {
			    mass : MassValue;
			}
			
			part frontAxleAssembly {
				mass : MassValue;
			}
			
			part rearAxleAssembly {
				mass : MassValue;
			}
		}
	}
	
	package VehicleAnalysisModel {
		private import VehicleDesignModel::Vehicle;
		
		requirement def MassAnalysisObjective {
			subject mass : MassValue;
			doc /* ... */
		}
	
		analysis def MassAnalysisCase {
			subject vehicle : Vehicle;
			objective : MassAnalysisObjective {
			    subject = MassAnalysisCase::result;
			}
			
			// Result
			vehicle.mass
		}
		
		analysis def AnalysisPlan {
			subject vehicle : Vehicle;			
			objective {
				doc /* ... */
			}
			
			analysis massAnalysisCase : MassAnalysisCase {
				/*
				 * By default, the subject of a nested analysis case bound to that
				 * of its containing analysis case or analysis case definition.
				 */
			 	return mass; 
			 }
		}
		
		part massAnalysisContext {
			analysis analysisPlan : AnalysisPlan {
				subject vehicle = VehicleDesignModel::vehicle;
			}
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "10a_analysis.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '10a-Analysis' {
    private import ISQ::*;
    private import SI::*;
    private import NumericalFunctions::*;
    package VehicleDesignModel {
        part def Vehicle {
            mass : MassValue;
        }
        part vehicle {
            attribute :>> mass : MassValue = sum((vehicle.engine.mass, vehicle.transmission.mass, vehicle.frontAxleAssembly.mass, vehicle.rearAxleAssembly.mass));
            part engine {
                mass : MassValue;
            }
            part transmission {
                mass : MassValue;
            }
            part frontAxleAssembly {
                mass : MassValue;
            }
            part rearAxleAssembly {
                mass : MassValue;
            }
        }
    }
    package VehicleAnalysisModel {
        private import VehicleDesignModel::Vehicle;
        requirement def MassAnalysisObjective {
            subject mass : MassValue;
            doc
            /* ... */
        }
        analysis def MassAnalysisCase {
            subject vehicle : Vehicle;
            objective  : MassAnalysisObjective {
                subject = MassAnalysisCase::result;
            }
            vehicle.mass
        }
        analysis def AnalysisPlan {
            subject vehicle : Vehicle;
            objective {
                doc
                /* ... */
            }
            analysis massAnalysisCase : MassAnalysisCase {
                /*
				 * By default, the subject of a nested analysis case bound to that
				 * of its containing analysis case or analysis case definition.
				 */
                return mass;
            }
        }
        part massAnalysisContext {
            analysis analysisPlan : AnalysisPlan {
                subject vehicle = VehicleDesignModel::vehicle;
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 41) (line 2) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 41) (line 2) (column 17) (len 3)))))
    (reference r1 (scope relative) (span (offset 65) (line 3) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 65) (line 3) (column 17) (len 2)))))
    (reference r2 (scope relative) (span (offset 88) (line 4) (column 17) (len 18)) (segments (segment 0 (token "NumericalFunctions") (name "NumericalFunctions") (separator none) (span (offset 88) (line 4) (column 17) (len 18)))))
    (reference r3 (scope relative) (span (offset 174) (line 8) (column 11) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 174) (line 8) (column 11) (len 9)))))
    (reference r4 (scope relative) (span (offset 223) (line 12) (column 15) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 223) (line 12) (column 15) (len 9)))))
    (reference r5 (scope relative) (span (offset 216) (line 12) (column 8) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 216) (line 12) (column 8) (len 4)))))
    (reference r6 (scope relative) (span (offset 235) (line 12) (column 27) (len 3)) (segments (segment 0 (token "sum") (name "sum") (separator none) (span (offset 235) (line 12) (column 27) (len 3)))))
    (reference r7 (scope relative) (span (offset 245) (line 13) (column 5) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 245) (line 13) (column 5) (len 7)))))
    (reference r8 (scope relative) (span (offset 253) (line 13) (column 13) (len 6)) (segments (segment 0 (token "engine") (name "engine") (separator none) (span (offset 253) (line 13) (column 13) (len 6)))))
    (reference r9 (scope relative) (span (offset 260) (line 13) (column 20) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 260) (line 13) (column 20) (len 4)))))
    (reference r10 (scope relative) (span (offset 271) (line 14) (column 5) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 271) (line 14) (column 5) (len 7)))))
    (reference r11 (scope relative) (span (offset 279) (line 14) (column 13) (len 12)) (segments (segment 0 (token "transmission") (name "transmission") (separator none) (span (offset 279) (line 14) (column 13) (len 12)))))
    (reference r12 (scope relative) (span (offset 292) (line 14) (column 26) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 292) (line 14) (column 26) (len 4)))))
    (reference r13 (scope relative) (span (offset 303) (line 15) (column 5) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 303) (line 15) (column 5) (len 7)))))
    (reference r14 (scope relative) (span (offset 311) (line 15) (column 13) (len 17)) (segments (segment 0 (token "frontAxleAssembly") (name "frontAxleAssembly") (separator none) (span (offset 311) (line 15) (column 13) (len 17)))))
    (reference r15 (scope relative) (span (offset 329) (line 15) (column 31) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 329) (line 15) (column 31) (len 4)))))
    (reference r16 (scope relative) (span (offset 340) (line 16) (column 5) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 340) (line 16) (column 5) (len 7)))))
    (reference r17 (scope relative) (span (offset 348) (line 16) (column 13) (len 16)) (segments (segment 0 (token "rearAxleAssembly") (name "rearAxleAssembly") (separator none) (span (offset 348) (line 16) (column 13) (len 16)))))
    (reference r18 (scope relative) (span (offset 365) (line 16) (column 30) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 365) (line 16) (column 30) (len 4)))))
    (reference r19 (scope relative) (span (offset 409) (line 20) (column 12) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 409) (line 20) (column 12) (len 9)))))
    (reference r20 (scope relative) (span (offset 466) (line 24) (column 15) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 466) (line 24) (column 15) (len 9)))))
    (reference r21 (scope relative) (span (offset 525) (line 28) (column 12) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 525) (line 28) (column 12) (len 9)))))
    (reference r22 (scope relative) (span (offset 583) (line 32) (column 12) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 583) (line 32) (column 12) (len 9)))))
    (reference r23 (scope relative) (span (offset 657) (line 38) (column 18) (len 27)) (segments (segment 0 (token "VehicleDesignModel") (name "VehicleDesignModel") (separator none) (span (offset 657) (line 38) (column 18) (len 18))) (segment 1 (token "Vehicle") (name "Vehicle") (separator colon-colon) (span (offset 677) (line 38) (column 38) (len 7)))))
    (reference r24 (scope relative) (span (offset 749) (line 41) (column 19) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 749) (line 41) (column 19) (len 9)))))
    (reference r25 (scope relative) (span (offset 838) (line 46) (column 22) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 838) (line 46) (column 22) (len 7)))))
    (reference r26 (scope relative) (span (offset 862) (line 47) (column 16) (len 21)) (segments (segment 0 (token "MassAnalysisObjective") (name "MassAnalysisObjective") (separator none) (span (offset 862) (line 47) (column 16) (len 21)))))
    (reference r27 (scope relative) (span (offset 903) (line 48) (column 18) (len 24)) (segments (segment 0 (token "MassAnalysisCase") (name "MassAnalysisCase") (separator none) (span (offset 903) (line 48) (column 18) (len 16))) (segment 1 (token "result") (name "result") (separator colon-colon) (span (offset 921) (line 48) (column 36) (len 6)))))
    (reference r28 (scope relative) (span (offset 954) (line 52) (column 4) (len 7)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 954) (line 52) (column 4) (len 7)))))
    (reference r29 (scope relative) (span (offset 962) (line 52) (column 12) (len 4)) (segments (segment 0 (token "mass") (name "mass") (separator none) (span (offset 962) (line 52) (column 12) (len 4)))))
    (reference r30 (scope relative) (span (offset 1025) (line 56) (column 22) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 1025) (line 56) (column 22) (len 7)))))
    (reference r31 (scope relative) (span (offset 1110) (line 61) (column 32) (len 16)) (segments (segment 0 (token "MassAnalysisCase") (name "MassAnalysisCase") (separator none) (span (offset 1110) (line 61) (column 32) (len 16)))))
    (reference r32 (scope relative) (span (offset 1371) (line 71) (column 28) (len 12)) (segments (segment 0 (token "AnalysisPlan") (name "AnalysisPlan") (separator none) (span (offset 1371) (line 71) (column 28) (len 12)))))
    (reference r33 (scope relative) (span (offset 1408) (line 72) (column 23) (len 27)) (segments (segment 0 (token "VehicleDesignModel") (name "VehicleDesignModel") (separator none) (span (offset 1408) (line 72) (column 23) (len 18))) (segment 1 (token "vehicle") (name "vehicle") (separator colon-colon) (span (offset 1428) (line 72) (column 43) (len 7)))))
  )
  (root (package (visibility none) (name "10a-Analysis") (body brace (import (visibility private) (target (span (span (offset 41) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 44) (line 2) (column 20) (len 3))) (separator (span (offset 44) (line 2) (column 20) (len 2))) (marker (span (offset 46) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 65) (line 3) (column 17) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 67) (line 3) (column 19) (len 3))) (separator (span (offset 67) (line 3) (column 19) (len 2))) (marker (span (offset 69) (line 3) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 88) (line 4) (column 17) (len 21))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 106) (line 4) (column 35) (len 3))) (separator (span (offset 106) (line 4) (column 35) (len 2))) (marker (span (offset 108) (line 4) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (package (visibility none) (name "VehicleDesignModel") (body brace (part-def (name "Vehicle") (modifiers) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "mass") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 235) (line 12) (column 27) (len 140)) (invocation (callee (expression (span (offset 235) (line 12) (column 27) (len 3)) (ref r6))) (arguments (argument (parameter none) (value (expression (span (offset 239) (line 12) (column 31) (len 135)) (sequence (sequence-list (element first (expression (span (offset 245) (line 13) (column 5) (len 19)) (member-access (base (expression (span (offset 245) (line 13) (column 5) (len 14)) (member-access (base (expression (span (offset 245) (line 13) (column 5) (len 7)) (ref r7))) (separator dot) (member (ref r8))))) (separator dot) (member (ref r9))))) (element comma (expression (span (offset 271) (line 14) (column 5) (len 25)) (member-access (base (expression (span (offset 271) (line 14) (column 5) (len 20)) (member-access (base (expression (span (offset 271) (line 14) (column 5) (len 7)) (ref r10))) (separator dot) (member (ref r11))))) (separator dot) (member (ref r12))))) (element comma (expression (span (offset 303) (line 15) (column 5) (len 30)) (member-access (base (expression (span (offset 303) (line 15) (column 5) (len 25)) (member-access (base (expression (span (offset 303) (line 15) (column 5) (len 7)) (ref r13))) (separator dot) (member (ref r14))))) (separator dot) (member (ref r15))))) (element comma (expression (span (offset 340) (line 16) (column 5) (len 29)) (member-access (base (expression (span (offset 340) (line 16) (column 5) (len 24)) (member-access (base (expression (span (offset 340) (line 16) (column 5) (len 7)) (ref r16))) (separator dot) (member (ref r17))))) (separator dot) (member (ref r18)))))))))))))))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "mass") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "transmission") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "mass") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxleAssembly") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "mass") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxleAssembly") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "mass") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))) (package (visibility none) (name "VehicleAnalysisModel") (body brace (import (visibility private) (target (span (span (offset 657) (line 38) (column 18) (len 27))) (all none) (ref r23) (shape (membership (recursive-suffix none)))) (body semicolon)) (requirement-def (name "MassAnalysisObjective") (modifiers) (body brace (subject (name "mass") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (doc (name none) (short-name none) (locale none) (body (span (offset 769) (line 42) (column 10) (len 5)) (normalized "... "))))) (analysis-case-def (name "MassAnalysisCase") (modifiers) (body brace (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (objective (visibility none) (name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r26)))) (body brace (subject (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 903) (line 48) (column 18) (len 24)) (ref r27))))) (body semicolon)))) (expression (span (offset 954) (line 52) (column 4) (len 12)) (member-access (base (expression (span (offset 954) (line 52) (column 4) (len 7)) (ref r28))) (separator dot) (member (ref r29)))))) (analysis-case-def (name "AnalysisPlan") (modifiers) (body brace (subject (name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r30)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (objective (visibility none) (name none) (type none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 1062) (line 58) (column 11) (len 5)) (normalized "... "))))) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "massAnalysisCase") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r31)))) (subsets none) (redefines none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1135) (line 62) (column 7) (len 145)) (normalized "By default, the subject of a nested analysis case bound to that\nof its containing analysis case or analysis case definition.\n"))) (case-return (declaration "mass") (target none) (type none) (redefines none) (feature-kind none) (subsetting false) (value none)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "massAnalysisContext") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "analysisPlan") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r32)))) (subsets none) (redefines none) (body brace (subject (name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1408) (line 72) (column 23) (len 27)) (ref r33))))) (body semicolon)))))))))))
)
~~~
