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
    (reference r2 (scope relative) (span (offset 810) (line 23) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 810) (line 23) (column 10) (len 8)))))
    (reference r3 (scope relative) (span (offset 1001) (line 29) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 1001) (line 29) (column 10) (len 8)))))
    (reference r4 (scope relative) (span (offset 1188) (line 35) (column 10) (len 8)) (segments (segment 0 (token "Approved") (name "Approved") (separator none) (span (offset 1188) (line 35) (column 10) (len 8)))))
  )
  (root (package (name "AnnotatingRequirementScopes") (body brace (requirement-def (name "Mass") (body brace (doc (name none) (locale none) (body (span (offset 79) (line 3) (column 15) (len 24)) (normalized "requirement definition "))) (comment (keyword (span (offset 114) (line 4) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 124) (line 4) (column 19) (len 19)) (normalized "requirement aside "))) (textual-rep (name "requirementRep") (language "text") (body (span (offset 191) (line 5) (column 46) (len 23)) (normalized "requirement rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r0)) (about) (body semicolon)))) (use-case-def (name "Deliver") (body brace (doc (name none) (locale none) (body (span (offset 283) (line 9) (column 15) (len 21)) (normalized "use case definition "))) (comment (keyword (span (offset 315) (line 10) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 325) (line 10) (column 19) (len 16)) (normalized "use case aside "))) (textual-rep (name "useCaseRep") (language "text") (body (span (offset 385) (line 11) (column 42) (len 20)) (normalized "use case rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r1)) (about) (body semicolon)) (return-ref (name "delivered") (body-span (span (offset 456) (line 13) (column 30) (len 161))) (body brace (doc (name none) (locale none) (body (span (offset 476) (line 14) (column 19) (len 17)) (normalized "return ref body "))) (comment (keyword (span (offset 508) (line 15) (column 13) (len 7))) (name none) (about) (locale none) (body (span (offset 518) (line 15) (column 23) (len 18)) (normalized "return ref aside "))) (textual-rep (name "returnRep") (language "text") (body (span (offset 583) (line 16) (column 45) (len 22)) (normalized "return ref rendering "))))))) (constraint-def (name "Bounded") (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 667) (line 20) (column 15) (len 23)) (normalized "constraint definition "))) (comment (keyword (span (offset 701) (line 21) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 711) (line 21) (column 19) (len 18)) (normalized "constraint aside "))) (textual-rep (name "constraintRep") (language "text") (body (span (offset 776) (line 22) (column 45) (len 22)) (normalized "constraint rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r2)) (about) (body semicolon)))) (calc-def (name "Total") (body brace (doc (name none) (locale none) (body (span (offset 861) (line 26) (column 15) (len 24)) (normalized "calculation definition "))) (comment (keyword (span (offset 896) (line 27) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 906) (line 27) (column 19) (len 19)) (normalized "calculation aside "))) (textual-rep (name "calcRep") (language "text") (body (span (offset 966) (line 28) (column 39) (len 23)) (normalized "calculation rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r3)) (about) (body semicolon)))) (view-def (name "Overview") (short-name none) (modifiers) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 1055) (line 32) (column 15) (len 17)) (normalized "view definition "))) (comment (keyword (span (offset 1083) (line 33) (column 9) (len 7))) (name none) (about) (locale none) (body (span (offset 1093) (line 33) (column 19) (len 12)) (normalized "view aside "))) (textual-rep (name "viewDefRep") (language "text") (body (span (offset 1149) (line 34) (column 42) (len 27)) (normalized "view definition rendering "))) (metadata-annotation (prefixes) (introducer at) (declared-name none) (type (ref r4)) (about) (body semicolon)))) (rendering-def))))
)
~~~
