# META
~~~sexpr
(snapshot (type semantic) (description "A `def`-less connection is a `ConnectionUsage` owning the whole `OccurrenceUsagePrefix` in every scope that owns one: a `#derivation` extension keyword, `abstract`, `ref`, `individual`, and the Systems-Library `: Type[0..*] nonunique :> a, b` declaration before the body. A `connection def` owns `DefinitionExtensionKeyword*` (`#multicausation`), and end declarations keep any `#Name` prefix metadata beside their declared name (`end #original r1 : R1;`, `end #cause c : R;`)."))
~~~
# SOURCE
~~~sysml
package ConnectionUsagePrefixes {
    requirement def R1;
    requirement def R2;
    connection def C;
    #derivation connection def D {
        end #original r1 : R1;
        end #derive r2 : R2;
    }
    #multicausation connection def M {
        end #cause c : R1;
        end #effect e : R2;
    }
    requirement req1 : R1;
    requirement req2 : R2;
    #derivation connection d {
        end #original ::> req1;
        end #derive ::> req2;
    }
    abstract connection cs : C[0..*] nonunique :> links, parts { }
    ref connection rc : C;
    part def Holder {
        #derivation connection : D {
            end r1 ::> req1;
            end r2 ::> req2;
        }
        ref connection held : C;
    }
    part holder {
        #derivation connection inPart : D;
        abstract connection many : C[*];
    }
    occurrence def Episode {
        individual connection once : C;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connection_usage_prefix_owning_scopes.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ConnectionUsagePrefixes {
    requirement def R1;
    requirement def R2;
    connection def C;
    #derivation connection def D {
        end #original r1 : R1;
        end #derive r2 : R2;
    }
    #multicausation connection def M {
        end #cause c : R1;
        end #effect e : R2;
    }
    requirement req1 : R1;
    requirement req2 : R2;
    #derivation connection d {
        end #original ::> req1;
        end #derive ::> req2;
    }
    abstract connection cs : C[0..*] nonunique :> links, parts {
    }
    ref connection rc : C;
    part def Holder {
        #derivation connection : D {
            end r1 ::> req1;
            end r2 ::> req2;
        }
        ref connection held : C;
    }
    part holder {
        #derivation connection inPart : D;
        abstract connection many : C[*];
    }
    occurrence def Episode {
        individual connection once : C;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 109) (line 5) (column 6) (len 10)) (segments (segment 0 (token "derivation") (name "derivation") (separator none) (span (offset 109) (line 5) (column 6) (len 10)))))
    (reference r1 (scope relative) (span (offset 152) (line 6) (column 14) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 152) (line 6) (column 14) (len 8)))))
    (reference r2 (scope relative) (span (offset 166) (line 6) (column 28) (len 2)) (segments (segment 0 (token "R1") (name "R1") (separator none) (span (offset 166) (line 6) (column 28) (len 2)))))
    (reference r3 (scope relative) (span (offset 183) (line 7) (column 14) (len 6)) (segments (segment 0 (token "derive") (name "derive") (separator none) (span (offset 183) (line 7) (column 14) (len 6)))))
    (reference r4 (scope relative) (span (offset 195) (line 7) (column 26) (len 2)) (segments (segment 0 (token "R2") (name "R2") (separator none) (span (offset 195) (line 7) (column 26) (len 2)))))
    (reference r5 (scope relative) (span (offset 210) (line 9) (column 6) (len 14)) (segments (segment 0 (token "multicausation") (name "multicausation") (separator none) (span (offset 210) (line 9) (column 6) (len 14)))))
    (reference r6 (scope relative) (span (offset 257) (line 10) (column 14) (len 5)) (segments (segment 0 (token "cause") (name "cause") (separator none) (span (offset 257) (line 10) (column 14) (len 5)))))
    (reference r7 (scope relative) (span (offset 267) (line 10) (column 24) (len 2)) (segments (segment 0 (token "R1") (name "R1") (separator none) (span (offset 267) (line 10) (column 24) (len 2)))))
    (reference r8 (scope relative) (span (offset 284) (line 11) (column 14) (len 6)) (segments (segment 0 (token "effect") (name "effect") (separator none) (span (offset 284) (line 11) (column 14) (len 6)))))
    (reference r9 (scope relative) (span (offset 295) (line 11) (column 25) (len 2)) (segments (segment 0 (token "R2") (name "R2") (separator none) (span (offset 295) (line 11) (column 25) (len 2)))))
    (reference r10 (scope relative) (span (offset 364) (line 15) (column 6) (len 10)) (segments (segment 0 (token "derivation") (name "derivation") (separator none) (span (offset 364) (line 15) (column 6) (len 10)))))
    (reference r11 (scope relative) (span (offset 403) (line 16) (column 14) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 403) (line 16) (column 14) (len 8)))))
    (reference r12 (scope relative) (span (offset 416) (line 16) (column 27) (len 4)) (segments (segment 0 (token "req1") (name "req1") (separator none) (span (offset 416) (line 16) (column 27) (len 4)))))
    (reference r13 (scope relative) (span (offset 435) (line 17) (column 14) (len 6)) (segments (segment 0 (token "derive") (name "derive") (separator none) (span (offset 435) (line 17) (column 14) (len 6)))))
    (reference r14 (scope relative) (span (offset 446) (line 17) (column 25) (len 4)) (segments (segment 0 (token "req2") (name "req2") (separator none) (span (offset 446) (line 17) (column 25) (len 4)))))
    (reference r15 (scope relative) (span (offset 487) (line 19) (column 30) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 487) (line 19) (column 30) (len 1)))))
    (reference r16 (scope relative) (span (offset 508) (line 19) (column 51) (len 5)) (segments (segment 0 (token "links") (name "links") (separator none) (span (offset 508) (line 19) (column 51) (len 5)))))
    (reference r17 (scope relative) (span (offset 515) (line 19) (column 58) (len 5)) (segments (segment 0 (token "parts") (name "parts") (separator none) (span (offset 515) (line 19) (column 58) (len 5)))))
    (reference r18 (scope relative) (span (offset 549) (line 20) (column 25) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 549) (line 20) (column 25) (len 1)))))
    (reference r19 (scope relative) (span (offset 583) (line 22) (column 10) (len 10)) (segments (segment 0 (token "derivation") (name "derivation") (separator none) (span (offset 583) (line 22) (column 10) (len 10)))))
    (reference r20 (scope relative) (span (offset 607) (line 22) (column 34) (len 1)) (segments (segment 0 (token "D") (name "D") (separator none) (span (offset 607) (line 22) (column 34) (len 1)))))
    (reference r21 (scope relative) (span (offset 634) (line 23) (column 24) (len 4)) (segments (segment 0 (token "req1") (name "req1") (separator none) (span (offset 634) (line 23) (column 24) (len 4)))))
    (reference r22 (scope relative) (span (offset 663) (line 24) (column 24) (len 4)) (segments (segment 0 (token "req2") (name "req2") (separator none) (span (offset 663) (line 24) (column 24) (len 4)))))
    (reference r23 (scope relative) (span (offset 709) (line 26) (column 31) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 709) (line 26) (column 31) (len 1)))))
    (reference r24 (scope relative) (span (offset 745) (line 29) (column 10) (len 10)) (segments (segment 0 (token "derivation") (name "derivation") (separator none) (span (offset 745) (line 29) (column 10) (len 10)))))
    (reference r25 (scope relative) (span (offset 776) (line 29) (column 41) (len 1)) (segments (segment 0 (token "D") (name "D") (separator none) (span (offset 776) (line 29) (column 41) (len 1)))))
    (reference r26 (scope relative) (span (offset 814) (line 30) (column 36) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 814) (line 30) (column 36) (len 1)))))
  )
  (root (package (name "ConnectionUsagePrefixes") (body brace (requirement-def (name "R1") (modifiers) (body semicolon)) (requirement-def (name "R2") (modifiers) (body semicolon)) (connection-def (name "C") (modifiers) (extensions) (specializes none) (body semicolon)) (connection-def (name "D") (modifiers) (extensions (ref r0)) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r1)) (short-name none) (identity (declaration (name "r1") (span (offset 161) (line 6) (column 23) (len 2)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r3)) (short-name none) (identity (declaration (name "r2") (span (offset 190) (line 7) (column 21) (len 2)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (references none) (multiplicity none) (redefines none) (crosses none)))) (connection-def (name "M") (modifiers) (extensions (ref r5)) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r6)) (short-name none) (identity (declaration (name "c") (span (offset 263) (line 10) (column 20) (len 1)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r8)) (short-name none) (identity (declaration (name "e") (span (offset 291) (line 11) (column 21) (len 1)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (references none) (multiplicity none) (redefines none) (crosses none)))) (requirement-usage (name "req1") (multiplicity none)) (requirement-usage (name "req2") (multiplicity none)) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r10))) (declaration-name "d") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r11)) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r12)))) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r13)) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r14)))) (multiplicity none) (redefines none) (crosses none)))) (connection-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "cs") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity (lower (expression (span (offset 489) (line 19) (column 32) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (subsets (clause (relationship (kind subsets) (implied false) (targets (ref r16) (ref r17))) (value none))) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body brace)) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "rc") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body semicolon)) (part-def (name "Holder") (modifiers) (body brace (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r19))) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "r1") (span (offset 627) (line 23) (column 17) (len 2)))) (typing none) (references (relationship (kind references) (implied false) (targets (ref r21)))) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "r2") (span (offset 656) (line 24) (column 17) (len 2)))) (typing none) (references (relationship (kind references) (implied false) (targets (ref r22)))) (multiplicity none) (redefines none) (crosses none)))) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "held") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "holder") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r24))) (declaration-name "inPart") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body semicolon)) (connection-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "many") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body semicolon)))) (occurrence-def (modifiers)))))
)
~~~
