# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 18 (Action Performance): Action Performance Example"))
~~~
# SOURCE
~~~sysml
package 'Action Performance Example' {
	private import 'Action Decomposition'::*;
	
	part def Camera;
	part def AutoFocus;
	part def Imager;
	
	part camera : Camera {
		
		perform action takePhoto[*] ordered 
			references takePicture;
		
		part f : AutoFocus {
			perform takePhoto.focus;			
		}
		
		part i : Imager {
			perform takePhoto.shoot;
		}		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "18_action_performance_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Action Performance Example' {
    private import 'Action Decomposition'::*;
    part def Camera;
    part def AutoFocus;
    part def Imager;
    part camera : Camera {
        perform action takePhoto[*] ordered references takePicture;
        part f : AutoFocus {
            perform takePhoto.focus;
        }
        part i : Imager {
            perform takePhoto.shoot;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 55) (line 2) (column 17) (len 22)) (segments (segment 0 (token "'Action Decomposition'") (name "Action Decomposition") (separator none) (span (offset 55) (line 2) (column 17) (len 22)))))
    (reference r1 (scope relative) (span (offset 158) (line 8) (column 16) (len 6)) (segments (segment 0 (token "Camera") (name "Camera") (separator none) (span (offset 158) (line 8) (column 16) (len 6)))))
    (reference r2 (scope relative) (span (offset 223) (line 11) (column 15) (len 11)) (segments (segment 0 (token "takePicture") (name "takePicture") (separator none) (span (offset 223) (line 11) (column 15) (len 11)))))
    (reference r3 (scope relative) (span (offset 250) (line 13) (column 12) (len 9)) (segments (segment 0 (token "AutoFocus") (name "AutoFocus") (separator none) (span (offset 250) (line 13) (column 12) (len 9)))))
    (reference r4 (scope relative) (span (offset 273) (line 14) (column 12) (len 15)) (segments (segment 0 (token "takePhoto") (name "takePhoto") (separator none) (span (offset 273) (line 14) (column 12) (len 9))) (segment 1 (token "focus") (name "focus") (separator dot) (span (offset 283) (line 14) (column 22) (len 5)))))
    (reference r5 (scope relative) (span (offset 311) (line 17) (column 12) (len 6)) (segments (segment 0 (token "Imager") (name "Imager") (separator none) (span (offset 311) (line 17) (column 12) (len 6)))))
    (reference r6 (scope relative) (span (offset 331) (line 18) (column 12) (len 15)) (segments (segment 0 (token "takePhoto") (name "takePhoto") (separator none) (span (offset 331) (line 18) (column 12) (len 9))) (segment 1 (token "shoot") (name "shoot") (separator dot) (span (offset 341) (line 18) (column 22) (len 5)))))
  )
  (root (package (name "Action Performance Example") (body brace (import (target (span (span (offset 55) (line 2) (column 17) (len 25))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 77) (line 2) (column 39) (len 3))) (separator (span (offset 77) (line 2) (column 39) (len 2))) (marker (span (offset 79) (line 2) (column 41) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Camera") (modifiers) (body semicolon)) (part-def (name "AutoFocus") (modifiers) (body semicolon)) (part-def (name "Imager") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "camera") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform (target (action (name "takePhoto") (short-name none) (typing none) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering ordered) (uniqueness none)) (subsets none) (redefines none) (references (relationship (kind references) (implied false) (targets (ref r2)))) (crosses none) (intersects none))) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "f") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform (target (reference (action (ref r4)) (redefines none))) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "i") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform (target (reference (action (ref r6)) (redefines none))) (value none) (body semicolon)))))))))
)
~~~
