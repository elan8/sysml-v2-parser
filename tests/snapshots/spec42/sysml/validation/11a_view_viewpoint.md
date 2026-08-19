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
    (reference r3 (scope relative) (span (offset 268) (line 13) (column 29) (len 12)) (segments (segment 0 (token "AxleAssembly") (name "AxleAssembly") (separator none) (span (offset 268) (line 13) (column 29) (len 12)))))
    (reference r4 (scope relative) (span (offset 308) (line 14) (column 23) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 308) (line 14) (column 23) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 313) (line 14) (column 28) (len 4)))))
    (reference r5 (scope relative) (span (offset 351) (line 15) (column 23) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 351) (line 15) (column 23) (len 5)))))
    (reference r6 (scope relative) (span (offset 382) (line 16) (column 22) (len 4)) (segments (segment 0 (token "Axle") (name "Axle") (separator none) (span (offset 382) (line 16) (column 22) (len 4)))))
    (reference r7 (scope relative) (span (offset 481) (line 21) (column 28) (len 12)) (segments (segment 0 (token "AxleAssembly") (name "AxleAssembly") (separator none) (span (offset 481) (line 21) (column 28) (len 12)))))
    (reference r8 (scope relative) (span (offset 521) (line 22) (column 23) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 521) (line 22) (column 23) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 526) (line 22) (column 28) (len 4)))))
    (reference r9 (scope relative) (span (offset 563) (line 23) (column 22) (len 5)) (segments (segment 0 (token "Wheel") (name "Wheel") (separator none) (span (offset 563) (line 23) (column 22) (len 5)))))
    (reference r10 (scope relative) (span (offset 593) (line 24) (column 21) (len 4)) (segments (segment 0 (token "Axle") (name "Axle") (separator none) (span (offset 593) (line 24) (column 21) (len 4)))))
    (reference r11 (scope relative) (span (offset 685) (line 33) (column 18) (len 5)) (segments (segment 0 (token "Views") (name "Views") (separator none) (span (offset 685) (line 33) (column 18) (len 5)))))
    (reference r12 (scope relative) (span (offset 949) (line 47) (column 12) (len 30)) (segments (segment 0 (token "'system structure perspective'") (name "system structure perspective") (separator none) (span (offset 949) (line 47) (column 12) (len 30)))))
    (reference r13 (scope relative) (span (offset 991) (line 48) (column 11) (len 20)) (segments (segment 0 (token "SystemModel") (name "SystemModel") (separator none) (span (offset 991) (line 48) (column 11) (len 11))) (segment 1 (token "vehicle") (name "vehicle") (separator colon-colon) (span (offset 1004) (line 48) (column 24) (len 7)))))
    (reference r14 (scope relative) (span (offset 1017) (line 48) (column 37) (len 16)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 1017) (line 48) (column 37) (len 5))) (segment 1 (token "PartUsage") (name "PartUsage") (separator colon-colon) (span (offset 1024) (line 48) (column 44) (len 9)))))
  )
  (root (package (name "11a-View-Viewpoint") (body brace (package (name "SystemModel") (body brace (import (target (span (span (offset 73) (line 4) (column 18) (len 5))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 75) (line 4) (column 20) (len 3))) (separator (span (offset 75) (line 4) (column 20) (len 2))) (marker (span (offset 77) (line 4) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Vehicle") (body semicolon)) (part-def (name "AxleAssembly") (body semicolon)) (part-def (name "Axle") (body semicolon)) (part-def (name "Wheel") (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r2)))) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 226) (line 12) (column 34) (len 12)) (literal-with-unit (value (expression (span (offset 226) (line 12) (column 34) (len 4)) (integer 2500))) (unit (expression (span (offset 231) (line 12) (column 39) (len 6)) (bracket (expression (span (offset 231) (line 12) (column 39) (len 6)) (unit "SI::kg")))))))))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxleAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity (lower (expression (span (offset 281) (line 13) (column 42) (len 1)) (integer 1))) (upper (expression (span (offset 281) (line 13) (column 42) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r4)))) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 320) (line 14) (column 35) (len 7)) (literal-with-unit (value (expression (span (offset 320) (line 14) (column 35) (len 3)) (integer 150))) (unit (expression (span (offset 324) (line 14) (column 39) (len 2)) (bracket (expression (span (offset 324) (line 14) (column 39) (len 2)) (unit "kg")))))))))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity (lower (expression (span (offset 357) (line 15) (column 29) (len 1)) (integer 2))) (upper (expression (span (offset 357) (line 15) (column 29) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity (lower (expression (span (offset 387) (line 16) (column 27) (len 1)) (integer 1))) (upper (expression (span (offset 387) (line 16) (column 27) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "steeringAngle") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxleAssembly") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity (lower (expression (span (offset 494) (line 21) (column 41) (len 1)) (integer 1))) (upper (expression (span (offset 494) (line 21) (column 41) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r8)))) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 533) (line 22) (column 35) (len 7)) (literal-with-unit (value (expression (span (offset 533) (line 22) (column 35) (len 3)) (integer 250))) (unit (expression (span (offset 537) (line 22) (column 39) (len 2)) (bracket (expression (span (offset 537) (line 22) (column 39) (len 2)) (unit "kg")))))))))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearWheel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity (lower (expression (span (offset 569) (line 23) (column 28) (len 1)) (integer 2))) (upper (expression (span (offset 569) (line 23) (column 28) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rearAxle") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity (lower (expression (span (offset 598) (line 24) (column 26) (len 1)) (integer 1))) (upper (expression (span (offset 598) (line 24) (column 26) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))))) (package (name "ViewModel") (body brace (import (target (span (span (offset 685) (line 33) (column 18) (len 8))) (all none) (ref r11) (shape (namespace (wildcard-suffix (span (span (offset 690) (line 33) (column 23) (len 3))) (separator (span (offset 690) (line 33) (column 23) (len 2))) (marker (span (offset 692) (line 33) (column 25) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "systems engineer") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (concern-usage) (viewpoint-usage) (view (name "system structure generation") (short-name none) (type none) (body brace (satisfy (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (visibility none) (assert false) (negated false) (requirement (reference (ref r12))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (redefines none) (crosses none) (value none) (by none) (body semicolon)) (expose (target (span (span (offset 991) (line 48) (column 11) (len 43))) (all none) (ref r13) (shape (filter (recursive-suffix (span (span (offset 1011) (line 48) (column 31) (len 4))) (separator (span (offset 1011) (line 48) (column 31) (len 2))) (marker (span (offset 1013) (line 48) (column 33) (len 2)))) (members (filter-member (span (span (offset 1015) (line 48) (column 35) (len 19))) (open (span (offset 1015) (line 48) (column 35) (len 1))) (expression (expression (span (offset 1016) (line 48) (column 36) (len 17)) (classification (metaclass (ref r14))))) (close (span (offset 1033) (line 48) (column 53) (len 1)))))))) (body semicolon)) (view-rendering))))))))
)
~~~
