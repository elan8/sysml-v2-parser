# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Systems Library/AnalysisCases. In particular, AnalysisCases.sysml:21 uses `ref analysis self : AnalysisCase :>> Case::self;`, the corpus form that must remain one AnalysisCaseUsage with a source-backed occurrence prefix."))
~~~
# SOURCE
~~~sysml
standard library package AnalysisCases {
    doc
    /*
     * This package defines the base types for analysis cases and related behavioral elements
     * in the SysML language.
     */

    private import Performances::Evaluation;
    private import Performances::evaluations;
    private import Calculations::Calculation;
    private import Cases::Case;
    private import Cases::cases;

    abstract analysis def AnalysisCase :> Case {
        doc
        /*
         * AnalysisCase is the most general class of performances of AnalysisCaseDefinitions.
         * AnalysisCase is the base class of all AnalysisCaseDefinitions.
         */

        ref analysis self : AnalysisCase :>> Case::self;
        subject subj :>> Case::subj;

        abstract analysis subAnalysisCases : AnalysisCase[0..*] :> analysisCases, subcases {
            doc
            /*
             * Other AnalysisCases carried out as part of the performance of this AnalysisCase.
             */
        }
    }

    abstract analysis analysisCases : AnalysisCase[0..*] nonunique :> cases {
        doc
        /*
         * analysisCases is the base feature of all AnalysisCaseUsages.
         */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "analysis_cases.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package AnalysisCases {
    doc
    /*
     * This package defines the base types for analysis cases and related behavioral elements
     * in the SysML language.
     */
    private import Performances::Evaluation;
    private import Performances::evaluations;
    private import Calculations::Calculation;
    private import Cases::Case;
    private import Cases::cases;
    abstract analysis def AnalysisCase :> Case {
        doc
        /*
         * AnalysisCase is the most general class of performances of AnalysisCaseDefinitions.
         * AnalysisCase is the base class of all AnalysisCaseDefinitions.
         */
        ref analysis self : AnalysisCase :>> Case::self;
        subject subj :>> Case::subj;
        abstract analysis subAnalysisCases : AnalysisCase :> analysisCases, subcases {
            doc
            /*
             * Other AnalysisCases carried out as part of the performance of this AnalysisCase.
             */
        }
    }
    abstract analysis analysisCases : AnalysisCase :> cases {
        doc
        /*
         * analysisCases is the base feature of all AnalysisCaseUsages.
         */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 208) (line 8) (column 20) (len 24)) (segments (segment 0 (token "Performances") (name "Performances") (separator none) (span (offset 208) (line 8) (column 20) (len 12))) (segment 1 (token "Evaluation") (name "Evaluation") (separator colon-colon) (span (offset 222) (line 8) (column 34) (len 10)))))
    (reference r1 (scope relative) (span (offset 253) (line 9) (column 20) (len 25)) (segments (segment 0 (token "Performances") (name "Performances") (separator none) (span (offset 253) (line 9) (column 20) (len 12))) (segment 1 (token "evaluations") (name "evaluations") (separator colon-colon) (span (offset 267) (line 9) (column 34) (len 11)))))
    (reference r2 (scope relative) (span (offset 299) (line 10) (column 20) (len 25)) (segments (segment 0 (token "Calculations") (name "Calculations") (separator none) (span (offset 299) (line 10) (column 20) (len 12))) (segment 1 (token "Calculation") (name "Calculation") (separator colon-colon) (span (offset 313) (line 10) (column 34) (len 11)))))
    (reference r3 (scope relative) (span (offset 345) (line 11) (column 20) (len 11)) (segments (segment 0 (token "Cases") (name "Cases") (separator none) (span (offset 345) (line 11) (column 20) (len 5))) (segment 1 (token "Case") (name "Case") (separator colon-colon) (span (offset 352) (line 11) (column 27) (len 4)))))
    (reference r4 (scope relative) (span (offset 377) (line 12) (column 20) (len 12)) (segments (segment 0 (token "Cases") (name "Cases") (separator none) (span (offset 377) (line 12) (column 20) (len 5))) (segment 1 (token "cases") (name "cases") (separator colon-colon) (span (offset 384) (line 12) (column 27) (len 5)))))
    (reference r5 (scope relative) (span (offset 673) (line 21) (column 29) (len 12)) (segments (segment 0 (token "AnalysisCase") (name "AnalysisCase") (separator none) (span (offset 673) (line 21) (column 29) (len 12)))))
    (reference r6 (scope relative) (span (offset 690) (line 21) (column 46) (len 10)) (segments (segment 0 (token "Case") (name "Case") (separator none) (span (offset 690) (line 21) (column 46) (len 4))) (segment 1 (token "self") (name "self") (separator colon-colon) (span (offset 696) (line 21) (column 52) (len 4)))))
    (reference r7 (scope relative) (span (offset 727) (line 22) (column 26) (len 10)) (segments (segment 0 (token "Case") (name "Case") (separator none) (span (offset 727) (line 22) (column 26) (len 4))) (segment 1 (token "subj") (name "subj") (separator colon-colon) (span (offset 733) (line 22) (column 32) (len 4)))))
    (reference r8 (scope relative) (span (offset 785) (line 24) (column 46) (len 12)) (segments (segment 0 (token "AnalysisCase") (name "AnalysisCase") (separator none) (span (offset 785) (line 24) (column 46) (len 12)))))
    (reference r9 (scope relative) (span (offset 807) (line 24) (column 68) (len 13)) (segments (segment 0 (token "analysisCases") (name "analysisCases") (separator none) (span (offset 807) (line 24) (column 68) (len 13)))))
    (reference r10 (scope relative) (span (offset 822) (line 24) (column 83) (len 8)) (segments (segment 0 (token "subcases") (name "subcases") (separator none) (span (offset 822) (line 24) (column 83) (len 8)))))
    (reference r11 (scope relative) (span (offset 1031) (line 32) (column 39) (len 12)) (segments (segment 0 (token "AnalysisCase") (name "AnalysisCase") (separator none) (span (offset 1031) (line 32) (column 39) (len 12)))))
    (reference r12 (scope relative) (span (offset 1063) (line 32) (column 71) (len 5)) (segments (segment 0 (token "cases") (name "cases") (separator none) (span (offset 1063) (line 32) (column 71) (len 5)))))
  )
  (root (library-package (name "AnalysisCases") (standard true) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 55) (line 3) (column 7) (len 130)) (normalized "This package defines the base types for analysis cases and related behavioral elements\nin the SysML language.\n"))) (import (visibility private) (target (span (span (offset 208) (line 8) (column 20) (len 24))) (all none) (ref r0) (shape (membership (recursive-suffix none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 253) (line 9) (column 20) (len 25))) (all none) (ref r1) (shape (membership (recursive-suffix none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 299) (line 10) (column 20) (len 25))) (all none) (ref r2) (shape (membership (recursive-suffix none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 345) (line 11) (column 20) (len 11))) (all none) (ref r3) (shape (membership (recursive-suffix none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 377) (line 12) (column 20) (len 12))) (all none) (ref r4) (shape (membership (recursive-suffix none)))) (body semicolon)) (analysis-case-def (name "AnalysisCase") (modifiers (abstract (span (offset 396) (line 14) (column 5) (len 8)))) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 463) (line 16) (column 11) (len 178)) (normalized "AnalysisCase is the most general class of performances of AnalysisCaseDefinitions.\nAnalysisCase is the base class of all AnalysisCaseDefinitions.\n"))) (analysis-case-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (name "self") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (body semicolon)) (subject (name "subj") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (analysis-case-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "subAnalysisCases") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r9) (ref r10)))) (redefines none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 863) (line 26) (column 15) (len 110)) (normalized "Other AnalysisCases carried out as part of the performance of this AnalysisCase.\n"))))))) (analysis-case-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "analysisCases") (short-name none) (type (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r12)))) (redefines none) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 1093) (line 34) (column 11) (len 82)) (normalized "analysisCases is the base feature of all AnalysisCaseUsages.\n"))))))))
)
~~~
