# META
~~~sexpr
(snapshot (type recovery) (description "ExtendedUsage is absent from InterfaceNonOccurrenceUsageElement and MetadataBodyUsage; an invalid extension usage must not be split into a metadata prefix and an ordinary member."))
~~~
# SOURCE
~~~sysml
package RestrictedUsages {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    interface def I {
        #q illegal;
        #q def Retained;
    }
    interface i {
        #q illegal;
        #q def Retained;
    }
    metadata ann : Q {
        #q illegal;
        #q def Retained;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "extended_usage_restricted_scope_recovery.md"
    (diagnostics
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 114) (line 4) (column 9) (len 20)) (message "extended usage is not permitted in interface definition body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 183) (line 8) (column 9) (len 20)) (message "extended usage is not permitted in interface usage body"))
      (diagnostic (code "unexpected_keyword_in_scope") (severity error) (category parseerror) (span (offset 257) (line 12) (column 9) (len 20)) (message "extended usage is not permitted in metadata body"))
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
    (reference r0 (scope relative) (span (offset 53) (line 2) (column 27) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 53) (line 2) (column 27) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 66) (line 2) (column 40) (len 16)))))
    (reference r1 (scope relative) (span (offset 135) (line 5) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 135) (line 5) (column 10) (len 1)))))
    (reference r2 (scope relative) (span (offset 204) (line 9) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 204) (line 9) (column 10) (len 1)))))
    (reference r3 (scope relative) (span (offset 245) (line 11) (column 20) (len 1)) (segments (segment 0 (token "Q") (name "Q") (separator none) (span (offset 245) (line 11) (column 20) (len 1)))))
    (reference r4 (scope relative) (span (offset 278) (line 13) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 278) (line 13) (column 10) (len 1)))))
  )
  (root (package (name "RestrictedUsages") (body brace (metadata-def (name "Q") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (interface-def (name "I") (modifiers) (specializes none) (body brace (malformed (code "unexpected_keyword_in_scope") (found "#q illegal;") (span (offset 114) (line 4) (column 9) (len 20))) (extended-def (prefix-keywords ((ref r1))) (definition-prefix none) (name "Retained") (short-name none) (visibility none) (specializes none) (body semicolon)))) (interface-usage (form declaration) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "i") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part none) (body brace (malformed (code "unexpected_keyword_in_scope") (found "#q illegal;") (span (offset 183) (line 8) (column 9) (len 20))) (extended-def (prefix-keywords ((ref r2))) (definition-prefix none) (name "Retained") (short-name none) (visibility none) (specializes none) (body semicolon)))) (metadata-usage (declaration-name "ann") (short-name none) (type (ref r3)) (about) (body brace (malformed (code "unexpected_keyword_in_scope") (found "#q illegal;") (span (offset 257) (line 12) (column 9) (len 20))) (extended-def (prefix-keywords ((ref r4))) (definition-prefix none) (name "Retained") (short-name none) (visibility none) (specializes none) (body semicolon)))))))
)
~~~
