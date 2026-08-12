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
  )
  (root (package (name "Conditional Succession Example-1") (body (part-def (name "Scene") (body semicolon)) (part-def (name "Image") (body (default-reference-usage))) (part-def (name "Picture") (body semicolon)) (action-def) (action-def) (action-def) (action-usage))))
)
~~~
