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
      (diagnostic (code "recovered_part_usage_body_element") (severity error) (category parseerror) (span (offset 172) (line 10) (column 3) (len 69)) (message "unexpected token in part usage body"))
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
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 55) (line 2) (column 17) (len 22)) (segments (segment 0 (token "'Action Decomposition'") (name "Action Decomposition") (separator none) (span (offset 55) (line 2) (column 17) (len 22)))))
    (reference r1 (scope relative) (span (offset 158) (line 8) (column 16) (len 6)) (segments (segment 0 (token "Camera") (name "Camera") (separator none) (span (offset 158) (line 8) (column 16) (len 6)))))
    (reference r2 (scope relative) (span (offset 250) (line 13) (column 12) (len 9)) (segments (segment 0 (token "AutoFocus") (name "AutoFocus") (separator none) (span (offset 250) (line 13) (column 12) (len 9)))))
    (reference r3 (scope relative) (span (offset 311) (line 17) (column 12) (len 6)) (segments (segment 0 (token "Imager") (name "Imager") (separator none) (span (offset 311) (line 17) (column 12) (len 6)))))
  )
  (root (package (name "Action Performance Example") (body brace (import (target (span (span (offset 55) (line 2) (column 17) (len 25))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 77) (line 2) (column 39) (len 3))) (separator (span (offset 77) (line 2) (column 39) (len 2))) (marker (span (offset 79) (line 2) (column 41) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Camera") (body semicolon)) (part-def (name "AutoFocus") (body semicolon)) (part-def (name "Imager") (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "camera") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (malformed (code "recovered_part_usage_body_element") (found "perform action takePhoto[*] ordered") (span (offset 172) (line 10) (column 3) (len 69))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "f") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (perform))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "i") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (perform))))))))
)
~~~
