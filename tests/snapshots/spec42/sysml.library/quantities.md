# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/Quantities"))
~~~
# SOURCE
~~~sysml
standard library package Quantities {
	doc
	/*
	 * This package defines the root representations for quantities and their values.
	 */

	private import Collections::*;
	private import ScalarValues::NumericalValue;
	private import ScalarValues::Number;
	private import ScalarValues::Real;
	private import ScalarValues::Natural;
	private import ScalarValues::Boolean;
	private import ScalarValues::String;
	private import VectorValues::NumericalVectorValue;
	private import VectorValues::ThreeVectorValue;

	abstract attribute def TensorQuantityValue :> Array {
		doc
		/*
		 * The value of a quantity is a tuple of one or more numbers (i.e. mathematical number values) and a reference to a measurement reference.
		 * The most general case is a multi-dimensional, tensor quantity of any order. In engineering, the majority of quantities used are 
		 * scalar and vector quantities, that are tensor quantities of order 0 and 1 respectively.
		 * The measurement reference used to express a quantity value must have a type, dimensions and order that match the quantity, i.e.,
		 * a TensorQuantityValue must use a TensorMeasurementReference, a VectorQuantityValue a VectorMeasurementReference, 
		 * and a ScalarQuantityValue a ScalarMeasurementReference. See package MeasurementReferences for details.
		 */
	
		attribute isBound: Boolean;
		attribute num: Number[1..*] ordered nonunique :>> elements;
		attribute mRef: MeasurementReferences::TensorMeasurementReference;
        attribute :>> dimensions = mRef.dimensions;
		attribute order :>> rank;
        attribute contravariantOrder: Natural;
        attribute covariantOrder: Natural;

        assert constraint orderSum { contravariantOrder + covariantOrder == order }
        assert constraint boundMatch { (isBound == mRef.isBound) or (not isBound and mRef.isBound) }
	}

	abstract attribute def VectorQuantityValue :> TensorQuantityValue, NumericalVectorValue {
		attribute :>> mRef: MeasurementReferences::VectorMeasurementReference;
	}

	abstract attribute def ScalarQuantityValue :> VectorQuantityValue, NumericalValue {
		attribute :>> mRef: MeasurementReferences::ScalarMeasurementReference;
	}
	
	abstract attribute tensorQuantities: TensorQuantityValue[*] nonunique {
		doc
		/*
		 * Quantities are defined as self-standing features that can be used to consistently specify quantities as 
		 * features of occurrences. Each single quantity feature is subsetting the root feature tensorQuantities. 
		 * In other words, the codomain of a quantity feature is a suitable specialization of TensorQuantityValue.
		 */
	}
	abstract attribute vectorQuantities: VectorQuantityValue[*] nonunique :> tensorQuantities;
	abstract attribute scalarQuantities: ScalarQuantityValue[*] nonunique :> vectorQuantities;

	abstract attribute def '3dVectorQuantityValue' :> VectorQuantityValue, ThreeVectorValue {
        doc
    	/*
    	 * Most general representation of real 3-vector quantities
    	 */

        attribute :>> num: Real[3];
	}
	alias ThreeDVectorQuantityValue for '3dVectorQuantityValue';
	
    /*
     * Define generic aliases QuantityValue and quantities for the top level quantity attribute def and attribute.
     */
	alias QuantityValue for TensorQuantityValue;
	alias quantities for tensorQuantities;

	attribute def SystemOfQuantities {
		doc
		/*
		 * A SystemOfQuantities represents the essentials of [VIM] concept "system of quantities" (https://jcgm.bipm.org/vim/en/1.3.html), defined as a
		 * "set of quantities together with a set of noncontradictory equations relating those quantities".
		 * In order to establish such a set of noncontradictory equations a set of base quantities is selected. Subsequently the system of quantities is 
		 * completed by adding derived quantities which are products of powers of the base quantities.
		 */
	
		attribute baseQuantities: ScalarQuantityValue[*] ordered :> scalarQuantities;
	}

	attribute def QuantityPowerFactor {
		doc
		/*
		 * Representation of a quantity power factor, being the combination of a quantity and an exponent.
		 * 
		 * A sequence of QuantityPowerFactors for the baseQuantities of a SystemOfQuantities define the QuantityDimension of a scalar quantity.
		 */
	
		attribute quantity: ScalarQuantityValue[1];
		attribute exponent: Real[1];
	}

	attribute def QuantityDimension {
		doc
		/*
		 * Representation of quantity dimension, which is the product of powers of the set of base quantities defined for a particular system of quantities, units and scales.
		 */
	
		 attribute quantityPowerFactors: QuantityPowerFactor[*] ordered;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "quantities.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Quantities {
    doc
    /*
	 * This package defines the root representations for quantities and their values.
	 */
    private import Collections::*;
    private import ScalarValues::NumericalValue;
    private import ScalarValues::Number;
    private import ScalarValues::Real;
    private import ScalarValues::Natural;
    private import ScalarValues::Boolean;
    private import ScalarValues::String;
    private import VectorValues::NumericalVectorValue;
    private import VectorValues::ThreeVectorValue;
    abstract attribute def TensorQuantityValue :> Array {
        doc
        /*
		 * The value of a quantity is a tuple of one or more numbers (i.e. mathematical number values) and a reference to a measurement reference.
		 * The most general case is a multi-dimensional, tensor quantity of any order. In engineering, the majority of quantities used are 
		 * scalar and vector quantities, that are tensor quantities of order 0 and 1 respectively.
		 * The measurement reference used to express a quantity value must have a type, dimensions and order that match the quantity, i.e.,
		 * a TensorQuantityValue must use a TensorMeasurementReference, a VectorQuantityValue a VectorMeasurementReference, 
		 * and a ScalarQuantityValue a ScalarMeasurementReference. See package MeasurementReferences for details.
		 */
        attribute isBound : Boolean;
        attribute num : Number[1..*] ordered nonunique :>> elements;
        attribute mRef : MeasurementReferences::TensorMeasurementReference;
        attribute :>> dimensions = mRef.dimensions;
        attribute order :>> rank;
        attribute contravariantOrder : Natural;
        attribute covariantOrder : Natural;
        assert constraint orderSum {
            contravariantOrder + covariantOrder == order;
        }
        assert constraint boundMatch {
            (isBound == mRef.isBound) || (not isBound && mRef.isBound);
        }
    }
    abstract attribute def VectorQuantityValue :> TensorQuantityValue, NumericalVectorValue {
        attribute :>> mRef : MeasurementReferences::VectorMeasurementReference;
    }
    abstract attribute def ScalarQuantityValue :> VectorQuantityValue, NumericalValue {
        attribute :>> mRef : MeasurementReferences::ScalarMeasurementReference;
    }
    abstract attribute tensorQuantities : TensorQuantityValue[*] nonunique {
        doc
        /*
		 * Quantities are defined as self-standing features that can be used to consistently specify quantities as 
		 * features of occurrences. Each single quantity feature is subsetting the root feature tensorQuantities. 
		 * In other words, the codomain of a quantity feature is a suitable specialization of TensorQuantityValue.
		 */
    }
    abstract attribute vectorQuantities : VectorQuantityValue[*] nonunique :> tensorQuantities;
    abstract attribute scalarQuantities : ScalarQuantityValue[*] nonunique :> vectorQuantities;
    abstract attribute def '3dVectorQuantityValue' :> VectorQuantityValue, ThreeVectorValue {
        doc
        /*
    	 * Most general representation of real 3-vector quantities
    	 */
        attribute :>> num : Real[3];
    }
    alias ThreeDVectorQuantityValue for '3dVectorQuantityValue';
    /*
     * Define generic aliases QuantityValue and quantities for the top level quantity attribute def and attribute.
     */
    alias QuantityValue for TensorQuantityValue;
    alias quantities for tensorQuantities;
    attribute def SystemOfQuantities {
        doc
        /*
		 * A SystemOfQuantities represents the essentials of [VIM] concept "system of quantities" (https://jcgm.bipm.org/vim/en/1.3.html), defined as a
		 * "set of quantities together with a set of noncontradictory equations relating those quantities".
		 * In order to establish such a set of noncontradictory equations a set of base quantities is selected. Subsequently the system of quantities is 
		 * completed by adding derived quantities which are products of powers of the base quantities.
		 */
        attribute baseQuantities : ScalarQuantityValue[*] ordered :> scalarQuantities;
    }
    attribute def QuantityPowerFactor {
        doc
        /*
		 * Representation of a quantity power factor, being the combination of a quantity and an exponent.
		 * 
		 * A sequence of QuantityPowerFactors for the baseQuantities of a SystemOfQuantities define the QuantityDimension of a scalar quantity.
		 */
        attribute quantity : ScalarQuantityValue[1];
        attribute exponent : Real[1];
    }
    attribute def QuantityDimension {
        doc
        /*
		 * Representation of quantity dimension, which is the product of powers of the set of base quantities defined for a particular system of quantities, units and scales.
		 */
        attribute quantityPowerFactors : QuantityPowerFactor[*] ordered;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 152) (line 7) (column 17) (len 11)) (segments (segment 0 (token "Collections") (name "Collections") (separator none) (span (offset 152) (line 7) (column 17) (len 11)))))
    (reference r1 (scope relative) (span (offset 184) (line 8) (column 17) (len 28)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 184) (line 8) (column 17) (len 12))) (segment 1 (token "NumericalValue") (name "NumericalValue") (separator colon-colon) (span (offset 198) (line 8) (column 31) (len 14)))))
    (reference r2 (scope relative) (span (offset 230) (line 9) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 230) (line 9) (column 17) (len 12))) (segment 1 (token "Number") (name "Number") (separator colon-colon) (span (offset 244) (line 9) (column 31) (len 6)))))
    (reference r3 (scope relative) (span (offset 268) (line 10) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 268) (line 10) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 282) (line 10) (column 31) (len 4)))))
    (reference r4 (scope relative) (span (offset 304) (line 11) (column 17) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 304) (line 11) (column 17) (len 12))) (segment 1 (token "Natural") (name "Natural") (separator colon-colon) (span (offset 318) (line 11) (column 31) (len 7)))))
    (reference r5 (scope relative) (span (offset 343) (line 12) (column 17) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 343) (line 12) (column 17) (len 12))) (segment 1 (token "Boolean") (name "Boolean") (separator colon-colon) (span (offset 357) (line 12) (column 31) (len 7)))))
    (reference r6 (scope relative) (span (offset 382) (line 13) (column 17) (len 20)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 382) (line 13) (column 17) (len 12))) (segment 1 (token "String") (name "String") (separator colon-colon) (span (offset 396) (line 13) (column 31) (len 6)))))
    (reference r7 (scope relative) (span (offset 420) (line 14) (column 17) (len 34)) (segments (segment 0 (token "VectorValues") (name "VectorValues") (separator none) (span (offset 420) (line 14) (column 17) (len 12))) (segment 1 (token "NumericalVectorValue") (name "NumericalVectorValue") (separator colon-colon) (span (offset 434) (line 14) (column 31) (len 20)))))
    (reference r8 (scope relative) (span (offset 472) (line 15) (column 17) (len 30)) (segments (segment 0 (token "VectorValues") (name "VectorValues") (separator none) (span (offset 472) (line 15) (column 17) (len 12))) (segment 1 (token "ThreeVectorValue") (name "ThreeVectorValue") (separator colon-colon) (span (offset 486) (line 15) (column 31) (len 16)))))
    (reference r9 (scope relative) (span (offset 552) (line 17) (column 48) (len 5)) (segments (segment 0 (token "Array") (name "Array") (separator none) (span (offset 552) (line 17) (column 48) (len 5)))))
    (reference r10 (scope relative) (span (offset 1329) (line 28) (column 22) (len 7)) (segments (segment 0 (token "Boolean") (name "Boolean") (separator none) (span (offset 1329) (line 28) (column 22) (len 7)))))
    (reference r11 (scope relative) (span (offset 1355) (line 29) (column 18) (len 6)) (segments (segment 0 (token "Number") (name "Number") (separator none) (span (offset 1355) (line 29) (column 18) (len 6)))))
    (reference r12 (scope relative) (span (offset 1390) (line 29) (column 53) (len 8)) (segments (segment 0 (token "elements") (name "elements") (separator none) (span (offset 1390) (line 29) (column 53) (len 8)))))
    (reference r13 (scope relative) (span (offset 1418) (line 30) (column 19) (len 49)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 1418) (line 30) (column 19) (len 21))) (segment 1 (token "TensorMeasurementReference") (name "TensorMeasurementReference") (separator colon-colon) (span (offset 1441) (line 30) (column 42) (len 26)))))
    (reference r14 (scope relative) (span (offset 1491) (line 31) (column 23) (len 10)) (segments (segment 0 (token "dimensions") (name "dimensions") (separator none) (span (offset 1491) (line 31) (column 23) (len 10)))))
    (reference r15 (scope relative) (span (offset 1504) (line 31) (column 36) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 1504) (line 31) (column 36) (len 4)))))
    (reference r16 (scope relative) (span (offset 1509) (line 31) (column 41) (len 10)) (segments (segment 0 (token "dimensions") (name "dimensions") (separator none) (span (offset 1509) (line 31) (column 41) (len 10)))))
    (reference r17 (scope relative) (span (offset 1543) (line 32) (column 23) (len 4)) (segments (segment 0 (token "rank") (name "rank") (separator none) (span (offset 1543) (line 32) (column 23) (len 4)))))
    (reference r18 (scope relative) (span (offset 1587) (line 33) (column 39) (len 7)) (segments (segment 0 (token "Natural") (name "Natural") (separator none) (span (offset 1587) (line 33) (column 39) (len 7)))))
    (reference r19 (scope relative) (span (offset 1630) (line 34) (column 35) (len 7)) (segments (segment 0 (token "Natural") (name "Natural") (separator none) (span (offset 1630) (line 34) (column 35) (len 7)))))
    (reference r20 (scope relative) (span (offset 1876) (line 40) (column 48) (len 19)) (segments (segment 0 (token "TensorQuantityValue") (name "TensorQuantityValue") (separator none) (span (offset 1876) (line 40) (column 48) (len 19)))))
    (reference r21 (scope relative) (span (offset 1897) (line 40) (column 69) (len 20)) (segments (segment 0 (token "NumericalVectorValue") (name "NumericalVectorValue") (separator none) (span (offset 1897) (line 40) (column 69) (len 20)))))
    (reference r22 (scope relative) (span (offset 1942) (line 41) (column 23) (len 49)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 1942) (line 41) (column 23) (len 21))) (segment 1 (token "VectorMeasurementReference") (name "VectorMeasurementReference") (separator colon-colon) (span (offset 1965) (line 41) (column 46) (len 26)))))
    (reference r23 (scope relative) (span (offset 1936) (line 41) (column 17) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 1936) (line 41) (column 17) (len 4)))))
    (reference r24 (scope relative) (span (offset 2044) (line 44) (column 48) (len 19)) (segments (segment 0 (token "VectorQuantityValue") (name "VectorQuantityValue") (separator none) (span (offset 2044) (line 44) (column 48) (len 19)))))
    (reference r25 (scope relative) (span (offset 2065) (line 44) (column 69) (len 14)) (segments (segment 0 (token "NumericalValue") (name "NumericalValue") (separator none) (span (offset 2065) (line 44) (column 69) (len 14)))))
    (reference r26 (scope relative) (span (offset 2104) (line 45) (column 23) (len 49)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 2104) (line 45) (column 23) (len 21))) (segment 1 (token "ScalarMeasurementReference") (name "ScalarMeasurementReference") (separator colon-colon) (span (offset 2127) (line 45) (column 46) (len 26)))))
    (reference r27 (scope relative) (span (offset 2098) (line 45) (column 17) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 2098) (line 45) (column 17) (len 4)))))
    (reference r28 (scope relative) (span (offset 2817) (line 59) (column 52) (len 19)) (segments (segment 0 (token "VectorQuantityValue") (name "VectorQuantityValue") (separator none) (span (offset 2817) (line 59) (column 52) (len 19)))))
    (reference r29 (scope relative) (span (offset 2838) (line 59) (column 73) (len 16)) (segments (segment 0 (token "ThreeVectorValue") (name "ThreeVectorValue") (separator none) (span (offset 2838) (line 59) (column 73) (len 16)))))
    (reference r30 (scope relative) (span (offset 2978) (line 65) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 2978) (line 65) (column 28) (len 4)))))
    (reference r31 (scope relative) (span (offset 2973) (line 65) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 2973) (line 65) (column 23) (len 3)))))
    (reference r32 (scope relative) (span (offset 3027) (line 67) (column 38) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 3027) (line 67) (column 38) (len 23)))))
    (reference r33 (scope relative) (span (offset 3209) (line 72) (column 26) (len 19)) (segments (segment 0 (token "TensorQuantityValue") (name "TensorQuantityValue") (separator none) (span (offset 3209) (line 72) (column 26) (len 19)))))
    (reference r34 (scope relative) (span (offset 3252) (line 73) (column 23) (len 16)) (segments (segment 0 (token "tensorQuantities") (name "tensorQuantities") (separator none) (span (offset 3252) (line 73) (column 23) (len 16)))))
    (reference r35 (scope relative) (span (offset 3847) (line 84) (column 29) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 3847) (line 84) (column 29) (len 19)))))
    (reference r36 (scope relative) (span (offset 3881) (line 84) (column 63) (len 16)) (segments (segment 0 (token "scalarQuantities") (name "scalarQuantities") (separator none) (span (offset 3881) (line 84) (column 63) (len 16)))))
    (reference r37 (scope relative) (span (offset 4226) (line 95) (column 23) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 4226) (line 95) (column 23) (len 19)))))
    (reference r38 (scope relative) (span (offset 4272) (line 96) (column 23) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 4272) (line 96) (column 23) (len 4)))))
    (reference r39 (scope relative) (span (offset 4543) (line 105) (column 36) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 4543) (line 105) (column 36) (len 19)))))
  )
  (root (library-package (name "Quantities") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 46) (line 3) (column 4) (len 86)) (normalized "This package defines the root representations for quantities and their values.\n"))) (import (target (span (span (offset 152) (line 7) (column 17) (len 14))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 163) (line 7) (column 28) (len 3))) (separator (span (offset 163) (line 7) (column 28) (len 2))) (marker (span (offset 165) (line 7) (column 30) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 184) (line 8) (column 17) (len 28))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 230) (line 9) (column 17) (len 20))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 268) (line 10) (column 17) (len 18))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 304) (line 11) (column 17) (len 21))) (all none) (ref r4) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 343) (line 12) (column 17) (len 21))) (all none) (ref r5) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 382) (line 13) (column 17) (len 20))) (all none) (ref r6) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 420) (line 14) (column 17) (len 34))) (all none) (ref r7) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 472) (line 15) (column 17) (len 30))) (all none) (ref r8) (shape (membership (recursive-suffix none))))) (attribute-def (declaration-name "TensorQuantityValue") (short-name none) (modifiers (abstract (span (offset 506) (line 17) (column 2) (len 8)))) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 570) (line 19) (column 5) (len 733)) (normalized "The value of a quantity is a tuple of one or more numbers (i.e. mathematical number values) and a reference to a measurement reference.\nThe most general case is a multi-dimensional, tensor quantity of any order. In engineering, the majority of quantities used are \nscalar and vector quantities, that are tensor quantities of order 0 and 1 respectively.\nThe measurement reference used to express a quantity value must have a type, dimensions and order that match the quantity, i.e.,\na TensorQuantityValue must use a TensorMeasurementReference, a VectorQuantityValue a VectorMeasurementReference, \nand a ScalarQuantityValue a ScalarMeasurementReference. See package MeasurementReferences for details.\n"))) (attribute-usage (declaration-name "isBound") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "num") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r12)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "mRef") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r14)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1504) (line 31) (column 36) (len 15)) (member-access (base (expression (span (offset 1504) (line 31) (column 36) (len 4)) (ref r15))) (separator dot) (member (ref r16))))))) (body semicolon)) (attribute-usage (declaration-name "order") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "contravariantOrder") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "covariantOrder") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (assert-constraint) (assert-constraint))) (attribute-def (declaration-name "VectorQuantityValue") (short-name none) (modifiers (abstract (span (offset 1830) (line 40) (column 2) (len 8)))) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r20) (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r23)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "ScalarQuantityValue") (short-name none) (modifiers (abstract (span (offset 1998) (line 44) (column 2) (len 8)))) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r24) (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r26)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r27)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-usage) (attribute-usage) (attribute-def (declaration-name "3dVectorQuantityValue") (short-name none) (modifiers (abstract (span (offset 2767) (line 59) (column 2) (len 8)))) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r28) (ref r29)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 2876) (line 61) (column 8) (len 71)) (normalized "Most general representation of real 3-vector quantities\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r30)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r31)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (alias (name "ThreeDVectorQuantityValue") (target (ref r32)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 3060) (line 69) (column 7) (len 121)) (normalized "Define generic aliases QuantityValue and quantities for the top level quantity attribute def and attribute.\n"))) (alias (name "QuantityValue") (target (ref r33)) (body semicolon)) (alias (name "quantities") (target (ref r34)) (body semicolon)) (attribute-def (declaration-name "SystemOfQuantities") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 3317) (line 77) (column 5) (len 497)) (normalized "A SystemOfQuantities represents the essentials of [VIM] concept \"system of quantities\" (https://jcgm.bipm.org/vim/en/1.3.html), defined as a\n\"set of quantities together with a set of noncontradictory equations relating those quantities\".\nIn order to establish such a set of noncontradictory equations a set of base quantities is selected. Subsequently the system of quantities is \ncompleted by adding derived quantities which are products of powers of the base quantities.\n"))) (attribute-usage (declaration-name "baseQuantities") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r35)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r36)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "QuantityPowerFactor") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 3950) (line 89) (column 5) (len 249)) (normalized "Representation of a quantity power factor, being the combination of a quantity and an exponent.\n\nA sequence of QuantityPowerFactors for the baseQuantities of a SystemOfQuantities define the QuantityDimension of a scalar quantity.\n"))) (attribute-usage (declaration-name "quantity") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r37)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "exponent") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r38)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "QuantityDimension") (short-name none) (modifiers) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 4330) (line 101) (column 5) (len 173)) (normalized "Representation of quantity dimension, which is the product of powers of the set of base quantities defined for a particular system of quantities, units and scales.\n"))) (attribute-usage (declaration-name "quantityPowerFactors") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r39)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))))
)
~~~
