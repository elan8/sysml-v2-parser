# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Geometry): CarWithEnvelopingShape"))
~~~
# SOURCE
~~~sysml
package CarWithEnvelopingShape {
	private import ShapeItems::Box;
	private import SI::mm;

	part def Car {
		doc
		/*
		 * Example car with simple enveloping shape that is a solid box
		 */
	
		item boundingBox : Box [1] :> boundingShapes {
			:>> length = 4800 [mm];
			:>> width  = 1840 [mm];
			:>> height = 1350 [mm];
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "car_with_enveloping_shape.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package CarWithEnvelopingShape {
    private import ShapeItems::Box;
    private import SI::mm;
    part def Car {
        doc
        /*
		 * Example car with simple enveloping shape that is a solid box
		 */
        item boundingBox : Box[1] :> boundingShapes {
            attribute  :>> length = 4800 [mm];
            attribute  :>> width = 1840 [mm];
            attribute  :>> height = 1350 [mm];
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 49) (line 2) (column 17) (len 15)) (segments (segment 0 (token "ShapeItems") (name "ShapeItems") (separator none) (span (offset 49) (line 2) (column 17) (len 10))) (segment 1 (token "Box") (name "Box") (separator colon-colon) (span (offset 61) (line 2) (column 29) (len 3)))))
    (reference r1 (scope relative) (span (offset 82) (line 3) (column 17) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 82) (line 3) (column 17) (len 2))) (segment 1 (token "mm") (name "mm") (separator colon-colon) (span (offset 86) (line 3) (column 21) (len 2)))))
  )
  (root (package (name "CarWithEnvelopingShape") (body (import (target (span (span (offset 49) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 82) (line 3) (column 17) (len 6))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (part-def (name "Car") (body (doc) (item-usage))))))
)
~~~
