# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Systems Library/Allocations"))
~~~
# SOURCE
~~~sysml
standard library package Allocations {
	doc
	/*
	 * This package defines the base types for allocations and related structural elements
	 * in the SysML language.
	 */

	private import Base::Anything;
	private import Connections::*;

	allocation def Allocation :> BinaryConnection {
		doc
		/*
		 * Allocation is the most general class of allocation, represented as a connection 
		 * between the source of the allocation and the target. Allocation is the base type 
		 * of all AllocationDefinitions.
		 */
	
		end source: Anything :>> BinaryConnection::source;
		end target: Anything :>> BinaryConnection::target;
	}
	
	abstract allocation allocations: Allocation[0..*] nonunique :> binaryConnections {
		doc
		/*
		 * allocations is the base feature of all AllocationUsages.
		 */
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "allocations.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Allocations {
    doc
    /*
	 * This package defines the base types for allocations and related structural elements
	 * in the SysML language.
	 */
    private import Base::Anything;
    private import Connections::*;
    allocation def Allocation :> BinaryConnection {
        doc
        /*
		 * Allocation is the most general class of allocation, represented as a connection 
		 * between the source of the allocation and the target. Allocation is the base type 
		 * of all AllocationDefinitions.
		 */
        end source : Anything :>> BinaryConnection::source;
        end target : Anything :>> BinaryConnection::target;
    }
    allocation allocations : Allocation {
        doc
        /*
		 * allocations is the base feature of all AllocationUsages.
		 */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (name-language sysml)
  (references
    (reference r0 (scope relative) (span (offset 185) (line 8) (column 17) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 185) (line 8) (column 17) (len 4))) (segment 1 (token "Anything") (name "Anything") (separator colon-colon) (span (offset 191) (line 8) (column 23) (len 8)))))
    (reference r1 (scope relative) (span (offset 217) (line 9) (column 17) (len 11)) (segments (segment 0 (token "Connections") (name "Connections") (separator none) (span (offset 217) (line 9) (column 17) (len 11)))))
    (reference r2 (scope relative) (span (offset 524) (line 19) (column 15) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 524) (line 19) (column 15) (len 8)))))
    (reference r3 (scope relative) (span (offset 537) (line 19) (column 28) (len 24)) (segments (segment 0 (token "BinaryConnection") (name "BinaryConnection") (separator none) (span (offset 537) (line 19) (column 28) (len 16))) (segment 1 (token "source") (name "source") (separator colon-colon) (span (offset 555) (line 19) (column 46) (len 6)))))
    (reference r4 (scope relative) (span (offset 577) (line 20) (column 15) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 577) (line 20) (column 15) (len 8)))))
    (reference r5 (scope relative) (span (offset 590) (line 20) (column 28) (len 24)) (segments (segment 0 (token "BinaryConnection") (name "BinaryConnection") (separator none) (span (offset 590) (line 20) (column 28) (len 16))) (segment 1 (token "target") (name "target") (separator colon-colon) (span (offset 608) (line 20) (column 46) (len 6)))))
    (reference r6 (scope relative) (span (offset 655) (line 23) (column 35) (len 10)) (segments (segment 0 (token "Allocation") (name "Allocation") (separator none) (span (offset 655) (line 23) (column 35) (len 10)))))
  )
  (root (library-package (visibility none) (name "Allocations") (standard true) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 47) (line 3) (column 4) (len 118)) (normalized "This package defines the base types for allocations and related structural elements\nin the SysML language.\n"))) (import (visibility private) (target (span (span (offset 185) (line 8) (column 17) (len 14))) (all none) (ref r0) (shape (membership (recursive-suffix none)))) (body semicolon)) (import (visibility private) (target (span (span (offset 217) (line 9) (column 17) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 228) (line 9) (column 28) (len 3))) (separator (span (offset 228) (line 9) (column 28) (len 2))) (marker (span (offset 230) (line 9) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none)))) (body semicolon)) (allocation-def (name "Allocation") (modifiers) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 293) (line 13) (column 5) (len 212)) (normalized "Allocation is the most general class of allocation, represented as a connection \nbetween the source of the allocation and the target. Allocation is the base type \nof all AllocationDefinitions.\n"))) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "source") (span (offset 516) (line 19) (column 7) (len 6)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (references none) (multiplicity none) (redefines (relationship (kind redefines) (implied false) (targets (ref r3)))) (crosses none)) (end (prefix (direction none) (derived false) (constant false) (variance none)) (introducer bare) (extensions) (short-name none) (identity (declaration (name "target") (span (offset 569) (line 20) (column 7) (len 6)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (references none) (multiplicity none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (crosses none)))) (allocation-usage (name "allocations") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (ends) (body brace (doc (name none) (short-name none) (locale none) (body (span (offset 715) (line 25) (column 5) (len 66)) (normalized "allocations is the base feature of all AllocationUsages.\n"))))))))
)
~~~
