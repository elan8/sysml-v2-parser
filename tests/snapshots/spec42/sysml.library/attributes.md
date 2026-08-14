# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Systems Library/Attributes"))
~~~
# SOURCE
~~~sysml
standard library package Attributes {
    doc /*
 * This package defines the base types for attributes and related structural elements 
 * in the SysML language.
 */

    private import Base::DataValue;
    private import Base::dataValues;

    alias AttributeValue for DataValue {
        doc /*
		 * AttributeValue is the most general type of data values that represent qualities or characteristics 
		 * of a system or part of a system. AttributeValue is the base type of all AttributeDefinitions.
		 */
    }

    alias attributeValues for dataValues {
        doc /*
		 * attributeValues is the base feature for all AttributeUsages.
		 */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "attributes.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Attributes {
    doc
    /*
 * This package defines the base types for attributes and related structural elements 
 * in the SysML language.
 */
    private import Base::DataValue;
    private import Base::dataValues;
    alias AttributeValue for DataValue {
        doc
        /*
		 * AttributeValue is the most general type of data values that represent qualities or characteristics 
		 * of a system or part of a system. AttributeValue is the base type of all AttributeDefinitions.
		 */
    }
    alias attributeValues for dataValues {
        doc
        /*
		 * attributeValues is the base feature for all AttributeUsages.
		 */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 186) (line 7) (column 20) (len 15)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 186) (line 7) (column 20) (len 4))) (segment 1 (token "DataValue") (name "DataValue") (separator colon-colon) (span (offset 192) (line 7) (column 26) (len 9)))))
    (reference r1 (scope relative) (span (offset 222) (line 8) (column 20) (len 16)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 222) (line 8) (column 20) (len 4))) (segment 1 (token "dataValues") (name "dataValues") (separator colon-colon) (span (offset 228) (line 8) (column 26) (len 10)))))
    (reference r2 (scope relative) (span (offset 270) (line 10) (column 30) (len 9)) (segments (segment 0 (token "DataValue") (name "DataValue") (separator none) (span (offset 270) (line 10) (column 30) (len 9)))))
    (reference r3 (scope relative) (span (offset 544) (line 17) (column 31) (len 10)) (segments (segment 0 (token "dataValues") (name "dataValues") (separator none) (span (offset 544) (line 17) (column 31) (len 10)))))
  )
  (root (library-package (name "Attributes") (standard true) (body (doc) (import (target (span (span (offset 186) (line 7) (column 20) (len 15))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 222) (line 8) (column 20) (len 16))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (alias (name "AttributeValue") (target (ref r2)) (body brace (element-count 1))) (alias (name "attributeValues") (target (ref r3)) (body brace (element-count 1))))))
)
~~~
