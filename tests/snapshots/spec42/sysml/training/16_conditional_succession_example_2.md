# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 16 (Conditional Succession): Conditional Succession Example-2"))
~~~
# SOURCE
~~~sysml
package 'Conditional Succession Example-2' {
	part def Scene;
	part def Image {
		isWellFocused: ScalarValues::Boolean;
	}
	part def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
	action def TakePicture { in scene : Scene; out picture : Picture; }
	
	action takePicture : TakePicture {
		in item scene;
		out item picture;
		
		action focus : Focus {
			in item scene = takePicture::scene; 
			out item image;
		}
		
		if focus.image.isWellFocused then shoot;
		
		flow from focus.image to shoot.image;

		action shoot : Shoot {
			in item image; 
			out item picture = takePicture::picture;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "16_conditional_succession_example_2.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Conditional Succession Example-2' {
    part def Scene;
    part def Image {
        isWellFocused : ScalarValues::Boolean;
    }
    part def Picture;
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
    }
    action takePicture : TakePicture {
        in item scene;
        out item picture;
        action focus : Focus {
            in item scene = takePicture::scene;
            out item image;
        }
        if focus.image.isWellFocused then shoot;
        flow from focus.image to shoot.image;
        action shoot : Shoot {
            in item image;
            out item picture = takePicture::picture;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 97) (line 4) (column 18) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 97) (line 4) (column 18) (len 12))) (segment 1 (token "Boolean") (name "Boolean") (separator colon-colon) (span (offset 111) (line 4) (column 32) (len 7)))))
    (reference r1 (scope relative) (span (offset 175) (line 8) (column 32) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 175) (line 8) (column 32) (len 5)))))
    (reference r2 (scope relative) (span (offset 194) (line 8) (column 51) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 194) (line 8) (column 51) (len 5)))))
    (reference r3 (scope relative) (span (offset 233) (line 9) (column 31) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 233) (line 9) (column 31) (len 5)))))
    (reference r4 (scope relative) (span (offset 254) (line 9) (column 52) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 254) (line 9) (column 52) (len 7)))))
    (reference r5 (scope relative) (span (offset 303) (line 10) (column 38) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 303) (line 10) (column 38) (len 5)))))
    (reference r6 (scope relative) (span (offset 324) (line 10) (column 59) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 324) (line 10) (column 59) (len 7)))))
    (reference r7 (scope relative) (span (offset 457) (line 17) (column 20) (len 18)) (segments (segment 0 (token "takePicture") (name "takePicture") (separator none) (span (offset 457) (line 17) (column 20) (len 11))) (segment 1 (token "scene") (name "scene") (separator colon-colon) (span (offset 470) (line 17) (column 33) (len 5)))))
    (reference r8 (scope relative) (span (offset 657) (line 27) (column 23) (len 20)) (segments (segment 0 (token "takePicture") (name "takePicture") (separator none) (span (offset 657) (line 27) (column 23) (len 11))) (segment 1 (token "picture") (name "picture") (separator colon-colon) (span (offset 670) (line 27) (column 36) (len 7)))))
  )
  (root (package (name "Conditional Succession Example-2") (body brace (part-def (name "Scene") (modifiers) (body semicolon)) (part-def (name "Image") (modifiers) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "isWellFocused") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Picture") (modifiers) (body semicolon)) (action-def (name "Focus") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "scene") (subsets none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 164) (line 8) (column 21) (len 17))) (in-out (direction out) (reference false) (declaration "image") (subsets none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 182) (line 8) (column 39) (len 18))))) (action-def (name "Shoot") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "image") (subsets none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 223) (line 9) (column 21) (len 16))) (in-out (direction out) (reference false) (declaration "picture") (subsets none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 240) (line 9) (column 38) (len 22))))) (action-def (name "TakePicture") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "scene") (subsets none) (type (ref r5)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 292) (line 10) (column 27) (len 17))) (in-out (direction out) (reference false) (declaration "picture") (subsets none) (type (ref r6)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 310) (line 10) (column 45) (len 22))))) (action-usage (name "takePicture") (short-name none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "scene") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "picture") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (action-usage (name "focus") (short-name none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "scene") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 457) (line 17) (column 20) (len 18)) (ref r7))))) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "image") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (if) (flow-usage) (action-usage (name "shoot") (short-name none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "image") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "picture") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 657) (line 27) (column 23) (len 20)) (ref r8))))) (body semicolon)))))))))
)
~~~
