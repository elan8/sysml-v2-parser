# META
~~~sexpr
(snapshot (type semantic) (description "`that` is a declared feature of the Kernel Semantic Library (Base::things::that), not a language keyword: it is authorable as a declaration and reaches expressions as an ordinary arena-backed reference, so declaration and reference stay distinct identities and reserving the spelling would make the library itself unparseable (spec42 Gap 41)."))
~~~
# SOURCE
~~~sysml
package ThatSelfReference {
    type Anything {
        feature self : Anything[1] subsets things chains things.that;
    }
    abstract feature things : Anything[1..*] nonunique {
        feature that : Anything[1];
    }
    datatype UnitBoundedReal :> Real {
        inv unitBound {
            -1.0 <= that & that <= 1.0
        }
    }
    part def Occurrences {
        constraint viaCast {
            (that as Occurrence).member
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "that_self_reference.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ThatSelfReference {
    type Anything {
        feature self : Anything[1] :> things chains things.that;
    }
    abstract feature things : Anything[1..*] nonunique {
        feature that : Anything[1];
    }
    datatype UnitBoundedReal :> Real {
        inv unitBound {
            -1.0 <= that & that <= 1.0;
        }
    }
    part def Occurrences {
        constraint viaCast {
            (that as Occurrence).member;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 255) (line 8) (column 33) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 255) (line 8) (column 33) (len 4)))))
    (reference r1 (scope relative) (span (offset 410) (line 15) (column 14) (len 4)) (segments (segment 0 (token "that") (name "that") (separator none) (span (offset 410) (line 15) (column 14) (len 4)))))
    (reference r2 (scope relative) (span (offset 418) (line 15) (column 22) (len 10)) (segments (segment 0 (token "Occurrence") (name "Occurrence") (separator none) (span (offset 418) (line 15) (column 22) (len 10)))))
    (reference r3 (scope relative) (span (offset 430) (line 15) (column 34) (len 6)) (segments (segment 0 (token "member") (name "member") (separator none) (span (offset 430) (line 15) (column 34) (len 6)))))
  )
  (root (package (name "ThatSelfReference") (body brace (kerml-classifier (keyword type) (abstract false) (name "Anything") (specializes none) (body brace (kerml-feature))) (kerml-feature (name "things") (body brace (kerml-feature))) (kerml-classifier (keyword datatype) (abstract false) (name "UnitBoundedReal") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body brace (invariant))) (part-def (name "Occurrences") (modifiers) (body brace (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "viaCast") (short-name none) (type none) (multiplicity none) (subsets none) (redefines none) (body brace (expression (span (offset 409) (line 15) (column 13) (len 27)) (member-access (base (expression (span (offset 409) (line 15) (column 13) (len 20)) (parenthesized (expression (span (offset 410) (line 15) (column 14) (len 18)) (type-check (kind as) (operand (expression (span (offset 410) (line 15) (column 14) (len 4)) (ref r1))) (type (ref r2))))))) (separator dot) (member (ref r3)))))))))))
)
~~~
