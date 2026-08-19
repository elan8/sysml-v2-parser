# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/ISQChemistryMolecular"))
~~~
# SOURCE
~~~sysml
standard library package ISQChemistryMolecular {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-9:2019 "Physical chemistry and molecular physics"
     * see also https://www.iso.org/standard/64979.html
     * 
     * Note 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,
     * with Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.
     * Note 2: For vector and tensor quantities currently the unit and quantity value type for their (scalar) magnitude is 
     * defined, as well as their typical Cartesian 3d VectorMeasurementReference (i.e. coordinate system) 
     * or TensorMeasurementReference.
     */

    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import ISQBase::*;

    /* Quantity definitions referenced from other ISQ packages */
    private import ISQSpaceTime::AngularMeasureValue;
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-9 item 9-1 number of entities */
    attribute numberOfEntities: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 9-1 number of entities
         * symbol(s): `N(X)`, `N_X`
         * application domain: generic
         * name: NumberOfEntities (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of elementary entities of kind `X` in a system
         * remarks: The elementary entities must be specified and can be atoms, molecules, ions, electrons, other particle, or a specified group of such particles. It is important to always give a precise specification of the entity involved; this should preferably be done by the empirical chemical formula of the material involved.
         */
    }

    /* ISO-80000-9 item 9-2 amount of substance, number of moles */
    /* See package ISQBase for the declarations of AmountOfSubstanceValue and AmountOfSubstanceUnit */

    alias NumberOfMolesUnit for AmountOfSubstanceUnit;
    alias NumberOfMolesValue for AmountOfSubstanceValue;
    alias numberOfMoles for amountOfSubstance;

    /* ISO-80000-9 item 9-3 relative atomic mass */
    attribute def RelativeAtomicMassValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-3 relative atomic mass
         * symbol(s): `A_r(X)`
         * application domain: generic
         * name: RelativeAtomicMass (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the average mass (ISO 80000-4) of atom `X` and the unified atomic mass (ISO 80000-10)
         * remarks: A similar quantity "relative molecular mass" can be defined for molecules. EXAMPLE `A_r(Cl) ~~ 35.453` `A_r(CO_2) ~~ 44` The relative atomic or relative molecular mass depends on the nuclidic composition. The International Union of Pure and Applied Chemistry (IUPAC) accepts the use of the special names "atomic weight" and "molecular weight" for the quantities "relative atomic mass" and "relative molecular mass", respectively. The use of these traditional names is deprecated.
         */
    }
    attribute relativeAtomicMass: RelativeAtomicMassValue :> scalarQuantities;

    /* ISO-80000-9 item 9-4 molar mass */
    attribute def MolarMassValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-4 molar mass
         * symbol(s): `M(X)`
         * application domain: generic
         * name: MolarMass
         * quantity dimension: M^1*N^-1
         * measurement unit(s): g/mol, kg*mol^-1
         * tensor order: 0
         * definition: for a pure substance `X`, quotient of mass `m(X)` (ISO 80000-4) and amount `n` of substance (item 9-2): `M = m/n`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarMassUnit[1];
    }

    attribute molarMass: MolarMassValue[*] nonunique :> scalarQuantities;

    attribute def MolarMassUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-5 molar volume */
    attribute def MolarVolumeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-5 molar volume
         * symbol(s): `V_m`
         * application domain: generic
         * name: MolarVolume
         * quantity dimension: L^3*N^-1
         * measurement unit(s): m^3*mol^-1
         * tensor order: 0
         * definition: for a pure substance, quotient of its volume `V` (ISO 80000-3) and amount `n` of substance (item 9-2): `V_m = V/n`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarVolumeUnit[1];
    }

    attribute molarVolume: MolarVolumeValue[*] nonunique :> scalarQuantities;

    attribute def MolarVolumeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-6.1 molar internal energy */
    attribute def MolarInternalEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-6.1 molar internal energy
         * symbol(s): `U_m`
         * application domain: generic
         * name: MolarInternalEnergy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of internal energy `U` (ISO 80000-5) and amount `n` of substance (item 9-2): `U_m = U/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarInternalEnergyUnit[1];
    }

    attribute molarInternalEnergy: MolarInternalEnergyValue[*] nonunique :> scalarQuantities;

    attribute def MolarInternalEnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-6.2 molar enthalpy */
    attribute def MolarEnthalpyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-6.2 molar enthalpy
         * symbol(s): `H_m`
         * application domain: generic
         * name: MolarEnthalpy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of enthalpy `H` (ISO 80000-5) and amount `n` of substance (item 9-2): `H_m = H/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarEnthalpyUnit[1];
    }

    attribute molarEnthalpy: MolarEnthalpyValue[*] nonunique :> scalarQuantities;

    attribute def MolarEnthalpyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-6.3 molar Helmholtz energy */
    attribute def MolarHelmholtzEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-6.3 molar Helmholtz energy
         * symbol(s): `F_m`
         * application domain: generic
         * name: MolarHelmholtzEnergy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of the Helmholtz energy `F` (ISO 80000-5) and amount `n` of substance (item 9-2): `F_m = F/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarHelmholtzEnergyUnit[1];
    }

    attribute molarHelmholtzEnergy: MolarHelmholtzEnergyValue[*] nonunique :> scalarQuantities;

    attribute def MolarHelmholtzEnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-6.4 molar Gibbs energy */
    attribute def MolarGibbsEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-6.4 molar Gibbs energy
         * symbol(s): `G_m`
         * application domain: generic
         * name: MolarGibbsEnergy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of the Gibbs energy `G` (ISO 80000-5) and amount `n` of substance (item 9-2): `G_m = G/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarGibbsEnergyUnit[1];
    }

    attribute molarGibbsEnergy: MolarGibbsEnergyValue[*] nonunique :> scalarQuantities;

    attribute def MolarGibbsEnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-7 molar heat capacity */
    attribute def MolarHeatCapacityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-7 molar heat capacity
         * symbol(s): `C_m`
         * application domain: generic
         * name: MolarHeatCapacity
         * quantity dimension: L^2*M^1*T^-2*Θ^-1*N^-1
         * measurement unit(s): J/(mol*K), kg*m^2*s^-2*K^-1*mol^-1
         * tensor order: 0
         * definition: quotient of heat capacity `C` (ISO 80000-5) and amount of substance `n` (item 9-2): `C_m = C/n`
         * remarks: Conditions (constant pressure or volume etc.) must be specified.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarHeatCapacityUnit[1];
    }

    attribute molarHeatCapacity: MolarHeatCapacityValue[*] nonunique :> scalarQuantities;

    attribute def MolarHeatCapacityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-8 molar entropy */
    attribute def MolarEntropyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-8 molar entropy
         * symbol(s): `S_m`
         * application domain: generic
         * name: MolarEntropy
         * quantity dimension: L^2*M^1*T^-2*Θ^-1*N^-1
         * measurement unit(s): J/(mol*K), kg*m^2*s^-2*K^-1*mol^-1
         * tensor order: 0
         * definition: quotient of entropy `S` (ISO 80000-5) and amount `n` of substance (item 9-2): `S_m = S/n`
         * remarks: Conditions (constant pressure or volume etc.) must be specified.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarEntropyUnit[1];
    }

    attribute molarEntropy: MolarEntropyValue[*] nonunique :> scalarQuantities;

    attribute def MolarEntropyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-9.1 particle concentration */
    attribute def ParticleConcentrationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-9.1 particle concentration
         * symbol(s): `n`, `(C)`
         * application domain: generic
         * name: ParticleConcentration
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number `N` of particles (item 9-1) and volume `V `(ISO 80000-3): `n = N/V`
         * remarks: The term "number density" is also used.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ParticleConcentrationUnit[1];
    }

    attribute particleConcentration: ParticleConcentrationValue[*] nonunique :> scalarQuantities;

    attribute def ParticleConcentrationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-9 item 9-9.2 molecular concentration */
    attribute molecularConcentration: ParticleConcentrationValue :> scalarQuantities {
        doc
        /*
         * source: item 9-9.2 molecular concentration
         * symbol(s): `C(X)`, `C_X`
         * application domain: generic
         * name: MolecularConcentration (specializes ParticleConcentration)
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of number `N_X` of molecules of substance `X` and volume `V` (ISO 80000-3) of the mixture: `C_X = N_X/V`
         * remarks: None.
         */
    }

    /* ISO-80000-9 item 9-10 mass concentration */
    attribute def MassConcentrationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-10 mass concentration
         * symbol(s): `γ_X`, `(ρ_X)`
         * application domain: generic
         * name: MassConcentration
         * quantity dimension: L^-3*M^1
         * measurement unit(s): g/l, kg*m^-3
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of mass `m_X` (ISO 80000-4) of substance `X` and volume `V` (ISO 80000-3) of the mixture: `γ_X = m_X/V`
         * remarks: Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassConcentrationUnit[1];
    }

    attribute massConcentration: MassConcentrationValue[*] nonunique :> scalarQuantities;

    attribute def MassConcentrationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-9 item 9-11 mass fraction */
    attribute def MassFractionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-11 mass fraction
         * symbol(s): `w_X`
         * application domain: generic
         * name: MassFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of mass `m_X` (ISO 80000-4) of substance `X` and total mass `m` of the mixture: `w_X = m_X/m`
         * remarks: None.
         */
    }
    attribute massFraction: MassFractionValue :> scalarQuantities;

    /* ISO-80000-9 item 9-12.1 amount-of-substance concentration */
    attribute def AmountOfSubstanceConcentrationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-12.1 amount-of-substance concentration
         * symbol(s): `c_X`
         * application domain: generic
         * name: AmountOfSubstanceConcentration
         * quantity dimension: L^-3*N^1
         * measurement unit(s): mol/l, mol*m^-3
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of amount `n_X` of substance (item 9-2) of `X` and volume `V` (ISO 80000-3) of the mixture: `c_X = n_X/V`
         * remarks: In chemistry, the name "amount-of-substance concentration" is generally abbreviated to the single word "concentration", it being assumed that the adjective "amount-of-substance" is intended. For this reason, however, the word "mass" should never be omitted from the name "mass concentration" in item 9-10. Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AmountOfSubstanceConcentrationUnit[1];
    }

    attribute amountOfSubstanceConcentration: AmountOfSubstanceConcentrationValue[*] nonunique :> scalarQuantities;

    attribute def AmountOfSubstanceConcentrationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-12.2 standard amount-of-substance concentration */
    attribute standardAmountOfSubstanceConcentration: AmountOfSubstanceConcentrationValue :> scalarQuantities {
        doc
        /*
         * source: item 9-12.2 standard amount-of-substance concentration
         * symbol(s): `c^!(X)`
         * application domain: generic
         * name: StandardAmountOfSubstanceConcentration (specializes AmountOfSubstanceConcentration)
         * quantity dimension: L^-3*N^1
         * measurement unit(s): mol/l, mol*m^-3
         * tensor order: 0
         * definition: for substance `X`, one mole per litre
         * remarks: Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
    }

    /* ISO-80000-9 item 9-13 amount-of-substance fraction mole fraction */
    attribute def AmountOfSubstanceFractionMoleFractionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-13 amount-of-substance fraction mole fraction
         * symbol(s): `x_X`, `y_X`
         * application domain: generic
         * name: AmountOfSubstanceFractionMoleFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of amount of substance `n_X` (item 9-2) of `X` and total amount `n` of substance (item 9-2) in the mixture: `x_X = n_X/n`
         * remarks: For condensed phases, `x_X` is used, and for gaseous mixtures `y_X` may be used. The unsystematic name "mole fraction" is still used. However, the use of this name is deprecated. For this quantity, the entity used to define the amount of substance should always be a single molecule for every species in the mixture.
         */
    }
    attribute amountOfSubstanceFractionMoleFraction: AmountOfSubstanceFractionMoleFractionValue :> scalarQuantities;

    /* ISO-80000-9 item 9-14 volume fraction */
    attribute def VolumeFractionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-14 volume fraction
         * symbol(s): `φ_X`
         * application domain: generic
         * name: VolumeFraction
         * quantity dimension: 1
         * measurement unit(s): ml/l, 1
         * tensor order: 0
         * definition: for substance `X`, quotient of product of amount of substance fraction `x_X` (item 9-13) of `X` and molar volume `V_(m,X)` (item 9-5) of the pure substance `X` at the same temperature (ISO 80000-5) and pressure (ISO 80000-4), and sum over all substances `i` of products of amount-of-substance fractions `x_i` (item 9-13) of substance `i` and their molar volumes `V_(m,i)` (item 9-5): `φ_X = (x_X V_(m,X))/(sum_i x_i V_(m,i))`
         * remarks: Generally, the volume fraction is temperature dependent. Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
        attribute :>> num: Real;
        attribute :>> mRef: VolumeFractionUnit[1];
    }

    attribute volumeFraction: VolumeFractionValue[*] nonunique :> scalarQuantities;

    attribute def VolumeFractionUnit :> DimensionOneUnit {
    }

    /* ISO-80000-9 item 9-15 molality */
    attribute def MolalityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-15 molality
         * symbol(s): `b_B`, `m_B`
         * application domain: generic
         * name: Molality
         * quantity dimension: M^-1*N^1
         * measurement unit(s): mol/kg
         * tensor order: 0
         * definition: quotient of amount of substance (item 9-2) of solute `B` and mass `m_A` (ISO 80000-4) of the solvent substance `A`: `b_B = n_B/m_A`
         * remarks: The alternative symbol `m_B` should be avoided in situations where it might be mistaken for the mass of substance B. However, the symbol `m_B` is much more commonly used than the symbol `b_B` for molality, despite the possible confusion with mass.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolalityUnit[1];
    }

    attribute molality: MolalityValue[*] nonunique :> scalarQuantities;

    attribute def MolalityUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-16 latent heat of phase transition, enthalpy of phase transition */
    attribute latentHeatOfPhaseTransition: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 9-16 latent heat of phase transition, enthalpy of phase transition
         * symbol(s): `C_"pt"`
         * application domain: generic
         * name: LatentHeatOfPhaseTransition (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) necessary to be added or subtracted isothermally and isobarically to a system to completely undergo the phase transition
         * remarks: Mostly, molar or specific quantity is used and phase transition is expressed explicitly, e.g. molar latent heat of evaporation. The subscript "pt" is the qualifier for the phase transition, which may be changed to e.g. "l-g". The term "enthalpy of phase transition" is mainly used in theory.
         */
    }

    alias enthalpyOfPhaseTransition for latentHeatOfPhaseTransition;

    /* ISO-80000-9 item 9-17 chemical potential */
    attribute def ChemicalPotentialValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-17 chemical potential
         * symbol(s): `μ_X`
         * application domain: chemistry
         * name: ChemicalPotential
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: partial derivative of the Gibbs energy (ISO 80000-5) with respect to amount `n_X` of substance `X` (item 9-2) at constant temperature `T` (ISO 80000-5) and pressure `p `(ISO 80000-4): `μ_X = ((del G)/(del n_X))_(T,p)`
         * remarks: For a pure substance, where `G_m` is the molar Gibbs energy. In a mixture, `μ_B` is the partial molar Gibbs energy. In condensed matter physics, the chemical potential of electrons is energy.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ChemicalPotentialUnit[1];
    }

    attribute chemicalPotential: ChemicalPotentialValue[*] nonunique :> scalarQuantities;

    attribute def ChemicalPotentialUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-18 absolute activity */
    attribute def AbsoluteActivityValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-18 absolute activity
         * symbol(s): `λ_X`
         * application domain: generic
         * name: AbsoluteActivity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X`, exponential of quotient of chemical potential `μ_X` of substance `B` (item 9-17), and product of molar gas constant `R` (item 9-37.1) and thermodynamic temperature `T` (ISO 80000-5): `λ_X = exp(μ_X/(RT))`
         * remarks: None.
         */
    }
    attribute absoluteActivity: AbsoluteActivityValue :> scalarQuantities;

    /* ISO-80000-9 item 9-19 partial pressure */
    attribute def PartialPressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-19 partial pressure
         * symbol(s): `p_X`
         * application domain: generic
         * name: PartialPressure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: for substance `X` in a gaseous mixture, product of amount-of-substance fraction `y_X` of substance X (item 9-13) and total pressure `p` (ISO 80000-4): `p_X = y_X p`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PartialPressureUnit[1];
    }

    attribute partialPressure: PartialPressureValue[*] nonunique :> scalarQuantities;

    attribute def PartialPressureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-9 item 9-20 fugacity */
    attribute def FugacityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-20 fugacity
         * symbol(s): `tilde(p)_X`
         * application domain: generic
         * name: Fugacity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: for substance `X`, quantity proportional to the absolute activity, `λ_X` (item 9-18), the proportionality factor, which is a function of temperature (ISO 80000-5) only, being determined by the condition that, at constant temperature and composition, `p_X/tilde(p)_X` tends to 1 for an indefinitely dilute gas
         * remarks: `tilde(p)_X = λ_X * lim_(p->0) (p_X/λ_X)` where `p` is total pressure (ISO 80000-4). The IUPAC preferred symbol for fugacity is `f`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: FugacityUnit[1];
    }

    attribute fugacity: FugacityValue[*] nonunique :> scalarQuantities;

    attribute def FugacityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-9 item 9-21 standard chemical potential */
    attribute def StandardChemicalPotentialValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-21 standard chemical potential
         * symbol(s): `μ_B^!`, `μ^!`
         * application domain: generic
         * name: StandardChemicalPotential
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: for substance `B`, value of the chemical potential (item 9-17) at specified standard conditions
         * remarks: `μ_B^! = RT ln(λ^!)` where `μ_B^!` is a function of temperature `T` at the standard pressure `p = p^!` The standard chemical potential depends on the choice of standard state, which must be specified. In a liquid or solid solution, the standard state is referenced to the ideal dilute behaviour of the solute (substance `B`).
         */
        attribute :>> num: Real;
        attribute :>> mRef: StandardChemicalPotentialUnit[1];
    }

    attribute standardChemicalPotential: StandardChemicalPotentialValue[*] nonunique :> scalarQuantities;

    attribute def StandardChemicalPotentialUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-22 activity factor */
    attribute def ActivityFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-22 activity factor
         * symbol(s): `f_X`
         * application domain: generic
         * name: ActivityFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a liquid or a solid mixture, quotient of absolute activity `λ_X` (item 9-18) of substance `X` and the product of absolute activity `λ_X^"*"` of the pure substance `X` at the same temperature (ISO 80000-5) and pressure (ISO 80000-4) and amount-of-substance fraction `x_X` of substance `X` (item 9-13): `f_X = λ_X/(λ_X^"*" x_X)`
         * remarks: The systematic name is "activity factor", but the name "activity coefficient" is also commonly used (see item 9-25). Activity factors can also be obtained applying Raoult’s law or Henry’s law.
         */
    }
    attribute activityFactor: ActivityFactorValue :> scalarQuantities;

    /* ISO-80000-9 item 9-23 standard absolute activity in mixture */
    attribute def StandardAbsoluteActivityInMixtureValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-23 standard absolute activity in mixture
         * symbol(s): `λ_X^!`
         * application domain: in a mixture
         * name: StandardAbsoluteActivityInMixture (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a liquid or a solid mixture, absolute activity `λ_X^"*"` (item 9-18) of the pure substance `X` at the same temperature (ISO 80000-5) but at standard pressure (ISO 80000-4) `10^5 ["Pa"]`: `λ_X^! = λ_X"*" (p^!)`
         * remarks: This quantity is a function of temperature only.
         */
    }
    attribute standardAbsoluteActivityInMixture: StandardAbsoluteActivityInMixtureValue :> scalarQuantities;

    /* ISO-80000-9 item 9-24 activity of solute, relative activity of solute */
    attribute def ActivityOfSoluteValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-24 activity of solute, relative activity of solute
         * symbol(s): `a_X`, `a_(m,X)`
         * application domain: generic
         * name: ActivityOfSolute (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a solute `X` in a solution, quantity proportional to the absolute activity, `λ_X` (item 9-18), the proportionality factor, which is a function of temperature (ISO 80000-5) and pressure (ISO 80000-4) only, being determined by the condition that, at constant temperature and pressure, `a_X` divided by the molality (item 9-15) ratio, `b_X/b^!` tends to 1 at infinite dilution; `b_X` is the molality of solute `X` (item 9-15), and `b^!` is standard molality: `a_X = λ_X * lim_(sum b_X -> 0) (b_X//b^!)/λ_X`
         * remarks: The quantity `a_(c,X)` , similarly defined in terms of the concentration ratio `c_X/c^!` , is also called the activity or relative activity of solute `X`; `c^!` is a standard amount-of-substance concentration (item 9-12.2): `a_(c,X) = λ_X * lim_(sum c_X -> 0) (c_X//c^!)/λ_X`, where `sum` denotes summation over all the solute substances. This especially applies to a dilute liquid solution.
         */
    }
    attribute activityOfSolute: ActivityOfSoluteValue :> scalarQuantities;

    alias relativeActivityOfSolute for activityOfSolute;

    /* ISO-80000-9 item 9-25 activity coefficient */
    attribute def ActivityCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-25 activity coefficient
         * symbol(s): `γ_B`
         * application domain: generic
         * name: ActivityCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a solute `B` in a solution, quotient of activity `a_B` of solute `B` (item 9-24), and quotient of the molality (item 9-15) `b_B` of substance `B` and standard molality `b^!`: `γ_B = a_B/(b_B//b^!)`
         * remarks: The name "activity coefficient of solute B" is also used for the quantity `γ_B` defined as: `γ_B = a_(c,B)/(c_B//c^!)` See item 9-22.
         */
    }
    attribute activityCoefficient: ActivityCoefficientValue :> scalarQuantities;

    /* ISO-80000-9 item 9-26 standard absolute activity in solution */
    attribute def StandardAbsoluteActivityInSolutionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-26 standard absolute activity in solution
         * symbol(s): `λ_B^!`
         * application domain: in a solution
         * name: StandardAbsoluteActivityInSolution (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a solute `B` in a solution: `λ_B^! = lim_(sum b_B -> 0) [λ_B ((p^!)b^!)/b_B]` where ∑ denotes summation over all solutes, `p^!` is a standard pressure (ISO 80000-4), `b^!` is standard molality, and `b_B` is the molality of substance `B` (item 9-15)
         * remarks: This quantity is a function of temperature only. It especially applies to a dilute liquid solution. The standard pressure is `10^5 ["Pa"]`.
         */
    }
    attribute standardAbsoluteActivityInSolution: StandardAbsoluteActivityInSolutionValue :> scalarQuantities;

    /* ISO-80000-9 item 9-27.1 activity of solvent, relative activity of solvent */
    attribute def ActivityOfSolventValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-27.1 activity of solvent, relative activity of solvent
         * symbol(s): `a_A`
         * application domain: generic
         * name: ActivityOfSolvent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for the solvent `A` in a solution, quotient of the absolute activity of substance `A`, `λ_A` (item 9-18), and that, `λ_A^"*"` , of the pure solvent at the same temperature (ISO 80000-5) and pressure (ISO 80000-4): `a_A = λ_A/λ_A^"*"`
         * remarks: None.
         */
    }
    attribute activityOfSolvent: ActivityOfSolventValue :> scalarQuantities;

    alias relativeActivityOfSolvent for activityOfSolvent;

    /* ISO-80000-9 item 9-27.2 osmotic factor of solvent, osmotic coefficient of solvent A */
    attribute def OsmoticFactorOfSolventValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-27.2 osmotic factor of solvent, osmotic coefficient of solvent A
         * symbol(s): `φ`
         * application domain: generic
         * name: OsmoticFactorOfSolvent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `φ = -(M_A sum b_B)^-1 ln(a_A)` where `M_A` is the molar mass (item 9-4) of the solvent A, ∑ denotes summation over all the solutes, `b_B` is the molality of solute B (item 9-15), and `a_A` is the activity of solvent A (item 9-27.1)
         * remarks: The name "osmotic coefficient" is generally used, although the name "osmotic factor" is more systematic. This concept especially applies to a dilute liquid solution.
         */
    }
    attribute osmoticFactorOfSolvent: OsmoticFactorOfSolventValue :> scalarQuantities;

    alias osmoticCoefficientOfSolventA for osmoticFactorOfSolvent;

    /* ISO-80000-9 item 9-27.3 standard absolute activity of solvent */
    attribute def StandardAbsoluteActivityOfSolventValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-27.3 standard absolute activity of solvent
         * symbol(s): `λ_A^!`
         * application domain: in a dilute solution
         * name: StandardAbsoluteActivityOfSolvent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for solvent `A`, standard absolute activity (item 9-23) of the pure substance `A` at the same temperature (ISO 80000-5) and at a standard pressure `p^!` (ISO 80000-4): `λ_A^! = λ_A^"*" p^!`
         * remarks: None.
         */
    }
    attribute standardAbsoluteActivityOfSolvent: StandardAbsoluteActivityOfSolventValue :> scalarQuantities;

    /* ISO-80000-9 item 9-28 osmotic pressure */
    attribute def OsmoticPressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-28 osmotic pressure
         * symbol(s): `Π`
         * application domain: generic
         * name: OsmoticPressure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: excess pressure (ISO 80000-4) required to maintain osmotic equilibrium between a solution and the pure solvent separated by a membrane permeable to the solvent only
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: OsmoticPressureUnit[1];
    }

    attribute osmoticPressure: OsmoticPressureValue[*] nonunique :> scalarQuantities;

    attribute def OsmoticPressureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-9 item 9-29 stoichiometric number of substance */
    attribute def StoichiometricNumberOfSubstanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-29 stoichiometric number of substance
         * symbol(s): `ν_B`
         * application domain: generic
         * name: StoichiometricNumberOfSubstance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `B`, an integer number or a simple fraction, being negative for a reactant and positive for a product, occurring in the expression for a chemical reaction: `0 = sum ν_B` where the symbol `B` denotes the reactants and products involved in the reaction
         * remarks: EXAMPLE `(1/2)"N"_2 + (3/2)"H"_2 = "N""H"_3` ; `ν("N"_2) = -1/2`, `ν("H"_2) = -3/2`, `ν("N""H"_3) = +1`.
         */
    }
    attribute stoichiometricNumberOfSubstance: StoichiometricNumberOfSubstanceValue :> scalarQuantities;

    /* ISO-80000-9 item 9-30 affinity of a chemical reaction */
    attribute def AffinityOfAChemicalReactionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-30 affinity of a chemical reaction
         * symbol(s): `A`
         * application domain: generic
         * name: AffinityOfAChemicalReaction
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: negative of the sum over all substances `B` of products of stoichiometric number `ν_B` of substance `B` (item 9-29) and chemical potential `μ_B` of substance `B` (item 9-17): `A = -sum ν_B μ_B`
         * remarks: The affinity of a reaction is a measure of the "driving force" of the reaction. When it is positive, the reaction goes spontaneously from reactants to products, and when it is negative, the reaction goes in the opposite direction. Another way to write the definition is: `A = ((del G)/(del ξ))_(p,T)` where `G` is Gibbs energy (ISO 80000-5) and `ξ` is the extent of the reaction (item 9-31). Note that `ν_B` is negative for reactants and positive for products.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AffinityOfAChemicalReactionUnit[1];
    }

    attribute affinityOfAChemicalReaction: AffinityOfAChemicalReactionValue[*] nonunique :> scalarQuantities;

    attribute def AffinityOfAChemicalReactionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-31 extent of reaction */
    attribute extentOfReaction: AmountOfSubstanceValue :> scalarQuantities {
        doc
        /*
         * source: item 9-31 extent of reaction
         * symbol(s): `ξ`
         * application domain: generic
         * name: ExtentOfReaction (specializes AmountOfSubstance)
         * quantity dimension: N^1
         * measurement unit(s): mol
         * tensor order: 0
         * definition: difference of initial amount `n_(B "in")` of substance `B` (item 9-2) and equilibrium amount `n_(B "eq")` of substance `B` (item 9-2) divided by stoichiometric number `ν_B` of substance `B` (item 9-29): `ξ = (n_(B "eq") - n_(B "in"))/ν_B`
         * remarks: See remark to item 9-30.
         */
    }

    /* ISO-80000-9 item 9-32 standard equilibrium constant, thermodynamic equilibrium constant */
    attribute def StandardEquilibriumConstantValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-32 standard equilibrium constant, thermodynamic equilibrium constant
         * symbol(s): `K^!`
         * application domain: generic
         * name: StandardEquilibriumConstant (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a chemical reaction, product for all substances `B` of standard absolute activity `λ_B^!` of substance `B` (item 9-26) in power of minus stoichiometric number `ν_B` of substance `B` (item 9-29): `K^! = prod_B (tilde(a) λ_B^!)^(-ν_B)`
         * remarks: This quantity is a function of temperature only. Others depend on temperature, pressure, and composition. One can define in an analogous way an equilibrium constant in terms of fugacity, `K_f`, molality, `K_m`, etc.
         */
    }
    attribute standardEquilibriumConstant: StandardEquilibriumConstantValue :> scalarQuantities;

    alias thermodynamicEquilibriumConstant for standardEquilibriumConstant;

    /* ISO-80000-9 item 9-33 equilibrium constant on pressure basis */
    attribute def EquilibriumConstantOnPressureBasisValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-33 equilibrium constant on pressure basis
         * symbol(s): `K_p`
         * application domain: pressure basis
         * name: EquilibriumConstantOnPressureBasis
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: for gases, product for all substances `B` of partial pressure `p_B` of substance `B` (item 9-19) in power of its stoichiometric number `ν_B` (item 9-29): `K_p = prod_B (p_B)^(ν_B)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EquilibriumConstantOnPressureBasisUnit[1];
    }

    attribute equilibriumConstantOnPressureBasis: EquilibriumConstantOnPressureBasisValue[*] nonunique :> scalarQuantities;

    attribute def EquilibriumConstantOnPressureBasisUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-9 item 9-34 equilibrium constant on concentration basis */
    attribute def EquilibriumConstantOnConcentrationBasisValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-34 equilibrium constant on concentration basis
         * symbol(s): `K_c`
         * application domain: concentration basis
         * name: EquilibriumConstantOnConcentrationBasis
         * quantity dimension: L^-3*N^1
         * measurement unit(s): mol/m^3
         * tensor order: 0
         * definition: for solutions, product for all substances `B` of concentration `c_B` of substance `B` (item 9-9.1) in power of its stoichiometric number `ν_B` (item 9-29): `K_c = prod_B (c_B)^(ν_B)`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EquilibriumConstantOnConcentrationBasisUnit[1];
    }

    attribute equilibriumConstantOnConcentrationBasis: EquilibriumConstantOnConcentrationBasisValue[*] nonunique :> scalarQuantities;

    attribute def EquilibriumConstantOnConcentrationBasisUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-35.1 microcanonical partition function */
    attribute microcanonicalPartitionFunction: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 9-35.1 microcanonical partition function
         * symbol(s): `Ω`
         * application domain: generic
         * name: MicrocanonicalPartitionFunction (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of all quantum states `r` consistent with given energy `E` (ISO 80000-4), volume (ISO 80000-3), and external fields: `Ω = sum_r 1`
         * remarks: `S = k ln(Ω)` where `S` is entropy (ISO 80000-5) and `k` is the Boltzmann constant (ISO 80000-1).
         */
    }

    /* ISO-80000-9 item 9-35.2 canonical partition function */
    attribute def CanonicalPartitionFunctionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-35.2 canonical partition function
         * symbol(s): `Z`
         * application domain: generic
         * name: CanonicalPartitionFunction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: sum over quantum states of energy `E_r` (ISO 80000-4), expressed by: `Z = sum_r exp(-E_r/(kT))` where `k` is the Boltzmann constant (ISO 80000-1), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: `A = -kT ln(Z)` where `A` is Helmholtz energy (ISO 80000-5).
         */
    }
    attribute canonicalPartitionFunction: CanonicalPartitionFunctionValue :> scalarQuantities;

    /* ISO-80000-9 item 9-35.3 grand-canonical partition function, grand partition function */
    attribute def GrandCanonicalPartitionFunctionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-35.3 grand-canonical partition function, grand partition function
         * symbol(s): `Ξ`
         * application domain: generic
         * name: GrandCanonicalPartitionFunction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: sum of canonical partition function `Z(N_A,N_B,…)` for the given number of particles `A,B` multiplied by absolute activities (item 9-18) `λ_A, λ_B, ...` of particles `A, B`: `Ξ = sum_(N_A, N_B, ...) Z(N_A, N_B, …) * λ_A^(N_A) * λ_B^(N_B) * ...`
         * remarks: `A - sum μ_B n_B = -kT ln(Ξ)` where `A` is Helmholtz energy (ISO 80000-5), `μ_B` is the chemical potential of substance `B`, and `n_B` is the amount of substance `B`.
         */
    }
    attribute grandCanonicalPartitionFunction: GrandCanonicalPartitionFunctionValue :> scalarQuantities;

    alias grandPartitionFunction for grandCanonicalPartitionFunction;

    /* ISO-80000-9 item 9-35.4 molecular partition function, partition function of a molecule */
    attribute def MolecularPartitionFunctionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-35.4 molecular partition function, partition function of a molecule
         * symbol(s): `q`
         * application domain: generic
         * name: MolecularPartitionFunction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `q = sum_r exp(-ε_r/(kT))` where `ε_r` is the energy (ISO 80000-5) of the `r`-th level of the molecule consistent with given volume (ISO 80000-3) and external fields, `k` is the Boltzmann constant (ISO 80000-1), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute molecularPartitionFunction: MolecularPartitionFunctionValue :> scalarQuantities;

    alias partitionFunctionOfAMolecule for molecularPartitionFunction;

    /* ISO-80000-9 item 9-36.1 statistical weight of subsystem */
    attribute statisticalWeightOfSubsystem: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 9-36.1 statistical weight of subsystem
         * symbol(s): `g`
         * application domain: generic
         * name: StatisticalWeightOfSubsystem (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of different microstates in a subsystem
         * remarks: None.
         */
    }

    /* ISO-80000-9 item 9-36.2 degeneracy, multiplicity */
    attribute def DegeneracyValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-36.2 degeneracy, multiplicity
         * symbol(s): `g`
         * application domain: generic
         * name: Degeneracy (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for quantum level, statistical weight of that level
         * remarks: If `g = 1`, the level is called non-degenerate.
         */
    }
    attribute degeneracy: DegeneracyValue :> scalarQuantities;

    alias multiplicity for degeneracy;

    /* ISO-80000-9 item 9-37.1 molar gas constant */
    attribute def MolarGasConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-37.1 molar gas constant
         * symbol(s): `R`
         * application domain: generic
         * name: MolarGasConstant
         * quantity dimension: L^2*M^1*T^-2*Θ^-1*N^-1
         * measurement unit(s): J/(mol*K), kg*m^2*s^-2*K^-1*mol^-1
         * tensor order: 0
         * definition: product of the Boltzmann constant (ISO 80000-1) and the Avogadro constant (ISO 80000-1)
         * remarks: For an ideal gas, `pV_m = RT`
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarGasConstantUnit[1];
    }

    attribute molarGasConstant: MolarGasConstantValue[*] nonunique :> scalarQuantities;

    attribute def MolarGasConstantUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-37.2 specific gas constant */
    /* Refer to declaration for SpecificGasConstant in ISQThermodynamics item 5-26 specific gas constant */

    /* ISO-80000-9 item 9-38 mean free path */
    attribute meanFreePath: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 9-38 mean free path
         * symbol(s): `l`, `λ`
         * application domain: chemistry
         * name: MeanFreePath (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: for a particle, the average distance `d` (ISO 80000-3) between two successive collisions with other particles
         * remarks: None.
         */
    }

    /* ISO-80000-9 item 9-39 diffusion coefficient */
    attribute def DiffusionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-39 diffusion coefficient
         * symbol(s): `D`
         * application domain: chemistry
         * name: DiffusionCoefficient
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: proportionality coefficient of local molecular concentration `C_B` (item 9-9.2) of substance `B` in the mixture multiplied by the local average velocity (ISO 80000-3) `v_B` of the molecules of `B`, and minus the gradient of the local molecular concentration `C_B` (item 9-9.2) of substance `B` in the mixture, expressed by: `C_B(v_B) = -D grad C_B`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DiffusionCoefficientUnit[1];
    }

    attribute diffusionCoefficient: DiffusionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def DiffusionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-9 item 9-40.1 thermal diffusion ratio */
    attribute def ThermalDiffusionRatioValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-40.1 thermal diffusion ratio
         * symbol(s): `k_T`
         * application domain: generic
         * name: ThermalDiffusionRatio (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in a steady-state of a binary mixture in which thermal diffusion occurs, proportionality factor between gradient of the amount-of-subsstance fraction `x_B` (item 9-13) of the heavier substance `B`, and negative gradient of the local thermodynamic temperature `T` (ISO 80000-5) divided by that temperature (ISO 80000-5): `grad x_B = -(k_T/T) grad T`
         * remarks: None.
         */
    }
    attribute thermalDiffusionRatio: ThermalDiffusionRatioValue :> scalarQuantities;

    /* ISO-80000-9 item 9-40.2 thermal diffusion factor */
    attribute def ThermalDiffusionFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-40.2 thermal diffusion factor
         * symbol(s): `α_T`
         * application domain: generic
         * name: ThermalDiffusionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the thermal diffusion ratio `k_T` (item 9-40.1), and the product of the local amount-of-substance fractions `x_A`, `x_B` (item 9-13) of two substances `A` and `B`: `α_T = k_T//(x_A x_B)`
         * remarks: None.
         */
    }
    attribute thermalDiffusionFactor: ThermalDiffusionFactorValue :> scalarQuantities;

    /* ISO-80000-9 item 9-41 thermal diffusion coefficient */
    attribute def ThermalDiffusionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-41 thermal diffusion coefficient
         * symbol(s): `D_T`
         * application domain: generic
         * name: ThermalDiffusionCoefficient
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: product of the thermal diffusion ratio `k_T` (item 9-40.1) and the diffusion coefficient `D` (item 9-39): `D_T = k_T*D`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalDiffusionCoefficientUnit[1];
    }

    attribute thermalDiffusionCoefficient: ThermalDiffusionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def ThermalDiffusionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-9 item 9-42 ionic strength */
    attribute def IonicStrengthValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-42 ionic strength
         * symbol(s): `I`
         * application domain: generic
         * name: IonicStrength
         * quantity dimension: M^-1*N^1
         * measurement unit(s): mol*kg^-1
         * tensor order: 0
         * definition: in a sample, one half of the sum of square of the charge number `z_i` (ISO 80000-10) of `i`-th ion multiplied by its molality `b_i` (item 9-15) over any involved ion: `I = 1/2 sum z_i^2 b_i`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: IonicStrengthUnit[1];
    }

    attribute ionicStrength: IonicStrengthValue[*] nonunique :> scalarQuantities;

    attribute def IonicStrengthUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-43 degree of dissociation, dissociation fraction */
    attribute def DegreeOfDissociationValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-43 degree of dissociation, dissociation fraction
         * symbol(s): `α`
         * application domain: generic
         * name: DegreeOfDissociation (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in a sample, quotient of the number `n_d` of dissociated molecules and the total number `n` of molecules: `α = n_D / n`
         * remarks: None.
         */
    }
    attribute degreeOfDissociation: DegreeOfDissociationValue :> scalarQuantities;

    alias dissociationFraction for degreeOfDissociation;

    /* ISO-80000-9 item 9-44 electrolytic conductivity */
    attribute def ElectrolyticConductivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-44 electrolytic conductivity
         * symbol(s): `κ`
         * application domain: generic
         * name: ElectrolyticConductivity
         * quantity dimension: L^-3*M^-1*T^3*I^2
         * measurement unit(s): S/m, kg^-1*m^-3*s^3*A^2
         * tensor order: 0
         * definition: quotient of the magnitude of electric current density `J` (IEC 80000-6) and the magnitude electric field strength `E` (IEC 80000-6) in an electrolyte: `κ = J/E`
         * remarks: For anisotropic media, `κ` is a tensor. In IEC 80000-6 the symbols `σ`, `γ` are used.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ElectrolyticConductivityUnit[1];
    }

    attribute electrolyticConductivity: ElectrolyticConductivityValue[*] nonunique :> scalarQuantities;

    attribute def ElectrolyticConductivityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF); }
    }

    /* ISO-80000-9 item 9-45 molar conductivity */
    attribute def MolarConductivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-45 molar conductivity
         * symbol(s): `Λ_m`
         * application domain: generic
         * name: MolarConductivity
         * quantity dimension: M^-1*T^3*I^2*N^-1
         * measurement unit(s): S*m^2/mol, kg^-1*s^3*A^2*mol^-1
         * tensor order: 0
         * definition: in an electrolyte, quotient of electrolytic conductivity `κ` (item 9-44) and amount-of-substance concentration `c_B` (item 9-12.1): `Λ_m = κ/c_B`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarConductivityUnit[1];
    }

    attribute molarConductivity: MolarConductivityValue[*] nonunique :> scalarQuantities;

    attribute def MolarConductivityUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute electricCurrentPF: QuantityPowerFactor[1] { :>> quantity = isq.I; :>> exponent = 2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-46 transport number of the ion B, current fraction of the ion B */
    attribute def TransportNumberOfTheIonBValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-46 transport number of the ion B, current fraction of the ion B
         * symbol(s): `t_B`
         * application domain: generic
         * name: TransportNumberOfTheIonB (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for the ion `B`, quotient of electric current `i_B` (IEC 80000-6) carried by the ion `B` and total electric current `i` (IEC 80000-6) in an electrolyte: `t_B = i_B/i`
         * remarks: None.
         */
    }
    attribute transportNumberOfTheIonB: TransportNumberOfTheIonBValue :> scalarQuantities;

    alias currentFractionOfTheIonB for transportNumberOfTheIonB;

    /* ISO-80000-9 item 9-47 angle of optical rotation */
    attribute angleOfOpticalRotation: AngularMeasureValue :> scalarQuantities {
        doc
        /*
         * source: item 9-47 angle of optical rotation
         * symbol(s): `α`
         * application domain: generic
         * name: AngleOfOpticalRotation (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): rad
         * tensor order: 0
         * definition: angle through which plane-polarized light is rotated clockwise, as seen when facing the light source, in passing through an optically active medium
         * remarks: None.
         */
    }

    /* ISO-80000-9 item 9-48 molar optical rotatory power */
    attribute def MolarOpticalRotatoryPowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-48 molar optical rotatory power
         * symbol(s): `α_n`
         * application domain: generic
         * name: MolarOpticalRotatoryPower
         * quantity dimension: L^2*N^-1
         * measurement unit(s): rad*m^2/mol, m^2*mol^-1
         * tensor order: 0
         * definition: angle `α` of optical rotation (item 9-47), multiplied by the quotient of cross-sectional area `A` (ISO 80000-3) of a linearly polarized light beam and the amount of substance `n` (item 9-2) of the optically active component in the path of the beam: `α_n = (α A)/n`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MolarOpticalRotatoryPowerUnit[1];
    }

    attribute molarOpticalRotatoryPower: MolarOpticalRotatoryPowerValue[*] nonunique :> scalarQuantities;

    attribute def MolarOpticalRotatoryPowerUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute amountOfSubstancePF: QuantityPowerFactor[1] { :>> quantity = isq.N; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF); }
    }

    /* ISO-80000-9 item 9-49 specific optical rotatory power */
    attribute def SpecificOpticalRotatoryPowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-49 specific optical rotatory power
         * symbol(s): `α_m`
         * application domain: generic
         * name: SpecificOpticalRotatoryPower
         * quantity dimension: L^2*M^-1
         * measurement unit(s): rad*m^2/kg, kg^-1*m^2
         * tensor order: 0
         * definition: angle `α` of optical rotation (item 9-47), multiplied by the quotient of cross-sectional area `A` (ISO 80000-3) of a linearly polarized light beam and the mass `m` (ISO 80000-4) of the optically active component in the path of the beam: `α_m = (α A)/m`
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificOpticalRotatoryPowerUnit[1];
    }

    attribute specificOpticalRotatoryPower: SpecificOpticalRotatoryPowerValue[*] nonunique :> scalarQuantities;

    attribute def SpecificOpticalRotatoryPowerUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_chemistry_molecular.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package ISQChemistryMolecular {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-9:2019 "Physical chemistry and molecular physics"
     * see also https://www.iso.org/standard/64979.html
     * 
     * Note 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,
     * with Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.
     * Note 2: For vector and tensor quantities currently the unit and quantity value type for their (scalar) magnitude is 
     * defined, as well as their typical Cartesian 3d VectorMeasurementReference (i.e. coordinate system) 
     * or TensorMeasurementReference.
     */
    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import ISQBase::*;
    private import ISQSpaceTime::AngularMeasureValue;
    private import ISQThermodynamics::EnergyValue;
    attribute def numberOfEntities : CountValue {
        doc
        /*
         * source: item 9-1 number of entities
         * symbol(s): `N(X)`, `N_X`
         * application domain: generic
         * name: NumberOfEntities (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of elementary entities of kind `X` in a system
         * remarks: The elementary entities must be specified and can be atoms, molecules, ions, electrons, other particle, or a specified group of such particles. It is important to always give a precise specification of the entity involved; this should preferably be done by the empirical chemical formula of the material involved.
         */
    }
    alias NumberOfMolesUnit for AmountOfSubstanceUnit;
    alias NumberOfMolesValue for AmountOfSubstanceValue;
    alias numberOfMoles for amountOfSubstance;
    attribute def RelativeAtomicMassValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-3 relative atomic mass
         * symbol(s): `A_r(X)`
         * application domain: generic
         * name: RelativeAtomicMass (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the average mass (ISO 80000-4) of atom `X` and the unified atomic mass (ISO 80000-10)
         * remarks: A similar quantity "relative molecular mass" can be defined for molecules. EXAMPLE `A_r(Cl) ~~ 35.453` `A_r(CO_2) ~~ 44` The relative atomic or relative molecular mass depends on the nuclidic composition. The International Union of Pure and Applied Chemistry (IUPAC) accepts the use of the special names "atomic weight" and "molecular weight" for the quantities "relative atomic mass" and "relative molecular mass", respectively. The use of these traditional names is deprecated.
         */
    }
    attribute def relativeAtomicMass : RelativeAtomicMassValue;
    attribute def MolarMassValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-4 molar mass
         * symbol(s): `M(X)`
         * application domain: generic
         * name: MolarMass
         * quantity dimension: M^1*N^-1
         * measurement unit(s): g/mol, kg*mol^-1
         * tensor order: 0
         * definition: for a pure substance `X`, quotient of mass `m(X)` (ISO 80000-4) and amount `n` of substance (item 9-2): `M = m/n`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarMassUnit[1];
    }
    attribute def molarMass : MolarMassValue[*] nonunique;
    attribute def MolarMassUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, amountOfSubstancePF);
        }
    }
    attribute def MolarVolumeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-5 molar volume
         * symbol(s): `V_m`
         * application domain: generic
         * name: MolarVolume
         * quantity dimension: L^3*N^-1
         * measurement unit(s): m^3*mol^-1
         * tensor order: 0
         * definition: for a pure substance, quotient of its volume `V` (ISO 80000-3) and amount `n` of substance (item 9-2): `V_m = V/n`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarVolumeUnit[1];
    }
    attribute def molarVolume : MolarVolumeValue[*] nonunique;
    attribute def MolarVolumeUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 3;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF);
        }
    }
    attribute def MolarInternalEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-6.1 molar internal energy
         * symbol(s): `U_m`
         * application domain: generic
         * name: MolarInternalEnergy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of internal energy `U` (ISO 80000-5) and amount `n` of substance (item 9-2): `U_m = U/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarInternalEnergyUnit[1];
    }
    attribute def molarInternalEnergy : MolarInternalEnergyValue[*] nonunique;
    attribute def MolarInternalEnergyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF);
        }
    }
    attribute def MolarEnthalpyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-6.2 molar enthalpy
         * symbol(s): `H_m`
         * application domain: generic
         * name: MolarEnthalpy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of enthalpy `H` (ISO 80000-5) and amount `n` of substance (item 9-2): `H_m = H/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarEnthalpyUnit[1];
    }
    attribute def molarEnthalpy : MolarEnthalpyValue[*] nonunique;
    attribute def MolarEnthalpyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF);
        }
    }
    attribute def MolarHelmholtzEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-6.3 molar Helmholtz energy
         * symbol(s): `F_m`
         * application domain: generic
         * name: MolarHelmholtzEnergy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of the Helmholtz energy `F` (ISO 80000-5) and amount `n` of substance (item 9-2): `F_m = F/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarHelmholtzEnergyUnit[1];
    }
    attribute def molarHelmholtzEnergy : MolarHelmholtzEnergyValue[*] nonunique;
    attribute def MolarHelmholtzEnergyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF);
        }
    }
    attribute def MolarGibbsEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-6.4 molar Gibbs energy
         * symbol(s): `G_m`
         * application domain: generic
         * name: MolarGibbsEnergy
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: quotient of the Gibbs energy `G` (ISO 80000-5) and amount `n` of substance (item 9-2): `G_m = G/n`
         * remarks: Molar quantities are normally only used with reference to pure substances.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarGibbsEnergyUnit[1];
    }
    attribute def molarGibbsEnergy : MolarGibbsEnergyValue[*] nonunique;
    attribute def MolarGibbsEnergyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF);
        }
    }
    attribute def MolarHeatCapacityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-7 molar heat capacity
         * symbol(s): `C_m`
         * application domain: generic
         * name: MolarHeatCapacity
         * quantity dimension: L^2*M^1*T^-2*Θ^-1*N^-1
         * measurement unit(s): J/(mol*K), kg*m^2*s^-2*K^-1*mol^-1
         * tensor order: 0
         * definition: quotient of heat capacity `C` (ISO 80000-5) and amount of substance `n` (item 9-2): `C_m = C/n`
         * remarks: Conditions (constant pressure or volume etc.) must be specified.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarHeatCapacityUnit[1];
    }
    attribute def molarHeatCapacity : MolarHeatCapacityValue[*] nonunique;
    attribute def MolarHeatCapacityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF, amountOfSubstancePF);
        }
    }
    attribute def MolarEntropyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-8 molar entropy
         * symbol(s): `S_m`
         * application domain: generic
         * name: MolarEntropy
         * quantity dimension: L^2*M^1*T^-2*Θ^-1*N^-1
         * measurement unit(s): J/(mol*K), kg*m^2*s^-2*K^-1*mol^-1
         * tensor order: 0
         * definition: quotient of entropy `S` (ISO 80000-5) and amount `n` of substance (item 9-2): `S_m = S/n`
         * remarks: Conditions (constant pressure or volume etc.) must be specified.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarEntropyUnit[1];
    }
    attribute def molarEntropy : MolarEntropyValue[*] nonunique;
    attribute def MolarEntropyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF, amountOfSubstancePF);
        }
    }
    attribute def ParticleConcentrationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-9.1 particle concentration
         * symbol(s): `n`, `(C)`
         * application domain: generic
         * name: ParticleConcentration
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: quotient of number `N` of particles (item 9-1) and volume `V `(ISO 80000-3): `n = N/V`
         * remarks: The term "number density" is also used.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ParticleConcentrationUnit[1];
    }
    attribute def particleConcentration : ParticleConcentrationValue[*] nonunique;
    attribute def ParticleConcentrationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    attribute def molecularConcentration : ParticleConcentrationValue {
        doc
        /*
         * source: item 9-9.2 molecular concentration
         * symbol(s): `C(X)`, `C_X`
         * application domain: generic
         * name: MolecularConcentration (specializes ParticleConcentration)
         * quantity dimension: L^-3
         * measurement unit(s): m^-3
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of number `N_X` of molecules of substance `X` and volume `V` (ISO 80000-3) of the mixture: `C_X = N_X/V`
         * remarks: None.
         */
    }
    attribute def MassConcentrationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-10 mass concentration
         * symbol(s): `γ_X`, `(ρ_X)`
         * application domain: generic
         * name: MassConcentration
         * quantity dimension: L^-3*M^1
         * measurement unit(s): g/l, kg*m^-3
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of mass `m_X` (ISO 80000-4) of substance `X` and volume `V` (ISO 80000-3) of the mixture: `γ_X = m_X/V`
         * remarks: Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassConcentrationUnit[1];
    }
    attribute def massConcentration : MassConcentrationValue[*] nonunique;
    attribute def MassConcentrationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }
    attribute def MassFractionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-11 mass fraction
         * symbol(s): `w_X`
         * application domain: generic
         * name: MassFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of mass `m_X` (ISO 80000-4) of substance `X` and total mass `m` of the mixture: `w_X = m_X/m`
         * remarks: None.
         */
    }
    attribute def massFraction : MassFractionValue;
    attribute def AmountOfSubstanceConcentrationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-12.1 amount-of-substance concentration
         * symbol(s): `c_X`
         * application domain: generic
         * name: AmountOfSubstanceConcentration
         * quantity dimension: L^-3*N^1
         * measurement unit(s): mol/l, mol*m^-3
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of amount `n_X` of substance (item 9-2) of `X` and volume `V` (ISO 80000-3) of the mixture: `c_X = n_X/V`
         * remarks: In chemistry, the name "amount-of-substance concentration" is generally abbreviated to the single word "concentration", it being assumed that the adjective "amount-of-substance" is intended. For this reason, however, the word "mass" should never be omitted from the name "mass concentration" in item 9-10. Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AmountOfSubstanceConcentrationUnit[1];
    }
    attribute def amountOfSubstanceConcentration : AmountOfSubstanceConcentrationValue[*] nonunique;
    attribute def AmountOfSubstanceConcentrationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF);
        }
    }
    attribute def standardAmountOfSubstanceConcentration : AmountOfSubstanceConcentrationValue {
        doc
        /*
         * source: item 9-12.2 standard amount-of-substance concentration
         * symbol(s): `c^!(X)`
         * application domain: generic
         * name: StandardAmountOfSubstanceConcentration (specializes AmountOfSubstanceConcentration)
         * quantity dimension: L^-3*N^1
         * measurement unit(s): mol/l, mol*m^-3
         * tensor order: 0
         * definition: for substance `X`, one mole per litre
         * remarks: Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
    }
    attribute def AmountOfSubstanceFractionMoleFractionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-13 amount-of-substance fraction mole fraction
         * symbol(s): `x_X`, `y_X`
         * application domain: generic
         * name: AmountOfSubstanceFractionMoleFraction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a mixture, quotient of amount of substance `n_X` (item 9-2) of `X` and total amount `n` of substance (item 9-2) in the mixture: `x_X = n_X/n`
         * remarks: For condensed phases, `x_X` is used, and for gaseous mixtures `y_X` may be used. The unsystematic name "mole fraction" is still used. However, the use of this name is deprecated. For this quantity, the entity used to define the amount of substance should always be a single molecule for every species in the mixture.
         */
    }
    attribute def amountOfSubstanceFractionMoleFraction : AmountOfSubstanceFractionMoleFractionValue;
    attribute def VolumeFractionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-14 volume fraction
         * symbol(s): `φ_X`
         * application domain: generic
         * name: VolumeFraction
         * quantity dimension: 1
         * measurement unit(s): ml/l, 1
         * tensor order: 0
         * definition: for substance `X`, quotient of product of amount of substance fraction `x_X` (item 9-13) of `X` and molar volume `V_(m,X)` (item 9-5) of the pure substance `X` at the same temperature (ISO 80000-5) and pressure (ISO 80000-4), and sum over all substances `i` of products of amount-of-substance fractions `x_i` (item 9-13) of substance `i` and their molar volumes `V_(m,i)` (item 9-5): `φ_X = (x_X V_(m,X))/(sum_i x_i V_(m,i))`
         * remarks: Generally, the volume fraction is temperature dependent. Decided by the 16th CGPM (1979), both "l" and "L" are allowed for the symbols for the litre.
         */
        attribute :>> num : Real;
        attribute :>> mRef : VolumeFractionUnit[1];
    }
    attribute def volumeFraction : VolumeFractionValue[*] nonunique;
    attribute def VolumeFractionUnit :> DimensionOneUnit {
    }
    attribute def MolalityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-15 molality
         * symbol(s): `b_B`, `m_B`
         * application domain: generic
         * name: Molality
         * quantity dimension: M^-1*N^1
         * measurement unit(s): mol/kg
         * tensor order: 0
         * definition: quotient of amount of substance (item 9-2) of solute `B` and mass `m_A` (ISO 80000-4) of the solvent substance `A`: `b_B = n_B/m_A`
         * remarks: The alternative symbol `m_B` should be avoided in situations where it might be mistaken for the mass of substance B. However, the symbol `m_B` is much more commonly used than the symbol `b_B` for molality, despite the possible confusion with mass.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolalityUnit[1];
    }
    attribute def molality : MolalityValue[*] nonunique;
    attribute def MolalityUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, amountOfSubstancePF);
        }
    }
    attribute def latentHeatOfPhaseTransition : EnergyValue {
        doc
        /*
         * source: item 9-16 latent heat of phase transition, enthalpy of phase transition
         * symbol(s): `C_"pt"`
         * application domain: generic
         * name: LatentHeatOfPhaseTransition (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy (ISO 80000-5) necessary to be added or subtracted isothermally and isobarically to a system to completely undergo the phase transition
         * remarks: Mostly, molar or specific quantity is used and phase transition is expressed explicitly, e.g. molar latent heat of evaporation. The subscript "pt" is the qualifier for the phase transition, which may be changed to e.g. "l-g". The term "enthalpy of phase transition" is mainly used in theory.
         */
    }
    alias enthalpyOfPhaseTransition for latentHeatOfPhaseTransition;
    attribute def ChemicalPotentialValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-17 chemical potential
         * symbol(s): `μ_X`
         * application domain: chemistry
         * name: ChemicalPotential
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: partial derivative of the Gibbs energy (ISO 80000-5) with respect to amount `n_X` of substance `X` (item 9-2) at constant temperature `T` (ISO 80000-5) and pressure `p `(ISO 80000-4): `μ_X = ((del G)/(del n_X))_(T,p)`
         * remarks: For a pure substance, where `G_m` is the molar Gibbs energy. In a mixture, `μ_B` is the partial molar Gibbs energy. In condensed matter physics, the chemical potential of electrons is energy.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ChemicalPotentialUnit[1];
    }
    attribute def chemicalPotential : ChemicalPotentialValue[*] nonunique;
    attribute def ChemicalPotentialUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF);
        }
    }
    attribute def AbsoluteActivityValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-18 absolute activity
         * symbol(s): `λ_X`
         * application domain: generic
         * name: AbsoluteActivity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X`, exponential of quotient of chemical potential `μ_X` of substance `B` (item 9-17), and product of molar gas constant `R` (item 9-37.1) and thermodynamic temperature `T` (ISO 80000-5): `λ_X = exp(μ_X/(RT))`
         * remarks: None.
         */
    }
    attribute def absoluteActivity : AbsoluteActivityValue;
    attribute def PartialPressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-19 partial pressure
         * symbol(s): `p_X`
         * application domain: generic
         * name: PartialPressure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: for substance `X` in a gaseous mixture, product of amount-of-substance fraction `y_X` of substance X (item 9-13) and total pressure `p` (ISO 80000-4): `p_X = y_X p`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PartialPressureUnit[1];
    }
    attribute def partialPressure : PartialPressureValue[*] nonunique;
    attribute def PartialPressureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def FugacityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-20 fugacity
         * symbol(s): `tilde(p)_X`
         * application domain: generic
         * name: Fugacity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: for substance `X`, quantity proportional to the absolute activity, `λ_X` (item 9-18), the proportionality factor, which is a function of temperature (ISO 80000-5) only, being determined by the condition that, at constant temperature and composition, `p_X/tilde(p)_X` tends to 1 for an indefinitely dilute gas
         * remarks: `tilde(p)_X = λ_X * lim_(p->0) (p_X/λ_X)` where `p` is total pressure (ISO 80000-4). The IUPAC preferred symbol for fugacity is `f`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : FugacityUnit[1];
    }
    attribute def fugacity : FugacityValue[*] nonunique;
    attribute def FugacityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def StandardChemicalPotentialValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-21 standard chemical potential
         * symbol(s): `μ_B^!`, `μ^!`
         * application domain: generic
         * name: StandardChemicalPotential
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: for substance `B`, value of the chemical potential (item 9-17) at specified standard conditions
         * remarks: `μ_B^! = RT ln(λ^!)` where `μ_B^!` is a function of temperature `T` at the standard pressure `p = p^!` The standard chemical potential depends on the choice of standard state, which must be specified. In a liquid or solid solution, the standard state is referenced to the ideal dilute behaviour of the solute (substance `B`).
         */
        attribute :>> num : Real;
        attribute :>> mRef : StandardChemicalPotentialUnit[1];
    }
    attribute def standardChemicalPotential : StandardChemicalPotentialValue[*] nonunique;
    attribute def StandardChemicalPotentialUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF);
        }
    }
    attribute def ActivityFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-22 activity factor
         * symbol(s): `f_X`
         * application domain: generic
         * name: ActivityFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a liquid or a solid mixture, quotient of absolute activity `λ_X` (item 9-18) of substance `X` and the product of absolute activity `λ_X^"*"` of the pure substance `X` at the same temperature (ISO 80000-5) and pressure (ISO 80000-4) and amount-of-substance fraction `x_X` of substance `X` (item 9-13): `f_X = λ_X/(λ_X^"*" x_X)`
         * remarks: The systematic name is "activity factor", but the name "activity coefficient" is also commonly used (see item 9-25). Activity factors can also be obtained applying Raoult’s law or Henry’s law.
         */
    }
    attribute def activityFactor : ActivityFactorValue;
    attribute def StandardAbsoluteActivityInMixtureValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-23 standard absolute activity in mixture
         * symbol(s): `λ_X^!`
         * application domain: in a mixture
         * name: StandardAbsoluteActivityInMixture (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `X` in a liquid or a solid mixture, absolute activity `λ_X^"*"` (item 9-18) of the pure substance `X` at the same temperature (ISO 80000-5) but at standard pressure (ISO 80000-4) `10^5 ["Pa"]`: `λ_X^! = λ_X"*" (p^!)`
         * remarks: This quantity is a function of temperature only.
         */
    }
    attribute def standardAbsoluteActivityInMixture : StandardAbsoluteActivityInMixtureValue;
    attribute def ActivityOfSoluteValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-24 activity of solute, relative activity of solute
         * symbol(s): `a_X`, `a_(m,X)`
         * application domain: generic
         * name: ActivityOfSolute (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a solute `X` in a solution, quantity proportional to the absolute activity, `λ_X` (item 9-18), the proportionality factor, which is a function of temperature (ISO 80000-5) and pressure (ISO 80000-4) only, being determined by the condition that, at constant temperature and pressure, `a_X` divided by the molality (item 9-15) ratio, `b_X/b^!` tends to 1 at infinite dilution; `b_X` is the molality of solute `X` (item 9-15), and `b^!` is standard molality: `a_X = λ_X * lim_(sum b_X -> 0) (b_X//b^!)/λ_X`
         * remarks: The quantity `a_(c,X)` , similarly defined in terms of the concentration ratio `c_X/c^!` , is also called the activity or relative activity of solute `X`; `c^!` is a standard amount-of-substance concentration (item 9-12.2): `a_(c,X) = λ_X * lim_(sum c_X -> 0) (c_X//c^!)/λ_X`, where `sum` denotes summation over all the solute substances. This especially applies to a dilute liquid solution.
         */
    }
    attribute def activityOfSolute : ActivityOfSoluteValue;
    alias relativeActivityOfSolute for activityOfSolute;
    attribute def ActivityCoefficientValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-25 activity coefficient
         * symbol(s): `γ_B`
         * application domain: generic
         * name: ActivityCoefficient (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a solute `B` in a solution, quotient of activity `a_B` of solute `B` (item 9-24), and quotient of the molality (item 9-15) `b_B` of substance `B` and standard molality `b^!`: `γ_B = a_B/(b_B//b^!)`
         * remarks: The name "activity coefficient of solute B" is also used for the quantity `γ_B` defined as: `γ_B = a_(c,B)/(c_B//c^!)` See item 9-22.
         */
    }
    attribute def activityCoefficient : ActivityCoefficientValue;
    attribute def StandardAbsoluteActivityInSolutionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-26 standard absolute activity in solution
         * symbol(s): `λ_B^!`
         * application domain: in a solution
         * name: StandardAbsoluteActivityInSolution (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a solute `B` in a solution: `λ_B^! = lim_(sum b_B -> 0) [λ_B ((p^!)b^!)/b_B]` where ∑ denotes summation over all solutes, `p^!` is a standard pressure (ISO 80000-4), `b^!` is standard molality, and `b_B` is the molality of substance `B` (item 9-15)
         * remarks: This quantity is a function of temperature only. It especially applies to a dilute liquid solution. The standard pressure is `10^5 ["Pa"]`.
         */
    }
    attribute def standardAbsoluteActivityInSolution : StandardAbsoluteActivityInSolutionValue;
    attribute def ActivityOfSolventValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-27.1 activity of solvent, relative activity of solvent
         * symbol(s): `a_A`
         * application domain: generic
         * name: ActivityOfSolvent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for the solvent `A` in a solution, quotient of the absolute activity of substance `A`, `λ_A` (item 9-18), and that, `λ_A^"*"` , of the pure solvent at the same temperature (ISO 80000-5) and pressure (ISO 80000-4): `a_A = λ_A/λ_A^"*"`
         * remarks: None.
         */
    }
    attribute def activityOfSolvent : ActivityOfSolventValue;
    alias relativeActivityOfSolvent for activityOfSolvent;
    attribute def OsmoticFactorOfSolventValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-27.2 osmotic factor of solvent, osmotic coefficient of solvent A
         * symbol(s): `φ`
         * application domain: generic
         * name: OsmoticFactorOfSolvent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `φ = -(M_A sum b_B)^-1 ln(a_A)` where `M_A` is the molar mass (item 9-4) of the solvent A, ∑ denotes summation over all the solutes, `b_B` is the molality of solute B (item 9-15), and `a_A` is the activity of solvent A (item 9-27.1)
         * remarks: The name "osmotic coefficient" is generally used, although the name "osmotic factor" is more systematic. This concept especially applies to a dilute liquid solution.
         */
    }
    attribute def osmoticFactorOfSolvent : OsmoticFactorOfSolventValue;
    alias osmoticCoefficientOfSolventA for osmoticFactorOfSolvent;
    attribute def StandardAbsoluteActivityOfSolventValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-27.3 standard absolute activity of solvent
         * symbol(s): `λ_A^!`
         * application domain: in a dilute solution
         * name: StandardAbsoluteActivityOfSolvent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for solvent `A`, standard absolute activity (item 9-23) of the pure substance `A` at the same temperature (ISO 80000-5) and at a standard pressure `p^!` (ISO 80000-4): `λ_A^! = λ_A^"*" p^!`
         * remarks: None.
         */
    }
    attribute def standardAbsoluteActivityOfSolvent : StandardAbsoluteActivityOfSolventValue;
    attribute def OsmoticPressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-28 osmotic pressure
         * symbol(s): `Π`
         * application domain: generic
         * name: OsmoticPressure
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: excess pressure (ISO 80000-4) required to maintain osmotic equilibrium between a solution and the pure solvent separated by a membrane permeable to the solvent only
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : OsmoticPressureUnit[1];
    }
    attribute def osmoticPressure : OsmoticPressureValue[*] nonunique;
    attribute def OsmoticPressureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def StoichiometricNumberOfSubstanceValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-29 stoichiometric number of substance
         * symbol(s): `ν_B`
         * application domain: generic
         * name: StoichiometricNumberOfSubstance (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for substance `B`, an integer number or a simple fraction, being negative for a reactant and positive for a product, occurring in the expression for a chemical reaction: `0 = sum ν_B` where the symbol `B` denotes the reactants and products involved in the reaction
         * remarks: EXAMPLE `(1/2)"N"_2 + (3/2)"H"_2 = "N""H"_3` ; `ν("N"_2) = -1/2`, `ν("H"_2) = -3/2`, `ν("N""H"_3) = +1`.
         */
    }
    attribute def stoichiometricNumberOfSubstance : StoichiometricNumberOfSubstanceValue;
    attribute def AffinityOfAChemicalReactionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-30 affinity of a chemical reaction
         * symbol(s): `A`
         * application domain: generic
         * name: AffinityOfAChemicalReaction
         * quantity dimension: L^2*M^1*T^-2*N^-1
         * measurement unit(s): J/mol, kg*m^2*s^-2*mol^-1
         * tensor order: 0
         * definition: negative of the sum over all substances `B` of products of stoichiometric number `ν_B` of substance `B` (item 9-29) and chemical potential `μ_B` of substance `B` (item 9-17): `A = -sum ν_B μ_B`
         * remarks: The affinity of a reaction is a measure of the "driving force" of the reaction. When it is positive, the reaction goes spontaneously from reactants to products, and when it is negative, the reaction goes in the opposite direction. Another way to write the definition is: `A = ((del G)/(del ξ))_(p,T)` where `G` is Gibbs energy (ISO 80000-5) and `ξ` is the extent of the reaction (item 9-31). Note that `ν_B` is negative for reactants and positive for products.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AffinityOfAChemicalReactionUnit[1];
    }
    attribute def affinityOfAChemicalReaction : AffinityOfAChemicalReactionValue[*] nonunique;
    attribute def AffinityOfAChemicalReactionUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, amountOfSubstancePF);
        }
    }
    attribute def extentOfReaction : AmountOfSubstanceValue {
        doc
        /*
         * source: item 9-31 extent of reaction
         * symbol(s): `ξ`
         * application domain: generic
         * name: ExtentOfReaction (specializes AmountOfSubstance)
         * quantity dimension: N^1
         * measurement unit(s): mol
         * tensor order: 0
         * definition: difference of initial amount `n_(B "in")` of substance `B` (item 9-2) and equilibrium amount `n_(B "eq")` of substance `B` (item 9-2) divided by stoichiometric number `ν_B` of substance `B` (item 9-29): `ξ = (n_(B "eq") - n_(B "in"))/ν_B`
         * remarks: See remark to item 9-30.
         */
    }
    attribute def StandardEquilibriumConstantValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-32 standard equilibrium constant, thermodynamic equilibrium constant
         * symbol(s): `K^!`
         * application domain: generic
         * name: StandardEquilibriumConstant (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for a chemical reaction, product for all substances `B` of standard absolute activity `λ_B^!` of substance `B` (item 9-26) in power of minus stoichiometric number `ν_B` of substance `B` (item 9-29): `K^! = prod_B (tilde(a) λ_B^!)^(-ν_B)`
         * remarks: This quantity is a function of temperature only. Others depend on temperature, pressure, and composition. One can define in an analogous way an equilibrium constant in terms of fugacity, `K_f`, molality, `K_m`, etc.
         */
    }
    attribute def standardEquilibriumConstant : StandardEquilibriumConstantValue;
    alias thermodynamicEquilibriumConstant for standardEquilibriumConstant;
    attribute def EquilibriumConstantOnPressureBasisValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-33 equilibrium constant on pressure basis
         * symbol(s): `K_p`
         * application domain: pressure basis
         * name: EquilibriumConstantOnPressureBasis
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: for gases, product for all substances `B` of partial pressure `p_B` of substance `B` (item 9-19) in power of its stoichiometric number `ν_B` (item 9-29): `K_p = prod_B (p_B)^(ν_B)`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : EquilibriumConstantOnPressureBasisUnit[1];
    }
    attribute def equilibriumConstantOnPressureBasis : EquilibriumConstantOnPressureBasisValue[*] nonunique;
    attribute def EquilibriumConstantOnPressureBasisUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def EquilibriumConstantOnConcentrationBasisValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-34 equilibrium constant on concentration basis
         * symbol(s): `K_c`
         * application domain: concentration basis
         * name: EquilibriumConstantOnConcentrationBasis
         * quantity dimension: L^-3*N^1
         * measurement unit(s): mol/m^3
         * tensor order: 0
         * definition: for solutions, product for all substances `B` of concentration `c_B` of substance `B` (item 9-9.1) in power of its stoichiometric number `ν_B` (item 9-29): `K_c = prod_B (c_B)^(ν_B)`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : EquilibriumConstantOnConcentrationBasisUnit[1];
    }
    attribute def equilibriumConstantOnConcentrationBasis : EquilibriumConstantOnConcentrationBasisValue[*] nonunique;
    attribute def EquilibriumConstantOnConcentrationBasisUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF);
        }
    }
    attribute def microcanonicalPartitionFunction : CountValue {
        doc
        /*
         * source: item 9-35.1 microcanonical partition function
         * symbol(s): `Ω`
         * application domain: generic
         * name: MicrocanonicalPartitionFunction (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of all quantum states `r` consistent with given energy `E` (ISO 80000-4), volume (ISO 80000-3), and external fields: `Ω = sum_r 1`
         * remarks: `S = k ln(Ω)` where `S` is entropy (ISO 80000-5) and `k` is the Boltzmann constant (ISO 80000-1).
         */
    }
    attribute def CanonicalPartitionFunctionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-35.2 canonical partition function
         * symbol(s): `Z`
         * application domain: generic
         * name: CanonicalPartitionFunction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: sum over quantum states of energy `E_r` (ISO 80000-4), expressed by: `Z = sum_r exp(-E_r/(kT))` where `k` is the Boltzmann constant (ISO 80000-1), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: `A = -kT ln(Z)` where `A` is Helmholtz energy (ISO 80000-5).
         */
    }
    attribute def canonicalPartitionFunction : CanonicalPartitionFunctionValue;
    attribute def GrandCanonicalPartitionFunctionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-35.3 grand-canonical partition function, grand partition function
         * symbol(s): `Ξ`
         * application domain: generic
         * name: GrandCanonicalPartitionFunction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: sum of canonical partition function `Z(N_A,N_B,…)` for the given number of particles `A,B` multiplied by absolute activities (item 9-18) `λ_A, λ_B, ...` of particles `A, B`: `Ξ = sum_(N_A, N_B, ...) Z(N_A, N_B, …) * λ_A^(N_A) * λ_B^(N_B) * ...`
         * remarks: `A - sum μ_B n_B = -kT ln(Ξ)` where `A` is Helmholtz energy (ISO 80000-5), `μ_B` is the chemical potential of substance `B`, and `n_B` is the amount of substance `B`.
         */
    }
    attribute def grandCanonicalPartitionFunction : GrandCanonicalPartitionFunctionValue;
    alias grandPartitionFunction for grandCanonicalPartitionFunction;
    attribute def MolecularPartitionFunctionValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-35.4 molecular partition function, partition function of a molecule
         * symbol(s): `q`
         * application domain: generic
         * name: MolecularPartitionFunction (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `q = sum_r exp(-ε_r/(kT))` where `ε_r` is the energy (ISO 80000-5) of the `r`-th level of the molecule consistent with given volume (ISO 80000-3) and external fields, `k` is the Boltzmann constant (ISO 80000-1), and `T` is thermodynamic temperature (ISO 80000-5)
         * remarks: None.
         */
    }
    attribute def molecularPartitionFunction : MolecularPartitionFunctionValue;
    alias partitionFunctionOfAMolecule for molecularPartitionFunction;
    attribute def statisticalWeightOfSubsystem : CountValue {
        doc
        /*
         * source: item 9-36.1 statistical weight of subsystem
         * symbol(s): `g`
         * application domain: generic
         * name: StatisticalWeightOfSubsystem (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of different microstates in a subsystem
         * remarks: None.
         */
    }
    attribute def DegeneracyValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-36.2 degeneracy, multiplicity
         * symbol(s): `g`
         * application domain: generic
         * name: Degeneracy (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for quantum level, statistical weight of that level
         * remarks: If `g = 1`, the level is called non-degenerate.
         */
    }
    attribute def degeneracy : DegeneracyValue;
    alias multiplicity for degeneracy;
    attribute def MolarGasConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-37.1 molar gas constant
         * symbol(s): `R`
         * application domain: generic
         * name: MolarGasConstant
         * quantity dimension: L^2*M^1*T^-2*Θ^-1*N^-1
         * measurement unit(s): J/(mol*K), kg*m^2*s^-2*K^-1*mol^-1
         * tensor order: 0
         * definition: product of the Boltzmann constant (ISO 80000-1) and the Avogadro constant (ISO 80000-1)
         * remarks: For an ideal gas, `pV_m = RT`
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarGasConstantUnit[1];
    }
    attribute def molarGasConstant : MolarGasConstantValue[*] nonunique;
    attribute def MolarGasConstantUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF, amountOfSubstancePF);
        }
    }
    attribute def meanFreePath : LengthValue {
        doc
        /*
         * source: item 9-38 mean free path
         * symbol(s): `l`, `λ`
         * application domain: chemistry
         * name: MeanFreePath (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: for a particle, the average distance `d` (ISO 80000-3) between two successive collisions with other particles
         * remarks: None.
         */
    }
    attribute def DiffusionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-39 diffusion coefficient
         * symbol(s): `D`
         * application domain: chemistry
         * name: DiffusionCoefficient
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: proportionality coefficient of local molecular concentration `C_B` (item 9-9.2) of substance `B` in the mixture multiplied by the local average velocity (ISO 80000-3) `v_B` of the molecules of `B`, and minus the gradient of the local molecular concentration `C_B` (item 9-9.2) of substance `B` in the mixture, expressed by: `C_B(v_B) = -D grad C_B`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DiffusionCoefficientUnit[1];
    }
    attribute def diffusionCoefficient : DiffusionCoefficientValue[*] nonunique;
    attribute def DiffusionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    attribute def ThermalDiffusionRatioValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-40.1 thermal diffusion ratio
         * symbol(s): `k_T`
         * application domain: generic
         * name: ThermalDiffusionRatio (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in a steady-state of a binary mixture in which thermal diffusion occurs, proportionality factor between gradient of the amount-of-subsstance fraction `x_B` (item 9-13) of the heavier substance `B`, and negative gradient of the local thermodynamic temperature `T` (ISO 80000-5) divided by that temperature (ISO 80000-5): `grad x_B = -(k_T/T) grad T`
         * remarks: None.
         */
    }
    attribute def thermalDiffusionRatio : ThermalDiffusionRatioValue;
    attribute def ThermalDiffusionFactorValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-40.2 thermal diffusion factor
         * symbol(s): `α_T`
         * application domain: generic
         * name: ThermalDiffusionFactor (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of the thermal diffusion ratio `k_T` (item 9-40.1), and the product of the local amount-of-substance fractions `x_A`, `x_B` (item 9-13) of two substances `A` and `B`: `α_T = k_T//(x_A x_B)`
         * remarks: None.
         */
    }
    attribute def thermalDiffusionFactor : ThermalDiffusionFactorValue;
    attribute def ThermalDiffusionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-41 thermal diffusion coefficient
         * symbol(s): `D_T`
         * application domain: generic
         * name: ThermalDiffusionCoefficient
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: product of the thermal diffusion ratio `k_T` (item 9-40.1) and the diffusion coefficient `D` (item 9-39): `D_T = k_T*D`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThermalDiffusionCoefficientUnit[1];
    }
    attribute def thermalDiffusionCoefficient : ThermalDiffusionCoefficientValue[*] nonunique;
    attribute def ThermalDiffusionCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    attribute def IonicStrengthValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-42 ionic strength
         * symbol(s): `I`
         * application domain: generic
         * name: IonicStrength
         * quantity dimension: M^-1*N^1
         * measurement unit(s): mol*kg^-1
         * tensor order: 0
         * definition: in a sample, one half of the sum of square of the charge number `z_i` (ISO 80000-10) of `i`-th ion multiplied by its molality `b_i` (item 9-15) over any involved ion: `I = 1/2 sum z_i^2 b_i`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : IonicStrengthUnit[1];
    }
    attribute def ionicStrength : IonicStrengthValue[*] nonunique;
    attribute def IonicStrengthUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, amountOfSubstancePF);
        }
    }
    attribute def DegreeOfDissociationValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-43 degree of dissociation, dissociation fraction
         * symbol(s): `α`
         * application domain: generic
         * name: DegreeOfDissociation (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: in a sample, quotient of the number `n_d` of dissociated molecules and the total number `n` of molecules: `α = n_D / n`
         * remarks: None.
         */
    }
    attribute def degreeOfDissociation : DegreeOfDissociationValue;
    alias dissociationFraction for degreeOfDissociation;
    attribute def ElectrolyticConductivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-44 electrolytic conductivity
         * symbol(s): `κ`
         * application domain: generic
         * name: ElectrolyticConductivity
         * quantity dimension: L^-3*M^-1*T^3*I^2
         * measurement unit(s): S/m, kg^-1*m^-3*s^3*A^2
         * tensor order: 0
         * definition: quotient of the magnitude of electric current density `J` (IEC 80000-6) and the magnitude electric field strength `E` (IEC 80000-6) in an electrolyte: `κ = J/E`
         * remarks: For anisotropic media, `κ` is a tensor. In IEC 80000-6 the symbols `σ`, `γ` are used.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ElectrolyticConductivityUnit[1];
    }
    attribute def electrolyticConductivity : ElectrolyticConductivityValue[*] nonunique;
    attribute def ElectrolyticConductivityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -3;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 3;
        }
        private attribute electricCurrentPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.I;
            attribute :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, electricCurrentPF);
        }
    }
    attribute def MolarConductivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-45 molar conductivity
         * symbol(s): `Λ_m`
         * application domain: generic
         * name: MolarConductivity
         * quantity dimension: M^-1*T^3*I^2*N^-1
         * measurement unit(s): S*m^2/mol, kg^-1*s^3*A^2*mol^-1
         * tensor order: 0
         * definition: in an electrolyte, quotient of electrolytic conductivity `κ` (item 9-44) and amount-of-substance concentration `c_B` (item 9-12.1): `Λ_m = κ/c_B`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarConductivityUnit[1];
    }
    attribute def molarConductivity : MolarConductivityValue[*] nonunique;
    attribute def MolarConductivityUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 3;
        }
        private attribute electricCurrentPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.I;
            attribute :>> exponent = 2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF, electricCurrentPF, amountOfSubstancePF);
        }
    }
    attribute def TransportNumberOfTheIonBValue :> DimensionOneValue {
        doc
        /*
         * source: item 9-46 transport number of the ion B, current fraction of the ion B
         * symbol(s): `t_B`
         * application domain: generic
         * name: TransportNumberOfTheIonB (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: for the ion `B`, quotient of electric current `i_B` (IEC 80000-6) carried by the ion `B` and total electric current `i` (IEC 80000-6) in an electrolyte: `t_B = i_B/i`
         * remarks: None.
         */
    }
    attribute def transportNumberOfTheIonB : TransportNumberOfTheIonBValue;
    alias currentFractionOfTheIonB for transportNumberOfTheIonB;
    attribute def angleOfOpticalRotation : AngularMeasureValue {
        doc
        /*
         * source: item 9-47 angle of optical rotation
         * symbol(s): `α`
         * application domain: generic
         * name: AngleOfOpticalRotation (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): rad
         * tensor order: 0
         * definition: angle through which plane-polarized light is rotated clockwise, as seen when facing the light source, in passing through an optically active medium
         * remarks: None.
         */
    }
    attribute def MolarOpticalRotatoryPowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-48 molar optical rotatory power
         * symbol(s): `α_n`
         * application domain: generic
         * name: MolarOpticalRotatoryPower
         * quantity dimension: L^2*N^-1
         * measurement unit(s): rad*m^2/mol, m^2*mol^-1
         * tensor order: 0
         * definition: angle `α` of optical rotation (item 9-47), multiplied by the quotient of cross-sectional area `A` (ISO 80000-3) of a linearly polarized light beam and the amount of substance `n` (item 9-2) of the optically active component in the path of the beam: `α_n = (α A)/n`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MolarOpticalRotatoryPowerUnit[1];
    }
    attribute def molarOpticalRotatoryPower : MolarOpticalRotatoryPowerValue[*] nonunique;
    attribute def MolarOpticalRotatoryPowerUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute amountOfSubstancePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.N;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, amountOfSubstancePF);
        }
    }
    attribute def SpecificOpticalRotatoryPowerValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 9-49 specific optical rotatory power
         * symbol(s): `α_m`
         * application domain: generic
         * name: SpecificOpticalRotatoryPower
         * quantity dimension: L^2*M^-1
         * measurement unit(s): rad*m^2/kg, kg^-1*m^2
         * tensor order: 0
         * definition: angle `α` of optical rotation (item 9-47), multiplied by the quotient of cross-sectional area `A` (ISO 80000-3) of a linearly polarized light beam and the mass `m` (ISO 80000-4) of the optically active component in the path of the beam: `α_m = (α A)/m`
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificOpticalRotatoryPowerUnit[1];
    }
    attribute def specificOpticalRotatoryPower : SpecificOpticalRotatoryPowerValue[*] nonunique;
    attribute def SpecificOpticalRotatoryPowerUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF);
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 819) (line 15) (column 20) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 819) (line 15) (column 20) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 833) (line 15) (column 34) (len 4)))))
    (reference r1 (scope relative) (span (offset 858) (line 16) (column 20) (len 10)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 858) (line 16) (column 20) (len 10)))))
    (reference r2 (scope relative) (span (offset 892) (line 17) (column 20) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 892) (line 17) (column 20) (len 21)))))
    (reference r3 (scope relative) (span (offset 937) (line 18) (column 20) (len 7)) (segments (segment 0 (token "ISQBase") (name "ISQBase") (separator none) (span (offset 937) (line 18) (column 20) (len 7)))))
    (reference r4 (scope relative) (span (offset 1035) (line 21) (column 20) (len 33)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 1035) (line 21) (column 20) (len 12))) (segment 1 (token "AngularMeasureValue") (name "AngularMeasureValue") (separator colon-colon) (span (offset 1049) (line 21) (column 34) (len 19)))))
    (reference r5 (scope relative) (span (offset 1089) (line 22) (column 20) (len 30)) (segments (segment 0 (token "ISQThermodynamics") (name "ISQThermodynamics") (separator none) (span (offset 1089) (line 22) (column 20) (len 17))) (segment 1 (token "EnergyValue") (name "EnergyValue") (separator colon-colon) (span (offset 1108) (line 22) (column 39) (len 11)))))
    (reference r6 (scope relative) (span (offset 1204) (line 25) (column 33) (len 10)) (segments (segment 0 (token "CountValue") (name "CountValue") (separator none) (span (offset 1204) (line 25) (column 33) (len 10)))))
    (reference r7 (scope relative) (span (offset 2164) (line 43) (column 33) (len 21)) (segments (segment 0 (token "AmountOfSubstanceUnit") (name "AmountOfSubstanceUnit") (separator none) (span (offset 2164) (line 43) (column 33) (len 21)))))
    (reference r8 (scope relative) (span (offset 2220) (line 44) (column 34) (len 22)) (segments (segment 0 (token "AmountOfSubstanceValue") (name "AmountOfSubstanceValue") (separator none) (span (offset 2220) (line 44) (column 34) (len 22)))))
    (reference r9 (scope relative) (span (offset 2272) (line 45) (column 29) (len 17)) (segments (segment 0 (token "amountOfSubstance") (name "amountOfSubstance") (separator none) (span (offset 2272) (line 45) (column 29) (len 17)))))
    (reference r10 (scope relative) (span (offset 2389) (line 48) (column 46) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 2389) (line 48) (column 46) (len 17)))))
    (reference r11 (scope relative) (span (offset 3389) (line 62) (column 35) (len 23)) (segments (segment 0 (token "RelativeAtomicMassValue") (name "RelativeAtomicMassValue") (separator none) (span (offset 3389) (line 62) (column 35) (len 23)))))
    (reference r12 (scope relative) (span (offset 3513) (line 65) (column 37) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 3513) (line 65) (column 37) (len 19)))))
    (reference r13 (scope relative) (span (offset 4010) (line 78) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 4010) (line 78) (column 28) (len 4)))))
    (reference r14 (scope relative) (span (offset 4005) (line 78) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 4005) (line 78) (column 23) (len 3)))))
    (reference r15 (scope relative) (span (offset 4044) (line 79) (column 29) (len 13)) (segments (segment 0 (token "MolarMassUnit") (name "MolarMassUnit") (separator none) (span (offset 4044) (line 79) (column 29) (len 13)))))
    (reference r16 (scope relative) (span (offset 4038) (line 79) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 4038) (line 79) (column 23) (len 4)))))
    (reference r17 (scope relative) (span (offset 4094) (line 82) (column 26) (len 14)) (segments (segment 0 (token "MolarMassValue") (name "MolarMassValue") (separator none) (span (offset 4094) (line 82) (column 26) (len 14)))))
    (reference r18 (scope relative) (span (offset 4179) (line 84) (column 36) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 4179) (line 84) (column 36) (len 11)))))
    (reference r19 (scope relative) (span (offset 4227) (line 85) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 4227) (line 85) (column 35) (len 19)))))
    (reference r20 (scope relative) (span (offset 4256) (line 85) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 4256) (line 85) (column 64) (len 8)))))
    (reference r21 (scope relative) (span (offset 4267) (line 85) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 4267) (line 85) (column 75) (len 3)))))
    (reference r22 (scope relative) (span (offset 4271) (line 85) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 4271) (line 85) (column 79) (len 1)))))
    (reference r23 (scope relative) (span (offset 4278) (line 85) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 4278) (line 85) (column 86) (len 8)))))
    (reference r24 (scope relative) (span (offset 4341) (line 86) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 4341) (line 86) (column 48) (len 19)))))
    (reference r25 (scope relative) (span (offset 4370) (line 86) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 4370) (line 86) (column 77) (len 8)))))
    (reference r26 (scope relative) (span (offset 4381) (line 86) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 4381) (line 86) (column 88) (len 3)))))
    (reference r27 (scope relative) (span (offset 4385) (line 86) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 4385) (line 86) (column 92) (len 1)))))
    (reference r28 (scope relative) (span (offset 4392) (line 86) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 4392) (line 86) (column 99) (len 8)))))
    (reference r29 (scope relative) (span (offset 4431) (line 87) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 4431) (line 87) (column 23) (len 17)))))
    (reference r30 (scope relative) (span (offset 4455) (line 87) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 4455) (line 87) (column 47) (len 20)))))
    (reference r31 (scope relative) (span (offset 4479) (line 87) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 4479) (line 87) (column 71) (len 6)))))
    (reference r32 (scope relative) (span (offset 4487) (line 87) (column 79) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 4487) (line 87) (column 79) (len 19)))))
    (reference r33 (scope relative) (span (offset 4600) (line 91) (column 39) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 4600) (line 91) (column 39) (len 19)))))
    (reference r34 (scope relative) (span (offset 5095) (line 104) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 5095) (line 104) (column 28) (len 4)))))
    (reference r35 (scope relative) (span (offset 5090) (line 104) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 5090) (line 104) (column 23) (len 3)))))
    (reference r36 (scope relative) (span (offset 5129) (line 105) (column 29) (len 15)) (segments (segment 0 (token "MolarVolumeUnit") (name "MolarVolumeUnit") (separator none) (span (offset 5129) (line 105) (column 29) (len 15)))))
    (reference r37 (scope relative) (span (offset 5123) (line 105) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 5123) (line 105) (column 23) (len 4)))))
    (reference r38 (scope relative) (span (offset 5183) (line 108) (column 28) (len 16)) (segments (segment 0 (token "MolarVolumeValue") (name "MolarVolumeValue") (separator none) (span (offset 5183) (line 108) (column 28) (len 16)))))
    (reference r39 (scope relative) (span (offset 5272) (line 110) (column 38) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 5272) (line 110) (column 38) (len 11)))))
    (reference r40 (scope relative) (span (offset 5322) (line 111) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 5322) (line 111) (column 37) (len 19)))))
    (reference r41 (scope relative) (span (offset 5351) (line 111) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 5351) (line 111) (column 66) (len 8)))))
    (reference r42 (scope relative) (span (offset 5362) (line 111) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 5362) (line 111) (column 77) (len 3)))))
    (reference r43 (scope relative) (span (offset 5366) (line 111) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 5366) (line 111) (column 81) (len 1)))))
    (reference r44 (scope relative) (span (offset 5373) (line 111) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 5373) (line 111) (column 88) (len 8)))))
    (reference r45 (scope relative) (span (offset 5436) (line 112) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 5436) (line 112) (column 48) (len 19)))))
    (reference r46 (scope relative) (span (offset 5465) (line 112) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 5465) (line 112) (column 77) (len 8)))))
    (reference r47 (scope relative) (span (offset 5476) (line 112) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 5476) (line 112) (column 88) (len 3)))))
    (reference r48 (scope relative) (span (offset 5480) (line 112) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 5480) (line 112) (column 92) (len 1)))))
    (reference r49 (scope relative) (span (offset 5487) (line 112) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 5487) (line 112) (column 99) (len 8)))))
    (reference r50 (scope relative) (span (offset 5526) (line 113) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 5526) (line 113) (column 23) (len 17)))))
    (reference r51 (scope relative) (span (offset 5550) (line 113) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 5550) (line 113) (column 47) (len 20)))))
    (reference r52 (scope relative) (span (offset 5574) (line 113) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 5574) (line 113) (column 71) (len 8)))))
    (reference r53 (scope relative) (span (offset 5584) (line 113) (column 81) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 5584) (line 113) (column 81) (len 19)))))
    (reference r54 (scope relative) (span (offset 5716) (line 117) (column 47) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 5716) (line 117) (column 47) (len 19)))))
    (reference r55 (scope relative) (span (offset 6306) (line 130) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 6306) (line 130) (column 28) (len 4)))))
    (reference r56 (scope relative) (span (offset 6301) (line 130) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 6301) (line 130) (column 23) (len 3)))))
    (reference r57 (scope relative) (span (offset 6340) (line 131) (column 29) (len 23)) (segments (segment 0 (token "MolarInternalEnergyUnit") (name "MolarInternalEnergyUnit") (separator none) (span (offset 6340) (line 131) (column 29) (len 23)))))
    (reference r58 (scope relative) (span (offset 6334) (line 131) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 6334) (line 131) (column 23) (len 4)))))
    (reference r59 (scope relative) (span (offset 6410) (line 134) (column 36) (len 24)) (segments (segment 0 (token "MolarInternalEnergyValue") (name "MolarInternalEnergyValue") (separator none) (span (offset 6410) (line 134) (column 36) (len 24)))))
    (reference r60 (scope relative) (span (offset 6515) (line 136) (column 46) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 6515) (line 136) (column 46) (len 11)))))
    (reference r61 (scope relative) (span (offset 6565) (line 137) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 6565) (line 137) (column 37) (len 19)))))
    (reference r62 (scope relative) (span (offset 6594) (line 137) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 6594) (line 137) (column 66) (len 8)))))
    (reference r63 (scope relative) (span (offset 6605) (line 137) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 6605) (line 137) (column 77) (len 3)))))
    (reference r64 (scope relative) (span (offset 6609) (line 137) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 6609) (line 137) (column 81) (len 1)))))
    (reference r65 (scope relative) (span (offset 6616) (line 137) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 6616) (line 137) (column 88) (len 8)))))
    (reference r66 (scope relative) (span (offset 6666) (line 138) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 6666) (line 138) (column 35) (len 19)))))
    (reference r67 (scope relative) (span (offset 6695) (line 138) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 6695) (line 138) (column 64) (len 8)))))
    (reference r68 (scope relative) (span (offset 6706) (line 138) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 6706) (line 138) (column 75) (len 3)))))
    (reference r69 (scope relative) (span (offset 6710) (line 138) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 6710) (line 138) (column 79) (len 1)))))
    (reference r70 (scope relative) (span (offset 6717) (line 138) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 6717) (line 138) (column 86) (len 8)))))
    (reference r71 (scope relative) (span (offset 6771) (line 139) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 6771) (line 139) (column 39) (len 19)))))
    (reference r72 (scope relative) (span (offset 6800) (line 139) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 6800) (line 139) (column 68) (len 8)))))
    (reference r73 (scope relative) (span (offset 6811) (line 139) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 6811) (line 139) (column 79) (len 3)))))
    (reference r74 (scope relative) (span (offset 6815) (line 139) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 6815) (line 139) (column 83) (len 1)))))
    (reference r75 (scope relative) (span (offset 6822) (line 139) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 6822) (line 139) (column 90) (len 8)))))
    (reference r76 (scope relative) (span (offset 6886) (line 140) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 6886) (line 140) (column 48) (len 19)))))
    (reference r77 (scope relative) (span (offset 6915) (line 140) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 6915) (line 140) (column 77) (len 8)))))
    (reference r78 (scope relative) (span (offset 6926) (line 140) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 6926) (line 140) (column 88) (len 3)))))
    (reference r79 (scope relative) (span (offset 6930) (line 140) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 6930) (line 140) (column 92) (len 1)))))
    (reference r80 (scope relative) (span (offset 6937) (line 140) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 6937) (line 140) (column 99) (len 8)))))
    (reference r81 (scope relative) (span (offset 6976) (line 141) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 6976) (line 141) (column 23) (len 17)))))
    (reference r82 (scope relative) (span (offset 7000) (line 141) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 7000) (line 141) (column 47) (len 20)))))
    (reference r83 (scope relative) (span (offset 7024) (line 141) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 7024) (line 141) (column 71) (len 8)))))
    (reference r84 (scope relative) (span (offset 7034) (line 141) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 7034) (line 141) (column 81) (len 6)))))
    (reference r85 (scope relative) (span (offset 7042) (line 141) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 7042) (line 141) (column 89) (len 10)))))
    (reference r86 (scope relative) (span (offset 7054) (line 141) (column 101) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 7054) (line 141) (column 101) (len 19)))))
    (reference r87 (scope relative) (span (offset 7173) (line 145) (column 41) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 7173) (line 145) (column 41) (len 19)))))
    (reference r88 (scope relative) (span (offset 7743) (line 158) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 7743) (line 158) (column 28) (len 4)))))
    (reference r89 (scope relative) (span (offset 7738) (line 158) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 7738) (line 158) (column 23) (len 3)))))
    (reference r90 (scope relative) (span (offset 7777) (line 159) (column 29) (len 17)) (segments (segment 0 (token "MolarEnthalpyUnit") (name "MolarEnthalpyUnit") (separator none) (span (offset 7777) (line 159) (column 29) (len 17)))))
    (reference r91 (scope relative) (span (offset 7771) (line 159) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 7771) (line 159) (column 23) (len 4)))))
    (reference r92 (scope relative) (span (offset 7835) (line 162) (column 30) (len 18)) (segments (segment 0 (token "MolarEnthalpyValue") (name "MolarEnthalpyValue") (separator none) (span (offset 7835) (line 162) (column 30) (len 18)))))
    (reference r93 (scope relative) (span (offset 7928) (line 164) (column 40) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 7928) (line 164) (column 40) (len 11)))))
    (reference r94 (scope relative) (span (offset 7978) (line 165) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 7978) (line 165) (column 37) (len 19)))))
    (reference r95 (scope relative) (span (offset 8007) (line 165) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 8007) (line 165) (column 66) (len 8)))))
    (reference r96 (scope relative) (span (offset 8018) (line 165) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 8018) (line 165) (column 77) (len 3)))))
    (reference r97 (scope relative) (span (offset 8022) (line 165) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 8022) (line 165) (column 81) (len 1)))))
    (reference r98 (scope relative) (span (offset 8029) (line 165) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 8029) (line 165) (column 88) (len 8)))))
    (reference r99 (scope relative) (span (offset 8079) (line 166) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 8079) (line 166) (column 35) (len 19)))))
    (reference r100 (scope relative) (span (offset 8108) (line 166) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 8108) (line 166) (column 64) (len 8)))))
    (reference r101 (scope relative) (span (offset 8119) (line 166) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 8119) (line 166) (column 75) (len 3)))))
    (reference r102 (scope relative) (span (offset 8123) (line 166) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 8123) (line 166) (column 79) (len 1)))))
    (reference r103 (scope relative) (span (offset 8130) (line 166) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 8130) (line 166) (column 86) (len 8)))))
    (reference r104 (scope relative) (span (offset 8184) (line 167) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 8184) (line 167) (column 39) (len 19)))))
    (reference r105 (scope relative) (span (offset 8213) (line 167) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 8213) (line 167) (column 68) (len 8)))))
    (reference r106 (scope relative) (span (offset 8224) (line 167) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 8224) (line 167) (column 79) (len 3)))))
    (reference r107 (scope relative) (span (offset 8228) (line 167) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 8228) (line 167) (column 83) (len 1)))))
    (reference r108 (scope relative) (span (offset 8235) (line 167) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 8235) (line 167) (column 90) (len 8)))))
    (reference r109 (scope relative) (span (offset 8299) (line 168) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 8299) (line 168) (column 48) (len 19)))))
    (reference r110 (scope relative) (span (offset 8328) (line 168) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 8328) (line 168) (column 77) (len 8)))))
    (reference r111 (scope relative) (span (offset 8339) (line 168) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 8339) (line 168) (column 88) (len 3)))))
    (reference r112 (scope relative) (span (offset 8343) (line 168) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 8343) (line 168) (column 92) (len 1)))))
    (reference r113 (scope relative) (span (offset 8350) (line 168) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 8350) (line 168) (column 99) (len 8)))))
    (reference r114 (scope relative) (span (offset 8389) (line 169) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 8389) (line 169) (column 23) (len 17)))))
    (reference r115 (scope relative) (span (offset 8413) (line 169) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 8413) (line 169) (column 47) (len 20)))))
    (reference r116 (scope relative) (span (offset 8437) (line 169) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 8437) (line 169) (column 71) (len 8)))))
    (reference r117 (scope relative) (span (offset 8447) (line 169) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 8447) (line 169) (column 81) (len 6)))))
    (reference r118 (scope relative) (span (offset 8455) (line 169) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 8455) (line 169) (column 89) (len 10)))))
    (reference r119 (scope relative) (span (offset 8467) (line 169) (column 101) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 8467) (line 169) (column 101) (len 19)))))
    (reference r120 (scope relative) (span (offset 8601) (line 173) (column 48) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 8601) (line 173) (column 48) (len 19)))))
    (reference r121 (scope relative) (span (offset 9198) (line 186) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 9198) (line 186) (column 28) (len 4)))))
    (reference r122 (scope relative) (span (offset 9193) (line 186) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 9193) (line 186) (column 23) (len 3)))))
    (reference r123 (scope relative) (span (offset 9232) (line 187) (column 29) (len 24)) (segments (segment 0 (token "MolarHelmholtzEnergyUnit") (name "MolarHelmholtzEnergyUnit") (separator none) (span (offset 9232) (line 187) (column 29) (len 24)))))
    (reference r124 (scope relative) (span (offset 9226) (line 187) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 9226) (line 187) (column 23) (len 4)))))
    (reference r125 (scope relative) (span (offset 9304) (line 190) (column 37) (len 25)) (segments (segment 0 (token "MolarHelmholtzEnergyValue") (name "MolarHelmholtzEnergyValue") (separator none) (span (offset 9304) (line 190) (column 37) (len 25)))))
    (reference r126 (scope relative) (span (offset 9411) (line 192) (column 47) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 9411) (line 192) (column 47) (len 11)))))
    (reference r127 (scope relative) (span (offset 9461) (line 193) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 9461) (line 193) (column 37) (len 19)))))
    (reference r128 (scope relative) (span (offset 9490) (line 193) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 9490) (line 193) (column 66) (len 8)))))
    (reference r129 (scope relative) (span (offset 9501) (line 193) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 9501) (line 193) (column 77) (len 3)))))
    (reference r130 (scope relative) (span (offset 9505) (line 193) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 9505) (line 193) (column 81) (len 1)))))
    (reference r131 (scope relative) (span (offset 9512) (line 193) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 9512) (line 193) (column 88) (len 8)))))
    (reference r132 (scope relative) (span (offset 9562) (line 194) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 9562) (line 194) (column 35) (len 19)))))
    (reference r133 (scope relative) (span (offset 9591) (line 194) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 9591) (line 194) (column 64) (len 8)))))
    (reference r134 (scope relative) (span (offset 9602) (line 194) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 9602) (line 194) (column 75) (len 3)))))
    (reference r135 (scope relative) (span (offset 9606) (line 194) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 9606) (line 194) (column 79) (len 1)))))
    (reference r136 (scope relative) (span (offset 9613) (line 194) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 9613) (line 194) (column 86) (len 8)))))
    (reference r137 (scope relative) (span (offset 9667) (line 195) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 9667) (line 195) (column 39) (len 19)))))
    (reference r138 (scope relative) (span (offset 9696) (line 195) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 9696) (line 195) (column 68) (len 8)))))
    (reference r139 (scope relative) (span (offset 9707) (line 195) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 9707) (line 195) (column 79) (len 3)))))
    (reference r140 (scope relative) (span (offset 9711) (line 195) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 9711) (line 195) (column 83) (len 1)))))
    (reference r141 (scope relative) (span (offset 9718) (line 195) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 9718) (line 195) (column 90) (len 8)))))
    (reference r142 (scope relative) (span (offset 9782) (line 196) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 9782) (line 196) (column 48) (len 19)))))
    (reference r143 (scope relative) (span (offset 9811) (line 196) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 9811) (line 196) (column 77) (len 8)))))
    (reference r144 (scope relative) (span (offset 9822) (line 196) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 9822) (line 196) (column 88) (len 3)))))
    (reference r145 (scope relative) (span (offset 9826) (line 196) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 9826) (line 196) (column 92) (len 1)))))
    (reference r146 (scope relative) (span (offset 9833) (line 196) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 9833) (line 196) (column 99) (len 8)))))
    (reference r147 (scope relative) (span (offset 9872) (line 197) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 9872) (line 197) (column 23) (len 17)))))
    (reference r148 (scope relative) (span (offset 9896) (line 197) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 9896) (line 197) (column 47) (len 20)))))
    (reference r149 (scope relative) (span (offset 9920) (line 197) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 9920) (line 197) (column 71) (len 8)))))
    (reference r150 (scope relative) (span (offset 9930) (line 197) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 9930) (line 197) (column 81) (len 6)))))
    (reference r151 (scope relative) (span (offset 9938) (line 197) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 9938) (line 197) (column 89) (len 10)))))
    (reference r152 (scope relative) (span (offset 9950) (line 197) (column 101) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 9950) (line 197) (column 101) (len 19)))))
    (reference r153 (scope relative) (span (offset 10076) (line 201) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 10076) (line 201) (column 44) (len 19)))))
    (reference r154 (scope relative) (span (offset 10661) (line 214) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 10661) (line 214) (column 28) (len 4)))))
    (reference r155 (scope relative) (span (offset 10656) (line 214) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 10656) (line 214) (column 23) (len 3)))))
    (reference r156 (scope relative) (span (offset 10695) (line 215) (column 29) (len 20)) (segments (segment 0 (token "MolarGibbsEnergyUnit") (name "MolarGibbsEnergyUnit") (separator none) (span (offset 10695) (line 215) (column 29) (len 20)))))
    (reference r157 (scope relative) (span (offset 10689) (line 215) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 10689) (line 215) (column 23) (len 4)))))
    (reference r158 (scope relative) (span (offset 10759) (line 218) (column 33) (len 21)) (segments (segment 0 (token "MolarGibbsEnergyValue") (name "MolarGibbsEnergyValue") (separator none) (span (offset 10759) (line 218) (column 33) (len 21)))))
    (reference r159 (scope relative) (span (offset 10858) (line 220) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 10858) (line 220) (column 43) (len 11)))))
    (reference r160 (scope relative) (span (offset 10908) (line 221) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 10908) (line 221) (column 37) (len 19)))))
    (reference r161 (scope relative) (span (offset 10937) (line 221) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 10937) (line 221) (column 66) (len 8)))))
    (reference r162 (scope relative) (span (offset 10948) (line 221) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 10948) (line 221) (column 77) (len 3)))))
    (reference r163 (scope relative) (span (offset 10952) (line 221) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 10952) (line 221) (column 81) (len 1)))))
    (reference r164 (scope relative) (span (offset 10959) (line 221) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 10959) (line 221) (column 88) (len 8)))))
    (reference r165 (scope relative) (span (offset 11009) (line 222) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 11009) (line 222) (column 35) (len 19)))))
    (reference r166 (scope relative) (span (offset 11038) (line 222) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 11038) (line 222) (column 64) (len 8)))))
    (reference r167 (scope relative) (span (offset 11049) (line 222) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 11049) (line 222) (column 75) (len 3)))))
    (reference r168 (scope relative) (span (offset 11053) (line 222) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 11053) (line 222) (column 79) (len 1)))))
    (reference r169 (scope relative) (span (offset 11060) (line 222) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 11060) (line 222) (column 86) (len 8)))))
    (reference r170 (scope relative) (span (offset 11114) (line 223) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 11114) (line 223) (column 39) (len 19)))))
    (reference r171 (scope relative) (span (offset 11143) (line 223) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 11143) (line 223) (column 68) (len 8)))))
    (reference r172 (scope relative) (span (offset 11154) (line 223) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 11154) (line 223) (column 79) (len 3)))))
    (reference r173 (scope relative) (span (offset 11158) (line 223) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 11158) (line 223) (column 83) (len 1)))))
    (reference r174 (scope relative) (span (offset 11165) (line 223) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 11165) (line 223) (column 90) (len 8)))))
    (reference r175 (scope relative) (span (offset 11229) (line 224) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 11229) (line 224) (column 48) (len 19)))))
    (reference r176 (scope relative) (span (offset 11258) (line 224) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 11258) (line 224) (column 77) (len 8)))))
    (reference r177 (scope relative) (span (offset 11269) (line 224) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 11269) (line 224) (column 88) (len 3)))))
    (reference r178 (scope relative) (span (offset 11273) (line 224) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 11273) (line 224) (column 92) (len 1)))))
    (reference r179 (scope relative) (span (offset 11280) (line 224) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 11280) (line 224) (column 99) (len 8)))))
    (reference r180 (scope relative) (span (offset 11319) (line 225) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 11319) (line 225) (column 23) (len 17)))))
    (reference r181 (scope relative) (span (offset 11343) (line 225) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 11343) (line 225) (column 47) (len 20)))))
    (reference r182 (scope relative) (span (offset 11367) (line 225) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 11367) (line 225) (column 71) (len 8)))))
    (reference r183 (scope relative) (span (offset 11377) (line 225) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 11377) (line 225) (column 81) (len 6)))))
    (reference r184 (scope relative) (span (offset 11385) (line 225) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 11385) (line 225) (column 89) (len 10)))))
    (reference r185 (scope relative) (span (offset 11397) (line 225) (column 101) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 11397) (line 225) (column 101) (len 19)))))
    (reference r186 (scope relative) (span (offset 11523) (line 229) (column 45) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 11523) (line 229) (column 45) (len 19)))))
    (reference r187 (scope relative) (span (offset 12110) (line 242) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 12110) (line 242) (column 28) (len 4)))))
    (reference r188 (scope relative) (span (offset 12105) (line 242) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 12105) (line 242) (column 23) (len 3)))))
    (reference r189 (scope relative) (span (offset 12144) (line 243) (column 29) (len 21)) (segments (segment 0 (token "MolarHeatCapacityUnit") (name "MolarHeatCapacityUnit") (separator none) (span (offset 12144) (line 243) (column 29) (len 21)))))
    (reference r190 (scope relative) (span (offset 12138) (line 243) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 12138) (line 243) (column 23) (len 4)))))
    (reference r191 (scope relative) (span (offset 12210) (line 246) (column 34) (len 22)) (segments (segment 0 (token "MolarHeatCapacityValue") (name "MolarHeatCapacityValue") (separator none) (span (offset 12210) (line 246) (column 34) (len 22)))))
    (reference r192 (scope relative) (span (offset 12311) (line 248) (column 44) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 12311) (line 248) (column 44) (len 11)))))
    (reference r193 (scope relative) (span (offset 12361) (line 249) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12361) (line 249) (column 37) (len 19)))))
    (reference r194 (scope relative) (span (offset 12390) (line 249) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12390) (line 249) (column 66) (len 8)))))
    (reference r195 (scope relative) (span (offset 12401) (line 249) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12401) (line 249) (column 77) (len 3)))))
    (reference r196 (scope relative) (span (offset 12405) (line 249) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 12405) (line 249) (column 81) (len 1)))))
    (reference r197 (scope relative) (span (offset 12412) (line 249) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12412) (line 249) (column 88) (len 8)))))
    (reference r198 (scope relative) (span (offset 12462) (line 250) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12462) (line 250) (column 35) (len 19)))))
    (reference r199 (scope relative) (span (offset 12491) (line 250) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12491) (line 250) (column 64) (len 8)))))
    (reference r200 (scope relative) (span (offset 12502) (line 250) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12502) (line 250) (column 75) (len 3)))))
    (reference r201 (scope relative) (span (offset 12506) (line 250) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 12506) (line 250) (column 79) (len 1)))))
    (reference r202 (scope relative) (span (offset 12513) (line 250) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12513) (line 250) (column 86) (len 8)))))
    (reference r203 (scope relative) (span (offset 12567) (line 251) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12567) (line 251) (column 39) (len 19)))))
    (reference r204 (scope relative) (span (offset 12596) (line 251) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12596) (line 251) (column 68) (len 8)))))
    (reference r205 (scope relative) (span (offset 12607) (line 251) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12607) (line 251) (column 79) (len 3)))))
    (reference r206 (scope relative) (span (offset 12611) (line 251) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 12611) (line 251) (column 83) (len 1)))))
    (reference r207 (scope relative) (span (offset 12618) (line 251) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12618) (line 251) (column 90) (len 8)))))
    (reference r208 (scope relative) (span (offset 12689) (line 252) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12689) (line 252) (column 55) (len 19)))))
    (reference r209 (scope relative) (span (offset 12718) (line 252) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12718) (line 252) (column 84) (len 8)))))
    (reference r210 (scope relative) (span (offset 12729) (line 252) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12729) (line 252) (column 95) (len 3)))))
    (reference r211 (scope relative) (span (offset 12733) (line 252) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 12733) (line 252) (column 99) (len 4)))))
    (reference r212 (scope relative) (span (offset 12743) (line 252) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12743) (line 252) (column 109) (len 8)))))
    (reference r213 (scope relative) (span (offset 12807) (line 253) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12807) (line 253) (column 48) (len 19)))))
    (reference r214 (scope relative) (span (offset 12836) (line 253) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12836) (line 253) (column 77) (len 8)))))
    (reference r215 (scope relative) (span (offset 12847) (line 253) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12847) (line 253) (column 88) (len 3)))))
    (reference r216 (scope relative) (span (offset 12851) (line 253) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 12851) (line 253) (column 92) (len 1)))))
    (reference r217 (scope relative) (span (offset 12858) (line 253) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12858) (line 253) (column 99) (len 8)))))
    (reference r218 (scope relative) (span (offset 12897) (line 254) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 12897) (line 254) (column 23) (len 17)))))
    (reference r219 (scope relative) (span (offset 12921) (line 254) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 12921) (line 254) (column 47) (len 20)))))
    (reference r220 (scope relative) (span (offset 12945) (line 254) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 12945) (line 254) (column 71) (len 8)))))
    (reference r221 (scope relative) (span (offset 12955) (line 254) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 12955) (line 254) (column 81) (len 6)))))
    (reference r222 (scope relative) (span (offset 12963) (line 254) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 12963) (line 254) (column 89) (len 10)))))
    (reference r223 (scope relative) (span (offset 12975) (line 254) (column 101) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 12975) (line 254) (column 101) (len 26)))))
    (reference r224 (scope relative) (span (offset 13003) (line 254) (column 129) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 13003) (line 254) (column 129) (len 19)))))
    (reference r225 (scope relative) (span (offset 13118) (line 258) (column 40) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 13118) (line 258) (column 40) (len 19)))))
    (reference r226 (scope relative) (span (offset 13688) (line 271) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 13688) (line 271) (column 28) (len 4)))))
    (reference r227 (scope relative) (span (offset 13683) (line 271) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 13683) (line 271) (column 23) (len 3)))))
    (reference r228 (scope relative) (span (offset 13722) (line 272) (column 29) (len 16)) (segments (segment 0 (token "MolarEntropyUnit") (name "MolarEntropyUnit") (separator none) (span (offset 13722) (line 272) (column 29) (len 16)))))
    (reference r229 (scope relative) (span (offset 13716) (line 272) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 13716) (line 272) (column 23) (len 4)))))
    (reference r230 (scope relative) (span (offset 13778) (line 275) (column 29) (len 17)) (segments (segment 0 (token "MolarEntropyValue") (name "MolarEntropyValue") (separator none) (span (offset 13778) (line 275) (column 29) (len 17)))))
    (reference r231 (scope relative) (span (offset 13869) (line 277) (column 39) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 13869) (line 277) (column 39) (len 11)))))
    (reference r232 (scope relative) (span (offset 13919) (line 278) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 13919) (line 278) (column 37) (len 19)))))
    (reference r233 (scope relative) (span (offset 13948) (line 278) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 13948) (line 278) (column 66) (len 8)))))
    (reference r234 (scope relative) (span (offset 13959) (line 278) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 13959) (line 278) (column 77) (len 3)))))
    (reference r235 (scope relative) (span (offset 13963) (line 278) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 13963) (line 278) (column 81) (len 1)))))
    (reference r236 (scope relative) (span (offset 13970) (line 278) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 13970) (line 278) (column 88) (len 8)))))
    (reference r237 (scope relative) (span (offset 14020) (line 279) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 14020) (line 279) (column 35) (len 19)))))
    (reference r238 (scope relative) (span (offset 14049) (line 279) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 14049) (line 279) (column 64) (len 8)))))
    (reference r239 (scope relative) (span (offset 14060) (line 279) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 14060) (line 279) (column 75) (len 3)))))
    (reference r240 (scope relative) (span (offset 14064) (line 279) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 14064) (line 279) (column 79) (len 1)))))
    (reference r241 (scope relative) (span (offset 14071) (line 279) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 14071) (line 279) (column 86) (len 8)))))
    (reference r242 (scope relative) (span (offset 14125) (line 280) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 14125) (line 280) (column 39) (len 19)))))
    (reference r243 (scope relative) (span (offset 14154) (line 280) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 14154) (line 280) (column 68) (len 8)))))
    (reference r244 (scope relative) (span (offset 14165) (line 280) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 14165) (line 280) (column 79) (len 3)))))
    (reference r245 (scope relative) (span (offset 14169) (line 280) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 14169) (line 280) (column 83) (len 1)))))
    (reference r246 (scope relative) (span (offset 14176) (line 280) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 14176) (line 280) (column 90) (len 8)))))
    (reference r247 (scope relative) (span (offset 14247) (line 281) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 14247) (line 281) (column 55) (len 19)))))
    (reference r248 (scope relative) (span (offset 14276) (line 281) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 14276) (line 281) (column 84) (len 8)))))
    (reference r249 (scope relative) (span (offset 14287) (line 281) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 14287) (line 281) (column 95) (len 3)))))
    (reference r250 (scope relative) (span (offset 14291) (line 281) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 14291) (line 281) (column 99) (len 4)))))
    (reference r251 (scope relative) (span (offset 14301) (line 281) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 14301) (line 281) (column 109) (len 8)))))
    (reference r252 (scope relative) (span (offset 14365) (line 282) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 14365) (line 282) (column 48) (len 19)))))
    (reference r253 (scope relative) (span (offset 14394) (line 282) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 14394) (line 282) (column 77) (len 8)))))
    (reference r254 (scope relative) (span (offset 14405) (line 282) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 14405) (line 282) (column 88) (len 3)))))
    (reference r255 (scope relative) (span (offset 14409) (line 282) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 14409) (line 282) (column 92) (len 1)))))
    (reference r256 (scope relative) (span (offset 14416) (line 282) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 14416) (line 282) (column 99) (len 8)))))
    (reference r257 (scope relative) (span (offset 14455) (line 283) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 14455) (line 283) (column 23) (len 17)))))
    (reference r258 (scope relative) (span (offset 14479) (line 283) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 14479) (line 283) (column 47) (len 20)))))
    (reference r259 (scope relative) (span (offset 14503) (line 283) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 14503) (line 283) (column 71) (len 8)))))
    (reference r260 (scope relative) (span (offset 14513) (line 283) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 14513) (line 283) (column 81) (len 6)))))
    (reference r261 (scope relative) (span (offset 14521) (line 283) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 14521) (line 283) (column 89) (len 10)))))
    (reference r262 (scope relative) (span (offset 14533) (line 283) (column 101) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 14533) (line 283) (column 101) (len 26)))))
    (reference r263 (scope relative) (span (offset 14561) (line 283) (column 129) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 14561) (line 283) (column 129) (len 19)))))
    (reference r264 (scope relative) (span (offset 14696) (line 287) (column 49) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 14696) (line 287) (column 49) (len 19)))))
    (reference r265 (scope relative) (span (offset 15214) (line 300) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 15214) (line 300) (column 28) (len 4)))))
    (reference r266 (scope relative) (span (offset 15209) (line 300) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 15209) (line 300) (column 23) (len 3)))))
    (reference r267 (scope relative) (span (offset 15248) (line 301) (column 29) (len 25)) (segments (segment 0 (token "ParticleConcentrationUnit") (name "ParticleConcentrationUnit") (separator none) (span (offset 15248) (line 301) (column 29) (len 25)))))
    (reference r268 (scope relative) (span (offset 15242) (line 301) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 15242) (line 301) (column 23) (len 4)))))
    (reference r269 (scope relative) (span (offset 15322) (line 304) (column 38) (len 26)) (segments (segment 0 (token "ParticleConcentrationValue") (name "ParticleConcentrationValue") (separator none) (span (offset 15322) (line 304) (column 38) (len 26)))))
    (reference r270 (scope relative) (span (offset 15431) (line 306) (column 48) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 15431) (line 306) (column 48) (len 11)))))
    (reference r271 (scope relative) (span (offset 15481) (line 307) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 15481) (line 307) (column 37) (len 19)))))
    (reference r272 (scope relative) (span (offset 15510) (line 307) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 15510) (line 307) (column 66) (len 8)))))
    (reference r273 (scope relative) (span (offset 15521) (line 307) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 15521) (line 307) (column 77) (len 3)))))
    (reference r274 (scope relative) (span (offset 15525) (line 307) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 15525) (line 307) (column 81) (len 1)))))
    (reference r275 (scope relative) (span (offset 15532) (line 307) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 15532) (line 307) (column 88) (len 8)))))
    (reference r276 (scope relative) (span (offset 15571) (line 308) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 15571) (line 308) (column 23) (len 17)))))
    (reference r277 (scope relative) (span (offset 15595) (line 308) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 15595) (line 308) (column 47) (len 20)))))
    (reference r278 (scope relative) (span (offset 15618) (line 308) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 15618) (line 308) (column 70) (len 8)))))
    (reference r279 (scope relative) (span (offset 15732) (line 312) (column 39) (len 26)) (segments (segment 0 (token "ParticleConcentrationValue") (name "ParticleConcentrationValue") (separator none) (span (offset 15732) (line 312) (column 39) (len 26)))))
    (reference r280 (scope relative) (span (offset 16418) (line 328) (column 45) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 16418) (line 328) (column 45) (len 19)))))
    (reference r281 (scope relative) (span (offset 17057) (line 341) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 17057) (line 341) (column 28) (len 4)))))
    (reference r282 (scope relative) (span (offset 17052) (line 341) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 17052) (line 341) (column 23) (len 3)))))
    (reference r283 (scope relative) (span (offset 17091) (line 342) (column 29) (len 21)) (segments (segment 0 (token "MassConcentrationUnit") (name "MassConcentrationUnit") (separator none) (span (offset 17091) (line 342) (column 29) (len 21)))))
    (reference r284 (scope relative) (span (offset 17085) (line 342) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 17085) (line 342) (column 23) (len 4)))))
    (reference r285 (scope relative) (span (offset 17157) (line 345) (column 34) (len 22)) (segments (segment 0 (token "MassConcentrationValue") (name "MassConcentrationValue") (separator none) (span (offset 17157) (line 345) (column 34) (len 22)))))
    (reference r286 (scope relative) (span (offset 17258) (line 347) (column 44) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 17258) (line 347) (column 44) (len 11)))))
    (reference r287 (scope relative) (span (offset 17308) (line 348) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 17308) (line 348) (column 37) (len 19)))))
    (reference r288 (scope relative) (span (offset 17337) (line 348) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 17337) (line 348) (column 66) (len 8)))))
    (reference r289 (scope relative) (span (offset 17348) (line 348) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 17348) (line 348) (column 77) (len 3)))))
    (reference r290 (scope relative) (span (offset 17352) (line 348) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 17352) (line 348) (column 81) (len 1)))))
    (reference r291 (scope relative) (span (offset 17359) (line 348) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 17359) (line 348) (column 88) (len 8)))))
    (reference r292 (scope relative) (span (offset 17410) (line 349) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 17410) (line 349) (column 35) (len 19)))))
    (reference r293 (scope relative) (span (offset 17439) (line 349) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 17439) (line 349) (column 64) (len 8)))))
    (reference r294 (scope relative) (span (offset 17450) (line 349) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 17450) (line 349) (column 75) (len 3)))))
    (reference r295 (scope relative) (span (offset 17454) (line 349) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 17454) (line 349) (column 79) (len 1)))))
    (reference r296 (scope relative) (span (offset 17461) (line 349) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 17461) (line 349) (column 86) (len 8)))))
    (reference r297 (scope relative) (span (offset 17499) (line 350) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 17499) (line 350) (column 23) (len 17)))))
    (reference r298 (scope relative) (span (offset 17523) (line 350) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 17523) (line 350) (column 47) (len 20)))))
    (reference r299 (scope relative) (span (offset 17547) (line 350) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 17547) (line 350) (column 71) (len 8)))))
    (reference r300 (scope relative) (span (offset 17557) (line 350) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 17557) (line 350) (column 81) (len 6)))))
    (reference r301 (scope relative) (span (offset 17660) (line 354) (column 40) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 17660) (line 354) (column 40) (len 17)))))
    (reference r302 (scope relative) (span (offset 18202) (line 368) (column 29) (len 17)) (segments (segment 0 (token "MassFractionValue") (name "MassFractionValue") (separator none) (span (offset 18202) (line 368) (column 29) (len 17)))))
    (reference r303 (scope relative) (span (offset 18367) (line 371) (column 58) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 18367) (line 371) (column 58) (len 19)))))
    (reference r304 (scope relative) (span (offset 19335) (line 384) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 19335) (line 384) (column 28) (len 4)))))
    (reference r305 (scope relative) (span (offset 19330) (line 384) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 19330) (line 384) (column 23) (len 3)))))
    (reference r306 (scope relative) (span (offset 19369) (line 385) (column 29) (len 34)) (segments (segment 0 (token "AmountOfSubstanceConcentrationUnit") (name "AmountOfSubstanceConcentrationUnit") (separator none) (span (offset 19369) (line 385) (column 29) (len 34)))))
    (reference r307 (scope relative) (span (offset 19363) (line 385) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 19363) (line 385) (column 23) (len 4)))))
    (reference r308 (scope relative) (span (offset 19461) (line 388) (column 47) (len 35)) (segments (segment 0 (token "AmountOfSubstanceConcentrationValue") (name "AmountOfSubstanceConcentrationValue") (separator none) (span (offset 19461) (line 388) (column 47) (len 35)))))
    (reference r309 (scope relative) (span (offset 19588) (line 390) (column 57) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 19588) (line 390) (column 57) (len 11)))))
    (reference r310 (scope relative) (span (offset 19638) (line 391) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 19638) (line 391) (column 37) (len 19)))))
    (reference r311 (scope relative) (span (offset 19667) (line 391) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 19667) (line 391) (column 66) (len 8)))))
    (reference r312 (scope relative) (span (offset 19678) (line 391) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 19678) (line 391) (column 77) (len 3)))))
    (reference r313 (scope relative) (span (offset 19682) (line 391) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 19682) (line 391) (column 81) (len 1)))))
    (reference r314 (scope relative) (span (offset 19689) (line 391) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 19689) (line 391) (column 88) (len 8)))))
    (reference r315 (scope relative) (span (offset 19753) (line 392) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 19753) (line 392) (column 48) (len 19)))))
    (reference r316 (scope relative) (span (offset 19782) (line 392) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 19782) (line 392) (column 77) (len 8)))))
    (reference r317 (scope relative) (span (offset 19793) (line 392) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 19793) (line 392) (column 88) (len 3)))))
    (reference r318 (scope relative) (span (offset 19797) (line 392) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 19797) (line 392) (column 92) (len 1)))))
    (reference r319 (scope relative) (span (offset 19804) (line 392) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 19804) (line 392) (column 99) (len 8)))))
    (reference r320 (scope relative) (span (offset 19842) (line 393) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 19842) (line 393) (column 23) (len 17)))))
    (reference r321 (scope relative) (span (offset 19866) (line 393) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 19866) (line 393) (column 47) (len 20)))))
    (reference r322 (scope relative) (span (offset 19890) (line 393) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 19890) (line 393) (column 71) (len 8)))))
    (reference r323 (scope relative) (span (offset 19900) (line 393) (column 81) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 19900) (line 393) (column 81) (len 19)))))
    (reference r324 (scope relative) (span (offset 20062) (line 397) (column 55) (len 35)) (segments (segment 0 (token "AmountOfSubstanceConcentrationValue") (name "AmountOfSubstanceConcentrationValue") (separator none) (span (offset 20062) (line 397) (column 55) (len 35)))))
    (reference r325 (scope relative) (span (offset 20835) (line 413) (column 65) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 20835) (line 413) (column 65) (len 17)))))
    (reference r326 (scope relative) (span (offset 21802) (line 427) (column 54) (len 42)) (segments (segment 0 (token "AmountOfSubstanceFractionMoleFractionValue") (name "AmountOfSubstanceFractionMoleFractionValue") (separator none) (span (offset 21802) (line 427) (column 54) (len 42)))))
    (reference r327 (scope relative) (span (offset 21956) (line 430) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 21956) (line 430) (column 42) (len 19)))))
    (reference r328 (scope relative) (span (offset 22905) (line 443) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 22905) (line 443) (column 28) (len 4)))))
    (reference r329 (scope relative) (span (offset 22900) (line 443) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 22900) (line 443) (column 23) (len 3)))))
    (reference r330 (scope relative) (span (offset 22939) (line 444) (column 29) (len 18)) (segments (segment 0 (token "VolumeFractionUnit") (name "VolumeFractionUnit") (separator none) (span (offset 22939) (line 444) (column 29) (len 18)))))
    (reference r331 (scope relative) (span (offset 22933) (line 444) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 22933) (line 444) (column 23) (len 4)))))
    (reference r332 (scope relative) (span (offset 22999) (line 447) (column 31) (len 19)) (segments (segment 0 (token "VolumeFractionValue") (name "VolumeFractionValue") (separator none) (span (offset 22999) (line 447) (column 31) (len 19)))))
    (reference r333 (scope relative) (span (offset 23094) (line 449) (column 41) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 23094) (line 449) (column 41) (len 16)))))
    (reference r334 (scope relative) (span (offset 23196) (line 453) (column 36) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 23196) (line 453) (column 36) (len 19)))))
    (reference r335 (scope relative) (span (offset 23947) (line 466) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 23947) (line 466) (column 28) (len 4)))))
    (reference r336 (scope relative) (span (offset 23942) (line 466) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 23942) (line 466) (column 23) (len 3)))))
    (reference r337 (scope relative) (span (offset 23981) (line 467) (column 29) (len 12)) (segments (segment 0 (token "MolalityUnit") (name "MolalityUnit") (separator none) (span (offset 23981) (line 467) (column 29) (len 12)))))
    (reference r338 (scope relative) (span (offset 23975) (line 467) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 23975) (line 467) (column 23) (len 4)))))
    (reference r339 (scope relative) (span (offset 24029) (line 470) (column 25) (len 13)) (segments (segment 0 (token "MolalityValue") (name "MolalityValue") (separator none) (span (offset 24029) (line 470) (column 25) (len 13)))))
    (reference r340 (scope relative) (span (offset 24112) (line 472) (column 35) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 24112) (line 472) (column 35) (len 11)))))
    (reference r341 (scope relative) (span (offset 24160) (line 473) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 24160) (line 473) (column 35) (len 19)))))
    (reference r342 (scope relative) (span (offset 24189) (line 473) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 24189) (line 473) (column 64) (len 8)))))
    (reference r343 (scope relative) (span (offset 24200) (line 473) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 24200) (line 473) (column 75) (len 3)))))
    (reference r344 (scope relative) (span (offset 24204) (line 473) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 24204) (line 473) (column 79) (len 1)))))
    (reference r345 (scope relative) (span (offset 24211) (line 473) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 24211) (line 473) (column 86) (len 8)))))
    (reference r346 (scope relative) (span (offset 24275) (line 474) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 24275) (line 474) (column 48) (len 19)))))
    (reference r347 (scope relative) (span (offset 24304) (line 474) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 24304) (line 474) (column 77) (len 8)))))
    (reference r348 (scope relative) (span (offset 24315) (line 474) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 24315) (line 474) (column 88) (len 3)))))
    (reference r349 (scope relative) (span (offset 24319) (line 474) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 24319) (line 474) (column 92) (len 1)))))
    (reference r350 (scope relative) (span (offset 24326) (line 474) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 24326) (line 474) (column 99) (len 8)))))
    (reference r351 (scope relative) (span (offset 24364) (line 475) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 24364) (line 475) (column 23) (len 17)))))
    (reference r352 (scope relative) (span (offset 24388) (line 475) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 24388) (line 475) (column 47) (len 20)))))
    (reference r353 (scope relative) (span (offset 24412) (line 475) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 24412) (line 475) (column 71) (len 6)))))
    (reference r354 (scope relative) (span (offset 24420) (line 475) (column 79) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 24420) (line 475) (column 79) (len 19)))))
    (reference r355 (scope relative) (span (offset 24588) (line 479) (column 44) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 24588) (line 479) (column 44) (len 11)))))
    (reference r356 (scope relative) (span (offset 25526) (line 494) (column 41) (len 27)) (segments (segment 0 (token "latentHeatOfPhaseTransition") (name "latentHeatOfPhaseTransition") (separator none) (span (offset 25526) (line 494) (column 41) (len 27)))))
    (reference r357 (scope relative) (span (offset 25651) (line 497) (column 45) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 25651) (line 497) (column 45) (len 19)))))
    (reference r358 (scope relative) (span (offset 26477) (line 510) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 26477) (line 510) (column 28) (len 4)))))
    (reference r359 (scope relative) (span (offset 26472) (line 510) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 26472) (line 510) (column 23) (len 3)))))
    (reference r360 (scope relative) (span (offset 26511) (line 511) (column 29) (len 21)) (segments (segment 0 (token "ChemicalPotentialUnit") (name "ChemicalPotentialUnit") (separator none) (span (offset 26511) (line 511) (column 29) (len 21)))))
    (reference r361 (scope relative) (span (offset 26505) (line 511) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 26505) (line 511) (column 23) (len 4)))))
    (reference r362 (scope relative) (span (offset 26577) (line 514) (column 34) (len 22)) (segments (segment 0 (token "ChemicalPotentialValue") (name "ChemicalPotentialValue") (separator none) (span (offset 26577) (line 514) (column 34) (len 22)))))
    (reference r363 (scope relative) (span (offset 26678) (line 516) (column 44) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 26678) (line 516) (column 44) (len 11)))))
    (reference r364 (scope relative) (span (offset 26728) (line 517) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 26728) (line 517) (column 37) (len 19)))))
    (reference r365 (scope relative) (span (offset 26757) (line 517) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 26757) (line 517) (column 66) (len 8)))))
    (reference r366 (scope relative) (span (offset 26768) (line 517) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 26768) (line 517) (column 77) (len 3)))))
    (reference r367 (scope relative) (span (offset 26772) (line 517) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 26772) (line 517) (column 81) (len 1)))))
    (reference r368 (scope relative) (span (offset 26779) (line 517) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 26779) (line 517) (column 88) (len 8)))))
    (reference r369 (scope relative) (span (offset 26829) (line 518) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 26829) (line 518) (column 35) (len 19)))))
    (reference r370 (scope relative) (span (offset 26858) (line 518) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 26858) (line 518) (column 64) (len 8)))))
    (reference r371 (scope relative) (span (offset 26869) (line 518) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 26869) (line 518) (column 75) (len 3)))))
    (reference r372 (scope relative) (span (offset 26873) (line 518) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 26873) (line 518) (column 79) (len 1)))))
    (reference r373 (scope relative) (span (offset 26880) (line 518) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 26880) (line 518) (column 86) (len 8)))))
    (reference r374 (scope relative) (span (offset 26934) (line 519) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 26934) (line 519) (column 39) (len 19)))))
    (reference r375 (scope relative) (span (offset 26963) (line 519) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 26963) (line 519) (column 68) (len 8)))))
    (reference r376 (scope relative) (span (offset 26974) (line 519) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 26974) (line 519) (column 79) (len 3)))))
    (reference r377 (scope relative) (span (offset 26978) (line 519) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 26978) (line 519) (column 83) (len 1)))))
    (reference r378 (scope relative) (span (offset 26985) (line 519) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 26985) (line 519) (column 90) (len 8)))))
    (reference r379 (scope relative) (span (offset 27049) (line 520) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 27049) (line 520) (column 48) (len 19)))))
    (reference r380 (scope relative) (span (offset 27078) (line 520) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 27078) (line 520) (column 77) (len 8)))))
    (reference r381 (scope relative) (span (offset 27089) (line 520) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 27089) (line 520) (column 88) (len 3)))))
    (reference r382 (scope relative) (span (offset 27093) (line 520) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 27093) (line 520) (column 92) (len 1)))))
    (reference r383 (scope relative) (span (offset 27100) (line 520) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 27100) (line 520) (column 99) (len 8)))))
    (reference r384 (scope relative) (span (offset 27139) (line 521) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 27139) (line 521) (column 23) (len 17)))))
    (reference r385 (scope relative) (span (offset 27163) (line 521) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 27163) (line 521) (column 47) (len 20)))))
    (reference r386 (scope relative) (span (offset 27187) (line 521) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 27187) (line 521) (column 71) (len 8)))))
    (reference r387 (scope relative) (span (offset 27197) (line 521) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 27197) (line 521) (column 81) (len 6)))))
    (reference r388 (scope relative) (span (offset 27205) (line 521) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 27205) (line 521) (column 89) (len 10)))))
    (reference r389 (scope relative) (span (offset 27217) (line 521) (column 101) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 27217) (line 521) (column 101) (len 19)))))
    (reference r390 (scope relative) (span (offset 27341) (line 525) (column 44) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 27341) (line 525) (column 44) (len 17)))))
    (reference r391 (scope relative) (span (offset 27988) (line 539) (column 33) (len 21)) (segments (segment 0 (token "AbsoluteActivityValue") (name "AbsoluteActivityValue") (separator none) (span (offset 27988) (line 539) (column 33) (len 21)))))
    (reference r392 (scope relative) (span (offset 28123) (line 542) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 28123) (line 542) (column 43) (len 19)))))
    (reference r393 (scope relative) (span (offset 28688) (line 555) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 28688) (line 555) (column 28) (len 4)))))
    (reference r394 (scope relative) (span (offset 28683) (line 555) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 28683) (line 555) (column 23) (len 3)))))
    (reference r395 (scope relative) (span (offset 28722) (line 556) (column 29) (len 19)) (segments (segment 0 (token "PartialPressureUnit") (name "PartialPressureUnit") (separator none) (span (offset 28722) (line 556) (column 29) (len 19)))))
    (reference r396 (scope relative) (span (offset 28716) (line 556) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 28716) (line 556) (column 23) (len 4)))))
    (reference r397 (scope relative) (span (offset 28784) (line 559) (column 32) (len 20)) (segments (segment 0 (token "PartialPressureValue") (name "PartialPressureValue") (separator none) (span (offset 28784) (line 559) (column 32) (len 20)))))
    (reference r398 (scope relative) (span (offset 28881) (line 561) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 28881) (line 561) (column 42) (len 11)))))
    (reference r399 (scope relative) (span (offset 28931) (line 562) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 28931) (line 562) (column 37) (len 19)))))
    (reference r400 (scope relative) (span (offset 28960) (line 562) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 28960) (line 562) (column 66) (len 8)))))
    (reference r401 (scope relative) (span (offset 28971) (line 562) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 28971) (line 562) (column 77) (len 3)))))
    (reference r402 (scope relative) (span (offset 28975) (line 562) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 28975) (line 562) (column 81) (len 1)))))
    (reference r403 (scope relative) (span (offset 28982) (line 562) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 28982) (line 562) (column 88) (len 8)))))
    (reference r404 (scope relative) (span (offset 29033) (line 563) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 29033) (line 563) (column 35) (len 19)))))
    (reference r405 (scope relative) (span (offset 29062) (line 563) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 29062) (line 563) (column 64) (len 8)))))
    (reference r406 (scope relative) (span (offset 29073) (line 563) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 29073) (line 563) (column 75) (len 3)))))
    (reference r407 (scope relative) (span (offset 29077) (line 563) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 29077) (line 563) (column 79) (len 1)))))
    (reference r408 (scope relative) (span (offset 29084) (line 563) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 29084) (line 563) (column 86) (len 8)))))
    (reference r409 (scope relative) (span (offset 29138) (line 564) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 29138) (line 564) (column 39) (len 19)))))
    (reference r410 (scope relative) (span (offset 29167) (line 564) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 29167) (line 564) (column 68) (len 8)))))
    (reference r411 (scope relative) (span (offset 29178) (line 564) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 29178) (line 564) (column 79) (len 3)))))
    (reference r412 (scope relative) (span (offset 29182) (line 564) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 29182) (line 564) (column 83) (len 1)))))
    (reference r413 (scope relative) (span (offset 29189) (line 564) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 29189) (line 564) (column 90) (len 8)))))
    (reference r414 (scope relative) (span (offset 29228) (line 565) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 29228) (line 565) (column 23) (len 17)))))
    (reference r415 (scope relative) (span (offset 29252) (line 565) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 29252) (line 565) (column 47) (len 20)))))
    (reference r416 (scope relative) (span (offset 29276) (line 565) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 29276) (line 565) (column 71) (len 8)))))
    (reference r417 (scope relative) (span (offset 29286) (line 565) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 29286) (line 565) (column 81) (len 6)))))
    (reference r418 (scope relative) (span (offset 29294) (line 565) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 29294) (line 565) (column 89) (len 10)))))
    (reference r419 (scope relative) (span (offset 29392) (line 569) (column 36) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 29392) (line 569) (column 36) (len 19)))))
    (reference r420 (scope relative) (span (offset 30223) (line 582) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 30223) (line 582) (column 28) (len 4)))))
    (reference r421 (scope relative) (span (offset 30218) (line 582) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 30218) (line 582) (column 23) (len 3)))))
    (reference r422 (scope relative) (span (offset 30257) (line 583) (column 29) (len 12)) (segments (segment 0 (token "FugacityUnit") (name "FugacityUnit") (separator none) (span (offset 30257) (line 583) (column 29) (len 12)))))
    (reference r423 (scope relative) (span (offset 30251) (line 583) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 30251) (line 583) (column 23) (len 4)))))
    (reference r424 (scope relative) (span (offset 30305) (line 586) (column 25) (len 13)) (segments (segment 0 (token "FugacityValue") (name "FugacityValue") (separator none) (span (offset 30305) (line 586) (column 25) (len 13)))))
    (reference r425 (scope relative) (span (offset 30388) (line 588) (column 35) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 30388) (line 588) (column 35) (len 11)))))
    (reference r426 (scope relative) (span (offset 30438) (line 589) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 30438) (line 589) (column 37) (len 19)))))
    (reference r427 (scope relative) (span (offset 30467) (line 589) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 30467) (line 589) (column 66) (len 8)))))
    (reference r428 (scope relative) (span (offset 30478) (line 589) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 30478) (line 589) (column 77) (len 3)))))
    (reference r429 (scope relative) (span (offset 30482) (line 589) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 30482) (line 589) (column 81) (len 1)))))
    (reference r430 (scope relative) (span (offset 30489) (line 589) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 30489) (line 589) (column 88) (len 8)))))
    (reference r431 (scope relative) (span (offset 30540) (line 590) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 30540) (line 590) (column 35) (len 19)))))
    (reference r432 (scope relative) (span (offset 30569) (line 590) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 30569) (line 590) (column 64) (len 8)))))
    (reference r433 (scope relative) (span (offset 30580) (line 590) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 30580) (line 590) (column 75) (len 3)))))
    (reference r434 (scope relative) (span (offset 30584) (line 590) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 30584) (line 590) (column 79) (len 1)))))
    (reference r435 (scope relative) (span (offset 30591) (line 590) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 30591) (line 590) (column 86) (len 8)))))
    (reference r436 (scope relative) (span (offset 30645) (line 591) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 30645) (line 591) (column 39) (len 19)))))
    (reference r437 (scope relative) (span (offset 30674) (line 591) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 30674) (line 591) (column 68) (len 8)))))
    (reference r438 (scope relative) (span (offset 30685) (line 591) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 30685) (line 591) (column 79) (len 3)))))
    (reference r439 (scope relative) (span (offset 30689) (line 591) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 30689) (line 591) (column 83) (len 1)))))
    (reference r440 (scope relative) (span (offset 30696) (line 591) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 30696) (line 591) (column 90) (len 8)))))
    (reference r441 (scope relative) (span (offset 30735) (line 592) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 30735) (line 592) (column 23) (len 17)))))
    (reference r442 (scope relative) (span (offset 30759) (line 592) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 30759) (line 592) (column 47) (len 20)))))
    (reference r443 (scope relative) (span (offset 30783) (line 592) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 30783) (line 592) (column 71) (len 8)))))
    (reference r444 (scope relative) (span (offset 30793) (line 592) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 30793) (line 592) (column 81) (len 6)))))
    (reference r445 (scope relative) (span (offset 30801) (line 592) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 30801) (line 592) (column 89) (len 10)))))
    (reference r446 (scope relative) (span (offset 30935) (line 596) (column 53) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 30935) (line 596) (column 53) (len 19)))))
    (reference r447 (scope relative) (span (offset 31799) (line 609) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 31799) (line 609) (column 28) (len 4)))))
    (reference r448 (scope relative) (span (offset 31794) (line 609) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 31794) (line 609) (column 23) (len 3)))))
    (reference r449 (scope relative) (span (offset 31833) (line 610) (column 29) (len 29)) (segments (segment 0 (token "StandardChemicalPotentialUnit") (name "StandardChemicalPotentialUnit") (separator none) (span (offset 31833) (line 610) (column 29) (len 29)))))
    (reference r450 (scope relative) (span (offset 31827) (line 610) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 31827) (line 610) (column 23) (len 4)))))
    (reference r451 (scope relative) (span (offset 31915) (line 613) (column 42) (len 30)) (segments (segment 0 (token "StandardChemicalPotentialValue") (name "StandardChemicalPotentialValue") (separator none) (span (offset 31915) (line 613) (column 42) (len 30)))))
    (reference r452 (scope relative) (span (offset 32032) (line 615) (column 52) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 32032) (line 615) (column 52) (len 11)))))
    (reference r453 (scope relative) (span (offset 32082) (line 616) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 32082) (line 616) (column 37) (len 19)))))
    (reference r454 (scope relative) (span (offset 32111) (line 616) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 32111) (line 616) (column 66) (len 8)))))
    (reference r455 (scope relative) (span (offset 32122) (line 616) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 32122) (line 616) (column 77) (len 3)))))
    (reference r456 (scope relative) (span (offset 32126) (line 616) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 32126) (line 616) (column 81) (len 1)))))
    (reference r457 (scope relative) (span (offset 32133) (line 616) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 32133) (line 616) (column 88) (len 8)))))
    (reference r458 (scope relative) (span (offset 32183) (line 617) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 32183) (line 617) (column 35) (len 19)))))
    (reference r459 (scope relative) (span (offset 32212) (line 617) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 32212) (line 617) (column 64) (len 8)))))
    (reference r460 (scope relative) (span (offset 32223) (line 617) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 32223) (line 617) (column 75) (len 3)))))
    (reference r461 (scope relative) (span (offset 32227) (line 617) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 32227) (line 617) (column 79) (len 1)))))
    (reference r462 (scope relative) (span (offset 32234) (line 617) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 32234) (line 617) (column 86) (len 8)))))
    (reference r463 (scope relative) (span (offset 32288) (line 618) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 32288) (line 618) (column 39) (len 19)))))
    (reference r464 (scope relative) (span (offset 32317) (line 618) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 32317) (line 618) (column 68) (len 8)))))
    (reference r465 (scope relative) (span (offset 32328) (line 618) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 32328) (line 618) (column 79) (len 3)))))
    (reference r466 (scope relative) (span (offset 32332) (line 618) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 32332) (line 618) (column 83) (len 1)))))
    (reference r467 (scope relative) (span (offset 32339) (line 618) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 32339) (line 618) (column 90) (len 8)))))
    (reference r468 (scope relative) (span (offset 32403) (line 619) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 32403) (line 619) (column 48) (len 19)))))
    (reference r469 (scope relative) (span (offset 32432) (line 619) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 32432) (line 619) (column 77) (len 8)))))
    (reference r470 (scope relative) (span (offset 32443) (line 619) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 32443) (line 619) (column 88) (len 3)))))
    (reference r471 (scope relative) (span (offset 32447) (line 619) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 32447) (line 619) (column 92) (len 1)))))
    (reference r472 (scope relative) (span (offset 32454) (line 619) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 32454) (line 619) (column 99) (len 8)))))
    (reference r473 (scope relative) (span (offset 32493) (line 620) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 32493) (line 620) (column 23) (len 17)))))
    (reference r474 (scope relative) (span (offset 32517) (line 620) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 32517) (line 620) (column 47) (len 20)))))
    (reference r475 (scope relative) (span (offset 32541) (line 620) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 32541) (line 620) (column 71) (len 8)))))
    (reference r476 (scope relative) (span (offset 32551) (line 620) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 32551) (line 620) (column 81) (len 6)))))
    (reference r477 (scope relative) (span (offset 32559) (line 620) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 32559) (line 620) (column 89) (len 10)))))
    (reference r478 (scope relative) (span (offset 32571) (line 620) (column 101) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 32571) (line 620) (column 101) (len 19)))))
    (reference r479 (scope relative) (span (offset 32691) (line 624) (column 42) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 32691) (line 624) (column 42) (len 17)))))
    (reference r480 (scope relative) (span (offset 33647) (line 638) (column 31) (len 19)) (segments (segment 0 (token "ActivityFactorValue") (name "ActivityFactorValue") (separator none) (span (offset 33647) (line 638) (column 31) (len 19)))))
    (reference r481 (scope relative) (span (offset 33819) (line 641) (column 61) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 33819) (line 641) (column 61) (len 17)))))
    (reference r482 (scope relative) (span (offset 34577) (line 655) (column 50) (len 38)) (segments (segment 0 (token "StandardAbsoluteActivityInMixtureValue") (name "StandardAbsoluteActivityInMixtureValue") (separator none) (span (offset 34577) (line 655) (column 50) (len 38)))))
    (reference r483 (scope relative) (span (offset 34761) (line 658) (column 44) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 34761) (line 658) (column 44) (len 17)))))
    (reference r484 (scope relative) (span (offset 36120) (line 672) (column 33) (len 21)) (segments (segment 0 (token "ActivityOfSoluteValue") (name "ActivityOfSoluteValue") (separator none) (span (offset 36120) (line 672) (column 33) (len 21)))))
    (reference r485 (scope relative) (span (offset 36203) (line 674) (column 40) (len 16)) (segments (segment 0 (token "activityOfSolute") (name "activityOfSolute") (separator none) (span (offset 36203) (line 674) (column 40) (len 16)))))
    (reference r486 (scope relative) (span (offset 36321) (line 677) (column 47) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 36321) (line 677) (column 47) (len 17)))))
    (reference r487 (scope relative) (span (offset 37083) (line 691) (column 36) (len 24)) (segments (segment 0 (token "ActivityCoefficientValue") (name "ActivityCoefficientValue") (separator none) (span (offset 37083) (line 691) (column 36) (len 24)))))
    (reference r488 (scope relative) (span (offset 37262) (line 694) (column 62) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 37262) (line 694) (column 62) (len 17)))))
    (reference r489 (scope relative) (span (offset 38138) (line 708) (column 51) (len 39)) (segments (segment 0 (token "StandardAbsoluteActivityInSolutionValue") (name "StandardAbsoluteActivityInSolutionValue") (separator none) (span (offset 38138) (line 708) (column 51) (len 39)))))
    (reference r490 (scope relative) (span (offset 38328) (line 711) (column 45) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 38328) (line 711) (column 45) (len 17)))))
    (reference r491 (scope relative) (span (offset 39021) (line 725) (column 34) (len 22)) (segments (segment 0 (token "ActivityOfSolventValue") (name "ActivityOfSolventValue") (separator none) (span (offset 39021) (line 725) (column 34) (len 22)))))
    (reference r492 (scope relative) (span (offset 39106) (line 727) (column 41) (len 17)) (segments (segment 0 (token "activityOfSolvent") (name "activityOfSolvent") (separator none) (span (offset 39106) (line 727) (column 41) (len 17)))))
    (reference r493 (scope relative) (span (offset 39269) (line 730) (column 50) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 39269) (line 730) (column 50) (len 17)))))
    (reference r494 (scope relative) (span (offset 40158) (line 744) (column 39) (len 27)) (segments (segment 0 (token "OsmoticFactorOfSolventValue") (name "OsmoticFactorOfSolventValue") (separator none) (span (offset 40158) (line 744) (column 39) (len 27)))))
    (reference r495 (scope relative) (span (offset 40251) (line 746) (column 44) (len 22)) (segments (segment 0 (token "osmoticFactorOfSolvent") (name "osmoticFactorOfSolvent") (separator none) (span (offset 40251) (line 746) (column 44) (len 22)))))
    (reference r496 (scope relative) (span (offset 40408) (line 749) (column 61) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 40408) (line 749) (column 61) (len 17)))))
    (reference r497 (scope relative) (span (offset 41091) (line 763) (column 50) (len 38)) (segments (segment 0 (token "StandardAbsoluteActivityOfSolventValue") (name "StandardAbsoluteActivityOfSolventValue") (separator none) (span (offset 41091) (line 763) (column 50) (len 38)))))
    (reference r498 (scope relative) (span (offset 41243) (line 766) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 41243) (line 766) (column 43) (len 19)))))
    (reference r499 (scope relative) (span (offset 41807) (line 779) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 41807) (line 779) (column 28) (len 4)))))
    (reference r500 (scope relative) (span (offset 41802) (line 779) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 41802) (line 779) (column 23) (len 3)))))
    (reference r501 (scope relative) (span (offset 41841) (line 780) (column 29) (len 19)) (segments (segment 0 (token "OsmoticPressureUnit") (name "OsmoticPressureUnit") (separator none) (span (offset 41841) (line 780) (column 29) (len 19)))))
    (reference r502 (scope relative) (span (offset 41835) (line 780) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 41835) (line 780) (column 23) (len 4)))))
    (reference r503 (scope relative) (span (offset 41903) (line 783) (column 32) (len 20)) (segments (segment 0 (token "OsmoticPressureValue") (name "OsmoticPressureValue") (separator none) (span (offset 41903) (line 783) (column 32) (len 20)))))
    (reference r504 (scope relative) (span (offset 42000) (line 785) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 42000) (line 785) (column 42) (len 11)))))
    (reference r505 (scope relative) (span (offset 42050) (line 786) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 42050) (line 786) (column 37) (len 19)))))
    (reference r506 (scope relative) (span (offset 42079) (line 786) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 42079) (line 786) (column 66) (len 8)))))
    (reference r507 (scope relative) (span (offset 42090) (line 786) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 42090) (line 786) (column 77) (len 3)))))
    (reference r508 (scope relative) (span (offset 42094) (line 786) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 42094) (line 786) (column 81) (len 1)))))
    (reference r509 (scope relative) (span (offset 42101) (line 786) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 42101) (line 786) (column 88) (len 8)))))
    (reference r510 (scope relative) (span (offset 42152) (line 787) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 42152) (line 787) (column 35) (len 19)))))
    (reference r511 (scope relative) (span (offset 42181) (line 787) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 42181) (line 787) (column 64) (len 8)))))
    (reference r512 (scope relative) (span (offset 42192) (line 787) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 42192) (line 787) (column 75) (len 3)))))
    (reference r513 (scope relative) (span (offset 42196) (line 787) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 42196) (line 787) (column 79) (len 1)))))
    (reference r514 (scope relative) (span (offset 42203) (line 787) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 42203) (line 787) (column 86) (len 8)))))
    (reference r515 (scope relative) (span (offset 42257) (line 788) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 42257) (line 788) (column 39) (len 19)))))
    (reference r516 (scope relative) (span (offset 42286) (line 788) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 42286) (line 788) (column 68) (len 8)))))
    (reference r517 (scope relative) (span (offset 42297) (line 788) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 42297) (line 788) (column 79) (len 3)))))
    (reference r518 (scope relative) (span (offset 42301) (line 788) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 42301) (line 788) (column 83) (len 1)))))
    (reference r519 (scope relative) (span (offset 42308) (line 788) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 42308) (line 788) (column 90) (len 8)))))
    (reference r520 (scope relative) (span (offset 42347) (line 789) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 42347) (line 789) (column 23) (len 17)))))
    (reference r521 (scope relative) (span (offset 42371) (line 789) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 42371) (line 789) (column 47) (len 20)))))
    (reference r522 (scope relative) (span (offset 42395) (line 789) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 42395) (line 789) (column 71) (len 8)))))
    (reference r523 (scope relative) (span (offset 42405) (line 789) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 42405) (line 789) (column 81) (len 6)))))
    (reference r524 (scope relative) (span (offset 42413) (line 789) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 42413) (line 789) (column 89) (len 10)))))
    (reference r525 (scope relative) (span (offset 42560) (line 793) (column 59) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 42560) (line 793) (column 59) (len 17)))))
    (reference r526 (scope relative) (span (offset 43395) (line 807) (column 48) (len 36)) (segments (segment 0 (token "StoichiometricNumberOfSubstanceValue") (name "StoichiometricNumberOfSubstanceValue") (separator none) (span (offset 43395) (line 807) (column 48) (len 36)))))
    (reference r527 (scope relative) (span (offset 43572) (line 810) (column 55) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 43572) (line 810) (column 55) (len 19)))))
    (reference r528 (scope relative) (span (offset 44666) (line 823) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 44666) (line 823) (column 28) (len 4)))))
    (reference r529 (scope relative) (span (offset 44661) (line 823) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 44661) (line 823) (column 23) (len 3)))))
    (reference r530 (scope relative) (span (offset 44700) (line 824) (column 29) (len 31)) (segments (segment 0 (token "AffinityOfAChemicalReactionUnit") (name "AffinityOfAChemicalReactionUnit") (separator none) (span (offset 44700) (line 824) (column 29) (len 31)))))
    (reference r531 (scope relative) (span (offset 44694) (line 824) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 44694) (line 824) (column 23) (len 4)))))
    (reference r532 (scope relative) (span (offset 44786) (line 827) (column 44) (len 32)) (segments (segment 0 (token "AffinityOfAChemicalReactionValue") (name "AffinityOfAChemicalReactionValue") (separator none) (span (offset 44786) (line 827) (column 44) (len 32)))))
    (reference r533 (scope relative) (span (offset 44907) (line 829) (column 54) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 44907) (line 829) (column 54) (len 11)))))
    (reference r534 (scope relative) (span (offset 44957) (line 830) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 44957) (line 830) (column 37) (len 19)))))
    (reference r535 (scope relative) (span (offset 44986) (line 830) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 44986) (line 830) (column 66) (len 8)))))
    (reference r536 (scope relative) (span (offset 44997) (line 830) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 44997) (line 830) (column 77) (len 3)))))
    (reference r537 (scope relative) (span (offset 45001) (line 830) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 45001) (line 830) (column 81) (len 1)))))
    (reference r538 (scope relative) (span (offset 45008) (line 830) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 45008) (line 830) (column 88) (len 8)))))
    (reference r539 (scope relative) (span (offset 45058) (line 831) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 45058) (line 831) (column 35) (len 19)))))
    (reference r540 (scope relative) (span (offset 45087) (line 831) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 45087) (line 831) (column 64) (len 8)))))
    (reference r541 (scope relative) (span (offset 45098) (line 831) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 45098) (line 831) (column 75) (len 3)))))
    (reference r542 (scope relative) (span (offset 45102) (line 831) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 45102) (line 831) (column 79) (len 1)))))
    (reference r543 (scope relative) (span (offset 45109) (line 831) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 45109) (line 831) (column 86) (len 8)))))
    (reference r544 (scope relative) (span (offset 45163) (line 832) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 45163) (line 832) (column 39) (len 19)))))
    (reference r545 (scope relative) (span (offset 45192) (line 832) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 45192) (line 832) (column 68) (len 8)))))
    (reference r546 (scope relative) (span (offset 45203) (line 832) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 45203) (line 832) (column 79) (len 3)))))
    (reference r547 (scope relative) (span (offset 45207) (line 832) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 45207) (line 832) (column 83) (len 1)))))
    (reference r548 (scope relative) (span (offset 45214) (line 832) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 45214) (line 832) (column 90) (len 8)))))
    (reference r549 (scope relative) (span (offset 45278) (line 833) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 45278) (line 833) (column 48) (len 19)))))
    (reference r550 (scope relative) (span (offset 45307) (line 833) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 45307) (line 833) (column 77) (len 8)))))
    (reference r551 (scope relative) (span (offset 45318) (line 833) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 45318) (line 833) (column 88) (len 3)))))
    (reference r552 (scope relative) (span (offset 45322) (line 833) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 45322) (line 833) (column 92) (len 1)))))
    (reference r553 (scope relative) (span (offset 45329) (line 833) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 45329) (line 833) (column 99) (len 8)))))
    (reference r554 (scope relative) (span (offset 45368) (line 834) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 45368) (line 834) (column 23) (len 17)))))
    (reference r555 (scope relative) (span (offset 45392) (line 834) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 45392) (line 834) (column 47) (len 20)))))
    (reference r556 (scope relative) (span (offset 45416) (line 834) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 45416) (line 834) (column 71) (len 8)))))
    (reference r557 (scope relative) (span (offset 45426) (line 834) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 45426) (line 834) (column 81) (len 6)))))
    (reference r558 (scope relative) (span (offset 45434) (line 834) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 45434) (line 834) (column 89) (len 10)))))
    (reference r559 (scope relative) (span (offset 45446) (line 834) (column 101) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 45446) (line 834) (column 101) (len 19)))))
    (reference r560 (scope relative) (span (offset 45560) (line 838) (column 33) (len 22)) (segments (segment 0 (token "AmountOfSubstanceValue") (name "AmountOfSubstanceValue") (separator none) (span (offset 45560) (line 838) (column 33) (len 22)))))
    (reference r561 (scope relative) (span (offset 46387) (line 854) (column 55) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 46387) (line 854) (column 55) (len 17)))))
    (reference r562 (scope relative) (span (offset 47328) (line 868) (column 44) (len 32)) (segments (segment 0 (token "StandardEquilibriumConstantValue") (name "StandardEquilibriumConstantValue") (separator none) (span (offset 47328) (line 868) (column 44) (len 32)))))
    (reference r563 (scope relative) (span (offset 47430) (line 870) (column 48) (len 27)) (segments (segment 0 (token "standardEquilibriumConstant") (name "standardEquilibriumConstant") (separator none) (span (offset 47430) (line 870) (column 48) (len 27)))))
    (reference r564 (scope relative) (span (offset 47592) (line 873) (column 62) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 47592) (line 873) (column 62) (len 19)))))
    (reference r565 (scope relative) (span (offset 48223) (line 886) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 48223) (line 886) (column 28) (len 4)))))
    (reference r566 (scope relative) (span (offset 48218) (line 886) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 48218) (line 886) (column 23) (len 3)))))
    (reference r567 (scope relative) (span (offset 48257) (line 887) (column 29) (len 38)) (segments (segment 0 (token "EquilibriumConstantOnPressureBasisUnit") (name "EquilibriumConstantOnPressureBasisUnit") (separator none) (span (offset 48257) (line 887) (column 29) (len 38)))))
    (reference r568 (scope relative) (span (offset 48251) (line 887) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 48251) (line 887) (column 23) (len 4)))))
    (reference r569 (scope relative) (span (offset 48357) (line 890) (column 51) (len 39)) (segments (segment 0 (token "EquilibriumConstantOnPressureBasisValue") (name "EquilibriumConstantOnPressureBasisValue") (separator none) (span (offset 48357) (line 890) (column 51) (len 39)))))
    (reference r570 (scope relative) (span (offset 48492) (line 892) (column 61) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 48492) (line 892) (column 61) (len 11)))))
    (reference r571 (scope relative) (span (offset 48542) (line 893) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 48542) (line 893) (column 37) (len 19)))))
    (reference r572 (scope relative) (span (offset 48571) (line 893) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 48571) (line 893) (column 66) (len 8)))))
    (reference r573 (scope relative) (span (offset 48582) (line 893) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 48582) (line 893) (column 77) (len 3)))))
    (reference r574 (scope relative) (span (offset 48586) (line 893) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 48586) (line 893) (column 81) (len 1)))))
    (reference r575 (scope relative) (span (offset 48593) (line 893) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 48593) (line 893) (column 88) (len 8)))))
    (reference r576 (scope relative) (span (offset 48644) (line 894) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 48644) (line 894) (column 35) (len 19)))))
    (reference r577 (scope relative) (span (offset 48673) (line 894) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 48673) (line 894) (column 64) (len 8)))))
    (reference r578 (scope relative) (span (offset 48684) (line 894) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 48684) (line 894) (column 75) (len 3)))))
    (reference r579 (scope relative) (span (offset 48688) (line 894) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 48688) (line 894) (column 79) (len 1)))))
    (reference r580 (scope relative) (span (offset 48695) (line 894) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 48695) (line 894) (column 86) (len 8)))))
    (reference r581 (scope relative) (span (offset 48749) (line 895) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 48749) (line 895) (column 39) (len 19)))))
    (reference r582 (scope relative) (span (offset 48778) (line 895) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 48778) (line 895) (column 68) (len 8)))))
    (reference r583 (scope relative) (span (offset 48789) (line 895) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 48789) (line 895) (column 79) (len 3)))))
    (reference r584 (scope relative) (span (offset 48793) (line 895) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 48793) (line 895) (column 83) (len 1)))))
    (reference r585 (scope relative) (span (offset 48800) (line 895) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 48800) (line 895) (column 90) (len 8)))))
    (reference r586 (scope relative) (span (offset 48839) (line 896) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 48839) (line 896) (column 23) (len 17)))))
    (reference r587 (scope relative) (span (offset 48863) (line 896) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 48863) (line 896) (column 47) (len 20)))))
    (reference r588 (scope relative) (span (offset 48887) (line 896) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 48887) (line 896) (column 71) (len 8)))))
    (reference r589 (scope relative) (span (offset 48897) (line 896) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 48897) (line 896) (column 81) (len 6)))))
    (reference r590 (scope relative) (span (offset 48905) (line 896) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 48905) (line 896) (column 89) (len 10)))))
    (reference r591 (scope relative) (span (offset 49069) (line 900) (column 67) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 49069) (line 900) (column 67) (len 19)))))
    (reference r592 (scope relative) (span (offset 49703) (line 913) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 49703) (line 913) (column 28) (len 4)))))
    (reference r593 (scope relative) (span (offset 49698) (line 913) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 49698) (line 913) (column 23) (len 3)))))
    (reference r594 (scope relative) (span (offset 49737) (line 914) (column 29) (len 43)) (segments (segment 0 (token "EquilibriumConstantOnConcentrationBasisUnit") (name "EquilibriumConstantOnConcentrationBasisUnit") (separator none) (span (offset 49737) (line 914) (column 29) (len 43)))))
    (reference r595 (scope relative) (span (offset 49731) (line 914) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 49731) (line 914) (column 23) (len 4)))))
    (reference r596 (scope relative) (span (offset 49847) (line 917) (column 56) (len 44)) (segments (segment 0 (token "EquilibriumConstantOnConcentrationBasisValue") (name "EquilibriumConstantOnConcentrationBasisValue") (separator none) (span (offset 49847) (line 917) (column 56) (len 44)))))
    (reference r597 (scope relative) (span (offset 49992) (line 919) (column 66) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 49992) (line 919) (column 66) (len 11)))))
    (reference r598 (scope relative) (span (offset 50042) (line 920) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 50042) (line 920) (column 37) (len 19)))))
    (reference r599 (scope relative) (span (offset 50071) (line 920) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 50071) (line 920) (column 66) (len 8)))))
    (reference r600 (scope relative) (span (offset 50082) (line 920) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 50082) (line 920) (column 77) (len 3)))))
    (reference r601 (scope relative) (span (offset 50086) (line 920) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 50086) (line 920) (column 81) (len 1)))))
    (reference r602 (scope relative) (span (offset 50093) (line 920) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 50093) (line 920) (column 88) (len 8)))))
    (reference r603 (scope relative) (span (offset 50157) (line 921) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 50157) (line 921) (column 48) (len 19)))))
    (reference r604 (scope relative) (span (offset 50186) (line 921) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 50186) (line 921) (column 77) (len 8)))))
    (reference r605 (scope relative) (span (offset 50197) (line 921) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 50197) (line 921) (column 88) (len 3)))))
    (reference r606 (scope relative) (span (offset 50201) (line 921) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 50201) (line 921) (column 92) (len 1)))))
    (reference r607 (scope relative) (span (offset 50208) (line 921) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 50208) (line 921) (column 99) (len 8)))))
    (reference r608 (scope relative) (span (offset 50246) (line 922) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 50246) (line 922) (column 23) (len 17)))))
    (reference r609 (scope relative) (span (offset 50270) (line 922) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 50270) (line 922) (column 47) (len 20)))))
    (reference r610 (scope relative) (span (offset 50294) (line 922) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 50294) (line 922) (column 71) (len 8)))))
    (reference r611 (scope relative) (span (offset 50304) (line 922) (column 81) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 50304) (line 922) (column 81) (len 19)))))
    (reference r612 (scope relative) (span (offset 50450) (line 926) (column 48) (len 10)) (segments (segment 0 (token "CountValue") (name "CountValue") (separator none) (span (offset 50450) (line 926) (column 48) (len 10)))))
    (reference r613 (scope relative) (span (offset 51216) (line 942) (column 54) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 51216) (line 942) (column 54) (len 17)))))
    (reference r614 (scope relative) (span (offset 51919) (line 956) (column 43) (len 31)) (segments (segment 0 (token "CanonicalPartitionFunctionValue") (name "CanonicalPartitionFunctionValue") (separator none) (span (offset 51919) (line 956) (column 43) (len 31)))))
    (reference r615 (scope relative) (span (offset 52126) (line 959) (column 59) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 52126) (line 959) (column 59) (len 17)))))
    (reference r616 (scope relative) (span (offset 53037) (line 973) (column 48) (len 36)) (segments (segment 0 (token "GrandCanonicalPartitionFunctionValue") (name "GrandCanonicalPartitionFunctionValue") (separator none) (span (offset 53037) (line 973) (column 48) (len 36)))))
    (reference r617 (scope relative) (span (offset 53133) (line 975) (column 38) (len 31)) (segments (segment 0 (token "grandCanonicalPartitionFunction") (name "grandCanonicalPartitionFunction") (separator none) (span (offset 53133) (line 975) (column 38) (len 31)))))
    (reference r618 (scope relative) (span (offset 53317) (line 978) (column 54) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 53317) (line 978) (column 54) (len 17)))))
    (reference r619 (scope relative) (span (offset 54085) (line 992) (column 43) (len 31)) (segments (segment 0 (token "MolecularPartitionFunctionValue") (name "MolecularPartitionFunctionValue") (separator none) (span (offset 54085) (line 992) (column 43) (len 31)))))
    (reference r620 (scope relative) (span (offset 54182) (line 994) (column 44) (len 26)) (segments (segment 0 (token "molecularPartitionFunction") (name "molecularPartitionFunction") (separator none) (span (offset 54182) (line 994) (column 44) (len 26)))))
    (reference r621 (scope relative) (span (offset 54321) (line 997) (column 45) (len 10)) (segments (segment 0 (token "CountValue") (name "CountValue") (separator none) (span (offset 54321) (line 997) (column 45) (len 10)))))
    (reference r622 (scope relative) (span (offset 54876) (line 1013) (column 38) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 54876) (line 1013) (column 38) (len 17)))))
    (reference r623 (scope relative) (span (offset 55384) (line 1027) (column 27) (len 15)) (segments (segment 0 (token "DegeneracyValue") (name "DegeneracyValue") (separator none) (span (offset 55384) (line 1027) (column 27) (len 15)))))
    (reference r624 (scope relative) (span (offset 55449) (line 1029) (column 28) (len 10)) (segments (segment 0 (token "degeneracy") (name "degeneracy") (separator none) (span (offset 55449) (line 1029) (column 28) (len 10)))))
    (reference r625 (scope relative) (span (offset 55558) (line 1032) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 55558) (line 1032) (column 44) (len 19)))))
    (reference r626 (scope relative) (span (offset 56101) (line 1045) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 56101) (line 1045) (column 28) (len 4)))))
    (reference r627 (scope relative) (span (offset 56096) (line 1045) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 56096) (line 1045) (column 23) (len 3)))))
    (reference r628 (scope relative) (span (offset 56135) (line 1046) (column 29) (len 20)) (segments (segment 0 (token "MolarGasConstantUnit") (name "MolarGasConstantUnit") (separator none) (span (offset 56135) (line 1046) (column 29) (len 20)))))
    (reference r629 (scope relative) (span (offset 56129) (line 1046) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 56129) (line 1046) (column 23) (len 4)))))
    (reference r630 (scope relative) (span (offset 56199) (line 1049) (column 33) (len 21)) (segments (segment 0 (token "MolarGasConstantValue") (name "MolarGasConstantValue") (separator none) (span (offset 56199) (line 1049) (column 33) (len 21)))))
    (reference r631 (scope relative) (span (offset 56298) (line 1051) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 56298) (line 1051) (column 43) (len 11)))))
    (reference r632 (scope relative) (span (offset 56348) (line 1052) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 56348) (line 1052) (column 37) (len 19)))))
    (reference r633 (scope relative) (span (offset 56377) (line 1052) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 56377) (line 1052) (column 66) (len 8)))))
    (reference r634 (scope relative) (span (offset 56388) (line 1052) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 56388) (line 1052) (column 77) (len 3)))))
    (reference r635 (scope relative) (span (offset 56392) (line 1052) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 56392) (line 1052) (column 81) (len 1)))))
    (reference r636 (scope relative) (span (offset 56399) (line 1052) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 56399) (line 1052) (column 88) (len 8)))))
    (reference r637 (scope relative) (span (offset 56449) (line 1053) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 56449) (line 1053) (column 35) (len 19)))))
    (reference r638 (scope relative) (span (offset 56478) (line 1053) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 56478) (line 1053) (column 64) (len 8)))))
    (reference r639 (scope relative) (span (offset 56489) (line 1053) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 56489) (line 1053) (column 75) (len 3)))))
    (reference r640 (scope relative) (span (offset 56493) (line 1053) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 56493) (line 1053) (column 79) (len 1)))))
    (reference r641 (scope relative) (span (offset 56500) (line 1053) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 56500) (line 1053) (column 86) (len 8)))))
    (reference r642 (scope relative) (span (offset 56554) (line 1054) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 56554) (line 1054) (column 39) (len 19)))))
    (reference r643 (scope relative) (span (offset 56583) (line 1054) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 56583) (line 1054) (column 68) (len 8)))))
    (reference r644 (scope relative) (span (offset 56594) (line 1054) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 56594) (line 1054) (column 79) (len 3)))))
    (reference r645 (scope relative) (span (offset 56598) (line 1054) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 56598) (line 1054) (column 83) (len 1)))))
    (reference r646 (scope relative) (span (offset 56605) (line 1054) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 56605) (line 1054) (column 90) (len 8)))))
    (reference r647 (scope relative) (span (offset 56676) (line 1055) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 56676) (line 1055) (column 55) (len 19)))))
    (reference r648 (scope relative) (span (offset 56705) (line 1055) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 56705) (line 1055) (column 84) (len 8)))))
    (reference r649 (scope relative) (span (offset 56716) (line 1055) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 56716) (line 1055) (column 95) (len 3)))))
    (reference r650 (scope relative) (span (offset 56720) (line 1055) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 56720) (line 1055) (column 99) (len 4)))))
    (reference r651 (scope relative) (span (offset 56730) (line 1055) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 56730) (line 1055) (column 109) (len 8)))))
    (reference r652 (scope relative) (span (offset 56794) (line 1056) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 56794) (line 1056) (column 48) (len 19)))))
    (reference r653 (scope relative) (span (offset 56823) (line 1056) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 56823) (line 1056) (column 77) (len 8)))))
    (reference r654 (scope relative) (span (offset 56834) (line 1056) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 56834) (line 1056) (column 88) (len 3)))))
    (reference r655 (scope relative) (span (offset 56838) (line 1056) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 56838) (line 1056) (column 92) (len 1)))))
    (reference r656 (scope relative) (span (offset 56845) (line 1056) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 56845) (line 1056) (column 99) (len 8)))))
    (reference r657 (scope relative) (span (offset 56884) (line 1057) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 56884) (line 1057) (column 23) (len 17)))))
    (reference r658 (scope relative) (span (offset 56908) (line 1057) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 56908) (line 1057) (column 47) (len 20)))))
    (reference r659 (scope relative) (span (offset 56932) (line 1057) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 56932) (line 1057) (column 71) (len 8)))))
    (reference r660 (scope relative) (span (offset 56942) (line 1057) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 56942) (line 1057) (column 81) (len 6)))))
    (reference r661 (scope relative) (span (offset 56950) (line 1057) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 56950) (line 1057) (column 89) (len 10)))))
    (reference r662 (scope relative) (span (offset 56962) (line 1057) (column 101) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 56962) (line 1057) (column 101) (len 26)))))
    (reference r663 (scope relative) (span (offset 56990) (line 1057) (column 129) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 56990) (line 1057) (column 129) (len 19)))))
    (reference r664 (scope relative) (span (offset 57261) (line 1064) (column 29) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 57261) (line 1064) (column 29) (len 11)))))
    (reference r665 (scope relative) (span (offset 57861) (line 1080) (column 48) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 57861) (line 1080) (column 48) (len 19)))))
    (reference r666 (scope relative) (span (offset 58607) (line 1093) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 58607) (line 1093) (column 28) (len 4)))))
    (reference r667 (scope relative) (span (offset 58602) (line 1093) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 58602) (line 1093) (column 23) (len 3)))))
    (reference r668 (scope relative) (span (offset 58641) (line 1094) (column 29) (len 24)) (segments (segment 0 (token "DiffusionCoefficientUnit") (name "DiffusionCoefficientUnit") (separator none) (span (offset 58641) (line 1094) (column 29) (len 24)))))
    (reference r669 (scope relative) (span (offset 58635) (line 1094) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 58635) (line 1094) (column 23) (len 4)))))
    (reference r670 (scope relative) (span (offset 58713) (line 1097) (column 37) (len 25)) (segments (segment 0 (token "DiffusionCoefficientValue") (name "DiffusionCoefficientValue") (separator none) (span (offset 58713) (line 1097) (column 37) (len 25)))))
    (reference r671 (scope relative) (span (offset 58820) (line 1099) (column 47) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 58820) (line 1099) (column 47) (len 11)))))
    (reference r672 (scope relative) (span (offset 58870) (line 1100) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 58870) (line 1100) (column 37) (len 19)))))
    (reference r673 (scope relative) (span (offset 58899) (line 1100) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 58899) (line 1100) (column 66) (len 8)))))
    (reference r674 (scope relative) (span (offset 58910) (line 1100) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 58910) (line 1100) (column 77) (len 3)))))
    (reference r675 (scope relative) (span (offset 58914) (line 1100) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 58914) (line 1100) (column 81) (len 1)))))
    (reference r676 (scope relative) (span (offset 58921) (line 1100) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 58921) (line 1100) (column 88) (len 8)))))
    (reference r677 (scope relative) (span (offset 58975) (line 1101) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 58975) (line 1101) (column 39) (len 19)))))
    (reference r678 (scope relative) (span (offset 59004) (line 1101) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 59004) (line 1101) (column 68) (len 8)))))
    (reference r679 (scope relative) (span (offset 59015) (line 1101) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 59015) (line 1101) (column 79) (len 3)))))
    (reference r680 (scope relative) (span (offset 59019) (line 1101) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 59019) (line 1101) (column 83) (len 1)))))
    (reference r681 (scope relative) (span (offset 59026) (line 1101) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 59026) (line 1101) (column 90) (len 8)))))
    (reference r682 (scope relative) (span (offset 59065) (line 1102) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 59065) (line 1102) (column 23) (len 17)))))
    (reference r683 (scope relative) (span (offset 59089) (line 1102) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 59089) (line 1102) (column 47) (len 20)))))
    (reference r684 (scope relative) (span (offset 59113) (line 1102) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 59113) (line 1102) (column 71) (len 8)))))
    (reference r685 (scope relative) (span (offset 59123) (line 1102) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 59123) (line 1102) (column 81) (len 10)))))
    (reference r686 (scope relative) (span (offset 59251) (line 1106) (column 49) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 59251) (line 1106) (column 49) (len 17)))))
    (reference r687 (scope relative) (span (offset 60037) (line 1120) (column 38) (len 26)) (segments (segment 0 (token "ThermalDiffusionRatioValue") (name "ThermalDiffusionRatioValue") (separator none) (span (offset 60037) (line 1120) (column 38) (len 26)))))
    (reference r688 (scope relative) (span (offset 60194) (line 1123) (column 50) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 60194) (line 1123) (column 50) (len 17)))))
    (reference r689 (scope relative) (span (offset 60835) (line 1137) (column 39) (len 27)) (segments (segment 0 (token "ThermalDiffusionFactorValue") (name "ThermalDiffusionFactorValue") (separator none) (span (offset 60835) (line 1137) (column 39) (len 27)))))
    (reference r690 (scope relative) (span (offset 61001) (line 1140) (column 55) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 61001) (line 1140) (column 55) (len 19)))))
    (reference r691 (scope relative) (span (offset 61533) (line 1153) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 61533) (line 1153) (column 28) (len 4)))))
    (reference r692 (scope relative) (span (offset 61528) (line 1153) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 61528) (line 1153) (column 23) (len 3)))))
    (reference r693 (scope relative) (span (offset 61567) (line 1154) (column 29) (len 31)) (segments (segment 0 (token "ThermalDiffusionCoefficientUnit") (name "ThermalDiffusionCoefficientUnit") (separator none) (span (offset 61567) (line 1154) (column 29) (len 31)))))
    (reference r694 (scope relative) (span (offset 61561) (line 1154) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 61561) (line 1154) (column 23) (len 4)))))
    (reference r695 (scope relative) (span (offset 61653) (line 1157) (column 44) (len 32)) (segments (segment 0 (token "ThermalDiffusionCoefficientValue") (name "ThermalDiffusionCoefficientValue") (separator none) (span (offset 61653) (line 1157) (column 44) (len 32)))))
    (reference r696 (scope relative) (span (offset 61774) (line 1159) (column 54) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 61774) (line 1159) (column 54) (len 11)))))
    (reference r697 (scope relative) (span (offset 61824) (line 1160) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 61824) (line 1160) (column 37) (len 19)))))
    (reference r698 (scope relative) (span (offset 61853) (line 1160) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 61853) (line 1160) (column 66) (len 8)))))
    (reference r699 (scope relative) (span (offset 61864) (line 1160) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 61864) (line 1160) (column 77) (len 3)))))
    (reference r700 (scope relative) (span (offset 61868) (line 1160) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 61868) (line 1160) (column 81) (len 1)))))
    (reference r701 (scope relative) (span (offset 61875) (line 1160) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 61875) (line 1160) (column 88) (len 8)))))
    (reference r702 (scope relative) (span (offset 61929) (line 1161) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 61929) (line 1161) (column 39) (len 19)))))
    (reference r703 (scope relative) (span (offset 61958) (line 1161) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 61958) (line 1161) (column 68) (len 8)))))
    (reference r704 (scope relative) (span (offset 61969) (line 1161) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 61969) (line 1161) (column 79) (len 3)))))
    (reference r705 (scope relative) (span (offset 61973) (line 1161) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 61973) (line 1161) (column 83) (len 1)))))
    (reference r706 (scope relative) (span (offset 61980) (line 1161) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 61980) (line 1161) (column 90) (len 8)))))
    (reference r707 (scope relative) (span (offset 62019) (line 1162) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 62019) (line 1162) (column 23) (len 17)))))
    (reference r708 (scope relative) (span (offset 62043) (line 1162) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 62043) (line 1162) (column 47) (len 20)))))
    (reference r709 (scope relative) (span (offset 62067) (line 1162) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 62067) (line 1162) (column 71) (len 8)))))
    (reference r710 (scope relative) (span (offset 62077) (line 1162) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 62077) (line 1162) (column 81) (len 10)))))
    (reference r711 (scope relative) (span (offset 62186) (line 1166) (column 41) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 62186) (line 1166) (column 41) (len 19)))))
    (reference r712 (scope relative) (span (offset 62759) (line 1179) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 62759) (line 1179) (column 28) (len 4)))))
    (reference r713 (scope relative) (span (offset 62754) (line 1179) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 62754) (line 1179) (column 23) (len 3)))))
    (reference r714 (scope relative) (span (offset 62793) (line 1180) (column 29) (len 17)) (segments (segment 0 (token "IonicStrengthUnit") (name "IonicStrengthUnit") (separator none) (span (offset 62793) (line 1180) (column 29) (len 17)))))
    (reference r715 (scope relative) (span (offset 62787) (line 1180) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 62787) (line 1180) (column 23) (len 4)))))
    (reference r716 (scope relative) (span (offset 62851) (line 1183) (column 30) (len 18)) (segments (segment 0 (token "IonicStrengthValue") (name "IonicStrengthValue") (separator none) (span (offset 62851) (line 1183) (column 30) (len 18)))))
    (reference r717 (scope relative) (span (offset 62944) (line 1185) (column 40) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 62944) (line 1185) (column 40) (len 11)))))
    (reference r718 (scope relative) (span (offset 62992) (line 1186) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 62992) (line 1186) (column 35) (len 19)))))
    (reference r719 (scope relative) (span (offset 63021) (line 1186) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 63021) (line 1186) (column 64) (len 8)))))
    (reference r720 (scope relative) (span (offset 63032) (line 1186) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 63032) (line 1186) (column 75) (len 3)))))
    (reference r721 (scope relative) (span (offset 63036) (line 1186) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 63036) (line 1186) (column 79) (len 1)))))
    (reference r722 (scope relative) (span (offset 63043) (line 1186) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 63043) (line 1186) (column 86) (len 8)))))
    (reference r723 (scope relative) (span (offset 63107) (line 1187) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 63107) (line 1187) (column 48) (len 19)))))
    (reference r724 (scope relative) (span (offset 63136) (line 1187) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 63136) (line 1187) (column 77) (len 8)))))
    (reference r725 (scope relative) (span (offset 63147) (line 1187) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 63147) (line 1187) (column 88) (len 3)))))
    (reference r726 (scope relative) (span (offset 63151) (line 1187) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 63151) (line 1187) (column 92) (len 1)))))
    (reference r727 (scope relative) (span (offset 63158) (line 1187) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 63158) (line 1187) (column 99) (len 8)))))
    (reference r728 (scope relative) (span (offset 63196) (line 1188) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 63196) (line 1188) (column 23) (len 17)))))
    (reference r729 (scope relative) (span (offset 63220) (line 1188) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 63220) (line 1188) (column 47) (len 20)))))
    (reference r730 (scope relative) (span (offset 63244) (line 1188) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 63244) (line 1188) (column 71) (len 6)))))
    (reference r731 (scope relative) (span (offset 63252) (line 1188) (column 79) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 63252) (line 1188) (column 79) (len 19)))))
    (reference r732 (scope relative) (span (offset 63408) (line 1192) (column 48) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 63408) (line 1192) (column 48) (len 17)))))
    (reference r733 (scope relative) (span (offset 63983) (line 1206) (column 37) (len 25)) (segments (segment 0 (token "DegreeOfDissociationValue") (name "DegreeOfDissociationValue") (separator none) (span (offset 63983) (line 1206) (column 37) (len 25)))))
    (reference r734 (scope relative) (span (offset 64066) (line 1208) (column 36) (len 20)) (segments (segment 0 (token "degreeOfDissociation") (name "degreeOfDissociation") (separator none) (span (offset 64066) (line 1208) (column 36) (len 20)))))
    (reference r735 (scope relative) (span (offset 64198) (line 1211) (column 52) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 64198) (line 1211) (column 52) (len 19)))))
    (reference r736 (scope relative) (span (offset 64871) (line 1224) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 64871) (line 1224) (column 28) (len 4)))))
    (reference r737 (scope relative) (span (offset 64866) (line 1224) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 64866) (line 1224) (column 23) (len 3)))))
    (reference r738 (scope relative) (span (offset 64905) (line 1225) (column 29) (len 28)) (segments (segment 0 (token "ElectrolyticConductivityUnit") (name "ElectrolyticConductivityUnit") (separator none) (span (offset 64905) (line 1225) (column 29) (len 28)))))
    (reference r739 (scope relative) (span (offset 64899) (line 1225) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 64899) (line 1225) (column 23) (len 4)))))
    (reference r740 (scope relative) (span (offset 64985) (line 1228) (column 41) (len 29)) (segments (segment 0 (token "ElectrolyticConductivityValue") (name "ElectrolyticConductivityValue") (separator none) (span (offset 64985) (line 1228) (column 41) (len 29)))))
    (reference r741 (scope relative) (span (offset 65100) (line 1230) (column 51) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 65100) (line 1230) (column 51) (len 11)))))
    (reference r742 (scope relative) (span (offset 65150) (line 1231) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 65150) (line 1231) (column 37) (len 19)))))
    (reference r743 (scope relative) (span (offset 65179) (line 1231) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 65179) (line 1231) (column 66) (len 8)))))
    (reference r744 (scope relative) (span (offset 65190) (line 1231) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 65190) (line 1231) (column 77) (len 3)))))
    (reference r745 (scope relative) (span (offset 65194) (line 1231) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 65194) (line 1231) (column 81) (len 1)))))
    (reference r746 (scope relative) (span (offset 65201) (line 1231) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 65201) (line 1231) (column 88) (len 8)))))
    (reference r747 (scope relative) (span (offset 65252) (line 1232) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 65252) (line 1232) (column 35) (len 19)))))
    (reference r748 (scope relative) (span (offset 65281) (line 1232) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 65281) (line 1232) (column 64) (len 8)))))
    (reference r749 (scope relative) (span (offset 65292) (line 1232) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 65292) (line 1232) (column 75) (len 3)))))
    (reference r750 (scope relative) (span (offset 65296) (line 1232) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 65296) (line 1232) (column 79) (len 1)))))
    (reference r751 (scope relative) (span (offset 65303) (line 1232) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 65303) (line 1232) (column 86) (len 8)))))
    (reference r752 (scope relative) (span (offset 65358) (line 1233) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 65358) (line 1233) (column 39) (len 19)))))
    (reference r753 (scope relative) (span (offset 65387) (line 1233) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 65387) (line 1233) (column 68) (len 8)))))
    (reference r754 (scope relative) (span (offset 65398) (line 1233) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 65398) (line 1233) (column 79) (len 3)))))
    (reference r755 (scope relative) (span (offset 65402) (line 1233) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 65402) (line 1233) (column 83) (len 1)))))
    (reference r756 (scope relative) (span (offset 65409) (line 1233) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 65409) (line 1233) (column 90) (len 8)))))
    (reference r757 (scope relative) (span (offset 65470) (line 1234) (column 46) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 65470) (line 1234) (column 46) (len 19)))))
    (reference r758 (scope relative) (span (offset 65499) (line 1234) (column 75) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 65499) (line 1234) (column 75) (len 8)))))
    (reference r759 (scope relative) (span (offset 65510) (line 1234) (column 86) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 65510) (line 1234) (column 86) (len 3)))))
    (reference r760 (scope relative) (span (offset 65514) (line 1234) (column 90) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 65514) (line 1234) (column 90) (len 1)))))
    (reference r761 (scope relative) (span (offset 65521) (line 1234) (column 97) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 65521) (line 1234) (column 97) (len 8)))))
    (reference r762 (scope relative) (span (offset 65559) (line 1235) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 65559) (line 1235) (column 23) (len 17)))))
    (reference r763 (scope relative) (span (offset 65583) (line 1235) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 65583) (line 1235) (column 47) (len 20)))))
    (reference r764 (scope relative) (span (offset 65607) (line 1235) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 65607) (line 1235) (column 71) (len 8)))))
    (reference r765 (scope relative) (span (offset 65617) (line 1235) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 65617) (line 1235) (column 81) (len 6)))))
    (reference r766 (scope relative) (span (offset 65625) (line 1235) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 65625) (line 1235) (column 89) (len 10)))))
    (reference r767 (scope relative) (span (offset 65637) (line 1235) (column 101) (len 17)) (segments (segment 0 (token "electricCurrentPF") (name "electricCurrentPF") (separator none) (span (offset 65637) (line 1235) (column 101) (len 17)))))
    (reference r768 (scope relative) (span (offset 65761) (line 1239) (column 45) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 65761) (line 1239) (column 45) (len 19)))))
    (reference r769 (scope relative) (span (offset 66334) (line 1252) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 66334) (line 1252) (column 28) (len 4)))))
    (reference r770 (scope relative) (span (offset 66329) (line 1252) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 66329) (line 1252) (column 23) (len 3)))))
    (reference r771 (scope relative) (span (offset 66368) (line 1253) (column 29) (len 21)) (segments (segment 0 (token "MolarConductivityUnit") (name "MolarConductivityUnit") (separator none) (span (offset 66368) (line 1253) (column 29) (len 21)))))
    (reference r772 (scope relative) (span (offset 66362) (line 1253) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 66362) (line 1253) (column 23) (len 4)))))
    (reference r773 (scope relative) (span (offset 66434) (line 1256) (column 34) (len 22)) (segments (segment 0 (token "MolarConductivityValue") (name "MolarConductivityValue") (separator none) (span (offset 66434) (line 1256) (column 34) (len 22)))))
    (reference r774 (scope relative) (span (offset 66535) (line 1258) (column 44) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 66535) (line 1258) (column 44) (len 11)))))
    (reference r775 (scope relative) (span (offset 66583) (line 1259) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 66583) (line 1259) (column 35) (len 19)))))
    (reference r776 (scope relative) (span (offset 66612) (line 1259) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 66612) (line 1259) (column 64) (len 8)))))
    (reference r777 (scope relative) (span (offset 66623) (line 1259) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 66623) (line 1259) (column 75) (len 3)))))
    (reference r778 (scope relative) (span (offset 66627) (line 1259) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 66627) (line 1259) (column 79) (len 1)))))
    (reference r779 (scope relative) (span (offset 66634) (line 1259) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 66634) (line 1259) (column 86) (len 8)))))
    (reference r780 (scope relative) (span (offset 66689) (line 1260) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 66689) (line 1260) (column 39) (len 19)))))
    (reference r781 (scope relative) (span (offset 66718) (line 1260) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 66718) (line 1260) (column 68) (len 8)))))
    (reference r782 (scope relative) (span (offset 66729) (line 1260) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 66729) (line 1260) (column 79) (len 3)))))
    (reference r783 (scope relative) (span (offset 66733) (line 1260) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 66733) (line 1260) (column 83) (len 1)))))
    (reference r784 (scope relative) (span (offset 66740) (line 1260) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 66740) (line 1260) (column 90) (len 8)))))
    (reference r785 (scope relative) (span (offset 66801) (line 1261) (column 46) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 66801) (line 1261) (column 46) (len 19)))))
    (reference r786 (scope relative) (span (offset 66830) (line 1261) (column 75) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 66830) (line 1261) (column 75) (len 8)))))
    (reference r787 (scope relative) (span (offset 66841) (line 1261) (column 86) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 66841) (line 1261) (column 86) (len 3)))))
    (reference r788 (scope relative) (span (offset 66845) (line 1261) (column 90) (len 1)) (segments (segment 0 (token "I") (name "I") (separator none) (span (offset 66845) (line 1261) (column 90) (len 1)))))
    (reference r789 (scope relative) (span (offset 66852) (line 1261) (column 97) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 66852) (line 1261) (column 97) (len 8)))))
    (reference r790 (scope relative) (span (offset 66915) (line 1262) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 66915) (line 1262) (column 48) (len 19)))))
    (reference r791 (scope relative) (span (offset 66944) (line 1262) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 66944) (line 1262) (column 77) (len 8)))))
    (reference r792 (scope relative) (span (offset 66955) (line 1262) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 66955) (line 1262) (column 88) (len 3)))))
    (reference r793 (scope relative) (span (offset 66959) (line 1262) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 66959) (line 1262) (column 92) (len 1)))))
    (reference r794 (scope relative) (span (offset 66966) (line 1262) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 66966) (line 1262) (column 99) (len 8)))))
    (reference r795 (scope relative) (span (offset 67005) (line 1263) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 67005) (line 1263) (column 23) (len 17)))))
    (reference r796 (scope relative) (span (offset 67029) (line 1263) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 67029) (line 1263) (column 47) (len 20)))))
    (reference r797 (scope relative) (span (offset 67053) (line 1263) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 67053) (line 1263) (column 71) (len 6)))))
    (reference r798 (scope relative) (span (offset 67061) (line 1263) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 67061) (line 1263) (column 79) (len 10)))))
    (reference r799 (scope relative) (span (offset 67073) (line 1263) (column 91) (len 17)) (segments (segment 0 (token "electricCurrentPF") (name "electricCurrentPF") (separator none) (span (offset 67073) (line 1263) (column 91) (len 17)))))
    (reference r800 (scope relative) (span (offset 67092) (line 1263) (column 110) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 67092) (line 1263) (column 110) (len 19)))))
    (reference r801 (scope relative) (span (offset 67267) (line 1267) (column 52) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 67267) (line 1267) (column 52) (len 17)))))
    (reference r802 (scope relative) (span (offset 67912) (line 1281) (column 41) (len 29)) (segments (segment 0 (token "TransportNumberOfTheIonBValue") (name "TransportNumberOfTheIonBValue") (separator none) (span (offset 67912) (line 1281) (column 41) (len 29)))))
    (reference r803 (scope relative) (span (offset 68003) (line 1283) (column 40) (len 24)) (segments (segment 0 (token "transportNumberOfTheIonB") (name "transportNumberOfTheIonB") (separator none) (span (offset 68003) (line 1283) (column 40) (len 24)))))
    (reference r804 (scope relative) (span (offset 68126) (line 1286) (column 39) (len 19)) (segments (segment 0 (token "AngularMeasureValue") (name "AngularMeasureValue") (separator none) (span (offset 68126) (line 1286) (column 39) (len 19)))))
    (reference r805 (scope relative) (span (offset 68806) (line 1302) (column 53) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 68806) (line 1302) (column 53) (len 19)))))
    (reference r806 (scope relative) (span (offset 69499) (line 1315) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 69499) (line 1315) (column 28) (len 4)))))
    (reference r807 (scope relative) (span (offset 69494) (line 1315) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 69494) (line 1315) (column 23) (len 3)))))
    (reference r808 (scope relative) (span (offset 69533) (line 1316) (column 29) (len 29)) (segments (segment 0 (token "MolarOpticalRotatoryPowerUnit") (name "MolarOpticalRotatoryPowerUnit") (separator none) (span (offset 69533) (line 1316) (column 29) (len 29)))))
    (reference r809 (scope relative) (span (offset 69527) (line 1316) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 69527) (line 1316) (column 23) (len 4)))))
    (reference r810 (scope relative) (span (offset 69615) (line 1319) (column 42) (len 30)) (segments (segment 0 (token "MolarOpticalRotatoryPowerValue") (name "MolarOpticalRotatoryPowerValue") (separator none) (span (offset 69615) (line 1319) (column 42) (len 30)))))
    (reference r811 (scope relative) (span (offset 69732) (line 1321) (column 52) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 69732) (line 1321) (column 52) (len 11)))))
    (reference r812 (scope relative) (span (offset 69782) (line 1322) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 69782) (line 1322) (column 37) (len 19)))))
    (reference r813 (scope relative) (span (offset 69811) (line 1322) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 69811) (line 1322) (column 66) (len 8)))))
    (reference r814 (scope relative) (span (offset 69822) (line 1322) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 69822) (line 1322) (column 77) (len 3)))))
    (reference r815 (scope relative) (span (offset 69826) (line 1322) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 69826) (line 1322) (column 81) (len 1)))))
    (reference r816 (scope relative) (span (offset 69833) (line 1322) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 69833) (line 1322) (column 88) (len 8)))))
    (reference r817 (scope relative) (span (offset 69896) (line 1323) (column 48) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 69896) (line 1323) (column 48) (len 19)))))
    (reference r818 (scope relative) (span (offset 69925) (line 1323) (column 77) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 69925) (line 1323) (column 77) (len 8)))))
    (reference r819 (scope relative) (span (offset 69936) (line 1323) (column 88) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 69936) (line 1323) (column 88) (len 3)))))
    (reference r820 (scope relative) (span (offset 69940) (line 1323) (column 92) (len 1)) (segments (segment 0 (token "N") (name "N") (separator none) (span (offset 69940) (line 1323) (column 92) (len 1)))))
    (reference r821 (scope relative) (span (offset 69947) (line 1323) (column 99) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 69947) (line 1323) (column 99) (len 8)))))
    (reference r822 (scope relative) (span (offset 69986) (line 1324) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 69986) (line 1324) (column 23) (len 17)))))
    (reference r823 (scope relative) (span (offset 70010) (line 1324) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 70010) (line 1324) (column 47) (len 20)))))
    (reference r824 (scope relative) (span (offset 70034) (line 1324) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 70034) (line 1324) (column 71) (len 8)))))
    (reference r825 (scope relative) (span (offset 70044) (line 1324) (column 81) (len 19)) (segments (segment 0 (token "amountOfSubstancePF") (name "amountOfSubstancePF") (separator none) (span (offset 70044) (line 1324) (column 81) (len 19)))))
    (reference r826 (scope relative) (span (offset 70194) (line 1328) (column 56) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 70194) (line 1328) (column 56) (len 19)))))
    (reference r827 (scope relative) (span (offset 70879) (line 1341) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 70879) (line 1341) (column 28) (len 4)))))
    (reference r828 (scope relative) (span (offset 70874) (line 1341) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 70874) (line 1341) (column 23) (len 3)))))
    (reference r829 (scope relative) (span (offset 70913) (line 1342) (column 29) (len 32)) (segments (segment 0 (token "SpecificOpticalRotatoryPowerUnit") (name "SpecificOpticalRotatoryPowerUnit") (separator none) (span (offset 70913) (line 1342) (column 29) (len 32)))))
    (reference r830 (scope relative) (span (offset 70907) (line 1342) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 70907) (line 1342) (column 23) (len 4)))))
    (reference r831 (scope relative) (span (offset 71001) (line 1345) (column 45) (len 33)) (segments (segment 0 (token "SpecificOpticalRotatoryPowerValue") (name "SpecificOpticalRotatoryPowerValue") (separator none) (span (offset 71001) (line 1345) (column 45) (len 33)))))
    (reference r832 (scope relative) (span (offset 71124) (line 1347) (column 55) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 71124) (line 1347) (column 55) (len 11)))))
    (reference r833 (scope relative) (span (offset 71174) (line 1348) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 71174) (line 1348) (column 37) (len 19)))))
    (reference r834 (scope relative) (span (offset 71203) (line 1348) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 71203) (line 1348) (column 66) (len 8)))))
    (reference r835 (scope relative) (span (offset 71214) (line 1348) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 71214) (line 1348) (column 77) (len 3)))))
    (reference r836 (scope relative) (span (offset 71218) (line 1348) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 71218) (line 1348) (column 81) (len 1)))))
    (reference r837 (scope relative) (span (offset 71225) (line 1348) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 71225) (line 1348) (column 88) (len 8)))))
    (reference r838 (scope relative) (span (offset 71275) (line 1349) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 71275) (line 1349) (column 35) (len 19)))))
    (reference r839 (scope relative) (span (offset 71304) (line 1349) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 71304) (line 1349) (column 64) (len 8)))))
    (reference r840 (scope relative) (span (offset 71315) (line 1349) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 71315) (line 1349) (column 75) (len 3)))))
    (reference r841 (scope relative) (span (offset 71319) (line 1349) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 71319) (line 1349) (column 79) (len 1)))))
    (reference r842 (scope relative) (span (offset 71326) (line 1349) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 71326) (line 1349) (column 86) (len 8)))))
    (reference r843 (scope relative) (span (offset 71365) (line 1350) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 71365) (line 1350) (column 23) (len 17)))))
    (reference r844 (scope relative) (span (offset 71389) (line 1350) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 71389) (line 1350) (column 47) (len 20)))))
    (reference r845 (scope relative) (span (offset 71413) (line 1350) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 71413) (line 1350) (column 71) (len 8)))))
    (reference r846 (scope relative) (span (offset 71423) (line 1350) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 71423) (line 1350) (column 81) (len 6)))))
  )
  (root (library-package (name "ISQChemistryMolecular") (standard true) (body brace (doc) (import (target (span (span (offset 819) (line 15) (column 20) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 858) (line 16) (column 20) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 868) (line 16) (column 30) (len 3))) (separator (span (offset 868) (line 16) (column 30) (len 2))) (marker (span (offset 870) (line 16) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 892) (line 17) (column 20) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 913) (line 17) (column 41) (len 3))) (separator (span (offset 913) (line 17) (column 41) (len 2))) (marker (span (offset 915) (line 17) (column 43) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 937) (line 18) (column 20) (len 10))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 944) (line 18) (column 27) (len 3))) (separator (span (offset 944) (line 18) (column 27) (len 2))) (marker (span (offset 946) (line 18) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1035) (line 21) (column 20) (len 33))) (all none) (ref r4) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1089) (line 22) (column 20) (len 30))) (all none) (ref r5) (shape (membership (recursive-suffix none))))) (attribute-def (declaration-name "numberOfEntities") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (alias (name "NumberOfMolesUnit") (target (ref r7)) (body semicolon)) (alias (name "NumberOfMolesValue") (target (ref r8)) (body semicolon)) (alias (name "numberOfMoles") (target (ref r9)) (body semicolon)) (attribute-def (declaration-name "RelativeAtomicMassValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "relativeAtomicMass") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "MolarMassValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r14)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "molarMass") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MolarMassUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4267) (line 85) (column 75) (len 5)) (member-access (base (expression (span (offset 4267) (line 85) (column 75) (len 3)) (ref r21))) (separator dot) (member (ref r22))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r23)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4289) (line 85) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r25)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4381) (line 86) (column 88) (len 5)) (member-access (base (expression (span (offset 4381) (line 86) (column 88) (len 3)) (ref r26))) (separator dot) (member (ref r27))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r28)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4403) (line 86) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 4404) (line 86) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r29)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r30)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4478) (line 87) (column 70) (len 29)) (tuple (expression (span (offset 4479) (line 87) (column 71) (len 6)) (ref r31)) (expression (span (offset 4487) (line 87) (column 79) (len 19)) (ref r32))))))) (body semicolon)))))) (attribute-def (declaration-name "MolarVolumeValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r33)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r34)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r35)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r36)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r37)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "molarVolume") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r38)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MolarVolumeUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r39)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r40)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r41)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5362) (line 111) (column 77) (len 5)) (member-access (base (expression (span (offset 5362) (line 111) (column 77) (len 3)) (ref r42))) (separator dot) (member (ref r43))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r44)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5384) (line 111) (column 99) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r45)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r46)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5476) (line 112) (column 88) (len 5)) (member-access (base (expression (span (offset 5476) (line 112) (column 88) (len 3)) (ref r47))) (separator dot) (member (ref r48))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r49)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5498) (line 112) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 5499) (line 112) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r50)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r51)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5573) (line 113) (column 70) (len 31)) (tuple (expression (span (offset 5574) (line 113) (column 71) (len 8)) (ref r52)) (expression (span (offset 5584) (line 113) (column 81) (len 19)) (ref r53))))))) (body semicolon)))))) (attribute-def (declaration-name "MolarInternalEnergyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r54)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r55)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r56)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r57)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r58)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "molarInternalEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r59)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MolarInternalEnergyUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r60)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r61)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r62)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6605) (line 137) (column 77) (len 5)) (member-access (base (expression (span (offset 6605) (line 137) (column 77) (len 3)) (ref r63))) (separator dot) (member (ref r64))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r65)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6627) (line 137) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r66)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r67)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6706) (line 138) (column 75) (len 5)) (member-access (base (expression (span (offset 6706) (line 138) (column 75) (len 3)) (ref r68))) (separator dot) (member (ref r69))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r70)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6728) (line 138) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r71)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r72)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6811) (line 139) (column 79) (len 5)) (member-access (base (expression (span (offset 6811) (line 139) (column 79) (len 3)) (ref r73))) (separator dot) (member (ref r74))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r75)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6833) (line 139) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 6834) (line 139) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r76)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r77)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6926) (line 140) (column 88) (len 5)) (member-access (base (expression (span (offset 6926) (line 140) (column 88) (len 3)) (ref r78))) (separator dot) (member (ref r79))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r80)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6948) (line 140) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 6949) (line 140) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r81)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r82)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7023) (line 141) (column 70) (len 51)) (tuple (expression (span (offset 7024) (line 141) (column 71) (len 8)) (ref r83)) (expression (span (offset 7034) (line 141) (column 81) (len 6)) (ref r84)) (expression (span (offset 7042) (line 141) (column 89) (len 10)) (ref r85)) (expression (span (offset 7054) (line 141) (column 101) (len 19)) (ref r86))))))) (body semicolon)))))) (attribute-def (declaration-name "MolarEnthalpyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r87)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r88)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r89)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r90)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r91)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "molarEnthalpy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r92)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MolarEnthalpyUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r93)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r94)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r95)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8018) (line 165) (column 77) (len 5)) (member-access (base (expression (span (offset 8018) (line 165) (column 77) (len 3)) (ref r96))) (separator dot) (member (ref r97))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r98)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8040) (line 165) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r99)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r100)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8119) (line 166) (column 75) (len 5)) (member-access (base (expression (span (offset 8119) (line 166) (column 75) (len 3)) (ref r101))) (separator dot) (member (ref r102))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r103)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8141) (line 166) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r104)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r105)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8224) (line 167) (column 79) (len 5)) (member-access (base (expression (span (offset 8224) (line 167) (column 79) (len 3)) (ref r106))) (separator dot) (member (ref r107))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r108)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8246) (line 167) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 8247) (line 167) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r109)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r110)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8339) (line 168) (column 88) (len 5)) (member-access (base (expression (span (offset 8339) (line 168) (column 88) (len 3)) (ref r111))) (separator dot) (member (ref r112))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r113)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8361) (line 168) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 8362) (line 168) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r114)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r115)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8436) (line 169) (column 70) (len 51)) (tuple (expression (span (offset 8437) (line 169) (column 71) (len 8)) (ref r116)) (expression (span (offset 8447) (line 169) (column 81) (len 6)) (ref r117)) (expression (span (offset 8455) (line 169) (column 89) (len 10)) (ref r118)) (expression (span (offset 8467) (line 169) (column 101) (len 19)) (ref r119))))))) (body semicolon)))))) (attribute-def (declaration-name "MolarHelmholtzEnergyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r120)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r121)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r122)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r123)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r124)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "molarHelmholtzEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r125)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MolarHelmholtzEnergyUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r126)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r127)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r128)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9501) (line 193) (column 77) (len 5)) (member-access (base (expression (span (offset 9501) (line 193) (column 77) (len 3)) (ref r129))) (separator dot) (member (ref r130))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r131)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9523) (line 193) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r132)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r133)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9602) (line 194) (column 75) (len 5)) (member-access (base (expression (span (offset 9602) (line 194) (column 75) (len 3)) (ref r134))) (separator dot) (member (ref r135))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r136)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9624) (line 194) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r137)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r138)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9707) (line 195) (column 79) (len 5)) (member-access (base (expression (span (offset 9707) (line 195) (column 79) (len 3)) (ref r139))) (separator dot) (member (ref r140))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r141)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9729) (line 195) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 9730) (line 195) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r142)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r143)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9822) (line 196) (column 88) (len 5)) (member-access (base (expression (span (offset 9822) (line 196) (column 88) (len 3)) (ref r144))) (separator dot) (member (ref r145))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r146)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9844) (line 196) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 9845) (line 196) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r147)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r148)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9919) (line 197) (column 70) (len 51)) (tuple (expression (span (offset 9920) (line 197) (column 71) (len 8)) (ref r149)) (expression (span (offset 9930) (line 197) (column 81) (len 6)) (ref r150)) (expression (span (offset 9938) (line 197) (column 89) (len 10)) (ref r151)) (expression (span (offset 9950) (line 197) (column 101) (len 19)) (ref r152))))))) (body semicolon)))))) (attribute-def (declaration-name "MolarGibbsEnergyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r153)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r154)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r155)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r156)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r157)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "molarGibbsEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r158)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MolarGibbsEnergyUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r159)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r160)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r161)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10948) (line 221) (column 77) (len 5)) (member-access (base (expression (span (offset 10948) (line 221) (column 77) (len 3)) (ref r162))) (separator dot) (member (ref r163))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r164)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10970) (line 221) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r165)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r166)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11049) (line 222) (column 75) (len 5)) (member-access (base (expression (span (offset 11049) (line 222) (column 75) (len 3)) (ref r167))) (separator dot) (member (ref r168))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r169)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11071) (line 222) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r170)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r171)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11154) (line 223) (column 79) (len 5)) (member-access (base (expression (span (offset 11154) (line 223) (column 79) (len 3)) (ref r172))) (separator dot) (member (ref r173))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r174)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11176) (line 223) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 11177) (line 223) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r175)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r176)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11269) (line 224) (column 88) (len 5)) (member-access (base (expression (span (offset 11269) (line 224) (column 88) (len 3)) (ref r177))) (separator dot) (member (ref r178))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r179)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11291) (line 224) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 11292) (line 224) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r180)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r181)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11366) (line 225) (column 70) (len 51)) (tuple (expression (span (offset 11367) (line 225) (column 71) (len 8)) (ref r182)) (expression (span (offset 11377) (line 225) (column 81) (len 6)) (ref r183)) (expression (span (offset 11385) (line 225) (column 89) (len 10)) (ref r184)) (expression (span (offset 11397) (line 225) (column 101) (len 19)) (ref r185))))))) (body semicolon)))))) (attribute-def (declaration-name "MolarHeatCapacityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r186)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r187)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r188)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r189)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r190)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "molarHeatCapacity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r191)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MolarHeatCapacityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r192)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r193)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r194)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12401) (line 249) (column 77) (len 5)) (member-access (base (expression (span (offset 12401) (line 249) (column 77) (len 3)) (ref r195))) (separator dot) (member (ref r196))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r197)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12423) (line 249) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r198)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r199)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12502) (line 250) (column 75) (len 5)) (member-access (base (expression (span (offset 12502) (line 250) (column 75) (len 3)) (ref r200))) (separator dot) (member (ref r201))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r202)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12524) (line 250) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r203)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r204)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12607) (line 251) (column 79) (len 5)) (member-access (base (expression (span (offset 12607) (line 251) (column 79) (len 3)) (ref r205))) (separator dot) (member (ref r206))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r207)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12629) (line 251) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 12630) (line 251) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r208)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r209)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12729) (line 252) (column 95) (len 8)) (member-access (base (expression (span (offset 12729) (line 252) (column 95) (len 3)) (ref r210))) (separator dot) (member (ref r211))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r212)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12754) (line 252) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 12755) (line 252) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r213)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r214)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12847) (line 253) (column 88) (len 5)) (member-access (base (expression (span (offset 12847) (line 253) (column 88) (len 3)) (ref r215))) (separator dot) (member (ref r216))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r217)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12869) (line 253) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 12870) (line 253) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r218)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r219)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12944) (line 254) (column 70) (len 79)) (tuple (expression (span (offset 12945) (line 254) (column 71) (len 8)) (ref r220)) (expression (span (offset 12955) (line 254) (column 81) (len 6)) (ref r221)) (expression (span (offset 12963) (line 254) (column 89) (len 10)) (ref r222)) (expression (span (offset 12975) (line 254) (column 101) (len 26)) (ref r223)) (expression (span (offset 13003) (line 254) (column 129) (len 19)) (ref r224))))))) (body semicolon)))))) (attribute-def (declaration-name "MolarEntropyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r225)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r226)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r227)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r228)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r229)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "molarEntropy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r230)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MolarEntropyUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r231)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r232)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r233)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13959) (line 278) (column 77) (len 5)) (member-access (base (expression (span (offset 13959) (line 278) (column 77) (len 3)) (ref r234))) (separator dot) (member (ref r235))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r236)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13981) (line 278) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r237)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r238)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14060) (line 279) (column 75) (len 5)) (member-access (base (expression (span (offset 14060) (line 279) (column 75) (len 3)) (ref r239))) (separator dot) (member (ref r240))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r241)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14082) (line 279) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r242)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r243)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14165) (line 280) (column 79) (len 5)) (member-access (base (expression (span (offset 14165) (line 280) (column 79) (len 3)) (ref r244))) (separator dot) (member (ref r245))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r246)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14187) (line 280) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 14188) (line 280) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r247)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r248)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14287) (line 281) (column 95) (len 8)) (member-access (base (expression (span (offset 14287) (line 281) (column 95) (len 3)) (ref r249))) (separator dot) (member (ref r250))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r251)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14312) (line 281) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 14313) (line 281) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r252)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r253)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14405) (line 282) (column 88) (len 5)) (member-access (base (expression (span (offset 14405) (line 282) (column 88) (len 3)) (ref r254))) (separator dot) (member (ref r255))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r256)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14427) (line 282) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 14428) (line 282) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r257)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r258)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14502) (line 283) (column 70) (len 79)) (tuple (expression (span (offset 14503) (line 283) (column 71) (len 8)) (ref r259)) (expression (span (offset 14513) (line 283) (column 81) (len 6)) (ref r260)) (expression (span (offset 14521) (line 283) (column 89) (len 10)) (ref r261)) (expression (span (offset 14533) (line 283) (column 101) (len 26)) (ref r262)) (expression (span (offset 14561) (line 283) (column 129) (len 19)) (ref r263))))))) (body semicolon)))))) (attribute-def (declaration-name "ParticleConcentrationValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r264)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r265)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r266)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r267)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r268)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "particleConcentration") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r269)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "ParticleConcentrationUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r270)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r271)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r272)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15521) (line 307) (column 77) (len 5)) (member-access (base (expression (span (offset 15521) (line 307) (column 77) (len 3)) (ref r273))) (separator dot) (member (ref r274))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r275)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15543) (line 307) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 15544) (line 307) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r276)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r277)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15618) (line 308) (column 70) (len 8)) (ref r278))))) (body semicolon)))))) (attribute-def (declaration-name "molecularConcentration") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r279)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "MassConcentrationValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r280)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r281)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r282)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r283)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r284)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "massConcentration") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r285)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MassConcentrationUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r286)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r287)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r288)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17348) (line 348) (column 77) (len 5)) (member-access (base (expression (span (offset 17348) (line 348) (column 77) (len 3)) (ref r289))) (separator dot) (member (ref r290))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r291)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17370) (line 348) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 17371) (line 348) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r292)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r293)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17450) (line 349) (column 75) (len 5)) (member-access (base (expression (span (offset 17450) (line 349) (column 75) (len 3)) (ref r294))) (separator dot) (member (ref r295))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r296)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17472) (line 349) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r297)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r298)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17546) (line 350) (column 70) (len 18)) (tuple (expression (span (offset 17547) (line 350) (column 71) (len 8)) (ref r299)) (expression (span (offset 17557) (line 350) (column 81) (len 6)) (ref r300))))))) (body semicolon)))))) (attribute-def (declaration-name "MassFractionValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r301)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "massFraction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r302)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "AmountOfSubstanceConcentrationValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r303)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r304)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r305)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r306)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r307)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "amountOfSubstanceConcentration") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r308)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "AmountOfSubstanceConcentrationUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r309)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r310)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r311)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19678) (line 391) (column 77) (len 5)) (member-access (base (expression (span (offset 19678) (line 391) (column 77) (len 3)) (ref r312))) (separator dot) (member (ref r313))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r314)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19700) (line 391) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 19701) (line 391) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r315)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r316)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19793) (line 392) (column 88) (len 5)) (member-access (base (expression (span (offset 19793) (line 392) (column 88) (len 3)) (ref r317))) (separator dot) (member (ref r318))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r319)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19815) (line 392) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r320)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r321)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19889) (line 393) (column 70) (len 31)) (tuple (expression (span (offset 19890) (line 393) (column 71) (len 8)) (ref r322)) (expression (span (offset 19900) (line 393) (column 81) (len 19)) (ref r323))))))) (body semicolon)))))) (attribute-def (declaration-name "standardAmountOfSubstanceConcentration") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r324)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "AmountOfSubstanceFractionMoleFractionValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r325)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "amountOfSubstanceFractionMoleFraction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r326)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "VolumeFractionValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r327)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r328)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r329)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r330)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r331)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "volumeFraction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r332)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "VolumeFractionUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r333)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace)) (attribute-def (declaration-name "MolalityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r334)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r335)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r336)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r337)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r338)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "molality") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r339)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MolalityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r340)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r341)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r342)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24200) (line 473) (column 75) (len 5)) (member-access (base (expression (span (offset 24200) (line 473) (column 75) (len 3)) (ref r343))) (separator dot) (member (ref r344))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r345)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24222) (line 473) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 24223) (line 473) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r346)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r347)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24315) (line 474) (column 88) (len 5)) (member-access (base (expression (span (offset 24315) (line 474) (column 88) (len 3)) (ref r348))) (separator dot) (member (ref r349))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r350)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24337) (line 474) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r351)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r352)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24411) (line 475) (column 70) (len 29)) (tuple (expression (span (offset 24412) (line 475) (column 71) (len 6)) (ref r353)) (expression (span (offset 24420) (line 475) (column 79) (len 19)) (ref r354))))))) (body semicolon)))))) (attribute-def (declaration-name "latentHeatOfPhaseTransition") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r355)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (alias (name "enthalpyOfPhaseTransition") (target (ref r356)) (body semicolon)) (attribute-def (declaration-name "ChemicalPotentialValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r357)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r358)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r359)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r360)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r361)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "chemicalPotential") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r362)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "ChemicalPotentialUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r363)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r364)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r365)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26768) (line 517) (column 77) (len 5)) (member-access (base (expression (span (offset 26768) (line 517) (column 77) (len 3)) (ref r366))) (separator dot) (member (ref r367))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r368)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26790) (line 517) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r369)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r370)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26869) (line 518) (column 75) (len 5)) (member-access (base (expression (span (offset 26869) (line 518) (column 75) (len 3)) (ref r371))) (separator dot) (member (ref r372))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r373)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26891) (line 518) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r374)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r375)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26974) (line 519) (column 79) (len 5)) (member-access (base (expression (span (offset 26974) (line 519) (column 79) (len 3)) (ref r376))) (separator dot) (member (ref r377))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r378)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 26996) (line 519) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 26997) (line 519) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r379)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r380)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27089) (line 520) (column 88) (len 5)) (member-access (base (expression (span (offset 27089) (line 520) (column 88) (len 3)) (ref r381))) (separator dot) (member (ref r382))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r383)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27111) (line 520) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 27112) (line 520) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r384)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r385)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27186) (line 521) (column 70) (len 51)) (tuple (expression (span (offset 27187) (line 521) (column 71) (len 8)) (ref r386)) (expression (span (offset 27197) (line 521) (column 81) (len 6)) (ref r387)) (expression (span (offset 27205) (line 521) (column 89) (len 10)) (ref r388)) (expression (span (offset 27217) (line 521) (column 101) (len 19)) (ref r389))))))) (body semicolon)))))) (attribute-def (declaration-name "AbsoluteActivityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r390)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "absoluteActivity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r391)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "PartialPressureValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r392)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r393)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r394)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r395)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r396)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "partialPressure") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r397)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "PartialPressureUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r398)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r399)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r400)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 28971) (line 562) (column 77) (len 5)) (member-access (base (expression (span (offset 28971) (line 562) (column 77) (len 3)) (ref r401))) (separator dot) (member (ref r402))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r403)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 28993) (line 562) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 28994) (line 562) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r404)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r405)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29073) (line 563) (column 75) (len 5)) (member-access (base (expression (span (offset 29073) (line 563) (column 75) (len 3)) (ref r406))) (separator dot) (member (ref r407))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r408)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29095) (line 563) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r409)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r410)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29178) (line 564) (column 79) (len 5)) (member-access (base (expression (span (offset 29178) (line 564) (column 79) (len 3)) (ref r411))) (separator dot) (member (ref r412))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r413)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29200) (line 564) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 29201) (line 564) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r414)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r415)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29275) (line 565) (column 70) (len 30)) (tuple (expression (span (offset 29276) (line 565) (column 71) (len 8)) (ref r416)) (expression (span (offset 29286) (line 565) (column 81) (len 6)) (ref r417)) (expression (span (offset 29294) (line 565) (column 89) (len 10)) (ref r418))))))) (body semicolon)))))) (attribute-def (declaration-name "FugacityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r419)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r420)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r421)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r422)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r423)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "fugacity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r424)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "FugacityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r425)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r426)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r427)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30478) (line 589) (column 77) (len 5)) (member-access (base (expression (span (offset 30478) (line 589) (column 77) (len 3)) (ref r428))) (separator dot) (member (ref r429))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r430)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30500) (line 589) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 30501) (line 589) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r431)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r432)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30580) (line 590) (column 75) (len 5)) (member-access (base (expression (span (offset 30580) (line 590) (column 75) (len 3)) (ref r433))) (separator dot) (member (ref r434))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r435)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30602) (line 590) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r436)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r437)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30685) (line 591) (column 79) (len 5)) (member-access (base (expression (span (offset 30685) (line 591) (column 79) (len 3)) (ref r438))) (separator dot) (member (ref r439))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r440)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30707) (line 591) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 30708) (line 591) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r441)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r442)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30782) (line 592) (column 70) (len 30)) (tuple (expression (span (offset 30783) (line 592) (column 71) (len 8)) (ref r443)) (expression (span (offset 30793) (line 592) (column 81) (len 6)) (ref r444)) (expression (span (offset 30801) (line 592) (column 89) (len 10)) (ref r445))))))) (body semicolon)))))) (attribute-def (declaration-name "StandardChemicalPotentialValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r446)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r447)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r448)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r449)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r450)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "standardChemicalPotential") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r451)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "StandardChemicalPotentialUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r452)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r453)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r454)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32122) (line 616) (column 77) (len 5)) (member-access (base (expression (span (offset 32122) (line 616) (column 77) (len 3)) (ref r455))) (separator dot) (member (ref r456))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r457)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32144) (line 616) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r458)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r459)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32223) (line 617) (column 75) (len 5)) (member-access (base (expression (span (offset 32223) (line 617) (column 75) (len 3)) (ref r460))) (separator dot) (member (ref r461))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r462)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32245) (line 617) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r463)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r464)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32328) (line 618) (column 79) (len 5)) (member-access (base (expression (span (offset 32328) (line 618) (column 79) (len 3)) (ref r465))) (separator dot) (member (ref r466))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r467)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32350) (line 618) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 32351) (line 618) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r468)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r469)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32443) (line 619) (column 88) (len 5)) (member-access (base (expression (span (offset 32443) (line 619) (column 88) (len 3)) (ref r470))) (separator dot) (member (ref r471))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r472)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32465) (line 619) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 32466) (line 619) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r473)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r474)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32540) (line 620) (column 70) (len 51)) (tuple (expression (span (offset 32541) (line 620) (column 71) (len 8)) (ref r475)) (expression (span (offset 32551) (line 620) (column 81) (len 6)) (ref r476)) (expression (span (offset 32559) (line 620) (column 89) (len 10)) (ref r477)) (expression (span (offset 32571) (line 620) (column 101) (len 19)) (ref r478))))))) (body semicolon)))))) (attribute-def (declaration-name "ActivityFactorValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r479)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "activityFactor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r480)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "StandardAbsoluteActivityInMixtureValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r481)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "standardAbsoluteActivityInMixture") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r482)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "ActivityOfSoluteValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r483)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "activityOfSolute") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r484)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (alias (name "relativeActivityOfSolute") (target (ref r485)) (body semicolon)) (attribute-def (declaration-name "ActivityCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r486)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "activityCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r487)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "StandardAbsoluteActivityInSolutionValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r488)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "standardAbsoluteActivityInSolution") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r489)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "ActivityOfSolventValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r490)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "activityOfSolvent") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r491)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (alias (name "relativeActivityOfSolvent") (target (ref r492)) (body semicolon)) (attribute-def (declaration-name "OsmoticFactorOfSolventValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r493)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "osmoticFactorOfSolvent") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r494)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (alias (name "osmoticCoefficientOfSolventA") (target (ref r495)) (body semicolon)) (attribute-def (declaration-name "StandardAbsoluteActivityOfSolventValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r496)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "standardAbsoluteActivityOfSolvent") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r497)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "OsmoticPressureValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r498)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r499)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r500)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r501)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r502)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "osmoticPressure") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r503)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "OsmoticPressureUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r504)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r505)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r506)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 42090) (line 786) (column 77) (len 5)) (member-access (base (expression (span (offset 42090) (line 786) (column 77) (len 3)) (ref r507))) (separator dot) (member (ref r508))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r509)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 42112) (line 786) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 42113) (line 786) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r510)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r511)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 42192) (line 787) (column 75) (len 5)) (member-access (base (expression (span (offset 42192) (line 787) (column 75) (len 3)) (ref r512))) (separator dot) (member (ref r513))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r514)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 42214) (line 787) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r515)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r516)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 42297) (line 788) (column 79) (len 5)) (member-access (base (expression (span (offset 42297) (line 788) (column 79) (len 3)) (ref r517))) (separator dot) (member (ref r518))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r519)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 42319) (line 788) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 42320) (line 788) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r520)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r521)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 42394) (line 789) (column 70) (len 30)) (tuple (expression (span (offset 42395) (line 789) (column 71) (len 8)) (ref r522)) (expression (span (offset 42405) (line 789) (column 81) (len 6)) (ref r523)) (expression (span (offset 42413) (line 789) (column 89) (len 10)) (ref r524))))))) (body semicolon)))))) (attribute-def (declaration-name "StoichiometricNumberOfSubstanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r525)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "stoichiometricNumberOfSubstance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r526)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "AffinityOfAChemicalReactionValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r527)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r528)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r529)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r530)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r531)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "affinityOfAChemicalReaction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r532)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "AffinityOfAChemicalReactionUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r533)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r534)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r535)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 44997) (line 830) (column 77) (len 5)) (member-access (base (expression (span (offset 44997) (line 830) (column 77) (len 3)) (ref r536))) (separator dot) (member (ref r537))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r538)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45019) (line 830) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r539)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r540)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45098) (line 831) (column 75) (len 5)) (member-access (base (expression (span (offset 45098) (line 831) (column 75) (len 3)) (ref r541))) (separator dot) (member (ref r542))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r543)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45120) (line 831) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r544)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r545)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45203) (line 832) (column 79) (len 5)) (member-access (base (expression (span (offset 45203) (line 832) (column 79) (len 3)) (ref r546))) (separator dot) (member (ref r547))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r548)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45225) (line 832) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 45226) (line 832) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r549)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r550)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45318) (line 833) (column 88) (len 5)) (member-access (base (expression (span (offset 45318) (line 833) (column 88) (len 3)) (ref r551))) (separator dot) (member (ref r552))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r553)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45340) (line 833) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 45341) (line 833) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r554)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r555)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45415) (line 834) (column 70) (len 51)) (tuple (expression (span (offset 45416) (line 834) (column 71) (len 8)) (ref r556)) (expression (span (offset 45426) (line 834) (column 81) (len 6)) (ref r557)) (expression (span (offset 45434) (line 834) (column 89) (len 10)) (ref r558)) (expression (span (offset 45446) (line 834) (column 101) (len 19)) (ref r559))))))) (body semicolon)))))) (attribute-def (declaration-name "extentOfReaction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r560)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "StandardEquilibriumConstantValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r561)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "standardEquilibriumConstant") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r562)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (alias (name "thermodynamicEquilibriumConstant") (target (ref r563)) (body semicolon)) (attribute-def (declaration-name "EquilibriumConstantOnPressureBasisValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r564)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r565)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r566)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r567)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r568)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "equilibriumConstantOnPressureBasis") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r569)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "EquilibriumConstantOnPressureBasisUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r570)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r571)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r572)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48582) (line 893) (column 77) (len 5)) (member-access (base (expression (span (offset 48582) (line 893) (column 77) (len 3)) (ref r573))) (separator dot) (member (ref r574))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r575)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48604) (line 893) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 48605) (line 893) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r576)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r577)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48684) (line 894) (column 75) (len 5)) (member-access (base (expression (span (offset 48684) (line 894) (column 75) (len 3)) (ref r578))) (separator dot) (member (ref r579))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r580)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48706) (line 894) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r581)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r582)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48789) (line 895) (column 79) (len 5)) (member-access (base (expression (span (offset 48789) (line 895) (column 79) (len 3)) (ref r583))) (separator dot) (member (ref r584))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r585)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48811) (line 895) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 48812) (line 895) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r586)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r587)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48886) (line 896) (column 70) (len 30)) (tuple (expression (span (offset 48887) (line 896) (column 71) (len 8)) (ref r588)) (expression (span (offset 48897) (line 896) (column 81) (len 6)) (ref r589)) (expression (span (offset 48905) (line 896) (column 89) (len 10)) (ref r590))))))) (body semicolon)))))) (attribute-def (declaration-name "EquilibriumConstantOnConcentrationBasisValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r591)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r592)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r593)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r594)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r595)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "equilibriumConstantOnConcentrationBasis") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r596)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "EquilibriumConstantOnConcentrationBasisUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r597)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r598)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r599)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50082) (line 920) (column 77) (len 5)) (member-access (base (expression (span (offset 50082) (line 920) (column 77) (len 3)) (ref r600))) (separator dot) (member (ref r601))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r602)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50104) (line 920) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 50105) (line 920) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r603)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r604)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50197) (line 921) (column 88) (len 5)) (member-access (base (expression (span (offset 50197) (line 921) (column 88) (len 3)) (ref r605))) (separator dot) (member (ref r606))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r607)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50219) (line 921) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r608)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r609)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50293) (line 922) (column 70) (len 31)) (tuple (expression (span (offset 50294) (line 922) (column 71) (len 8)) (ref r610)) (expression (span (offset 50304) (line 922) (column 81) (len 19)) (ref r611))))))) (body semicolon)))))) (attribute-def (declaration-name "microcanonicalPartitionFunction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r612)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "CanonicalPartitionFunctionValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r613)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "canonicalPartitionFunction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r614)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "GrandCanonicalPartitionFunctionValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r615)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "grandCanonicalPartitionFunction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r616)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (alias (name "grandPartitionFunction") (target (ref r617)) (body semicolon)) (attribute-def (declaration-name "MolecularPartitionFunctionValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r618)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "molecularPartitionFunction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r619)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (alias (name "partitionFunctionOfAMolecule") (target (ref r620)) (body semicolon)) (attribute-def (declaration-name "statisticalWeightOfSubsystem") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r621)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "DegeneracyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r622)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "degeneracy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r623)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (alias (name "multiplicity") (target (ref r624)) (body semicolon)) (attribute-def (declaration-name "MolarGasConstantValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r625)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r626)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r627)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r628)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r629)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "molarGasConstant") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r630)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MolarGasConstantUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r631)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r632)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r633)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56388) (line 1052) (column 77) (len 5)) (member-access (base (expression (span (offset 56388) (line 1052) (column 77) (len 3)) (ref r634))) (separator dot) (member (ref r635))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r636)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56410) (line 1052) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r637)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r638)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56489) (line 1053) (column 75) (len 5)) (member-access (base (expression (span (offset 56489) (line 1053) (column 75) (len 3)) (ref r639))) (separator dot) (member (ref r640))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r641)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56511) (line 1053) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r642)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r643)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56594) (line 1054) (column 79) (len 5)) (member-access (base (expression (span (offset 56594) (line 1054) (column 79) (len 3)) (ref r644))) (separator dot) (member (ref r645))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r646)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56616) (line 1054) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 56617) (line 1054) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r647)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r648)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56716) (line 1055) (column 95) (len 8)) (member-access (base (expression (span (offset 56716) (line 1055) (column 95) (len 3)) (ref r649))) (separator dot) (member (ref r650))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r651)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56741) (line 1055) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 56742) (line 1055) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r652)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r653)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56834) (line 1056) (column 88) (len 5)) (member-access (base (expression (span (offset 56834) (line 1056) (column 88) (len 3)) (ref r654))) (separator dot) (member (ref r655))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r656)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56856) (line 1056) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 56857) (line 1056) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r657)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r658)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56931) (line 1057) (column 70) (len 79)) (tuple (expression (span (offset 56932) (line 1057) (column 71) (len 8)) (ref r659)) (expression (span (offset 56942) (line 1057) (column 81) (len 6)) (ref r660)) (expression (span (offset 56950) (line 1057) (column 89) (len 10)) (ref r661)) (expression (span (offset 56962) (line 1057) (column 101) (len 26)) (ref r662)) (expression (span (offset 56990) (line 1057) (column 129) (len 19)) (ref r663))))))) (body semicolon)))))) (attribute-def (declaration-name "meanFreePath") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r664)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "DiffusionCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r665)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r666)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r667)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r668)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r669)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "diffusionCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r670)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "DiffusionCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r671)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r672)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r673)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 58910) (line 1100) (column 77) (len 5)) (member-access (base (expression (span (offset 58910) (line 1100) (column 77) (len 3)) (ref r674))) (separator dot) (member (ref r675))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r676)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 58932) (line 1100) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r677)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r678)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59015) (line 1101) (column 79) (len 5)) (member-access (base (expression (span (offset 59015) (line 1101) (column 79) (len 3)) (ref r679))) (separator dot) (member (ref r680))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r681)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59037) (line 1101) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 59038) (line 1101) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r682)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r683)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 59112) (line 1102) (column 70) (len 22)) (tuple (expression (span (offset 59113) (line 1102) (column 71) (len 8)) (ref r684)) (expression (span (offset 59123) (line 1102) (column 81) (len 10)) (ref r685))))))) (body semicolon)))))) (attribute-def (declaration-name "ThermalDiffusionRatioValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r686)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "thermalDiffusionRatio") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r687)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "ThermalDiffusionFactorValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r688)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "thermalDiffusionFactor") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r689)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "ThermalDiffusionCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r690)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r691)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r692)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r693)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r694)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "thermalDiffusionCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r695)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "ThermalDiffusionCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r696)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r697)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r698)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 61864) (line 1160) (column 77) (len 5)) (member-access (base (expression (span (offset 61864) (line 1160) (column 77) (len 3)) (ref r699))) (separator dot) (member (ref r700))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r701)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 61886) (line 1160) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r702)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r703)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 61969) (line 1161) (column 79) (len 5)) (member-access (base (expression (span (offset 61969) (line 1161) (column 79) (len 3)) (ref r704))) (separator dot) (member (ref r705))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r706)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 61991) (line 1161) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 61992) (line 1161) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r707)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r708)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 62066) (line 1162) (column 70) (len 22)) (tuple (expression (span (offset 62067) (line 1162) (column 71) (len 8)) (ref r709)) (expression (span (offset 62077) (line 1162) (column 81) (len 10)) (ref r710))))))) (body semicolon)))))) (attribute-def (declaration-name "IonicStrengthValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r711)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r712)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r713)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r714)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r715)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "ionicStrength") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r716)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "IonicStrengthUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r717)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r718)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r719)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 63032) (line 1186) (column 75) (len 5)) (member-access (base (expression (span (offset 63032) (line 1186) (column 75) (len 3)) (ref r720))) (separator dot) (member (ref r721))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r722)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 63054) (line 1186) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 63055) (line 1186) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r723)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r724)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 63147) (line 1187) (column 88) (len 5)) (member-access (base (expression (span (offset 63147) (line 1187) (column 88) (len 3)) (ref r725))) (separator dot) (member (ref r726))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r727)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 63169) (line 1187) (column 110) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r728)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r729)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 63243) (line 1188) (column 70) (len 29)) (tuple (expression (span (offset 63244) (line 1188) (column 71) (len 6)) (ref r730)) (expression (span (offset 63252) (line 1188) (column 79) (len 19)) (ref r731))))))) (body semicolon)))))) (attribute-def (declaration-name "DegreeOfDissociationValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r732)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "degreeOfDissociation") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r733)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (alias (name "dissociationFraction") (target (ref r734)) (body semicolon)) (attribute-def (declaration-name "ElectrolyticConductivityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r735)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r736)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r737)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r738)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r739)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "electrolyticConductivity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r740)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "ElectrolyticConductivityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r741)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r742)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r743)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 65190) (line 1231) (column 77) (len 5)) (member-access (base (expression (span (offset 65190) (line 1231) (column 77) (len 3)) (ref r744))) (separator dot) (member (ref r745))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r746)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 65212) (line 1231) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 65213) (line 1231) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r747)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r748)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 65292) (line 1232) (column 75) (len 5)) (member-access (base (expression (span (offset 65292) (line 1232) (column 75) (len 3)) (ref r749))) (separator dot) (member (ref r750))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r751)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 65314) (line 1232) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 65315) (line 1232) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r752)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r753)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 65398) (line 1233) (column 79) (len 5)) (member-access (base (expression (span (offset 65398) (line 1233) (column 79) (len 3)) (ref r754))) (separator dot) (member (ref r755))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r756)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 65420) (line 1233) (column 101) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name "electricCurrentPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r757)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r758)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 65510) (line 1234) (column 86) (len 5)) (member-access (base (expression (span (offset 65510) (line 1234) (column 86) (len 3)) (ref r759))) (separator dot) (member (ref r760))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r761)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 65532) (line 1234) (column 108) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r762)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r763)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 65606) (line 1235) (column 70) (len 49)) (tuple (expression (span (offset 65607) (line 1235) (column 71) (len 8)) (ref r764)) (expression (span (offset 65617) (line 1235) (column 81) (len 6)) (ref r765)) (expression (span (offset 65625) (line 1235) (column 89) (len 10)) (ref r766)) (expression (span (offset 65637) (line 1235) (column 101) (len 17)) (ref r767))))))) (body semicolon)))))) (attribute-def (declaration-name "MolarConductivityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r768)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r769)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r770)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r771)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r772)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "molarConductivity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r773)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MolarConductivityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r774)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r775)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r776)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 66623) (line 1259) (column 75) (len 5)) (member-access (base (expression (span (offset 66623) (line 1259) (column 75) (len 3)) (ref r777))) (separator dot) (member (ref r778))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r779)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 66645) (line 1259) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 66646) (line 1259) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r780)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r781)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 66729) (line 1260) (column 79) (len 5)) (member-access (base (expression (span (offset 66729) (line 1260) (column 79) (len 3)) (ref r782))) (separator dot) (member (ref r783))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r784)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 66751) (line 1260) (column 101) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name "electricCurrentPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r785)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r786)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 66841) (line 1261) (column 86) (len 5)) (member-access (base (expression (span (offset 66841) (line 1261) (column 86) (len 3)) (ref r787))) (separator dot) (member (ref r788))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r789)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 66863) (line 1261) (column 108) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r790)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r791)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 66955) (line 1262) (column 88) (len 5)) (member-access (base (expression (span (offset 66955) (line 1262) (column 88) (len 3)) (ref r792))) (separator dot) (member (ref r793))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r794)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 66977) (line 1262) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 66978) (line 1262) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r795)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r796)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 67052) (line 1263) (column 70) (len 60)) (tuple (expression (span (offset 67053) (line 1263) (column 71) (len 6)) (ref r797)) (expression (span (offset 67061) (line 1263) (column 79) (len 10)) (ref r798)) (expression (span (offset 67073) (line 1263) (column 91) (len 17)) (ref r799)) (expression (span (offset 67092) (line 1263) (column 110) (len 19)) (ref r800))))))) (body semicolon)))))) (attribute-def (declaration-name "TransportNumberOfTheIonBValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r801)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "transportNumberOfTheIonB") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r802)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (alias (name "currentFractionOfTheIonB") (target (ref r803)) (body semicolon)) (attribute-def (declaration-name "angleOfOpticalRotation") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r804)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "MolarOpticalRotatoryPowerValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r805)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r806)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r807)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r808)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r809)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "molarOpticalRotatoryPower") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r810)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "MolarOpticalRotatoryPowerUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r811)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r812)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r813)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 69822) (line 1322) (column 77) (len 5)) (member-access (base (expression (span (offset 69822) (line 1322) (column 77) (len 3)) (ref r814))) (separator dot) (member (ref r815))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r816)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 69844) (line 1322) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "amountOfSubstancePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r817)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r818)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 69936) (line 1323) (column 88) (len 5)) (member-access (base (expression (span (offset 69936) (line 1323) (column 88) (len 3)) (ref r819))) (separator dot) (member (ref r820))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r821)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 69958) (line 1323) (column 110) (len 2)) (unary (operator "-") (operand (expression (span (offset 69959) (line 1323) (column 111) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r822)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r823)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 70033) (line 1324) (column 70) (len 31)) (tuple (expression (span (offset 70034) (line 1324) (column 71) (len 8)) (ref r824)) (expression (span (offset 70044) (line 1324) (column 81) (len 19)) (ref r825))))))) (body semicolon)))))) (attribute-def (declaration-name "SpecificOpticalRotatoryPowerValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r826)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r827)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r828)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r829)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r830)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "specificOpticalRotatoryPower") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r831)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SpecificOpticalRotatoryPowerUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r832)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r833)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r834)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 71214) (line 1348) (column 77) (len 5)) (member-access (base (expression (span (offset 71214) (line 1348) (column 77) (len 3)) (ref r835))) (separator dot) (member (ref r836))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r837)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 71236) (line 1348) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r838)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r839)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 71315) (line 1349) (column 75) (len 5)) (member-access (base (expression (span (offset 71315) (line 1349) (column 75) (len 3)) (ref r840))) (separator dot) (member (ref r841))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r842)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 71337) (line 1349) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 71338) (line 1349) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r843)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r844)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 71412) (line 1350) (column 70) (len 18)) (tuple (expression (span (offset 71413) (line 1350) (column 71) (len 8)) (ref r845)) (expression (span (offset 71423) (line 1350) (column 81) (len 6)) (ref r846))))))) (body semicolon)))))))))
)
~~~
