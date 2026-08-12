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
        flow from from focus.image to shoot.image;
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
    (reference r4 (scope relative) (span (offset 353) (line 15) (column 17) (len 5)) (segments (segment 0 (token "Focus") (name "Focus") (separator none) (span (offset 353) (line 15) (column 17) (len 5)))))
    (reference r5 (scope relative) (span (offset 438) (line 19) (column 9) (len 5)) (segments (segment 0 (token "focus") (name "focus") (separator none) (span (offset 438) (line 19) (column 9) (len 5)))))
    (reference r6 (scope relative) (span (offset 449) (line 19) (column 20) (len 5)) (segments (segment 0 (token "shoot") (name "shoot") (separator none) (span (offset 449) (line 19) (column 20) (len 5)))))
    (reference r7 (scope relative) (span (offset 475) (line 21) (column 17) (len 5)) (segments (segment 0 (token "Shoot") (name "Shoot") (separator none) (span (offset 475) (line 21) (column 17) (len 5)))))
  )
  (root (package (name "Action Succession Example-1") (body (item-def) (item-def) (item-def) (action-def (name "Focus") (specializes none) (body (in-out (direction in) (declaration "scene") (type (ref r0)) (redefines none) (value none) (span (offset 115) (line 6) (column 21) (len 17))) (in-out (direction out) (declaration "image") (type (ref r1)) (redefines none) (value none) (span (offset 133) (line 6) (column 39) (len 18))))) (action-def (name "Shoot") (specializes none) (body (in-out (direction in) (declaration "image") (type (ref r2)) (redefines none) (value none) (span (offset 174) (line 7) (column 21) (len 16))) (in-out (direction out) (declaration "picture") (type (ref r3)) (redefines none) (value none) (span (offset 191) (line 7) (column 38) (len 22))))) (action-def (name "TakePicture") (specializes none) (body (item-usage) (item-usage) (bind) (action-usage (declaration "focus") (type (ref r4))) (flow-usage) (first (source (expression (span (offset 438) (line 19) (column 9) (len 5)) (ref r5))) (target (expression (span (offset 449) (line 19) (column 20) (len 5)) (ref r6))) (body semicolon)) (action-usage (declaration "shoot") (type (ref r7))) (bind))))))
)
~~~
