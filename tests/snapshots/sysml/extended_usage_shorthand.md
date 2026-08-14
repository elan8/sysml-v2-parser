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
        #service;
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
    (reference r0 (scope relative) (span (offset 157) (line 5) (column 31) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 157) (line 5) (column 31) (len 6)))))
  )
  (root (package (name "ExtendedUsageShorthand") (body (extended-def (prefix-keywords ("clouddd")) (definition-prefix none) (def false) (name "ArrowheadCore") (specializes none) (body (part-usage) (metadata-keyword-usage) (port-def (name "Authorisation") (specializes none) (body (attribute-usage (declaration-name "token") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (extended-def (prefix-keywords ("situation")) (definition-prefix none) (def true) (name "Failure") (specializes none) (body semicolon)))))
)
~~~
