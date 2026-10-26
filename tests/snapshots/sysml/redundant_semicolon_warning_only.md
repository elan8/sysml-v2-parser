# META
~~~sexpr
(snapshot (type semantic) (description "SysML 8.2.2.6.3 permits the semicolon after a braced member as a separate empty anonymous reference usage. Warn at the exact semicolon, retain the usage and following sibling, and preserve strict/editor and format/reparse agreement, including Unicode source positions."))
~~~
# SOURCE
~~~sysml
package WarningOnly {
    part def 'Mötör' {};
    part drive : 'Mötör';
    action def A {
        action start {};
        ;
        action following;
    }
    requirement def R {
        require constraint { true };
        ;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "redundant_semicolon_warning_only.md"
    (diagnostics
      (diagnostic (code "redundant_semicolon_after_body") (severity warning) (category parseerror) (span (offset 47) (line 2) (column 26) (len 1)) (message "possible redundant ';' after a closing '}': parsed as an empty anonymous usage"))
      (diagnostic (code "redundant_semicolon_after_body") (severity warning) (category parseerror) (span (offset 119) (line 5) (column 24) (len 1)) (message "possible redundant ';' after a closing '}': parsed as an empty anonymous usage"))
      (diagnostic (code "redundant_semicolon_after_body") (severity warning) (category parseerror) (span (offset 222) (line 10) (column 36) (len 1)) (message "possible redundant ';' after a closing '}': parsed as an empty anonymous usage"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package WarningOnly {
    part def 'Mötör' {
    }
    ;
    part drive : 'Mötör';
    action def A {
        action start {
        }
        ;
        ;
        action following;
    }
    requirement def R {
        require constraint {
            true
        }
        ;
        ;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 66) (line 3) (column 18) (len 9)) (segments (segment 0 (token "'Mötör'") (name "Mötör") (separator none) (span (offset 66) (line 3) (column 18) (len 9)))))
  )
  (root (package (name "WarningOnly") (body brace (part-def (name "Mötör") (modifiers) (body brace)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "drive") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (action-def (name "A") (modifiers) (specializes none) (body brace (action-usage (keyword action) (name "start") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (action-usage (keyword action) (name "following") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)))) (requirement-def (name "R") (modifiers) (body brace (require-constraint (kind require) (constraint-keyword true) (name none) (target none) (specializations) (value none) (body brace (expression (span (offset 216) (line 10) (column 30) (len 4)) (boolean true)))) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
