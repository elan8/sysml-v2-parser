# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 15 (Actions): Action Decomposition"))
~~~
# SOURCE
~~~sysml
package 'Action Decomposition' {
	part def Scene;
	part def Image;
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
  (document "15_action_decomposition.md"
    (diagnostics
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 518) (line 22) (column 4) (len 13)) (message "unexpected token in action body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Action Decomposition' {
    part def Scene;
    part def Image;
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
    (reference r0 (scope relative) (span (offset 119) (line 6) (column 32) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 119) (line 6) (column 32) (len 5)))))
    (reference r1 (scope relative) (span (offset 138) (line 6) (column 51) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 138) (line 6) (column 51) (len 5)))))
    (reference r2 (scope relative) (span (offset 177) (line 7) (column 31) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 177) (line 7) (column 31) (len 5)))))
    (reference r3 (scope relative) (span (offset 198) (line 7) (column 52) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 198) (line 7) (column 52) (len 7)))))
    (reference r4 (scope relative) (span (offset 247) (line 8) (column 38) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 247) (line 8) (column 38) (len 5)))))
    (reference r5 (scope relative) (span (offset 268) (line 8) (column 59) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 268) (line 8) (column 59) (len 7)))))
  )
  (root (package (name "Action Decomposition") (body (part-def (name "Scene") (body semicolon)) (part-def (name "Image") (body semicolon)) (part-def (name "Picture") (body semicolon)) (action-def (name "Focus") (specializes none) (body (in-out (direction in) (declaration "scene") (type (ref r0)) (redefines none) (value none) (span (offset 108) (line 6) (column 21) (len 17))) (in-out (direction out) (declaration "image") (type (ref r1)) (redefines none) (value none) (span (offset 126) (line 6) (column 39) (len 18))))) (action-def (name "Shoot") (specializes none) (body (in-out (direction in) (declaration "image") (type (ref r2)) (redefines none) (value none) (span (offset 167) (line 7) (column 21) (len 16))) (in-out (direction out) (declaration "picture") (type (ref r3)) (redefines none) (value none) (span (offset 184) (line 7) (column 38) (len 22))))) (action-def (name "TakePicture") (specializes none) (body (in-out (direction in) (declaration "scene") (type (ref r4)) (redefines none) (value none) (span (offset 236) (line 8) (column 27) (len 17))) (in-out (direction out) (declaration "picture") (type (ref r5)) (redefines none) (value none) (span (offset 254) (line 8) (column 45) (len 22))))) (action-usage))))
)
~~~
