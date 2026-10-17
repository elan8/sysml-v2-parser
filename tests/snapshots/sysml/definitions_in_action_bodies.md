# META
~~~sexpr
(snapshot (type semantic) (description "Issue 200 case 7: definitions in ActionBodyItem retain their declarations and bodies in action definitions, usages and state action bodies."))
~~~
# SOURCE
~~~sysml
package DefinitionsInActions {
    action def A {
        part def P { part p; }
        item def I { attribute a; }
        view def V { view v; }
        rendering def R;
        viewpoint def VP;
    }
    action a {
        part def P;
        item def I;
        view def V;
        rendering def R;
        viewpoint def VP;
    }
    state def S {
        do action { part def P; item def I; }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "definitions_in_action_bodies.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package DefinitionsInActions {
    action def A {
        part def P {
            part p;
        }
        item def I {
            attribute a;
        }
        view def V {
            view v;
        }
        rendering def R;
        viewpoint def VP;
    }
    action a {
        part def P;
        item def I;
        view def V;
        rendering def R;
        viewpoint def VP;
    }
    state def S {
        do action {
            part def P;
            item def I;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
  )
  (root (package (name "DefinitionsInActions") (body brace (action-def (name "A") (modifiers) (specializes none) (body brace (part-def (name "P") (modifiers) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "p") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (item-def (name "I") (modifiers) (individual false) (specializes none) (body brace (attribute-usage (declaration-name "a") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (view-def (name "V") (short-name none) (modifiers) (specializes none) (body brace (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "v") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body semicolon)))) (rendering-def (name "R") (short-name none) (modifiers) (specializes none) (body semicolon)) (viewpoint-def (name "VP") (short-name none) (modifiers) (specializes none) (body semicolon)))) (action-usage (keyword action) (name "a") (short-name none) (prefix (abstract false) (variation false) (reference false) (individual false)) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (body brace (part-def (name "P") (modifiers) (body semicolon)) (item-def (name "I") (modifiers) (individual false) (specializes none) (body semicolon)) (view-def (name "V") (short-name none) (modifiers) (specializes none) (body semicolon)) (rendering-def (name "R") (short-name none) (modifiers) (specializes none) (body semicolon)) (viewpoint-def (name "VP") (short-name none) (modifiers) (specializes none) (body semicolon)))) (state-def (name "S") (modifiers) (body brace (do (action (action (name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none)) (body brace (part-def (name "P") (modifiers) (body semicolon)) (item-def (name "I") (modifiers) (individual false) (specializes none) (body semicolon)))))))))
)
~~~
