# META
~~~sexpr
(snapshot (type provenance) (description "Verifies import all and complete import/expose targets retain exact aggregate, wildcard, recursive, combined, filter, token, separator, trivia, and delimiter provenance."))
~~~
# SOURCE
~~~sysml
package Provenance {
    private import all Everything::**;
    private import Plain;
    private import Recursive:: /* recursive marker */ **;
    private import Namespace:: /* wildcard marker */ *;
    private import Deep:: /* wildcard */ * /* combined gap */ :: /* recursive */ **;
    private import Filtered [ /* first */ Filters::One /* close */ ] [Filters::Two];
    private import RecursiveFiltered:: /* recursive filter */ ** [Filters::Three];

    view overview : Views::General {
        expose Plain;
        expose Recursive:: /* recursive marker */ **;
        expose Namespace:: /* wildcard marker */ *;
        expose Deep:: /* wildcard */ * /* combined gap */ :: /* recursive */ **;
        expose Filtered [ /* first */ Filters::One /* close */ ] [Filters::Two];
        expose RecursiveFiltered:: /* recursive filter */ ** [Filters::Three];
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
    (reference r3 (scope relative) (span (offset 163) (line 5) (column 20) (len 9)) (segments (segment 0 (token "Namespace") (name "Namespace") (separator none) (span (offset 163) (line 5) (column 20) (len 9)))))
    (reference r4 (scope relative) (span (offset 219) (line 6) (column 20) (len 4)) (segments (segment 0 (token "Deep") (name "Deep") (separator none) (span (offset 219) (line 6) (column 20) (len 4)))))
    (reference r5 (scope relative) (span (offset 304) (line 7) (column 20) (len 8)) (segments (segment 0 (token "Filtered") (name "Filtered") (separator none) (span (offset 304) (line 7) (column 20) (len 8)))))
    (reference r6 (scope relative) (span (offset 327) (line 7) (column 43) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 327) (line 7) (column 43) (len 7))) (segment 1 (token "One") (name "One") (separator colon-colon) (span (offset 336) (line 7) (column 52) (len 3)))))
    (reference r7 (scope relative) (span (offset 355) (line 7) (column 71) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 355) (line 7) (column 71) (len 7))) (segment 1 (token "Two") (name "Two") (separator colon-colon) (span (offset 364) (line 7) (column 80) (len 3)))))
    (reference r8 (scope relative) (span (offset 389) (line 8) (column 20) (len 17)) (segments (segment 0 (token "RecursiveFiltered") (name "RecursiveFiltered") (separator none) (span (offset 389) (line 8) (column 20) (len 17)))))
    (reference r9 (scope relative) (span (offset 436) (line 8) (column 67) (len 14)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 436) (line 8) (column 67) (len 7))) (segment 1 (token "Three") (name "Three") (separator colon-colon) (span (offset 445) (line 8) (column 76) (len 5)))))
    (reference r10 (scope relative) (span (offset 474) (line 10) (column 21) (len 14)) (segments (segment 0 (token "Views") (name "Views") (separator none) (span (offset 474) (line 10) (column 21) (len 5))) (segment 1 (token "General") (name "General") (separator colon-colon) (span (offset 481) (line 10) (column 28) (len 7)))))
    (reference r11 (scope relative) (span (offset 506) (line 11) (column 16) (len 5)) (segments (segment 0 (token "Plain") (name "Plain") (separator none) (span (offset 506) (line 11) (column 16) (len 5)))))
    (reference r12 (scope relative) (span (offset 528) (line 12) (column 16) (len 9)) (segments (segment 0 (token "Recursive") (name "Recursive") (separator none) (span (offset 528) (line 12) (column 16) (len 9)))))
    (reference r13 (scope relative) (span (offset 582) (line 13) (column 16) (len 9)) (segments (segment 0 (token "Namespace") (name "Namespace") (separator none) (span (offset 582) (line 13) (column 16) (len 9)))))
    (reference r14 (scope relative) (span (offset 634) (line 14) (column 16) (len 4)) (segments (segment 0 (token "Deep") (name "Deep") (separator none) (span (offset 634) (line 14) (column 16) (len 4)))))
    (reference r15 (scope relative) (span (offset 715) (line 15) (column 16) (len 8)) (segments (segment 0 (token "Filtered") (name "Filtered") (separator none) (span (offset 715) (line 15) (column 16) (len 8)))))
    (reference r16 (scope relative) (span (offset 738) (line 15) (column 39) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 738) (line 15) (column 39) (len 7))) (segment 1 (token "One") (name "One") (separator colon-colon) (span (offset 747) (line 15) (column 48) (len 3)))))
    (reference r17 (scope relative) (span (offset 766) (line 15) (column 67) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 766) (line 15) (column 67) (len 7))) (segment 1 (token "Two") (name "Two") (separator colon-colon) (span (offset 775) (line 15) (column 76) (len 3)))))
    (reference r18 (scope relative) (span (offset 796) (line 16) (column 16) (len 17)) (segments (segment 0 (token "RecursiveFiltered") (name "RecursiveFiltered") (separator none) (span (offset 796) (line 16) (column 16) (len 17)))))
    (reference r19 (scope relative) (span (offset 843) (line 16) (column 63) (len 14)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 843) (line 16) (column 63) (len 7))) (segment 1 (token "Three") (name "Three") (separator colon-colon) (span (offset 852) (line 16) (column 72) (len 5)))))
  )
  (root (package (name "Provenance") (body brace (import (visibility private) (target (span (span (offset 40) (line 2) (column 20) (len 18))) (all (span (offset 40) (line 2) (column 20) (len 3))) (ref r0) (shape (membership (recursive-suffix (span (span (offset 54) (line 2) (column 34) (len 4))) (separator (span (offset 54) (line 2) (column 34) (len 2))) (marker (span (offset 56) (line 2) (column 36) (len 2))))))) (body semicolon)) (import (visibility private) (target (span (span (offset 79) (line 3) (column 20) (len 5))) (all none) (ref r1) (shape (membership (recursive-suffix none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 105) (line 4) (column 20) (len 37))) (all none) (ref r2) (shape (membership (recursive-suffix (span (span (offset 114) (line 4) (column 29) (len 28))) (separator (span (offset 114) (line 4) (column 29) (len 2))) (marker (span (offset 140) (line 4) (column 55) (len 2))))))) (body semicolon)) (import (visibility private) (target (span (span (offset 163) (line 5) (column 20) (len 35))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 172) (line 5) (column 29) (len 26))) (separator (span (offset 172) (line 5) (column 29) (len 2))) (marker (span (offset 197) (line 5) (column 54) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 219) (line 6) (column 20) (len 64))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 223) (line 6) (column 24) (len 19))) (separator (span (offset 223) (line 6) (column 24) (len 2))) (marker (span (offset 241) (line 6) (column 42) (len 1)))) (recursive-suffix (span (span (offset 262) (line 6) (column 63) (len 21))) (separator (span (offset 262) (line 6) (column 63) (len 2))) (marker (span (offset 281) (line 6) (column 82) (len 2)))) (combined-recursive-suffix-span (span (offset 223) (line 6) (column 24) (len 60)))))) (body semicolon)) (import (visibility private) (target (span (span (offset 304) (line 7) (column 20) (len 64))) (all none) (ref r5) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 313) (line 7) (column 29) (len 40))) (open (span (offset 313) (line 7) (column 29) (len 1))) (expression (expression (span (offset 327) (line 7) (column 43) (len 12)) (ref r6))) (close (span (offset 352) (line 7) (column 68) (len 1)))) (filter-member (span (span (offset 354) (line 7) (column 70) (len 14))) (open (span (offset 354) (line 7) (column 70) (len 1))) (expression (expression (span (offset 355) (line 7) (column 71) (len 12)) (ref r7))) (close (span (offset 367) (line 7) (column 83) (len 1)))))))) (body semicolon)) (import (visibility private) (target (span (span (offset 389) (line 8) (column 20) (len 62))) (all none) (ref r8) (shape (filter (recursive-suffix (span (span (offset 406) (line 8) (column 37) (len 28))) (separator (span (offset 406) (line 8) (column 37) (len 2))) (marker (span (offset 432) (line 8) (column 63) (len 2)))) (members (filter-member (span (span (offset 435) (line 8) (column 66) (len 16))) (open (span (offset 435) (line 8) (column 66) (len 1))) (expression (expression (span (offset 436) (line 8) (column 67) (len 14)) (ref r9))) (close (span (offset 450) (line 8) (column 81) (len 1)))))))) (body semicolon)) (view (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "overview") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (references none) (crosses none) (redefines none) (value none) (body brace (expose (target (span (span (offset 506) (line 11) (column 16) (len 5))) (all none) (ref r11) (shape (membership (recursive-suffix none)))) (body semicolon)) (expose (target (span (span (offset 528) (line 12) (column 16) (len 37))) (all none) (ref r12) (shape (membership (recursive-suffix (span (span (offset 537) (line 12) (column 25) (len 28))) (separator (span (offset 537) (line 12) (column 25) (len 2))) (marker (span (offset 563) (line 12) (column 51) (len 2))))))) (body semicolon)) (expose (target (span (span (offset 582) (line 13) (column 16) (len 35))) (all none) (ref r13) (shape (namespace (wildcard-suffix (span (span (offset 591) (line 13) (column 25) (len 26))) (separator (span (offset 591) (line 13) (column 25) (len 2))) (marker (span (offset 616) (line 13) (column 50) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (expose (target (span (span (offset 634) (line 14) (column 16) (len 64))) (all none) (ref r14) (shape (namespace (wildcard-suffix (span (span (offset 638) (line 14) (column 20) (len 19))) (separator (span (offset 638) (line 14) (column 20) (len 2))) (marker (span (offset 656) (line 14) (column 38) (len 1)))) (recursive-suffix (span (span (offset 677) (line 14) (column 59) (len 21))) (separator (span (offset 677) (line 14) (column 59) (len 2))) (marker (span (offset 696) (line 14) (column 78) (len 2)))) (combined-recursive-suffix-span (span (offset 638) (line 14) (column 20) (len 60)))))) (body semicolon)) (expose (target (span (span (offset 715) (line 15) (column 16) (len 64))) (all none) (ref r15) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 724) (line 15) (column 25) (len 40))) (open (span (offset 724) (line 15) (column 25) (len 1))) (expression (expression (span (offset 738) (line 15) (column 39) (len 12)) (ref r16))) (close (span (offset 763) (line 15) (column 64) (len 1)))) (filter-member (span (span (offset 765) (line 15) (column 66) (len 14))) (open (span (offset 765) (line 15) (column 66) (len 1))) (expression (expression (span (offset 766) (line 15) (column 67) (len 12)) (ref r17))) (close (span (offset 778) (line 15) (column 79) (len 1)))))))) (body semicolon)) (expose (target (span (span (offset 796) (line 16) (column 16) (len 62))) (all none) (ref r18) (shape (filter (recursive-suffix (span (span (offset 813) (line 16) (column 33) (len 28))) (separator (span (offset 813) (line 16) (column 33) (len 2))) (marker (span (offset 839) (line 16) (column 59) (len 2)))) (members (filter-member (span (span (offset 842) (line 16) (column 62) (len 16))) (open (span (offset 842) (line 16) (column 62) (len 1))) (expression (expression (span (offset 843) (line 16) (column 63) (len 14)) (ref r19))) (close (span (offset 857) (line 16) (column 77) (len 1)))))))) (body semicolon)))))))
)
~~~
