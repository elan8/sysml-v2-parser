# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Requirement Derivation/DerivationConnections"))
~~~
# SOURCE
~~~sysml
standard library package DerivationConnections {
	doc
	/*
	 * This package provides a library model for derivation connections between requirements.
	 */
	 
	private import SequenceFunctions::excludes;
	private import ControlFunctions::allTrue;
	
	requirement originalRequirements[*] {
		doc /* originalRequirements are the original requirements in Derivation connections. */
	}
	requirement derivedRequirements[*] {
		doc /* derivedRequirements are the derived requirments in Derivation connections. */
	}
	
	abstract connection def Derivation {
		doc
		/*
		 * A Derivation connection asserts that one or more derivedRequirements are derived from
		 * a single originalRequirement. This means that any subject that satisfies the
		 * originalRequirement should, in itself or though other things related to it, satisfy
		 * each of the derivedRequirements.
		 * 
		 * A connection usage typed by Derivation must have requirement usages for all its ends.
		 * The single end for the originalRequirement should subset originalRequirement, while
		 * the rest of the ends should subset derivedRequirements.
		 */
		
		// Note: This redefinition causes a distinguishibility problem for binary connections, becuse
		// participant is already redefined for them to limit the multiplicity to 2.
		// ref requirement :>> participant {
		//	doc /* All the participants in a Derivation must be requirements. */
		// }
		
		ref requirement originalRequirement[1] :>> originalRequirements :> participant {
			doc /* The single original requirement. */
		}
		ref requirement :>> derivedRequirements[1..*] :> participant {
			doc /* The one or more requirements that are derived from the original requirement. */
		}
		
		private assert constraint originalNotDerived {
			doc /* The original requirement must not be a derived requirement. */
			
			derivedRequirements->excludes(originalRequirement)
		}
		
		private assert constraint originalImpliesDerived {
			doc 
			/* 
			 * Whenever the originalRequirement is satisfied, all of the derivedRequirements must also
			 * be satisfied.
			 */
			 
			originalRequirement.result implies allTrue(derivedRequirements.result)
		}	
	}
	
	abstract connection derivations : Derivation[*] {
		doc /* derivations is the base feature for Derivation connection usages. */
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "derivation_connections.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package DerivationConnections {
    doc
    /*
	 * This package provides a library model for derivation connections between requirements.
	 */
    private import SequenceFunctions::excludes;
    private import ControlFunctions::allTrue;
    requirement originalRequirements[*] {
        doc
        /* originalRequirements are the original requirements in Derivation connections. */
    }
    requirement derivedRequirements[*] {
        doc
        /* derivedRequirements are the derived requirments in Derivation connections. */
    }
    abstract connection def Derivation {
        doc
        /*
		 * A Derivation connection asserts that one or more derivedRequirements are derived from
		 * a single originalRequirement. This means that any subject that satisfies the
		 * originalRequirement should, in itself or though other things related to it, satisfy
		 * each of the derivedRequirements.
		 * 
		 * A connection usage typed by Derivation must have requirement usages for all its ends.
		 * The single end for the originalRequirement should subset originalRequirement, while
		 * the rest of the ends should subset derivedRequirements.
		 */
        ref requirement originalRequirement[1] :>> originalRequirements :> participant {
            doc
            /* The single original requirement. */
        }
        ref requirement [1..*] :>> derivedRequirements :> participant {
            doc
            /* The one or more requirements that are derived from the original requirement. */
        }
        private assert constraint originalNotDerived {
            doc
            /* The original requirement must not be a derived requirement. */
            derivedRequirements->excludes(originalRequirement);
        }
        private assert constraint originalImpliesDerived {
            doc
            /* 
			 * Whenever the originalRequirement is satisfied, all of the derivedRequirements must also
			 * be satisfied.
			 */
            originalRequirement.result implies allTrue(derivedRequirements.result);
        }
    }
    abstract connection def derivations : Derivation {
        doc
        /* derivations is the base feature for Derivation connection usages. */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 173) (line 7) (column 17) (len 27)) (segments (segment 0 (token "SequenceFunctions") (name "SequenceFunctions") (separator none) (span (offset 173) (line 7) (column 17) (len 17))) (segment 1 (token "excludes") (name "excludes") (separator colon-colon) (span (offset 192) (line 7) (column 36) (len 8)))))
    (reference r1 (scope relative) (span (offset 218) (line 8) (column 17) (len 25)) (segments (segment 0 (token "ControlFunctions") (name "ControlFunctions") (separator none) (span (offset 218) (line 8) (column 17) (len 16))) (segment 1 (token "allTrue") (name "allTrue") (separator colon-colon) (span (offset 236) (line 8) (column 35) (len 7)))))
    (reference r2 (scope relative) (span (offset 1457) (line 36) (column 46) (len 20)) (segments (segment 0 (token "originalRequirements") (name "originalRequirements") (separator none) (span (offset 1457) (line 36) (column 46) (len 20)))))
    (reference r3 (scope relative) (span (offset 1481) (line 36) (column 70) (len 11)) (segments (segment 0 (token "participant") (name "participant") (separator none) (span (offset 1481) (line 36) (column 70) (len 11)))))
    (reference r4 (scope relative) (span (offset 1567) (line 39) (column 23) (len 19)) (segments (segment 0 (token "derivedRequirements") (name "derivedRequirements") (separator none) (span (offset 1567) (line 39) (column 23) (len 19)))))
    (reference r5 (scope relative) (span (offset 1596) (line 39) (column 52) (len 11)) (segments (segment 0 (token "participant") (name "participant") (separator none) (span (offset 1596) (line 39) (column 52) (len 11)))))
    (reference r6 (scope relative) (span (offset 2207) (line 60) (column 36) (len 10)) (segments (segment 0 (token "Derivation") (name "Derivation") (separator none) (span (offset 2207) (line 60) (column 36) (len 10)))))
  )
  (root (library-package (name "DerivationConnections") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 57) (line 3) (column 4) (len 94)) (normalized "This package provides a library model for derivation connections between requirements.\n"))) (import (target (span (span (offset 173) (line 7) (column 17) (len 27))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 218) (line 8) (column 17) (len 25))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (requirement-usage (name "originalRequirements") (multiplicity (lower unbounded) (upper unbounded))) (requirement-usage (name "derivedRequirements") (multiplicity (lower unbounded) (upper unbounded))) (connection-def (name "Derivation") (modifiers (abstract (span (offset 510) (line 17) (column 2) (len 8)))) (role ordinary) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 557) (line 19) (column 5) (len 551)) (normalized "A Derivation connection asserts that one or more derivedRequirements are derived from\na single originalRequirement. This means that any subject that satisfies the\noriginalRequirement should, in itself or though other things related to it, satisfy\neach of the derivedRequirements.\n\nA connection usage typed by Derivation must have requirement usages for all its ends.\nThe single end for the originalRequirement should subset originalRequirement, while\nthe rest of the ends should subset derivedRequirements.\n"))) (ref (name "originalRequirement") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind requirement) (typing none) (multiplicity (lower (expression (span (offset 1450) (line 36) (column 39) (len 1)) (integer 1))) (upper (expression (span (offset 1450) (line 36) (column 39) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r2)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r3)))) (body brace (doc (name none) (locale none) (body (span (offset 1504) (line 37) (column 10) (len 34)) (normalized "The single original requirement. "))))) (ref (name "") (short-name none) (prefix (direction none) (derived false) (usage-prefix none) (constant false)) (kind requirement) (typing none) (multiplicity (lower (expression (span (offset 1587) (line 39) (column 43) (len 1)) (integer 1))) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r5)))) (body brace (doc (name none) (locale none) (body (span (offset 1619) (line 40) (column 10) (len 78)) (normalized "The one or more requirements that are derived from the original requirement. "))))) (assert-constraint) (assert-constraint))) (connection-def (name "derivations") (modifiers (abstract (span (offset 2173) (line 60) (column 2) (len 8)))) (role ordinary) (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (body brace (doc (name none) (locale none) (body (span (offset 2231) (line 61) (column 9) (len 67)) (normalized "derivations is the base feature for Derivation connection usages. "))))))))
)
~~~
