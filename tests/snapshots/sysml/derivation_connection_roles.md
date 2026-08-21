# META
~~~sexpr
(snapshot (type semantic) (description "A `#derivation connection` carries a typed role, and its ends carry the fixed `#original` and `#derive` roles rather than declaration names. Each marker keeps its own span, and a derivation end has no typing -- its target is a reference subsetting. The connection definition projects (role derivation) where an ordinary one projects (role ordinary)."))
~~~
# SOURCE
~~~sysml
package DerivationConnectionRoles {
    requirement def OriginalReq;
    requirement def DerivedReq;
    #derivation connection {
        end #original ::> OriginalReq;
        end #derive ::> DerivedReq;
    }
    connection def Ordinary {
        end left ::> OriginalReq;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "derivation_connection_roles.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package DerivationConnectionRoles {
    requirement def OriginalReq;
    requirement def DerivedReq;
    #derivation connection def {
        end #original ::> OriginalReq;
        end #derive ::> DerivedReq;
    }
    connection def Ordinary {
        end left ::> OriginalReq;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 156) (line 5) (column 27) (len 11)) (segments (segment 0 (token "OriginalReq") (name "OriginalReq") (separator none) (span (offset 156) (line 5) (column 27) (len 11)))))
    (reference r1 (scope relative) (span (offset 193) (line 6) (column 25) (len 10)) (segments (segment 0 (token "DerivedReq") (name "DerivedReq") (separator none) (span (offset 193) (line 6) (column 25) (len 10)))))
    (reference r2 (scope relative) (span (offset 262) (line 9) (column 22) (len 11)) (segments (segment 0 (token "OriginalReq") (name "OriginalReq") (separator none) (span (offset 262) (line 9) (column 22) (len 11)))))
  )
  (root (package (name "DerivationConnectionRoles") (body brace (requirement-def (name "OriginalReq") (modifiers) (body semicolon)) (requirement-def (name "DerivedReq") (modifiers) (body semicolon)) (connection-def (name none) (modifiers) (role (derivation (span (offset 105) (line 4) (column 5) (len 11)))) (specializes none) (body brace (end (introducer bare) (short-name none) (identity (derivation-role (kind original) (span (offset 142) (line 5) (column 13) (len 9)))) (typing none) (references (relationship (kind references) (implied false) (targets (ref r0)))) (multiplicity none) (redefines none) (crosses none) (nested-usage none)) (end (introducer bare) (short-name none) (identity (derivation-role (kind derive) (span (offset 181) (line 6) (column 13) (len 7)))) (typing none) (references (relationship (kind references) (implied false) (targets (ref r1)))) (multiplicity none) (redefines none) (crosses none) (nested-usage none)))) (connection-def (name "Ordinary") (modifiers) (role ordinary) (specializes none) (body brace (end (introducer bare) (short-name none) (identity (declaration (name "left") (span (offset 253) (line 9) (column 13) (len 4)))) (typing none) (references (relationship (kind references) (implied false) (targets (ref r2)))) (multiplicity none) (redefines none) (crosses none) (nested-usage none)))))))
)
~~~
