# META
~~~sexpr
(snapshot (type semantic) (description "Direct upstream KerML Simple Tests/Features.kerml excerpt: TypeFeaturingPart follows a typed and subsetted feature declaration."))
~~~
# SOURCE
~~~sysml
package Features {
    classifier A;
    classifier B;
    feature f;
    feature g;
    feature x typed by A, B references f subsets g;
    classifier C;
    feature y;
    featuring F of y by C;
    feature y1 : A :> x featured by C;
    feature z unions f, g disjoint from y;
    feature z1 intersects f,g differences y, y1, z;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "upstream_features_type_featuring.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Features {
    classifier A;
    classifier B;
    feature f;
    feature g;
    feature x typed by A, B :> g ::> f;
    classifier C;
    feature y;
    featuring F of y by C;
    feature y1 : A :> x featured by C;
    feature z unions f, g disjoint from y;
    feature z1 intersects f, g differences y, y1, z;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 189) (line 9) (column 20) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 189) (line 9) (column 20) (len 1)))))
    (reference r1 (scope relative) (span (offset 194) (line 9) (column 25) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 194) (line 9) (column 25) (len 1)))))
    (reference r2 (scope relative) (span (offset 233) (line 10) (column 37) (len 1)) (segments (segment 0 (token "C") (name "C") (separator none) (span (offset 233) (line 10) (column 37) (len 1)))))
    (reference r3 (scope relative) (span (offset 257) (line 11) (column 22) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 257) (line 11) (column 22) (len 1)))))
    (reference r4 (scope relative) (span (offset 260) (line 11) (column 25) (len 1)) (segments (segment 0 (token "g") (name "g") (separator none) (span (offset 260) (line 11) (column 25) (len 1)))))
    (reference r5 (scope relative) (span (offset 276) (line 11) (column 41) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 276) (line 11) (column 41) (len 1)))))
    (reference r6 (scope relative) (span (offset 305) (line 12) (column 27) (len 1)) (segments (segment 0 (token "f") (name "f") (separator none) (span (offset 305) (line 12) (column 27) (len 1)))))
    (reference r7 (scope relative) (span (offset 307) (line 12) (column 29) (len 1)) (segments (segment 0 (token "g") (name "g") (separator none) (span (offset 307) (line 12) (column 29) (len 1)))))
    (reference r8 (scope relative) (span (offset 321) (line 12) (column 43) (len 1)) (segments (segment 0 (token "y") (name "y") (separator none) (span (offset 321) (line 12) (column 43) (len 1)))))
    (reference r9 (scope relative) (span (offset 324) (line 12) (column 46) (len 2)) (segments (segment 0 (token "y1") (name "y1") (separator none) (span (offset 324) (line 12) (column 46) (len 2)))))
    (reference r10 (scope relative) (span (offset 328) (line 12) (column 50) (len 1)) (segments (segment 0 (token "z") (name "z") (separator none) (span (offset 328) (line 12) (column 50) (len 1)))))
  )
  (root (package (name "Features") (body brace (kerml-classifier (keyword classifier) (abstract false) (name "A") (specializes none) (body semicolon)) (kerml-classifier (keyword classifier) (abstract false) (name "B") (specializes none) (body semicolon)) (kerml-feature (name "f") (relationships) (value none) (body semicolon)) (kerml-feature (name "g") (relationships) (value none) (body semicolon)) (kerml-feature (name "x") (relationships) (value none) (body semicolon)) (kerml-classifier (keyword classifier) (abstract false) (name "C") (specializes none) (body semicolon)) (kerml-feature (name "y") (relationships) (value none) (body semicolon)) (kerml-relationship (keyword featuring) (source (ref r0)) (target (ref r1))) (kerml-feature (name "y1") (relationships (featured-by (ref r2))) (value none) (body semicolon)) (kerml-feature (name "z") (relationships (type-relationship (keyword unions) (targets (ref r3) (ref r4))) (type-relationship (keyword disjoint from) (targets (ref r5)))) (value none) (body semicolon)) (kerml-feature (name "z1") (relationships (type-relationship (keyword intersects) (targets (ref r6) (ref r7))) (type-relationship (keyword differences) (targets (ref r8) (ref r9) (ref r10)))) (value none) (body semicolon)))))
)
~~~
