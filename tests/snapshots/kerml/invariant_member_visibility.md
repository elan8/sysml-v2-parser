# META
~~~sexpr
(snapshot (type semantic) (description "KerML TypeBody FeatureMember -> OwnedFeatureMember owns MemberPrefix visibility before every FeatureElement alternative, including `inv`. The class and behavior owners retain private, protected, public, and absent visibility through canonical emission."))
~~~
# SOURCE
~~~sysml
package InvariantMemberVisibility {
    class C {
        private inv privateCheck { 1 == 1 }
        protected inv protectedCheck { 2 == 2 }
    }
    behavior B {
        public inv publicCheck { 3 == 3 }
        inv defaultCheck { 4 == 4 }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "invariant_member_visibility.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package InvariantMemberVisibility {
    class C {
        private inv privateCheck {
            1 == 1;
        }
        protected inv protectedCheck {
            2 == 2;
        }
    }
    behavior B {
        public inv publicCheck {
            3 == 3;
        }
        inv defaultCheck {
            4 == 4;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "InvariantMemberVisibility") (body brace (kerml-classifier (keyword class) (abstract false) (name "C") (specializes none) (conjugates none) (body brace (invariant) (invariant))) (kerml-classifier (keyword behavior) (abstract false) (name "B") (specializes none) (conjugates none) (body brace (invariant) (invariant))))))
)
~~~
