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
            attribute  :>> mass : MassValue = sum((vehicle.engine.mass, vehicle.transmission.mass, vehicle.frontAxleAssembly.mass, vehicle.rearAxleAssembly.mass));
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
            objective : MassAnalysisObjective  {
                subject = MassAnalysisCase::result;
            }
            vehicle.mass;
        }
        analysis def AnalysisPlan {
            subject vehicle : Vehicle;
            objective  {
                doc
                /* ... */
            }
            analysis massAnalysisCase : MassAnalysisCase {
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
  (references
    (reference r0 (scope relative) (span (offset 41) (line 2) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 41) (line 2) (column 17) (len 3)))))
    (reference r1 (scope relative) (span (offset 65) (line 3) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 65) (line 3) (column 17) (len 2)))))
    (reference r2 (scope relative) (span (offset 88) (line 4) (column 17) (len 18)) (segments (segment 0 (token "NumericalFunctions") (name "NumericalFunctions") (separator none) (span (offset 88) (line 4) (column 17) (len 18)))))
    (reference r3 (scope relative) (span (offset 657) (line 38) (column 18) (len 27)) (segments (segment 0 (token "VehicleDesignModel") (name "VehicleDesignModel") (separator none) (span (offset 657) (line 38) (column 18) (len 18))) (segment 1 (token "Vehicle") (name "Vehicle") (separator colon-colon) (span (offset 677) (line 38) (column 38) (len 7)))))
    (reference r4 (scope relative) (span (offset 749) (line 41) (column 19) (len 9)) (segments (segment 0 (token "MassValue") (name "MassValue") (separator none) (span (offset 749) (line 41) (column 19) (len 9)))))
  )
  (root (package (name "10a-Analysis") (body (import (target (span (span (offset 41) (line 2) (column 17) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 44) (line 2) (column 20) (len 3))) (separator (span (offset 44) (line 2) (column 20) (len 2))) (marker (span (offset 46) (line 2) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 65) (line 3) (column 17) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 67) (line 3) (column 19) (len 3))) (separator (span (offset 67) (line 3) (column 19) (len 2))) (marker (span (offset 69) (line 3) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 88) (line 4) (column 17) (len 21))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 106) (line 4) (column 35) (len 3))) (separator (span (offset 106) (line 4) (column 35) (len 2))) (marker (span (offset 108) (line 4) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "VehicleDesignModel") (body (part-def (name "Vehicle") (body (default-reference-usage))) (part-usage))) (package (name "VehicleAnalysisModel") (body (import (target (span (span (offset 657) (line 38) (column 18) (len 27))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (requirement-def (name "MassAnalysisObjective") (body (subject (name "mass") (type (ref r4)) (redefines none) (value none)) (doc))) (analysis-case-def) (analysis-case-def) (part-usage))))))
)
~~~
