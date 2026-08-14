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
  )
  (root (package (name "Demo") (body (enum-def) (part-def (name "StatusNamedType") (body semicolon)) (part-def (name "Base") (body (attribute-usage (declaration-name "value") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Derived") (body semicolon)) (part-usage))))
)
~~~
