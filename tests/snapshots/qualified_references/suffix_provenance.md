# META
~~~sexpr
(snapshot (type provenance) (description "Verifies import and expose wildcard, recursive, combined, and filter suffixes retain exact token, separator, trivia, and delimiter provenance."))
~~~
# SOURCE
~~~sysml
package Provenance {
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
    (reference r0 (scope relative) (span (offset 32) (line 2) (column 12) (len 5)) (segments (segment 0 (token "Plain") (name "Plain") (separator none) (span (offset 32) (line 2) (column 12) (len 5)))))
    (reference r1 (scope relative) (span (offset 50) (line 3) (column 12) (len 9)) (segments (segment 0 (token "Recursive") (name "Recursive") (separator none) (span (offset 50) (line 3) (column 12) (len 9)))))
    (reference r2 (scope relative) (span (offset 100) (line 4) (column 12) (len 9)) (segments (segment 0 (token "Namespace") (name "Namespace") (separator none) (span (offset 100) (line 4) (column 12) (len 9)))))
    (reference r3 (scope relative) (span (offset 148) (line 5) (column 12) (len 4)) (segments (segment 0 (token "Deep") (name "Deep") (separator none) (span (offset 148) (line 5) (column 12) (len 4)))))
    (reference r4 (scope relative) (span (offset 225) (line 6) (column 12) (len 8)) (segments (segment 0 (token "Filtered") (name "Filtered") (separator none) (span (offset 225) (line 6) (column 12) (len 8)))))
    (reference r5 (scope relative) (span (offset 248) (line 6) (column 35) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 248) (line 6) (column 35) (len 7))) (segment 1 (token "One") (name "One") (separator colon-colon) (span (offset 257) (line 6) (column 44) (len 3)))))
    (reference r6 (scope relative) (span (offset 276) (line 6) (column 63) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 276) (line 6) (column 63) (len 7))) (segment 1 (token "Two") (name "Two") (separator colon-colon) (span (offset 285) (line 6) (column 72) (len 3)))))
    (reference r7 (scope relative) (span (offset 302) (line 7) (column 12) (len 17)) (segments (segment 0 (token "RecursiveFiltered") (name "RecursiveFiltered") (separator none) (span (offset 302) (line 7) (column 12) (len 17)))))
    (reference r8 (scope relative) (span (offset 349) (line 7) (column 59) (len 14)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 349) (line 7) (column 59) (len 7))) (segment 1 (token "Three") (name "Three") (separator colon-colon) (span (offset 358) (line 7) (column 68) (len 5)))))
    (reference r9 (scope relative) (span (offset 387) (line 9) (column 21) (len 14)) (segments (segment 0 (token "Views") (name "Views") (separator none) (span (offset 387) (line 9) (column 21) (len 5))) (segment 1 (token "General") (name "General") (separator colon-colon) (span (offset 394) (line 9) (column 28) (len 7)))))
    (reference r10 (scope relative) (span (offset 419) (line 10) (column 16) (len 5)) (segments (segment 0 (token "Plain") (name "Plain") (separator none) (span (offset 419) (line 10) (column 16) (len 5)))))
    (reference r11 (scope relative) (span (offset 441) (line 11) (column 16) (len 9)) (segments (segment 0 (token "Recursive") (name "Recursive") (separator none) (span (offset 441) (line 11) (column 16) (len 9)))))
    (reference r12 (scope relative) (span (offset 495) (line 12) (column 16) (len 9)) (segments (segment 0 (token "Namespace") (name "Namespace") (separator none) (span (offset 495) (line 12) (column 16) (len 9)))))
    (reference r13 (scope relative) (span (offset 547) (line 13) (column 16) (len 4)) (segments (segment 0 (token "Deep") (name "Deep") (separator none) (span (offset 547) (line 13) (column 16) (len 4)))))
    (reference r14 (scope relative) (span (offset 628) (line 14) (column 16) (len 8)) (segments (segment 0 (token "Filtered") (name "Filtered") (separator none) (span (offset 628) (line 14) (column 16) (len 8)))))
    (reference r15 (scope relative) (span (offset 651) (line 14) (column 39) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 651) (line 14) (column 39) (len 7))) (segment 1 (token "One") (name "One") (separator colon-colon) (span (offset 660) (line 14) (column 48) (len 3)))))
    (reference r16 (scope relative) (span (offset 679) (line 14) (column 67) (len 12)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 679) (line 14) (column 67) (len 7))) (segment 1 (token "Two") (name "Two") (separator colon-colon) (span (offset 688) (line 14) (column 76) (len 3)))))
    (reference r17 (scope relative) (span (offset 709) (line 15) (column 16) (len 17)) (segments (segment 0 (token "RecursiveFiltered") (name "RecursiveFiltered") (separator none) (span (offset 709) (line 15) (column 16) (len 17)))))
    (reference r18 (scope relative) (span (offset 756) (line 15) (column 63) (len 14)) (segments (segment 0 (token "Filters") (name "Filters") (separator none) (span (offset 756) (line 15) (column 63) (len 7))) (segment 1 (token "Three") (name "Three") (separator colon-colon) (span (offset 765) (line 15) (column 72) (len 5)))))
  )
  (root (package (name "Provenance") (body (import (target (ref r0) (shape (membership (recursive-suffix none))))) (import (target (ref r1) (shape (membership (recursive-suffix (span (span (offset 59) (line 3) (column 21) (len 28))) (separator (span (offset 59) (line 3) (column 21) (len 2))) (marker (span (offset 85) (line 3) (column 47) (len 2)))))))) (import (target (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 109) (line 4) (column 21) (len 26))) (separator (span (offset 109) (line 4) (column 21) (len 2))) (marker (span (offset 134) (line 4) (column 46) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 152) (line 5) (column 16) (len 19))) (separator (span (offset 152) (line 5) (column 16) (len 2))) (marker (span (offset 170) (line 5) (column 34) (len 1)))) (recursive-suffix (span (span (offset 191) (line 5) (column 55) (len 21))) (separator (span (offset 191) (line 5) (column 55) (len 2))) (marker (span (offset 210) (line 5) (column 74) (len 2)))) (combined-recursive-suffix-span (span (offset 152) (line 5) (column 16) (len 60))))))) (import (target (ref r4) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 234) (line 6) (column 21) (len 40))) (open (span (offset 234) (line 6) (column 21) (len 1))) (expression (ref r5)) (close (span (offset 273) (line 6) (column 60) (len 1)))) (filter-member (span (span (offset 275) (line 6) (column 62) (len 14))) (open (span (offset 275) (line 6) (column 62) (len 1))) (expression (ref r6)) (close (span (offset 288) (line 6) (column 75) (len 1))))))))) (import (target (ref r7) (shape (filter (recursive-suffix (span (span (offset 319) (line 7) (column 29) (len 28))) (separator (span (offset 319) (line 7) (column 29) (len 2))) (marker (span (offset 345) (line 7) (column 55) (len 2)))) (members (filter-member (span (span (offset 348) (line 7) (column 58) (len 16))) (open (span (offset 348) (line 7) (column 58) (len 1))) (expression (ref r8)) (close (span (offset 363) (line 7) (column 73) (len 1))))))))) (view (name "overview") (type (ref r9)) (body (expose (target (ref r10) (shape (membership (recursive-suffix none))))) (expose (target (ref r11) (shape (membership (recursive-suffix (span (span (offset 450) (line 11) (column 25) (len 28))) (separator (span (offset 450) (line 11) (column 25) (len 2))) (marker (span (offset 476) (line 11) (column 51) (len 2)))))))) (expose (target (ref r12) (shape (namespace (wildcard-suffix (span (span (offset 504) (line 12) (column 25) (len 26))) (separator (span (offset 504) (line 12) (column 25) (len 2))) (marker (span (offset 529) (line 12) (column 50) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (expose (target (ref r13) (shape (namespace (wildcard-suffix (span (span (offset 551) (line 13) (column 20) (len 19))) (separator (span (offset 551) (line 13) (column 20) (len 2))) (marker (span (offset 569) (line 13) (column 38) (len 1)))) (recursive-suffix (span (span (offset 590) (line 13) (column 59) (len 21))) (separator (span (offset 590) (line 13) (column 59) (len 2))) (marker (span (offset 609) (line 13) (column 78) (len 2)))) (combined-recursive-suffix-span (span (offset 551) (line 13) (column 20) (len 60))))))) (expose (target (ref r14) (shape (filter (recursive-suffix none) (members (filter-member (span (span (offset 637) (line 14) (column 25) (len 40))) (open (span (offset 637) (line 14) (column 25) (len 1))) (expression (ref r15)) (close (span (offset 676) (line 14) (column 64) (len 1)))) (filter-member (span (span (offset 678) (line 14) (column 66) (len 14))) (open (span (offset 678) (line 14) (column 66) (len 1))) (expression (ref r16)) (close (span (offset 691) (line 14) (column 79) (len 1))))))))) (expose (target (ref r17) (shape (filter (recursive-suffix (span (span (offset 726) (line 15) (column 33) (len 28))) (separator (span (offset 726) (line 15) (column 33) (len 2))) (marker (span (offset 752) (line 15) (column 59) (len 2)))) (members (filter-member (span (span (offset 755) (line 15) (column 62) (len 16))) (open (span (offset 755) (line 15) (column 62) (len 1))) (expression (ref r18)) (close (span (offset 770) (line 15) (column 77) (len 1))))))))))))))
)
~~~
