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
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 467) (line 21) (column 4) (len 12)) (message "unexpected token in action body"))
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
        flow from from focus.image to shoot.image;
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
  )
  (root (package (name "Action Shorthand Example") (body (item-def) (item-def) (item-def) (action-def) (action-def) (action-def))))
)
~~~
