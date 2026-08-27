# META
~~~sexpr
(snapshot (type semantic) (description "SysML Validation (15-Properties-Values-Expressions): 15_06-System of Quantities"))
~~~
# SOURCE
~~~sysml
package '15_06-System of Quantities' {
    private import ISQ::*;

	/*
	 * A System of Quantities is represented by a model library package.
	 * 
	 * Its structure is modeled after the International System of Quantities (ISQ):
	 * - Quantity dimension is defined as the product of powers of a selected set of base quantities.
	 * - A system of quantities is multi-dimensional space spanned by the powers of its base quantities.
	 * - Any base quantity is modeled as a specialization of a SimpleUnit. Such a specialized SimpleUnit defines one base unit vector 
	 *   (with power one by definition), e.g. MassUnit with symbol M, that establishes a base quantity dimension for the system of quantities, 
	 *	 without committing yet to a particular choice of measurement unit.
	 * - To complete the system of quantities any number of derived quantities can be added.
	 * - A derived quantity is modeled as a specialization of a DerivedUnit. A DerivedUnit is defined in terms of so-called UnitPowerFactors. 
	 *   Each UnitPowerFactor is a combination of a base (or other derived) quantity and an exponent.
	 * - As an example the AccelerationUnit (specialization of DerivedUnit) can be defined as the combination of LengthUnit (symbol L) 
	 *   to the power 1 and TimeUnit (symbol T) to the power -2, so having quantity dimension L¹⋅T⁻².
	 * - A quantity of dimension one is defined as a derived quantity for which the effective exponent for each 
	 *   of its base quantity power factors is zero. Historically a quantity of dimension one was also called a dimensionless quantity.
	 * - A quantity of dimension one may be defined by adding all quantity power factors that cancel out by having positive and negative 
	 *   exponents. Doing so enables distinction between different 'kinds of' quantities of dimension one, e.g:
	 *   angle (L¹⋅L⁻¹), mass ratio (L¹⋅L⁻¹), power ratio (L²⋅M⋅T⁻³⋅L⁻²⋅M⁻¹⋅T³), Mach number (L¹⋅T⁻¹⋅L⁻¹⋅T¹).
	 * 
	 * The International System of Quantities (ISQ) as defined in ISO/IEC 80000 is added as a predefined model library to SysML v2.
	 * However, this does not prevent to model any other system of quantities in another model library and use it.
	 */
	 
	 /*
	  * Above capabilities were implemented in:
      * - standard library Quantities:
      *   TensorQuantityValue, VectorQuantityValue, ScalarQuantityValue,
      *   tensorQuantities, vectorQuantities, scalarQuantities, 
      *   SystemOfQuantities
	  * - standard library MeasurementReferences:
	  *   TensorMeasurementReference, VectorMeasurementReference, ScalarMeasurementReference,
      *   SystemOfUnits
	  * - standard library ISQBase:
	  *   attribute <isq> 'International System of Quantities': SystemOfQuantities in ISQBase
	  */
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_06_system_of_quantities.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '15_06-System of Quantities' {
    private import ISQ::*;
    /*
	 * A System of Quantities is represented by a model library package.
	 * 
	 * Its structure is modeled after the International System of Quantities (ISQ):
	 * - Quantity dimension is defined as the product of powers of a selected set of base quantities.
	 * - A system of quantities is multi-dimensional space spanned by the powers of its base quantities.
	 * - Any base quantity is modeled as a specialization of a SimpleUnit. Such a specialized SimpleUnit defines one base unit vector 
	 *   (with power one by definition), e.g. MassUnit with symbol M, that establishes a base quantity dimension for the system of quantities, 
	 *	 without committing yet to a particular choice of measurement unit.
	 * - To complete the system of quantities any number of derived quantities can be added.
	 * - A derived quantity is modeled as a specialization of a DerivedUnit. A DerivedUnit is defined in terms of so-called UnitPowerFactors. 
	 *   Each UnitPowerFactor is a combination of a base (or other derived) quantity and an exponent.
	 * - As an example the AccelerationUnit (specialization of DerivedUnit) can be defined as the combination of LengthUnit (symbol L) 
	 *   to the power 1 and TimeUnit (symbol T) to the power -2, so having quantity dimension L¹⋅T⁻².
	 * - A quantity of dimension one is defined as a derived quantity for which the effective exponent for each 
	 *   of its base quantity power factors is zero. Historically a quantity of dimension one was also called a dimensionless quantity.
	 * - A quantity of dimension one may be defined by adding all quantity power factors that cancel out by having positive and negative 
	 *   exponents. Doing so enables distinction between different 'kinds of' quantities of dimension one, e.g:
	 *   angle (L¹⋅L⁻¹), mass ratio (L¹⋅L⁻¹), power ratio (L²⋅M⋅T⁻³⋅L⁻²⋅M⁻¹⋅T³), Mach number (L¹⋅T⁻¹⋅L⁻¹⋅T¹).
	 * 
	 * The International System of Quantities (ISQ) as defined in ISO/IEC 80000 is added as a predefined model library to SysML v2.
	 * However, this does not prevent to model any other system of quantities in another model library and use it.
	 */
    /*
	  * Above capabilities were implemented in:
      * - standard library Quantities:
      *   TensorQuantityValue, VectorQuantityValue, ScalarQuantityValue,
      *   tensorQuantities, vectorQuantities, scalarQuantities, 
      *   SystemOfQuantities
	  * - standard library MeasurementReferences:
	  *   TensorMeasurementReference, VectorMeasurementReference, ScalarMeasurementReference,
      *   SystemOfUnits
	  * - standard library ISQBase:
	  *   attribute <isq> 'International System of Quantities': SystemOfQuantities in ISQBase
	  */
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 58) (line 2) (column 20) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 58) (line 2) (column 20) (len 3)))))
  )
  (root (package (name "15_06-System of Quantities") (body brace (import (target (span (span (offset 58) (line 2) (column 20) (len 6))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 61) (line 2) (column 23) (len 3))) (separator (span (offset 61) (line 2) (column 23) (len 2))) (marker (span (offset 63) (line 2) (column 25) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 70) (line 4) (column 4) (len 2159)) (normalized "A System of Quantities is represented by a model library package.\n\nIts structure is modeled after the International System of Quantities (ISQ):\n- Quantity dimension is defined as the product of powers of a selected set of base quantities.\n- A system of quantities is multi-dimensional space spanned by the powers of its base quantities.\n- Any base quantity is modeled as a specialization of a SimpleUnit. Such a specialized SimpleUnit defines one base unit vector \n  (with power one by definition), e.g. MassUnit with symbol M, that establishes a base quantity dimension for the system of quantities, \n\t without committing yet to a particular choice of measurement unit.\n- To complete the system of quantities any number of derived quantities can be added.\n- A derived quantity is modeled as a specialization of a DerivedUnit. A DerivedUnit is defined in terms of so-called UnitPowerFactors. \n  Each UnitPowerFactor is a combination of a base (or other derived) quantity and an exponent.\n- As an example the AccelerationUnit (specialization of DerivedUnit) can be defined as the combination of LengthUnit (symbol L) \n  to the power 1 and TimeUnit (symbol T) to the power -2, so having quantity dimension L¹⋅T⁻².\n- A quantity of dimension one is defined as a derived quantity for which the effective exponent for each \n  of its base quantity power factors is zero. Historically a quantity of dimension one was also called a dimensionless quantity.\n- A quantity of dimension one may be defined by adding all quantity power factors that cancel out by having positive and negative \n  exponents. Doing so enables distinction between different 'kinds of' quantities of dimension one, e.g:\n  angle (L¹⋅L⁻¹), mass ratio (L¹⋅L⁻¹), power ratio (L²⋅M⋅T⁻³⋅L⁻²⋅M⁻¹⋅T³), Mach number (L¹⋅T⁻¹⋅L⁻¹⋅T¹).\n\nThe International System of Quantities (ISQ) as defined in ISO/IEC 80000 is added as a predefined model library to SysML v2.\nHowever, this does not prevent to model any other system of quantities in another model library and use it.\n"))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2239) (line 28) (column 5) (len 541)) (normalized "Above capabilities were implemented in:\n- standard library Quantities:\n  TensorQuantityValue, VectorQuantityValue, ScalarQuantityValue,\n  tensorQuantities, vectorQuantities, scalarQuantities, \n  SystemOfQuantities\n- standard library MeasurementReferences:\n  TensorMeasurementReference, VectorMeasurementReference, ScalarMeasurementReference,\n  SystemOfUnits\n- standard library ISQBase:\n  attribute <isq> 'International System of Quantities': SystemOfQuantities in ISQBase\n"))))))
)
~~~
