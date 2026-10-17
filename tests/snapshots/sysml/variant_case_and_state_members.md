# META
~~~sexpr
(snapshot (type semantic) (description "Issue 200 case 8: variants in use case, case and state bodies retain VariantMembership and typed usages instead of becoming unrelated ordinary members."))
~~~
# SOURCE
~~~sysml
package VariantCases {
    variation use case choices {
        variant use case choiceOne;
        variant use case choiceTwo;
    }
    variation case def Cases {
        variant case c;
        variant analysis a;
        variant verification v;
        variant use case u;
    }
    variation state def Modes {
        variant state on;
        variant state off;
    }
    action def Variants {
        variant calc calculation;
        variant constraint condition;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "variant_case_and_state_members.md"
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
  )
  (root (package (name "VariantCases") (body brace (use-case-usage (declaration (name "choices") (short-name none) (abstract false) (type none) (subsets none)) (variation true) (body brace (variant-usage (target none) (usage (use-case-usage (declaration (name "choiceOne") (short-name none) (abstract false) (type none) (subsets none)) (variation false) (body semicolon))) (body absent)) (variant-usage (target none) (usage (use-case-usage (declaration (name "choiceTwo") (short-name none) (abstract false) (type none) (subsets none)) (variation false) (body semicolon))) (body absent)))) (case-def (modifiers (variation (span (offset 138) (line 6) (column 5) (len 9))))) (state-def (name "Modes") (modifiers (variation (span (offset 287) (line 12) (column 5) (len 9)))) (body brace (variant-usage (target none) (usage (state-usage (name "on") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))) (body absent)) (variant-usage (target none) (usage (state-usage (name "off") (short-name none) (prefix (direction none) (derived false) (abstract false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body semicolon))) (body absent)))) (action-def (name "Variants") (modifiers) (specializes none) (body brace (variant-usage (target none) (usage (calc-usage (name "calculation") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon))) (body absent)) (variant-usage (target none) (usage (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "condition") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body semicolon))) (body absent)))))))
)
~~~
