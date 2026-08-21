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
    (reference r4 (scope relative) (span (offset 263) (line 10) (column 19) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 263) (line 10) (column 19) (len 5)))))
    (reference r5 (scope relative) (span (offset 291) (line 11) (column 22) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 291) (line 11) (column 22) (len 7)))))
    (reference r6 (scope relative) (span (offset 319) (line 13) (column 17) (len 5)) (segments (segment 0 (token "Focus") (name "Focus") (separator none) (span (offset 319) (line 13) (column 17) (len 5)))))
    (reference r7 (scope relative) (span (offset 346) (line 14) (column 20) (len 18)) (segments (segment 0 (token "TakePicture") (name "TakePicture") (separator none) (span (offset 346) (line 14) (column 20) (len 11))) (segment 1 (token "scene") (name "scene") (separator colon-colon) (span (offset 359) (line 14) (column 33) (len 5)))))
    (reference r8 (scope relative) (span (offset 404) (line 18) (column 13) (len 11)) (segments (segment 0 (token "focus") (name "focus") (separator none) (span (offset 404) (line 18) (column 13) (len 5))) (segment 1 (token "image") (name "image") (separator dot) (span (offset 410) (line 18) (column 19) (len 5)))))
    (reference r9 (scope relative) (span (offset 419) (line 18) (column 28) (len 11)) (segments (segment 0 (token "shoot") (name "shoot") (separator none) (span (offset 419) (line 18) (column 28) (len 5))) (segment 1 (token "image") (name "image") (separator dot) (span (offset 425) (line 18) (column 34) (len 5)))))
  )
  (root (package (name "Action Shorthand Example") (body brace (item-def (name "Scene") (modifiers) (individual false) (specializes none) (body semicolon)) (item-def (name "Image") (modifiers) (individual false) (specializes none) (body semicolon)) (item-def (name "Picture") (modifiers) (individual false) (specializes none) (body semicolon)) (action-def (name "Focus") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "scene") (subsets none) (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 112) (line 6) (column 21) (len 17))) (in-out (direction out) (reference false) (declaration "image") (subsets none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 130) (line 6) (column 39) (len 18))))) (action-def (name "Shoot") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "image") (subsets none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 171) (line 7) (column 21) (len 16))) (in-out (direction out) (reference false) (declaration "picture") (subsets none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 188) (line 7) (column 38) (len 22))))) (action-def (name "TakePicture") (modifiers) (specializes none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "scene") (short-name none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "picture") (short-name none) (type (ref r5)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (action-usage (name "focus") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "scene") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 346) (line 14) (column 20) (len 18)) (ref r7))))) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "image") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (flow-usage (kind flow) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r8)) (references none))) (to (connector-end (multiplicity none) (target (ref r9)) (references none))))) (body (body semicolon))) (then-action))))))
)
~~~
