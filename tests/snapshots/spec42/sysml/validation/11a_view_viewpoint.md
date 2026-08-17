# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (11-View and Viewpoint): 11a-View-Viewpoint"))
~~~
# SOURCE
~~~sysml
package '11a-View-Viewpoint' {
	
	package SystemModel {
		private import SI::*;
		
		part def Vehicle;
		part def AxleAssembly;
		part def Axle;
		part def Wheel;
		
		part vehicle : Vehicle {
			attribute mass :> ISQ::mass = 2500[SI::kg];
			part frontAxleAssembly : AxleAssembly[1] {
				attribute mass :> ISQ::mass = 150[kg];
				part frontWheel : Wheel[2];
				part frontAxle : Axle[1] {
					attribute mass;
					attribute steeringAngle;
				}
			}
			part rearAxleAssembly : AxleAssembly[1] {
				attribute mass :> ISQ::mass = 250[kg];
				part rearWheel : Wheel[2];
				part rearAxle : Axle[1] {
					attribute mass;
				}
			}
		}
		
	}
	
	package ViewModel {
		private import Views::*;
	
		part 'systems engineer';
		
		concern 'system breakdown' {
			subject;
			stakeholder :>> 'systems engineer';
		}
		
		viewpoint 'system structure perspective' {		
			frame 'system breakdown';
		}
		
		view 'system structure generation' {
			satisfy 'system structure perspective';
			expose SystemModel::vehicle::**[@SysML::PartUsage];
			render asElementTable {
				view :>> columnView[1] {
					render asTextualNotation;
				}
			}
		}
	
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "11a_view_viewpoint.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '11a-View-Viewpoint' {
    package SystemModel {
        private import SI::*;
        part def Vehicle;
        part def AxleAssembly;
        part def Axle;
        part def Wheel;
        part vehicle : Vehicle {
            attribute mass :> ISQ::mass = 2500 ['SI::kg'];
            part frontAxleAssembly : AxleAssembly[1] {
                attribute mass :> ISQ::mass = 150 [kg];
                part frontWheel : Wheel[2];
                part frontAxle : Axle[1] {
                    attribute mass;
                    attribute steeringAngle;
                }
            }
            part rearAxleAssembly : AxleAssembly[1] {
                attribute mass :> ISQ::mass = 250 [kg];
                part rearWheel : Wheel[2];
                part rearAxle : Axle[1] {
                    attribute mass;
                }
            }
        }
    }
    package ViewModel {
        private import Views::*;
        part 'systems engineer';
        concern 'system breakdown' {
            subject;
            stakeholder :>> 'systems engineer';
        }
        viewpoint 'system structure perspective' {
            frame 'system breakdown';
        }
        view 'system structure generation' {
            satisfy 'system structure perspective';
            expose SystemModel::vehicle::** [@SysML::PartUsage];
            render asElementTable {
                view  :>> columnView[1] {
                    render asTextualNotation;
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
    (reference r0 (scope relative) (span (offset 73) (line 4) (column 18) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 73) (line 4) (column 18) (len 2)))))
    (reference r1 (scope relative) (span (offset 183) (line 11) (column 18) (len 7)) (segments (segment 0 (token "Vehicle") (name "Vehicle") (separator none) (span (offset 183) (line 11) (column 18) (len 7)))))
    (reference r2 (scope relative) (span (offset 214) (line 12) (column 22) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 214) (line 12) (column 22) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 219) (line 12) (column 27) (len 4)))))
    (reference r3 (scope relative) (span (offset 685) (line 33) (column 18) (len 5)) (segments (segment 0 (token "Views") (name "Views") (separator none) (span (offset 685) (line 33) (column 18) (len 5)))))
    (reference r4 (scope relative) (span (offset 949) (line 47) (column 12) (len 30)) (segments (segment 0 (token "'system structure perspective'") (name "system structure perspective") (separator none) (span (offset 949) (line 47) (column 12) (len 30)))))
    (reference r5 (scope relative) (span (offset 991) (line 48) (column 11) (len 20)) (segments (segment 0 (token "SystemModel") (name "SystemModel") (separator none) (span (offset 991) (line 48) (column 11) (len 11))) (segment 1 (token "vehicle") (name "vehicle") (separator colon-colon) (span (offset 1004) (line 48) (column 24) (len 7)))))
    (reference r6 (scope relative) (span (offset 1017) (line 48) (column 37) (len 16)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 1017) (line 48) (column 37) (len 5))) (segment 1 (token "PartUsage") (name "PartUsage") (separator colon-colon) (span (offset 1024) (line 48) (column 44) (len 9)))))
  )
  (root (package (name "11a-View-Viewpoint") (body brace (package (name "SystemModel") (body brace (import (target (span (span (offset 73) (line 4) (column 18) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 75) (line 4) (column 20) (len 3))) (separator (span (offset 75) (line 4) (column 20) (len 2))) (marker (span (offset 77) (line 4) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle") (body semicolon)) (part-def (name "AxleAssembly") (body semicolon)) (part-def (name "Axle") (body semicolon)) (part-def (name "Wheel") (body semicolon)) (part-usage (declaration-name "vehicle") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 226) (line 12) (column 34) (len 12)) (literal-with-unit (value (expression (span (offset 226) (line 12) (column 34) (len 4)) (integer 2500))) (unit (expression (span (offset 231) (line 12) (column 39) (len 6)) (bracket (expression (span (offset 231) (line 12) (column 39) (len 6)) (unit "SI::kg")))))))))) (body semicolon)) (part-usage) (part-usage))))) (package (name "ViewModel") (body brace (import (target (span (span (offset 685) (line 33) (column 18) (len 8))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 690) (line 33) (column 23) (len 3))) (separator (span (offset 690) (line 33) (column 23) (len 2))) (marker (span (offset 692) (line 33) (column 25) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (declaration-name "systems engineer") (typing none) (body semicolon)) (concern-usage) (viewpoint-usage) (view (name "system structure generation") (type none) (body brace (satisfy (assert false) (negated false) (requirement (reference (ref r4))) (typing none) (multiplicity none) (ordered false) (nonunique false) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)) (expose (target (span (span (offset 991) (line 48) (column 11) (len 43))) (all none) (ref r5) (shape (filter (recursive-suffix (span (span (offset 1011) (line 48) (column 31) (len 4))) (separator (span (offset 1011) (line 48) (column 31) (len 2))) (marker (span (offset 1013) (line 48) (column 33) (len 2)))) (members (filter-member (span (span (offset 1015) (line 48) (column 35) (len 19))) (open (span (offset 1015) (line 48) (column 35) (len 1))) (expression (expression (span (offset 1016) (line 48) (column 36) (len 17)) (classification (metaclass (ref r6))))) (close (span (offset 1033) (line 48) (column 53) (len 1)))))))) (body semicolon)) (view-rendering))))))))
)
~~~
