# META
~~~sexpr
(snapshot (type semantic) (description "The bare #keyword-prefixed extended-usage shorthand with no def keyword (#clouddd ArrowheadCore { ... }) parses into the typed ExtendedDefinition node with has_def_keyword false, alongside the def-suffixed sibling (spec42 Gap 39)."))
~~~
# SOURCE
~~~sysml
package ExtendedUsageShorthand {
    #clouddd ArrowheadCore {
        part registry;
        #service port def Authorisation {
            attribute token : String;
        }
    }
    #situation def Failure;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "extended_usage_shorthand.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ExtendedUsageShorthand {
    #clouddd ArrowheadCore {
        part registry;
        #service
        port def Authorisation {
            attribute token : String;
        }
    }
    #situation def Failure;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 38) (line 2) (column 6) (len 7)) (segments (segment 0 (token "clouddd") (name "clouddd") (separator none) (span (offset 38) (line 2) (column 6) (len 7)))))
    (reference r1 (scope relative) (span (offset 94) (line 4) (column 10) (len 7)) (segments (segment 0 (token "service") (name "service") (separator none) (span (offset 94) (line 4) (column 10) (len 7)))))
    (reference r2 (scope relative) (span (offset 157) (line 5) (column 31) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 157) (line 5) (column 31) (len 6)))))
    (reference r3 (scope relative) (span (offset 186) (line 8) (column 6) (len 9)) (segments (segment 0 (token "situation") (name "situation") (separator none) (span (offset 186) (line 8) (column 6) (len 9)))))
  )
  (root (package (name "ExtendedUsageShorthand") (body brace (extended-def (prefix-keywords ((ref r0))) (definition-prefix none) (def false) (name "ArrowheadCore") (specializes none) (body brace (part-usage (declaration-name "registry") (typing none) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (metadata-keyword-usage (type (ref r1)) (body none)) (port-def (name "Authorisation") (specializes none) (body brace (attribute-usage (declaration-name "token") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (extended-def (prefix-keywords ((ref r3))) (definition-prefix none) (def true) (name "Failure") (specializes none) (body semicolon)))))
)
~~~
