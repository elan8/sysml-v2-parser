# META
~~~sexpr
(snapshot (type semantic) (description "The shared occurrence-style definition body dispatches its structured members before falling back to opaque capture. A visibility prefix therefore no longer hides the member behind it, and ref members reach the body at all; a form the parser does not model is still retained with a diagnostic, which is what the fallback is for. Written as an occurrence definition rather than the flow definition the library shapes come from, because flow definitions have no emitter yet. An occurrence definition body still projects as a bare marker, so the format section is what shows the structured members survived: (stable-idempotent) means the emitted text is byte-identical to the source, so a dropped visibility prefix or ref member would break it. The diagnostics section pins the fallback."))
~~~
# SOURCE
~~~sysml
package OccurrenceBodyMembers {
    occurrence def Message {
        private attribute seBeforeNum : Natural[1];
        protected attribute teAfterNum : Natural[1];
        private part p : T;
        ref self : SuccessionFlow :>> Flow::self;
        private ref action thisConnection = self;
        connection :>> c connect a to b;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "occurrence_body_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sexpr
(stable-idempotent)
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 101) (line 3) (column 41) (len 7)) (segments (segment 0 (token "Natural") (name "Natural") (separator none) (span (offset 101) (line 3) (column 41) (len 7)))))
    (reference r1 (scope relative) (span (offset 154) (line 4) (column 42) (len 7)) (segments (segment 0 (token "Natural") (name "Natural") (separator none) (span (offset 154) (line 4) (column 42) (len 7)))))
    (reference r2 (scope relative) (span (offset 191) (line 5) (column 26) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 191) (line 5) (column 26) (len 1)))))
    (reference r3 (scope relative) (span (offset 213) (line 6) (column 20) (len 14)) (segments (segment 0 (token "SuccessionFlow") (name "SuccessionFlow") (separator none) (span (offset 213) (line 6) (column 20) (len 14)))))
    (reference r4 (scope relative) (span (offset 232) (line 6) (column 39) (len 10)) (segments (segment 0 (token "Flow") (name "Flow") (separator none) (span (offset 232) (line 6) (column 39) (len 4))) (segment 1 (token "self") (name "self") (separator colon-colon) (span (offset 238) (line 6) (column 45) (len 4)))))
    (reference r5 (scope relative) (span (offset 288) (line 7) (column 45) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 288) (line 7) (column 45) (len 4)))))
    (reference r6 (scope relative) (span (offset 317) (line 8) (column 24) (len 1)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 317) (line 8) (column 24) (len 1)))))
    (reference r7 (scope relative) (span (offset 327) (line 8) (column 34) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 327) (line 8) (column 34) (len 1)))))
    (reference r8 (scope relative) (span (offset 332) (line 8) (column 39) (len 1)) (segments (segment 0 (token "b") (name "b") (separator none) (span (offset 332) (line 8) (column 39) (len 1)))))
  )
  (root (package (visibility none) (name "OccurrenceBodyMembers") (body brace (occurrence-def (name "Message") (short-name none) (modifiers) (specializes none) (body brace (attribute-usage (declaration-name "seBeforeNum") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "teAfterNum") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (ref (name "self") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (subsets none) (body semicolon)) (ref (name "thisConnection") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (extensions) (kind action) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 288) (line 7) (column 45) (len 4)) (ref r5))))) (redefines none) (subsets none) (body semicolon)) (connection-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value none) (connect (expression (span (offset 327) (line 8) (column 34) (len 1)) (ref r7)) (expression (span (offset 332) (line 8) (column 39) (len 1)) (ref r8))) (body semicolon)))))))
)
~~~
