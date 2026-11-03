# META
~~~sexpr
(snapshot (type semantic) (description "View definitions and usages admit attribute usages through DefinitionBodyItem (SysML 8.2.2.26); preserve names, types, values and nested body formatting."))
~~~
# SOURCE
~~~sysml
package Views {
    view def Document {
        attribute heading : ScalarValues::String = "Heading";
    }
    view document : Document {
        attribute number : ScalarValues::String = "42";
        attribute :>> heading = "Needs";
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "view_body_attributes.md"
    (diagnostics
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
    (reference r0 (scope relative) (span (offset 68) (line 3) (column 29) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 68) (line 3) (column 29) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 82) (line 3) (column 43) (len 6)))))
    (reference r1 (scope relative) (span (offset 128) (line 5) (column 21) (len 8)) (segments (segment 0 (token "Document") (name "Document") (separator none) (span (offset 128) (line 5) (column 21) (len 8)))))
    (reference r2 (scope relative) (span (offset 166) (line 6) (column 28) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 166) (line 6) (column 28) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 180) (line 6) (column 42) (len 6)))))
    (reference r3 (scope relative) (span (offset 217) (line 7) (column 23) (len 7)) (segments (segment 0 (token "heading") (name "heading") (separator none) (span (offset 217) (line 7) (column 23) (len 7)))))
  )
  (root (package (visibility none) (name "Views") (body brace (view-def (name "Document") (short-name none) (modifiers) (specializes none) (body brace (attribute-usage (declaration-name "heading") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 91) (line 3) (column 52) (len 9)) (string "Heading"))))) (body semicolon)))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "document") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "number") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 189) (line 6) (column 51) (len 4)) (string "42"))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 227) (line 7) (column 33) (len 7)) (string "Needs"))))) (body semicolon)))))))
)
~~~
