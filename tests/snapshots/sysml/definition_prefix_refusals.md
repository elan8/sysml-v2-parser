# META
~~~sexpr
(snapshot (type semantic) (description "MetadataDefinition refuses variation and EnumerationDefinition refuses abstract/variation prefixes, preserving each malformed member with an error and retaining the valid siblings (SysML BNF 1652 and 518)."))
~~~
# SOURCE
~~~sysml
package DefinitionPrefixRefusals {
    variation metadata def RejectedVariationMetadata;
    metadata def AcceptedMetadata;
    abstract enum def RejectedAbstractEnum;
    enum def AcceptedEnumAfterAbstract;
    variation enum def RejectedVariationEnum;
    enum def AcceptedEnumAfterVariation;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "definition_prefix_refusals.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 39) (line 2) (column 5) (len 49)) (message "unexpected token in package body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 39) (line 2) (column 5) (len 49)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
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
  )
  (root (package (name "DefinitionPrefixRefusals") (body brace (malformed (code "recovered_package_body_element") (found "variation metadata def RejectedVariationMetadata;") (span (offset 39) (line 2) (column 5) (len 49))) (metadata-def (name "AcceptedMetadata") (abstract false) (specializes none) (body semicolon)) (malformed (code "recovered_package_body_element") (found "abstract enum def RejectedAbstractEnum;") (span (offset 128) (line 4) (column 5) (len 39))) (enum-def (name "AcceptedEnumAfterAbstract") (body semicolon)) (malformed (code "recovered_package_body_element") (found "variation enum def RejectedVariationEnum;") (span (offset 212) (line 6) (column 5) (len 41))) (enum-def (name "AcceptedEnumAfterVariation") (body semicolon)))))
)
~~~
