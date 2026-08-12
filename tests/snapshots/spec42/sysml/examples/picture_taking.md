# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Camera): PictureTaking"))
~~~
# SOURCE
~~~sysml
package PictureTaking {
	part def Exposure;
	
	action def Focus { out xrsl: Exposure; }
	action def Shoot { in xsf: Exposure; }	
		
	action takePicture {		
		action focus: Focus[1];
		flow of Exposure from focus.xrsl to shoot.xsf;
		action shoot: Shoot[1];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "picture_taking.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package PictureTaking {
    part def Exposure;
    action def Focus {
        out xrsl : Exposure;
    }
    action def Shoot {
        in xsf : Exposure;
    }
    action takePicture {
        action focus : Focus[1];
        flow  of Exposure from focus.xrsl to shoot.xsf;
        action shoot : Shoot[1];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "PictureTaking") (body (part-def (name "Exposure") (body semicolon)) (action-def) (action-def) (action-usage))))
)
~~~
