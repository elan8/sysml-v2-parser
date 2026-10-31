# META
~~~sexpr
(snapshot (type recovery) (description "A document that ends while bodies are still open, as it does while the user is typing. Each open body keeps the members read before end of input and records (body brace (unterminated)) instead of a } span, and missing_closing_brace reports end of input once. The closed part definition, the action definition and the occurrence body inside the open part definition keep their authored }; the formatter writes none for the open package, part definition and nested part usage."))
~~~
# SOURCE
~~~sysml
package Unclosed {
    private import Lib::*;
    part def Closed;
    action def Move {
        in speed : Real;
    }
    part def Robot {
        part m : Motr;
        occurrence o { }
        part arm : Arm {
            attribute reach : Real;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "unterminated_body_keeps_members.md"
    (diagnostics
      (diagnostic (code "missing_closing_brace") (severity none) (category parseerror) (span (offset 249) (line 11) (column 36) (len 1)) (message "missing closing '}'"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Unclosed {
    private import Lib::*;
    part def Closed;
    action def Move {
        in speed : Real;
    }
    part def Robot {
        part m : Motr;
        occurrence o {
        }
        part arm : Arm {
            attribute reach : Real;
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 38) (line 2) (column 20) (len 3)) (segments (segment 0 (token "Lib") (name "Lib") (separator none) (span (offset 38) (line 2) (column 20) (len 3)))))
    (reference r1 (scope relative) (span (offset 108) (line 5) (column 20) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 108) (line 5) (column 20) (len 4)))))
    (reference r2 (scope relative) (span (offset 158) (line 8) (column 18) (len 4)) (segments (segment 0 (token "Motr") (name "Motr") (separator none) (span (offset 158) (line 8) (column 18) (len 4)))))
    (reference r3 (scope relative) (span (offset 208) (line 10) (column 20) (len 3)) (segments (segment 0 (token "Arm") (name "Arm") (separator none) (span (offset 208) (line 10) (column 20) (len 3)))))
    (reference r4 (scope relative) (span (offset 244) (line 11) (column 31) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 244) (line 11) (column 31) (len 4)))))
  )
  (root (package (visibility none) (name "Unclosed") (body brace (unterminated) (import (visibility private) (target (span (span (offset 38) (line 2) (column 20) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 41) (line 2) (column 23) (len 3))) (separator (span (offset 41) (line 2) (column 23) (len 2))) (marker (span (offset 43) (line 2) (column 25) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (part-def (name "Closed") (modifiers) (body semicolon)) (action-def (name "Move") (modifiers) (specializes none) (body brace (in-out (direction in) (kind none) (reference false) (declaration "speed") (short-name none) (subsets none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 97) (line 5) (column 9) (len 16))))) (part-def (name "Robot") (modifiers) (body brace (unterminated) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "m") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "o") (short-name none) (target none) (body brace)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "arm") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (unterminated) (attribute-usage (declaration-name "reach") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))))
)
~~~
