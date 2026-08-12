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
        if focus.image.isWellFocused  {
            then shoot;
        }
        flow from from focus.image to shoot.image;
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
  )
  (root (package (name "Conditional Succession Example-2") (body (part-def (name "Scene") (body semicolon)) (part-def (name "Image") (body (default-reference-usage))) (part-def (name "Picture") (body semicolon)) (action-def) (action-def) (action-def) (action-usage))))
)
~~~
