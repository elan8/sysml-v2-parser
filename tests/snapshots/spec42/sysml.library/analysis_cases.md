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
  (references
    (reference r0 (scope relative) (span (offset 208) (line 8) (column 20) (len 24)) (segments (segment 0 (token "Performances") (name "Performances") (separator none) (span (offset 208) (line 8) (column 20) (len 12))) (segment 1 (token "Evaluation") (name "Evaluation") (separator colon-colon) (span (offset 222) (line 8) (column 34) (len 10)))))
    (reference r1 (scope relative) (span (offset 253) (line 9) (column 20) (len 25)) (segments (segment 0 (token "Performances") (name "Performances") (separator none) (span (offset 253) (line 9) (column 20) (len 12))) (segment 1 (token "evaluations") (name "evaluations") (separator colon-colon) (span (offset 267) (line 9) (column 34) (len 11)))))
    (reference r2 (scope relative) (span (offset 299) (line 10) (column 20) (len 25)) (segments (segment 0 (token "Calculations") (name "Calculations") (separator none) (span (offset 299) (line 10) (column 20) (len 12))) (segment 1 (token "Calculation") (name "Calculation") (separator colon-colon) (span (offset 313) (line 10) (column 34) (len 11)))))
    (reference r3 (scope relative) (span (offset 345) (line 11) (column 20) (len 11)) (segments (segment 0 (token "Cases") (name "Cases") (separator none) (span (offset 345) (line 11) (column 20) (len 5))) (segment 1 (token "Case") (name "Case") (separator colon-colon) (span (offset 352) (line 11) (column 27) (len 4)))))
    (reference r4 (scope relative) (span (offset 377) (line 12) (column 20) (len 12)) (segments (segment 0 (token "Cases") (name "Cases") (separator none) (span (offset 377) (line 12) (column 20) (len 5))) (segment 1 (token "cases") (name "cases") (separator colon-colon) (span (offset 384) (line 12) (column 27) (len 5)))))
    (reference r5 (scope relative) (span (offset 1031) (line 32) (column 39) (len 12)) (segments (segment 0 (token "AnalysisCase") (name "AnalysisCase") (separator none) (span (offset 1031) (line 32) (column 39) (len 12)))))
    (reference r6 (scope relative) (span (offset 1063) (line 32) (column 71) (len 5)) (segments (segment 0 (token "cases") (name "cases") (separator none) (span (offset 1063) (line 32) (column 71) (len 5)))))
  )
  (root (library-package (name "AnalysisCases") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 55) (line 3) (column 7) (len 130)) (normalized "This package defines the base types for analysis cases and related behavioral elements\nin the SysML language.\n"))) (import (target (span (span (offset 208) (line 8) (column 20) (len 24))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 253) (line 9) (column 20) (len 25))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 299) (line 10) (column 20) (len 25))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 345) (line 11) (column 20) (len 11))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 377) (line 12) (column 20) (len 12))) (all none) (ref r4) (shape (membership (recursive-suffix none))))) (analysis-case-def (modifiers (abstract (span (offset 396) (line 14) (column 5) (len 8))))) (analysis-case-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (name "analysisCases") (type (ref r5)) (subsets (relationship (kind subsets) (implied false) (targets (ref r6)))) (redefines none)))))
)
~~~
