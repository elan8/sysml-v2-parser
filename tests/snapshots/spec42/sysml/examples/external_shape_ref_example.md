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
             :>> length = 140[mm];
             :>> width = 148[mm];
             :>> height = 90[mm];
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
    (reference r4 (scope relative) (span (offset 285) (line 13) (column 23) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 285) (line 13) (column 23) (len 6)))))
    (reference r5 (scope relative) (span (offset 319) (line 14) (column 24) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 319) (line 14) (column 24) (len 6)))))
    (reference r6 (scope relative) (span (offset 379) (line 18) (column 23) (len 5)) (segments (segment 0 (token "Shell") (name "Shell") (separator none) (span (offset 379) (line 18) (column 23) (len 5)))))
    (reference r7 (scope relative) (span (offset 371) (line 18) (column 15) (len 5)) (segments (segment 0 (token "shape") (name "shape") (separator none) (span (offset 371) (line 18) (column 15) (len 5)))))
    (reference r8 (scope relative) (span (offset 399) (line 19) (column 13) (len 16)) (segments (segment 0 (token "ExternalShapeRef") (name "ExternalShapeRef") (separator none) (span (offset 399) (line 19) (column 13) (len 16)))))
    (reference r9 (scope relative) (span (offset 422) (line 20) (column 5) (len 7)) (segments (segment 0 (token "purpose") (name "purpose") (separator none) (span (offset 422) (line 20) (column 5) (len 7)))))
    (reference r10 (scope relative) (span (offset 447) (line 21) (column 5) (len 8)) (segments (segment 0 (token "shapeIri") (name "shapeIri") (separator none) (span (offset 447) (line 21) (column 5) (len 8)))))
    (reference r11 (scope relative) (span (offset 577) (line 25) (column 43) (len 3)) (segments (segment 0 (token "Box") (name "Box") (separator none) (span (offset 577) (line 25) (column 43) (len 3)))))
    (reference r12 (scope relative) (span (offset 584) (line 25) (column 50) (len 16)) (segments (segment 0 (token "envelopingShapes") (name "envelopingShapes") (separator none) (span (offset 584) (line 25) (column 50) (len 16)))))
    (reference r13 (scope relative) (span (offset 610) (line 26) (column 8) (len 6)) (segments (segment 0 (token "length") (name "length") (separator none) (span (offset 610) (line 26) (column 8) (len 6)))))
    (reference r14 (scope relative) (span (offset 623) (line 26) (column 21) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 623) (line 26) (column 21) (len 2)))))
    (reference r15 (scope relative) (span (offset 635) (line 27) (column 8) (len 5)) (segments (segment 0 (token "width") (name "width") (separator none) (span (offset 635) (line 27) (column 8) (len 5)))))
    (reference r16 (scope relative) (span (offset 647) (line 27) (column 20) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 647) (line 27) (column 20) (len 2)))))
    (reference r17 (scope relative) (span (offset 659) (line 28) (column 8) (len 6)) (segments (segment 0 (token "height") (name "height") (separator none) (span (offset 659) (line 28) (column 8) (len 6)))))
    (reference r18 (scope relative) (span (offset 671) (line 28) (column 20) (len 2)) (segments (segment 0 (token "mm") (name "mm") (separator none) (span (offset 671) (line 28) (column 20) (len 2)))))
  )
  (root (package (name "ExternalShapeRefExample") (body brace (import (target (span (span (offset 50) (line 2) (column 17) (len 20))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 88) (line 3) (column 17) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 98) (line 3) (column 27) (len 3))) (separator (span (offset 98) (line 3) (column 27) (len 2))) (marker (span (offset 100) (line 3) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 119) (line 4) (column 17) (len 9))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 146) (line 5) (column 17) (len 6))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (metadata-def (name "ExternalShapeRef") (abstract false) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 198) (line 9) (column 5) (len 60)) (normalized "Metadata to reference an externally defined shape.\n"))) (attribute-usage (declaration-name "purpose") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "shapeIri") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "myBatteryUnit") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "") (short-name none) (type (ref r6)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (value none) (body brace (metadata-annotation (prefixes) (introducer metadata) (declared-name none) (type (ref r8)) (about) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r9)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 432) (line 20) (column 15) (len 9)) (string "highLoD"))))) (body semicolon)) (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r10)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 458) (line 21) (column 16) (len 63)) (string "file:/detailed-geometry/LEMS-250W_BatteryHousing_Example.step"))))) (body semicolon)))))) (item-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "envelopingBoxBatteryUnit") (short-name none) (type (ref r11)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r12)))) (redefines none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 619) (line 26) (column 17) (len 7)) (bracket (base (expression (span (offset 619) (line 26) (column 17) (len 3)) (integer 140))) (operands (sequence-list (element first (expression (span (offset 623) (line 26) (column 21) (len 2)) (ref r14)))))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 643) (line 27) (column 16) (len 7)) (bracket (base (expression (span (offset 643) (line 27) (column 16) (len 3)) (integer 148))) (operands (sequence-list (element first (expression (span (offset 647) (line 27) (column 20) (len 2)) (ref r16)))))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 668) (line 28) (column 17) (len 6)) (bracket (base (expression (span (offset 668) (line 28) (column 17) (len 2)) (integer 90))) (operands (sequence-list (element first (expression (span (offset 671) (line 28) (column 20) (len 2)) (ref r18)))))))))) (body semicolon)))))))))
)
~~~
