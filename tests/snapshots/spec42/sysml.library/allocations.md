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
  (references
    (reference r0 (scope relative) (span (offset 185) (line 8) (column 17) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 185) (line 8) (column 17) (len 4))) (segment 1 (token "Anything") (name "Anything") (separator colon-colon) (span (offset 191) (line 8) (column 23) (len 8)))))
    (reference r1 (scope relative) (span (offset 217) (line 9) (column 17) (len 11)) (segments (segment 0 (token "Connections") (name "Connections") (separator none) (span (offset 217) (line 9) (column 17) (len 11)))))
  )
  (root (library-package (name "Allocations") (standard true) (body brace (doc) (import (target (span (span (offset 185) (line 8) (column 17) (len 14))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 217) (line 9) (column 17) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 228) (line 9) (column 28) (len 3))) (separator (span (offset 228) (line 9) (column 28) (len 2))) (marker (span (offset 230) (line 9) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (allocation-def (name "Allocation") (modifiers)) (allocation-usage))))
)
~~~
