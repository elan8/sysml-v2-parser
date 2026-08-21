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
        flow of Exposure from focus.xrsl to shoot.xsf;
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
    (reference r2 (scope relative) (span (offset 172) (line 8) (column 17) (len 5)) (segments (segment 0 (token "Focus") (name "Focus") (separator none) (span (offset 172) (line 8) (column 17) (len 5)))))
    (reference r3 (scope relative) (span (offset 247) (line 10) (column 17) (len 5)) (segments (segment 0 (token "Shoot") (name "Shoot") (separator none) (span (offset 247) (line 10) (column 17) (len 5)))))
  )
  (root (package (name "PictureTaking") (body brace (part-def (name "Exposure") (modifiers) (body semicolon)) (action-def (name "Focus") (modifiers) (specializes none) (body brace (in-out (direction out) (reference false) (declaration "xrsl") (subsets none) (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 66) (line 4) (column 21) (len 19))))) (action-def (name "Shoot") (modifiers) (specializes none) (body brace (in-out (direction in) (reference false) (declaration "xsf") (subsets none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 108) (line 5) (column 21) (len 17))))) (action-usage (name "takePicture") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (action-usage (name "focus") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity (lower (expression (span (offset 178) (line 8) (column 23) (len 1)) (integer 1))) (upper (expression (span (offset 178) (line 8) (column 23) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (flow-usage) (action-usage (name "shoot") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity (lower (expression (span (offset 253) (line 10) (column 23) (len 1)) (integer 1))) (upper (expression (span (offset 253) (line 10) (column 23) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))))))
)
~~~
