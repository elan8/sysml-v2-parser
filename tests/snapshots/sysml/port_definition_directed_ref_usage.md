# META
~~~sexpr
(snapshot (type semantic) (description "Port definition and usage bodies retain visibility-prefixed directed reference usages with multiple typings, nested part/port redefinitions, and following aliases."))
~~~
# SOURCE
~~~sysml
package PortReferences {
    part def A;
    part def B {
        part b;
        part c;
        port x;
    }

    private port def C {
        private in ref y : A, B {
            part B_b redefines B::b;
            part B_c redefines B::c;
            port B_x redefines B::x;
        }
        alias z1 for y;
        alias z2 for y;
    }

    port c : C {
        protected out ref z : A, B {
            port B_x redefines B::x;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "port_definition_directed_ref_usage.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package PortReferences {
    part def A;
    part def B {
        part b;
        part c;
        port x;
    }
    private port def C {
        private in ref y : A, B {
            part B_b redefines B::b;
            part B_c redefines B::c;
            port B_x redefines B::x;
        }
        alias z1 for y;
        alias z2 for y;
    }
    port c : C {
        protected out ref z : A, B {
            port B_x redefines B::x;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 165) (line 10) (column 28) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 165) (line 10) (column 28) (len 1)))))
    (reference r1 (scope relative) (span (offset 168) (line 10) (column 31) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 168) (line 10) (column 31) (len 1)))))
    (reference r2 (scope relative) (span (offset 203) (line 11) (column 32) (len 4)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 203) (line 11) (column 32) (len 1))) (segment 1 (token "b") (name "b") (separator colon-colon) (span (offset 206) (line 11) (column 35) (len 1)))))
    (reference r3 (scope relative) (span (offset 240) (line 12) (column 32) (len 4)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 240) (line 12) (column 32) (len 1))) (segment 1 (token "c") (name "c") (separator colon-colon) (span (offset 243) (line 12) (column 35) (len 1)))))
    (reference r4 (scope relative) (span (offset 277) (line 13) (column 32) (len 4)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 277) (line 13) (column 32) (len 1))) (segment 1 (token "x") (name "x") (separator colon-colon) (span (offset 280) (line 13) (column 35) (len 1)))))
    (reference r5 (scope relative) (span (offset 314) (line 15) (column 22) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 314) (line 15) (column 22) (len 1)))))
    (reference r6 (scope relative) (span (offset 338) (line 16) (column 22) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 338) (line 16) (column 22) (len 1)))))
    (reference r7 (scope relative) (span (offset 361) (line 19) (column 14) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 361) (line 19) (column 14) (len 1)))))
    (reference r8 (scope relative) (span (offset 395) (line 20) (column 31) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 395) (line 20) (column 31) (len 1)))))
    (reference r9 (scope relative) (span (offset 398) (line 20) (column 34) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 398) (line 20) (column 34) (len 1)))))
    (reference r10 (scope relative) (span (offset 433) (line 21) (column 32) (len 4)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 433) (line 21) (column 32) (len 1))) (segment 1 (token "x") (name "x") (separator colon-colon) (span (offset 436) (line 21) (column 35) (len 1)))))
  )
  (root (package (name "PortReferences") (body brace (part-def (name "A") (modifiers) (body semicolon)) (part-def (name "B") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "b") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "x") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (port-def (name "C") (modifiers) (specializes none) (body brace (ref (name "y") (short-name none) (prefix (direction in) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0) (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "B_b") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "B_c") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "B_x") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (alias (name "z1") (target (ref r5)) (body semicolon)) (alias (name "z2") (target (ref r6)) (body semicolon)))) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (ref (name "z") (short-name none) (prefix (direction out) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8) (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "B_x") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))))
)
~~~
