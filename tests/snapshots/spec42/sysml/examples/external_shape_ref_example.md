# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Geometry): ExternalShapeRefExample"))
~~~
# SOURCE
~~~sysml
package ExternalShapeRefExample {
	private import ScalarValues::String;
	private import ShapeItems::*;
	private import ISQ::mass;
	private import SI::mm;

	metadata def ExternalShapeRef {
		doc
		/*
		 * Metadata to reference an externally defined shape.
		 */
	
		attribute purpose : String[1];
		attribute shapeIri : String[1];
	}
	
	part myBatteryUnit {
	    item :>> shape : Shell {
			metadata ExternalShapeRef {
				purpose = "highLoD";
				shapeIri = "file:/detailed-geometry/LEMS-250W_BatteryHousing_Example.step";
			}
		}		

		private item envelopingBoxBatteryUnit : Box :> envelopingShapes {
			:>> length = 140[mm];
			:>> width = 148[mm];
			:>> height = 90[mm];
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "external_shape_ref_example.md"
    (diagnostics
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 390) (line 19) (column 4) (len 140)) (message "unexpected keyword `metadata` in attribute body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package ExternalShapeRefExample {
    private import ScalarValues::String;
    private import ShapeItems::*;
    private import ISQ::mass;
    private import SI::mm;
    metadata def ExternalShapeRef {
        doc
        /*
		 * Metadata to reference an externally defined shape.
		 */
        attribute purpose : String[1];
        attribute shapeIri : String[1];
    }
    part myBatteryUnit {
        item :>> shape : Shell {
            metadata ExternalShapeRef {
				purpose = "highLoD";
				shapeIri = "file:/detailed-geometry/LEMS-250W_BatteryHousing_Example.step";
			}
        }
        private item envelopingBoxBatteryUnit : Box :> envelopingShapes {
            attribute :>> length = 140 [mm];
            attribute :>> width = 148 [mm];
            attribute :>> height = 90 [mm];
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 50) (line 2) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 50) (line 2) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 64) (line 2) (column 31) (len 6)))))
    (reference r1 (scope relative) (span (offset 88) (line 3) (column 17) (len 10)) (segments (segment 0 (token "ShapeItems") (name "ShapeItems") (separator none) (span (offset 88) (line 3) (column 17) (len 10)))))
    (reference r2 (scope relative) (span (offset 119) (line 4) (column 17) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 119) (line 4) (column 17) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 124) (line 4) (column 22) (len 4)))))
    (reference r3 (scope relative) (span (offset 146) (line 5) (column 17) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 146) (line 5) (column 17) (len 2))) (segment 1 (token "mm") (name "mm") (separator colon-colon) (span (offset 150) (line 5) (column 21) (len 2)))))
  )
  (root (package (name "ExternalShapeRefExample") (body brace (import (target (span (span (offset 50) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 88) (line 3) (column 17) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 98) (line 3) (column 27) (len 3))) (separator (span (offset 98) (line 3) (column 27) (len 2))) (marker (span (offset 100) (line 3) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 119) (line 4) (column 17) (len 9))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 146) (line 5) (column 17) (len 6))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (metadata-def) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "myBatteryUnit") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "")) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "envelopingBoxBatteryUnit")))))))
)
~~~
