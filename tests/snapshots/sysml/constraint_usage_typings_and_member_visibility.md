# META
~~~sexpr
(snapshot (type semantic) (description "A constraint usage keeps every type of its Typings clause and the clause's spelling (#174), and directed and keyword-less redefinition members keep their MemberPrefix visibility in constraint, calculation and action bodies (#178)."))
~~~
# SOURCE
~~~sysml
package Constraints {
    constraint def A;
    constraint def B;
    constraint both : A, B;
    constraint one : A;
    part def Q {
        constraint nested : A, B [1];
        constraint spelled defined by A;
    }
    constraint def C {
        private in x : Real;
        public out y : Real;
        protected inout z;
        private redefines b = 1;
        protected :>> c = 2;
        x > 0
    }
    calc def F {
        private in x : Real;
        private redefines b = 1;
        x
    }
    action def Act {
        private in p : Real;
        in q : Real;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "constraint_usage_typings_and_member_visibility.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Constraints {
    constraint def A;
    constraint def B;
    constraint both : A, B;
    constraint one : A;
    part def Q {
        constraint nested : A, B[1];
        constraint spelled defined by A;
    }
    constraint def C {
        private in x : Real;
        public out y : Real;
        protected inout z;
        private :>> b = 1;
        protected :>> c = 2;
        x > 0;
    }
    calc def F {
        private in x : Real;
        private attribute :>> b = 1;
        x;
    }
    action def Act {
        private in p : Real;
        in q : Real;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 88) (line 4) (column 23) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 88) (line 4) (column 23) (len 1)))))
    (reference r1 (scope relative) (span (offset 91) (line 4) (column 26) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 91) (line 4) (column 26) (len 1)))))
    (reference r2 (scope relative) (span (offset 115) (line 5) (column 22) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 115) (line 5) (column 22) (len 1)))))
    (reference r3 (scope relative) (span (offset 163) (line 7) (column 29) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 163) (line 7) (column 29) (len 1)))))
    (reference r4 (scope relative) (span (offset 166) (line 7) (column 32) (len 1)) (segments (segment 0 (token "B") (name "B") (separator none) (span (offset 166) (line 7) (column 32) (len 1)))))
    (reference r5 (scope relative) (span (offset 211) (line 8) (column 39) (len 1)) (segments (segment 0 (token "A") (name "A") (separator none) (span (offset 211) (line 8) (column 39) (len 1)))))
    (reference r6 (scope relative) (span (offset 354) (line 14) (column 27) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 354) (line 14) (column 27) (len 1)))))
    (reference r7 (scope relative) (span (offset 383) (line 15) (column 23) (len 1)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 383) (line 15) (column 23) (len 1)))))
    (reference r8 (scope relative) (span (offset 398) (line 16) (column 9) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 398) (line 16) (column 9) (len 1)))))
    (reference r9 (scope relative) (span (offset 450) (line 19) (column 24) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 450) (line 19) (column 24) (len 4)))))
    (reference r10 (scope relative) (span (offset 482) (line 20) (column 27) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 482) (line 20) (column 27) (len 1)))))
    (reference r11 (scope relative) (span (offset 497) (line 21) (column 9) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 497) (line 21) (column 9) (len 1)))))
    (reference r12 (scope relative) (span (offset 549) (line 24) (column 24) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 549) (line 24) (column 24) (len 4)))))
    (reference r13 (scope relative) (span (offset 570) (line 25) (column 16) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 570) (line 25) (column 16) (len 4)))))
  )
  (root (package (name "Constraints") (body brace (constraint-def (name "A") (modifiers) (specializes none) (body semicolon)) (constraint-def (name "B") (modifiers) (specializes none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "both") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r0) (ref r1)))) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "one") (short-name none) (type (ref r2)) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (part-def (name "Q") (modifiers) (body brace (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "nested") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r3) (ref r4)))) (multiplicity (lower none) (upper (expression (span (offset 169) (line 7) (column 35) (len 1)) (integer 1)))) (subsets none) (redefines none) (body semicolon)) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "spelled") (short-name none) (type (ref r5)) (multiplicity none) (subsets none) (redefines none) (body semicolon)))) (constraint-def (name "C") (modifiers) (specializes none) (body brace (in-out-declaration) (in-out-declaration) (in-out-declaration) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 358) (line 14) (column 31) (len 1)) (integer 1))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 387) (line 15) (column 27) (len 1)) (integer 2))))) (body semicolon)) (expression (span (offset 398) (line 16) (column 9) (len 5)) (binary (operator ">") (left (expression (span (offset 398) (line 16) (column 9) (len 1)) (ref r8))) (right (expression (span (offset 402) (line 16) (column 13) (len 1)) (integer 0))))))) (calc-def (name "F") (modifiers) (body brace (kerml-feature (prefix (head basic) (direction in) (derived false) (abstract false) (portion none) (variability none) (metadata)) (kind none) (member false) (all false) (name "x") (specializations (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9))))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (relationships) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 486) (line 20) (column 31) (len 1)) (integer 1))))) (body semicolon)) (expression (expression (span (offset 497) (line 21) (column 9) (len 1)) (ref r11))))) (action-def (name "Act") (modifiers) (specializes none) (body brace (in-out (visibility Private) (direction in) (kind none) (reference false) (declaration "p") (short-name none) (subsets none) (type (ref r12)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 534) (line 24) (column 9) (len 20))) (in-out (direction in) (kind none) (reference false) (declaration "q") (short-name none) (subsets none) (type (ref r13)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (redefines none) (value none) (span (offset 563) (line 25) (column 9) (len 12))))))))
)
~~~
