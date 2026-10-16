# META
~~~sexpr
(snapshot (type recovery) (description "After fallback retirement, malformed blocks recover across nested members, quoted delimiters and comments without consuming later typed siblings."))
~~~
# SOURCE
~~~sysml
package RecoveryBlocks {
    action def Rejected disjoint from Other {
        doc /* A brace } and a semicolon ; are documentation. */
        action nested;
    }
    action def Retained;
    part def Owner {
        connection broken : {
            doc /* } ; { */
            attribute 'brace } ;';
        }
        part retained;
    }
    view def Overview {
        ref broken : {
            doc /* } ; { */
        }
        view retained;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fallback_removal_balanced_recovery.md"
    (diagnostics
      (diagnostic (code "missing_body_or_semicolon") (severity error) (category parseerror) (span (offset 29) (line 2) (column 5) (len 135)) (message "expected ';' or '{' after action definition header"))
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 219) (line 8) (column 9) (len 103)) (message "unexpected token in part definition body"))
      (diagnostic (code "missing_type_reference") (severity error) (category parseerror) (span (offset 375) (line 15) (column 9) (len 61)) (message "expected reference type after ':'"))
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
  )
  (root (package (name "RecoveryBlocks") (body brace (malformed (code "missing_body_or_semicolon") (found "action def Rejected disjoint from Other {") (span (offset 29) (line 2) (column 5) (len 135))) (action-def (name "Retained") (modifiers) (specializes none) (body semicolon)) (part-def (name "Owner") (modifiers) (body brace (malformed (code "recovered_part_def_body_element") (found "connection broken : {") (span (offset 219) (line 8) (column 9) (len 103))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "retained") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (view-def (name "Overview") (short-name none) (modifiers) (specializes none) (body brace (malformed (code "missing_type_reference") (found "ref broken : {") (span (offset 375) (line 15) (column 9) (len 61))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "retained") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)))))))
)
~~~
