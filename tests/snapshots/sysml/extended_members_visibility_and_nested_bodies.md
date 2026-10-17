# META
~~~sexpr
(snapshot (type semantic) (description "Extension members retain visibility, short names, comments and nested bodies, while native prefixed usages and anonymous metadata-keyword usages keep their own alternatives."))
~~~
# SOURCE
~~~sysml
package ExtendedDetails {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    part def Owner {
        /* This comment remains a separate member. */
        private abstract #q def <inner> Inner {
            #q nested;
        }
        private #q instance : Inner = value;
        #q part native;
        #q;
    }
    action def A {
        action doSomething;
        merge m { #q def Nested; #q instance; }
    }
    state def S {
        do action { #q def Nested; #q instance; }
    }
    perform action work { #q def Nested; #q instance; }
    case def F { return ref result { #q def Nested; #q instance; } }
    view report {
        render rendering renderer { #q def Nested; #q instance; }
        render renderer { #q def Nested; #q instance; }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "extended_members_visibility_and_nested_bodies.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ExtendedDetails {
    metadata def <q> Q :> Metaobjects::SemanticMetadata;
    part def Owner {
        /* This comment remains a separate member. */
        private abstract #q def <inner> Inner {
            #q nested;
        }
        private #q instance : Inner = value;
        #q part native;
        #q;
    }
    action def A {
        action doSomething;
        merge m {
            #q def Nested;
            #q instance;
        }
    }
    state def S {
        do action {
            #q def Nested;
            #q instance;
        }
    }
    perform action work {
        #q def Nested;
        #q instance;
    }
    case def F {
        return ref result {
            #q def Nested;
            #q instance;
        }
    }
    view report {
        render rendering renderer {
            #q def Nested;
            #q instance;
        }
        render renderer {
            #q def Nested;
            #q instance;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 52) (line 2) (column 27) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 52) (line 2) (column 27) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 65) (line 2) (column 40) (len 16)))))
    (reference r1 (scope relative) (span (offset 184) (line 5) (column 27) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 184) (line 5) (column 27) (len 1)))))
    (reference r2 (scope relative) (span (offset 219) (line 6) (column 14) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 219) (line 6) (column 14) (len 1)))))
    (reference r3 (scope relative) (span (offset 256) (line 8) (column 18) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 256) (line 8) (column 18) (len 1)))))
    (reference r4 (scope relative) (span (offset 269) (line 8) (column 31) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 269) (line 8) (column 31) (len 5)))))
    (reference r5 (scope relative) (span (offset 277) (line 8) (column 39) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 277) (line 8) (column 39) (len 5)))))
    (reference r6 (scope relative) (span (offset 293) (line 9) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 293) (line 9) (column 10) (len 1)))))
    (reference r7 (scope relative) (span (offset 317) (line 10) (column 10) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 317) (line 10) (column 10) (len 1)))))
    (reference r8 (scope relative) (span (offset 387) (line 14) (column 15) (len 1)) (segments (segment 0 (token "m") (name "m") (separator none) (span (offset 387) (line 14) (column 15) (len 1)))))
    (reference r9 (scope relative) (span (offset 392) (line 14) (column 20) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 392) (line 14) (column 20) (len 1)))))
    (reference r10 (scope relative) (span (offset 407) (line 14) (column 35) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 407) (line 14) (column 35) (len 1)))))
    (reference r11 (scope relative) (span (offset 466) (line 17) (column 22) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 466) (line 17) (column 22) (len 1)))))
    (reference r12 (scope relative) (span (offset 481) (line 17) (column 37) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 481) (line 17) (column 37) (len 1)))))
    (reference r13 (scope relative) (span (offset 595) (line 20) (column 39) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 595) (line 20) (column 39) (len 1)))))
    (reference r14 (scope relative) (span (offset 610) (line 20) (column 54) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 610) (line 20) (column 54) (len 1)))))
    (reference r15 (scope relative) (span (offset 681) (line 22) (column 38) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 681) (line 22) (column 38) (len 1)))))
    (reference r16 (scope relative) (span (offset 696) (line 22) (column 53) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 696) (line 22) (column 53) (len 1)))))
    (reference r17 (scope relative) (span (offset 725) (line 23) (column 16) (len 8)) (segments (segment 0 (token "renderer") (name "renderer") (separator none) (span (offset 725) (line 23) (column 16) (len 8)))))
    (reference r18 (scope relative) (span (offset 737) (line 23) (column 28) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 737) (line 23) (column 28) (len 1)))))
    (reference r19 (scope relative) (span (offset 752) (line 23) (column 43) (len 1)) (segments (segment 0 (token "q") (name "q") (separator none) (span (offset 752) (line 23) (column 43) (len 1)))))
  )
  (root (package (name "ExtendedDetails") (body brace (metadata-def (name "Q") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body semicolon)) (part-def (name "Owner") (modifiers) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 114) (line 4) (column 11) (len 41)) (normalized "This comment remains a separate member. "))) (extended-def (prefix-keywords ((ref r1))) (definition-prefix abstract) (name "Inner") (short-name "inner") (visibility private) (specializes none) (body brace (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r2)) (declaration (name "nested") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (extended-usage (visibility private) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r3)) (declaration (name "instance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 277) (line 8) (column 39) (len 5)) (ref r5))))) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions (ref r6))) (declaration-name "native") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (metadata-keyword-usage (type (ref r7)) (body semicolon)))) (action-def (name "A") (modifiers) (specializes none) (body brace (action-usage (keyword action) (name "doSomething") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon)) (merge (prefix (direction none) (derived false) (variance none) (constant false) (individual false) (portion none) (extensions)) (declaration (named (expression (span (offset 387) (line 14) (column 15) (len 1)) (ref r8)))) (body brace (open-brace (span (offset 389) (line 14) (column 17) (len 1))) (members (extended-def (prefix-keywords ((ref r9))) (definition-prefix none) (name "Nested") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r10)) (declaration (name "instance") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon))) (close-brace (span (offset 419) (line 14) (column 47) (len 1))))))) (state-def (name "S") (modifiers) (body brace (do (action (action (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none)) (body brace (extended-def (prefix-keywords ((ref r11))) (definition-prefix none) (name "Nested") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r12)) (declaration (name "instance") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))))) (perform (target (action (name "work") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (value none) (body brace (action) (action))) (case-def (name "F") (modifiers) (body brace (return-ref (name "result") (short-name none) (body-span (span (offset 592) (line 20) (column 36) (len 31))) (body brace (extended-def (prefix-keywords ((ref r13))) (definition-prefix none) (name "Nested") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r14)) (declaration (name "instance") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "report") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (view-rendering (form inline "renderer") (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (extended-def (prefix-keywords ((ref r15))) (definition-prefix none) (name "Nested") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r16)) (declaration (name "instance") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))) (view-rendering (form reference (ref r17)) (type none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (extended-def (prefix-keywords ((ref r18))) (definition-prefix none) (name "Nested") (short-name none) (visibility none) (specializes none) (body semicolon)) (extended-usage (visibility none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false)) (extensions (ref r19)) (declaration (name "instance") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (body semicolon)))))))))
)
~~~
