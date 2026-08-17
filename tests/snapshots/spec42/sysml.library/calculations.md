# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Systems Library/Calculations"))
~~~
# SOURCE
~~~sysml
standard library package Calculations {
	doc
	/*
	 * This package defines the base types for calculations and related behavioral elements in the
	 * SysML language.
	 */

	private import Performances::Evaluation;
	private import Performances::evaluations;
	private import Actions::Action;
	private import Actions::actions;
	
	abstract calc def Calculation :> Action, Evaluation {
		doc
		/*
		 * Calculation is the most general class of evaluations of CalculationDefinitions in a
		 * system or part of a system. Calculation is the base class of all CalculationDefinitions.
		 */
	
		ref calc self: Calculation :>> Action::self, Evaluation::self;
		
		abstract calc subcalculations: Calculation :> calculations, subactions {
			doc
			/*
			 * The subactions of this Calculation that are Calculations.
			 */
		}
		
	}
	
	abstract calc calculations: Calculation[0..*] nonunique :> actions, evaluations {
		doc
		/*
		 * calculations is the base Feature for all CalculationUsages.
		 */
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "calculations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Calculations {
    doc
    /*
	 * This package defines the base types for calculations and related behavioral elements in the
	 * SysML language.
	 */
    private import Performances::Evaluation;
    private import Performances::evaluations;
    private import Actions::Action;
    private import Actions::actions;
    calc def Calculation :> Action, Evaluation {
        doc
        /*
		 * Calculation is the most general class of evaluations of CalculationDefinitions in a
		 * system or part of a system. Calculation is the base class of all CalculationDefinitions.
		 */
        ref calc self : Calculation :>> Action::self, Evaluation::self;
        abstract calc subcalculations : Calculation :> calculations, subactions {
            doc
            /*
			 * The subactions of this Calculation that are Calculations.
			 */
        }
    }
    calc def calculations :> actions, evaluations {
        doc
        /*
		 * calculations is the base Feature for all CalculationUsages.
		 */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 187) (line 8) (column 17) (len 24)) (segments (segment 0 (token "Performances") (name "Performances") (separator none) (span (offset 187) (line 8) (column 17) (len 12))) (segment 1 (token "Evaluation") (name "Evaluation") (separator colon-colon) (span (offset 201) (line 8) (column 31) (len 10)))))
    (reference r1 (scope relative) (span (offset 229) (line 9) (column 17) (len 25)) (segments (segment 0 (token "Performances") (name "Performances") (separator none) (span (offset 229) (line 9) (column 17) (len 12))) (segment 1 (token "evaluations") (name "evaluations") (separator colon-colon) (span (offset 243) (line 9) (column 31) (len 11)))))
    (reference r2 (scope relative) (span (offset 272) (line 10) (column 17) (len 15)) (segments (segment 0 (token "Actions") (name "Actions") (separator none) (span (offset 272) (line 10) (column 17) (len 7))) (segment 1 (token "Action") (name "Action") (separator colon-colon) (span (offset 281) (line 10) (column 26) (len 6)))))
    (reference r3 (scope relative) (span (offset 305) (line 11) (column 17) (len 16)) (segments (segment 0 (token "Actions") (name "Actions") (separator none) (span (offset 305) (line 11) (column 17) (len 7))) (segment 1 (token "actions") (name "actions") (separator colon-colon) (span (offset 314) (line 11) (column 26) (len 7)))))
  )
  (root (library-package (name "Calculations") (standard true) (body brace (doc) (import (target (span (span (offset 187) (line 8) (column 17) (len 24))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 229) (line 9) (column 17) (len 25))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 272) (line 10) (column 17) (len 15))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 305) (line 11) (column 17) (len 16))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (calc-def (name "Calculation") (body brace (doc) (calc-usage) (calc-usage))) (calc-def (name "calculations") (body brace (doc))))))
)
~~~
