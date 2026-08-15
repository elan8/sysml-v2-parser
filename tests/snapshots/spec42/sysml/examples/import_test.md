# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Simple Tests): ImportTest"))
~~~
# SOURCE
~~~sysml
package ImportTest {
    package Pkg1 {
    	private import Pkg2::Pkg21::Pkg211::P211;
    	private import Pkg2::Pkg21::*;
    	private import Pkg211::*::**;
        part p11 : Pkg211::P211;
        part def P12;
    }

    package Pkg2 {
        private import Pkg1::*;
        package Pkg21 {
        	package Pkg211 {
        		part def P211 :> P12;
        	}
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "import_test.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ImportTest {
    package Pkg1 {
        private import Pkg2::Pkg21::Pkg211::P211;
        private import Pkg2::Pkg21::*;
        private import Pkg211::*::**;
        part p11 : Pkg211::P211;
        part def P12;
    }
    package Pkg2 {
        private import Pkg1::*;
        package Pkg21 {
            package Pkg211 {
                part def P211 :> P12;
            }
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 60) (line 3) (column 21) (len 25)) (segments (segment 0 (token "Pkg2") (name "Pkg2") (separator none) (span (offset 60) (line 3) (column 21) (len 4))) (segment 1 (token "Pkg21") (name "Pkg21") (separator colon-colon) (span (offset 66) (line 3) (column 27) (len 5))) (segment 2 (token "Pkg211") (name "Pkg211") (separator colon-colon) (span (offset 73) (line 3) (column 34) (len 6))) (segment 3 (token "P211") (name "P211") (separator colon-colon) (span (offset 81) (line 3) (column 42) (len 4)))))
    (reference r1 (scope relative) (span (offset 107) (line 4) (column 21) (len 11)) (segments (segment 0 (token "Pkg2") (name "Pkg2") (separator none) (span (offset 107) (line 4) (column 21) (len 4))) (segment 1 (token "Pkg21") (name "Pkg21") (separator colon-colon) (span (offset 113) (line 4) (column 27) (len 5)))))
    (reference r2 (scope relative) (span (offset 143) (line 5) (column 21) (len 6)) (segments (segment 0 (token "Pkg211") (name "Pkg211") (separator none) (span (offset 143) (line 5) (column 21) (len 6)))))
    (reference r3 (scope relative) (span (offset 262) (line 11) (column 24) (len 4)) (segments (segment 0 (token "Pkg1") (name "Pkg1") (separator none) (span (offset 262) (line 11) (column 24) (len 4)))))
  )
  (root (package (name "ImportTest") (body brace (package (name "Pkg1") (body brace (import (target (span (span (offset 60) (line 3) (column 21) (len 25))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 107) (line 4) (column 21) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 118) (line 4) (column 32) (len 3))) (separator (span (offset 118) (line 4) (column 32) (len 2))) (marker (span (offset 120) (line 4) (column 34) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 143) (line 5) (column 21) (len 13))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 149) (line 5) (column 27) (len 3))) (separator (span (offset 149) (line 5) (column 27) (len 2))) (marker (span (offset 151) (line 5) (column 29) (len 1)))) (recursive-suffix (span (span (offset 152) (line 5) (column 30) (len 4))) (separator (span (offset 152) (line 5) (column 30) (len 2))) (marker (span (offset 154) (line 5) (column 32) (len 2)))) (combined-recursive-suffix-span (span (offset 149) (line 5) (column 27) (len 7))))))) (part-usage) (part-def (name "P12") (body semicolon)))) (package (name "Pkg2") (body brace (import (target (span (span (offset 262) (line 11) (column 24) (len 7))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 266) (line 11) (column 28) (len 3))) (separator (span (offset 266) (line 11) (column 28) (len 2))) (marker (span (offset 268) (line 11) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Pkg21") (body brace (package (name "Pkg211") (body brace (part-def (name "P211") (body semicolon)))))))))))
)
~~~
