# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 17 (Control): Merge Example"))
~~~
# SOURCE
~~~sysml
package 'Merge Example' {
	part def Scene;
	part def Image;
	part def Picture;
	
	action def Focus { in item scene : Scene; out item image : Image; }
	action def Shoot { in item image : Image; out item picture : Picture; }
	action def Display { in item picture : Picture; }
	action def TakePicture;
	
	action takePicture : TakePicture {
		first start;
		
		then merge continue;
			
		then action trigger {
			out item scene : Scene;
		}
		
		flow from trigger.scene to focus.scene;
		
		then action focus : Focus {
			in item scene;
			out item image;
		}
		
		flow from focus.image to shoot.image;
		
		then action shoot : Shoot {
			in item image ;
			out item picture;
		}
		
		flow from shoot.picture to display.picture;
		
		then action display : Display {
			in item picture;
		}
		
		then continue;	
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "17_merge_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Merge Example' {
    part def Scene;
    part def Image;
    part def Picture;
    action def Focus {
        in item scene : Scene;
        out item image : Image;
    }
    action def Shoot {
        in item image : Image;
        out item picture : Picture;
    }
    action def Display {
        in item picture : Picture;
    }
    action def TakePicture;
    action takePicture : TakePicture {
        first start;
        then merge continue;
        then action trigger {
            out item scene : Scene;
        }
        flow from from trigger.scene to focus.scene;
        then action focus : Focus {
            in item scene;
            out item image;
        }
        flow from from focus.image to shoot.image;
        then action shoot : Shoot {
            in item image;
            out item picture;
        }
        flow from from shoot.picture to display.picture;
        then action display : Display {
            in item picture;
        }
        then continue;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Merge Example") (body (part-def (name "Scene") (body semicolon)) (part-def (name "Image") (body semicolon)) (part-def (name "Picture") (body semicolon)) (action-def (name "Focus") (specializes none) (body (item-usage) (item-usage))) (action-def (name "Shoot") (specializes none) (body (item-usage) (item-usage))) (action-def (name "Display") (specializes none) (body (item-usage))) (action-def (name "TakePicture") (specializes none) (body semicolon)) (action-usage))))
)
~~~
