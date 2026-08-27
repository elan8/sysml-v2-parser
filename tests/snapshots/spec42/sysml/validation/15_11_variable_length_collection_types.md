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
    /* Examples of declaring syntactic sugar-like names for instantiating collection types. */
    attribute def 'Bag<SparePart>' :> Bag {
        ref part :>> elements : SparePart;
    }
    attribute def 'List<Integer>' :> List {
        value : Integer :>> elements;
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
    (reference r2 (scope relative) (span (offset 288) (line 10) (column 36) (len 3)) (segments (segment 0 (token "Bag") (name "Bag") (separator none) (span (offset 288) (line 10) (column 36) (len 3)))))
    (reference r3 (scope relative) (span (offset 319) (line 11) (column 26) (len 9)) (segments (segment 0 (token "SparePart") (name "SparePart") (separator none) (span (offset 319) (line 11) (column 26) (len 9)))))
    (reference r4 (scope relative) (span (offset 309) (line 11) (column 16) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 309) (line 11) (column 16) (len 8)))))
    (reference r5 (scope relative) (span (offset 369) (line 14) (column 35) (len 4)) (segments (segment 0 (token "List") (name "List") (separator none) (span (offset 369) (line 14) (column 35) (len 4)))))
    (reference r6 (scope relative) (span (offset 398) (line 15) (column 23) (len 7)) (segments (segment 0 (token "Integer") (name "Integer") (separator none) (span (offset 398) (line 15) (column 23) (len 7)))))
    (reference r7 (scope relative) (span (offset 388) (line 15) (column 13) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 388) (line 15) (column 13) (len 8)))))
    (reference r8 (scope relative) (span (offset 444) (line 18) (column 33) (len 3)) (segments (segment 0 (token "Set") (name "Set") (separator none) (span (offset 444) (line 18) (column 33) (len 3)))))
    (reference r9 (scope relative) (span (offset 476) (line 19) (column 27) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 476) (line 19) (column 27) (len 6)))))
    (reference r10 (scope relative) (span (offset 466) (line 19) (column 17) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 466) (line 19) (column 17) (len 8)))))
    (reference r11 (scope relative) (span (offset 528) (line 22) (column 40) (len 10)) (segments (segment 0 (token "OrderedSet") (name "OrderedSet") (separator none) (span (offset 528) (line 22) (column 40) (len 10)))))
    (reference r12 (scope relative) (span (offset 566) (line 23) (column 26) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 566) (line 23) (column 26) (len 6)))))
    (reference r13 (scope relative) (span (offset 556) (line 23) (column 16) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 556) (line 23) (column 16) (len 8)))))
    (reference r14 (scope relative) (span (offset 617) (line 26) (column 39) (len 4)) (segments (segment 0 (token "List") (name "List") (separator none) (span (offset 617) (line 26) (column 39) (len 4)))))
    (reference r15 (scope relative) (span (offset 650) (line 27) (column 27) (len 3)) (segments (segment 0 (token "Set") (name "Set") (separator none) (span (offset 650) (line 27) (column 27) (len 3)))))
    (reference r16 (scope relative) (span (offset 640) (line 27) (column 17) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 640) (line 27) (column 17) (len 8)))))
    (reference r17 (scope relative) (span (offset 682) (line 28) (column 27) (len 6)) (segments (segment 0 (token "Person") (name "Person") (separator none) (span (offset 682) (line 28) (column 27) (len 6)))))
    (reference r18 (scope relative) (span (offset 672) (line 28) (column 17) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 672) (line 28) (column 17) (len 8)))))
    (reference r19 (scope relative) (span (offset 734) (line 32) (column 36) (len 5)) (segments (segment 0 (token "Array") (name "Array") (separator none) (span (offset 734) (line 32) (column 36) (len 5)))))
    (reference r20 (scope relative) (span (offset 768) (line 33) (column 27) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 768) (line 33) (column 27) (len 4)))))
    (reference r21 (scope relative) (span (offset 758) (line 33) (column 17) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 758) (line 33) (column 17) (len 8)))))
    (reference r22 (scope relative) (span (offset 790) (line 34) (column 17) (len 10)) (segments (segment 0 (token "dimensions") (name "dimensions") (separator none) (span (offset 790) (line 34) (column 17) (len 10)))))
  )
  (root (package (name "15_11-Variable Length Collection Types") (body brace (import (target (span (span (offset 67) (line 2) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 79) (line 2) (column 29) (len 3))) (separator (span (offset 79) (line 2) (column 29) (len 2))) (marker (span (offset 81) (line 2) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 100) (line 3) (column 17) (len 14))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 111) (line 3) (column 28) (len 3))) (separator (span (offset 111) (line 3) (column 28) (len 2))) (marker (span (offset 113) (line 3) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-def (name "SparePart") (modifiers) (body semicolon)) (part-def (name "Person") (modifiers) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 162) (line 8) (column 4) (len 86)) (normalized "Examples of declaring syntactic sugar-like names for instantiating collection types. "))) (attribute-def (declaration-name "Bag<SparePart>") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r4)))) (value none) (body semicolon)))) (attribute-def (declaration-name "List<Integer>") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name "value") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "Set<String>") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "OrderedSet<Person>") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (value none) (body semicolon)))) (attribute-def (declaration-name "List<Set<Person>>") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (references none) (crosses none) (intersects none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference true) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r18)))) (value none) (body semicolon)))))) (attribute-def (declaration-name "Array<Real>[4]") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r21)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 803) (line 34) (column 30) (len 1)) (integer 4))))) (body semicolon)))))))
)
~~~
