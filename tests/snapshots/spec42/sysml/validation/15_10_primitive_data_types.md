# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (15-Properties-Values-Expressions): 15_10-Primitive Data Types"))
~~~
# SOURCE
~~~sysml
package '15.10-Primitive Data Types' {
	/*
	 * Primitive data types are defined in normative model libraries.
	 * Any more specialized data types can be declared in user-defined 
	 * model libraries or models as needed.
	 */
	 
	private import ScalarValues::Integer {
	doc
	/*
	 * The unqualified Integer is signed, in line with integer numbers in mathematics.
	 */
	}
	
	private import ScalarValues::Natural;
	attribute def UnsignedInteger :> Natural {
		doc /* Mathematically, unsigned integers are just natural numbers (non-negative integers). */		
	}
	
	private import ScalarValues::Real {
	doc
	/*
	 * The unqualified Real is signed, in line with real numbers in mathematics.
	 */
	}
	
	attribute def UnsignedReal :> Real {
		doc
		/*
		 * Example of restriction of the base Real datatype.
		 */
		attribute x: Real :>> self;
		assert constraint { x >= 0.0 }
	}
	
	private import ScalarValues::String {
		doc
		/*
		 * String attributes are sequences of characters.
		 */
	}
	
	private import ScalarValues::Boolean {
		doc
		/*
		 * Boolean type has two legal attributes: true, false.
		 */
	}
	
	private import Time::DateTime;
	
	enum def ConditionColor {
		doc
		/*
		 * Enumerations are defined as an implicit restriction of the extent of the
		 * enumeration type to the listed enumeration values.
		 * Note: Enumerations are currently limited to attributes.
		 */
	
		enum red;
		enum yellow;
		enum green;
	}
	
	attribute def ConditionLevel {
		attribute associatedColor : ConditionColor;
	}
	
	enum def SeverityEnum :> ConditionLevel {
		danger { 
			:>> associatedColor = ConditionColor::red;
		}
		warning { 
			:>> associatedColor = ConditionColor::yellow;
		}
		normal { 
			:>> associatedColor = ConditionColor::green;
		}
	}
	
	attribute def Diameter :> ISQ::LengthValue;	
	enum def DiameterChoice :> Diameter {
		small = 60 [SI::mm];
		medium = 70 [SI::mm];
		large = 80 [SI::mm];
	}	
	attribute aperatureDiameter: DiameterChoice = DiameterChoice::small;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_10_primitive_data_types.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15.10-Primitive Data Types' {
    /*
	 * Primitive data types are defined in normative model libraries.
	 * Any more specialized data types can be declared in user-defined 
	 * model libraries or models as needed.
	 */
    private import ScalarValues::Integer {
        doc
        /*
	 * The unqualified Integer is signed, in line with integer numbers in mathematics.
	 */
    }
    private import ScalarValues::Natural;
    attribute def UnsignedInteger :> Natural {
        doc
        /* Mathematically, unsigned integers are just natural numbers (non-negative integers). */
    }
    private import ScalarValues::Real {
        doc
        /*
	 * The unqualified Real is signed, in line with real numbers in mathematics.
	 */
    }
    attribute def UnsignedReal :> Real {
        doc
        /*
		 * Example of restriction of the base Real datatype.
		 */
        attribute x : Real :>> self;
        assert constraint {
            x >= 0.0;
        }
    }
    private import ScalarValues::String {
        doc
        /*
		 * String attributes are sequences of characters.
		 */
    }
    private import ScalarValues::Boolean {
        doc
        /*
		 * Boolean type has two legal attributes: true, false.
		 */
    }
    private import Time::DateTime;
    enum def ConditionColor {
        doc
        /*
		 * Enumerations are defined as an implicit restriction of the extent of the
		 * enumeration type to the listed enumeration values.
		 * Note: Enumerations are currently limited to attributes.
		 */
        red;
        yellow;
        green;
    }
    attribute def ConditionLevel {
        attribute associatedColor : ConditionColor;
    }
    enum def SeverityEnum :> ConditionLevel {
        danger {
            attribute :>> associatedColor = ConditionColor::red;
        }
        warning {
            attribute :>> associatedColor = ConditionColor::yellow;
        }
        normal {
            attribute :>> associatedColor = ConditionColor::green;
        }
    }
    attribute def Diameter :> ISQ::LengthValue;
    enum def DiameterChoice :> Diameter {
        small = 60[SI::mm];
        medium = 70[SI::mm];
        large = 80[SI::mm];
    }
    attribute aperatureDiameter : DiameterChoice = DiameterChoice::small;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 244) (line 8) (column 17) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 244) (line 8) (column 17) (len 12))) (segment 1 (token "Integer") (name "Integer") (separator colon-colon) (span (offset 258) (line 8) (column 31) (len 7)))))
    (reference r1 (scope relative) (span (offset 387) (line 15) (column 17) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 387) (line 15) (column 17) (len 12))) (segment 1 (token "Natural") (name "Natural") (separator colon-colon) (span (offset 401) (line 15) (column 31) (len 7)))))
    (reference r2 (scope relative) (span (offset 444) (line 16) (column 35) (len 7)) (segments (segment 0 (token "Natural") (name "Natural") (separator none) (span (offset 444) (line 16) (column 35) (len 7)))))
    (reference r3 (scope relative) (span (offset 573) (line 20) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 573) (line 20) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 587) (line 20) (column 31) (len 4)))))
    (reference r4 (scope relative) (span (offset 722) (line 27) (column 32) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 722) (line 27) (column 32) (len 4)))))
    (reference r5 (scope relative) (span (offset 816) (line 32) (column 16) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 816) (line 32) (column 16) (len 4)))))
    (reference r6 (scope relative) (span (offset 825) (line 32) (column 25) (len 4)) (segments (segment 0 (token "self") (name "self") (separator none) (span (offset 825) (line 32) (column 25) (len 4)))))
    (reference r7 (scope relative) (span (offset 885) (line 36) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 885) (line 36) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 899) (line 36) (column 31) (len 6)))))
    (reference r8 (scope relative) (span (offset 998) (line 43) (column 17) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 998) (line 43) (column 17) (len 12))) (segment 1 (token "Boolean") (name "Boolean") (separator colon-colon) (span (offset 1012) (line 43) (column 31) (len 7)))))
    (reference r9 (scope relative) (span (offset 1117) (line 50) (column 17) (len 14)) (segments (segment 0 (token "Time") (name "Time") (separator none) (span (offset 1117) (line 50) (column 17) (len 4))) (segment 1 (token "DateTime") (name "DateTime") (separator colon-colon) (span (offset 1123) (line 50) (column 23) (len 8)))))
    (reference r10 (scope relative) (span (offset 1484) (line 66) (column 31) (len 14)) (segments (segment 0 (token "ConditionColor") (name "ConditionColor") (separator none) (span (offset 1484) (line 66) (column 31) (len 14)))))
    (reference r11 (scope relative) (span (offset 1567) (line 71) (column 8) (len 15)) (segments (segment 0 (token "associatedColor") (name "associatedColor") (separator none) (span (offset 1567) (line 71) (column 8) (len 15)))))
    (reference r12 (scope relative) (span (offset 1585) (line 71) (column 26) (len 19)) (segments (segment 0 (token "ConditionColor") (name "ConditionColor") (separator none) (span (offset 1585) (line 71) (column 26) (len 14))) (segment 1 (token "red") (name "red") (separator colon-colon) (span (offset 1601) (line 71) (column 42) (len 3)))))
    (reference r13 (scope relative) (span (offset 1630) (line 74) (column 8) (len 15)) (segments (segment 0 (token "associatedColor") (name "associatedColor") (separator none) (span (offset 1630) (line 74) (column 8) (len 15)))))
    (reference r14 (scope relative) (span (offset 1648) (line 74) (column 26) (len 22)) (segments (segment 0 (token "ConditionColor") (name "ConditionColor") (separator none) (span (offset 1648) (line 74) (column 26) (len 14))) (segment 1 (token "yellow") (name "yellow") (separator colon-colon) (span (offset 1664) (line 74) (column 42) (len 6)))))
    (reference r15 (scope relative) (span (offset 1695) (line 77) (column 8) (len 15)) (segments (segment 0 (token "associatedColor") (name "associatedColor") (separator none) (span (offset 1695) (line 77) (column 8) (len 15)))))
    (reference r16 (scope relative) (span (offset 1713) (line 77) (column 26) (len 21)) (segments (segment 0 (token "ConditionColor") (name "ConditionColor") (separator none) (span (offset 1713) (line 77) (column 26) (len 14))) (segment 1 (token "green") (name "green") (separator colon-colon) (span (offset 1729) (line 77) (column 42) (len 5)))))
    (reference r17 (scope relative) (span (offset 1772) (line 81) (column 28) (len 16)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 1772) (line 81) (column 28) (len 3))) (segment 1 (token "LengthValue") (name "LengthValue") (separator colon-colon) (span (offset 1777) (line 81) (column 33) (len 11)))))
    (reference r18 (scope relative) (span (offset 1844) (line 83) (column 15) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 1844) (line 83) (column 15) (len 2))) (segment 1 (token "mm") (name "mm") (separator colon-colon) (span (offset 1848) (line 83) (column 19) (len 2)))))
    (reference r19 (scope relative) (span (offset 1868) (line 84) (column 16) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 1868) (line 84) (column 16) (len 2))) (segment 1 (token "mm") (name "mm") (separator colon-colon) (span (offset 1872) (line 84) (column 20) (len 2)))))
    (reference r20 (scope relative) (span (offset 1891) (line 85) (column 15) (len 6)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 1891) (line 85) (column 15) (len 2))) (segment 1 (token "mm") (name "mm") (separator colon-colon) (span (offset 1895) (line 85) (column 19) (len 2)))))
  )
  (root (package (name "15.10-Primitive Data Types") (body brace (comment (keyword none) (name none) (about) (locale none) (body (span (offset 42) (line 2) (column 4) (len 180)) (normalized "Primitive data types are defined in normative model libraries.\nAny more specialized data types can be declared in user-defined \nmodel libraries or models as needed.\n"))) (import (target (span (span (offset 244) (line 8) (column 17) (len 21))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 387) (line 15) (column 17) (len 21))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (attribute-def (declaration-name "UnsignedInteger") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 462) (line 17) (column 9) (len 85)) (normalized "Mathematically, unsigned integers are just natural numbers (non-negative integers). "))))) (import (target (span (span (offset 573) (line 20) (column 17) (len 18))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (attribute-def (declaration-name "UnsignedReal") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 739) (line 29) (column 5) (len 59)) (normalized "Example of restriction of the base Real datatype.\n"))) (attribute-usage (declaration-name "x") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r6)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (assert-constraint))) (import (target (span (span (offset 885) (line 36) (column 17) (len 20))) (all none) (ref r7) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 998) (line 43) (column 17) (len 21))) (all none) (ref r8) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1117) (line 50) (column 17) (len 14))) (all none) (ref r9) (shape (membership (recursive-suffix none))))) (enum-def (name "ConditionColor") (body brace (doc (name none) (locale none) (body (span (offset 1172) (line 54) (column 5) (len 199)) (normalized "Enumerations are defined as an implicit restriction of the extent of the\nenumeration type to the listed enumeration values.\nNote: Enumerations are currently limited to attributes.\n"))) (enum-value (name "red") (short-name none) (value none) (body semicolon) (span (offset 1378) (line 60) (column 3) (len 9))) (enum-value (name "yellow") (short-name none) (value none) (body semicolon) (span (offset 1390) (line 61) (column 3) (len 12))) (enum-value (name "green") (short-name none) (value none) (body semicolon) (span (offset 1405) (line 62) (column 3) (len 11))))) (attribute-def (declaration-name "ConditionLevel") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "associatedColor") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (enum-def (name "SeverityEnum") (body brace (enum-value (name "danger") (short-name none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1585) (line 71) (column 26) (len 19)) (ref r12))))) (body semicolon))) (span (offset 1550) (line 70) (column 3) (len 59))) (enum-value (name "warning") (short-name none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1648) (line 74) (column 26) (len 22)) (ref r14))))) (body semicolon))) (span (offset 1612) (line 73) (column 3) (len 63))) (enum-value (name "normal") (short-name none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1713) (line 77) (column 26) (len 21)) (ref r16))))) (body semicolon))) (span (offset 1678) (line 76) (column 3) (len 61))))) (attribute-def (declaration-name "Diameter") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (enum-def (name "DiameterChoice") (body brace (enum-value (name "small") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1840) (line 83) (column 11) (len 11)) (bracket (base (expression (span (offset 1840) (line 83) (column 11) (len 2)) (integer 60))) (operands (sequence-list (element first (expression (span (offset 1844) (line 83) (column 15) (len 6)) (ref r18)))))))))) (body semicolon) (span (offset 1832) (line 83) (column 3) (len 20))) (enum-value (name "medium") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1864) (line 84) (column 12) (len 11)) (bracket (base (expression (span (offset 1864) (line 84) (column 12) (len 2)) (integer 70))) (operands (sequence-list (element first (expression (span (offset 1868) (line 84) (column 16) (len 6)) (ref r19)))))))))) (body semicolon) (span (offset 1855) (line 84) (column 3) (len 21))) (enum-value (name "large") (short-name none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1887) (line 85) (column 11) (len 11)) (bracket (base (expression (span (offset 1887) (line 85) (column 11) (len 2)) (integer 80))) (operands (sequence-list (element first (expression (span (offset 1891) (line 85) (column 15) (len 6)) (ref r20)))))))))) (body semicolon) (span (offset 1879) (line 85) (column 3) (len 20))))) (attribute-usage))))
)
~~~
