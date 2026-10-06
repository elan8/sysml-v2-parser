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
    abstract calc def Calculation :> Action, Evaluation {
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
    abstract calc calculations : Calculation[0..*] nonunique :> actions, evaluations {
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
    (reference r4 (scope relative) (span (offset 599) (line 20) (column 18) (len 11)) (segments (segment 0 (token "Calculation") (name "Calculation") (separator none) (span (offset 599) (line 20) (column 18) (len 11)))))
    (reference r5 (scope relative) (span (offset 615) (line 20) (column 34) (len 12)) (segments (segment 0 (token "Action") (name "Action") (separator none) (span (offset 615) (line 20) (column 34) (len 6))) (segment 1 (token "self") (name "self") (separator colon-colon) (span (offset 623) (line 20) (column 42) (len 4)))))
    (reference r6 (scope relative) (span (offset 629) (line 20) (column 48) (len 16)) (segments (segment 0 (token "Evaluation") (name "Evaluation") (separator none) (span (offset 629) (line 20) (column 48) (len 10))) (segment 1 (token "self") (name "self") (separator colon-colon) (span (offset 641) (line 20) (column 60) (len 4)))))
    (reference r7 (scope relative) (span (offset 683) (line 22) (column 34) (len 11)) (segments (segment 0 (token "Calculation") (name "Calculation") (separator none) (span (offset 683) (line 22) (column 34) (len 11)))))
    (reference r8 (scope relative) (span (offset 698) (line 22) (column 49) (len 12)) (segments (segment 0 (token "calculations") (name "calculations") (separator none) (span (offset 698) (line 22) (column 49) (len 12)))))
    (reference r9 (scope relative) (span (offset 712) (line 22) (column 63) (len 10)) (segments (segment 0 (token "subactions") (name "subactions") (separator none) (span (offset 712) (line 22) (column 63) (len 10)))))
    (reference r10 (scope relative) (span (offset 850) (line 31) (column 30) (len 11)) (segments (segment 0 (token "Calculation") (name "Calculation") (separator none) (span (offset 850) (line 31) (column 30) (len 11)))))
    (reference r11 (scope relative) (span (offset 881) (line 31) (column 61) (len 7)) (segments (segment 0 (token "actions") (name "actions") (separator none) (span (offset 881) (line 31) (column 61) (len 7)))))
    (reference r12 (scope relative) (span (offset 890) (line 31) (column 70) (len 11)) (segments (segment 0 (token "evaluations") (name "evaluations") (separator none) (span (offset 890) (line 31) (column 70) (len 11)))))
  )
  (root (library-package (name "Calculations") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 48) (line 3) (column 4) (len 119)) (normalized "This package defines the base types for calculations and related behavioral elements in the\nSysML language.\n"))) (import (target (span (span (offset 187) (line 8) (column 17) (len 24))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 229) (line 9) (column 17) (len 25))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 272) (line 10) (column 17) (len 15))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 305) (line 11) (column 17) (len 16))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (calc-def (name "Calculation") (modifiers (abstract (span (offset 326) (line 13) (column 2) (len 8)))) (body brace (doc (name none) (locale none) (body (span (offset 390) (line 15) (column 5) (len 187)) (normalized "Calculation is the most general class of evaluations of CalculationDefinitions in a\nsystem or part of a system. Calculation is the base class of all CalculationDefinitions.\n"))) (calc-usage (name "self") (short-name none) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (ref r5) (ref r6)) (value none) (body semicolon)) (calc-usage (name "subcalculations") (short-name none) (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets (relationship (kind subsets) (implied false) (targets (ref r8) (ref r9)))) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 737) (line 24) (column 6) (len 69)) (normalized "The subactions of this Calculation that are Calculations.\n"))))))) (calc-usage (name "calculations") (short-name none) (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity (lower (expression (span (offset 862) (line 31) (column 42) (len 1)) (integer 0))) (upper (expression (span (offset 865) (line 31) (column 45) (len 1)) (infinity)))) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (subsets (relationship (kind subsets) (implied false) (targets (ref r11) (ref r12)))) (redefines none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 914) (line 33) (column 5) (len 69)) (normalized "calculations is the base Feature for all CalculationUsages.\n"))))))))
)
~~~
