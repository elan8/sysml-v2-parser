# META
~~~sexpr
(snapshot (type recovery) (description "DefaultReferenceUsage = ( isEnd ?= 'end' )? RefPrefix UsageDeclaration ValuePart? UsageBody (reference SysML.xtext 630-633) admits a bare `end` wherever a NonOccurrenceUsageMember is; the parser dispatches it in connection, interface and occurrence bodies, including the nameless `end :>> end1 ::> d1;` (upstream ConnectionTest) as an anonymous EndDecl. A package body and an attribute-definition body do not dispatch it yet, so those two recover exactly and their following valid siblings remain typed."))
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
    connection def ConnectionHolder {
        end :>> end1 ::> d1;
        end valid : T;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "default_reference_usage_end_recovery.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 47) (line 2) (column 5) (len 13)) (message "unexpected token in package body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 47) (line 2) (column 5) (len 13)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
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
    (reference r1 (scope relative) (span (offset 264) (line 10) (column 21) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 264) (line 10) (column 21) (len 1)))))
  )
  (root (package (name "DefaultReferenceUsageEndRecovery") (body brace (malformed (code "recovered_package_body_element") (found "end : T;") (span (offset 47) (line 2) (column 5) (len 13))) (part-def (name "PackageSibling") (modifiers) (body semicolon)) (attribute-def (declaration-name "Holder") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (malformed (code "recovered_attribute_body_element") (found "end :>> target;") (span (offset 120) (line 5) (column 9) (len 24))) (attribute-usage (declaration-name "validInside") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (connection-def (name "ConnectionHolder") (modifiers) (role ordinary) (specializes none) (body brace (malformed (code "recovered_connection_def_body_element") (found "end :>> end1 ::> d1;") (span (offset 223) (line 9) (column 9) (len 29))) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "valid") (span (offset 256) (line 10) (column 13) (len 5)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (references none) (multiplicity none) (redefines none) (crosses none)))))))
)
~~~
