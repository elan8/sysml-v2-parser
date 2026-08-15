# META
~~~sexpr
(snapshot (type provenance) (description "Verifies import all and complete import/expose targets retain exact aggregate, wildcard, recursive, combined, filter, token, separator, trivia, and delimiter provenance."))
~~~
# SOURCE
~~~sysml
package Provenance {
    import all Everything::**;
    import Plain;
    import Recursive:: /* recursive marker */ **;
    import Namespace:: /* wildcard marker */ *;
    import Deep:: /* wildcard */ * /* combined gap */ :: /* recursive */ **;
    import Filtered [ /* first */ Filters::One /* close */ ] [Filters::Two];
    import RecursiveFiltered:: /* recursive filter */ ** [Filters::Three];

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
    import all Everything::**;
    import Plain;
    import Recursive::**;
    import Namespace::*;
    import Deep::*::**;
    import Filtered [Filters::One] [Filters::Two];
    import RecursiveFiltered::** [Filters::Three];
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
  (references
    (reference r0 (scope relative) (span (offset 36) (line 2) (column 16) (len 10)) (segments (segment 0 (token "Everything") (name "Everything") (separator none) (span (offset 36) (line 2) (column 16) (len 10)))))
    (reference r1 (scope relative) (span (offset 63) (line 3) (column 12) (len 5)) (segments (segment 0 (token "Plain") (name "Plain") (separator none) (span (offset 63) (line 3) (column 12) (len 5)))))
    (reference r2 (scope relative) (span (offset 81) (line 4) (column 12) (len 9)) (segments (segment 0 (token "Recursive") (name "Recursive") (separator none) (span (offset 81) (line 4) (column 12) (len 9)))))
    (reference r3 (scope relative) (span (offset 131) (line 5) (column 12) (len 9)) (segments (segment 0 (token "Namespace") (name "Namespace") (separator none) (span (offset 131) (line 5) (column 12) (len 9)))))
    (reference r4 (scope relative) (span (offset 179) (line 6) (column 12) (len 4)) (segments (segment 0 (token "Deep") (name "Deep") (separator none) (span (offset 179) (line 6) (column 12) (len 4)))))
    (reference r5 (scope relative) (span (offset 256) (line 7) (column 12) (len 8)) (segments (segment 0 (token "Filtered") (name "Filtered") (separator none) (span (offset 256) (line 7) (column 12) (len 8)))))
    (reference r6 (scope relative) (span (offset 279) (line 7) (column 35) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 279) (line 7) (column 35) (len 7))) (segment 1 (token "One") (name "One") (separator colon-colon) (span (offset 288) (line 7) (column 44) (len 3)))))
    (reference r7 (scope relative) (span (offset 307) (line 7) (column 63) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 307) (line 7) (column 63) (len 7))) (segment 1 (token "Two") (name "Two") (separator colon-colon) (span (offset 316) (line 7) (column 72) (len 3)))))
    (reference r8 (scope relative) (span (offset 333) (line 8) (column 12) (len 17)) (segments (segment 0 (token "RecursiveFiltered") (name "RecursiveFiltered") (separator none) (span (offset 333) (line 8) (column 12) (len 17)))))
    (reference r9 (scope relative) (span (offset 380) (line 8) (column 59) (len 14)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 380) (line 8) (column 59) (len 7))) (segment 1 (token "Three") (name "Three") (separator colon-colon) (span (offset 389) (line 8) (column 68) (len 5)))))
    (reference r10 (scope relative) (span (offset 418) (line 10) (column 21) (len 14)) (segments (segment 0 (token "Views") (name "Views") (separator none) (span (offset 418) (line 10) (column 21) (len 5))) (segment 1 (token "General") (name "General") (separator colon-colon) (span (offset 425) (line 10) (column 28) (len 7)))))
    (reference r11 (scope relative) (span (offset 450) (line 11) (column 16) (len 5)) (segments (segment 0 (token "Plain") (name "Plain") (separator none) (span (offset 450) (line 11) (column 16) (len 5)))))
    (reference r12 (scope relative) (span (offset 472) (line 12) (column 16) (len 9)) (segments (segment 0 (token "Recursive") (name "Recursive") (separator none) (span (offset 472) (line 12) (column 16) (len 9)))))
    (reference r13 (scope relative) (span (offset 526) (line 13) (column 16) (len 9)) (segments (segment 0 (token "Namespace") (name "Namespace") (separator none) (span (offset 526) (line 13) (column 16) (len 9)))))
    (reference r14 (scope relative) (span (offset 578) (line 14) (column 16) (len 4)) (segments (segment 0 (token "Deep") (name "Deep") (separator none) (span (offset 578) (line 14) (column 16) (len 4)))))
    (reference r15 (scope relative) (span (offset 659) (line 15) (column 16) (len 8)) (segments (segment 0 (token "Filtered") (name "Filtered") (separator none) (span (offset 659) (line 15) (column 16) (len 8)))))
    (reference r16 (scope relative) (span (offset 682) (line 15) (column 39) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 682) (line 15) (column 39) (len 7))) (segment 1 (token "One") (name "One") (separator colon-colon) (span (offset 691) (line 15) (column 48) (len 3)))))
    (reference r17 (scope relative) (span (offset 710) (line 15) (column 67) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 710) (line 15) (column 67) (len 7))) (segment 1 (token "Two") (name "Two") (separator colon-colon) (span (offset 719) (line 15) (column 76) (len 3)))))
    (reference r18 (scope relative) (span (offset 740) (line 16) (column 16) (len 17)) (segments (segment 0 (token "RecursiveFiltered") (name "RecursiveFiltered") (separator none) (span (offset 740) (line 16) (column 16) (len 17)))))
    (reference r19 (scope relative) (span (offset 787) (line 16) (column 63) (len 14)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 787) (line 16) (column 63) (len 7))) (segment 1 (token "Three") (name "Three") (separator colon-colon) (span (offset 796) (line 16) (column 72) (len 5)))))
  )
  (root (package (name "Provenance") (body brace (import (target (span (span (offset 32) (line 2) (column 12) (len 18))) (all (span (offset 32) (line 2) (column 12) (len 3))) (ref r0) (shape (membership (recursive-suffix (span (span (offset 46) (line 2) (column 26) (len 4))) (separator (span (offset 46) (line 2) (column 26) (len 2))) (marker (span (offset 48) (line 2) (column 28) (len 2)))))))) (import (target (span (span (offset 63) (line 3) (column 12) (len 5))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 81) (line 4) (column 12) (len 37))) (all none) (ref r2) (shape (membership (recursive-suffix (span (span (offset 90) (line 4) (column 21) (len 28))) (separator (span (offset 90) (line 4) (column 21) (len 2))) (marker (span (offset 116) (line 4) (column 47) (len 2)))))))) (import (target (span (span (offset 131) (line 5) (column 12) (len 35))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 140) (line 5) (column 21) (len 26))) (separator (span (offset 140) (line 5) (column 21) (len 2))) (marker (span (offset 165) (line 5) (column 46) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 179) (line 6) (column 12) (len 64))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 183) (line 6) (column 16) (len 19))) (separator (span (offset 183) (line 6) (column 16) (len 2))) (marker (span (offset 201) (line 6) (column 34) (len 1)))) (recursive-suffix (span (span (offset 222) (line 6) (column 55) (len 21))) (separator (span (offset 222) (line 6) (column 55) (len 2))) (marker (span (offset 241) (line 6) (column 74) (len 2)))) (combined-recursive-suffix-span (span (offset 183) (line 6) (column 16) (len 60))))))) (import (target (span (span (offset 256) (line 7) (column 12) (len 64))) (all none) (ref r5) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 265) (line 7) (column 21) (len 40))) (open (span (offset 265) (line 7) (column 21) (len 1))) (expression (expression (span (offset 279) (line 7) (column 35) (len 12)) (ref r6))) (close (span (offset 304) (line 7) (column 60) (len 1)))) (filter-member (span (span (offset 306) (line 7) (column 62) (len 14))) (open (span (offset 306) (line 7) (column 62) (len 1))) (expression (expression (span (offset 307) (line 7) (column 63) (len 12)) (ref r7))) (close (span (offset 319) (line 7) (column 75) (len 1))))))))) (import (target (span (span (offset 333) (line 8) (column 12) (len 62))) (all none) (ref r8) (shape (filter (recursive-suffix (span (span (offset 350) (line 8) (column 29) (len 28))) (separator (span (offset 350) (line 8) (column 29) (len 2))) (marker (span (offset 376) (line 8) (column 55) (len 2)))) (members (filter-member (span (span (offset 379) (line 8) (column 58) (len 16))) (open (span (offset 379) (line 8) (column 58) (len 1))) (expression (expression (span (offset 380) (line 8) (column 59) (len 14)) (ref r9))) (close (span (offset 394) (line 8) (column 73) (len 1))))))))) (view (name "overview") (type (ref r10)) (body brace (expose (target (span (span (offset 450) (line 11) (column 16) (len 5))) (all none) (ref r11) (shape (membership (recursive-suffix none))))) (expose (target (span (span (offset 472) (line 12) (column 16) (len 37))) (all none) (ref r12) (shape (membership (recursive-suffix (span (span (offset 481) (line 12) (column 25) (len 28))) (separator (span (offset 481) (line 12) (column 25) (len 2))) (marker (span (offset 507) (line 12) (column 51) (len 2)))))))) (expose (target (span (span (offset 526) (line 13) (column 16) (len 35))) (all none) (ref r13) (shape (namespace (wildcard-suffix (span (span (offset 535) (line 13) (column 25) (len 26))) (separator (span (offset 535) (line 13) (column 25) (len 2))) (marker (span (offset 560) (line 13) (column 50) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (expose (target (span (span (offset 578) (line 14) (column 16) (len 64))) (all none) (ref r14) (shape (namespace (wildcard-suffix (span (span (offset 582) (line 14) (column 20) (len 19))) (separator (span (offset 582) (line 14) (column 20) (len 2))) (marker (span (offset 600) (line 14) (column 38) (len 1)))) (recursive-suffix (span (span (offset 621) (line 14) (column 59) (len 21))) (separator (span (offset 621) (line 14) (column 59) (len 2))) (marker (span (offset 640) (line 14) (column 78) (len 2)))) (combined-recursive-suffix-span (span (offset 582) (line 14) (column 20) (len 60))))))) (expose (target (span (span (offset 659) (line 15) (column 16) (len 64))) (all none) (ref r15) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 668) (line 15) (column 25) (len 40))) (open (span (offset 668) (line 15) (column 25) (len 1))) (expression (expression (span (offset 682) (line 15) (column 39) (len 12)) (ref r16))) (close (span (offset 707) (line 15) (column 64) (len 1)))) (filter-member (span (span (offset 709) (line 15) (column 66) (len 14))) (open (span (offset 709) (line 15) (column 66) (len 1))) (expression (expression (span (offset 710) (line 15) (column 67) (len 12)) (ref r17))) (close (span (offset 722) (line 15) (column 79) (len 1))))))))) (expose (target (span (span (offset 740) (line 16) (column 16) (len 62))) (all none) (ref r18) (shape (filter (recursive-suffix (span (span (offset 757) (line 16) (column 33) (len 28))) (separator (span (offset 757) (line 16) (column 33) (len 2))) (marker (span (offset 783) (line 16) (column 59) (len 2)))) (members (filter-member (span (span (offset 786) (line 16) (column 62) (len 16))) (open (span (offset 786) (line 16) (column 62) (len 1))) (expression (expression (span (offset 787) (line 16) (column 63) (len 14)) (ref r19))) (close (span (offset 801) (line 16) (column 77) (len 1))))))))))))))
)
~~~
