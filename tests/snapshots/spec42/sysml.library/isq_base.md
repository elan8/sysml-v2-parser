# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/ISQBase"))
~~~
# SOURCE
~~~sysml
standard library package ISQBase {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO/IEC 80000
     * 
     * Note 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,
     * with Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.
     */

    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;

    /* ISO-80000-3 item 3-1.1 length */
    attribute def LengthValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-1.1 length
         * symbol(s): `l`, `L`
         * application domain: generic
         * name: Length
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: linear extent in space between any two points
         * remarks: Length does not need to be measured along a straight line. Length is one of the seven base quantities in the International System of Units (ISO 80000-1).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LengthUnit[1];
    }

    attribute length: LengthValue[*] nonunique :> scalarQuantities;

    attribute def LengthUnit :> SimpleUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-3 item 3-9 duration, time */
    attribute def DurationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-9 duration, time
         * symbol(s): `t`
         * application domain: generic
         * name: Duration
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: measure of the time difference between two events
         * remarks: Duration is often just called time. Time is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). Duration is a measure of a time interval.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DurationUnit[1];
    }

    attribute duration: DurationValue[*] nonunique :> scalarQuantities;

    attribute def DurationUnit :> SimpleUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-4 item 4-1 mass */
    attribute def MassValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-1 mass
         * symbol(s): `m`
         * application domain: generic
         * name: Mass
         * quantity dimension: M^1
         * measurement unit(s): kg
         * tensor order: 0
         * definition: property of a body which expresses itself in terms of inertia with regard to changes in its state of motion as well as its gravitational attraction to other bodies
         * remarks: The kilogram (kg) is one of the seven base units (see ISO 80000-1) of the International System of Units, the SI. See also IEC 60050-113.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassUnit[1];
    }

    attribute mass: MassValue[*] nonunique :> scalarQuantities;

    attribute def MassUnit :> SimpleUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = massPF; }
    }

    /* ISO-80000-5 item 5-1 thermodynamic temperature, temperature */
    attribute def ThermodynamicTemperatureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-1 thermodynamic temperature, temperature
         * symbol(s): `T`, `Θ`
         * application domain: generic
         * name: ThermodynamicTemperature
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: partial derivative of internal energy with respect to entropy at constant volume and constant number of particles in the system: `T = ((partial U)/(partial S))_(V,N)` where `U` is internal energy (item 5-20.2), `S` is entropy (item 5-18), `V` is volume (ISO 80000-3), and `N` is number of particles
         * remarks: It is measured with a primary thermometer, examples of which are gas thermometers of different kinds, noise thermometers, or radiation thermometers. The Boltzmann constant (ISO 80000-1) relates energy at the individual particle level with thermodynamic temperature. Differences of thermodynamic temperatures or changes may be expressed either in kelvin, symbol K, or in degrees Celsius, symbol °C (item 5-2). Thermodynamic temperature is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). The International Temperature Scale of 1990. For the purpose of practical measurements, the International Temperature Scale of 1990, ITS-90, was adopted by CIPM in 1989, which is a close approximation to the thermodynamic temperature scale. The quantities defined by this scale are denoted `T_90` and `t_90`, respectively (replacing `T_68` and `t_68` defined by the International Practical Temperature Scale of 1968, IPTS-68), where `t_90/(1 °C) = T_90/(1 K) - 273,15`. The units of `T_90` and `t_90` are the kelvin, symbol K, and the degree Celsius, symbol °C (item 5-2), respectively. For further information, see References [5], [6]. For ready conversion between temperatures reported on the International Temperature Scale and thermodynamic temperatures the systematic deviations can be found in Reference [7].
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermodynamicTemperatureUnit[1];
    }

    attribute thermodynamicTemperature: ThermodynamicTemperatureValue[*] nonunique :> scalarQuantities;

    attribute def ThermodynamicTemperatureUnit :> SimpleUnit {
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = thermodynamicTemperaturePF; }
    }

    /* IEC-80000-6 item 6-1 electric current */
    attribute def ElectricCurrentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-1 electric current
         * symbol(s): `I`, `i`
         * application domain: generic
         * name: ElectricCurrent
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: electric current is one of the base quantities in the International System of Quantities, ISQ, on which the International System of Units, SI, is based
         * remarks: Electric current is the quantity that can often be measured with an ammeter. The electric current through a surface is the quotient of the electric charge (item 6-2) transferred through the surface during a time interval by the duration of that interval. For a more complete definition, see item 6-8 and IEC 60050-121, item 121-11-13.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectricCurrentUnit[1];
    }

    attribute electricCurrent: ElectricCurrentValue[*] nonunique :> scalarQuantities;

    attribute def ElectricCurrentUnit :> SimpleUnit {
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = electricCurrentPF; }
    }

    /* ISO-80000-7 item 7-14 luminous intensity */
    attribute def LuminousIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-14 luminous intensity
         * symbol(s): `I_v`, `(I)`
         * application domain: generic
         * name: LuminousIntensity
         * quantity dimension: J^1
         * measurement unit(s): cd
         * tensor order: 0
         * definition: density of luminous flux with respect to solid angle in a specified direction, expressed by `I_v = (dΦ_v)/(dΩ)` where `Φ_v` is the luminous flux (item 7-13) emitted in a specified direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction
         * remarks: The definition holds strictly only for a point source. The distribution of the luminous intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,ϕ)`, is used to determine the luminous flux (item 7-13) within a certain solid angle (ISO 80000-3) `Ω` of a source: `Φ_v = int int_Ω I_v(θ,φ) sin(θ) dφ dθ`. Luminous intensity can be derived from the spectral radiant intensity distribution by `I_v = K_m int_0^∞ I_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `I_(e,λ)(λ)` is the spectral radiant intensity (item 7-5.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). The corresponding radiometric quantity is "radiant intensity" (item 7-5.1). The corresponding quantity for photons is "photon intensity" (item 7-21).
         */
        attribute :>> num: Real;
        attribute :>> mRef: LuminousIntensityUnit[1];
    }

    attribute luminousIntensity: LuminousIntensityValue[*] nonunique :> scalarQuantities;

    attribute def LuminousIntensityUnit :> SimpleUnit {
        private attribute luminousIntensityPF: QuantityPowerFactor[1] { :>> quantity = isq.J; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = luminousIntensityPF; }
    }

    /* ISO-80000-9 item 9-2 amount of substance, number of moles */
    attribute def AmountOfSubstanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-2 amount of substance, number of moles
         * symbol(s): `n(X)`
         * application domain: generic
         * name: AmountOfSubstance
         * quantity dimension: N^1
         * measurement unit(s): mol
         * tensor order: 0
         * definition: quotient of number `N` of specified elementary entities of kind `X` (item 9-1) in a sample, and the Avogadro constant `N_A` (ISO 80000-1): `n(X) = N(X)/N_A`
         * remarks: Amount of substance is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). Elementary entities, such as molecules, atoms, ions, electrons, holes and other quasi-particles, double bonds can be used. It is necessary to specify precisely the entity involved, e.g. atoms of hydrogen `H` vs. molecules of hydrogen `H_2`, preferably by giving the molecular chemical formula of the material involved. In the name "amount of substance", the words "of substance" could be replaced by words specifying the substance concerned, e.g. "amount of hydrogen chloride, `HCl`", or "amount of benzene, `C_6H_6`". The name "number of moles" is often used for "amount of substance", but this is deprecated because the name of a quantity should be distinguished from the name of the unit.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AmountOfSubstanceUnit[1];
    }

    attribute amountOfSubstance: AmountOfSubstanceValue[*] nonunique :> scalarQuantities;

    attribute def AmountOfSubstanceUnit :> SimpleUnit {
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = amountOfSubstancePF; }
    }

    attribute <isq> 'International System of Quantities': SystemOfQuantities {
        doc
        /*
         * Declaration of the International System of Quantities (ISQ), 
         * including its base quantities and symbols as specified in ISO 80000-1:2009.
         */
        attribute :>> baseQuantities = ( L, M, T, I, 'Θ', N, J );
        
        attribute L: LengthValue[1];
        attribute M: MassValue[1];
        attribute T: DurationValue[1];
        attribute I: ElectricCurrentValue[1];
        attribute 'Θ': ThermodynamicTemperatureValue[1];
        attribute N: AmountOfSubstanceValue[1];
        attribute J: LuminousIntensityValue[1];
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_base.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package ISQBase {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO/IEC 80000
     * 
     * Note 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,
     * with Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.
     */
    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;
    /* ISO-80000-3 item 3-1.1 length */
    attribute def LengthValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-1.1 length
         * symbol(s): `l`, `L`
         * application domain: generic
         * name: Length
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: linear extent in space between any two points
         * remarks: Length does not need to be measured along a straight line. Length is one of the seven base quantities in the International System of Units (ISO 80000-1).
         */
        attribute :>> num : Real;
        attribute :>> mRef : LengthUnit[1];
    }
    attribute length : LengthValue[*] nonunique :> scalarQuantities;
    attribute def LengthUnit :> SimpleUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
             :>> quantity = isq.L;
             :>> exponent = 1;
        }
        attribute :>> quantityDimension {
             :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-3 item 3-9 duration, time */
    attribute def DurationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-9 duration, time
         * symbol(s): `t`
         * application domain: generic
         * name: Duration
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: measure of the time difference between two events
         * remarks: Duration is often just called time. Time is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). Duration is a measure of a time interval.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DurationUnit[1];
    }
    attribute duration : DurationValue[*] nonunique :> scalarQuantities;
    attribute def DurationUnit :> SimpleUnit {
        private attribute durationPF : QuantityPowerFactor[1] {
             :>> quantity = isq.T;
             :>> exponent = 1;
        }
        attribute :>> quantityDimension {
             :>> quantityPowerFactors = durationPF;
        }
    }
    /* ISO-80000-4 item 4-1 mass */
    attribute def MassValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 4-1 mass
         * symbol(s): `m`
         * application domain: generic
         * name: Mass
         * quantity dimension: M^1
         * measurement unit(s): kg
         * tensor order: 0
         * definition: property of a body which expresses itself in terms of inertia with regard to changes in its state of motion as well as its gravitational attraction to other bodies
         * remarks: The kilogram (kg) is one of the seven base units (see ISO 80000-1) of the International System of Units, the SI. See also IEC 60050-113.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassUnit[1];
    }
    attribute mass : MassValue[*] nonunique :> scalarQuantities;
    attribute def MassUnit :> SimpleUnit {
        private attribute massPF : QuantityPowerFactor[1] {
             :>> quantity = isq.M;
             :>> exponent = 1;
        }
        attribute :>> quantityDimension {
             :>> quantityPowerFactors = massPF;
        }
    }
    /* ISO-80000-5 item 5-1 thermodynamic temperature, temperature */
    attribute def ThermodynamicTemperatureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-1 thermodynamic temperature, temperature
         * symbol(s): `T`, `Θ`
         * application domain: generic
         * name: ThermodynamicTemperature
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: partial derivative of internal energy with respect to entropy at constant volume and constant number of particles in the system: `T = ((partial U)/(partial S))_(V,N)` where `U` is internal energy (item 5-20.2), `S` is entropy (item 5-18), `V` is volume (ISO 80000-3), and `N` is number of particles
         * remarks: It is measured with a primary thermometer, examples of which are gas thermometers of different kinds, noise thermometers, or radiation thermometers. The Boltzmann constant (ISO 80000-1) relates energy at the individual particle level with thermodynamic temperature. Differences of thermodynamic temperatures or changes may be expressed either in kelvin, symbol K, or in degrees Celsius, symbol °C (item 5-2). Thermodynamic temperature is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). The International Temperature Scale of 1990. For the purpose of practical measurements, the International Temperature Scale of 1990, ITS-90, was adopted by CIPM in 1989, which is a close approximation to the thermodynamic temperature scale. The quantities defined by this scale are denoted `T_90` and `t_90`, respectively (replacing `T_68` and `t_68` defined by the International Practical Temperature Scale of 1968, IPTS-68), where `t_90/(1 °C) = T_90/(1 K) - 273,15`. The units of `T_90` and `t_90` are the kelvin, symbol K, and the degree Celsius, symbol °C (item 5-2), respectively. For further information, see References [5], [6]. For ready conversion between temperatures reported on the International Temperature Scale and thermodynamic temperatures the systematic deviations can be found in Reference [7].
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThermodynamicTemperatureUnit[1];
    }
    attribute thermodynamicTemperature : ThermodynamicTemperatureValue[*] nonunique :> scalarQuantities;
    attribute def ThermodynamicTemperatureUnit :> SimpleUnit {
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
             :>> quantity = isq.'Θ';
             :>> exponent = 1;
        }
        attribute :>> quantityDimension {
             :>> quantityPowerFactors = thermodynamicTemperaturePF;
        }
    }
    /* IEC-80000-6 item 6-1 electric current */
    attribute def ElectricCurrentValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 6-1 electric current
         * symbol(s): `I`, `i`
         * application domain: generic
         * name: ElectricCurrent
         * quantity dimension: I^1
         * measurement unit(s): A
         * tensor order: 0
         * definition: electric current is one of the base quantities in the International System of Quantities, ISQ, on which the International System of Units, SI, is based
         * remarks: Electric current is the quantity that can often be measured with an ammeter. The electric current through a surface is the quotient of the electric charge (item 6-2) transferred through the surface during a time interval by the duration of that interval. For a more complete definition, see item 6-8 and IEC 60050-121, item 121-11-13.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectricCurrentUnit[1];
    }
    attribute electricCurrent : ElectricCurrentValue[*] nonunique :> scalarQuantities;
    attribute def ElectricCurrentUnit :> SimpleUnit {
        private attribute electricCurrentPF : QuantityPowerFactor[1] {
             :>> quantity = isq.I;
             :>> exponent = 1;
        }
        attribute :>> quantityDimension {
             :>> quantityPowerFactors = electricCurrentPF;
        }
    }
    /* ISO-80000-7 item 7-14 luminous intensity */
    attribute def LuminousIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 7-14 luminous intensity
         * symbol(s): `I_v`, `(I)`
         * application domain: generic
         * name: LuminousIntensity
         * quantity dimension: J^1
         * measurement unit(s): cd
         * tensor order: 0
         * definition: density of luminous flux with respect to solid angle in a specified direction, expressed by `I_v = (dΦ_v)/(dΩ)` where `Φ_v` is the luminous flux (item 7-13) emitted in a specified direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction
         * remarks: The definition holds strictly only for a point source. The distribution of the luminous intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,ϕ)`, is used to determine the luminous flux (item 7-13) within a certain solid angle (ISO 80000-3) `Ω` of a source: `Φ_v = int int_Ω I_v(θ,φ) sin(θ) dφ dθ`. Luminous intensity can be derived from the spectral radiant intensity distribution by `I_v = K_m int_0^∞ I_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `I_(e,λ)(λ)` is the spectral radiant intensity (item 7-5.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). The corresponding radiometric quantity is "radiant intensity" (item 7-5.1). The corresponding quantity for photons is "photon intensity" (item 7-21).
         */
        attribute :>> num : Real;
        attribute :>> mRef : LuminousIntensityUnit[1];
    }
    attribute luminousIntensity : LuminousIntensityValue[*] nonunique :> scalarQuantities;
    attribute def LuminousIntensityUnit :> SimpleUnit {
        private attribute luminousIntensityPF : QuantityPowerFactor[1] {
             :>> quantity = isq.J;
             :>> exponent = 1;
        }
        attribute :>> quantityDimension {
             :>> quantityPowerFactors = luminousIntensityPF;
        }
    }
    /* ISO-80000-9 item 9-2 amount of substance, number of moles */
    attribute def AmountOfSubstanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-2 amount of substance, number of moles
         * symbol(s): `n(X)`
         * application domain: generic
         * name: AmountOfSubstance
         * quantity dimension: N^1
         * measurement unit(s): mol
         * tensor order: 0
         * definition: quotient of number `N` of specified elementary entities of kind `X` (item 9-1) in a sample, and the Avogadro constant `N_A` (ISO 80000-1): `n(X) = N(X)/N_A`
         * remarks: Amount of substance is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). Elementary entities, such as molecules, atoms, ions, electrons, holes and other quasi-particles, double bonds can be used. It is necessary to specify precisely the entity involved, e.g. atoms of hydrogen `H` vs. molecules of hydrogen `H_2`, preferably by giving the molecular chemical formula of the material involved. In the name "amount of substance", the words "of substance" could be replaced by words specifying the substance concerned, e.g. "amount of hydrogen chloride, `HCl`", or "amount of benzene, `C_6H_6`". The name "number of moles" is often used for "amount of substance", but this is deprecated because the name of a quantity should be distinguished from the name of the unit.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AmountOfSubstanceUnit[1];
    }
    attribute amountOfSubstance : AmountOfSubstanceValue[*] nonunique :> scalarQuantities;
    attribute def AmountOfSubstanceUnit :> SimpleUnit {
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
             :>> quantity = isq.N;
             :>> exponent = 1;
        }
        attribute :>> quantityDimension {
             :>> quantityPowerFactors = amountOfSubstancePF;
        }
    }
    attribute <isq> 'International System of Quantities' : SystemOfQuantities {
        doc
        /*
         * Declaration of the International System of Quantities (ISQ), 
         * including its base quantities and symbols as specified in ISO 80000-1:2009.
         */
        attribute :>> baseQuantities = (L, M, T, I, 'Θ', N, J);
        attribute L : LengthValue[1];
        attribute M : MassValue[1];
        attribute T : DurationValue[1];
        attribute I : ElectricCurrentValue[1];
        attribute 'Θ' : ThermodynamicTemperatureValue[1];
        attribute N : AmountOfSubstanceValue[1];
        attribute J : LuminousIntensityValue[1];
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 434) (line 11) (column 20) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 434) (line 11) (column 20) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 448) (line 11) (column 34) (len 4)))))
    (reference r1 (scope relative) (span (offset 473) (line 12) (column 20) (len 10)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 473) (line 12) (column 20) (len 10)))))
    (reference r2 (scope relative) (span (offset 507) (line 13) (column 20) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 507) (line 13) (column 20) (len 21)))))
    (reference r3 (scope relative) (span (offset 607) (line 16) (column 34) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 607) (line 16) (column 34) (len 19)))))
    (reference r4 (scope relative) (span (offset 1161) (line 29) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 1161) (line 29) (column 28) (len 4)))))
    (reference r5 (scope relative) (span (offset 1156) (line 29) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 1156) (line 29) (column 23) (len 3)))))
    (reference r6 (scope relative) (span (offset 1195) (line 30) (column 29) (len 10)) (segments (segment 0 (token "LengthUnit") (name "LengthUnit") (separator none) (span (offset 1195) (line 30) (column 29) (len 10)))))
    (reference r7 (scope relative) (span (offset 1189) (line 30) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 1189) (line 30) (column 23) (len 4)))))
    (reference r8 (scope relative) (span (offset 1318) (line 35) (column 33) (len 10)) (segments (segment 0 (token "SimpleUnit") (name "SimpleUnit") (separator none) (span (offset 1318) (line 35) (column 33) (len 10)))))
    (reference r9 (scope relative) (span (offset 1367) (line 36) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 1367) (line 36) (column 37) (len 19)))))
    (reference r10 (scope relative) (span (offset 1396) (line 36) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 1396) (line 36) (column 66) (len 8)))))
    (reference r11 (scope relative) (span (offset 1407) (line 36) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 1407) (line 36) (column 77) (len 3)))))
    (reference r12 (scope relative) (span (offset 1411) (line 36) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 1411) (line 36) (column 81) (len 1)))))
    (reference r13 (scope relative) (span (offset 1418) (line 36) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 1418) (line 36) (column 88) (len 8)))))
    (reference r14 (scope relative) (span (offset 1456) (line 37) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 1456) (line 37) (column 23) (len 17)))))
    (reference r15 (scope relative) (span (offset 1480) (line 37) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 1480) (line 37) (column 47) (len 20)))))
    (reference r16 (scope relative) (span (offset 1503) (line 37) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 1503) (line 37) (column 70) (len 8)))))
    (reference r17 (scope relative) (span (offset 1603) (line 41) (column 36) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 1603) (line 41) (column 36) (len 19)))))
    (reference r18 (scope relative) (span (offset 2195) (line 54) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 2195) (line 54) (column 28) (len 4)))))
    (reference r19 (scope relative) (span (offset 2190) (line 54) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 2190) (line 54) (column 23) (len 3)))))
    (reference r20 (scope relative) (span (offset 2229) (line 55) (column 29) (len 12)) (segments (segment 0 (token "DurationUnit") (name "DurationUnit") (separator none) (span (offset 2229) (line 55) (column 29) (len 12)))))
    (reference r21 (scope relative) (span (offset 2223) (line 55) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 2223) (line 55) (column 23) (len 4)))))
    (reference r22 (scope relative) (span (offset 2360) (line 60) (column 35) (len 10)) (segments (segment 0 (token "SimpleUnit") (name "SimpleUnit") (separator none) (span (offset 2360) (line 60) (column 35) (len 10)))))
    (reference r23 (scope relative) (span (offset 2411) (line 61) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 2411) (line 61) (column 39) (len 19)))))
    (reference r24 (scope relative) (span (offset 2440) (line 61) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 2440) (line 61) (column 68) (len 8)))))
    (reference r25 (scope relative) (span (offset 2451) (line 61) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 2451) (line 61) (column 79) (len 3)))))
    (reference r26 (scope relative) (span (offset 2455) (line 61) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 2455) (line 61) (column 83) (len 1)))))
    (reference r27 (scope relative) (span (offset 2462) (line 61) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 2462) (line 61) (column 90) (len 8)))))
    (reference r28 (scope relative) (span (offset 2500) (line 62) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 2500) (line 62) (column 23) (len 17)))))
    (reference r29 (scope relative) (span (offset 2524) (line 62) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 2524) (line 62) (column 47) (len 20)))))
    (reference r30 (scope relative) (span (offset 2547) (line 62) (column 70) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 2547) (line 62) (column 70) (len 10)))))
    (reference r31 (scope relative) (span (offset 2635) (line 66) (column 32) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 2635) (line 66) (column 32) (len 19)))))
    (reference r32 (scope relative) (span (offset 3280) (line 79) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 3280) (line 79) (column 28) (len 4)))))
    (reference r33 (scope relative) (span (offset 3275) (line 79) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 3275) (line 79) (column 23) (len 3)))))
    (reference r34 (scope relative) (span (offset 3314) (line 80) (column 29) (len 8)) (segments (segment 0 (token "MassUnit") (name "MassUnit") (separator none) (span (offset 3314) (line 80) (column 29) (len 8)))))
    (reference r35 (scope relative) (span (offset 3308) (line 80) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 3308) (line 80) (column 23) (len 4)))))
    (reference r36 (scope relative) (span (offset 3429) (line 85) (column 31) (len 10)) (segments (segment 0 (token "SimpleUnit") (name "SimpleUnit") (separator none) (span (offset 3429) (line 85) (column 31) (len 10)))))
    (reference r37 (scope relative) (span (offset 3476) (line 86) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 3476) (line 86) (column 35) (len 19)))))
    (reference r38 (scope relative) (span (offset 3505) (line 86) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 3505) (line 86) (column 64) (len 8)))))
    (reference r39 (scope relative) (span (offset 3516) (line 86) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 3516) (line 86) (column 75) (len 3)))))
    (reference r40 (scope relative) (span (offset 3520) (line 86) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 3520) (line 86) (column 79) (len 1)))))
    (reference r41 (scope relative) (span (offset 3527) (line 86) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 3527) (line 86) (column 86) (len 8)))))
    (reference r42 (scope relative) (span (offset 3565) (line 87) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 3565) (line 87) (column 23) (len 17)))))
    (reference r43 (scope relative) (span (offset 3589) (line 87) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 3589) (line 87) (column 47) (len 20)))))
    (reference r44 (scope relative) (span (offset 3612) (line 87) (column 70) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 3612) (line 87) (column 70) (len 6)))))
    (reference r45 (scope relative) (span (offset 3750) (line 91) (column 52) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 3750) (line 91) (column 52) (len 19)))))
    (reference r46 (scope relative) (span (offset 5808) (line 104) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 5808) (line 104) (column 28) (len 4)))))
    (reference r47 (scope relative) (span (offset 5803) (line 104) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 5803) (line 104) (column 23) (len 3)))))
    (reference r48 (scope relative) (span (offset 5842) (line 105) (column 29) (len 28)) (segments (segment 0 (token "ThermodynamicTemperatureUnit") (name "ThermodynamicTemperatureUnit") (separator none) (span (offset 5842) (line 105) (column 29) (len 28)))))
    (reference r49 (scope relative) (span (offset 5836) (line 105) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 5836) (line 105) (column 23) (len 4)))))
    (reference r50 (scope relative) (span (offset 6037) (line 110) (column 51) (len 10)) (segments (segment 0 (token "SimpleUnit") (name "SimpleUnit") (separator none) (span (offset 6037) (line 110) (column 51) (len 10)))))
    (reference r51 (scope relative) (span (offset 6104) (line 111) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 6104) (line 111) (column 55) (len 19)))))
    (reference r52 (scope relative) (span (offset 6133) (line 111) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 6133) (line 111) (column 84) (len 8)))))
    (reference r53 (scope relative) (span (offset 6144) (line 111) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 6144) (line 111) (column 95) (len 3)))))
    (reference r54 (scope relative) (span (offset 6148) (line 111) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 6148) (line 111) (column 99) (len 4)))))
    (reference r55 (scope relative) (span (offset 6158) (line 111) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 6158) (line 111) (column 109) (len 8)))))
    (reference r56 (scope relative) (span (offset 6196) (line 112) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 6196) (line 112) (column 23) (len 17)))))
    (reference r57 (scope relative) (span (offset 6220) (line 112) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 6220) (line 112) (column 47) (len 20)))))
    (reference r58 (scope relative) (span (offset 6243) (line 112) (column 70) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 6243) (line 112) (column 70) (len 26)))))
    (reference r59 (scope relative) (span (offset 6370) (line 116) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 6370) (line 116) (column 43) (len 19)))))
    (reference r60 (scope relative) (span (offset 7228) (line 129) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 7228) (line 129) (column 28) (len 4)))))
    (reference r61 (scope relative) (span (offset 7223) (line 129) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 7223) (line 129) (column 23) (len 3)))))
    (reference r62 (scope relative) (span (offset 7262) (line 130) (column 29) (len 19)) (segments (segment 0 (token "ElectricCurrentUnit") (name "ElectricCurrentUnit") (separator none) (span (offset 7262) (line 130) (column 29) (len 19)))))
    (reference r63 (scope relative) (span (offset 7256) (line 130) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 7256) (line 130) (column 23) (len 4)))))
    (reference r64 (scope relative) (span (offset 7421) (line 135) (column 42) (len 10)) (segments (segment 0 (token "SimpleUnit") (name "SimpleUnit") (separator none) (span (offset 7421) (line 135) (column 42) (len 10)))))
    (reference r65 (scope relative) (span (offset 7479) (line 136) (column 46) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 7479) (line 136) (column 46) (len 19)))))
    (reference r66 (scope relative) (span (offset 7508) (line 136) (column 75) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 7508) (line 136) (column 75) (len 8)))))
    (reference r67 (scope relative) (span (offset 7519) (line 136) (column 86) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 7519) (line 136) (column 86) (len 3)))))
    (reference r68 (scope relative) (span (offset 7523) (line 136) (column 90) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 7523) (line 136) (column 90) (len 1)))))
    (reference r69 (scope relative) (span (offset 7530) (line 136) (column 97) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 7530) (line 136) (column 97) (len 8)))))
    (reference r70 (scope relative) (span (offset 7568) (line 137) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 7568) (line 137) (column 23) (len 17)))))
    (reference r71 (scope relative) (span (offset 7592) (line 137) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 7592) (line 137) (column 47) (len 20)))))
    (reference r72 (scope relative) (span (offset 7615) (line 137) (column 70) (len 17)) (segments (segment 0 (token "electricCurrentPF") (name "electricCurrentPF") (separator none) (span (offset 7615) (line 137) (column 70) (len 17)))))
    (reference r73 (scope relative) (span (offset 7738) (line 141) (column 45) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 7738) (line 141) (column 45) (len 19)))))
    (reference r74 (scope relative) (span (offset 9220) (line 154) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 9220) (line 154) (column 28) (len 4)))))
    (reference r75 (scope relative) (span (offset 9215) (line 154) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 9215) (line 154) (column 23) (len 3)))))
    (reference r76 (scope relative) (span (offset 9254) (line 155) (column 29) (len 21)) (segments (segment 0 (token "LuminousIntensityUnit") (name "LuminousIntensityUnit") (separator none) (span (offset 9254) (line 155) (column 29) (len 21)))))
    (reference r77 (scope relative) (span (offset 9248) (line 155) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 9248) (line 155) (column 23) (len 4)))))
    (reference r78 (scope relative) (span (offset 9421) (line 160) (column 44) (len 10)) (segments (segment 0 (token "SimpleUnit") (name "SimpleUnit") (separator none) (span (offset 9421) (line 160) (column 44) (len 10)))))
    (reference r79 (scope relative) (span (offset 9481) (line 161) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 9481) (line 161) (column 48) (len 19)))))
    (reference r80 (scope relative) (span (offset 9510) (line 161) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 9510) (line 161) (column 77) (len 8)))))
    (reference r81 (scope relative) (span (offset 9521) (line 161) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 9521) (line 161) (column 88) (len 3)))))
    (reference r82 (scope relative) (span (offset 9525) (line 161) (column 92) (len 1)) (segments (segment 0 (token "J") (name "J") (separator none) (span (offset 9525) (line 161) (column 92) (len 1)))))
    (reference r83 (scope relative) (span (offset 9532) (line 161) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 9532) (line 161) (column 99) (len 8)))))
    (reference r84 (scope relative) (span (offset 9570) (line 162) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 9570) (line 162) (column 23) (len 17)))))
    (reference r85 (scope relative) (span (offset 9594) (line 162) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 9594) (line 162) (column 47) (len 20)))))
    (reference r86 (scope relative) (span (offset 9617) (line 162) (column 70) (len 19)) (segments (segment 0 (token "luminousIntensityPF") (name "luminousIntensityPF") (separator none) (span (offset 9617) (line 162) (column 70) (len 19)))))
    (reference r87 (scope relative) (span (offset 9759) (line 166) (column 45) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 9759) (line 166) (column 45) (len 19)))))
    (reference r88 (scope relative) (span (offset 11123) (line 179) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 11123) (line 179) (column 28) (len 4)))))
    (reference r89 (scope relative) (span (offset 11118) (line 179) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 11118) (line 179) (column 23) (len 3)))))
    (reference r90 (scope relative) (span (offset 11157) (line 180) (column 29) (len 21)) (segments (segment 0 (token "AmountOfSubstanceUnit") (name "AmountOfSubstanceUnit") (separator none) (span (offset 11157) (line 180) (column 29) (len 21)))))
    (reference r91 (scope relative) (span (offset 11151) (line 180) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 11151) (line 180) (column 23) (len 4)))))
    (reference r92 (scope relative) (span (offset 11324) (line 185) (column 44) (len 10)) (segments (segment 0 (token "SimpleUnit") (name "SimpleUnit") (separator none) (span (offset 11324) (line 185) (column 44) (len 10)))))
    (reference r93 (scope relative) (span (offset 11384) (line 186) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 11384) (line 186) (column 48) (len 19)))))
    (reference r94 (scope relative) (span (offset 11413) (line 186) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 11413) (line 186) (column 77) (len 8)))))
    (reference r95 (scope relative) (span (offset 11424) (line 186) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 11424) (line 186) (column 88) (len 3)))))
    (reference r96 (scope relative) (span (offset 11428) (line 186) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 11428) (line 186) (column 92) (len 1)))))
    (reference r97 (scope relative) (span (offset 11435) (line 186) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 11435) (line 186) (column 99) (len 8)))))
    (reference r98 (scope relative) (span (offset 11473) (line 187) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 11473) (line 187) (column 23) (len 17)))))
    (reference r99 (scope relative) (span (offset 11497) (line 187) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 11497) (line 187) (column 47) (len 20)))))
    (reference r100 (scope relative) (span (offset 11520) (line 187) (column 70) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 11520) (line 187) (column 70) (len 19)))))
  )
  (root (library-package (name "ISQBase") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 49) (line 3) (column 7) (len 362)) (normalized "International System of Quantities and Units\nGenerated on 2025-03-13T15:00:05Z from standard ISO/IEC 80000\n\nNote 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,\nwith Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.\n"))) (import (target (span (span (offset 434) (line 11) (column 20) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 473) (line 12) (column 20) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 483) (line 12) (column 30) (len 3))) (separator (span (offset 483) (line 12) (column 30) (len 2))) (marker (span (offset 485) (line 12) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 507) (line 13) (column 20) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 528) (line 13) (column 41) (len 3))) (separator (span (offset 528) (line 13) (column 41) (len 2))) (marker (span (offset 530) (line 13) (column 43) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 540) (line 15) (column 7) (len 31)) (normalized "ISO-80000-3 item 3-1.1 length "))) (attribute-def (declaration-name "LengthValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 651) (line 18) (column 11) (len 480)) (normalized "source: item 3-1.1 length\nsymbol(s): `l`, `L`\napplication domain: generic\nname: Length\nquantity dimension: L^1\nmeasurement unit(s): m\ntensor order: 0\ndefinition: linear extent in space between any two points\nremarks: Length does not need to be measured along a straight line. Length is one of the seven base quantities in the International System of Units (ISO 80000-1).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r5)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r7)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "LengthUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r10)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1407) (line 36) (column 77) (len 5)) (member-access (base (expression (span (offset 1407) (line 36) (column 77) (len 3)) (ref r11))) (separator dot) (member (ref r12))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r13)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1429) (line 36) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r14)))) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 1503) (line 37) (column 70) (len 8)) (ref r16))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1528) (line 40) (column 7) (len 37)) (normalized "ISO-80000-3 item 3-9 duration, time "))) (attribute-def (declaration-name "DurationValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1647) (line 43) (column 11) (len 518)) (normalized "source: item 3-9 duration, time\nsymbol(s): `t`\napplication domain: generic\nname: Duration\nquantity dimension: T^1\nmeasurement unit(s): s\ntensor order: 0\ndefinition: measure of the time difference between two events\nremarks: Duration is often just called time. Time is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). Duration is a measure of a time interval.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r21)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "DurationUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r24)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2451) (line 61) (column 79) (len 5)) (member-access (base (expression (span (offset 2451) (line 61) (column 79) (len 3)) (ref r25))) (separator dot) (member (ref r26))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r27)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2473) (line 61) (column 101) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r28)))) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r29)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2547) (line 62) (column 70) (len 10)) (ref r30))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2574) (line 65) (column 7) (len 27)) (normalized "ISO-80000-4 item 4-1 mass "))) (attribute-def (declaration-name "MassValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r31)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 2679) (line 68) (column 11) (len 571)) (normalized "source: item 4-1 mass\nsymbol(s): `m`\napplication domain: generic\nname: Mass\nquantity dimension: M^1\nmeasurement unit(s): kg\ntensor order: 0\ndefinition: property of a body which expresses itself in terms of inertia with regard to changes in its state of motion as well as its gravitational attraction to other bodies\nremarks: The kilogram (kg) is one of the seven base units (see ISO 80000-1) of the International System of Units, the SI. See also IEC 60050-113.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r32)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r33)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r34)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r35)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "MassUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r36)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r37)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r38)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3516) (line 86) (column 75) (len 5)) (member-access (base (expression (span (offset 3516) (line 86) (column 75) (len 3)) (ref r39))) (separator dot) (member (ref r40))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r41)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3538) (line 86) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r42)))) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r43)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3612) (line 87) (column 70) (len 6)) (ref r44))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 3635) (line 90) (column 7) (len 61)) (normalized "ISO-80000-5 item 5-1 thermodynamic temperature, temperature "))) (attribute-def (declaration-name "ThermodynamicTemperatureValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r45)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 3794) (line 93) (column 11) (len 1984)) (normalized "source: item 5-1 thermodynamic temperature, temperature\nsymbol(s): `T`, `Θ`\napplication domain: generic\nname: ThermodynamicTemperature\nquantity dimension: Θ^1\nmeasurement unit(s): K\ntensor order: 0\ndefinition: partial derivative of internal energy with respect to entropy at constant volume and constant number of particles in the system: `T = ((partial U)/(partial S))_(V,N)` where `U` is internal energy (item 5-20.2), `S` is entropy (item 5-18), `V` is volume (ISO 80000-3), and `N` is number of particles\nremarks: It is measured with a primary thermometer, examples of which are gas thermometers of different kinds, noise thermometers, or radiation thermometers. The Boltzmann constant (ISO 80000-1) relates energy at the individual particle level with thermodynamic temperature. Differences of thermodynamic temperatures or changes may be expressed either in kelvin, symbol K, or in degrees Celsius, symbol °C (item 5-2). Thermodynamic temperature is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). The International Temperature Scale of 1990. For the purpose of practical measurements, the International Temperature Scale of 1990, ITS-90, was adopted by CIPM in 1989, which is a close approximation to the thermodynamic temperature scale. The quantities defined by this scale are denoted `T_90` and `t_90`, respectively (replacing `T_68` and `t_68` defined by the International Practical Temperature Scale of 1968, IPTS-68), where `t_90/(1 °C) = T_90/(1 K) - 273,15`. The units of `T_90` and `t_90` are the kelvin, symbol K, and the degree Celsius, symbol °C (item 5-2), respectively. For further information, see References [5], [6]. For ready conversion between temperatures reported on the International Temperature Scale and thermodynamic temperatures the systematic deviations can be found in Reference [7].\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r46)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r47)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r48)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r49)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ThermodynamicTemperatureUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r50)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r51)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r52)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6144) (line 111) (column 95) (len 8)) (member-access (base (expression (span (offset 6144) (line 111) (column 95) (len 3)) (ref r53))) (separator dot) (member (ref r54))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r55)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6169) (line 111) (column 120) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r56)))) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r57)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6243) (line 112) (column 70) (len 26)) (ref r58))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 6286) (line 115) (column 7) (len 39)) (normalized "IEC-80000-6 item 6-1 electric current "))) (attribute-def (declaration-name "ElectricCurrentValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r59)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 6414) (line 118) (column 11) (len 784)) (normalized "source: item 6-1 electric current\nsymbol(s): `I`, `i`\napplication domain: generic\nname: ElectricCurrent\nquantity dimension: I^1\nmeasurement unit(s): A\ntensor order: 0\ndefinition: electric current is one of the base quantities in the International System of Quantities, ISQ, on which the International System of Units, SI, is based\nremarks: Electric current is the quantity that can often be measured with an ammeter. The electric current through a surface is the quotient of the electric charge (item 6-2) transferred through the surface during a time interval by the duration of that interval. For a more complete definition, see item 6-8 and IEC 60050-121, item 121-11-13.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r60)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r61)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r62)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r63)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "ElectricCurrentUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r64)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "electricCurrentPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r65)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r66)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7519) (line 136) (column 86) (len 5)) (member-access (base (expression (span (offset 7519) (line 136) (column 86) (len 3)) (ref r67))) (separator dot) (member (ref r68))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r69)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7541) (line 136) (column 108) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r70)))) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r71)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7615) (line 137) (column 70) (len 17)) (ref r72))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 7649) (line 140) (column 7) (len 42)) (normalized "ISO-80000-7 item 7-14 luminous intensity "))) (attribute-def (declaration-name "LuminousIntensityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r73)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 7782) (line 143) (column 11) (len 1408)) (normalized "source: item 7-14 luminous intensity\nsymbol(s): `I_v`, `(I)`\napplication domain: generic\nname: LuminousIntensity\nquantity dimension: J^1\nmeasurement unit(s): cd\ntensor order: 0\ndefinition: density of luminous flux with respect to solid angle in a specified direction, expressed by `I_v = (dΦ_v)/(dΩ)` where `Φ_v` is the luminous flux (item 7-13) emitted in a specified direction, and `Ω` is the solid angle (ISO 80000-3) containing that direction\nremarks: The definition holds strictly only for a point source. The distribution of the luminous intensities as a function of the direction of emission, e.g. given by the polar angles `(θ,ϕ)`, is used to determine the luminous flux (item 7-13) within a certain solid angle (ISO 80000-3) `Ω` of a source: `Φ_v = int int_Ω I_v(θ,φ) sin(θ) dφ dθ`. Luminous intensity can be derived from the spectral radiant intensity distribution by `I_v = K_m int_0^∞ I_(e,λ)(λ) V(λ) dλ`, where `K_m` is maximum luminous efficacy (item 7-11.3), `I_(e,λ)(λ)` is the spectral radiant intensity (item 7-5.2) at wavelength `λ` (ISO 80000-3), and `V(λ)` is spectral luminous efficiency (item 7-10.2). The corresponding radiometric quantity is \"radiant intensity\" (item 7-5.1). The corresponding quantity for photons is \"photon intensity\" (item 7-21).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r74)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r75)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r76)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r77)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "LuminousIntensityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r78)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "luminousIntensityPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r79)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r80)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9521) (line 161) (column 88) (len 5)) (member-access (base (expression (span (offset 9521) (line 161) (column 88) (len 3)) (ref r81))) (separator dot) (member (ref r82))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r83)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9543) (line 161) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r84)))) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r85)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9617) (line 162) (column 70) (len 19)) (ref r86))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 9653) (line 165) (column 7) (len 59)) (normalized "ISO-80000-9 item 9-2 amount of substance, number of moles "))) (attribute-def (declaration-name "AmountOfSubstanceValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r87)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 9803) (line 168) (column 11) (len 1290)) (normalized "source: item 9-2 amount of substance, number of moles\nsymbol(s): `n(X)`\napplication domain: generic\nname: AmountOfSubstance\nquantity dimension: N^1\nmeasurement unit(s): mol\ntensor order: 0\ndefinition: quotient of number `N` of specified elementary entities of kind `X` (item 9-1) in a sample, and the Avogadro constant `N_A` (ISO 80000-1): `n(X) = N(X)/N_A`\nremarks: Amount of substance is one of the seven base quantities in the International System of Quantities, ISQ (see ISO 80000-1). Elementary entities, such as molecules, atoms, ions, electrons, holes and other quasi-particles, double bonds can be used. It is necessary to specify precisely the entity involved, e.g. atoms of hydrogen `H` vs. molecules of hydrogen `H_2`, preferably by giving the molecular chemical formula of the material involved. In the name \"amount of substance\", the words \"of substance\" could be replaced by words specifying the substance concerned, e.g. \"amount of hydrogen chloride, `HCl`\", or \"amount of benzene, `C_6H_6`\". The name \"number of moles\" is often used for \"amount of substance\", but this is deprecated because the name of a quantity should be distinguished from the name of the unit.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r88)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r89)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r90)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r91)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-usage) (attribute-def (declaration-name "AmountOfSubstanceUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r92)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r93)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r94)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11424) (line 186) (column 88) (len 5)) (member-access (base (expression (span (offset 11424) (line 186) (column 88) (len 3)) (ref r95))) (separator dot) (member (ref r96))))))) (body semicolon)) (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r97)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11446) (line 186) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r98)))) (references none) (crosses none) (intersects none) (value none) (body brace (default-reference-usage (prefix (direction none) (derived false) (variance none) (constant false)) (declaration-name none) (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r99)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11520) (line 187) (column 70) (len 19)) (ref r100))))) (body semicolon)))))) (attribute-usage))))
)
~~~
