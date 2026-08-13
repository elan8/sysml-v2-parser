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
    (reference r0 (scope relative) (span (offset 76) (line 4) (column 31) (len 8)) (segments (segment 0 (token "Exposure") (name "Exposure") (separator none) (span (offset 76) (line 4) (column 31) (len 8)))))
    (reference r1 (scope relative) (span (offset 116) (line 5) (column 29) (len 8)) (segments (segment 0 (token "Exposure") (name "Exposure") (separator none) (span (offset 116) (line 5) (column 29) (len 8)))))
  )
  (root (package (name "PictureTaking") (body (part-def (name "Exposure") (body semicolon)) (action-def (name "Focus") (specializes none) (body (in-out (direction out) (reference false) (declaration "xrsl") (type (ref r0)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 66) (line 4) (column 21) (len 19))))) (action-def (name "Shoot") (specializes none) (body (in-out (direction in) (reference false) (declaration "xsf") (type (ref r1)) (multiplicity none) (ordered false) (nonunique false) (redefines none) (value none) (span (offset 108) (line 5) (column 21) (len 17))))) (action-usage))))
)
~~~
