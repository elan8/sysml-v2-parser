# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 17 (Control): Camera"))
~~~
# SOURCE
~~~sysml
package Camera {
	private import 'Action Decomposition'::*;
	
	part def Camera;
	part def FocusingSubsystem;
	part def ImagingSubsystem;
	
	part camera : Camera {
		ref item scene : Scene;
		part photos : Picture[*];
				
		part autoFocus {
			in ref item scene : Scene = camera::scene;		
			out ref item realImage : Image;
		}
		
		flow autoFocus.realImage to imager.focusedImage;
		
		part imager {
			in item focusedImage : Image;		
			out item photo : Picture :> photos;
		}
		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "17_camera.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Camera {
    private import 'Action Decomposition'::*;
    part def Camera;
    part def FocusingSubsystem;
    part def ImagingSubsystem;
    part camera : Camera {
        ref item scene : Scene;
        part photos : Picture[*];
        part autoFocus {
            in ref item scene : Scene = camera::scene;
            out ref item realImage : Image;
        }
        flow from autoFocus.realImage to imager.focusedImage;
        part imager {
            in item focusedImage : Image;
            out item photo : Picture :> photos;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 33) (line 2) (column 17) (len 22)) (segments (segment 0 (token "'Action Decomposition'") (name "Action Decomposition") (separator none) (span (offset 33) (line 2) (column 17) (len 22)))))
    (reference r1 (scope relative) (span (offset 154) (line 8) (column 16) (len 6)) (segments (segment 0 (token "Camera") (name "Camera") (separator none) (span (offset 154) (line 8) (column 16) (len 6)))))
    (reference r2 (scope relative) (span (offset 182) (line 9) (column 20) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 182) (line 9) (column 20) (len 5)))))
    (reference r3 (scope relative) (span (offset 205) (line 10) (column 17) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 205) (line 10) (column 17) (len 7)))))
    (reference r4 (scope relative) (span (offset 264) (line 13) (column 24) (len 5)) (segments (segment 0 (token "Scene") (name "Scene") (separator none) (span (offset 264) (line 13) (column 24) (len 5)))))
    (reference r5 (scope relative) (span (offset 272) (line 13) (column 32) (len 13)) (segments (segment 0 (token "camera") (name "camera") (separator none) (span (offset 272) (line 13) (column 32) (len 6))) (segment 1 (token "scene") (name "scene") (separator colon-colon) (span (offset 280) (line 13) (column 40) (len 5)))))
    (reference r6 (scope relative) (span (offset 317) (line 14) (column 29) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 317) (line 14) (column 29) (len 5)))))
    (reference r7 (scope relative) (span (offset 338) (line 17) (column 8) (len 19)) (segments (segment 0 (token "autoFocus") (name "autoFocus") (separator none) (span (offset 338) (line 17) (column 8) (len 9))) (segment 1 (token "realImage") (name "realImage") (separator dot) (span (offset 348) (line 17) (column 18) (len 9)))))
    (reference r8 (scope relative) (span (offset 361) (line 17) (column 31) (len 19)) (segments (segment 0 (token "imager") (name "imager") (separator none) (span (offset 361) (line 17) (column 31) (len 6))) (segment 1 (token "focusedImage") (name "focusedImage") (separator dot) (span (offset 368) (line 17) (column 38) (len 12)))))
    (reference r9 (scope relative) (span (offset 427) (line 20) (column 27) (len 5)) (segments (segment 0 (token "Image") (name "Image") (separator none) (span (offset 427) (line 20) (column 27) (len 5)))))
    (reference r10 (scope relative) (span (offset 456) (line 21) (column 21) (len 7)) (segments (segment 0 (token "Picture") (name "Picture") (separator none) (span (offset 456) (line 21) (column 21) (len 7)))))
    (reference r11 (scope relative) (span (offset 467) (line 21) (column 32) (len 6)) (segments (segment 0 (token "photos") (name "photos") (separator none) (span (offset 467) (line 21) (column 32) (len 6)))))
  )
  (root (package (name "Camera") (body brace (import (target (span (span (offset 33) (line 2) (column 17) (len 25))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 55) (line 2) (column 39) (len 3))) (separator (span (offset 55) (line 2) (column 39) (len 2))) (marker (span (offset 57) (line 2) (column 41) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Camera") (modifiers) (body semicolon)) (part-def (name "FocusingSubsystem") (modifiers) (body semicolon)) (part-def (name "ImagingSubsystem") (modifiers) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "camera") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration "scene") (short-name none) (type (ref r2)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "photos") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "autoFocus") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration "scene") (short-name none) (type (ref r4)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 272) (line 13) (column 32) (len 13)) (ref r5))))) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration "realImage") (short-name none) (type (ref r6)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r7)) (references none))) (to (connector-end (multiplicity none) (target (ref r8)) (references none))))) (body (body semicolon))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "imager") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (item-usage (prefix (direction in) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "focusedImage") (short-name none) (type (ref r9)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "photo") (short-name none) (type (ref r10)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r11)))) (redefines none) (value none) (body semicolon)))))))))
)
~~~
