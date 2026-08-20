# META
~~~sexpr
(snapshot (type semantic) (description "The two definition productions that do not reach BasicDefinitionPrefix refuse what they do not spell, and the refusal is a recovery node with an exact span and a surviving valid sibling rather than a silently dropped keyword. MetadataDefinition (SysML BNF 1652) inlines `isAbstract ?= 'abstract'` with no `variation` alternative; EnumerationDefinition (BNF 518) spells no prefix slot at all. Note the diagnostic message is misattributed: a package-body member whose first keyword is in the extended-library starter list is reported as that unimplemented production whatever the real cause. The span, the retained text and the siblings are correct; only the classification is wrong. Recorded as deferred debt in planning/spec42-upstream-gap-audit.md."))
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
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 39) (line 2) (column 5) (len 49)) (message "the spec-valid extended-library declaration production is retained but not structurally implemented"))
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 128) (line 4) (column 5) (len 39)) (message "the spec-valid extended-library declaration production is retained but not structurally implemented"))
      (diagnostic (code "unsupported_grammar_form") (severity warning) (category unsupportedgrammarform) (span (offset 212) (line 6) (column 5) (len 41)) (message "the spec-valid extended-library declaration production is retained but not structurally implemented"))
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
  (references
  )
  (root (package (name "DefinitionPrefixRefusals") (body brace (extended-library-declaration) (metadata-def (name "AcceptedMetadata") (abstract false) (specializes none) (body semicolon)) (extended-library-declaration) (enum-def (name "AcceptedEnumAfterAbstract") (body semicolon)) (extended-library-declaration) (enum-def (name "AcceptedEnumAfterVariation") (body semicolon)))))
)
~~~
