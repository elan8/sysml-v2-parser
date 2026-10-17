# META
~~~sexpr
(snapshot (type semantic) (description "InterfaceBody and MetadataBody admit DefinitionMember, including extended definitions; their restricted usage alternatives are retained."))
~~~
# SOURCE
~~~sysml
package ExtendedRestrictedOwners {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    interface def I { #q def Inner; }
    interface i { #q def Inner; }
    metadata ann : Q { #q def Inner; }
    enum def Choices { enum one { #q def Inner; #q instance; } }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "extended_definitions_restricted_bodies.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ExtendedRestrictedOwners {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    interface def I {
        #q def Inner;
    }
    interface i {
        #q def Inner;
    }
    metadata ann : Q {
        #q def Inner;
    }
    enum def Choices {
        enum one {
            #q def Inner;
            #q instance;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 61) (line 2) (column 27) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 61) (line 2) (column 27) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 74) (line 2) (column 40) (len 16)))))
    (reference r1 (scope relative) (span (offset 115) (line 3) (column 24) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 115) (line 3) (column 24) (len 1)))))
    (reference r2 (scope relative) (span (offset 149) (line 4) (column 20) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 149) (line 4) (column 20) (len 1)))))
    (reference r3 (scope relative) (span (offset 183) (line 5) (column 20) (len 1)) (segments (segment 0 (token "Q") (name "Q") (separator none) (span (offset 183) (line 5) (column 20) (len 1)))))
    (reference r4 (scope relative) (span (offset 188) (line 5) (column 25) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 188) (line 5) (column 25) (len 1)))))
    (reference r5 (scope relative) (span (offset 238) (line 6) (column 36) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 238) (line 6) (column 36) (len 1)))))
    (reference r6 (scope relative) (span (offset 252) (line 6) (column 50) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 252) (line 6) (column 50) (len 1)))))
  )
  (root (package (name "ExtendedRestrictedOwners") (body brace (metadata-def (name "Q") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (interface-def (name "I") (modifiers) (specializes none) (body brace (extended-def (prefix-keywords ((ref r1))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)))) (interface-usage (form declaration) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "i") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part none) (body brace (extended-def (prefix-keywords ((ref r2))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)))) (metadata-usage (declaration-name "ann") (short-name none) (type (ref r3)) (about) (body brace (extended-def (prefix-keywords ((ref r4))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)))) (enum-def (name "Choices") (body brace (enum-value (extensions) (enum-keyword present) (visibility none) (name "one") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (extended-def (prefix-keywords ((ref r5))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r6)) (declaration (name "instance") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon))) (span (offset 226) (line 6) (column 24) (len 39))))))))
)
~~~
