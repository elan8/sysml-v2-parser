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
    abstract constraint def ConstraintCheck :> BooleanEvaluation {
        doc
        /*
		 * ConstraintCheck is the most general class for constraint checking. ConstraintCheck is the base
		 * type of all ConstraintDefinitions.
		 */
        ref constraint self : ConstraintCheck :>> BooleanEvaluation::self;
    }
    abstract constraint constraintChecks : ConstraintCheck[0..*] :> booleanEvaluations {
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
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 174) (line 8) (column 17) (len 31)) (segments (segment 0 (token "Performances") (name "Performances") (separator none) (span (offset 174) (line 8) (column 17) (len 12))) (segment 1 (token "BooleanEvaluation") (name "BooleanEvaluation") (separator colon-colon) (span (offset 188) (line 8) (column 31) (len 17)))))
    (reference r1 (scope relative) (span (offset 223) (line 9) (column 17) (len 32)) (segments (segment 0 (token "Performances") (name "Performances") (separator none) (span (offset 223) (line 9) (column 17) (len 12))) (segment 1 (token "booleanEvaluations") (name "booleanEvaluations") (separator colon-colon) (span (offset 237) (line 9) (column 31) (len 18)))))
    (reference r2 (scope relative) (span (offset 273) (line 10) (column 17) (len 29)) (segments (segment 0 (token "Performances") (name "Performances") (separator none) (span (offset 273) (line 10) (column 17) (len 12))) (segment 1 (token "trueEvaluations") (name "trueEvaluations") (separator colon-colon) (span (offset 287) (line 10) (column 31) (len 15)))))
    (reference r3 (scope relative) (span (offset 320) (line 11) (column 17) (len 30)) (segments (segment 0 (token "Performances") (name "Performances") (separator none) (span (offset 320) (line 11) (column 17) (len 12))) (segment 1 (token "falseEvaluations") (name "falseEvaluations") (separator colon-colon) (span (offset 334) (line 11) (column 31) (len 16)))))
    (reference r4 (scope relative) (span (offset 398) (line 13) (column 45) (len 17)) (segments (segment 0 (token "BooleanEvaluation") (name "BooleanEvaluation") (separator none) (span (offset 398) (line 13) (column 45) (len 17)))))
    (reference r5 (scope relative) (span (offset 600) (line 20) (column 24) (len 15)) (segments (segment 0 (token "ConstraintCheck") (name "ConstraintCheck") (separator none) (span (offset 600) (line 20) (column 24) (len 15)))))
    (reference r6 (scope relative) (span (offset 620) (line 20) (column 44) (len 23)) (segments (segment 0 (token "BooleanEvaluation") (name "BooleanEvaluation") (separator none) (span (offset 620) (line 20) (column 44) (len 17))) (segment 1 (token "self") (name "self") (separator colon-colon) (span (offset 639) (line 20) (column 63) (len 4)))))
    (reference r7 (scope relative) (span (offset 689) (line 23) (column 40) (len 15)) (segments (segment 0 (token "ConstraintCheck") (name "ConstraintCheck") (separator none) (span (offset 689) (line 23) (column 40) (len 15)))))
    (reference r8 (scope relative) (span (offset 724) (line 23) (column 75) (len 18)) (segments (segment 0 (token "booleanEvaluations") (name "booleanEvaluations") (separator none) (span (offset 724) (line 23) (column 75) (len 18)))))
    (reference r9 (scope relative) (span (offset 883) (line 30) (column 50) (len 16)) (segments (segment 0 (token "constraintChecks") (name "constraintChecks") (separator none) (span (offset 883) (line 30) (column 50) (len 16)))))
    (reference r10 (scope relative) (span (offset 901) (line 30) (column 68) (len 15)) (segments (segment 0 (token "trueEvaluations") (name "trueEvaluations") (separator none) (span (offset 901) (line 30) (column 68) (len 15)))))
    (reference r11 (scope relative) (span (offset 1096) (line 37) (column 49) (len 16)) (segments (segment 0 (token "constraintChecks") (name "constraintChecks") (separator none) (span (offset 1096) (line 37) (column 49) (len 16)))))
    (reference r12 (scope relative) (span (offset 1114) (line 37) (column 67) (len 16)) (segments (segment 0 (token "falseEvaluations") (name "falseEvaluations") (separator none) (span (offset 1114) (line 37) (column 67) (len 16)))))
  )
  (root (library-package (name "Constraints") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 47) (line 3) (column 4) (len 107)) (normalized "This package defines the base types for constraints and related elements in the\nSysML language.\n"))) (import (target (span (span (offset 174) (line 8) (column 17) (len 31))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 223) (line 9) (column 17) (len 32))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 273) (line 10) (column 17) (len 29))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 320) (line 11) (column 17) (len 30))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (constraint-def (name "ConstraintCheck") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r4)))) (body brace (doc (name none) (locale none) (body (span (offset 428) (line 15) (column 5) (len 144)) (normalized "ConstraintCheck is the most general class for constraint checking. ConstraintCheck is the base\ntype of all ConstraintDefinitions.\n"))) (constraint-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name "self") (short-name none) (type (ref r5)) (multiplicity none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (body semicolon)))) (constraint-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "constraintChecks") (short-name none) (type (ref r7)) (multiplicity (lower (expression (span (offset 705) (line 23) (column 56) (len 1)) (integer 0))) (upper unbounded)) (subsets (relationship (kind subsets) (implied false) (targets (ref r8)))) (redefines none) (body brace (doc (name none) (locale none) (body (span (offset 755) (line 25) (column 5) (len 71)) (normalized "constraintChecks is the base feature of all ConstraintUsages.\n"))))) (constraint-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "assertedConstraintChecks") (short-name none) (type none) (multiplicity none) (subsets (relationship (kind subsets) (implied false) (targets (ref r9) (ref r10)))) (redefines none) (body brace (doc (name none) (locale none) (body (span (offset 929) (line 32) (column 5) (len 110)) (normalized "assertedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be true.\n"))))) (constraint-usage (prefix (direction none) (derived false) (variance abstract) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "negatedConstraintChecks") (short-name none) (type none) (multiplicity none) (subsets (relationship (kind subsets) (implied false) (targets (ref r11) (ref r12)))) (redefines none) (body brace (doc (name none) (locale none) (body (span (offset 1143) (line 39) (column 5) (len 110)) (normalized "negatedConstraintChecks is the subset of constraintChecks for ConstraintChecks asserted to be false.\n"))))))))
)
~~~
