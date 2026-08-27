# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 14 (Action Definitions): Action Definition Example"))
~~~
# SOURCE
~~~sysml
package 'Action Definition Example' {
	item def Scene;
	item def Image;
	item def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
		
	action def TakePicture { in scene : Scene; out picture : Picture;
		bind focus.scene = scene;
		
		action focus: Focus { in scene; out image; }
		
		flow from focus.image to shoot.image;
		
		action shoot: Shoot { in image; out picture; }
		
		bind shoot.picture = picture;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14_action_definition_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Action Definition Example' {
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
        in scene : Scene;
        out picture : Picture;
        bind focus.scene = scene;
        action focus : Focus {
            in scene;
            out image;
        }
        flow from focus.image to shoot.image;
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
    (reference r0 (scope relative) (span (offset 124) (line 6) (column 32) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 124) (line 6) (column 32) (len 5)))))
    (reference r1 (scope relative) (span (offset 143) (line 6) (column 51) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 143) (line 6) (column 51) (len 5)))))
    (reference r2 (scope relative) (span (offset 182) (line 7) (column 31) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 182) (line 7) (column 31) (len 5)))))
    (reference r3 (scope relative) (span (offset 203) (line 7) (column 52) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 203) (line 7) (column 52) (len 7)))))
    (reference r4 (scope relative) (span (offset 255) (line 9) (column 38) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 255) (line 9) (column 38) (len 5)))))
    (reference r5 (scope relative) (span (offset 276) (line 9) (column 59) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 276) (line 9) (column 59) (len 7)))))
    (reference r6 (scope relative) (span (offset 332) (line 12) (column 17) (len 5)) (segments (segment 0 (token "Focus") (name "Focus") (separator none) (span (offset 332) (line 12) (column 17) (len 5)))))
    (reference r7 (scope relative) (span (offset 378) (line 14) (column 13) (len 11)) (segments (segment 0 (token "focus") (name "focus") (separator none) (span (offset 378) (line 14) (column 13) (len 5))) (segment 1 (token "image") (name "image") (separator dot) (span (offset 384) (line 14) (column 19) (len 5)))))
    (reference r8 (scope relative) (span (offset 393) (line 14) (column 28) (len 11)) (segments (segment 0 (token "shoot") (name "shoot") (separator none) (span (offset 393) (line 14) (column 28) (len 5))) (segment 1 (token "image") (name "image") (separator dot) (span (offset 399) (line 14) (column 34) (len 5)))))
    (reference r9 (scope relative) (span (offset 425) (line 16) (column 17) (len 5)) (segments (segment 0 (token "Shoot") (name "Shoot") (separator none) (span (offset 425) (line 16) (column 17) (len 5)))))
  )
  (root (package (name "Action Definition Example") (body brace (item-def (name "Scene") (modifiers) (individual false) (specializes none) (body semicolon)) (item-def (name "Image") (modifiers) (individual false) (specializes none) (body semicolon)) (item-def (name "Picture") (modifiers) (individual false) (specializes none) (body semicolon)) (action-def (name "Focus") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "scene") (subsets none) (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 113) (line 6) (column 21) (len 17))) (in-out (direction out) (kind none) (reference false) (declaration "image") (subsets none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 131) (line 6) (column 39) (len 18))))) (action-def (name "Shoot") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "image") (subsets none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 172) (line 7) (column 21) (len 16))) (in-out (direction out) (kind none) (reference false) (declaration "picture") (subsets none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 189) (line 7) (column 38) (len 22))))) (action-def (name "TakePicture") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "scene") (subsets none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 244) (line 9) (column 27) (len 17))) (in-out (direction out) (kind none) (reference false) (declaration "picture") (subsets none) (type (ref r5)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 262) (line 9) (column 45) (len 22))) (bind) (action-usage (keyword action) (name "focus") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration) (in-out-declaration))) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r7)) (references none))) (to (connector-end (multiplicity none) (target (ref r8)) (references none))))) (body (body semicolon))) (action-usage (keyword action) (name "shoot") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (in-out-declaration) (in-out-declaration))) (bind))))))
)
~~~
