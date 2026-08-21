# META
~~~sexpr
(snapshot (type semantic) (description "A keyword-less /* ... */ member is dispatched before the member set of every scope whose dispatch is a keyword lookup or an ordered alt: the calc-shaped bodies that KerML type bodies share with a SysML calculation body, an action definition body, an action usage body, and a constraint body. Each comment is its own member and the declaration written after it keeps its name, its typing and its KerML type-relationship tail -- unions, intersects, differences and disjoint from -- exactly as it parses with no comment in front of it. The keyword-led comment spelling, unaffected throughout, is written alongside for contrast (spec42 Gap 60)."))
~~~
# SOURCE
~~~sysml
package BareCommentMemberDispatch {
    behavior UnionsBehavior {
        /* c */ feature f : Anything unions g;
                feature h : Anything unions g;
        comment /* c */ feature i : Anything unions g;
    }
    classifier IntersectsClassifier {
        /* c */ feature f : Anything intersects g;
                feature h : Anything intersects g;
    }
    struct DifferencesStruct {
        /* c */ feature f : Anything differences g;
                feature h : Anything differences g;
    }
    function DisjointFunction {
        /* c */ feature f : Anything disjoint from g;
                feature h : Anything disjoint from g;
    }
    datatype MultiplicityDatatype {
        /* c */ feature f : Anything [*];
                feature h : Anything [*];
    }
    calc def CalculationBody {
        /* c */ feature f : Anything unions g;
                feature h : Anything unions g;
    }
    action def ActionDefinitionBody {
        /* c */ attribute a : Real;
        /* c */ doc /* the following annotating member keeps its own keyword */
    }
    action actionUsageBody {
        /* c */ attribute a : Real;
    }
    constraint def ConstraintBody {
        /* c */ constraint nested;
        /* c */ doc /* the following annotating member keeps its own keyword */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "bare_comment_member_dispatch.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package BareCommentMemberDispatch {
    behavior UnionsBehavior {
        /* c */
        feature f : Anything unions g;
        feature h : Anything unions g;
        comment
        /* c */
        feature i : Anything unions g;
    }
    classifier IntersectsClassifier {
        /* c */
        feature f : Anything intersects g;
        feature h : Anything intersects g;
    }
    struct DifferencesStruct {
        /* c */
        feature f : Anything differences g;
        feature h : Anything differences g;
    }
    function DisjointFunction {
        /* c */
        feature f : Anything disjoint from g;
        feature h : Anything disjoint from g;
    }
    datatype MultiplicityDatatype {
        /* c */
        feature f : Anything[*];
        feature h : Anything[*];
    }
    calc def CalculationBody {
        /* c */
        feature f : Anything unions g;
        feature h : Anything unions g;
    }
    action def ActionDefinitionBody {
        /* c */
        attribute a : Real;
        /* c */
        doc
        /* the following annotating member keeps its own keyword */
    }
    action actionUsageBody {
        /* c */
        attribute a : Real;
    }
    constraint def ConstraintBody {
        /* c */
        constraint nested;
        /* c */
        doc
        /* the following annotating member keeps its own keyword */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 1130) (line 32) (column 31) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1130) (line 32) (column 31) (len 4)))))
  )
  (root (package (name "BareCommentMemberDispatch") (body brace (kerml-classifier (keyword behavior) (abstract false) (name "UnionsBehavior") (specializes none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 76) (line 3) (column 11) (len 3)) (normalized "c "))) (kerml-feature) (kerml-feature) (comment (keyword (span (offset 168) (line 5) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 178) (line 5) (column 19) (len 3)) (normalized "c "))) (kerml-feature))) (kerml-classifier (keyword classifier) (abstract false) (name "IntersectsClassifier") (specializes none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 269) (line 8) (column 11) (len 3)) (normalized "c "))) (kerml-feature) (kerml-feature))) (kerml-classifier (keyword struct) (abstract false) (name "DifferencesStruct") (specializes none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 408) (line 12) (column 11) (len 3)) (normalized "c "))) (kerml-feature) (kerml-feature))) (kerml-classifier (keyword function) (abstract false) (name "DisjointFunction") (specializes none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 550) (line 16) (column 11) (len 3)) (normalized "c "))) (kerml-feature) (kerml-feature))) (kerml-classifier (keyword datatype) (abstract false) (name "MultiplicityDatatype") (specializes none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 700) (line 20) (column 11) (len 3)) (normalized "c "))) (kerml-feature) (kerml-feature))) (calc-def (name "CalculationBody") (modifiers) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 821) (line 24) (column 11) (len 3)) (normalized "c "))) (kerml-feature) (kerml-feature))) (action-def (name "ActionDefinitionBody") (modifiers) (specializes none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 959) (line 28) (column 11) (len 3)) (normalized "c "))) (attribute-usage) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 995) (line 29) (column 11) (len 3)) (normalized "c "))) (doc (name none) (locale none) (body (span (offset 1007) (line 29) (column 23) (len 55)) (normalized "the following annotating member keeps its own keyword "))))) (action-usage (name "actionUsageBody") (short-name none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1110) (line 32) (column 11) (len 3)) (normalized "c "))) (attribute-usage (declaration-name "a") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (constraint-def (name "ConstraintBody") (modifiers) (specializes none) (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1188) (line 35) (column 11) (len 3)) (normalized "c "))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "nested") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1223) (line 36) (column 11) (len 3)) (normalized "c "))) (doc (name none) (locale none) (body (span (offset 1235) (line 36) (column 23) (len 55)) (normalized "the following annotating member keeps its own keyword "))))))))
)
~~~
