# META
~~~sexpr
(snapshot (type recovery) (description "Collection operator BodyExpressions retain typed parameters, result expressions, reference provenance, and malformed-body recovery. A parameter may be undirected, a body may open with documentation, and a parameter may be terminated by its own documented brace body instead of a semicolon."))
~~~
# SOURCE
~~~sysml
package Collections {
    private import Filtered[items->forAll {
        in ref item : Domain::Item;
        item == selected.item
    }];

    private import Transformed[items->collect { in item; item.value }];

    private import Broken[items->select {
        in item : Domain::Item
        item == selected
    }];

    private import After[items->exists { in candidate; candidate != null }];

    private import Undirected[vertices->forAll { p1 : Domain::Point; vertices->exists { p2 : Domain::Point; p1 != p2 } }];

    private import Documented[alternatives->minimize { doc /* The minimum value. */ value }];

    private import ParameterBody[alternatives->selectOne { in ref a { doc /* The selected one. */ } a }];
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "collection_operator_bodies.md"
    (diagnostics
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 218) (line 9) (column 5) (len 101)) (message "unexpected token in package body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Collections {
    private import Filtered [items->forAll { in ref item : Domain::Item; item == selected.item }];
    private import Transformed [items->collect { in item; item.value }];
    private import Broken[items->select {
        in item : Domain::Item
        item == selected
    }];
    private import After [items->exists { in candidate; candidate != null }];
    private import Undirected [vertices->forAll { p1 : Domain::Point; vertices->exists { p2 : Domain::Point; p1 != p2 } }];
    private import Documented [alternatives->minimize { doc
    /* The minimum value. */ value }];
    private import ParameterBody [alternatives->selectOne { in ref a { doc
    /* The selected one. */ } a }];
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 41) (line 2) (column 20) (len 8)) (segments (segment 0 (token "Filtered") (name "Filtered") (separator none) (span (offset 41) (line 2) (column 20) (len 8)))))
    (reference r1 (scope relative) (span (offset 50) (line 2) (column 29) (len 5)) (segments (segment 0 (token "items") (name "items") (separator none) (span (offset 50) (line 2) (column 29) (len 5)))))
    (reference r2 (scope relative) (span (offset 88) (line 3) (column 23) (len 12)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 88) (line 3) (column 23) (len 6))) (segment 1 (token "Item") (name "Item") (separator colon-colon) (span (offset 96) (line 3) (column 31) (len 4)))))
    (reference r3 (scope relative) (span (offset 110) (line 4) (column 9) (len 4)) (segments (segment 0 (token "item") (name "item") (separator none) (span (offset 110) (line 4) (column 9) (len 4)))))
    (reference r4 (scope relative) (span (offset 118) (line 4) (column 17) (len 8)) (segments (segment 0 (token "selected") (name "selected") (separator none) (span (offset 118) (line 4) (column 17) (len 8)))))
    (reference r5 (scope relative) (span (offset 127) (line 4) (column 26) (len 4)) (segments (segment 0 (token "item") (name "item") (separator none) (span (offset 127) (line 4) (column 26) (len 4)))))
    (reference r6 (scope relative) (span (offset 160) (line 7) (column 20) (len 11)) (segments (segment 0 (token "Transformed") (name "Transformed") (separator none) (span (offset 160) (line 7) (column 20) (len 11)))))
    (reference r7 (scope relative) (span (offset 172) (line 7) (column 32) (len 5)) (segments (segment 0 (token "items") (name "items") (separator none) (span (offset 172) (line 7) (column 32) (len 5)))))
    (reference r8 (scope relative) (span (offset 198) (line 7) (column 58) (len 4)) (segments (segment 0 (token "item") (name "item") (separator none) (span (offset 198) (line 7) (column 58) (len 4)))))
    (reference r9 (scope relative) (span (offset 203) (line 7) (column 63) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 203) (line 7) (column 63) (len 5)))))
    (reference r10 (scope relative) (span (offset 340) (line 14) (column 20) (len 5)) (segments (segment 0 (token "After") (name "After") (separator none) (span (offset 340) (line 14) (column 20) (len 5)))))
    (reference r11 (scope relative) (span (offset 346) (line 14) (column 26) (len 5)) (segments (segment 0 (token "items") (name "items") (separator none) (span (offset 346) (line 14) (column 26) (len 5)))))
    (reference r12 (scope relative) (span (offset 376) (line 14) (column 56) (len 9)) (segments (segment 0 (token "candidate") (name "candidate") (separator none) (span (offset 376) (line 14) (column 56) (len 9)))))
    (reference r13 (scope relative) (span (offset 418) (line 16) (column 20) (len 10)) (segments (segment 0 (token "Undirected") (name "Undirected") (separator none) (span (offset 418) (line 16) (column 20) (len 10)))))
    (reference r14 (scope relative) (span (offset 429) (line 16) (column 31) (len 8)) (segments (segment 0 (token "vertices") (name "vertices") (separator none) (span (offset 429) (line 16) (column 31) (len 8)))))
    (reference r15 (scope relative) (span (offset 453) (line 16) (column 55) (len 13)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 453) (line 16) (column 55) (len 6))) (segment 1 (token "Point") (name "Point") (separator colon-colon) (span (offset 461) (line 16) (column 63) (len 5)))))
    (reference r16 (scope relative) (span (offset 468) (line 16) (column 70) (len 8)) (segments (segment 0 (token "vertices") (name "vertices") (separator none) (span (offset 468) (line 16) (column 70) (len 8)))))
    (reference r17 (scope relative) (span (offset 492) (line 16) (column 94) (len 13)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 492) (line 16) (column 94) (len 6))) (segment 1 (token "Point") (name "Point") (separator colon-colon) (span (offset 500) (line 16) (column 102) (len 5)))))
    (reference r18 (scope relative) (span (offset 507) (line 16) (column 109) (len 2)) (segments (segment 0 (token "p1") (name "p1") (separator none) (span (offset 507) (line 16) (column 109) (len 2)))))
    (reference r19 (scope relative) (span (offset 513) (line 16) (column 115) (len 2)) (segments (segment 0 (token "p2") (name "p2") (separator none) (span (offset 513) (line 16) (column 115) (len 2)))))
    (reference r20 (scope relative) (span (offset 542) (line 18) (column 20) (len 10)) (segments (segment 0 (token "Documented") (name "Documented") (separator none) (span (offset 542) (line 18) (column 20) (len 10)))))
    (reference r21 (scope relative) (span (offset 553) (line 18) (column 31) (len 12)) (segments (segment 0 (token "alternatives") (name "alternatives") (separator none) (span (offset 553) (line 18) (column 31) (len 12)))))
    (reference r22 (scope relative) (span (offset 607) (line 18) (column 85) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 607) (line 18) (column 85) (len 5)))))
    (reference r23 (scope relative) (span (offset 637) (line 20) (column 20) (len 13)) (segments (segment 0 (token "ParameterBody") (name "ParameterBody") (separator none) (span (offset 637) (line 20) (column 20) (len 13)))))
    (reference r24 (scope relative) (span (offset 651) (line 20) (column 34) (len 12)) (segments (segment 0 (token "alternatives") (name "alternatives") (separator none) (span (offset 651) (line 20) (column 34) (len 12)))))
    (reference r25 (scope relative) (span (offset 718) (line 20) (column 101) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 718) (line 20) (column 101) (len 1)))))
  )
  (root (package (name "Collections") (body brace (import (target (span (span (offset 41) (line 2) (column 20) (len 97))) (all none) (ref r0) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 49) (line 2) (column 28) (len 89))) (open (span (offset 49) (line 2) (column 28) (len 1))) (expression (expression (span (offset 50) (line 2) (column 29) (len 87)) (collection-op (operator "forAll") (base (expression (span (offset 50) (line 2) (column 29) (len 5)) (ref r1))) (arguments) (brace-body (body (span (offset 64) (line 2) (column 43) (len 73)) (open-brace (span (offset 64) (line 2) (column 43) (len 1))) (parameters (parameter (span (offset 74) (line 3) (column 9) (len 27)) (direction in (span (offset 74) (line 3) (column 9) (len 2))) (reference-keyword (span (offset 77) (line 3) (column 12) (len 3))) (declaration (name "item") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 100) (line 3) (column 35) (len 1)))))) (result (expression (span (offset 110) (line 4) (column 9) (len 21)) (binary (operator "==") (left (expression (span (offset 110) (line 4) (column 9) (len 4)) (ref r3))) (right (expression (span (offset 118) (line 4) (column 17) (len 13)) (member-access (base (expression (span (offset 118) (line 4) (column 17) (len 8)) (ref r4))) (separator dot) (member (ref r5)))))))) (close-brace (span (offset 136) (line 5) (column 5) (len 1)))))))) (close (span (offset 137) (line 5) (column 6) (len 1))))))))) (import (target (span (span (offset 160) (line 7) (column 20) (len 51))) (all none) (ref r6) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 171) (line 7) (column 31) (len 40))) (open (span (offset 171) (line 7) (column 31) (len 1))) (expression (expression (span (offset 172) (line 7) (column 32) (len 38)) (collection-op (operator "collect") (base (expression (span (offset 172) (line 7) (column 32) (len 5)) (ref r7))) (arguments) (brace-body (body (span (offset 187) (line 7) (column 47) (len 23)) (open-brace (span (offset 187) (line 7) (column 47) (len 1))) (parameters (parameter (span (offset 189) (line 7) (column 49) (len 8)) (direction in (span (offset 189) (line 7) (column 49) (len 2))) (reference-keyword none) (declaration (name "item") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 196) (line 7) (column 56) (len 1)))))) (result (expression (span (offset 198) (line 7) (column 58) (len 10)) (member-access (base (expression (span (offset 198) (line 7) (column 58) (len 4)) (ref r8))) (separator dot) (member (ref r9))))) (close-brace (span (offset 209) (line 7) (column 69) (len 1)))))))) (close (span (offset 210) (line 7) (column 70) (len 1))))))))) (malformed (code "recovered_package_body_element") (found "private import Broken[items->select {") (span (offset 218) (line 9) (column 5) (len 101))) (import (target (span (span (offset 340) (line 14) (column 20) (len 56))) (all none) (ref r10) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 345) (line 14) (column 25) (len 51))) (open (span (offset 345) (line 14) (column 25) (len 1))) (expression (expression (span (offset 346) (line 14) (column 26) (len 49)) (collection-op (operator "exists") (base (expression (span (offset 346) (line 14) (column 26) (len 5)) (ref r11))) (arguments) (brace-body (body (span (offset 360) (line 14) (column 40) (len 35)) (open-brace (span (offset 360) (line 14) (column 40) (len 1))) (parameters (parameter (span (offset 362) (line 14) (column 42) (len 13)) (direction in (span (offset 362) (line 14) (column 42) (len 2))) (reference-keyword none) (declaration (name "candidate") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 374) (line 14) (column 54) (len 1)))))) (result (expression (span (offset 376) (line 14) (column 56) (len 17)) (binary (operator "!=") (left (expression (span (offset 376) (line 14) (column 56) (len 9)) (ref r12))) (right (expression (span (offset 389) (line 14) (column 69) (len 4)) (null)))))) (close-brace (span (offset 394) (line 14) (column 74) (len 1)))))))) (close (span (offset 395) (line 14) (column 75) (len 1))))))))) (import (target (span (span (offset 418) (line 16) (column 20) (len 102))) (all none) (ref r13) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 428) (line 16) (column 30) (len 92))) (open (span (offset 428) (line 16) (column 30) (len 1))) (expression (expression (span (offset 429) (line 16) (column 31) (len 90)) (collection-op (operator "forAll") (base (expression (span (offset 429) (line 16) (column 31) (len 8)) (ref r14))) (arguments) (brace-body (body (span (offset 446) (line 16) (column 48) (len 73)) (open-brace (span (offset 446) (line 16) (column 48) (len 1))) (parameters (parameter (span (offset 448) (line 16) (column 50) (len 19)) (direction none) (reference-keyword none) (declaration (name "p1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 466) (line 16) (column 68) (len 1)))))) (result (expression (span (offset 468) (line 16) (column 70) (len 49)) (collection-op (operator "exists") (base (expression (span (offset 468) (line 16) (column 70) (len 8)) (ref r16))) (arguments) (brace-body (body (span (offset 485) (line 16) (column 87) (len 32)) (open-brace (span (offset 485) (line 16) (column 87) (len 1))) (parameters (parameter (span (offset 487) (line 16) (column 89) (len 19)) (direction none) (reference-keyword none) (declaration (name "p2") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 505) (line 16) (column 107) (len 1)))))) (result (expression (span (offset 507) (line 16) (column 109) (len 8)) (binary (operator "!=") (left (expression (span (offset 507) (line 16) (column 109) (len 2)) (ref r18))) (right (expression (span (offset 513) (line 16) (column 115) (len 2)) (ref r19)))))) (close-brace (span (offset 516) (line 16) (column 118) (len 1)))))))) (close-brace (span (offset 518) (line 16) (column 120) (len 1)))))))) (close (span (offset 519) (line 16) (column 121) (len 1))))))))) (import (target (span (span (offset 542) (line 18) (column 20) (len 73))) (all none) (ref r20) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 552) (line 18) (column 30) (len 63))) (open (span (offset 552) (line 18) (column 30) (len 1))) (expression (expression (span (offset 553) (line 18) (column 31) (len 61)) (collection-op (operator "minimize") (base (expression (span (offset 553) (line 18) (column 31) (len 12)) (ref r21))) (arguments) (brace-body (body (span (offset 576) (line 18) (column 54) (len 38)) (open-brace (span (offset 576) (line 18) (column 54) (len 1))) (parameters) (result (expression (span (offset 607) (line 18) (column 85) (len 5)) (ref r22))) (close-brace (span (offset 613) (line 18) (column 91) (len 1)))))))) (close (span (offset 614) (line 18) (column 92) (len 1))))))))) (import (target (span (span (offset 637) (line 20) (column 20) (len 85))) (all none) (ref r23) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 650) (line 20) (column 33) (len 72))) (open (span (offset 650) (line 20) (column 33) (len 1))) (expression (expression (span (offset 651) (line 20) (column 34) (len 70)) (collection-op (operator "selectOne") (base (expression (span (offset 651) (line 20) (column 34) (len 12)) (ref r24))) (arguments) (brace-body (body (span (offset 675) (line 20) (column 58) (len 46)) (open-brace (span (offset 675) (line 20) (column 58) (len 1))) (parameters (parameter (span (offset 677) (line 20) (column 60) (len 40)) (direction in (span (offset 677) (line 20) (column 60) (len 2))) (reference-keyword (span (offset 680) (line 20) (column 63) (len 3))) (declaration (name "a") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (body (open-brace (span (offset 686) (line 20) (column 69) (len 1))) (doc present) (close-brace (span (offset 716) (line 20) (column 99) (len 1))))))) (result (expression (span (offset 718) (line 20) (column 101) (len 1)) (ref r25))) (close-brace (span (offset 720) (line 20) (column 103) (len 1)))))))) (close (span (offset 721) (line 20) (column 104) (len 1))))))))))))
)
~~~
