# META
~~~sexpr
(snapshot (type recovery) (description "A malformed literal metadata annotation in an item AttributeBody is retained as an explicit unsupported recovery member, and the following valid metadata annotation and attribute sibling remain typed. The metadata starter participates in structural recovery rather than swallowing later members."))
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
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 82) (line 3) (column 9) (len 58)) (message "this attribute body member is spec-valid but not structurally implemented"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package AttributeBodyLiteralMetadataRecovery {
    item def ShapeHolder {
        metadata {
            purpose = "missing-type";
        }
        metadata ExternalShapeRef {
            attribute purpose = "later-metadata";
        }
        attribute retained : String;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 158) (line 6) (column 18) (len 16)) (segments (segment 0 (token "ExternalShapeRef") (name "ExternalShapeRef") (separator none) (span (offset 158) (line 6) (column 18) (len 16)))))
    (reference r1 (scope relative) (span (offset 256) (line 9) (column 30) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 256) (line 9) (column 30) (len 6)))))
  )
  (root (package (name "AttributeBodyLiteralMetadataRecovery") (body brace (item-def (name "ShapeHolder") (modifiers) (individual false) (specializes none) (body brace (unsupported (production unmodelled-body-member) (code "unsupported_grammar_form") (found "metadata {\n            purpose = \"missing-type\";\n        }") (span (offset 82) (line 3) (column 9) (len 58))) (metadata-annotation (prefixes) (introducer metadata) (declared-name none) (type (ref r0)) (about) (body brace (attribute-usage (declaration-name "purpose") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 199) (line 7) (column 23) (len 16)) (string "later-metadata"))))) (body semicolon)))) (attribute-usage (declaration-name "retained") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
