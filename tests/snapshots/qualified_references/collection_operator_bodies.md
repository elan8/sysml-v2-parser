# META
~~~sexpr
(snapshot (type recovery) (description "Collection operator BodyExpressions retain typed parameters, result expressions, reference provenance, and malformed-body recovery. A parameter may be undirected, a body may open with documentation, and a parameter may be terminated by its own documented brace body instead of a semicolon."))
~~~
# SOURCE
~~~sysml
package Collections {
    private import Filtered[items->forAll {
        in ref 'item' : Domain::Item;
        item == selected.item
    }];

    private import Transformed[items->collect { in 'item'; item.value }];

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
      (diagnostic (code "recovered_package_body_element") (severity error) (category parseerror) (span (offset 222) (line 9) (column 5) (len 101)) (message "unexpected token in package body"))
    )
  )
)
~~~
# FORMAT
~~~sysml
package Collections {
    private import Filtered [items->forAll { in ref 'item' : Domain::Item; item == selected.item }];
    private import Transformed [items->collect { in 'item'; item.value }];
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
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 41) (line 2) (column 20) (len 8)) (segments (segment 0 (token "Filtered") (name "Filtered") (separator none) (span (offset 41) (line 2) (column 20) (len 8)))))
    (reference r1 (scope relative) (span (offset 50) (line 2) (column 29) (len 5)) (segments (segment 0 (token "items") (name "items") (separator none) (span (offset 50) (line 2) (column 29) (len 5)))))
    (reference r2 (scope relative) (span (offset 90) (line 3) (column 25) (len 12)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 90) (line 3) (column 25) (len 6))) (segment 1 (token "Item") (name "Item") (separator colon-colon) (span (offset 98) (line 3) (column 33) (len 4)))))
    (reference r3 (scope relative) (span (offset 112) (line 4) (column 9) (len 4)) (segments (segment 0 (token "item") (name "item") (separator none) (span (offset 112) (line 4) (column 9) (len 4)))))
    (reference r4 (scope relative) (span (offset 120) (line 4) (column 17) (len 8)) (segments (segment 0 (token "selected") (name "selected") (separator none) (span (offset 120) (line 4) (column 17) (len 8)))))
    (reference r5 (scope relative) (span (offset 129) (line 4) (column 26) (len 4)) (segments (segment 0 (token "item") (name "item") (separator none) (span (offset 129) (line 4) (column 26) (len 4)))))
    (reference r6 (scope relative) (span (offset 162) (line 7) (column 20) (len 11)) (segments (segment 0 (token "Transformed") (name "Transformed") (separator none) (span (offset 162) (line 7) (column 20) (len 11)))))
    (reference r7 (scope relative) (span (offset 174) (line 7) (column 32) (len 5)) (segments (segment 0 (token "items") (name "items") (separator none) (span (offset 174) (line 7) (column 32) (len 5)))))
    (reference r8 (scope relative) (span (offset 202) (line 7) (column 60) (len 4)) (segments (segment 0 (token "item") (name "item") (separator none) (span (offset 202) (line 7) (column 60) (len 4)))))
    (reference r9 (scope relative) (span (offset 207) (line 7) (column 65) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 207) (line 7) (column 65) (len 5)))))
    (reference r10 (scope relative) (span (offset 344) (line 14) (column 20) (len 5)) (segments (segment 0 (token "After") (name "After") (separator none) (span (offset 344) (line 14) (column 20) (len 5)))))
    (reference r11 (scope relative) (span (offset 350) (line 14) (column 26) (len 5)) (segments (segment 0 (token "items") (name "items") (separator none) (span (offset 350) (line 14) (column 26) (len 5)))))
    (reference r12 (scope relative) (span (offset 380) (line 14) (column 56) (len 9)) (segments (segment 0 (token "candidate") (name "candidate") (separator none) (span (offset 380) (line 14) (column 56) (len 9)))))
    (reference r13 (scope relative) (span (offset 422) (line 16) (column 20) (len 10)) (segments (segment 0 (token "Undirected") (name "Undirected") (separator none) (span (offset 422) (line 16) (column 20) (len 10)))))
    (reference r14 (scope relative) (span (offset 433) (line 16) (column 31) (len 8)) (segments (segment 0 (token "vertices") (name "vertices") (separator none) (span (offset 433) (line 16) (column 31) (len 8)))))
    (reference r15 (scope relative) (span (offset 457) (line 16) (column 55) (len 13)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 457) (line 16) (column 55) (len 6))) (segment 1 (token "Point") (name "Point") (separator colon-colon) (span (offset 465) (line 16) (column 63) (len 5)))))
    (reference r16 (scope relative) (span (offset 472) (line 16) (column 70) (len 8)) (segments (segment 0 (token "vertices") (name "vertices") (separator none) (span (offset 472) (line 16) (column 70) (len 8)))))
    (reference r17 (scope relative) (span (offset 496) (line 16) (column 94) (len 13)) (segments (segment 0 (token "Domain") (name "Domain") (separator none) (span (offset 496) (line 16) (column 94) (len 6))) (segment 1 (token "Point") (name "Point") (separator colon-colon) (span (offset 504) (line 16) (column 102) (len 5)))))
    (reference r18 (scope relative) (span (offset 511) (line 16) (column 109) (len 2)) (segments (segment 0 (token "p1") (name "p1") (separator none) (span (offset 511) (line 16) (column 109) (len 2)))))
    (reference r19 (scope relative) (span (offset 517) (line 16) (column 115) (len 2)) (segments (segment 0 (token "p2") (name "p2") (separator none) (span (offset 517) (line 16) (column 115) (len 2)))))
    (reference r20 (scope relative) (span (offset 546) (line 18) (column 20) (len 10)) (segments (segment 0 (token "Documented") (name "Documented") (separator none) (span (offset 546) (line 18) (column 20) (len 10)))))
    (reference r21 (scope relative) (span (offset 557) (line 18) (column 31) (len 12)) (segments (segment 0 (token "alternatives") (name "alternatives") (separator none) (span (offset 557) (line 18) (column 31) (len 12)))))
    (reference r22 (scope relative) (span (offset 611) (line 18) (column 85) (len 5)) (segments (segment 0 (token "value") (name "value") (separator none) (span (offset 611) (line 18) (column 85) (len 5)))))
    (reference r23 (scope relative) (span (offset 641) (line 20) (column 20) (len 13)) (segments (segment 0 (token "ParameterBody") (name "ParameterBody") (separator none) (span (offset 641) (line 20) (column 20) (len 13)))))
    (reference r24 (scope relative) (span (offset 655) (line 20) (column 34) (len 12)) (segments (segment 0 (token "alternatives") (name "alternatives") (separator none) (span (offset 655) (line 20) (column 34) (len 12)))))
    (reference r25 (scope relative) (span (offset 722) (line 20) (column 101) (len 1)) (segments (segment 0 (token "a") (name "a") (separator none) (span (offset 722) (line 20) (column 101) (len 1)))))
  )
  (root (package (visibility none) (name "Collections") (body brace (import (visibility private) (target (span (span (offset 41) (line 2) (column 20) (len 99))) (all none) (ref r0) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 49) (line 2) (column 28) (len 91))) (open (span (offset 49) (line 2) (column 28) (len 1))) (expression (expression (span (offset 50) (line 2) (column 29) (len 89)) (collection-op (operator "forAll") (base (expression (span (offset 50) (line 2) (column 29) (len 5)) (ref r1))) (arguments) (brace-body (body (span (offset 64) (line 2) (column 43) (len 75)) (open-brace (span (offset 64) (line 2) (column 43) (len 1))) (parameters (parameter (span (offset 74) (line 3) (column 9) (len 29)) (direction in (span (offset 74) (line 3) (column 9) (len 2))) (reference-keyword (span (offset 77) (line 3) (column 12) (len 3))) (declaration (name "item") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 102) (line 3) (column 37) (len 1)))))) (result (expression (span (offset 112) (line 4) (column 9) (len 21)) (binary (operator "==") (left (expression (span (offset 112) (line 4) (column 9) (len 4)) (ref r3))) (right (expression (span (offset 120) (line 4) (column 17) (len 13)) (member-access (base (expression (span (offset 120) (line 4) (column 17) (len 8)) (ref r4))) (separator dot) (member (ref r5)))))))) (close-brace (span (offset 138) (line 5) (column 5) (len 1)))))))) (close (span (offset 139) (line 5) (column 6) (len 1)))))))) (body semicolon)) (import (visibility private) (target (span (span (offset 162) (line 7) (column 20) (len 53))) (all none) (ref r6) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 173) (line 7) (column 31) (len 42))) (open (span (offset 173) (line 7) (column 31) (len 1))) (expression (expression (span (offset 174) (line 7) (column 32) (len 40)) (collection-op (operator "collect") (base (expression (span (offset 174) (line 7) (column 32) (len 5)) (ref r7))) (arguments) (brace-body (body (span (offset 189) (line 7) (column 47) (len 25)) (open-brace (span (offset 189) (line 7) (column 47) (len 1))) (parameters (parameter (span (offset 191) (line 7) (column 49) (len 10)) (direction in (span (offset 191) (line 7) (column 49) (len 2))) (reference-keyword none) (declaration (name "item") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 200) (line 7) (column 58) (len 1)))))) (result (expression (span (offset 202) (line 7) (column 60) (len 10)) (member-access (base (expression (span (offset 202) (line 7) (column 60) (len 4)) (ref r8))) (separator dot) (member (ref r9))))) (close-brace (span (offset 213) (line 7) (column 71) (len 1)))))))) (close (span (offset 214) (line 7) (column 72) (len 1)))))))) (body semicolon)) (malformed (code "recovered_package_body_element") (found "private import Broken[items->select {") (span (offset 222) (line 9) (column 5) (len 101))) (import (visibility private) (target (span (span (offset 344) (line 14) (column 20) (len 56))) (all none) (ref r10) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 349) (line 14) (column 25) (len 51))) (open (span (offset 349) (line 14) (column 25) (len 1))) (expression (expression (span (offset 350) (line 14) (column 26) (len 49)) (collection-op (operator "exists") (base (expression (span (offset 350) (line 14) (column 26) (len 5)) (ref r11))) (arguments) (brace-body (body (span (offset 364) (line 14) (column 40) (len 35)) (open-brace (span (offset 364) (line 14) (column 40) (len 1))) (parameters (parameter (span (offset 366) (line 14) (column 42) (len 13)) (direction in (span (offset 366) (line 14) (column 42) (len 2))) (reference-keyword none) (declaration (name "candidate") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 378) (line 14) (column 54) (len 1)))))) (result (expression (span (offset 380) (line 14) (column 56) (len 17)) (binary (operator "!=") (left (expression (span (offset 380) (line 14) (column 56) (len 9)) (ref r12))) (right (expression (span (offset 393) (line 14) (column 69) (len 4)) (null)))))) (close-brace (span (offset 398) (line 14) (column 74) (len 1)))))))) (close (span (offset 399) (line 14) (column 75) (len 1)))))))) (body semicolon)) (import (visibility private) (target (span (span (offset 422) (line 16) (column 20) (len 102))) (all none) (ref r13) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 432) (line 16) (column 30) (len 92))) (open (span (offset 432) (line 16) (column 30) (len 1))) (expression (expression (span (offset 433) (line 16) (column 31) (len 90)) (collection-op (operator "forAll") (base (expression (span (offset 433) (line 16) (column 31) (len 8)) (ref r14))) (arguments) (brace-body (body (span (offset 450) (line 16) (column 48) (len 73)) (open-brace (span (offset 450) (line 16) (column 48) (len 1))) (parameters (parameter (span (offset 452) (line 16) (column 50) (len 19)) (direction none) (reference-keyword none) (declaration (name "p1") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 470) (line 16) (column 68) (len 1)))))) (result (expression (span (offset 472) (line 16) (column 70) (len 49)) (collection-op (operator "exists") (base (expression (span (offset 472) (line 16) (column 70) (len 8)) (ref r16))) (arguments) (brace-body (body (span (offset 489) (line 16) (column 87) (len 32)) (open-brace (span (offset 489) (line 16) (column 87) (len 1))) (parameters (parameter (span (offset 491) (line 16) (column 89) (len 19)) (direction none) (reference-keyword none) (declaration (name "p2") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (semicolon (span (offset 509) (line 16) (column 107) (len 1)))))) (result (expression (span (offset 511) (line 16) (column 109) (len 8)) (binary (operator "!=") (left (expression (span (offset 511) (line 16) (column 109) (len 2)) (ref r18))) (right (expression (span (offset 517) (line 16) (column 115) (len 2)) (ref r19)))))) (close-brace (span (offset 520) (line 16) (column 118) (len 1)))))))) (close-brace (span (offset 522) (line 16) (column 120) (len 1)))))))) (close (span (offset 523) (line 16) (column 121) (len 1)))))))) (body semicolon)) (import (visibility private) (target (span (span (offset 546) (line 18) (column 20) (len 73))) (all none) (ref r20) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 556) (line 18) (column 30) (len 63))) (open (span (offset 556) (line 18) (column 30) (len 1))) (expression (expression (span (offset 557) (line 18) (column 31) (len 61)) (collection-op (operator "minimize") (base (expression (span (offset 557) (line 18) (column 31) (len 12)) (ref r21))) (arguments) (brace-body (body (span (offset 580) (line 18) (column 54) (len 38)) (open-brace (span (offset 580) (line 18) (column 54) (len 1))) (parameters) (result (expression (span (offset 611) (line 18) (column 85) (len 5)) (ref r22))) (close-brace (span (offset 617) (line 18) (column 91) (len 1)))))))) (close (span (offset 618) (line 18) (column 92) (len 1)))))))) (body semicolon)) (import (visibility private) (target (span (span (offset 641) (line 20) (column 20) (len 85))) (all none) (ref r23) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 654) (line 20) (column 33) (len 72))) (open (span (offset 654) (line 20) (column 33) (len 1))) (expression (expression (span (offset 655) (line 20) (column 34) (len 70)) (collection-op (operator "selectOne") (base (expression (span (offset 655) (line 20) (column 34) (len 12)) (ref r24))) (arguments) (brace-body (body (span (offset 679) (line 20) (column 58) (len 46)) (open-brace (span (offset 679) (line 20) (column 58) (len 1))) (parameters (parameter (span (offset 681) (line 20) (column 60) (len 40)) (direction in (span (offset 681) (line 20) (column 60) (len 2))) (reference-keyword (span (offset 684) (line 20) (column 63) (len 3))) (declaration (name "a") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none))) (terminator (body (open-brace (span (offset 690) (line 20) (column 69) (len 1))) (doc present) (close-brace (span (offset 720) (line 20) (column 99) (len 1))))))) (result (expression (span (offset 722) (line 20) (column 101) (len 1)) (ref r25))) (close-brace (span (offset 724) (line 20) (column 103) (len 1)))))))) (close (span (offset 725) (line 20) (column 104) (len 1)))))))) (body semicolon)))))
)
~~~
