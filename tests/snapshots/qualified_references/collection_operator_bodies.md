# META
~~~sexpr
(snapshot (type recovery) (description "Collection operator BodyExpressions retain typed parameters, result expressions, reference provenance, and malformed-body recovery. A parameter may be undirected, a body may open with documentation, and a parameter may be terminated by its own documented brace body instead of a semicolon."))
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

    import Undirected[vertices->forAll { p1 : Domain::Point; vertices->exists { p2 : Domain::Point; p1 != p2 } }];

    import Documented[alternatives->minimize { doc /* The minimum value. */ value }];

    import ParameterBody[alternatives->selectOne { in ref a { doc /* The selected one. */ } a }];
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
    import Filtered [items->forAll { in ref item : Domain::Item; item == selected.item }];
    import Transformed [items->collect { in item; item.value }];
    import Broken[items->select {
        in item : Domain::Item
        item == selected
    }];
    import After [items->exists { in candidate; candidate != null }];
    import Undirected [vertices->forAll { p1 : Domain::Point; vertices->exists { p2 : Domain::Point; p1 != p2 } }];
    import Documented [alternatives->minimize { doc
    /* The minimum value. */ value }];
    import ParameterBody [alternatives->selectOne { in ref a { doc
    /* The selected one. */ } a }];
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
    (reference r13 (scope relative) (span (offset 378) (line 16) (column 12) (len 10)) (segments (segment 0 (token "Undirected") (name "Undirected") (separator none) (span (offset 378) (line 16) (column 12) (len 10)))))
    (reference r14 (scope relative) (span (offset 389) (line 16) (column 23) (len 8)) (segments (segment 0 (token "vertices") (name "vertices") (separator none) (span (offset 389) (line 16) (column 23) (len 8)))))
    (reference r15 (scope relative) (span (offset 413) (line 16) (column 47) (len 13)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 413) (line 16) (column 47) (len 6))) (segment 1 (token "Point") (name "Point") (separator colon-colon) (span (offset 421) (line 16) (column 55) (len 5)))))
    (reference r16 (scope relative) (span (offset 428) (line 16) (column 62) (len 8)) (segments (segment 0 (token "vertices") (name "vertices") (separator none) (span (offset 428) (line 16) (column 62) (len 8)))))
    (reference r17 (scope relative) (span (offset 452) (line 16) (column 86) (len 13)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 452) (line 16) (column 86) (len 6))) (segment 1 (token "Point") (name "Point") (separator colon-colon) (span (offset 460) (line 16) (column 94) (len 5)))))
    (reference r18 (scope relative) (span (offset 467) (line 16) (column 101) (len 2)) (segments (segment 0 (token "p1") (name "p1") (separator none) (span (offset 467) (line 16) (column 101) (len 2)))))
    (reference r19 (scope relative) (span (offset 473) (line 16) (column 107) (len 2)) (segments (segment 0 (token "p2") (name "p2") (separator none) (span (offset 473) (line 16) (column 107) (len 2)))))
    (reference r20 (scope relative) (span (offset 494) (line 18) (column 12) (len 10)) (segments (segment 0 (token "Documented") (name "Documented") (separator none) (span (offset 494) (line 18) (column 12) (len 10)))))
    (reference r21 (scope relative) (span (offset 505) (line 18) (column 23) (len 12)) (segments (segment 0 (token "alternatives") (name "alternatives") (separator none) (span (offset 505) (line 18) (column 23) (len 12)))))
    (reference r22 (scope relative) (span (offset 559) (line 18) (column 77) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 559) (line 18) (column 77) (len 5)))))
    (reference r23 (scope relative) (span (offset 581) (line 20) (column 12) (len 13)) (segments (segment 0 (token "ParameterBody") (name "ParameterBody") (separator none) (span (offset 581) (line 20) (column 12) (len 13)))))
    (reference r24 (scope relative) (span (offset 595) (line 20) (column 26) (len 12)) (segments (segment 0 (token "alternatives") (name "alternatives") (separator none) (span (offset 595) (line 20) (column 26) (len 12)))))
    (reference r25 (scope relative) (span (offset 662) (line 20) (column 93) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 662) (line 20) (column 93) (len 1)))))
  )
  (root (package (name "Collections") (body brace (import (target (span (span (offset 33) (line 2) (column 12) (len 97))) (all none) (ref r0) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 41) (line 2) (column 20) (len 89))) (open (span (offset 41) (line 2) (column 20) (len 1))) (expression (expression (span (offset 42) (line 2) (column 21) (len 87)) (collection-op (operator "forAll") (base (expression (span (offset 42) (line 2) (column 21) (len 5)) (ref r1))) (arguments) (brace-body (body (span (offset 56) (line 2) (column 35) (len 73)) (open-brace (span (offset 56) (line 2) (column 35) (len 1))) (parameters (parameter (span (offset 66) (line 3) (column 9) (len 27)) (direction in (span (offset 66) (line 3) (column 9) (len 2))) (reference-keyword (span (offset 69) (line 3) (column 12) (len 3))) (declaration (name "item") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 92) (line 3) (column 35) (len 1)))))) (result (expression (span (offset 102) (line 4) (column 9) (len 21)) (binary (operator "==") (left (expression (span (offset 102) (line 4) (column 9) (len 4)) (ref r3))) (right (expression (span (offset 110) (line 4) (column 17) (len 13)) (member-access (base (expression (span (offset 110) (line 4) (column 17) (len 8)) (ref r4))) (separator dot) (member (ref r5)))))))) (close-brace (span (offset 128) (line 5) (column 5) (len 1)))))))) (close (span (offset 129) (line 5) (column 6) (len 1))))))))) (import (target (span (span (offset 144) (line 7) (column 12) (len 51))) (all none) (ref r6) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 155) (line 7) (column 23) (len 40))) (open (span (offset 155) (line 7) (column 23) (len 1))) (expression (expression (span (offset 156) (line 7) (column 24) (len 38)) (collection-op (operator "collect") (base (expression (span (offset 156) (line 7) (column 24) (len 5)) (ref r7))) (arguments) (brace-body (body (span (offset 171) (line 7) (column 39) (len 23)) (open-brace (span (offset 171) (line 7) (column 39) (len 1))) (parameters (parameter (span (offset 173) (line 7) (column 41) (len 8)) (direction in (span (offset 173) (line 7) (column 41) (len 2))) (reference-keyword none) (declaration (name "item") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 180) (line 7) (column 48) (len 1)))))) (result (expression (span (offset 182) (line 7) (column 50) (len 10)) (member-access (base (expression (span (offset 182) (line 7) (column 50) (len 4)) (ref r8))) (separator dot) (member (ref r9))))) (close-brace (span (offset 193) (line 7) (column 61) (len 1)))))))) (close (span (offset 194) (line 7) (column 62) (len 1))))))))) (malformed (code "recovered_package_body_element") (found "import Broken[items->select {") (span (offset 202) (line 9) (column 5) (len 99))) (import (target (span (span (offset 308) (line 14) (column 12) (len 56))) (all none) (ref r10) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 313) (line 14) (column 17) (len 51))) (open (span (offset 313) (line 14) (column 17) (len 1))) (expression (expression (span (offset 314) (line 14) (column 18) (len 49)) (collection-op (operator "exists") (base (expression (span (offset 314) (line 14) (column 18) (len 5)) (ref r11))) (arguments) (brace-body (body (span (offset 328) (line 14) (column 32) (len 35)) (open-brace (span (offset 328) (line 14) (column 32) (len 1))) (parameters (parameter (span (offset 330) (line 14) (column 34) (len 13)) (direction in (span (offset 330) (line 14) (column 34) (len 2))) (reference-keyword none) (declaration (name "candidate") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 342) (line 14) (column 46) (len 1)))))) (result (expression (span (offset 344) (line 14) (column 48) (len 17)) (binary (operator "!=") (left (expression (span (offset 344) (line 14) (column 48) (len 9)) (ref r12))) (right (expression (span (offset 357) (line 14) (column 61) (len 4)) (null)))))) (close-brace (span (offset 362) (line 14) (column 66) (len 1)))))))) (close (span (offset 363) (line 14) (column 67) (len 1))))))))) (import (target (span (span (offset 378) (line 16) (column 12) (len 102))) (all none) (ref r13) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 388) (line 16) (column 22) (len 92))) (open (span (offset 388) (line 16) (column 22) (len 1))) (expression (expression (span (offset 389) (line 16) (column 23) (len 90)) (collection-op (operator "forAll") (base (expression (span (offset 389) (line 16) (column 23) (len 8)) (ref r14))) (arguments) (brace-body (body (span (offset 406) (line 16) (column 40) (len 73)) (open-brace (span (offset 406) (line 16) (column 40) (len 1))) (parameters (parameter (span (offset 408) (line 16) (column 42) (len 19)) (direction none) (reference-keyword none) (declaration (name "p1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 426) (line 16) (column 60) (len 1)))))) (result (expression (span (offset 428) (line 16) (column 62) (len 49)) (collection-op (operator "exists") (base (expression (span (offset 428) (line 16) (column 62) (len 8)) (ref r16))) (arguments) (brace-body (body (span (offset 445) (line 16) (column 79) (len 32)) (open-brace (span (offset 445) (line 16) (column 79) (len 1))) (parameters (parameter (span (offset 447) (line 16) (column 81) (len 19)) (direction none) (reference-keyword none) (declaration (name "p2") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 465) (line 16) (column 99) (len 1)))))) (result (expression (span (offset 467) (line 16) (column 101) (len 8)) (binary (operator "!=") (left (expression (span (offset 467) (line 16) (column 101) (len 2)) (ref r18))) (right (expression (span (offset 473) (line 16) (column 107) (len 2)) (ref r19)))))) (close-brace (span (offset 476) (line 16) (column 110) (len 1)))))))) (close-brace (span (offset 478) (line 16) (column 112) (len 1)))))))) (close (span (offset 479) (line 16) (column 113) (len 1))))))))) (import (target (span (span (offset 494) (line 18) (column 12) (len 73))) (all none) (ref r20) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 504) (line 18) (column 22) (len 63))) (open (span (offset 504) (line 18) (column 22) (len 1))) (expression (expression (span (offset 505) (line 18) (column 23) (len 61)) (collection-op (operator "minimize") (base (expression (span (offset 505) (line 18) (column 23) (len 12)) (ref r21))) (arguments) (brace-body (body (span (offset 528) (line 18) (column 46) (len 38)) (open-brace (span (offset 528) (line 18) (column 46) (len 1))) (parameters) (result (expression (span (offset 559) (line 18) (column 77) (len 5)) (ref r22))) (close-brace (span (offset 565) (line 18) (column 83) (len 1)))))))) (close (span (offset 566) (line 18) (column 84) (len 1))))))))) (import (target (span (span (offset 581) (line 20) (column 12) (len 85))) (all none) (ref r23) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 594) (line 20) (column 25) (len 72))) (open (span (offset 594) (line 20) (column 25) (len 1))) (expression (expression (span (offset 595) (line 20) (column 26) (len 70)) (collection-op (operator "selectOne") (base (expression (span (offset 595) (line 20) (column 26) (len 12)) (ref r24))) (arguments) (brace-body (body (span (offset 619) (line 20) (column 50) (len 46)) (open-brace (span (offset 619) (line 20) (column 50) (len 1))) (parameters (parameter (span (offset 621) (line 20) (column 52) (len 40)) (direction in (span (offset 621) (line 20) (column 52) (len 2))) (reference-keyword (span (offset 624) (line 20) (column 55) (len 3))) (declaration (name "a") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (body (open-brace (span (offset 630) (line 20) (column 61) (len 1))) (doc present) (close-brace (span (offset 660) (line 20) (column 91) (len 1))))))) (result (expression (span (offset 662) (line 20) (column 93) (len 1)) (ref r25))) (close-brace (span (offset 664) (line 20) (column 95) (len 1)))))))) (close (span (offset 665) (line 20) (column 96) (len 1))))))))))))
)
~~~
