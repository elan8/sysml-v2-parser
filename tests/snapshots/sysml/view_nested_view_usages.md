# META
~~~sexpr
(snapshot (type semantic) (description "Nested view usages are members of view usage and view definition bodies, in authored order and recursively, in every header form view_usage accepts: plain, typed, with multiplicity, abstract, and the anonymous :>> redefinition; viewpoint members still dispatch to their own production."))
~~~
# SOURCE
~~~sysml
package NestedViews {
    view def DocumentType {
        doc /* A document type decomposed into sections. */
        view introduction;
        abstract view appendices[0..*];
        viewpoint reviewer;
    }
    view document : DocumentType {
        view zeta {
            doc /* Authored first. */
            view background;
        }
        view alpha : DocumentType {
            expose NestedViews::*;
        }
        view :>> introduction {
            doc /* Redefines the inherited section. */
        }
        view mid;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "view_nested_view_usages.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package NestedViews {
    view def DocumentType {
        doc
        /* A document type decomposed into sections. */
        view introduction;
        abstract view appendices[0..*];
        viewpoint reviewer;
    }
    view document : DocumentType {
        view zeta {
            doc
            /* Authored first. */
            view background;
        }
        view alpha : DocumentType {
            expose NestedViews::*;
        }
        view :>> introduction {
            doc
            /* Redefines the inherited section. */
        }
        view mid;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 231) (line 8) (column 21) (len 12)) (segments (segment 0 (token "DocumentType") (name "DocumentType") (separator none) (span (offset 231) (line 8) (column 21) (len 12)))))
    (reference r1 (scope relative) (span (offset 364) (line 13) (column 22) (len 12)) (segments (segment 0 (token "DocumentType") (name "DocumentType") (separator none) (span (offset 364) (line 13) (column 22) (len 12)))))
    (reference r2 (scope relative) (span (offset 398) (line 14) (column 20) (len 11)) (segments (segment 0 (token "NestedViews") (name "NestedViews") (separator none) (span (offset 398) (line 14) (column 20) (len 11)))))
    (reference r3 (scope relative) (span (offset 441) (line 16) (column 18) (len 12)) (segments (segment 0 (token "introduction") (name "introduction") (separator none) (span (offset 441) (line 16) (column 18) (len 12)))))
  )
  (root (package (name "NestedViews") (body brace (view-def (name "DocumentType") (short-name none) (modifiers) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 64) (line 3) (column 15) (len 43)) (normalized "A document type decomposed into sections. "))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "introduction") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)) (view (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "appendices") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 170) (line 5) (column 34) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)) (viewpoint-usage))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "document") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "zeta") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 284) (line 10) (column 19) (len 17)) (normalized "Authored first. "))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "background") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "alpha") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (expose (target (span (span (offset 398) (line 14) (column 20) (len 14))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 409) (line 14) (column 31) (len 3))) (separator (span (offset 409) (line 14) (column 31) (len 2))) (marker (span (offset 411) (line 14) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (value none) (body brace (doc (name none) (locale none) (body (span (offset 474) (line 17) (column 19) (len 34)) (normalized "Redefines the inherited section. "))))) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "mid") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)))))))
)
~~~
