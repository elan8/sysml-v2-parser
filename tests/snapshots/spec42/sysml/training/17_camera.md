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
  )
  (root (package (name "Camera") (body brace (import (target (span (span (offset 33) (line 2) (column 17) (len 25))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 55) (line 2) (column 39) (len 3))) (separator (span (offset 55) (line 2) (column 39) (len 2))) (marker (span (offset 57) (line 2) (column 41) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Camera") (body semicolon)) (part-def (name "FocusingSubsystem") (body semicolon)) (part-def (name "ImagingSubsystem") (body semicolon)) (part-usage (declaration-name "camera") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (body brace (ref (name "scene") (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind item) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (redefines none) (subsets none) (body semicolon)) (part-usage) (part-usage) (flow-usage) (part-usage))))))
)
~~~
