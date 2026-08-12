# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 14 (Action Definitions): Action Definition Example"))
~~~
# SOURCE
~~~sysml
package 'Action Definition Example' {
	item def Scene;
	item def Image;
	item def Picture;
	
	action def Focus { in scene : Scene; out image : Image; }
	action def Shoot { in image: Image; out picture : Picture; }	
		
	action def TakePicture { in scene : Scene; out picture : Picture;
		bind focus.scene = scene;
		
		action focus: Focus { in scene; out image; }
		
		flow from focus.image to shoot.image;
		
		action shoot: Shoot { in image; out picture; }
		
		bind shoot.picture = picture;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14_action_definition_example.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Action Definition Example' {
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
        in scene : Scene;
        out picture : Picture;
        bind focus.scene = scene;
        action focus : Focus {
            in scene;
            out image;
        }
        flow from from focus.image to shoot.image;
        action shoot : Shoot {
            in image;
            out picture;
        }
        bind shoot.picture = picture;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Action Definition Example") (body (item-def) (item-def) (item-def) (action-def) (action-def) (action-def))))
)
~~~
