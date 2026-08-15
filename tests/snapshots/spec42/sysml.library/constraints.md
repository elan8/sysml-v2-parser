# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Systems Library/Constraints"))
~~~
# SOURCE
~~~sysml
standard library package Constraints {
	doc
	/*
	 * This package defines the base types for constraints and related elements in the
	 * SysML language.
	 */

	private import Performances::BooleanEvaluation;
	private import Performances::booleanEvaluations;
	private import Performances::trueEvaluations;
	private import Performances::falseEvaluations;
	
	abstract constraint def ConstraintCheck :> BooleanEvaluation {
		doc
		/*
		 * ConstraintCheck is the most general class for constraint checking. ConstraintCheck is the base
		 * type of all ConstraintDefinitions.
		 */
	
		ref constraint self: ConstraintCheck :>> BooleanEvaluation::self;
	}
	
	abstract constraint constraintChecks: ConstraintCheck[0..*] nonunique :> booleanEvaluations {
		doc
		/*
		 * constraintChecks is the base feature of all ConstraintUsages.
		 */
	}
	
	abstract constraint assertedConstraintChecks :> constraintChecks, trueEvaluations {
		doc
		/*
		 * assertedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be true.
		 */
	}
		
	abstract constraint negatedConstraintChecks :> constraintChecks, falseEvaluations {
		doc
		/*
		 * negatedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be false.
		 */
	}
		
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "constraints.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Constraints {
    doc
    /*
	 * This package defines the base types for constraints and related elements in the
	 * SysML language.
	 */
    private import Performances::BooleanEvaluation;
    private import Performances::booleanEvaluations;
    private import Performances::trueEvaluations;
    private import Performances::falseEvaluations;
    constraint def ConstraintCheck :> BooleanEvaluation {
        doc
        /*
		 * ConstraintCheck is the most general class for constraint checking. ConstraintCheck is the base
		 * type of all ConstraintDefinitions.
		 */
        'ref';
        constraint self : ConstraintCheck :>> BooleanEvaluation::self;
    }
    constraint constraintChecks : ConstraintCheck :> booleanEvaluations {
        doc
        /*
		 * constraintChecks is the base feature of all ConstraintUsages.
		 */
    }
    constraint assertedConstraintChecks :> constraintChecks, trueEvaluations {
        doc
        /*
		 * assertedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be true.
		 */
    }
    constraint negatedConstraintChecks :> constraintChecks, falseEvaluations {
        doc
        /*
		 * negatedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be false.
		 */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 174) (line 8) (column 17) (len 31)) (segments (segment 0 (token "Performances") (name "Performances") (separator none) (span (offset 174) (line 8) (column 17) (len 12))) (segment 1 (token "BooleanEvaluation") (name "BooleanEvaluation") (separator colon-colon) (span (offset 188) (line 8) (column 31) (len 17)))))
    (reference r1 (scope relative) (span (offset 223) (line 9) (column 17) (len 32)) (segments (segment 0 (token "Performances") (name "Performances") (separator none) (span (offset 223) (line 9) (column 17) (len 12))) (segment 1 (token "booleanEvaluations") (name "booleanEvaluations") (separator colon-colon) (span (offset 237) (line 9) (column 31) (len 18)))))
    (reference r2 (scope relative) (span (offset 273) (line 10) (column 17) (len 29)) (segments (segment 0 (token "Performances") (name "Performances") (separator none) (span (offset 273) (line 10) (column 17) (len 12))) (segment 1 (token "trueEvaluations") (name "trueEvaluations") (separator colon-colon) (span (offset 287) (line 10) (column 31) (len 15)))))
    (reference r3 (scope relative) (span (offset 320) (line 11) (column 17) (len 30)) (segments (segment 0 (token "Performances") (name "Performances") (separator none) (span (offset 320) (line 11) (column 17) (len 12))) (segment 1 (token "falseEvaluations") (name "falseEvaluations") (separator colon-colon) (span (offset 334) (line 11) (column 31) (len 16)))))
  )
  (root (library-package (name "Constraints") (standard true) (body brace (doc) (import (target (span (span (offset 174) (line 8) (column 17) (len 31))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 223) (line 9) (column 17) (len 32))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 273) (line 10) (column 17) (len 29))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 320) (line 11) (column 17) (len 30))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (constraint-def) (constraint-usage) (constraint-usage) (constraint-usage))))
)
~~~
