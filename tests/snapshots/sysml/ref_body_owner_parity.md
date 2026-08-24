# META
~~~sexpr
(snapshot (type semantic) (description "The same ref body members parse identically under every owner: UsageBody is DefinitionBody, so a ref body does not depend on whether a connection, part, action, or state declaration owns it."))
~~~
# SOURCE
~~~sysml
package RefBodyOwnerParity {
    connection def C {
        ref underConnection : Anything {
            doc /* shared */
            comment /* shared */
            rep shared language "text" /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
    part def P {
        ref underPart : Anything {
            doc /* shared */
            comment /* shared */
            rep shared language "text" /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
    action def A {
        ref underAction : Anything {
            doc /* shared */
            comment /* shared */
            rep shared language "text" /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
    state def S {
        ref underState : Anything {
            doc /* shared */
            comment /* shared */
            rep shared language "text" /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ref_body_owner_parity.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package RefBodyOwnerParity {
    connection def C {
        ref underConnection : Anything {
            doc
            /* shared */
            comment
            /* shared */
            rep shared language "text"
            /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
    part def P {
        ref underPart : Anything {
            doc
            /* shared */
            comment
            /* shared */
            rep shared language "text"
            /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
    action def A {
        ref underAction : Anything {
            doc
            /* shared */
            comment
            /* shared */
            rep shared language "text"
            /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
    state def S {
        ref underState : Anything {
            doc
            /* shared */
            comment
            /* shared */
            rep shared language "text"
            /* shared */
            @Meta about x;
            attribute mass : Real;
            ref nested;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 82) (line 3) (column 31) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 82) (line 3) (column 31) (len 8)))))
    (reference r1 (scope relative) (span (offset 220) (line 7) (column 14) (len 4)) (segments (segment 0 (token "Meta") (name "Meta") (separator none) (span (offset 220) (line 7) (column 14) (len 4)))))
    (reference r2 (scope relative) (span (offset 231) (line 7) (column 25) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 231) (line 7) (column 25) (len 1)))))
    (reference r3 (scope relative) (span (offset 263) (line 8) (column 30) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 263) (line 8) (column 30) (len 4)))))
    (reference r4 (scope relative) (span (offset 350) (line 13) (column 25) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 350) (line 13) (column 25) (len 8)))))
    (reference r5 (scope relative) (span (offset 488) (line 17) (column 14) (len 4)) (segments (segment 0 (token "Meta") (name "Meta") (separator none) (span (offset 488) (line 17) (column 14) (len 4)))))
    (reference r6 (scope relative) (span (offset 499) (line 17) (column 25) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 499) (line 17) (column 25) (len 1)))))
    (reference r7 (scope relative) (span (offset 531) (line 18) (column 30) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 531) (line 18) (column 30) (len 4)))))
    (reference r8 (scope relative) (span (offset 622) (line 23) (column 27) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 622) (line 23) (column 27) (len 8)))))
    (reference r9 (scope relative) (span (offset 760) (line 27) (column 14) (len 4)) (segments (segment 0 (token "Meta") (name "Meta") (separator none) (span (offset 760) (line 27) (column 14) (len 4)))))
    (reference r10 (scope relative) (span (offset 771) (line 27) (column 25) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 771) (line 27) (column 25) (len 1)))))
    (reference r11 (scope relative) (span (offset 803) (line 28) (column 30) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 803) (line 28) (column 30) (len 4)))))
    (reference r12 (scope relative) (span (offset 892) (line 33) (column 26) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 892) (line 33) (column 26) (len 8)))))
    (reference r13 (scope relative) (span (offset 1030) (line 37) (column 14) (len 4)) (segments (segment 0 (token "Meta") (name "Meta") (separator none) (span (offset 1030) (line 37) (column 14) (len 4)))))
    (reference r14 (scope relative) (span (offset 1041) (line 37) (column 25) (len 1)) (segments (segment 0 (token "x") (name "x") (separator none) (span (offset 1041) (line 37) (column 25) (len 1)))))
    (reference r15 (scope relative) (span (offset 1073) (line 38) (column 30) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1073) (line 38) (column 30) (len 4)))))
  )
  (root (package (name "RefBodyOwnerParity") (body brace (connection-def (name "C") (modifiers) (role ordinary) (specializes none) (body brace (ref (name "underConnection") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body brace (doc (name none) (locale none) (body (span (offset 111) (line 4) (column 19) (len 8)) (normalized "shared "))) (comment (keyword (span (offset 134) (line 5) (column 13) (len 7))) (name none) (about) (locale none) (body (span (offset 144) (line 5) (column 23) (len 8)) (normalized "shared "))) (textual-rep (name "shared") (language "text") (body (span (offset 196) (line 6) (column 42) (len 8)) (normalized "shared "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r1)) (about (ref r2)) (body semicolon)) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (ref (name "nested") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)))))) (part-def (name "P") (modifiers) (body brace (ref (name "underPart") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body brace (doc (name none) (locale none) (body (span (offset 379) (line 14) (column 19) (len 8)) (normalized "shared "))) (comment (keyword (span (offset 402) (line 15) (column 13) (len 7))) (name none) (about) (locale none) (body (span (offset 412) (line 15) (column 23) (len 8)) (normalized "shared "))) (textual-rep (name "shared") (language "text") (body (span (offset 464) (line 16) (column 42) (len 8)) (normalized "shared "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r5)) (about (ref r6)) (body semicolon)) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (ref (name "nested") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)))))) (action-def (name "A") (modifiers) (specializes none) (body brace (ref (name "underAction") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body brace (doc (name none) (locale none) (body (span (offset 651) (line 24) (column 19) (len 8)) (normalized "shared "))) (comment (keyword (span (offset 674) (line 25) (column 13) (len 7))) (name none) (about) (locale none) (body (span (offset 684) (line 25) (column 23) (len 8)) (normalized "shared "))) (textual-rep (name "shared") (language "text") (body (span (offset 736) (line 26) (column 42) (len 8)) (normalized "shared "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r9)) (about (ref r10)) (body semicolon)) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (ref (name "nested") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)))))) (state-def (name "S") (modifiers) (body brace (ref (name "underState") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body brace (doc (name none) (locale none) (body (span (offset 921) (line 34) (column 19) (len 8)) (normalized "shared "))) (comment (keyword (span (offset 944) (line 35) (column 13) (len 7))) (name none) (about) (locale none) (body (span (offset 954) (line 35) (column 23) (len 8)) (normalized "shared "))) (textual-rep (name "shared") (language "text") (body (span (offset 1006) (line 36) (column 42) (len 8)) (normalized "shared "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r13)) (about (ref r14)) (body semicolon)) (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (ref (name "nested") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body semicolon)))))))))
)
~~~
