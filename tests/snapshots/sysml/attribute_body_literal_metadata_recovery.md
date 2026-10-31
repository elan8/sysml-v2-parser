# META
~~~sexpr
(snapshot (type recovery) (description "A malformed literal metadata annotation produces error recovery in an item AttributeBody, preserving the following typed metadata annotation and attribute sibling."))
~~~
# SOURCE
~~~sysml
package AttributeBodyLiteralMetadataRecovery {
    item def ShapeHolder {
        metadata {
            purpose = "missing-type";
        }
        metadata ExternalShapeRef {
            purpose = "later-metadata";
        }
        attribute retained : String;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "attribute_body_literal_metadata_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_attribute_body_element") (severity error) (category parseerror) (span (offset 82) (line 3) (column 9) (len 67)) (message "unexpected token in attribute body"))
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 158) (line 6) (column 18) (len 16)) (segments (segment 0 (token "ExternalShapeRef") (name "ExternalShapeRef") (separator none) (span (offset 158) (line 6) (column 18) (len 16)))))
    (reference r1 (scope relative) (span (offset 189) (line 7) (column 13) (len 7)) (segments (segment 0 (token "purpose") (name "purpose") (separator none) (span (offset 189) (line 7) (column 13) (len 7)))))
    (reference r2 (scope relative) (span (offset 256) (line 9) (column 30) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 256) (line 9) (column 30) (len 6)))))
  )
  (root (package (visibility none) (name "AttributeBodyLiteralMetadataRecovery") (body brace (item-def (name "ShapeHolder") (modifiers) (individual false) (specializes none) (body brace (malformed (code "recovered_attribute_body_element") (found "metadata {") (span (offset 82) (line 3) (column 9) (len 67))) (metadata-annotation (prefixes) (introducer metadata) (declared-name none) (type (ref r0)) (about) (body brace (metadata-body-usage (reference false) (redefinition-operator implicit) (target (ref r1)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 199) (line 7) (column 23) (len 16)) (string "later-metadata"))))) (body semicolon)))) (attribute-usage (declaration-name "retained") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
