# META
~~~sexpr
(snapshot (type semantic) (description "SysML semantic owner validation needs typed expose, requirement-verification, and view-rendering memberships even when their concrete owner is invalid. This fixture pairs the legal view/requirement owners with invalid package/part owners and keeps render distinct from ordinary rendering usage."))
~~~
# SOURCE
~~~sysml
package MembershipOwners {
    part def Component;
    requirement def Limit;
    rendering def Tree;
    view legalView {
        expose Component;
        render asTree : Tree;
    }
    requirement def LegalRequirement {
        verify requirement limit : Limit;
    }
    expose Component;
    part def InvalidOwner {
        verify requirement limit : Limit;
        render asTree : Tree;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "semantic_membership_invalid_owners.md"
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
  (references
    (reference r0 (scope relative) (span (offset 138) (line 6) (column 16) (len 9)) (segments (segment 0 (token "Component") (name "Component") (separator none) (span (offset 138) (line 6) (column 16) (len 9)))))
    (reference r1 (scope relative) (span (offset 173) (line 7) (column 25) (len 4)) (segments (segment 0 (token "Tree") (name "Tree") (separator none) (span (offset 173) (line 7) (column 25) (len 4)))))
    (reference r2 (scope relative) (span (offset 259) (line 10) (column 36) (len 5)) (segments (segment 0 (token "Limit") (name "Limit") (separator none) (span (offset 259) (line 10) (column 36) (len 5)))))
    (reference r3 (scope relative) (span (offset 283) (line 12) (column 12) (len 9)) (segments (segment 0 (token "Component") (name "Component") (separator none) (span (offset 283) (line 12) (column 12) (len 9)))))
    (reference r4 (scope relative) (span (offset 357) (line 14) (column 36) (len 5)) (segments (segment 0 (token "Limit") (name "Limit") (separator none) (span (offset 357) (line 14) (column 36) (len 5)))))
    (reference r5 (scope relative) (span (offset 388) (line 15) (column 25) (len 4)) (segments (segment 0 (token "Tree") (name "Tree") (separator none) (span (offset 388) (line 15) (column 25) (len 4)))))
  )
  (root (package (name "MembershipOwners") (body brace (part-def (name "Component") (modifiers) (body semicolon)) (requirement-def (name "Limit") (modifiers) (body semicolon)) (rendering-def (modifiers)) (view (name "legalView") (short-name none) (type none) (body brace (expose (target (span (span (offset 138) (line 6) (column 16) (len 9))) (all none) (ref r0) (shape (membership (recursive-suffix none)))) (body semicolon)) (view-rendering (name "asTree") (type (ref r1)) (body semicolon)))) (requirement-def (name "LegalRequirement") (modifiers) (body brace (verify (explicit-requirement true) (requirement (name "limit") (type (ref r2)) (body semicolon)) (target none) (redefines none)))) (expose (target (span (span (offset 283) (line 12) (column 12) (len 9))) (all none) (ref r3) (shape (membership (recursive-suffix none)))) (body semicolon)) (part-def (name "InvalidOwner") (modifiers) (body brace (verify (explicit-requirement true) (requirement (name "limit") (type (ref r4)) (body semicolon)) (target none) (redefines none)) (view-rendering (name "asTree") (type (ref r5)) (body semicolon)))))))
)
~~~
