# META
~~~sexpr
(snapshot (type recovery) (description "Malformed extension headers and nested bodies preserve their source and later typed siblings; extension members stay invalid in restricted enumeration and relationship bodies."))
~~~
# SOURCE
~~~sysml
package ExtendedRecovery {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    part def P {
        #q def Bad : ;
        #q retained;
        #q broken : ;
        #q def Good;
        part later;
    }
    action def A {
        #q def Bad : { doc /* } ; */ }
        #q def Good;
    }
    enum def Choices {
        #q def Bad;
        enum retained;
    }
    dependency from A to P {
        #q def Bad;
        doc /* retained */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "extended_members_recovery.md"
    (diagnostics
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 112) (line 4) (column 12) (len 20)) (message "unexpected keyword `def` in part definition body"))
      (diagnostic (code "unrecognized_declaration_in_scope") (severity error) (category parseerror) (span (offset 156) (line 6) (column 12) (len 19)) (message "unrecognized declaration `broken` in part definition body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 244) (line 11) (column 12) (len 36)) (message "unexpected keyword `def` in action body"))
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 330) (line 15) (column 9) (len 20)) (message "incomplete parser support for metadata syntax in enumeration body"))
      (diagnostic (code "unsupported_annotation_syntax") (severity warning) (category unsupportedgrammarform) (span (offset 408) (line 19) (column 9) (len 20)) (message "incomplete parser support for metadata syntax in relationship body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package ExtendedRecovery {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    part def P {
        #q
        def Bad : ;
        #q retained;
        #q
        broken : ;
        #q def Good;
        part later;
    }
    action def A {
        #q
        def Bad : { doc /* } ; */ }
        #q def Good;
    }
    enum def Choices {
        #q def Bad;
        enum retained;
    }
    dependency from A to P {
        #q def Bad;
        doc
        /* retained */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 53) (line 2) (column 27) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 53) (line 2) (column 27) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 66) (line 2) (column 40) (len 16)))))
    (reference r1 (scope relative) (span (offset 110) (line 4) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 110) (line 4) (column 10) (len 1)))))
    (reference r2 (scope relative) (span (offset 133) (line 5) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 133) (line 5) (column 10) (len 1)))))
    (reference r3 (scope relative) (span (offset 154) (line 6) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 154) (line 6) (column 10) (len 1)))))
    (reference r4 (scope relative) (span (offset 176) (line 7) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 176) (line 7) (column 10) (len 1)))))
    (reference r5 (scope relative) (span (offset 242) (line 11) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 242) (line 11) (column 10) (len 1)))))
    (reference r6 (scope relative) (span (offset 281) (line 12) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 281) (line 12) (column 10) (len 1)))))
    (reference r7 (scope relative) (span (offset 391) (line 18) (column 21) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 391) (line 18) (column 21) (len 1)))))
    (reference r8 (scope relative) (span (offset 396) (line 18) (column 26) (len 1)) (segments (segment 0 (token "P") (name "P") (separator none) (span (offset 396) (line 18) (column 26) (len 1)))))
  )
  (root (package (visibility none) (name "ExtendedRecovery") (body brace (metadata-def (name "Q") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (part-def (name "P") (modifiers) (body brace (metadata-keyword-usage (type (ref r1)) (body none)) (malformed (code "unexpected_keyword_in_scope") (found "def Bad : ;") (span (offset 112) (line 4) (column 12) (len 20))) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r2)) (declaration (name "retained") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)) (metadata-keyword-usage (type (ref r3)) (body none)) (malformed (code "unrecognized_declaration_in_scope") (found "broken : ;") (span (offset 156) (line 6) (column 12) (len 19))) (extended-def (prefix-keywords ((ref r4))) (definition-prefix none) (name "Good") (short-name none) (visibility none) (specializes none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "later") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (action-def (name "A") (modifiers) (specializes none) (body brace (metadata-keyword-usage (type (ref r5)) (body none)) (malformed (code "unexpected_keyword_in_scope") (found "def Bad : { doc /* } ; */ }") (span (offset 244) (line 11) (column 12) (len 36))) (extended-def (prefix-keywords ((ref r6))) (definition-prefix none) (name "Good") (short-name none) (visibility none) (specializes none) (body semicolon)))) (enum-def (name "Choices") (body brace (malformed (code "unsupported_annotation_syntax") (found "#q def Bad;") (span (offset 330) (line 15) (column 9) (len 20))) (enum-value (extensions) (enum-keyword present) (visibility none) (name "retained") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon) (span (offset 350) (line 16) (column 9) (len 14))))) (dependency (clients (ref r7)) (suppliers (ref r8)) (body brace (malformed (code "unsupported_annotation_syntax") (found "#q def Bad;") (span (offset 408) (line 19) (column 9) (len 20))) (doc (name none) (short-name none) (locale none) (body (span (offset 434) (line 20) (column 15) (len 10)) (normalized "retained "))))))))
)
~~~
