# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 16 (Conditional Succession): Conditional Succession Example-1"))
~~~
# SOURCE
~~~sysml
package 'Conditional Succession Example-1' {
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
				
		first focus 
			if focus.image.isWellFocused then shoot;
		
		flow from focus.image to shoot.image;

		action shoot : Shoot {
			in item; 
			out item picture = takePicture::picture;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "16_conditional_succession_example_1.md"
    (diagnostics
      (diagnostic (code "missing_semicolon") (severity error) (category parseerror) (span (offset 508) (line 21) (column 3) (len 62)) (message "missing semicolon before next declaration"))
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 637) (line 27) (column 4) (len 13)) (message "unexpected token in action body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Conditional Succession Example-1' {
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
        first focus 
			if focus.image.isWellFocused then shoot;
        flow from from focus.image to shoot.image;
        action shoot : Shoot {
            in item;
            out item picture = takePicture::picture;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 175) (line 8) (column 32) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 175) (line 8) (column 32) (len 5)))))
    (reference r1 (scope relative) (span (offset 194) (line 8) (column 51) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 194) (line 8) (column 51) (len 5)))))
    (reference r2 (scope relative) (span (offset 233) (line 9) (column 31) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 233) (line 9) (column 31) (len 5)))))
    (reference r3 (scope relative) (span (offset 254) (line 9) (column 52) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 254) (line 9) (column 52) (len 7)))))
    (reference r4 (scope relative) (span (offset 303) (line 10) (column 38) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 303) (line 10) (column 38) (len 5)))))
    (reference r5 (scope relative) (span (offset 324) (line 10) (column 59) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 324) (line 10) (column 59) (len 7)))))
  )
  (root (package (name "Conditional Succession Example-1") (body (part-def (name "Scene") (body semicolon)) (part-def (name "Image") (body (default-reference-usage))) (part-def (name "Picture") (body semicolon)) (action-def (name "Focus") (specializes none) (body (in-out (direction in) (reference false) (declaration "scene") (type (ref r0)) (multiplicity none) (redefines none) (value none) (span (offset 164) (line 8) (column 21) (len 17))) (in-out (direction out) (reference false) (declaration "image") (type (ref r1)) (multiplicity none) (redefines none) (value none) (span (offset 182) (line 8) (column 39) (len 18))))) (action-def (name "Shoot") (specializes none) (body (in-out (direction in) (reference false) (declaration "image") (type (ref r2)) (multiplicity none) (redefines none) (value none) (span (offset 223) (line 9) (column 21) (len 16))) (in-out (direction out) (reference false) (declaration "picture") (type (ref r3)) (multiplicity none) (redefines none) (value none) (span (offset 240) (line 9) (column 38) (len 22))))) (action-def (name "TakePicture") (specializes none) (body (in-out (direction in) (reference false) (declaration "scene") (type (ref r4)) (multiplicity none) (redefines none) (value none) (span (offset 292) (line 10) (column 27) (len 17))) (in-out (direction out) (reference false) (declaration "picture") (type (ref r5)) (multiplicity none) (redefines none) (value none) (span (offset 310) (line 10) (column 45) (len 22))))) (action-usage))))
)
~~~
