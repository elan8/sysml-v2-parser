# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (15-Properties-Values-Expressions): 15_11-Variable Length Collection Types"))
~~~
# SOURCE
~~~sysml
package '15_11-Variable Length Collection Types' {
	private import ScalarValues::*;
	private import Collections::*;
	
	part def SparePart;
	part def Person;
	
	/* Examples of declaring syntactic sugar-like names for instantiating collection types. */
	
	attribute def 'Bag<SparePart>' :> Bag {
		ref part :>> elements: SparePart;
	}
	
	attribute def 'List<Integer>' :> List {
		value :>> elements: Integer;
	}
	
	attribute def 'Set<String>' :> Set {
		attribute :>> elements: String;
	}
	
	attribute def 'OrderedSet<Person>' :> OrderedSet {
		ref part :>> elements: Person;
	}
	
	attribute def 'List<Set<Person>>' :> List {
		attribute :>> elements: Set {
			ref part :>> elements: Person;
		}
	}
	
	attribute def 'Array<Real>[4]' :> Array {
		attribute :>> elements: Real;
		attribute :>> dimensions = 4;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_11_variable_length_collection_types.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_11-Variable Length Collection Types' {
    private import ScalarValues::*;
    private import Collections::*;
    part def SparePart;
    part def Person;
    attribute def 'Bag<SparePart>' :> Bag {
        ref part :>> elements : SparePart;
    }
    attribute def 'List<Integer>' :> List {
        attribute :>> elements : Integer;
    }
    attribute def 'Set<String>' :> Set {
        attribute :>> elements : String;
    }
    attribute def 'OrderedSet<Person>' :> OrderedSet {
        ref part :>> elements : Person;
    }
    attribute def 'List<Set<Person>>' :> List {
        attribute :>> elements : Set {
            ref part :>> elements : Person;
        }
    }
    attribute def 'Array<Real>[4]' :> Array {
        attribute :>> elements : Real;
        attribute :>> dimensions = 4;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 67) (line 2) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 67) (line 2) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 100) (line 3) (column 17) (len 11)) (segments (segment 0 (token "Collections") (name "Collections") (separator none) (span (offset 100) (line 3) (column 17) (len 11)))))
  )
  (root (package (name "15_11-Variable Length Collection Types") (body brace (import (target (span (span (offset 67) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 79) (line 2) (column 29) (len 3))) (separator (span (offset 79) (line 2) (column 29) (len 2))) (marker (span (offset 81) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 100) (line 3) (column 17) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 111) (line 3) (column 28) (len 3))) (separator (span (offset 111) (line 3) (column 28) (len 2))) (marker (span (offset 113) (line 3) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "SparePart") (body semicolon)) (part-def (name "Person") (body semicolon)) (attribute-def (name "Bag<SparePart>") (multiplicity none)) (attribute-def (name "List<Integer>") (multiplicity none)) (attribute-def (name "Set<String>") (multiplicity none)) (attribute-def (name "OrderedSet<Person>") (multiplicity none)) (attribute-def (name "List<Set<Person>>") (multiplicity none)) (attribute-def (name "Array<Real>[4]") (multiplicity none)))))
)
~~~
