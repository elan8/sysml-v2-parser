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
    requirement originalRequirements {
        doc
        /* originalRequirements are the original requirements in Derivation connections. */
    }
    requirement derivedRequirements {
        doc
        /* derivedRequirements are the derived requirments in Derivation connections. */
    }
    connection def Derivation {
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
        ref originalRequirement :>> originalRequirements :> participant {
            doc
            /* The single original requirement. */
        }
        ref '' :>> derivedRequirements :> participant {
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
    connection def derivations : Derivation {
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
    (reference r2 (scope relative) (span (offset 2207) (line 60) (column 36) (len 10)) (segments (segment 0 (token "Derivation") (name "Derivation") (separator none) (span (offset 2207) (line 60) (column 36) (len 10)))))
  )
  (root (library-package (name "DerivationConnections") (standard true) (body (doc) (import (target (span (span (offset 173) (line 7) (column 17) (len 27))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 218) (line 8) (column 17) (len 25))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (requirement-usage) (requirement-usage) (connection-def (name "Derivation") (role ordinary) (specializes none) (body (doc) (ref) (ref) (assert-constraint) (assert-constraint))) (connection-def (name "derivations") (role ordinary) (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (body (doc))))))
)
~~~
