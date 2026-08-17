# META
~~~sexpr
(snapshot (type semantic) (description "RequirementBodyItem and CaseBodyItem extend DefinitionBodyItem and ActionBodyItem respectively, and CalculationBodyItem extends ActionBodyItem, so requirement, case, constraint, calculation, view-definition and rendering-definition bodies all own the whole AnnotatingElement production. A ReturnParameterMember body is a UsageBody and owns it too."))
~~~
# SOURCE
~~~sysml
package AnnotatingRequirementScopes {
    requirement def Mass {
        doc /* requirement definition */
        comment /* requirement aside */
        rep requirementRep language "text" /* requirement rendering */
        @Approved;
    }
    use case def Deliver {
        doc /* use case definition */
        comment /* use case aside */
        rep useCaseRep language "text" /* use case rendering */
        @Approved;
        return ref delivered {
            doc /* return ref body */
            comment /* return ref aside */
            rep returnRep language "text" /* return ref rendering */
        }
    }
    constraint def Bounded {
        doc /* constraint definition */
        comment /* constraint aside */
        rep constraintRep language "text" /* constraint rendering */
        @Approved;
    }
    calc def Total {
        doc /* calculation definition */
        comment /* calculation aside */
        rep calcRep language "text" /* calculation rendering */
        @Approved;
    }
    view def Overview {
        doc /* view definition */
        comment /* view aside */
        rep viewDefRep language "text" /* view definition rendering */
        @Approved;
    }
    rendering def AsTree {
        doc /* rendering definition */
        comment /* rendering aside */
        rep renderingDefRep language "text" /* rendering definition rendering */
        @Approved;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "annotating_requirement_scopes.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package AnnotatingRequirementScopes {
    requirement def Mass {
        doc
        /* requirement definition */
        comment
        /* requirement aside */
        rep requirementRep language "text"
        /* requirement rendering */
        @Approved;
    }
    use case def Deliver {
        doc
        /* use case definition */
        comment
        /* use case aside */
        rep useCaseRep language "text"
        /* use case rendering */
        @Approved;
        return ref delivered {
            doc
            /* return ref body */
            comment
            /* return ref aside */
            rep returnRep language "text"
            /* return ref rendering */
        }
    }
    constraint def Bounded {
        doc
        /* constraint definition */
        comment
        /* constraint aside */
        rep constraintRep language "text"
        /* constraint rendering */
        @Approved;
    }
    calc def Total {
        doc
        /* calculation definition */
        comment
        /* calculation aside */
        rep calcRep language "text"
        /* calculation rendering */
        @Approved;
    }
    view def Overview {
        doc
        /* view definition */
        comment
        /* view aside */
        rep viewDefRep language "text"
        /* view definition rendering */
        @Approved;
    }
    rendering def AsTree {
        doc
        /* rendering definition */
        comment
        /* rendering aside */
        rep renderingDefRep language "text"
        /* rendering definition rendering */
        @Approved;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 226) (line 6) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 226) (line 6) (column 10) (len 8)))))
    (reference r1 (scope relative) (span (offset 417) (line 12) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 417) (line 12) (column 10) (len 8)))))
    (reference r2 (scope relative) (span (offset 1001) (line 29) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 1001) (line 29) (column 10) (len 8)))))
  )
  (root (package (name "AnnotatingRequirementScopes") (body brace (requirement-def (name "Mass") (body brace (doc) (comment (keyword (span (offset 114) (line 4) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation (declared-name none) (type (ref r0)) (about) (body semicolon)))) (use-case-def (name "Deliver") (body brace (doc) (comment (keyword (span (offset 315) (line 10) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation (declared-name none) (type (ref r1)) (about) (body semicolon)) (return-ref (name "delivered") (body-span (span (offset 456) (line 13) (column 30) (len 161))) (body brace (doc) (comment (keyword (span (offset 508) (line 15) (column 13) (len 7))) (name none) (about) (locale none)) (textual-rep))))) (constraint-def) (calc-def (name "Total") (body brace (doc) (comment (keyword (span (offset 896) (line 27) (column 9) (len 7))) (name none) (about) (locale none)) (textual-rep) (metadata-annotation (declared-name none) (type (ref r2)) (about) (body semicolon)))) (view-def) (rendering-def))))
)
~~~
