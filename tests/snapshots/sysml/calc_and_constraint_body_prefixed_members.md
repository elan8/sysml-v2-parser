# META
~~~sexpr
(snapshot (type semantic) (description "Members of calculation, constraint, action, part and use case bodies that their gates used to miss: a calc usage with the multiplicity before its typing (#181), `individual calc def` in a calculation body (#175), prefixed control nodes in calculation and constraint bodies (#179), the whole ValuePart on an action-body `ref` member (#173), and `#Tag` kept on the control node, interface or calculation usage it prefixes (#180)."))
~~~
# SOURCE
~~~sysml
package Members {
    calc def C;
    part def X {
        calc before[2] : C;
        calc after : C [2];
        #Tag interface i : I connect a to b;
        #Tag calc tagged;
    }
    part x : X {
        #Tag interface j : I connect a to b;
        #Tag calc tagged;
    }
    use case def U {
        #Tag calc c;
    }
    calc def F {
        individual calc def D;
        private individual calc def E;
        individual fork f;
        in fork g;
        ref :>> x default y;
        ref :>> w := y;
    }
    constraint def K {
        in fork f;
        individual merge m;
    }
    action def A {
        #Tag merge m;
        #Tag decide d;
        #Tag calc c;
        ref :>> x default y;
        ref :>> z = y;
        ref :>> w := y;
        ref :>> v default := y;
        ref r :> s = y;
    }
    action a {
        #Tag join j;
        #Tag fork f;
        ref :>> x default y;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "calc_and_constraint_body_prefixed_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Members {
    calc def C;
    part def X {
        calc before : C[2];
        calc after : C[2];
        #Tag interface i : I connect a to b;
        #Tag calc tagged;
    }
    part x : X {
        #Tag interface j : I connect a to b;
        #Tag calc tagged;
    }
    use case def U {
        #Tag calc c;
    }
    calc def F {
        individual calc def D;
        private individual calc def E;
        individual fork f;
        in fork g;
        ref :>> x default y;
        ref :>> w := y;
    }
    constraint def K {
        in fork f;
        individual merge m;
    }
    action def A {
        #Tag merge m;
        #Tag decide d;
        #Tag calc c;
        ref :>> x default y;
        ref :>> z = y;
        ref :>> w := y;
        ref :>> v default := y;
        ref r :> s = y;
    }
    action a {
        #Tag join j;
        #Tag fork f;
        ref :>> x default y;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 116) (line 6) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 116) (line 6) (column 10) (len 3)))))
    (reference r1 (scope relative) (span (offset 134) (line 6) (column 28) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 134) (line 6) (column 28) (len 1)))))
    (reference r2 (scope relative) (span (offset 144) (line 6) (column 38) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 144) (line 6) (column 38) (len 1)))))
    (reference r3 (scope relative) (span (offset 149) (line 6) (column 43) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 149) (line 6) (column 43) (len 1)))))
    (reference r4 (scope relative) (span (offset 197) (line 9) (column 14) (len 1)) (segments (segment 0 (token "X") (name "X") (separator none) (span (offset 197) (line 9) (column 14) (len 1)))))
    (reference r5 (scope relative) (span (offset 210) (line 10) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 210) (line 10) (column 10) (len 3)))))
    (reference r6 (scope relative) (span (offset 228) (line 10) (column 28) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 228) (line 10) (column 28) (len 1)))))
    (reference r7 (scope relative) (span (offset 238) (line 10) (column 38) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 238) (line 10) (column 38) (len 1)))))
    (reference r8 (scope relative) (span (offset 243) (line 10) (column 43) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 243) (line 10) (column 43) (len 1)))))
    (reference r9 (scope relative) (span (offset 308) (line 14) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 308) (line 14) (column 10) (len 3)))))
    (reference r10 (scope relative) (span (offset 437) (line 19) (column 25) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 437) (line 19) (column 25) (len 1)))))
    (reference r11 (scope relative) (span (offset 456) (line 20) (column 17) (len 1)) (segments (segment 0 (token "g") (name "g") (separator none) (span (offset 456) (line 20) (column 17) (len 1)))))
    (reference r12 (scope relative) (span (offset 485) (line 21) (column 27) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 485) (line 21) (column 27) (len 1)))))
    (reference r13 (scope relative) (span (offset 475) (line 21) (column 17) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 475) (line 21) (column 17) (len 1)))))
    (reference r14 (scope relative) (span (offset 509) (line 22) (column 22) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 509) (line 22) (column 22) (len 1)))))
    (reference r15 (scope relative) (span (offset 504) (line 22) (column 17) (len 1)) (segments (segment 0 (token "w") (name "w") (separator none) (span (offset 504) (line 22) (column 17) (len 1)))))
    (reference r16 (scope relative) (span (offset 557) (line 25) (column 17) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 557) (line 25) (column 17) (len 1)))))
    (reference r17 (scope relative) (span (offset 585) (line 26) (column 26) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 585) (line 26) (column 26) (len 1)))))
    (reference r18 (scope relative) (span (offset 622) (line 29) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 622) (line 29) (column 10) (len 3)))))
    (reference r19 (scope relative) (span (offset 632) (line 29) (column 20) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 632) (line 29) (column 20) (len 1)))))
    (reference r20 (scope relative) (span (offset 644) (line 30) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 644) (line 30) (column 10) (len 3)))))
    (reference r21 (scope relative) (span (offset 655) (line 30) (column 21) (len 1)) (segments (segment 0 (token "d") (name "d") (separator none) (span (offset 655) (line 30) (column 21) (len 1)))))
    (reference r22 (scope relative) (span (offset 667) (line 31) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 667) (line 31) (column 10) (len 3)))))
    (reference r23 (scope relative) (span (offset 705) (line 32) (column 27) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 705) (line 32) (column 27) (len 1)))))
    (reference r24 (scope relative) (span (offset 695) (line 32) (column 17) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 695) (line 32) (column 17) (len 1)))))
    (reference r25 (scope relative) (span (offset 728) (line 33) (column 21) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 728) (line 33) (column 21) (len 1)))))
    (reference r26 (scope relative) (span (offset 724) (line 33) (column 17) (len 1)) (segments (segment 0 (token "z") (name "z") (separator none) (span (offset 724) (line 33) (column 17) (len 1)))))
    (reference r27 (scope relative) (span (offset 752) (line 34) (column 22) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 752) (line 34) (column 22) (len 1)))))
    (reference r28 (scope relative) (span (offset 747) (line 34) (column 17) (len 1)) (segments (segment 0 (token "w") (name "w") (separator none) (span (offset 747) (line 34) (column 17) (len 1)))))
    (reference r29 (scope relative) (span (offset 784) (line 35) (column 30) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 784) (line 35) (column 30) (len 1)))))
    (reference r30 (scope relative) (span (offset 771) (line 35) (column 17) (len 1)) (segments (segment 0 (token "v") (name "v") (separator none) (span (offset 771) (line 35) (column 17) (len 1)))))
    (reference r31 (scope relative) (span (offset 808) (line 36) (column 22) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 808) (line 36) (column 22) (len 1)))))
    (reference r32 (scope relative) (span (offset 804) (line 36) (column 18) (len 1)) (segments (segment 0 (token "s") (name "s") (separator none) (span (offset 804) (line 36) (column 18) (len 1)))))
    (reference r33 (scope relative) (span (offset 841) (line 39) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 841) (line 39) (column 10) (len 3)))))
    (reference r34 (scope relative) (span (offset 850) (line 39) (column 19) (len 1)) (segments (segment 0 (token "j") (name "j") (separator none) (span (offset 850) (line 39) (column 19) (len 1)))))
    (reference r35 (scope relative) (span (offset 862) (line 40) (column 10) (len 3)) (segments (segment 0 (token "Tag") (name "Tag") (separator none) (span (offset 862) (line 40) (column 10) (len 3)))))
    (reference r36 (scope relative) (span (offset 871) (line 40) (column 19) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 871) (line 40) (column 19) (len 1)))))
    (reference r37 (scope relative) (span (offset 900) (line 41) (column 27) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 900) (line 41) (column 27) (len 1)))))
    (reference r38 (scope relative) (span (offset 890) (line 41) (column 17) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 890) (line 41) (column 17) (len 1)))))
  )
  (root (package (name "Members") (body brace (calc-def (name "C") (modifiers) (body semicolon)) (part-def (name "X") (modifiers) (body brace (calc-usage) (calc-usage) (interface-usage (form typed-connect) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r0))) (name "i") (short-name none) (type (ref r1)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part (binary (from (interface-end (multiplicity none) (target (ref r2)))) (to (interface-end (multiplicity none) (target (ref r3)))))) (body semicolon)) (calc-usage))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "x") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (interface-usage (form typed-connect) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r5))) (name "j") (short-name none) (type (ref r6)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part (binary (from (interface-end (multiplicity none) (target (ref r7)))) (to (interface-end (multiplicity none) (target (ref r8)))))) (body semicolon)) (calc-usage))) (use-case-def (name "U") (modifiers) (body brace (calc-usage (name "c") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r9))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (calc-def (name "F") (modifiers) (body brace (calc-def) (calc-def) (fork (prefix (direction none) (derived false) (variance none) (constant false) (individual true) (portion none) (extensions)) (declaration (named (expression (span (offset 437) (line 19) (column 25) (len 1)) (ref r10)))) (body semicolon (span (span (offset 438) (line 19) (column 26) (len 1))))) (fork (prefix (direction in) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 456) (line 20) (column 17) (len 1)) (ref r11)))) (body semicolon (span (span (offset 457) (line 20) (column 18) (len 1))))) (ref (name none) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 485) (line 21) (column 27) (len 1)) (ref r12))))) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (subsets none) (body semicolon)) (ref (name none) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind assign) (default false) (expression (expression (span (offset 509) (line 22) (column 22) (len 1)) (ref r14))))) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (subsets none) (body semicolon)))) (constraint-def (name "K") (modifiers) (specializes none) (body brace (fork (prefix (direction in) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 557) (line 25) (column 17) (len 1)) (ref r16)))) (body semicolon (span (span (offset 558) (line 25) (column 18) (len 1))))) (merge (prefix (direction none) (derived false) (variance none) (constant false) (individual true) (portion none) (extensions)) (declaration (named (expression (span (offset 585) (line 26) (column 26) (len 1)) (ref r17)))) (body semicolon (span (span (offset 586) (line 26) (column 27) (len 1))))))) (action-def (name "A") (modifiers) (specializes none) (body brace (merge (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions (ref r18))) (declaration (named (expression (span (offset 632) (line 29) (column 20) (len 1)) (ref r19)))) (body semicolon (span (span (offset 633) (line 29) (column 21) (len 1))))) (decide (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions (ref r20))) (declaration (named (expression (span (offset 655) (line 30) (column 21) (len 1)) (ref r21)))) (body semicolon (span (span (offset 656) (line 30) (column 22) (len 1))))) (calc-usage (name "c") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r22))) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (ref (name none) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 705) (line 32) (column 27) (len 1)) (ref r23))))) (redefines (relationship (kind redefines) (implied false) (targets (ref r24)))) (subsets none) (body semicolon)) (ref (name none) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 728) (line 33) (column 21) (len 1)) (ref r25))))) (redefines (relationship (kind redefines) (implied false) (targets (ref r26)))) (subsets none) (body semicolon)) (ref (name none) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind assign) (default false) (expression (expression (span (offset 752) (line 34) (column 22) (len 1)) (ref r27))))) (redefines (relationship (kind redefines) (implied false) (targets (ref r28)))) (subsets none) (body semicolon)) (ref (name none) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind assign) (default true) (expression (expression (span (offset 784) (line 35) (column 30) (len 1)) (ref r29))))) (redefines (relationship (kind redefines) (implied false) (targets (ref r30)))) (subsets none) (body semicolon)) (ref (name "r") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 808) (line 36) (column 22) (len 1)) (ref r31))))) (redefines none) (subsets (relationship (kind subsets) (implied false) (targets (ref r32)))) (body semicolon)))) (action-usage (keyword action) (name "a") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (join (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions (ref r33))) (declaration (named (expression (span (offset 850) (line 39) (column 19) (len 1)) (ref r34)))) (body semicolon (span (span (offset 851) (line 39) (column 20) (len 1))))) (fork (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions (ref r35))) (declaration (named (expression (span (offset 871) (line 40) (column 19) (len 1)) (ref r36)))) (body semicolon (span (span (offset 872) (line 40) (column 20) (len 1))))) (ref (name none) (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 900) (line 41) (column 27) (len 1)) (ref r37))))) (redefines (relationship (kind redefines) (implied false) (targets (ref r38)))) (subsets none) (body semicolon)))))))
)
~~~
