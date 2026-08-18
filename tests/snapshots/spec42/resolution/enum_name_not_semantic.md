# META
~~~sexpr
(snapshot (type semantic) (description "Enum diagnostics use resolved semantic kind, not a type name spelling"))
~~~
# SOURCE
~~~sysml
package Demo {
    enum def StateCode {
        enum approved;
    }
    part def StatusNamedType;
    part def Base {
        attribute value : StatusNamedType;
    }
    part def Derived :> Base;
    part host : Derived {
        attribute value = "approved";
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "enum_name_not_semantic.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Demo {
    enum def StateCode {
        approved;
    }
    part def StatusNamedType;
    part def Base {
        attribute value : StatusNamedType;
    }
    part def Derived :> Base;
    part host : Derived {
        attribute value = "approved";
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 145) (line 7) (column 27) (len 15)) (segments (segment 0 (token "StatusNamedType") (name "StatusNamedType") (separator none) (span (offset 145) (line 7) (column 27) (len 15)))))
    (reference r1 (scope relative) (span (offset 214) (line 10) (column 17) (len 7)) (segments (segment 0 (token "Derived") (name "Derived") (separator none) (span (offset 214) (line 10) (column 17) (len 7)))))
  )
  (root (package (name "Demo") (body brace (enum-def (name "StateCode") (body brace (enum-value (name "approved") (short-name none) (value none) (body semicolon) (span (offset 48) (line 3) (column 9) (len 14))))) (part-def (name "StatusNamedType") (body semicolon)) (part-def (name "Base") (body brace (attribute-usage (declaration-name "value") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Derived") (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "host") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body brace (attribute-usage (declaration-name "value") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 250) (line 11) (column 27) (len 10)) (string "approved"))))) (body semicolon)))))))
)
~~~
