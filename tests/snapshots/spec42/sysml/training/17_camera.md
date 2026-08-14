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
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 244) (line 13) (column 4) (len 82)) (message "unexpected keyword `in` in part usage body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 404) (line 20) (column 4) (len 73)) (message "unexpected keyword `in` in part usage body"))
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
  )
  (root (package (name "Camera") (body (import (target (span (span (offset 33) (line 2) (column 17) (len 25))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 55) (line 2) (column 39) (len 3))) (separator (span (offset 55) (line 2) (column 39) (len 2))) (marker (span (offset 57) (line 2) (column 41) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "Camera") (body semicolon)) (part-def (name "FocusingSubsystem") (body semicolon)) (part-def (name "ImagingSubsystem") (body semicolon)) (part-usage))))
)
~~~
