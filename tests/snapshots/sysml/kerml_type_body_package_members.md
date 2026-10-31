# META
~~~sexpr
(snapshot (type semantic) (description "KerML TypeBodyElement admits NonFeatureMember packages. Authored visibility lives on Package.membership / LibraryPackage.membership, the two alternatives stay distinct, and a later alias preserves sibling order."))
~~~
# SOURCE
~~~sysml
package TypeBodyPackageMembers {
    class Outer {
        public package Inner {
            type Nested specializes Base;
        }
        private standard library package StandardInner {
            type Unit specializes Base;
        }
        alias outerAlias for Inner;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml_type_body_package_members.md"
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
    (reference r0 (scope relative) (span (offset 118) (line 4) (column 37) (len 4)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 118) (line 4) (column 37) (len 4)))))
    (reference r1 (scope relative) (span (offset 225) (line 7) (column 35) (len 4)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 225) (line 7) (column 35) (len 4)))))
    (reference r2 (scope relative) (span (offset 270) (line 9) (column 30) (len 5)) (segments (segment 0 (token "Inner") (name "Inner") (separator none) (span (offset 270) (line 9) (column 30) (len 5)))))
  )
  (root (package (visibility none) (name "TypeBodyPackageMembers") (body brace (kerml-classifier (keyword class) (abstract false) (name "Outer") (multiplicity none) (specializes none) (conjugates none) (body brace (package (visibility public) (name "Inner") (body brace (kerml-classifier (keyword type) (abstract false) (name "Nested") (multiplicity none) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (conjugates none) (body semicolon)))) (library-package (visibility private) (name "StandardInner") (standard true) (body brace (kerml-classifier (keyword type) (abstract false) (name "Unit") (multiplicity none) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1)))) (conjugates none) (body semicolon)))) (alias (name "outerAlias") (target (ref r2)) (body semicolon)))))))
)
~~~
