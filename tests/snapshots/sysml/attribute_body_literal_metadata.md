# META
~~~sexpr
(snapshot (type semantic) (description "Literal metadata annotations are typed AnnotatingMembers in an item AttributeBody and in the AttributeBody owned by a nested default-reference usage. The shared annotation retains its literal introducer, typed metadata reference, and attribute bindings."))
~~~
# SOURCE
~~~sysml
package AttributeBodyLiteralMetadata {
    item def ShapeHolder {
        metadata ExternalShapeRef {
            purpose = "item-body";
        }
        externalShape : Shape {
            metadata ExternalShapeRef {
                purpose = "default-reference-body";
            }
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "attribute_body_literal_metadata.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AttributeBodyLiteralMetadata {
    item def ShapeHolder {
        metadata ExternalShapeRef {
            attribute purpose = "item-body";
        }
        externalShape : Shape {
            metadata ExternalShapeRef {
                attribute purpose = "default-reference-body";
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 83) (line 3) (column 18) (len 16)) (segments (segment 0 (token "ExternalShapeRef") (name "ExternalShapeRef") (separator none) (span (offset 83) (line 3) (column 18) (len 16)))))
    (reference r1 (scope relative) (span (offset 171) (line 6) (column 25) (len 5)) (segments (segment 0 (token "Shape") (name "Shape") (separator none) (span (offset 171) (line 6) (column 25) (len 5)))))
    (reference r2 (scope relative) (span (offset 200) (line 7) (column 22) (len 16)) (segments (segment 0 (token "ExternalShapeRef") (name "ExternalShapeRef") (separator none) (span (offset 200) (line 7) (column 22) (len 16)))))
  )
  (root (package (name "AttributeBodyLiteralMetadata") (body brace (item-def (name "ShapeHolder") (modifiers) (individual false) (specializes none) (body brace (metadata-annotation (prefixes) (introducer metadata) (declared-name none) (type (ref r0)) (about) (body brace (attribute-usage (declaration-name "purpose") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 124) (line 4) (column 23) (len 11)) (string "item-body"))))) (body semicolon)))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "externalShape") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (metadata-annotation (prefixes) (introducer metadata) (declared-name none) (type (ref r2)) (about) (body brace (attribute-usage (declaration-name "purpose") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 245) (line 8) (column 27) (len 24)) (string "default-reference-body"))))) (body semicolon)))))))))))
)
~~~
