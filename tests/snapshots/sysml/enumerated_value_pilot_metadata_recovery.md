# META
~~~sexpr
(snapshot (type recovery) (description "The 2026-04 pin defines EnumeratedValue as optional `enum` followed by Usage (SysML-textual-bnf.kebnf 528-535). Unlike sibling Pilot SysML.xtext 765-771, it has no leading UsageExtensionKeyword* slot. The Pilot-only MetadataTest spelling `#Security enum secret : ClassificationLevel = 2;` therefore recovers as one exact enumeration member while adjacent pin-defined enum-prefixed values remain typed."))
~~~
# SOURCE
~~~sysml
package EnumeratedValuePilotMetadataRecovery {
    metadata def Security;
    enum def ClassificationLevel {
        enum uncl = 0;
        #Security enum secret : ClassificationLevel = 2;
        enum conf = 1;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "enumerated_value_pilot_metadata_recovery.md"
    (diagnostics
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 140) (line 5) (column 9) (len 57)) (message "incomplete parser support for metadata syntax in enumeration body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package EnumeratedValuePilotMetadataRecovery {
    metadata def Security;
    enum def ClassificationLevel {
        uncl = 0;
        #Security enum secret : ClassificationLevel = 2;
        conf = 1;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "EnumeratedValuePilotMetadataRecovery") (body brace (metadata-def (name "Security") (abstract false) (specializes none) (body semicolon)) (enum-def (name "ClassificationLevel") (body brace (enum-value (name "uncl") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 129) (line 4) (column 21) (len 1)) (integer 0))))) (body semicolon) (span (offset 117) (line 4) (column 9) (len 14))) (malformed (code "unsupported_annotation_syntax") (found "#Security enum secret : ClassificationLevel = 2;") (span (offset 140) (line 5) (column 9) (len 57))) (enum-value (name "conf") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 209) (line 6) (column 21) (len 1)) (integer 1))))) (body semicolon) (span (offset 197) (line 6) (column 9) (len 14))))))))
)
~~~
