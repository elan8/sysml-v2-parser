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
        flow from trigger.scene to focus.scene;
        then action focus : Focus {
            in item scene;
            out item image;
        }
        flow from focus.image to shoot.image;
        then action shoot : Shoot {
            in item image;
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
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 117) (line 6) (column 37) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 117) (line 6) (column 37) (len 5)))))
    (reference r1 (scope relative) (span (offset 141) (line 6) (column 61) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 141) (line 6) (column 61) (len 5)))))
    (reference r2 (scope relative) (span (offset 186) (line 7) (column 37) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 186) (line 7) (column 37) (len 5)))))
    (reference r3 (scope relative) (span (offset 212) (line 7) (column 63) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 212) (line 7) (column 63) (len 7)))))
    (reference r4 (scope relative) (span (offset 263) (line 8) (column 41) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 263) (line 8) (column 41) (len 7)))))
    (reference r5 (scope relative) (span (offset 323) (line 11) (column 23) (len 11)) (segments (segment 0 (token "TakePicture") (name "TakePicture") (separator none) (span (offset 323) (line 11) (column 23) (len 11)))))
    (reference r6 (scope relative) (span (offset 368) (line 14) (column 14) (len 8)) (segments (segment 0 (token "continue") (name "continue") (separator none) (span (offset 368) (line 14) (column 14) (len 8)))))
    (reference r7 (scope relative) (span (offset 452) (line 20) (column 13) (len 13)) (segments (segment 0 (token "trigger") (name "trigger") (separator none) (span (offset 452) (line 20) (column 13) (len 7))) (segment 1 (token "scene") (name "scene") (separator dot) (span (offset 460) (line 20) (column 21) (len 5)))))
    (reference r8 (scope relative) (span (offset 469) (line 20) (column 30) (len 11)) (segments (segment 0 (token "focus") (name "focus") (separator none) (span (offset 469) (line 20) (column 30) (len 5))) (segment 1 (token "scene") (name "scene") (separator dot) (span (offset 475) (line 20) (column 36) (len 5)))))
    (reference r9 (scope relative) (span (offset 571) (line 27) (column 13) (len 11)) (segments (segment 0 (token "focus") (name "focus") (separator none) (span (offset 571) (line 27) (column 13) (len 5))) (segment 1 (token "image") (name "image") (separator dot) (span (offset 577) (line 27) (column 19) (len 5)))))
    (reference r10 (scope relative) (span (offset 586) (line 27) (column 28) (len 11)) (segments (segment 0 (token "shoot") (name "shoot") (separator none) (span (offset 586) (line 27) (column 28) (len 5))) (segment 1 (token "image") (name "image") (separator dot) (span (offset 592) (line 27) (column 34) (len 5)))))
    (reference r11 (scope relative) (span (offset 691) (line 34) (column 13) (len 13)) (segments (segment 0 (token "shoot") (name "shoot") (separator none) (span (offset 691) (line 34) (column 13) (len 5))) (segment 1 (token "picture") (name "picture") (separator dot) (span (offset 697) (line 34) (column 19) (len 7)))))
    (reference r12 (scope relative) (span (offset 708) (line 34) (column 30) (len 15)) (segments (segment 0 (token "display") (name "display") (separator none) (span (offset 708) (line 34) (column 30) (len 7))) (segment 1 (token "picture") (name "picture") (separator dot) (span (offset 716) (line 34) (column 38) (len 7)))))
  )
  (root (package (name "Merge Example") (body brace (part-def (name "Scene") (modifiers) (body semicolon)) (part-def (name "Image") (modifiers) (body semicolon)) (part-def (name "Picture") (modifiers) (body semicolon)) (action-def (name "Focus") (modifiers) (specializes none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "scene") (short-name none) (type (ref r0)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "image") (short-name none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (action-def (name "Shoot") (modifiers) (specializes none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "image") (short-name none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "picture") (short-name none) (type (ref r3)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (action-def (name "Display") (modifiers) (specializes none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "picture") (short-name none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (action-def (name "TakePicture") (modifiers) (specializes none) (body semicolon)) (action-usage (keyword action) (name "takePicture") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (first) (then-control (merge (declaration (named (expression (span (offset 368) (line 14) (column 14) (len 8)) (ref r6)))) (body semicolon (span (span (offset 376) (line 14) (column 22) (len 1)))))) (then-action) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r7)) (references none))) (to (connector-end (multiplicity none) (target (ref r8)) (references none))))) (body (body semicolon))) (then-action) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r9)) (references none))) (to (connector-end (multiplicity none) (target (ref r10)) (references none))))) (body (body semicolon))) (then-action) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r11)) (references none))) (to (connector-end (multiplicity none) (target (ref r12)) (references none))))) (body (body semicolon))) (then-action) (then-action))))))
)
~~~
