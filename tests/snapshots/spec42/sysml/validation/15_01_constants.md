# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (15-Properties-Values-Expressions): 15_01-Constants"))
~~~
# SOURCE
~~~sysml
package '15_01-Constants' {
    private import MeasurementReferences::*;
    private import SI::*;
    private import RealFunctions::*;

    /* Note: Value properties that are bound to specific values are constants and have the specified
     * values in all contexts. It is not legal to redefine them.
     */    
    
    package 'Mathematical Constants' {
	    doc
	    /*
	     * Standard mathematical constants
	     * 
	     * Irrational constants cannot be represented exactly with finite precision.
	     * However, they can be required to be implemented with a attribute that is accurate
	     * to at least a certain precision.
	     * 
	     * (The decimal literals here should be interpreted as being fixed point and exact.)
	     */
    
        attribute e: Real {
        	assert constraint { round(e * 1E20) == 271828182845904523536.0 }
        }
        attribute pi: Real {
        	assert constraint { round(pi * 1E20) == 314159265358979323846.0 }
        }
    }

    package 'Fundamental Physical Constants' {
	    doc
	    /*
	     * Standard fundamental physical constants
	     * 
	     * Physical constants have a standard measured attribute to a finite precision.
	     *
	     * The reference source is:
	     * CODATA - Task Group on Fundamental Physical Constants (TGFC) - 2018 CODATA recommended values
	     * See https://codata.org/initiatives/strategic-programme/fundamental-physical-constants/
	     * For the actual values see https://pml.nist.gov/cuu/Constants/ 
	     */
    
        attribute 'fine structure constant'      : DimensionOneValue = 7.2973525693E-3[one];  // 2018 CODATA attribute 7.2973525693E-3;  uncertainty = 0.0000000011E-3
        attribute 'electron to proton mass ratio': DimensionOneValue = 5.44617021487E-4[one]; // 2018 CODATA attribute 5.44617021487E-4; uncertainty = 0.00000000033E-4 
        attribute 'speed of light in vacuum'     : SpeedValue = 299792458[m/s];               // 2018 CODATA attribute 299792458 m s^-1; (exact)
     }

    package 'Global Context' {
        attribute 'nominal earth gravitational acceleration': AccelerationValue = 9.80665['m/s²'];
    }

    package 'Model X Context' {
        attribute 'amplifier gain': DimensionOneValue = 3.5[one];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_01_constants.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_01-Constants' {
    private import MeasurementReferences::*;
    private import SI::*;
    private import RealFunctions::*;
    /* Note: Value properties that are bound to specific values are constants and have the specified
     * values in all contexts. It is not legal to redefine them.
     */
    package 'Mathematical Constants' {
        doc
        /*
	     * Standard mathematical constants
	     * 
	     * Irrational constants cannot be represented exactly with finite precision.
	     * However, they can be required to be implemented with a attribute that is accurate
	     * to at least a certain precision.
	     * 
	     * (The decimal literals here should be interpreted as being fixed point and exact.)
	     */
        attribute def e : Real {
            assert constraint {
                round(e * 1E20) == 271828182845904523536.0;
            }
        }
        attribute def pi : Real {
            assert constraint {
                round(pi * 1E20) == 314159265358979323846.0;
            }
        }
    }
    package 'Fundamental Physical Constants' {
        doc
        /*
	     * Standard fundamental physical constants
	     * 
	     * Physical constants have a standard measured attribute to a finite precision.
	     *
	     * The reference source is:
	     * CODATA - Task Group on Fundamental Physical Constants (TGFC) - 2018 CODATA recommended values
	     * See https://codata.org/initiatives/strategic-programme/fundamental-physical-constants/
	     * For the actual values see https://pml.nist.gov/cuu/Constants/ 
	     */
        attribute def 'fine structure constant' : DimensionOneValue = 7.2973525693E-3 [one];
        attribute def 'electron to proton mass ratio' : DimensionOneValue = 5.44617021487E-4 [one];
        attribute def 'speed of light in vacuum' : SpeedValue = 299792458 ['m/s'];
    }
    package 'Global Context' {
        attribute def 'nominal earth gravitational acceleration' : AccelerationValue = 9.80665 ['m/s²'];
    }
    package 'Model X Context' {
        attribute def 'amplifier gain' : DimensionOneValue = 3.5 [one];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 47) (line 2) (column 20) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 47) (line 2) (column 20) (len 21)))))
    (reference r1 (scope relative) (span (offset 92) (line 3) (column 20) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 92) (line 3) (column 20) (len 2)))))
    (reference r2 (scope relative) (span (offset 118) (line 4) (column 20) (len 13)) (segments (segment 0 (token "RealFunctions") (name "RealFunctions") (separator none) (span (offset 118) (line 4) (column 20) (len 13)))))
    (reference r3 (scope relative) (span (offset 772) (line 22) (column 22) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 772) (line 22) (column 22) (len 4)))))
    (reference r4 (scope relative) (span (offset 885) (line 25) (column 23) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 885) (line 25) (column 23) (len 4)))))
    (reference r5 (scope relative) (span (offset 1564) (line 43) (column 52) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 1564) (line 43) (column 52) (len 17)))))
    (reference r6 (scope relative) (span (offset 1731) (line 44) (column 52) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 1731) (line 44) (column 52) (len 17)))))
    (reference r7 (scope relative) (span (offset 1900) (line 45) (column 52) (len 10)) (segments (segment 0 (token "SpeedValue") (name "SpeedValue") (separator none) (span (offset 1900) (line 45) (column 52) (len 10)))))
    (reference r8 (scope relative) (span (offset 2095) (line 49) (column 63) (len 17)) (segments (segment 0 (token "AccelerationValue") (name "AccelerationValue") (separator none) (span (offset 2095) (line 49) (column 63) (len 17)))))
    (reference r9 (scope relative) (span (offset 2208) (line 53) (column 37) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 2208) (line 53) (column 37) (len 17)))))
  )
  (root (package (name "15_01-Constants") (body brace (import (target (span (span (offset 47) (line 2) (column 20) (len 24))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 68) (line 2) (column 41) (len 3))) (separator (span (offset 68) (line 2) (column 41) (len 2))) (marker (span (offset 70) (line 2) (column 43) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 92) (line 3) (column 20) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 94) (line 3) (column 22) (len 3))) (separator (span (offset 94) (line 3) (column 22) (len 2))) (marker (span (offset 96) (line 3) (column 24) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 118) (line 4) (column 20) (len 16))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 131) (line 4) (column 33) (len 3))) (separator (span (offset 131) (line 4) (column 33) (len 2))) (marker (span (offset 133) (line 4) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 143) (line 6) (column 7) (len 165)) (normalized "Note: Value properties that are bound to specific values are constants and have the specified\nvalues in all contexts. It is not legal to redefine them.\n"))) (package (name "Mathematical Constants") (body brace (doc (name none) (locale none) (body (span (offset 375) (line 12) (column 8) (len 368)) (normalized "Standard mathematical constants\n\nIrrational constants cannot be represented exactly with finite precision.\nHowever, they can be required to be implemented with a attribute that is accurate\nto at least a certain precision.\n\n(The decimal literals here should be interpreted as being fixed point and exact.)\n"))) (attribute-def (declaration-name "e") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (assert-constraint))) (attribute-def (declaration-name "pi") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (assert-constraint))))) (package (name "Fundamental Physical Constants") (body brace (doc (name none) (locale none) (body (span (offset 1047) (line 32) (column 8) (len 458)) (normalized "Standard fundamental physical constants\n\nPhysical constants have a standard measured attribute to a finite precision.\n\nThe reference source is:\nCODATA - Task Group on Fundamental Physical Constants (TGFC) - 2018 CODATA recommended values\nSee https://codata.org/initiatives/strategic-programme/fundamental-physical-constants/\nFor the actual values see https://pml.nist.gov/cuu/Constants/ \n"))) (attribute-def (declaration-name "fine structure constant") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1584) (line 43) (column 72) (len 20)) (literal-with-unit (value (expression (span (offset 1584) (line 43) (column 72) (len 15)) (real "7.2973525693E-3"))) (unit (expression (span (offset 1600) (line 43) (column 88) (len 3)) (bracket (expression (span (offset 1600) (line 43) (column 88) (len 3)) (unit "one")))))))))) (body semicolon)) (attribute-def (declaration-name "electron to proton mass ratio") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1751) (line 44) (column 72) (len 21)) (literal-with-unit (value (expression (span (offset 1751) (line 44) (column 72) (len 16)) (real "5.44617021487E-4"))) (unit (expression (span (offset 1768) (line 44) (column 89) (len 3)) (bracket (expression (span (offset 1768) (line 44) (column 89) (len 3)) (unit "one")))))))))) (body semicolon)) (attribute-def (declaration-name "speed of light in vacuum") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1913) (line 45) (column 65) (len 14)) (literal-with-unit (value (expression (span (offset 1913) (line 45) (column 65) (len 9)) (integer 299792458))) (unit (expression (span (offset 1923) (line 45) (column 75) (len 3)) (bracket (expression (span (offset 1923) (line 45) (column 75) (len 3)) (unit "m/s")))))))))) (body semicolon)))) (package (name "Global Context") (body brace (attribute-def (declaration-name "nominal earth gravitational acceleration") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2115) (line 49) (column 83) (len 16)) (literal-with-unit (value (expression (span (offset 2115) (line 49) (column 83) (len 7)) (real "9.80665"))) (unit (expression (span (offset 2123) (line 49) (column 91) (len 7)) (bracket (expression (span (offset 2123) (line 49) (column 91) (len 7)) (unit "m/s²")))))))))) (body semicolon)))) (package (name "Model X Context") (body brace (attribute-def (declaration-name "amplifier gain") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2228) (line 53) (column 57) (len 8)) (literal-with-unit (value (expression (span (offset 2228) (line 53) (column 57) (len 3)) (real "3.5"))) (unit (expression (span (offset 2232) (line 53) (column 61) (len 3)) (bracket (expression (span (offset 2232) (line 53) (column 61) (len 3)) (unit "one")))))))))) (body semicolon)))))))
)
~~~
