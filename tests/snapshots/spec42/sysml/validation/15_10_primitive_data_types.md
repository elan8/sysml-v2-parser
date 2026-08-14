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
        assert constraint  {
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
        red;
        yellow;
        green;
    }
    attribute def ConditionLevel {
        attribute associatedColor : ConditionColor;
    }
    enum def SeverityEnum :> ConditionLevel {
        danger;
        warning;
        normal;
    }
    attribute def Diameter :> ISQ::LengthValue;
    enum def DiameterChoice :> Diameter {
        small;
        medium;
        large;
    }
    attribute def aperatureDiameter : DiameterChoice = DiameterChoice::small;
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 244) (line 8) (column 17) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 244) (line 8) (column 17) (len 12))) (segment 1 (token "Integer") (name "Integer") (separator colon-colon) (span (offset 258) (line 8) (column 31) (len 7)))))
    (reference r1 (scope relative) (span (offset 387) (line 15) (column 17) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 387) (line 15) (column 17) (len 12))) (segment 1 (token "Natural") (name "Natural") (separator colon-colon) (span (offset 401) (line 15) (column 31) (len 7)))))
    (reference r2 (scope relative) (span (offset 573) (line 20) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 573) (line 20) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 587) (line 20) (column 31) (len 4)))))
    (reference r3 (scope relative) (span (offset 885) (line 36) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 885) (line 36) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 899) (line 36) (column 31) (len 6)))))
    (reference r4 (scope relative) (span (offset 998) (line 43) (column 17) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 998) (line 43) (column 17) (len 12))) (segment 1 (token "Boolean") (name "Boolean") (separator colon-colon) (span (offset 1012) (line 43) (column 31) (len 7)))))
    (reference r5 (scope relative) (span (offset 1117) (line 50) (column 17) (len 14)) (segments (segment 0 (token "Time") (name "Time") (separator none) (span (offset 1117) (line 50) (column 17) (len 4))) (segment 1 (token "DateTime") (name "DateTime") (separator colon-colon) (span (offset 1123) (line 50) (column 23) (len 8)))))
  )
  (root (package (name "15.10-Primitive Data Types") (body (import (target (span (span (offset 244) (line 8) (column 17) (len 22))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 387) (line 15) (column 17) (len 21))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (attribute-def) (import (target (span (span (offset 573) (line 20) (column 17) (len 19))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (attribute-def) (import (target (span (span (offset 885) (line 36) (column 17) (len 21))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 998) (line 43) (column 17) (len 22))) (all none) (ref r4) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1117) (line 50) (column 17) (len 14))) (all none) (ref r5) (shape (membership (recursive-suffix none))))) (enum-def) (attribute-def) (enum-def) (attribute-def) (enum-def) (attribute-def))))
)
~~~
