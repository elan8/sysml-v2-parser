# META
~~~sexpr
(snapshot (type recovery) (description "A malformed attribute-body member recovers as one element without consuming the following valid DefaultReferenceUsage whose header begins with a pinned `:>>` redefinition and typing. A later ordinary attribute sibling also remains typed."))
~~~
# SOURCE
~~~sysml
package DefaultReferenceUsageLeadingRedefinitionRecovery {
    attribute def Outer :> Unit {
        ??? malformed;
        :>> elements : Real[3];
        attribute later : Real;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "default_reference_usage_leading_redefinition_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_attribute_body_element") (severity error) (category parseerror) (span (offset 101) (line 3) (column 9) (len 23)) (message "unexpected token in attribute body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package DefaultReferenceUsageLeadingRedefinitionRecovery {
    attribute def Outer :> Unit {
        ??? malformed;
         : Real[3] :>> elements;
        attribute later : Real;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 86) (line 2) (column 28) (len 4)) (segments (segment 0 (token "Unit") (name "Unit") (separator none) (span (offset 86) (line 2) (column 28) (len 4)))))
    (reference r1 (scope relative) (span (offset 139) (line 4) (column 24) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 139) (line 4) (column 24) (len 4)))))
    (reference r2 (scope relative) (span (offset 128) (line 4) (column 13) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 128) (line 4) (column 13) (len 8)))))
    (reference r3 (scope relative) (span (offset 174) (line 5) (column 27) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 174) (line 5) (column 27) (len 4)))))
  )
  (root (package (name "DefaultReferenceUsageLeadingRedefinitionRecovery") (body brace (attribute-def (declaration-name "Outer") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (malformed (code "recovered_attribute_body_element") (found "??? malformed;") (span (offset 101) (line 3) (column 9) (len 23))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity (lower (expression (span (offset 144) (line 4) (column 29) (len 1)) (integer 3))) (upper (expression (span (offset 144) (line 4) (column 29) (len 1)) (integer 3)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "later") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
