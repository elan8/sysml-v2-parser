# META
~~~sexpr
(snapshot (type semantic) (description "A semicolon after a braced member is a separate grammar-valid empty anonymous usage and receives a warning. Action and requirement bodies retain it through the shared DefaultReferenceUsage parser; bare semicolons and braced expression terminators remain valid without invented recovery."))
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
      (diagnostic (code "redundant_semicolon_after_body") (severity warning) (category parseerror) (span (offset 71) (line 4) (column 6) (len 1)) (message "possible redundant ';' after a closing '}': parsed as an empty anonymous usage"))
      (diagnostic (code "redundant_semicolon_after_body") (severity warning) (category parseerror) (span (offset 128) (line 7) (column 10) (len 1)) (message "possible redundant ';' after a closing '}': parsed as an empty anonymous usage"))
      (diagnostic (code "redundant_semicolon_after_body") (severity warning) (category parseerror) (span (offset 169) (line 9) (column 10) (len 1)) (message "possible redundant ';' after a closing '}': parsed as an empty anonymous usage"))
      (diagnostic (code "redundant_semicolon_after_body") (severity warning) (category parseerror) (span (offset 237) (line 13) (column 11) (len 1)) (message "possible redundant ';' after a closing '}': parsed as an empty anonymous usage"))
      (diagnostic (code "redundant_semicolon_after_body") (severity warning) (category parseerror) (span (offset 360) (line 21) (column 2) (len 1)) (message "possible redundant ';' after a closing '}': parsed as an empty anonymous usage"))
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
  (root (package (visibility none) (name "Redundant") (body brace (part-def (name "Engine") (modifiers) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (action-def (name "Drive") (modifiers) (specializes none) (body brace (action-usage (keyword action) (name "start") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (action-usage (keyword send) (name none) (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "engine") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Bare") (modifiers) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "a") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))
)
~~~
