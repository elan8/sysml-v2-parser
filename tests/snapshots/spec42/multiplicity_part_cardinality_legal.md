# META
~~~sexpr
(snapshot (type semantic) (description "Every spelling MultiplicityPart actually admits -- a range alone, each keyword slot alone, both slots in either order, and a range followed by either order -- parses with no diagnostic at all, in a part definition body and in a KerML type body. Narrowing the slots to the production's cardinality must not cost any of these (SysML BNF 492-496, KerML BNF 636-640)."))
~~~
# SOURCE
~~~sysml
package MultiplicityPartLegal {
    part def Legal {
        attribute rangeOnly : Real[0..*];
        attribute orderedOnly : Real[0..*] ordered;
        attribute nonuniqueOnly : Real[0..*] nonunique;
        attribute orderedThenNonunique : Real[0..*] ordered nonunique;
        attribute nonuniqueThenOrdered : Real[0..*] nonunique ordered;
        attribute leadingRangeOrderedFirst [0..*] ordered nonunique : Real;
        attribute leadingRangeNonuniqueFirst [0..*] nonunique ordered : Real;
        attribute noModifiers : Real;
        part orderedParts : Real[0..*] ordered nonunique;
    }
    type KermlLegal {
        feature rangeOnly : Real[0..*];
        feature orderedOnly : Real[0..*] ordered;
        feature nonuniqueOnly : Real[0..*] nonunique;
        feature orderedThenNonunique : Real[0..*] ordered nonunique;
        feature nonuniqueThenOrdered : Real[0..*] nonunique ordered;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "multiplicity_part_cardinality_legal.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package MultiplicityPartLegal {
    part def Legal {
        attribute rangeOnly : Real[0..*];
        attribute orderedOnly : Real[0..*] ordered;
        attribute nonuniqueOnly : Real[0..*] nonunique;
        attribute orderedThenNonunique : Real[0..*] ordered nonunique;
        attribute nonuniqueThenOrdered : Real[0..*] nonunique ordered;
        attribute leadingRangeOrderedFirst : Real[0..*] ordered nonunique;
        attribute leadingRangeNonuniqueFirst : Real[0..*] nonunique ordered;
        attribute noModifiers : Real;
        part orderedParts : Real[0..*] ordered nonunique;
    }
    type KermlLegal {
        feature rangeOnly : Real[0..*];
        feature orderedOnly : Real[0..*] ordered;
        feature nonuniqueOnly : Real[0..*] nonunique;
        feature orderedThenNonunique : Real[0..*] ordered nonunique;
        feature nonuniqueThenOrdered : Real[0..*] nonunique ordered;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 83) (line 3) (column 31) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 83) (line 3) (column 31) (len 4)))))
    (reference r1 (scope relative) (span (offset 127) (line 4) (column 33) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 127) (line 4) (column 33) (len 4)))))
    (reference r2 (scope relative) (span (offset 181) (line 5) (column 35) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 181) (line 5) (column 35) (len 4)))))
    (reference r3 (scope relative) (span (offset 244) (line 6) (column 42) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 244) (line 6) (column 42) (len 4)))))
    (reference r4 (scope relative) (span (offset 315) (line 7) (column 42) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 315) (line 7) (column 42) (len 4)))))
    (reference r5 (scope relative) (span (offset 415) (line 8) (column 71) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 415) (line 8) (column 71) (len 4)))))
    (reference r6 (scope relative) (span (offset 493) (line 9) (column 73) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 493) (line 9) (column 73) (len 4)))))
    (reference r7 (scope relative) (span (offset 531) (line 10) (column 33) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 531) (line 10) (column 33) (len 4)))))
    (reference r8 (scope relative) (span (offset 565) (line 11) (column 29) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 565) (line 11) (column 29) (len 4)))))
  )
  (root (package (name "MultiplicityPartLegal") (body brace (part-def (name "Legal") (modifiers) (body brace (attribute-usage (declaration-name "rangeOnly") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "orderedOnly") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "nonuniqueOnly") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "orderedThenNonunique") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "nonuniqueThenOrdered") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "leadingRangeOrderedFirst") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "leadingRangeNonuniqueFirst") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "noModifiers") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "orderedParts") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity (lower (expression (span (offset 570) (line 11) (column 34) (len 1)) (integer 0))) (upper unbounded)) (multiplicity-modifiers (ordering ordered) (uniqueness nonunique)) (subsets none) (redefines none) (value none) (body semicolon)))) (kerml-classifier (keyword type) (abstract false) (name "KermlLegal") (specializes none) (body brace (kerml-feature (name "rangeOnly") (relationships) (value none) (body semicolon)) (kerml-feature (name "orderedOnly") (relationships) (value none) (body semicolon)) (kerml-feature (name "nonuniqueOnly") (relationships) (value none) (body semicolon)) (kerml-feature (name "orderedThenNonunique") (relationships) (value none) (body semicolon)) (kerml-feature (name "nonuniqueThenOrdered") (relationships) (value none) (body semicolon)))))))
)
~~~
