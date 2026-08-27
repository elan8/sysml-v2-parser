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
        flow from focus.image to shoot.image;
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
    (reference r6 (scope relative) (span (offset 304) (line 10) (column 23) (len 11)) (segments (segment 0 (token "TakePicture") (name "TakePicture") (separator none) (span (offset 304) (line 10) (column 23) (len 11)))))
    (reference r7 (scope relative) (span (offset 375) (line 14) (column 18) (len 5)) (segments (segment 0 (token "Focus") (name "Focus") (separator none) (span (offset 375) (line 14) (column 18) (len 5)))))
    (reference r8 (scope relative) (span (offset 402) (line 15) (column 20) (len 18)) (segments (segment 0 (token "takePicture") (name "takePicture") (separator none) (span (offset 402) (line 15) (column 20) (len 11))) (segment 1 (token "scene") (name "scene") (separator colon-colon) (span (offset 415) (line 15) (column 33) (len 5)))))
    (reference r9 (scope relative) (span (offset 461) (line 19) (column 13) (len 11)) (segments (segment 0 (token "focus") (name "focus") (separator none) (span (offset 461) (line 19) (column 13) (len 5))) (segment 1 (token "image") (name "image") (separator dot) (span (offset 467) (line 19) (column 19) (len 5)))))
    (reference r10 (scope relative) (span (offset 476) (line 19) (column 28) (len 11)) (segments (segment 0 (token "shoot") (name "shoot") (separator none) (span (offset 476) (line 19) (column 28) (len 5))) (segment 1 (token "image") (name "image") (separator dot) (span (offset 482) (line 19) (column 34) (len 5)))))
    (reference r11 (scope relative) (span (offset 507) (line 21) (column 18) (len 5)) (segments (segment 0 (token "Shoot") (name "Shoot") (separator none) (span (offset 507) (line 21) (column 18) (len 5)))))
    (reference r12 (scope relative) (span (offset 550) (line 23) (column 23) (len 20)) (segments (segment 0 (token "takePicture") (name "takePicture") (separator none) (span (offset 550) (line 23) (column 23) (len 11))) (segment 1 (token "picture") (name "picture") (separator colon-colon) (span (offset 563) (line 23) (column 36) (len 7)))))
  )
  (root (package (name "Action Decomposition") (body brace (part-def (name "Scene") (modifiers) (body semicolon)) (part-def (name "Image") (modifiers) (body semicolon)) (part-def (name "Picture") (modifiers) (body semicolon)) (action-def (name "Focus") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "scene") (subsets none) (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 108) (line 6) (column 21) (len 17))) (in-out (direction out) (kind none) (reference false) (declaration "image") (subsets none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 126) (line 6) (column 39) (len 18))))) (action-def (name "Shoot") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "image") (subsets none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 167) (line 7) (column 21) (len 16))) (in-out (direction out) (kind none) (reference false) (declaration "picture") (subsets none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 184) (line 7) (column 38) (len 22))))) (action-def (name "TakePicture") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "scene") (subsets none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 236) (line 8) (column 27) (len 17))) (in-out (direction out) (kind none) (reference false) (declaration "picture") (subsets none) (type (ref r5)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 254) (line 8) (column 45) (len 22))))) (action-usage (keyword action) (name "takePicture") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "scene") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "picture") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (action-usage (keyword action) (name "focus") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "scene") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 402) (line 15) (column 20) (len 18)) (ref r8))))) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "image") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r9)) (references none))) (to (connector-end (multiplicity none) (target (ref r10)) (references none))))) (body (body semicolon))) (action-usage (keyword action) (name "shoot") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration none) (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "picture") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 550) (line 23) (column 23) (len 20)) (ref r12))))) (body semicolon)))))))))
)
~~~
