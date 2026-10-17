# META
~~~sexpr
(snapshot (type recovery) (description "Failed member productions in part, port, occurrence, view, rendering, attribute and metadata bodies produce syntax errors and preserve the following typed siblings instead of keyword-only unsupported capture."))
~~~
# SOURCE
~~~sysml
package BodyErrors {
    item def Payload;
    part def Owner {
        connection broken : ;
        part retained : Payload;
    }
    port def Boundary {
        ref port broken : ;
        out item retained : Payload;
    }
    occurrence def Context {
        private attribute broken : ;
        attribute retained : Payload;
    }
    view def Overview {
        ref broken : ;
        view retained;
    }
    rendering def Renderer {
        ref broken : ;
        ref retained : Payload;
    }
    attribute def Value {
        derived attribute broken : ;
        attribute retained : Payload;
    }
    metadata def Tag {
        ref broken : ;
        ref retained : Payload;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "body_errors_preserve_siblings.md"
    (diagnostics
      (diagnostic (code "recovered_part_def_body_element") (severity error) (category parseerror) (span (offset 72) (line 4) (column 9) (len 30)) (message "unexpected token in part definition body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 72) (line 4) (column 9) (len 30)) (message "suppressed 2 cascading recovered diagnostics after earlier recovery errors"))
      (diagnostic (code "missing_type_reference") (severity error) (category parseerror) (span (offset 370) (line 16) (column 9) (len 23)) (message "expected reference type after ':'"))
      (diagnostic (code "missing_type_reference") (severity error) (category parseerror) (span (offset 451) (line 20) (column 9) (len 23)) (message "expected reference type after ':'"))
      (diagnostic (code "recovered_attribute_body_element") (severity error) (category parseerror) (span (offset 538) (line 24) (column 9) (len 37)) (message "unexpected token in attribute body"))
      (diagnostic (code "missing_type_reference") (severity error) (category parseerror) (span (offset 642) (line 28) (column 9) (len 23)) (message "expected reference type after ':'"))
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
    (reference r0 (scope relative) (span (offset 118) (line 5) (column 25) (len 7)) (segments (segment 0 (token "Payload") (name "Payload") (separator none) (span (offset 118) (line 5) (column 25) (len 7)))))
    (reference r1 (scope relative) (span (offset 213) (line 9) (column 29) (len 7)) (segments (segment 0 (token "Payload") (name "Payload") (separator none) (span (offset 213) (line 9) (column 29) (len 7)))))
    (reference r2 (scope relative) (span (offset 323) (line 13) (column 30) (len 7)) (segments (segment 0 (token "Payload") (name "Payload") (separator none) (span (offset 323) (line 13) (column 30) (len 7)))))
    (reference r3 (scope relative) (span (offset 489) (line 21) (column 24) (len 7)) (segments (segment 0 (token "Payload") (name "Payload") (separator none) (span (offset 489) (line 21) (column 24) (len 7)))))
    (reference r4 (scope relative) (span (offset 596) (line 25) (column 30) (len 7)) (segments (segment 0 (token "Payload") (name "Payload") (separator none) (span (offset 596) (line 25) (column 30) (len 7)))))
    (reference r5 (scope relative) (span (offset 680) (line 29) (column 24) (len 7)) (segments (segment 0 (token "Payload") (name "Payload") (separator none) (span (offset 680) (line 29) (column 24) (len 7)))))
  )
  (root (package (name "BodyErrors") (body brace (item-def (name "Payload") (modifiers) (individual false) (specializes none) (body semicolon)) (part-def (name "Owner") (modifiers) (body brace (malformed (code "recovered_part_def_body_element") (found "connection broken : ;") (span (offset 72) (line 4) (column 9) (len 30))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "retained") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (port-def (name "Boundary") (modifiers) (specializes none) (body brace (malformed (code "recovered_port_def_body_element") (found "ref port broken : ;") (span (offset 165) (line 8) (column 9) (len 28))) (item-usage (prefix (direction out) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "retained") (short-name none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (occurrence-def (name "Context") (short-name none) (modifiers) (specializes none) (body brace (malformed (code "recovered_occurrence_def_body_element") (found "private attribute broken : ;") (span (offset 265) (line 12) (column 9) (len 37))) (attribute-usage (declaration-name "retained") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (view-def (name "Overview") (short-name none) (modifiers) (specializes none) (body brace (malformed (code "missing_type_reference") (found "ref broken : ;") (span (offset 370) (line 16) (column 9) (len 23))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "retained") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)))) (rendering-def (name "Renderer") (short-name none) (modifiers) (specializes none) (body brace (malformed (code "missing_type_reference") (found "ref broken : ;") (span (offset 451) (line 20) (column 9) (len 23))) (ref (name "retained") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)))) (attribute-def (declaration-name "Value") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (malformed (code "recovered_attribute_body_element") (found "derived attribute broken : ;") (span (offset 538) (line 24) (column 9) (len 37))) (attribute-usage (declaration-name "retained") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (metadata-def (name "Tag") (abstract false) (specializes none) (body brace (malformed (code "missing_type_reference") (found "ref broken : ;") (span (offset 642) (line 28) (column 9) (len 23))) (ref (name "retained") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)))))))
)
~~~
