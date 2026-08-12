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
        flow from from focus.image to shoot.image;
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
    (reference r7 (scope relative) (span (offset 425) (line 16) (column 17) (len 5)) (segments (segment 0 (token "Shoot") (name "Shoot") (separator none) (span (offset 425) (line 16) (column 17) (len 5)))))
  )
  (root (package (name "Action Definition Example") (body (item-def) (item-def) (item-def) (action-def (name "Focus") (specializes none) (body (in-out (direction in) (reference false) (declaration "scene") (type (ref r0)) (multiplicity none) (redefines none) (value none) (span (offset 113) (line 6) (column 21) (len 17))) (in-out (direction out) (reference false) (declaration "image") (type (ref r1)) (multiplicity none) (redefines none) (value none) (span (offset 131) (line 6) (column 39) (len 18))))) (action-def (name "Shoot") (specializes none) (body (in-out (direction in) (reference false) (declaration "image") (type (ref r2)) (multiplicity none) (redefines none) (value none) (span (offset 172) (line 7) (column 21) (len 16))) (in-out (direction out) (reference false) (declaration "picture") (type (ref r3)) (multiplicity none) (redefines none) (value none) (span (offset 189) (line 7) (column 38) (len 22))))) (action-def (name "TakePicture") (specializes none) (body (in-out (direction in) (reference false) (declaration "scene") (type (ref r4)) (multiplicity none) (redefines none) (value none) (span (offset 244) (line 9) (column 27) (len 17))) (in-out (direction out) (reference false) (declaration "picture") (type (ref r5)) (multiplicity none) (redefines none) (value none) (span (offset 262) (line 9) (column 45) (len 22))) (bind) (action-usage (declaration "focus") (type (ref r6))) (flow-usage) (action-usage (declaration "shoot") (type (ref r7))) (bind))))))
)
~~~
