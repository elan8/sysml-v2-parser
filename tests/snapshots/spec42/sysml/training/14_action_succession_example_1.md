# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 14 (Action Definitions): Action Succession Example-1"))
~~~
# SOURCE
~~~sysml
package 'Action Succession Example-1' {
	item def Scene;
	item def Image;
	item def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
				
	action def TakePicture {
		in item scene : Scene;
		out item picture : Picture;
		
		bind focus.scene = scene;
		
		action focus: Focus { in scene; out image; }
		
		flow from focus.image to shoot.image;
		
		first focus then shoot;
		
		action shoot: Shoot { in image; out picture; }
		
		bind shoot.picture = picture;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14_action_succession_example_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Action Succession Example-1' {
    item def Scene;
    item def Image;
    item def Picture;
    action def Focus {
        in scene : Scene;
        out image : Image;
    }
    action def Shoot {
        in image : Image;
        out picture : Picture;
    }
    action def TakePicture {
        in item scene : Scene;
        out item picture : Picture;
        bind focus.scene = scene;
        action focus : Focus {
            in scene;
            out image;
        }
        flow from focus.image to shoot.image;
        first focus then shoot;
        action shoot : Shoot {
            in image;
            out picture;
        }
        bind shoot.picture = picture;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 126) (line 6) (column 32) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 126) (line 6) (column 32) (len 5)))))
    (reference r1 (scope relative) (span (offset 145) (line 6) (column 51) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 145) (line 6) (column 51) (len 5)))))
    (reference r2 (scope relative) (span (offset 184) (line 7) (column 31) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 184) (line 7) (column 31) (len 5)))))
    (reference r3 (scope relative) (span (offset 205) (line 7) (column 52) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 205) (line 7) (column 52) (len 7)))))
    (reference r4 (scope relative) (span (offset 266) (line 10) (column 19) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 266) (line 10) (column 19) (len 5)))))
    (reference r5 (scope relative) (span (offset 294) (line 11) (column 22) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 294) (line 11) (column 22) (len 7)))))
    (reference r6 (scope relative) (span (offset 353) (line 15) (column 17) (len 5)) (segments (segment 0 (token "Focus") (name "Focus") (separator none) (span (offset 353) (line 15) (column 17) (len 5)))))
    (reference r7 (scope relative) (span (offset 438) (line 19) (column 9) (len 5)) (segments (segment 0 (token "focus") (name "focus") (separator none) (span (offset 438) (line 19) (column 9) (len 5)))))
    (reference r8 (scope relative) (span (offset 449) (line 19) (column 20) (len 5)) (segments (segment 0 (token "shoot") (name "shoot") (separator none) (span (offset 449) (line 19) (column 20) (len 5)))))
    (reference r9 (scope relative) (span (offset 475) (line 21) (column 17) (len 5)) (segments (segment 0 (token "Shoot") (name "Shoot") (separator none) (span (offset 475) (line 21) (column 17) (len 5)))))
  )
  (root (package (name "Action Succession Example-1") (body brace (item-def (name "Scene") (modifiers) (individual false) (specializes none) (body semicolon)) (item-def (name "Image") (modifiers) (individual false) (specializes none) (body semicolon)) (item-def (name "Picture") (modifiers) (individual false) (specializes none) (body semicolon)) (action-def (name "Focus") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "scene") (subsets none) (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 115) (line 6) (column 21) (len 17))) (in-out (direction out) (reference false) (declaration "image") (subsets none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 133) (line 6) (column 39) (len 18))))) (action-def (name "Shoot") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "image") (subsets none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 174) (line 7) (column 21) (len 16))) (in-out (direction out) (reference false) (declaration "picture") (subsets none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 191) (line 7) (column 38) (len 22))))) (action-def (name "TakePicture") (modifiers) (specializes none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "scene") (short-name none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "picture") (short-name none) (type (ref r5)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (bind) (action-usage (name "focus") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration) (in-out-declaration))) (flow-usage) (first (source (expression (span (offset 438) (line 19) (column 9) (len 5)) (ref r7))) (target (expression (span (offset 449) (line 19) (column 20) (len 5)) (ref r8))) (body semicolon (span (span (offset 454) (line 19) (column 25) (len 1))))) (action-usage (name "shoot") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration) (in-out-declaration))) (bind))))))
)
~~~
