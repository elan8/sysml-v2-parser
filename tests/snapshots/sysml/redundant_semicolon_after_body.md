# META
~~~sexpr
(snapshot (type malformed) (description "A ';' directly after a member's closing '}' parses as an empty anonymous member and is reported; a bare ';' elsewhere, a ';' after a semicolon-terminated member, and the ';' ending a feature whose value is a braced expression are kept as members and not reported. Action bodies do not model the keyword-less member yet, so a ';' there is retained as recovered content rather than discarded."))
~~~
# SOURCE
~~~sysml
package Redundant {
    part def Engine {
        attribute mass;
    };
    action def Drive {
        action start {
        };
        send 'go' to engine {
        };
    }
    part vehicle {
        part engine : Engine {
        } ;
    }

    part def Bare {
        ;
        attribute a;;
    }
    attribute selected = (1, 2)->select {in x; x > 1};
};
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "redundant_semicolon_after_body.md"
    (diagnostics
      (diagnostic (code "redundant_semicolon_after_body") (severity error) (category parseerror) (span (offset 71) (line 4) (column 6) (len 1)) (message "unexpected ';' after a closing '}': a braced body needs no terminator"))
      (diagnostic (code "recovered_action_body_element") (severity error) (category parseerror) (span (offset 128) (line 7) (column 10) (len 10)) (message "unexpected token in action body"))
      (diagnostic (code "recovery_cascade_suppressed") (severity warning) (category parseerror) (span (offset 128) (line 7) (column 10) (len 10)) (message "suppressed 1 cascading recovered diagnostic after earlier recovery errors"))
      (diagnostic (code "redundant_semicolon_after_body") (severity error) (category parseerror) (span (offset 237) (line 13) (column 11) (len 1)) (message "unexpected ';' after a closing '}': a braced body needs no terminator"))
      (diagnostic (code "redundant_semicolon_after_body") (severity error) (category parseerror) (span (offset 360) (line 21) (column 2) (len 1)) (message "unexpected ';' after a closing '}': a braced body needs no terminator"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Redundant {
    part def Engine {
        attribute mass;
    }
    ;
    action def Drive {
        action start {
        }
        ;
        send 'go' to engine {
        }
        ;
    }
    part vehicle {
        part engine : Engine {}
        ;
    }
    part def Bare {
        ;
        attribute a;
        ;
    }
    attribute selected = (1, 2)->select { in x; x > 1 };
}

;
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 218) (line 12) (column 23) (len 6)) (segments (segment 0 (token "Engine") (name "Engine") (separator none) (span (offset 218) (line 12) (column 23) (len 6)))))
  )
  (root (package (name "Redundant") (body brace (part-def (name "Engine") (modifiers) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (action-def (name "Drive") (modifiers) (specializes none) (body brace (action-usage (keyword action) (name "start") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace)) (malformed (code "recovered_action_body_element") (found ";") (span (offset 128) (line 7) (column 10) (len 10))) (action-usage (keyword send) (name none) (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace)) (malformed (code "recovered_action_body_element") (found ";") (span (offset 169) (line 9) (column 10) (len 6))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Bare") (modifiers) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "a") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))
)
~~~
