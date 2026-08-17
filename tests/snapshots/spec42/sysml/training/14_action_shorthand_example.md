# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 14 (Action Definitions): Action Shorthand Example"))
~~~
# SOURCE
~~~sysml
package 'Action Shorthand Example' {
	item def Scene;
	item def Image;
	item def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
				
	action def TakePicture {
		in item scene : Scene;
		out item picture : Picture;
		
		action focus: Focus {
			in item scene = TakePicture::scene;
			out item image;
		}
		
		flow from focus.image to shoot.image;
		
		then action shoot: Shoot {
			in item;
			out item picture = TakePicture::picture;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14_action_shorthand_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Action Shorthand Example' {
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
        action focus : Focus {
            in item scene = TakePicture::scene;
            out item image;
        }
        flow from focus.image to shoot.image;
        then action shoot : Shoot {
            in item;
            out item picture = TakePicture::picture;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 123) (line 6) (column 32) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 123) (line 6) (column 32) (len 5)))))
    (reference r1 (scope relative) (span (offset 142) (line 6) (column 51) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 142) (line 6) (column 51) (len 5)))))
    (reference r2 (scope relative) (span (offset 181) (line 7) (column 31) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 181) (line 7) (column 31) (len 5)))))
    (reference r3 (scope relative) (span (offset 202) (line 7) (column 52) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 202) (line 7) (column 52) (len 7)))))
    (reference r4 (scope relative) (span (offset 319) (line 13) (column 17) (len 5)) (segments (segment 0 (token "Focus") (name "Focus") (separator none) (span (offset 319) (line 13) (column 17) (len 5)))))
  )
  (root (package (name "Action Shorthand Example") (body brace (item-def) (item-def) (item-def) (action-def (name "Focus") (specializes none) (body brace (in-out (direction in) (reference false) (declaration "scene") (subsets none) (type (ref r0)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 112) (line 6) (column 21) (len 17))) (in-out (direction out) (reference false) (declaration "image") (subsets none) (type (ref r1)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 130) (line 6) (column 39) (len 18))))) (action-def (name "Shoot") (specializes none) (body brace (in-out (direction in) (reference false) (declaration "image") (subsets none) (type (ref r2)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 171) (line 7) (column 21) (len 16))) (in-out (direction out) (reference false) (declaration "picture") (subsets none) (type (ref r3)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 188) (line 7) (column 38) (len 22))))) (action-def (name "TakePicture") (specializes none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "scene")) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "picture")) (action-usage (declaration "focus") (type (ref r4))) (flow-usage) (then-action))))))
)
~~~
