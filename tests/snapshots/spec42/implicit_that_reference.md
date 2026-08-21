# META
~~~sexpr
(snapshot (type semantic) (description "`that` is not a keyword of the pinned textual notation -- it is declared as an ordinary feature of Base::things (Base.kerml) and inherited, so it lexes as the identifier it is and resolves by name like any other. The same source therefore both declares it and refers to it, and a cast chain over it reaches the AST fully typed: a member access whose base is a parenthesized `as` type check whose operand is the reference. Nothing here needs a lexical marker; a parser-level keyword would make the declaration below unparseable (spec42 Gap 41)."))
~~~
# SOURCE
~~~sysml
package ImplicitThatReference {
    abstract feature things : Anything [1..*] nonunique {
        feature that : Anything[1];
    }
    datatype UnitBoundedReal :> Real {
        inv unitBound {
            -1.0 <= that & that <= 1.0
        }
    }
    part def Casts {
        attribute enclosing = (that as Occurrence).member;
        attribute qualified = Base::things::that;
        attribute shadowing = that;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "implicit_that_reference.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ImplicitThatReference {
    abstract feature things : Anything[1..*] nonunique {
        feature that : Anything[1];
    }
    datatype UnitBoundedReal :> Real {
        inv unitBound {
            -1.0 <= that & that <= 1.0;
        }
    }
    part def Casts {
        attribute enclosing = (that as Occurrence).member;
        attribute qualified = Base::things::that;
        attribute shadowing = that;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 164) (line 5) (column 33) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 164) (line 5) (column 33) (len 4)))))
    (reference r1 (scope relative) (span (offset 302) (line 11) (column 32) (len 4)) (segments (segment 0 (token "that") (name "that") (separator none) (span (offset 302) (line 11) (column 32) (len 4)))))
    (reference r2 (scope relative) (span (offset 310) (line 11) (column 40) (len 10)) (segments (segment 0 (token "Occurrence") (name "Occurrence") (separator none) (span (offset 310) (line 11) (column 40) (len 10)))))
    (reference r3 (scope relative) (span (offset 322) (line 11) (column 52) (len 6)) (segments (segment 0 (token "member") (name "member") (separator none) (span (offset 322) (line 11) (column 52) (len 6)))))
    (reference r4 (scope relative) (span (offset 360) (line 12) (column 31) (len 18)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 360) (line 12) (column 31) (len 4))) (segment 1 (token "things") (name "things") (separator colon-colon) (span (offset 366) (line 12) (column 37) (len 6))) (segment 2 (token "that") (name "that") (separator colon-colon) (span (offset 374) (line 12) (column 45) (len 4)))))
    (reference r5 (scope relative) (span (offset 410) (line 13) (column 31) (len 4)) (segments (segment 0 (token "that") (name "that") (separator none) (span (offset 410) (line 13) (column 31) (len 4)))))
  )
  (root (package (name "ImplicitThatReference") (body brace (kerml-feature (name "things") (relationships) (value none) (body brace (kerml-feature (name "that") (relationships) (value none) (body semicolon)))) (kerml-classifier (keyword datatype) (abstract false) (name "UnitBoundedReal") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r0)))) (body brace (invariant))) (part-def (name "Casts") (modifiers) (body brace (attribute-usage (declaration-name "enclosing") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 301) (line 11) (column 31) (len 27)) (member-access (base (expression (span (offset 301) (line 11) (column 31) (len 20)) (sequence (sequence-list (element first (expression (span (offset 302) (line 11) (column 32) (len 18)) (type-check (kind as) (operand (expression (span (offset 302) (line 11) (column 32) (len 4)) (ref r1))) (type (ref r2))))))))) (separator dot) (member (ref r3))))))) (body semicolon)) (attribute-usage (declaration-name "qualified") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 360) (line 12) (column 31) (len 18)) (ref r4))))) (body semicolon)) (attribute-usage (declaration-name "shadowing") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 410) (line 13) (column 31) (len 4)) (ref r5))))) (body semicolon)))))))
)
~~~
