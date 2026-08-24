# META
~~~sexpr
(snapshot (type semantic) (description "EndDecl preserves its immediate authored introducer across every shared owner: pinned SysML ReferenceUsage is (EndUsagePrefix | RefPrefix) 'ref' Usage (textual BNF 285-286 and 335-337), and the pinned Pilot grammar agrees. The existing KerML `feature` compatibility spelling is independently source-backed rather than discarded. This is deliberately distinct from Pilot-only `end part|port|item|occurrence` prefixes and Pilot-only bare DefaultReferenceUsage `end : ...` forms, which remain recovery cases."))
~~~
# SOURCE
~~~sysml
package EndDeclIntroducers {
    connection def ConnectionEnds {
        end ref bare;
        end [*] ref typed : Thing;
        end ref redirected ::> Target;
        end feature kermlFeature : Thing;
        end plain : Thing;
    }

    interface def InterfaceEnds {
        end ref interfaceRef : Thing;
    }

    part def PartOwner {
        ref holder {
            end ref partRef;
        }
    }

    occurrence OccurrenceOwner {
        end ref occurrenceRef : Thing;
    }

    part def InterfaceUsageOwner {
        interface link : InterfaceEnds {
            end ref interfaceUsageRef : Thing;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "end_decl_introducers.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package EndDeclIntroducers {
    connection def ConnectionEnds {
        end ref bare;
        end ref typed : Thing[*];
        end ref redirected ::> Target;
        end feature kermlFeature : Thing;
        end plain : Thing;
    }
    interface def InterfaceEnds {
        end ref interfaceRef : Thing;
    }
    part def PartOwner {
        ref holder {
            end ref partRef;
        }
    }
    occurrence OccurrenceOwner {
        end ref occurrenceRef : Thing;
    }
    part def InterfaceUsageOwner {
        interface link : InterfaceEnds {
            end ref interfaceUsageRef : Thing;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 115) (line 4) (column 29) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 115) (line 4) (column 29) (len 5)))))
    (reference r1 (scope relative) (span (offset 153) (line 5) (column 32) (len 6)) (segments (segment 0 (token "Target") (name "Target") (separator none) (span (offset 153) (line 5) (column 32) (len 6)))))
    (reference r2 (scope relative) (span (offset 196) (line 6) (column 36) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 196) (line 6) (column 36) (len 5)))))
    (reference r3 (scope relative) (span (offset 223) (line 7) (column 21) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 223) (line 7) (column 21) (len 5)))))
    (reference r4 (scope relative) (span (offset 302) (line 11) (column 32) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 302) (line 11) (column 32) (len 5)))))
    (reference r5 (scope relative) (span (offset 473) (line 21) (column 33) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 473) (line 21) (column 33) (len 5)))))
    (reference r6 (scope relative) (span (offset 603) (line 26) (column 41) (len 5)) (segments (segment 0 (token "Thing") (name "Thing") (separator none) (span (offset 603) (line 26) (column 41) (len 5)))))
  )
  (root (package (name "EndDeclIntroducers") (body brace (connection-def (name "ConnectionEnds") (modifiers) (role ordinary) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 77) (line 3) (column 13) (len 3)))) (short-name none) (identity (declaration (name "bare") (span (offset 81) (line 3) (column 17) (len 4)))) (typing none) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 103) (line 4) (column 17) (len 3)))) (short-name none) (identity (declaration (name "typed") (span (offset 107) (line 4) (column 21) (len 5)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (references none) (multiplicity (lower unbounded) (upper unbounded)) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 134) (line 5) (column 13) (len 3)))) (short-name none) (identity (declaration (name "redirected") (span (offset 138) (line 5) (column 17) (len 10)))) (typing none) (references (relationship (kind references) (implied false) (targets (ref r1)))) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (kerml-feature (span (offset 173) (line 6) (column 13) (len 7)))) (short-name none) (identity (declaration (name "kermlFeature") (span (offset 181) (line 6) (column 21) (len 12)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (references none) (multiplicity none) (redefines none) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (short-name none) (identity (declaration (name "plain") (span (offset 215) (line 7) (column 13) (len 5)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (references none) (multiplicity none) (redefines none) (crosses none)))) (interface-def (name "InterfaceEnds") (modifiers) (specializes none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 283) (line 11) (column 13) (len 3)))) (short-name none) (identity (declaration (name "interfaceRef") (span (offset 287) (line 11) (column 17) (len 12)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (references none) (multiplicity none) (redefines none) (crosses none)))) (part-def (name "PartOwner") (modifiers) (body brace (ref (name "holder") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines none) (subsets none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 378) (line 16) (column 17) (len 3)))) (short-name none) (identity (declaration (name "partRef") (span (offset 382) (line 16) (column 21) (len 7)))) (typing none) (references none) (multiplicity none) (redefines none) (crosses none)))))) (occurrence (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration "OccurrenceOwner") (short-name none) (target none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 453) (line 21) (column 13) (len 3)))) (short-name none) (identity (declaration (name "occurrenceRef") (span (offset 457) (line 21) (column 17) (len 13)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (references none) (multiplicity none) (redefines none) (crosses none)))) (part-def (name "InterfaceUsageOwner") (modifiers) (body brace (interface-usage (form declaration) (part none) (body brace (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer (reference (span (offset 579) (line 26) (column 17) (len 3)))) (short-name none) (identity (declaration (name "interfaceUsageRef") (span (offset 583) (line 26) (column 21) (len 17)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (references none) (multiplicity none) (redefines none) (crosses none)))))))))
)
~~~
