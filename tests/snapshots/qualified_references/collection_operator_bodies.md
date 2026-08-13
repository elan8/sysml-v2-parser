# META
~~~sexpr
(snapshot (type recovery) (description "Collection operator BodyExpressions retain typed parameters, result expressions, reference provenance, and malformed-body recovery."))
~~~
# SOURCE
~~~sysml
package Collections {
    import Filtered[items->forAll {
        in ref item : Domain::Item;
        item == selected.item
    }];

    import Transformed[items->collect { in item; item.value }];

    import Broken[items->select {
        in item : Domain::Item
        item == selected
    }];

    import After[items->exists { in candidate; candidate != null }];
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "collection_operator_bodies.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 202) (line 9) (column 5) (len 99)) (message "unexpected token in package body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Collections {
    import Filtered [items->forAll { in ref item : Domain::Item; 'item' == selected.'item' }];
    import Transformed [items->collect { in item; 'item'.value }];
    import Broken[items->select {
        in item : Domain::Item
        item == selected
    }];
    import After [items->exists { in candidate; candidate != null }];
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 33) (line 2) (column 12) (len 8)) (segments (segment 0 (token "Filtered") (name "Filtered") (separator none) (span (offset 33) (line 2) (column 12) (len 8)))))
    (reference r1 (scope relative) (span (offset 42) (line 2) (column 21) (len 5)) (segments (segment 0 (token "items") (name "items") (separator none) (span (offset 42) (line 2) (column 21) (len 5)))))
    (reference r2 (scope relative) (span (offset 80) (line 3) (column 23) (len 12)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 80) (line 3) (column 23) (len 6))) (segment 1 (token "Item") (name "Item") (separator colon-colon) (span (offset 88) (line 3) (column 31) (len 4)))))
    (reference r3 (scope relative) (span (offset 102) (line 4) (column 9) (len 4)) (segments (segment 0 (token "item") (name "item") (separator none) (span (offset 102) (line 4) (column 9) (len 4)))))
    (reference r4 (scope relative) (span (offset 110) (line 4) (column 17) (len 8)) (segments (segment 0 (token "selected") (name "selected") (separator none) (span (offset 110) (line 4) (column 17) (len 8)))))
    (reference r5 (scope relative) (span (offset 119) (line 4) (column 26) (len 4)) (segments (segment 0 (token "item") (name "item") (separator none) (span (offset 119) (line 4) (column 26) (len 4)))))
    (reference r6 (scope relative) (span (offset 144) (line 7) (column 12) (len 11)) (segments (segment 0 (token "Transformed") (name "Transformed") (separator none) (span (offset 144) (line 7) (column 12) (len 11)))))
    (reference r7 (scope relative) (span (offset 156) (line 7) (column 24) (len 5)) (segments (segment 0 (token "items") (name "items") (separator none) (span (offset 156) (line 7) (column 24) (len 5)))))
    (reference r8 (scope relative) (span (offset 182) (line 7) (column 50) (len 4)) (segments (segment 0 (token "item") (name "item") (separator none) (span (offset 182) (line 7) (column 50) (len 4)))))
    (reference r9 (scope relative) (span (offset 187) (line 7) (column 55) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 187) (line 7) (column 55) (len 5)))))
    (reference r10 (scope relative) (span (offset 308) (line 14) (column 12) (len 5)) (segments (segment 0 (token "After") (name "After") (separator none) (span (offset 308) (line 14) (column 12) (len 5)))))
    (reference r11 (scope relative) (span (offset 314) (line 14) (column 18) (len 5)) (segments (segment 0 (token "items") (name "items") (separator none) (span (offset 314) (line 14) (column 18) (len 5)))))
    (reference r12 (scope relative) (span (offset 344) (line 14) (column 48) (len 9)) (segments (segment 0 (token "candidate") (name "candidate") (separator none) (span (offset 344) (line 14) (column 48) (len 9)))))
  )
  (root (package (name "Collections") (body (import (target (span (span (offset 33) (line 2) (column 12) (len 97))) (all none) (ref r0) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 41) (line 2) (column 20) (len 89))) (open (span (offset 41) (line 2) (column 20) (len 1))) (expression (expression (span (offset 42) (line 2) (column 21) (len 87)) (collection-op (operator "forAll") (base (expression (span (offset 42) (line 2) (column 21) (len 5)) (ref r1))) (arguments) (brace-body (body (span (offset 56) (line 2) (column 35) (len 73)) (open-brace (span (offset 56) (line 2) (column 35) (len 1))) (parameters (parameter (span (offset 66) (line 3) (column 9) (len 27)) (direction in (span (offset 66) (line 3) (column 9) (len 2))) (reference-keyword (span (offset 69) (line 3) (column 12) (len 3))) (name "item" (span (offset 73) (line 3) (column 16) (len 4))) (typing (typed (separator (span (offset 78) (line 3) (column 21) (len 1))) (target (ref r2)))) (semicolon (span (offset 92) (line 3) (column 35) (len 1))))) (result (expression (span (offset 102) (line 4) (column 9) (len 21)) (binary (operator "==") (left (expression (span (offset 102) (line 4) (column 9) (len 4)) (ref r3))) (right (expression (span (offset 110) (line 4) (column 17) (len 13)) (member-access (base (expression (span (offset 110) (line 4) (column 17) (len 8)) (ref r4))) (separator dot) (member (ref r5)))))))) (close-brace (span (offset 128) (line 5) (column 5) (len 1)))))))) (close (span (offset 129) (line 5) (column 6) (len 1))))))))) (import (target (span (span (offset 144) (line 7) (column 12) (len 51))) (all none) (ref r6) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 155) (line 7) (column 23) (len 40))) (open (span (offset 155) (line 7) (column 23) (len 1))) (expression (expression (span (offset 156) (line 7) (column 24) (len 38)) (collection-op (operator "collect") (base (expression (span (offset 156) (line 7) (column 24) (len 5)) (ref r7))) (arguments) (brace-body (body (span (offset 171) (line 7) (column 39) (len 23)) (open-brace (span (offset 171) (line 7) (column 39) (len 1))) (parameters (parameter (span (offset 173) (line 7) (column 41) (len 8)) (direction in (span (offset 173) (line 7) (column 41) (len 2))) (reference-keyword none) (name "item" (span (offset 176) (line 7) (column 44) (len 4))) (typing none) (semicolon (span (offset 180) (line 7) (column 48) (len 1))))) (result (expression (span (offset 182) (line 7) (column 50) (len 10)) (member-access (base (expression (span (offset 182) (line 7) (column 50) (len 4)) (ref r8))) (separator dot) (member (ref r9))))) (close-brace (span (offset 193) (line 7) (column 61) (len 1)))))))) (close (span (offset 194) (line 7) (column 62) (len 1))))))))) (malformed (code "recovered_package_body_element") (found "import Broken[items->select {") (span (offset 202) (line 9) (column 5) (len 99))) (import (target (span (span (offset 308) (line 14) (column 12) (len 56))) (all none) (ref r10) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 313) (line 14) (column 17) (len 51))) (open (span (offset 313) (line 14) (column 17) (len 1))) (expression (expression (span (offset 314) (line 14) (column 18) (len 49)) (collection-op (operator "exists") (base (expression (span (offset 314) (line 14) (column 18) (len 5)) (ref r11))) (arguments) (brace-body (body (span (offset 328) (line 14) (column 32) (len 35)) (open-brace (span (offset 328) (line 14) (column 32) (len 1))) (parameters (parameter (span (offset 330) (line 14) (column 34) (len 13)) (direction in (span (offset 330) (line 14) (column 34) (len 2))) (reference-keyword none) (name "candidate" (span (offset 333) (line 14) (column 37) (len 9))) (typing none) (semicolon (span (offset 342) (line 14) (column 46) (len 1))))) (result (expression (span (offset 344) (line 14) (column 48) (len 17)) (binary (operator "!=") (left (expression (span (offset 344) (line 14) (column 48) (len 9)) (ref r12))) (right (expression (span (offset 357) (line 14) (column 61) (len 4)) (null)))))) (close-brace (span (offset 362) (line 14) (column 66) (len 1)))))))) (close (span (offset 363) (line 14) (column 67) (len 1))))))))))))
)
~~~
