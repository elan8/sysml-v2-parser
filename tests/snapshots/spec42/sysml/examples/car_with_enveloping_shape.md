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
            attribute :>> length = 4800 [mm];
            attribute :>> width = 1840 [mm];
            attribute :>> height = 1350 [mm];
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
    (reference r2 (scope relative) (span (offset 213) (line 11) (column 22) (len 3)) (segments (segment 0 (token "Box") (name "Box") (separator none) (span (offset 213) (line 11) (column 22) (len 3)))))
    (reference r3 (scope relative) (span (offset 224) (line 11) (column 33) (len 14)) (segments (segment 0 (token "boundingShapes") (name "boundingShapes") (separator none) (span (offset 224) (line 11) (column 33) (len 14)))))
    (reference r4 (scope relative) (span (offset 248) (line 12) (column 8) (len 6)) (segments (segment 0 (token "length") (name "length") (separator none) (span (offset 248) (line 12) (column 8) (len 6)))))
    (reference r5 (scope relative) (span (offset 275) (line 13) (column 8) (len 5)) (segments (segment 0 (token "width") (name "width") (separator none) (span (offset 275) (line 13) (column 8) (len 5)))))
    (reference r6 (scope relative) (span (offset 302) (line 14) (column 8) (len 6)) (segments (segment 0 (token "height") (name "height") (separator none) (span (offset 302) (line 14) (column 8) (len 6)))))
  )
  (root (package (name "CarWithEnvelopingShape") (body brace (import (target (span (span (offset 49) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 82) (line 3) (column 17) (len 6))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (part-def (name "Car") (modifiers) (body brace (doc (name none) (locale none) (body (span (offset 117) (line 7) (column 5) (len 70)) (normalized "Example car with simple enveloping shape that is a solid box\n"))) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "boundingBox") (short-name none) (type (ref r2)) (multiplicity (lower (expression (span (offset 218) (line 11) (column 27) (len 1)) (integer 1))) (upper (expression (span (offset 218) (line 11) (column 27) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r3)))) (redefines none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 257) (line 12) (column 17) (len 9)) (literal-with-unit (value (expression (span (offset 257) (line 12) (column 17) (len 4)) (integer 4800))) (unit (expression (span (offset 263) (line 12) (column 23) (len 2)) (bracket (expression (span (offset 263) (line 12) (column 23) (len 2)) (unit "mm")))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 284) (line 13) (column 17) (len 9)) (literal-with-unit (value (expression (span (offset 284) (line 13) (column 17) (len 4)) (integer 1840))) (unit (expression (span (offset 290) (line 13) (column 23) (len 2)) (bracket (expression (span (offset 290) (line 13) (column 23) (len 2)) (unit "mm")))))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 311) (line 14) (column 17) (len 9)) (literal-with-unit (value (expression (span (offset 311) (line 14) (column 17) (len 4)) (integer 1350))) (unit (expression (span (offset 317) (line 14) (column 23) (len 2)) (bracket (expression (span (offset 317) (line 14) (column 23) (len 2)) (unit "mm")))))))))) (body semicolon)))))))))
)
~~~
