# META
~~~sexpr
(snapshot (type semantic) (description "Issue 200 case 1: extended members view metadata definitions preserve typed ExtendedDefinition and ExtendedUsage members through their grammar-owned bodies."))
~~~
# SOURCE
~~~sysml
package ExtendedOwners {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    view def V {
        #q def Inner;
        #q instance : Inner;
    }
    viewpoint def VP {
        #q def Inner;
        #q instance : Inner;
    }
    rendering def R {
        #q def Inner;
        #q instance : Inner;
    }
    metadata def M {
        #q def Inner;
        #q instance : Inner;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "extended_members_view_metadata_definitions.md"
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
    (reference r0 (scope relative) (span (offset 51) (line 2) (column 27) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 51) (line 2) (column 27) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 64) (line 2) (column 40) (len 16)))))
    (reference r1 (scope relative) (span (offset 108) (line 4) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 108) (line 4) (column 10) (len 1)))))
    (reference r2 (scope relative) (span (offset 130) (line 5) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 130) (line 5) (column 10) (len 1)))))
    (reference r3 (scope relative) (span (offset 143) (line 5) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 143) (line 5) (column 23) (len 5)))))
    (reference r4 (scope relative) (span (offset 188) (line 8) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 188) (line 8) (column 10) (len 1)))))
    (reference r5 (scope relative) (span (offset 210) (line 9) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 210) (line 9) (column 10) (len 1)))))
    (reference r6 (scope relative) (span (offset 223) (line 9) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 223) (line 9) (column 23) (len 5)))))
    (reference r7 (scope relative) (span (offset 267) (line 12) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 267) (line 12) (column 10) (len 1)))))
    (reference r8 (scope relative) (span (offset 289) (line 13) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 289) (line 13) (column 10) (len 1)))))
    (reference r9 (scope relative) (span (offset 302) (line 13) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 302) (line 13) (column 23) (len 5)))))
    (reference r10 (scope relative) (span (offset 345) (line 16) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 345) (line 16) (column 10) (len 1)))))
    (reference r11 (scope relative) (span (offset 367) (line 17) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 367) (line 17) (column 10) (len 1)))))
    (reference r12 (scope relative) (span (offset 380) (line 17) (column 23) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 380) (line 17) (column 23) (len 5)))))
  )
  (root (package (visibility none) (name "ExtendedOwners") (body brace (metadata-def (name "Q") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (view-def (name "V") (short-name none) (modifiers) (specializes none) (body brace (extended-def (prefix-keywords ((ref r1))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r2)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (viewpoint-def (name "VP") (short-name none) (modifiers) (specializes none) (body brace (extended-def (prefix-keywords ((ref r4))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r5)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (rendering-def (name "R") (short-name none) (modifiers) (specializes none) (body brace (extended-def (prefix-keywords ((ref r7))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r8)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (metadata-def (name "M") (abstract false) (specializes none) (body brace (extended-def (prefix-keywords ((ref r10))) (definition-prefix none) (name "Inner") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r11)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))))))
)
~~~
