# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Camera): Camera"))
~~~
# SOURCE
~~~sysml
part def Camera {
	private import PictureTaking::*;
	
	perform action takePicture[*] :> PictureTaking::takePicture;
	
	part focusingSubsystem {
		perform takePicture.focus;
	}
	
	part imagingSubsystem {
		perform takePicture.shoot;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "camera.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
part def Camera {
    private import PictureTaking::*;
    perform action takePicture[*] :> PictureTaking::takePicture;
    part focusingSubsystem {
        perform takePicture.focus;
    }
    part imagingSubsystem {
        perform takePicture.shoot;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 34) (line 2) (column 17) (len 13)) (segments (segment 0 (token "PictureTaking") (name "PictureTaking") (separator none) (span (offset 34) (line 2) (column 17) (len 13)))))
    (reference r1 (scope relative) (span (offset 88) (line 4) (column 35) (len 26)) (segments (segment 0 (token "PictureTaking") (name "PictureTaking") (separator none) (span (offset 88) (line 4) (column 35) (len 13))) (segment 1 (token "takePicture") (name "takePicture") (separator colon-colon) (span (offset 103) (line 4) (column 50) (len 11)))))
  )
  (root (part-def (name "Camera") (body brace (import (target (span (span (offset 34) (line 2) (column 17) (len 16))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 47) (line 2) (column 30) (len 3))) (separator (span (offset 47) (line 2) (column 30) (len 2))) (marker (span (offset 49) (line 2) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (perform (declaration "takePicture") (action none) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "focusingSubsystem") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "imagingSubsystem") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (perform))))))
)
~~~
