# META
~~~sexpr
(snapshot (type provenance) (description "Verifies import all and complete import/expose targets retain exact aggregate, wildcard, recursive, combined, filter, token, separator, trivia, and delimiter provenance."))
~~~
# SOURCE
~~~sysml
package Provenance {
    private import all Everything::**;
    private import Plain;
    private import Recursive:: //* recursive marker */ **;
    private import Namespace:: //* wildcard marker */ *;
    private import Deep:: //* wildcard */ * //* combined gap */ :: //* recursive */ **;
    private import Filtered [ //* first */ Filters::One //* close */ ] [Filters::Two];
    private import RecursiveFiltered:: //* recursive filter */ ** [Filters::Three];

    view overview : Views::General {
        expose Plain;
        expose Recursive:: //* recursive marker */ **;
        expose Namespace:: //* wildcard marker */ *;
        expose Deep:: //* wildcard */ * //* combined gap */ :: //* recursive */ **;
        expose Filtered [ //* first */ Filters::One //* close */ ] [Filters::Two];
        expose RecursiveFiltered:: //* recursive filter */ ** [Filters::Three];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "suffix_provenance.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package Provenance {
    private import all Everything::**;
    private import Plain;
    private import Recursive::**;
    private import Namespace::*;
    private import Deep::*::**;
    private import Filtered [Filters::One] [Filters::Two];
    private import RecursiveFiltered::** [Filters::Three];
    view overview : Views::General {
        expose Plain;
        expose Recursive::**;
        expose Namespace::*;
        expose Deep::*::**;
        expose Filtered [Filters::One] [Filters::Two];
        expose RecursiveFiltered::** [Filters::Three];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 44) (line 2) (column 24) (len 10)) (segments (segment 0 (token "Everything") (name "Everything") (separator none) (span (offset 44) (line 2) (column 24) (len 10)))))
    (reference r1 (scope relative) (span (offset 79) (line 3) (column 20) (len 5)) (segments (segment 0 (token "Plain") (name "Plain") (separator none) (span (offset 79) (line 3) (column 20) (len 5)))))
    (reference r2 (scope relative) (span (offset 105) (line 4) (column 20) (len 9)) (segments (segment 0 (token "Recursive") (name "Recursive") (separator none) (span (offset 105) (line 4) (column 20) (len 9)))))
    (reference r3 (scope relative) (span (offset 164) (line 5) (column 20) (len 9)) (segments (segment 0 (token "Namespace") (name "Namespace") (separator none) (span (offset 164) (line 5) (column 20) (len 9)))))
    (reference r4 (scope relative) (span (offset 221) (line 6) (column 20) (len 4)) (segments (segment 0 (token "Deep") (name "Deep") (separator none) (span (offset 221) (line 6) (column 20) (len 4)))))
    (reference r5 (scope relative) (span (offset 309) (line 7) (column 20) (len 8)) (segments (segment 0 (token "Filtered") (name "Filtered") (separator none) (span (offset 309) (line 7) (column 20) (len 8)))))
    (reference r6 (scope relative) (span (offset 333) (line 7) (column 44) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 333) (line 7) (column 44) (len 7))) (segment 1 (token "One") (name "One") (separator colon-colon) (span (offset 342) (line 7) (column 53) (len 3)))))
    (reference r7 (scope relative) (span (offset 362) (line 7) (column 73) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 362) (line 7) (column 73) (len 7))) (segment 1 (token "Two") (name "Two") (separator colon-colon) (span (offset 371) (line 7) (column 82) (len 3)))))
    (reference r8 (scope relative) (span (offset 396) (line 8) (column 20) (len 17)) (segments (segment 0 (token "RecursiveFiltered") (name "RecursiveFiltered") (separator none) (span (offset 396) (line 8) (column 20) (len 17)))))
    (reference r9 (scope relative) (span (offset 444) (line 8) (column 68) (len 14)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 444) (line 8) (column 68) (len 7))) (segment 1 (token "Three") (name "Three") (separator colon-colon) (span (offset 453) (line 8) (column 77) (len 5)))))
    (reference r10 (scope relative) (span (offset 482) (line 10) (column 21) (len 14)) (segments (segment 0 (token "Views") (name "Views") (separator none) (span (offset 482) (line 10) (column 21) (len 5))) (segment 1 (token "General") (name "General") (separator colon-colon) (span (offset 489) (line 10) (column 28) (len 7)))))
    (reference r11 (scope relative) (span (offset 514) (line 11) (column 16) (len 5)) (segments (segment 0 (token "Plain") (name "Plain") (separator none) (span (offset 514) (line 11) (column 16) (len 5)))))
    (reference r12 (scope relative) (span (offset 536) (line 12) (column 16) (len 9)) (segments (segment 0 (token "Recursive") (name "Recursive") (separator none) (span (offset 536) (line 12) (column 16) (len 9)))))
    (reference r13 (scope relative) (span (offset 591) (line 13) (column 16) (len 9)) (segments (segment 0 (token "Namespace") (name "Namespace") (separator none) (span (offset 591) (line 13) (column 16) (len 9)))))
    (reference r14 (scope relative) (span (offset 644) (line 14) (column 16) (len 4)) (segments (segment 0 (token "Deep") (name "Deep") (separator none) (span (offset 644) (line 14) (column 16) (len 4)))))
    (reference r15 (scope relative) (span (offset 728) (line 15) (column 16) (len 8)) (segments (segment 0 (token "Filtered") (name "Filtered") (separator none) (span (offset 728) (line 15) (column 16) (len 8)))))
    (reference r16 (scope relative) (span (offset 752) (line 15) (column 40) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 752) (line 15) (column 40) (len 7))) (segment 1 (token "One") (name "One") (separator colon-colon) (span (offset 761) (line 15) (column 49) (len 3)))))
    (reference r17 (scope relative) (span (offset 781) (line 15) (column 69) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 781) (line 15) (column 69) (len 7))) (segment 1 (token "Two") (name "Two") (separator colon-colon) (span (offset 790) (line 15) (column 78) (len 3)))))
    (reference r18 (scope relative) (span (offset 811) (line 16) (column 16) (len 17)) (segments (segment 0 (token "RecursiveFiltered") (name "RecursiveFiltered") (separator none) (span (offset 811) (line 16) (column 16) (len 17)))))
    (reference r19 (scope relative) (span (offset 859) (line 16) (column 64) (len 14)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 859) (line 16) (column 64) (len 7))) (segment 1 (token "Three") (name "Three") (separator colon-colon) (span (offset 868) (line 16) (column 73) (len 5)))))
  )
  (root (package (visibility none) (name "Provenance") (body brace (import (visibility private) (target (span (span (offset 40) (line 2) (column 20) (len 18))) (all (span (offset 40) (line 2) (column 20) (len 3))) (ref r0) (shape (membership (recursive-suffix (span (span (offset 54) (line 2) (column 34) (len 4))) (separator (span (offset 54) (line 2) (column 34) (len 2))) (marker (span (offset 56) (line 2) (column 36) (len 2))))))) (body semicolon)) (import (visibility private) (target (span (span (offset 79) (line 3) (column 20) (len 5))) (all none) (ref r1) (shape (membership (recursive-suffix none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 105) (line 4) (column 20) (len 38))) (all none) (ref r2) (shape (membership (recursive-suffix (span (span (offset 114) (line 4) (column 29) (len 29))) (separator (span (offset 114) (line 4) (column 29) (len 2))) (marker (span (offset 141) (line 4) (column 56) (len 2))))))) (body semicolon)) (import (visibility private) (target (span (span (offset 164) (line 5) (column 20) (len 36))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 173) (line 5) (column 29) (len 27))) (separator (span (offset 173) (line 5) (column 29) (len 2))) (marker (span (offset 199) (line 5) (column 55) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 221) (line 6) (column 20) (len 67))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 225) (line 6) (column 24) (len 20))) (separator (span (offset 225) (line 6) (column 24) (len 2))) (marker (span (offset 244) (line 6) (column 43) (len 1)))) (recursive-suffix (span (span (offset 266) (line 6) (column 65) (len 22))) (separator (span (offset 266) (line 6) (column 65) (len 2))) (marker (span (offset 286) (line 6) (column 85) (len 2)))) (combined-recursive-suffix-span (span (offset 225) (line 6) (column 24) (len 63)))))) (body semicolon)) (import (visibility private) (target (span (span (offset 309) (line 7) (column 20) (len 66))) (all none) (ref r5) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 318) (line 7) (column 29) (len 42))) (open (span (offset 318) (line 7) (column 29) (len 1))) (expression (expression (span (offset 333) (line 7) (column 44) (len 12)) (ref r6))) (close (span (offset 359) (line 7) (column 70) (len 1)))) (filter-member (span (span (offset 361) (line 7) (column 72) (len 14))) (open (span (offset 361) (line 7) (column 72) (len 1))) (expression (expression (span (offset 362) (line 7) (column 73) (len 12)) (ref r7))) (close (span (offset 374) (line 7) (column 85) (len 1)))))))) (body semicolon)) (import (visibility private) (target (span (span (offset 396) (line 8) (column 20) (len 63))) (all none) (ref r8) (shape (filter (recursive-suffix (span (span (offset 413) (line 8) (column 37) (len 29))) (separator (span (offset 413) (line 8) (column 37) (len 2))) (marker (span (offset 440) (line 8) (column 64) (len 2)))) (members (filter-member (span (span (offset 443) (line 8) (column 67) (len 16))) (open (span (offset 443) (line 8) (column 67) (len 1))) (expression (expression (span (offset 444) (line 8) (column 68) (len 14)) (ref r9))) (close (span (offset 458) (line 8) (column 82) (len 1)))))))) (body semicolon)) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "overview") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (expose (target (span (span (offset 514) (line 11) (column 16) (len 5))) (all none) (ref r11) (shape (membership (recursive-suffix none)))) (body semicolon)) (expose (target (span (span (offset 536) (line 12) (column 16) (len 38))) (all none) (ref r12) (shape (membership (recursive-suffix (span (span (offset 545) (line 12) (column 25) (len 29))) (separator (span (offset 545) (line 12) (column 25) (len 2))) (marker (span (offset 572) (line 12) (column 52) (len 2))))))) (body semicolon)) (expose (target (span (span (offset 591) (line 13) (column 16) (len 36))) (all none) (ref r13) (shape (namespace (wildcard-suffix (span (span (offset 600) (line 13) (column 25) (len 27))) (separator (span (offset 600) (line 13) (column 25) (len 2))) (marker (span (offset 626) (line 13) (column 51) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (expose (target (span (span (offset 644) (line 14) (column 16) (len 67))) (all none) (ref r14) (shape (namespace (wildcard-suffix (span (span (offset 648) (line 14) (column 20) (len 20))) (separator (span (offset 648) (line 14) (column 20) (len 2))) (marker (span (offset 667) (line 14) (column 39) (len 1)))) (recursive-suffix (span (span (offset 689) (line 14) (column 61) (len 22))) (separator (span (offset 689) (line 14) (column 61) (len 2))) (marker (span (offset 709) (line 14) (column 81) (len 2)))) (combined-recursive-suffix-span (span (offset 648) (line 14) (column 20) (len 63)))))) (body semicolon)) (expose (target (span (span (offset 728) (line 15) (column 16) (len 66))) (all none) (ref r15) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 737) (line 15) (column 25) (len 42))) (open (span (offset 737) (line 15) (column 25) (len 1))) (expression (expression (span (offset 752) (line 15) (column 40) (len 12)) (ref r16))) (close (span (offset 778) (line 15) (column 66) (len 1)))) (filter-member (span (span (offset 780) (line 15) (column 68) (len 14))) (open (span (offset 780) (line 15) (column 68) (len 1))) (expression (expression (span (offset 781) (line 15) (column 69) (len 12)) (ref r17))) (close (span (offset 793) (line 15) (column 81) (len 1)))))))) (body semicolon)) (expose (target (span (span (offset 811) (line 16) (column 16) (len 63))) (all none) (ref r18) (shape (filter (recursive-suffix (span (span (offset 828) (line 16) (column 33) (len 29))) (separator (span (offset 828) (line 16) (column 33) (len 2))) (marker (span (offset 855) (line 16) (column 60) (len 2)))) (members (filter-member (span (span (offset 858) (line 16) (column 63) (len 16))) (open (span (offset 858) (line 16) (column 63) (len 1))) (expression (expression (span (offset 859) (line 16) (column 64) (len 14)) (ref r19))) (close (span (offset 873) (line 16) (column 78) (len 1)))))))) (body semicolon)))))))
)
~~~
