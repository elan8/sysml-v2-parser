# META
~~~sexpr
(snapshot (type semantic) (description "Body members the release corpus uses and these bodies rejected: an actor with a value, keyword-less and directed usages in connection, occurrence and requirement bodies, `require`/`assume` constraints with extension keywords and specialization clauses, imports in case bodies, port usages as flow-definition ends, nested interface usages and attributes in interface bodies, a port multiplicity after its redefinition, and a real literal without an integer part (#165, #168)."))
~~~
# SOURCE
~~~sysml
package Corpus {
    use case def Use {
        private import Cases::*;
        actor user = System::user;
        actor driver : Person [1] = fleet::driver;
        actor plain;
    }
    connection def Selection {
        end cart : Cart;
        end product : Product;
        :>> info = info1;
    }
    individual part run : Run {
        snapshot start {
            out :>> fuelEconomy = 35 [mph];
        }
    }
    requirement def MassRequirement {
        in massActual :> ISQ::mass;
        require constraint c1 :>> c;
        require constraint c2 : C :> base;
        assume #goal constraint payloadLimit;
        require #goal massLimit;
        require existing;
    }
    flow def FuelFlow {
        ref :>> payload : Fuel;
        end port supplierPort : FuelOutPort;
        end port consumerPort : FuelInPort;
    }
    interface def Hub {
        end lugNuts : LugNuts;
        end shanks : Shanks;
        interface fastener : Fastener [5] connect lugNuts.nut to shanks.shank;
    }
    part wheel {
        port lugNuts :>> lugNuts {
            port nut :>> nut [5];
        }
        interface hub : Hub connect a to b {
            interface inner :> fastener connect p to q;
            attribute :>> maxTorque = 90 * 1.356 [N * m];
        }
        attribute efficiency redefines efficiency = .6;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "body_members_from_release_corpus.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Corpus {
    use case def Use {
        private import Cases::*;
        actor user = System::user;
        actor driver : Person[1] = fleet::driver;
        actor plain;
    }
    connection def Selection {
        end cart : Cart;
        end product : Product;
        :>> info = info1;
    }
    individual part run : Run {
        snapshot start {
            out :>> fuelEconomy = 35[mph];
        }
    }
    requirement def MassRequirement {
        in massActual :> ISQ::mass;
        require constraint c1 :>> c;
        require constraint c2 : C :> base;
        assume #goal constraint payloadLimit;
        require #goal massLimit;
        require existing;
    }
    flow def FuelFlow {
        ref : Fuel :>> payload;
        end port supplierPort : FuelOutPort;
        end port consumerPort : FuelInPort;
    }
    interface def Hub {
        end lugNuts : LugNuts;
        end shanks : Shanks;
        interface fastener : Fastener[5] connect lugNuts.nut to shanks.shank;
    }
    part wheel {
        port lugNuts :>> lugNuts {
            port nut[5] :>> nut;
        }
        interface hub : Hub connect a to b {
            interface inner :> fastener connect p to q;
            attribute :>> maxTorque = 90 * 1.356[N * m];
        }
        attribute efficiency redefines efficiency = .6;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 63) (line 3) (column 24) (len 5)) (segments (segment 0 (token "Cases") (name "Cases") (separator none) (span (offset 63) (line 3) (column 24) (len 5)))))
    (reference r1 (scope relative) (span (offset 94) (line 4) (column 22) (len 12)) (segments (segment 0 (token "System") (name "System") (separator none) (span (offset 94) (line 4) (column 22) (len 6))) (segment 1 (token "user") (name "user") (separator colon-colon) (span (offset 102) (line 4) (column 30) (len 4)))))
    (reference r2 (scope relative) (span (offset 131) (line 5) (column 24) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 131) (line 5) (column 24) (len 6)))))
    (reference r3 (scope relative) (span (offset 144) (line 5) (column 37) (len 13)) (segments (segment 0 (token "fleet") (name "fleet") (separator none) (span (offset 144) (line 5) (column 37) (len 5))) (segment 1 (token "driver") (name "driver") (separator colon-colon) (span (offset 151) (line 5) (column 44) (len 6)))))
    (reference r4 (scope relative) (span (offset 236) (line 9) (column 20) (len 4)) (segments (segment 0 (token "Cart") (name "Cart") (separator none) (span (offset 236) (line 9) (column 20) (len 4)))))
    (reference r5 (scope relative) (span (offset 264) (line 10) (column 23) (len 7)) (segments (segment 0 (token "Product") (name "Product") (separator none) (span (offset 264) (line 10) (column 23) (len 7)))))
    (reference r6 (scope relative) (span (offset 285) (line 11) (column 13) (len 4)) (segments (segment 0 (token "info") (name "info") (separator none) (span (offset 285) (line 11) (column 13) (len 4)))))
    (reference r7 (scope relative) (span (offset 292) (line 11) (column 20) (len 5)) (segments (segment 0 (token "info1") (name "info1") (separator none) (span (offset 292) (line 11) (column 20) (len 5)))))
    (reference r8 (scope relative) (span (offset 331) (line 13) (column 27) (len 3)) (segments (segment 0 (token "Run") (name "Run") (separator none) (span (offset 331) (line 13) (column 27) (len 3)))))
    (reference r9 (scope relative) (span (offset 382) (line 15) (column 21) (len 11)) (segments (segment 0 (token "fuelEconomy") (name "fuelEconomy") (separator none) (span (offset 382) (line 15) (column 21) (len 11)))))
    (reference r10 (scope relative) (span (offset 400) (line 15) (column 39) (len 3)) (segments (segment 0 (token "mph") (name "mph") (separator none) (span (offset 400) (line 15) (column 39) (len 3)))))
    (reference r11 (scope relative) (span (offset 485) (line 19) (column 26) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 485) (line 19) (column 26) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 490) (line 19) (column 31) (len 4)))))
    (reference r12 (scope relative) (span (offset 530) (line 20) (column 35) (len 1)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 530) (line 20) (column 35) (len 1)))))
    (reference r13 (scope relative) (span (offset 565) (line 21) (column 33) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 565) (line 21) (column 33) (len 1)))))
    (reference r14 (scope relative) (span (offset 570) (line 21) (column 38) (len 4)) (segments (segment 0 (token "base") (name "base") (separator none) (span (offset 570) (line 21) (column 38) (len 4)))))
    (reference r15 (scope relative) (span (offset 592) (line 22) (column 17) (len 4)) (segments (segment 0 (token "goal") (name "goal") (separator none) (span (offset 592) (line 22) (column 17) (len 4)))))
    (reference r16 (scope relative) (span (offset 639) (line 23) (column 18) (len 4)) (segments (segment 0 (token "goal") (name "goal") (separator none) (span (offset 639) (line 23) (column 18) (len 4)))))
    (reference r17 (scope relative) (span (offset 671) (line 24) (column 17) (len 8)) (segments (segment 0 (token "existing") (name "existing") (separator none) (span (offset 671) (line 24) (column 17) (len 8)))))
    (reference r18 (scope relative) (span (offset 884) (line 32) (column 23) (len 7)) (segments (segment 0 (token "LugNuts") (name "LugNuts") (separator none) (span (offset 884) (line 32) (column 23) (len 7)))))
    (reference r19 (scope relative) (span (offset 914) (line 33) (column 22) (len 6)) (segments (segment 0 (token "Shanks") (name "Shanks") (separator none) (span (offset 914) (line 33) (column 22) (len 6)))))
    (reference r20 (scope relative) (span (offset 951) (line 34) (column 30) (len 8)) (segments (segment 0 (token "Fastener") (name "Fastener") (separator none) (span (offset 951) (line 34) (column 30) (len 8)))))
    (reference r21 (scope relative) (span (offset 972) (line 34) (column 51) (len 11)) (segments (segment 0 (token "lugNuts") (name "lugNuts") (separator none) (span (offset 972) (line 34) (column 51) (len 7))) (segment 1 (token "nut") (name "nut") (separator dot) (span (offset 980) (line 34) (column 59) (len 3)))))
    (reference r22 (scope relative) (span (offset 987) (line 34) (column 66) (len 12)) (segments (segment 0 (token "shanks") (name "shanks") (separator none) (span (offset 987) (line 34) (column 66) (len 6))) (segment 1 (token "shank") (name "shank") (separator dot) (span (offset 994) (line 34) (column 73) (len 5)))))
    (reference r23 (scope relative) (span (offset 1049) (line 37) (column 26) (len 7)) (segments (segment 0 (token "lugNuts") (name "lugNuts") (separator none) (span (offset 1049) (line 37) (column 26) (len 7)))))
    (reference r24 (scope relative) (span (offset 1084) (line 38) (column 26) (len 3)) (segments (segment 0 (token "nut") (name "nut") (separator none) (span (offset 1084) (line 38) (column 26) (len 3)))))
    (reference r25 (scope relative) (span (offset 1127) (line 40) (column 25) (len 3)) (segments (segment 0 (token "Hub") (name "Hub") (separator none) (span (offset 1127) (line 40) (column 25) (len 3)))))
    (reference r26 (scope relative) (span (offset 1139) (line 40) (column 37) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 1139) (line 40) (column 37) (len 1)))))
    (reference r27 (scope relative) (span (offset 1144) (line 40) (column 42) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 1144) (line 40) (column 42) (len 1)))))
    (reference r28 (scope relative) (span (offset 1196) (line 41) (column 49) (len 1)) (segments (segment 0 (token "p") (name "p") (separator none) (span (offset 1196) (line 41) (column 49) (len 1)))))
    (reference r29 (scope relative) (span (offset 1201) (line 41) (column 54) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 1201) (line 41) (column 54) (len 1)))))
    (reference r30 (scope relative) (span (offset 1230) (line 42) (column 27) (len 9)) (segments (segment 0 (token "maxTorque") (name "maxTorque") (separator none) (span (offset 1230) (line 42) (column 27) (len 9)))))
    (reference r31 (scope relative) (span (offset 1254) (line 42) (column 51) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 1254) (line 42) (column 51) (len 1)))))
    (reference r32 (scope relative) (span (offset 1258) (line 42) (column 55) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 1258) (line 42) (column 55) (len 1)))))
    (reference r33 (scope relative) (span (offset 1311) (line 44) (column 40) (len 10)) (segments (segment 0 (token "efficiency") (name "efficiency") (separator none) (span (offset 1311) (line 44) (column 40) (len 10)))))
  )
  (root (package (name "Corpus") (body brace (use-case-def (name "Use") (modifiers) (body brace (import (target (span (span (offset 63) (line 3) (column 24) (len 8))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 68) (line 3) (column 29) (len 3))) (separator (span (offset 68) (line 3) (column 29) (len 2))) (marker (span (offset 70) (line 3) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (actor (name "user") (short-name none) (type none) (multiplicity none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 94) (line 4) (column 22) (len 12)) (ref r1)))))) (actor (name "driver") (short-name none) (type (ref r2)) (multiplicity (lower none) (upper (expression (span (offset 139) (line 5) (column 32) (len 1)) (integer 1)))) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 144) (line 5) (column 37) (len 13)) (ref r3)))))) (actor (name "plain") (short-name none) (type none) (multiplicity none)))) (connection-def (name "Selection") (modifiers) (extensions) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "cart") (span (offset 229) (line 9) (column 13) (len 4)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "product") (span (offset 254) (line 10) (column 13) (len 7)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (references none) (multiplicity none) (redefines none) (crosses none)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 292) (line 11) (column 20) (len 5)) (ref r7))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual true) (portion none) (extensions)) (declaration-name "run") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion snapshot) (extensions)) (declaration "start") (short-name none) (target none) (body brace (default-reference-usage (prefix (direction out) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 396) (line 15) (column 35) (len 8)) (bracket (base (expression (span (offset 396) (line 15) (column 35) (len 2)) (integer 35))) (operands (sequence-list (element first (expression (span (offset 400) (line 15) (column 39) (len 3)) (ref r10)))))))))) (body semicolon)))))) (requirement-def (name "MassRequirement") (modifiers) (body brace (default-reference-usage (prefix (direction in) (derived false) (variance none) (constant false)) (declaration-name "massActual") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r11)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (require-constraint (kind require) (constraint-keyword true) (name "c1") (target none) (typing none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (body semicolon)) (require-constraint (kind require) (constraint-keyword true) (name "c2") (target none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r14)))) (body semicolon)) (require-constraint (kind assume) (constraint-keyword true) (name "payloadLimit") (target none) (typing none) (extensions (ref r15)) (body semicolon)) (require-constraint (kind require) (constraint-keyword false) (name "massLimit") (target none) (typing none) (extensions (ref r16)) (body semicolon)) (require-constraint (kind require) (constraint-keyword false) (name none) (target (ref r17)) (typing none) (body semicolon)))) (flow-def (name "FuelFlow") (modifiers)) (interface-def (name "Hub") (modifiers) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "lugNuts") (span (offset 874) (line 32) (column 13) (len 7)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "shanks") (span (offset 905) (line 33) (column 13) (len 6)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (references none) (multiplicity none) (redefines none) (crosses none)) (interface-usage (form typed-connect) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "fastener") (short-name none) (type (ref r20)) (multiplicity (lower none) (upper (expression (span (offset 961) (line 34) (column 40) (len 1)) (integer 5)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (part (binary (from (interface-end (multiplicity none) (target (ref r21)))) (to (interface-end (multiplicity none) (target (ref r22)))))) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "wheel") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "lugNuts") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r23)))) (references none) (crosses none) (intersects none) (value none) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "nut") (short-name none) (typing none) (multiplicity (lower none) (upper (expression (span (offset 1089) (line 38) (column 31) (len 1)) (integer 5)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r24)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (interface-usage (form typed-connect) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "hub") (short-name none) (type (ref r25)) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part (binary (from (interface-end (multiplicity none) (target (ref r26)))) (to (interface-end (multiplicity none) (target (ref r27)))))) (body brace (interface-usage (form typed-connect) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "inner") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part (binary (from (interface-end (multiplicity none) (target (ref r28)))) (to (interface-end (multiplicity none) (target (ref r29)))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r30)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1242) (line 42) (column 39) (len 18)) (binary (operator "*") (left (expression (span (offset 1242) (line 42) (column 39) (len 2)) (integer 90))) (right (expression (span (offset 1247) (line 42) (column 44) (len 13)) (bracket (base (expression (span (offset 1247) (line 42) (column 44) (len 5)) (real "1.356"))) (operands (sequence-list (element first (expression (span (offset 1254) (line 42) (column 51) (len 5)) (binary (operator "*") (left (expression (span (offset 1254) (line 42) (column 51) (len 1)) (ref r31))) (right (expression (span (offset 1258) (line 42) (column 55) (len 1)) (ref r32)))))))))))))))) (body semicolon)))) (attribute-usage (declaration-name "efficiency") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r33)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1324) (line 44) (column 53) (len 2)) (real ".6"))))) (body semicolon)))))))
)
~~~
