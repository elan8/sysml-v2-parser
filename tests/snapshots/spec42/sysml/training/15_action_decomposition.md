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
  )
  (root (package (name "Action Decomposition") (body (part-def (name "Scene") (body semicolon)) (part-def (name "Image") (body semicolon)) (part-def (name "Picture") (body semicolon)) (action-def) (action-def) (action-def) (action-usage))))
)
~~~
