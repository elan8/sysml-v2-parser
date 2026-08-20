# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/ISQ"))
~~~
# SOURCE
~~~sysml
standard library package ISQ {
    doc
    /*
     * International system of quantities (ISQ), as defined in ISO/IEC 80000
     */

	private import ScalarValues::Real;
	private import Quantities::*;
	private import MeasurementReferences::*;

	public import ISQBase::*;                  // ISO/IEC 80000 base quantities and general concepts
    public import ISQSpaceTime::*;             // ISO 80000-3 "Space and Time"
    public import ISQMechanics::*;             // ISO 80000-4 "Mechanics"
    public import ISQThermodynamics::*;        // ISO 80000-5 "Thermodynamics"
    public import ISQElectromagnetism::*;      // IEC 80000-6 "Electromagnetism"
    public import ISQLight::*;                 // ISO 80000-7 "Light"
    public import ISQAcoustics::*;             // ISO 80000-8 "Acoustics"
    public import ISQChemistryMolecular::*;    // ISO 80000-9 "Physical chemistry and molecular physics"
    public import ISQAtomicNuclear::*;         // ISO 80000-10 "Atomic and nuclear physics"
    public import ISQCharacteristicNumbers::*; // ISO 80000-11 "Characteristic numbers"
    public import ISQCondensedMatter::*;       // ISO 80000-12 "Condensed matter physics"
    public import ISQInformation::*;           // IEC 80000-13 "Information science and technology"

    /* Additional quantity declarations */

    attribute def TemperatureDifferenceValue :> ScalarQuantityValue {
        doc
        /*
         * temperature difference
         * A separate temperature difference quantity and unit are needed in order to support °C, °F and centrigrade temperature differences
         */
        attribute :>> num: Real;
        attribute :>> mRef: TemperatureDifferenceUnit[1];
    }
    
    attribute temperatureDifference: TemperatureDifferenceValue [*] nonunique :> scalarQuantities;

    attribute def TemperatureDifferenceUnit :> SimpleUnit {    
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = thermodynamicTemperaturePF; }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package ISQ {
    doc
    /*
     * International system of quantities (ISQ), as defined in ISO/IEC 80000
     */
    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;
    public import ISQBase::*;
    public import ISQSpaceTime::*;
    public import ISQMechanics::*;
    public import ISQThermodynamics::*;
    public import ISQElectromagnetism::*;
    public import ISQLight::*;
    public import ISQAcoustics::*;
    public import ISQChemistryMolecular::*;
    public import ISQAtomicNuclear::*;
    public import ISQCharacteristicNumbers::*;
    public import ISQCondensedMatter::*;
    public import ISQInformation::*;
    /* Additional quantity declarations */
    attribute def TemperatureDifferenceValue :> ScalarQuantityValue {
        doc
        /*
         * temperature difference
         * A separate temperature difference quantity and unit are needed in order to support °C, °F and centrigrade temperature differences
         */
        attribute :>> num : Real;
        attribute :>> mRef : TemperatureDifferenceUnit[1];
    }
    attribute temperatureDifference : TemperatureDifferenceValue[*] nonunique :> scalarQuantities;
    attribute def TemperatureDifferenceUnit :> SimpleUnit {
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
             :>> quantity = isq.'Θ';
             :>> exponent = 1;
        }
        attribute :>> quantityDimension {
             :>> quantityPowerFactors = thermodynamicTemperaturePF;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 148) (line 7) (column 17) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 148) (line 7) (column 17) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 162) (line 7) (column 31) (len 4)))))
    (reference r1 (scope relative) (span (offset 184) (line 8) (column 17) (len 10)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 184) (line 8) (column 17) (len 10)))))
    (reference r2 (scope relative) (span (offset 215) (line 9) (column 17) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 215) (line 9) (column 17) (len 21)))))
    (reference r3 (scope relative) (span (offset 257) (line 11) (column 16) (len 7)) (segments (segment 0 (token "ISQBase") (name "ISQBase") (separator none) (span (offset 257) (line 11) (column 16) (len 7)))))
    (reference r4 (scope relative) (span (offset 358) (line 12) (column 19) (len 12)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 358) (line 12) (column 19) (len 12)))))
    (reference r5 (scope relative) (span (offset 437) (line 13) (column 19) (len 12)) (segments (segment 0 (token "ISQMechanics") (name "ISQMechanics") (separator none) (span (offset 437) (line 13) (column 19) (len 12)))))
    (reference r6 (scope relative) (span (offset 511) (line 14) (column 19) (len 17)) (segments (segment 0 (token "ISQThermodynamics") (name "ISQThermodynamics") (separator none) (span (offset 511) (line 14) (column 19) (len 17)))))
    (reference r7 (scope relative) (span (offset 590) (line 15) (column 19) (len 19)) (segments (segment 0 (token "ISQElectromagnetism") (name "ISQElectromagnetism") (separator none) (span (offset 590) (line 15) (column 19) (len 19)))))
    (reference r8 (scope relative) (span (offset 671) (line 16) (column 19) (len 8)) (segments (segment 0 (token "ISQLight") (name "ISQLight") (separator none) (span (offset 671) (line 16) (column 19) (len 8)))))
    (reference r9 (scope relative) (span (offset 741) (line 17) (column 19) (len 12)) (segments (segment 0 (token "ISQAcoustics") (name "ISQAcoustics") (separator none) (span (offset 741) (line 17) (column 19) (len 12)))))
    (reference r10 (scope relative) (span (offset 815) (line 18) (column 19) (len 21)) (segments (segment 0 (token "ISQChemistryMolecular") (name "ISQChemistryMolecular") (separator none) (span (offset 815) (line 18) (column 19) (len 21)))))
    (reference r11 (scope relative) (span (offset 920) (line 19) (column 19) (len 16)) (segments (segment 0 (token "ISQAtomicNuclear") (name "ISQAtomicNuclear") (separator none) (span (offset 920) (line 19) (column 19) (len 16)))))
    (reference r12 (scope relative) (span (offset 1012) (line 20) (column 19) (len 24)) (segments (segment 0 (token "ISQCharacteristicNumbers") (name "ISQCharacteristicNumbers") (separator none) (span (offset 1012) (line 20) (column 19) (len 24)))))
    (reference r13 (scope relative) (span (offset 1100) (line 21) (column 19) (len 18)) (segments (segment 0 (token "ISQCondensedMatter") (name "ISQCondensedMatter") (separator none) (span (offset 1100) (line 21) (column 19) (len 18)))))
    (reference r14 (scope relative) (span (offset 1190) (line 22) (column 19) (len 14)) (segments (segment 0 (token "ISQInformation") (name "ISQInformation") (separator none) (span (offset 1190) (line 22) (column 19) (len 14)))))
    (reference r15 (scope relative) (span (offset 1365) (line 26) (column 49) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 1365) (line 26) (column 49) (len 19)))))
    (reference r16 (scope relative) (span (offset 1626) (line 32) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1626) (line 32) (column 28) (len 4)))))
    (reference r17 (scope relative) (span (offset 1621) (line 32) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 1621) (line 32) (column 23) (len 3)))))
    (reference r18 (scope relative) (span (offset 1660) (line 33) (column 29) (len 25)) (segments (segment 0 (token "TemperatureDifferenceUnit") (name "TemperatureDifferenceUnit") (separator none) (span (offset 1660) (line 33) (column 29) (len 25)))))
    (reference r19 (scope relative) (span (offset 1654) (line 33) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 1654) (line 33) (column 23) (len 4)))))
    (reference r20 (scope relative) (span (offset 1848) (line 38) (column 48) (len 10)) (segments (segment 0 (token "SimpleUnit") (name "SimpleUnit") (separator none) (span (offset 1848) (line 38) (column 48) (len 10)))))
    (reference r21 (scope relative) (span (offset 1919) (line 39) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 1919) (line 39) (column 55) (len 19)))))
    (reference r22 (scope relative) (span (offset 1948) (line 39) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 1948) (line 39) (column 84) (len 8)))))
    (reference r23 (scope relative) (span (offset 1959) (line 39) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 1959) (line 39) (column 95) (len 3)))))
    (reference r24 (scope relative) (span (offset 1963) (line 39) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 1963) (line 39) (column 99) (len 4)))))
    (reference r25 (scope relative) (span (offset 1973) (line 39) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 1973) (line 39) (column 109) (len 8)))))
    (reference r26 (scope relative) (span (offset 2011) (line 40) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 2011) (line 40) (column 23) (len 17)))))
    (reference r27 (scope relative) (span (offset 2035) (line 40) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 2035) (line 40) (column 47) (len 20)))))
    (reference r28 (scope relative) (span (offset 2058) (line 40) (column 70) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 2058) (line 40) (column 70) (len 26)))))
  )
  (root (library-package (name "ISQ") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 45) (line 3) (column 7) (len 83)) (normalized "International system of quantities (ISQ), as defined in ISO/IEC 80000\n"))) (import (target (span (span (offset 148) (line 7) (column 17) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 184) (line 8) (column 17) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 194) (line 8) (column 27) (len 3))) (separator (span (offset 194) (line 8) (column 27) (len 2))) (marker (span (offset 196) (line 8) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 215) (line 9) (column 17) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 236) (line 9) (column 38) (len 3))) (separator (span (offset 236) (line 9) (column 38) (len 2))) (marker (span (offset 238) (line 9) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 257) (line 11) (column 16) (len 10))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 264) (line 11) (column 23) (len 3))) (separator (span (offset 264) (line 11) (column 23) (len 2))) (marker (span (offset 266) (line 11) (column 25) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 358) (line 12) (column 19) (len 15))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 370) (line 12) (column 31) (len 3))) (separator (span (offset 370) (line 12) (column 31) (len 2))) (marker (span (offset 372) (line 12) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 437) (line 13) (column 19) (len 15))) (all none) (ref r5) (shape (namespace (wildcard-suffix (span (span (offset 449) (line 13) (column 31) (len 3))) (separator (span (offset 449) (line 13) (column 31) (len 2))) (marker (span (offset 451) (line 13) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 511) (line 14) (column 19) (len 20))) (all none) (ref r6) (shape (namespace (wildcard-suffix (span (span (offset 528) (line 14) (column 36) (len 3))) (separator (span (offset 528) (line 14) (column 36) (len 2))) (marker (span (offset 530) (line 14) (column 38) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 590) (line 15) (column 19) (len 22))) (all none) (ref r7) (shape (namespace (wildcard-suffix (span (span (offset 609) (line 15) (column 38) (len 3))) (separator (span (offset 609) (line 15) (column 38) (len 2))) (marker (span (offset 611) (line 15) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 671) (line 16) (column 19) (len 11))) (all none) (ref r8) (shape (namespace (wildcard-suffix (span (span (offset 679) (line 16) (column 27) (len 3))) (separator (span (offset 679) (line 16) (column 27) (len 2))) (marker (span (offset 681) (line 16) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 741) (line 17) (column 19) (len 15))) (all none) (ref r9) (shape (namespace (wildcard-suffix (span (span (offset 753) (line 17) (column 31) (len 3))) (separator (span (offset 753) (line 17) (column 31) (len 2))) (marker (span (offset 755) (line 17) (column 33) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 815) (line 18) (column 19) (len 24))) (all none) (ref r10) (shape (namespace (wildcard-suffix (span (span (offset 836) (line 18) (column 40) (len 3))) (separator (span (offset 836) (line 18) (column 40) (len 2))) (marker (span (offset 838) (line 18) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 920) (line 19) (column 19) (len 19))) (all none) (ref r11) (shape (namespace (wildcard-suffix (span (span (offset 936) (line 19) (column 35) (len 3))) (separator (span (offset 936) (line 19) (column 35) (len 2))) (marker (span (offset 938) (line 19) (column 37) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1012) (line 20) (column 19) (len 27))) (all none) (ref r12) (shape (namespace (wildcard-suffix (span (span (offset 1036) (line 20) (column 43) (len 3))) (separator (span (offset 1036) (line 20) (column 43) (len 2))) (marker (span (offset 1038) (line 20) (column 45) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1100) (line 21) (column 19) (len 21))) (all none) (ref r13) (shape (namespace (wildcard-suffix (span (span (offset 1118) (line 21) (column 37) (len 3))) (separator (span (offset 1118) (line 21) (column 37) (len 2))) (marker (span (offset 1120) (line 21) (column 39) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1190) (line 22) (column 19) (len 17))) (all none) (ref r14) (shape (namespace (wildcard-suffix (span (span (offset 1204) (line 22) (column 33) (len 3))) (separator (span (offset 1204) (line 22) (column 33) (len 2))) (marker (span (offset 1206) (line 22) (column 35) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1279) (line 24) (column 7) (len 34)) (normalized "Additional quantity declarations "))) (attribute-def (declaration-name "TemperatureDifferenceValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1409) (line 28) (column 11) (len 187)) (normalized "temperature difference\nA separate temperature difference quantity and unit are needed in order to support °C, °F and centrigrade temperature differences\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r16)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "TemperatureDifferenceUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1959) (line 39) (column 95) (len 8)) (member-access (base (expression (span (offset 1959) (line 39) (column 95) (len 3)) (ref r23))) (separator dot) (member (ref r24))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r25)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1984) (line 39) (column 120) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r26)))) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r27)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2058) (line 40) (column 70) (len 26)) (ref r28))))) (body semicolon)))))))))
)
~~~
