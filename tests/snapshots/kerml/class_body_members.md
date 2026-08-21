# META
~~~sexpr
(snapshot (type semantic) (description "KerML class/struct/datatype bodies accept feature members with member/composite/portion/var prefixes, step/expr/bool kinds, invariants, connectors with reference ends, nested class definitions, and var-prefixed directed parameters."))
~~~
# SOURCE
~~~sysml
package Classes {
    class A {
        feature innerSpaceDimension : Natural [1];
        member feature x : B;
        composite feature engine subsets carParts;
        portion feature all portions: Occurrence[1..*];
        var t : Integer;
        inv checkIt { x > 1 }
        connector a ::> a.x to b;
        class Inner { feature y : Integer; }
    }
    behavior B2 { out var y1; }
    package Q { connector a2 from x.s to y.t; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "class_body_members.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Classes {
    class A {
        feature innerSpaceDimension : Natural[1];
        member feature x : B;
        composite feature engine :> carParts;
        portion feature all portions : Occurrence[1..*];
        var t : Integer;
        inv checkIt {
            x > 1;
        }
        connector from a references a.x to b;
        class Inner {
            feature y : Integer;
        }
    }
    behavior B2 {
        out var y1;
    }
    package Q {
        connector a2 from x.s to y.t;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
  )
  (root (package (name "Classes") (body brace (kerml-classifier (keyword class) (abstract false) (name "A") (specializes none) (body brace (kerml-feature (name "innerSpaceDimension") (relationships) (value none) (body semicolon)) (kerml-feature (name "x") (relationships) (value none) (body semicolon)) (kerml-feature (name "engine") (relationships) (value none) (body semicolon)) (kerml-feature (name "portions") (relationships) (value none) (body semicolon)) (kerml-feature (name "t") (relationships) (value none) (body semicolon)) (invariant) (connector) (kerml-classifier (keyword class) (abstract false) (name "Inner") (specializes none) (body brace (kerml-feature (name "y") (relationships) (value none) (body semicolon)))))) (kerml-classifier (keyword behavior) (abstract false) (name "B2") (specializes none) (body brace (in-out-declaration))) (package (name "Q") (body brace (kerml-connector))))))
)
~~~
