# META
~~~sexpr
(snapshot (type recovery) (description "The pinned SysML grammar defines DefaultReferenceUsage as RefPrefix Usage and has no bare end prefix. Pilot accepts that extension, but this parser records bare end keyword-less forms as recovery while retaining the following valid package and attribute-body siblings."))
~~~
# SOURCE
~~~sysml
package DefaultReferenceUsageEndRecovery {
    end : T;
    part def PackageSibling;
    attribute def Holder {
        end :>> target;
        attribute validInside : T;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "default_reference_usage_end_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 47) (line 2) (column 5) (len 13)) (message "unexpected token in package body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 47) (line 2) (column 5) (len 13)) (message "suppressed 1 cascading recovered diagnostic after earlier recovery errors"))
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
  (references
    (reference r0 (scope relative) (span (offset 168) (line 6) (column 33) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 168) (line 6) (column 33) (len 1)))))
  )
  (root (package (name "DefaultReferenceUsageEndRecovery") (body brace (malformed (code "recovered_package_body_element") (found "end : T;") (span (offset 47) (line 2) (column 5) (len 13))) (part-def (name "PackageSibling") (modifiers) (body semicolon)) (attribute-def (declaration-name "Holder") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (malformed (code "recovered_attribute_body_element") (found "end :>> target;") (span (offset 120) (line 5) (column 9) (len 24))) (attribute-usage (declaration-name "validInside") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
