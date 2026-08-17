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
  )
  (root (package (name "15_01-Constants") (body brace (import (target (span (span (offset 47) (line 2) (column 20) (len 24))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 68) (line 2) (column 41) (len 3))) (separator (span (offset 68) (line 2) (column 41) (len 2))) (marker (span (offset 70) (line 2) (column 43) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 92) (line 3) (column 20) (len 5))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 94) (line 3) (column 22) (len 3))) (separator (span (offset 94) (line 3) (column 22) (len 2))) (marker (span (offset 96) (line 3) (column 24) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 118) (line 4) (column 20) (len 16))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 131) (line 4) (column 33) (len 3))) (separator (span (offset 131) (line 4) (column 33) (len 2))) (marker (span (offset 133) (line 4) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Mathematical Constants") (body brace (doc) (attribute-def (name "e") (multiplicity none)) (attribute-def (name "pi") (multiplicity none)))) (package (name "Fundamental Physical Constants") (body brace (doc) (attribute-def (name "fine structure constant") (multiplicity none)) (attribute-def (name "electron to proton mass ratio") (multiplicity none)) (attribute-def (name "speed of light in vacuum") (multiplicity none)))) (package (name "Global Context") (body brace (attribute-def (name "nominal earth gravitational acceleration") (multiplicity none)))) (package (name "Model X Context") (body brace (attribute-def (name "amplifier gain") (multiplicity none)))))))
)
~~~
