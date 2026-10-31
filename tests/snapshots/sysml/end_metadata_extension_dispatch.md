# META
~~~sexpr
(snapshot (type semantic) (description "Migrated GH33 control: scope-owned connector/interface ends keep prefix metadata when their syntax overlaps ExtendedUsage, while ordinary tagged declarations retain their extension nodes."))
~~~
# SOURCE
~~~sysml
package P {
    requirement def R1;
    requirement def R2;
    #derivation connection { end #original ::> R1; end #derive ::> R2; }
    interface def I { end #logical ::> R1; end #physical p : R2; }
    connection def C {
        /* This comment belongs before the end. */
        end #physical p : R2;
        #q value;
    }
    interface i {
        end #physical p : R2;
        #q def Inner;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "end_metadata_extension_dispatch.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    requirement def R1;
    requirement def R2;
    #derivation connection {
        end #original ::> R1;
        end #derive ::> R2;
    }
    interface def I {
        end #logical ::> R1;
        end #physical p : R2;
    }
    connection def C {
        /* This comment belongs before the end. */
        end #physical p : R2;
        #q value;
    }
    interface i {
        end #physical p : R2;
        #q def Inner;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 65) (line 4) (column 6) (len 10)) (segments (segment 0 (token "derivation") (name "derivation") (separator none) (span (offset 65) (line 4) (column 6) (len 10)))))
    (reference r1 (scope relative) (span (offset 94) (line 4) (column 35) (len 8)) (segments (segment 0 (token "original") (name "original") (separator none) (span (offset 94) (line 4) (column 35) (len 8)))))
    (reference r2 (scope relative) (span (offset 107) (line 4) (column 48) (len 2)) (segments (segment 0 (token "R1") (name "R1") (separator none) (span (offset 107) (line 4) (column 48) (len 2)))))
    (reference r3 (scope relative) (span (offset 116) (line 4) (column 57) (len 6)) (segments (segment 0 (token "derive") (name "derive") (separator none) (span (offset 116) (line 4) (column 57) (len 6)))))
    (reference r4 (scope relative) (span (offset 127) (line 4) (column 68) (len 2)) (segments (segment 0 (token "R2") (name "R2") (separator none) (span (offset 127) (line 4) (column 68) (len 2)))))
    (reference r5 (scope relative) (span (offset 160) (line 5) (column 28) (len 7)) (segments (segment 0 (token "logical") (name "logical") (separator none) (span (offset 160) (line 5) (column 28) (len 7)))))
    (reference r6 (scope relative) (span (offset 172) (line 5) (column 40) (len 2)) (segments (segment 0 (token "R1") (name "R1") (separator none) (span (offset 172) (line 5) (column 40) (len 2)))))
    (reference r7 (scope relative) (span (offset 181) (line 5) (column 49) (len 8)) (segments (segment 0 (token "physical") (name "physical") (separator none) (span (offset 181) (line 5) (column 49) (len 8)))))
    (reference r8 (scope relative) (span (offset 194) (line 5) (column 62) (len 2)) (segments (segment 0 (token "R2") (name "R2") (separator none) (span (offset 194) (line 5) (column 62) (len 2)))))
    (reference r9 (scope relative) (span (offset 287) (line 8) (column 14) (len 8)) (segments (segment 0 (token "physical") (name "physical") (separator none) (span (offset 287) (line 8) (column 14) (len 8)))))
    (reference r10 (scope relative) (span (offset 300) (line 8) (column 27) (len 2)) (segments (segment 0 (token "R2") (name "R2") (separator none) (span (offset 300) (line 8) (column 27) (len 2)))))
    (reference r11 (scope relative) (span (offset 313) (line 9) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 313) (line 9) (column 10) (len 1)))))
    (reference r12 (scope relative) (span (offset 359) (line 12) (column 14) (len 8)) (segments (segment 0 (token "physical") (name "physical") (separator none) (span (offset 359) (line 12) (column 14) (len 8)))))
    (reference r13 (scope relative) (span (offset 372) (line 12) (column 27) (len 2)) (segments (segment 0 (token "R2") (name "R2") (separator none) (span (offset 372) (line 12) (column 27) (len 2)))))
    (reference r14 (scope relative) (span (offset 385) (line 13) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 385) (line 13) (column 10) (len 1)))))
  )
  (root (package (visibility none) (name "P") (body brace (requirement-def (name "R1") (modifiers) (body semicolon)) (requirement-def (name "R2") (modifiers) (body semicolon)) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r0))) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (connect) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r1)) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r2)))) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r3)) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r4)))) (multiplicity none) (redefines none) (crosses none)))) (interface-def (name "I") (modifiers) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r5)) (short-name none) (identity anonymous) (typing none) (references (relationship (kind references) (implied false) (targets (ref r6)))) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r7)) (short-name none) (identity (declaration (name "p") (span (offset 190) (line 5) (column 58) (len 1)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (references none) (multiplicity none) (redefines none) (crosses none)))) (connection-def (name "C") (modifiers) (extensions) (specializes none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 233) (line 7) (column 11) (len 38)) (normalized "This comment belongs before the end. "))) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r9)) (short-name none) (identity (declaration (name "p") (span (offset 296) (line 8) (column 23) (len 1)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (references none) (multiplicity none) (redefines none) (crosses none)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r11)) (declaration (name "value") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (interface-usage (form declaration) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "i") (short-name none) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (part none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions (ref r12)) (short-name none) (identity (declaration (name "p") (span (offset 368) (line 12) (column 23) (len 1)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (references none) (multiplicity none) (redefines none) (crosses none)) (extended-def (prefix-keywords ((ref r14))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)))))))
)
~~~
