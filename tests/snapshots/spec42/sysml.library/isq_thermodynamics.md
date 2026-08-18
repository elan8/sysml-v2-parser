# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/ISQThermodynamics"))
~~~
# SOURCE
~~~sysml
standard library package ISQThermodynamics {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-5:2019 "Thermodynamics"
     * see also https://www.iso.org/standard/64976.html
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


    /* ISO-80000-5 item 5-1 thermodynamic temperature, temperature */
    /* See package ISQBase for the declarations of ThermodynamicTemperatureValue and ThermodynamicTemperatureUnit */

    alias TemperatureUnit for ThermodynamicTemperatureUnit;
    alias TemperatureValue for ThermodynamicTemperatureValue;
    alias temperature for thermodynamicTemperature;

    /* ISO-80000-5 item 5-2 Celsius temperature */
    attribute def CelsiusTemperatureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-2 Celsius temperature
         * symbol(s): `t`, `θ`
         * application domain: generic
         * name: CelsiusTemperature
         * quantity dimension: Θ^1
         * measurement unit(s): °C
         * tensor order: 0
         * definition: temperature difference from the thermodynamic temperature of the ice point is called the Celsius temperature t, which is defined by the quantity equation: `t = T - T_0` where `T` is thermodynamic temperature (item 5-1) and `T_0 = 273,15 K`
         * remarks: The unit degree Celsius is a special name for the kelvin for use in stating values of Celsius temperature. The unit degree Celsius is by definition equal in magnitude to the kelvin. A difference or interval of temperature may be expressed in kelvin or in degrees Celsius. The thermodynamic temperature `T_0` is 0,01 K below the thermodynamic temperature of the triple point of water. The symbol °C for the degree Celsius shall be preceded by a space (see ISO 80000-1). Prefixes are not allowed in combination with the unit °C.
         */
        attribute :>> num: Real;
        attribute :>> mRef: CelsiusTemperatureUnit[1];
    }

    attribute celsiusTemperature: CelsiusTemperatureValue[*] nonunique :> scalarQuantities;

    attribute def CelsiusTemperatureUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = thermodynamicTemperaturePF; }
    }

    /* ISO-80000-5 item 5-3.1 linear expansion coefficient */
    attribute def LinearExpansionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-3.1 linear expansion coefficient
         * symbol(s): `α_l`
         * application domain: generic
         * name: LinearExpansionCoefficient
         * quantity dimension: Θ^-1
         * measurement unit(s): K^-1
         * tensor order: 0
         * definition: relative change of length with temperature: `α_l = 1/l * (dl)/(dT)` where l is length (ISO 80000-3) and `T` is thermodynamic temperature (item 5-1)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LinearExpansionCoefficientUnit[1];
    }

    attribute linearExpansionCoefficient: LinearExpansionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def LinearExpansionCoefficientUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = thermodynamicTemperaturePF; }
    }

    /* ISO-80000-5 item 5-3.2 cubic expansion coefficient */
    attribute def CubicExpansionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-3.2 cubic expansion coefficient
         * symbol(s): `α_V`, `γ`
         * application domain: generic
         * name: CubicExpansionCoefficient
         * quantity dimension: Θ^-1
         * measurement unit(s): K^-1
         * tensor order: 0
         * definition: relative change of volume with temperature: `α_V = 1/V * (dV)/(dT)` where `V` is volume (ISO 80000-3) and `T` is thermodynamic temperature (item 5-1)
         * remarks: Also called volumetric expansion coefficient. The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num: Real;
        attribute :>> mRef: CubicExpansionCoefficientUnit[1];
    }

    attribute cubicExpansionCoefficient: CubicExpansionCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def CubicExpansionCoefficientUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = thermodynamicTemperaturePF; }
    }

    /* ISO-80000-5 item 5-3.3 relative pressure coefficient */
    attribute def RelativePressureCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-3.3 relative pressure coefficient
         * symbol(s): `α_p`
         * application domain: generic
         * name: RelativePressureCoefficient
         * quantity dimension: Θ^-1
         * measurement unit(s): K^-1
         * tensor order: 0
         * definition: relative change of pressure with temperature at constant volume: `α_p = 1/p * ((partial p)/(partial T))_V` where `p` is pressure (ISO 80000-4), `T` is thermodynamic temperature (item 5-1), and `V` is volume (ISO 80000-3)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num: Real;
        attribute :>> mRef: RelativePressureCoefficientUnit[1];
    }

    attribute relativePressureCoefficient: RelativePressureCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def RelativePressureCoefficientUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = thermodynamicTemperaturePF; }
    }

    /* ISO-80000-5 item 5-4 pressure coefficient */
    attribute def PressureCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-4 pressure coefficient
         * symbol(s): `β`
         * application domain: generic
         * name: PressureCoefficient
         * quantity dimension: L^-1*M^1*T^-2*Θ^-1
         * measurement unit(s): Pa/K, kg*m^-1*s^-2*K^-1
         * tensor order: 0
         * definition: change of pressure with temperature at constant volume: `β = ((partial p)/(partial T))_V` where `p` is pressure (ISO 80000-4), `T` is thermodynamic temperature (item 5-1), and `V` is volume (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PressureCoefficientUnit[1];
    }

    attribute pressureCoefficient: PressureCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def PressureCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-5.1 isothermal compressibility */
    attribute def IsothermalCompressibilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-5.1 isothermal compressibility
         * symbol(s): `ϰ_T`
         * application domain: generic
         * name: IsothermalCompressibility
         * quantity dimension: L^1*M^-1*T^2
         * measurement unit(s): Pa^-1, kg^-1*m*s^2
         * tensor order: 0
         * definition: negative relative change of volume with pressure at constant temperature: `ϰ_T = -1/V * ((partial V)/(partial p))_T` where `V` is volume (ISO 80000-3), `p` is pressure (ISO 80000-4), and `T` is thermodynamic temperature (item 5-1)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num: Real;
        attribute :>> mRef: IsothermalCompressibilityUnit[1];
    }

    attribute isothermalCompressibility: IsothermalCompressibilityValue[*] nonunique :> scalarQuantities;

    attribute def IsothermalCompressibilityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-5 item 5-5.2 isentropic compressibility */
    attribute def IsentropicCompressibilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-5.2 isentropic compressibility
         * symbol(s): `ϰ_S`
         * application domain: generic
         * name: IsentropicCompressibility
         * quantity dimension: L^1*M^-1*T^2
         * measurement unit(s): Pa^-1, kg^-1*m*s^2
         * tensor order: 0
         * definition: negative relative change of volume with pressure at constant entropy: `ϰ_S = -1/V * ((partial V)/(partial p))_S` where `V` is volume (ISO 80000-3), `p` is pressure (ISO 80000-4), and `S` is entropy (item 5-18)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num: Real;
        attribute :>> mRef: IsentropicCompressibilityUnit[1];
    }

    attribute isentropicCompressibility: IsentropicCompressibilityValue[*] nonunique :> scalarQuantities;

    attribute def IsentropicCompressibilityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-5 item 5-6.1 heat, amount of heat */
    attribute heat: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-6.1 heat, amount of heat
         * symbol(s): `Q`
         * application domain: generic
         * name: Heat (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference between the increase in the internal energy (item 5-20.2) of a system and the work (ISO 80000-4) done on the system, provided that the amounts of substances within the system are not changed
         * remarks: The heat transferred in an isothermal phase transformation should be expressed as the change in the appropriate state functions, e.g. `T ΔS`, where `T` is thermodynamic temperature (item 5-1) and `S` is entropy (item 5-18), or `ΔH`, where `H` is enthalpy (item 5-20.3). NOTE A supply of heat can correspond to an increase in thermodynamic temperature or to other effects, such as phase change or chemical processes; see item 5-6.2.
         */
    }

    alias amountOfHeat for heat;

    /* ISO-80000-5 item 5-6.2 latent heat */
    attribute latentHeat: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-6.2 latent heat
         * symbol(s): `Q`
         * application domain: generic
         * name: LatentHeat (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy released or absorbed by a system during a constant-temperature process
         * remarks: Examples of latent heat are latent heat of fusion (melting) and latent heat of vaporization (boiling).
         */
    }

    /* ISO-80000-5 item 5-7 heat flow rate */
    attribute def HeatFlowRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-7 heat flow rate
         * symbol(s): `dot(Q)`
         * application domain: generic
         * name: HeatFlowRate
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, J/s, kg*m^2*s^-3
         * tensor order: 0
         * definition: time rate at which heat (item 5-6.1) crosses a given surface
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: HeatFlowRateUnit[1];
    }

    attribute heatFlowRate: HeatFlowRateValue[*] nonunique :> scalarQuantities;

    attribute def HeatFlowRateUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-5 item 5-8 density of heat flow rate */
    attribute def DensityOfHeatFlowRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-8 density of heat flow rate
         * symbol(s): `q`, `φ`
         * application domain: generic
         * name: DensityOfHeatFlowRate
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: quotient of heat flow rate and area: `q = dot Q / A` where `dot Q` is heat flow rate (item 5-7) and A is area (ISO 80000-3) of a given surface
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DensityOfHeatFlowRateUnit[1];
    }

    attribute densityOfHeatFlowRate: DensityOfHeatFlowRateValue[*] nonunique :> scalarQuantities;

    attribute def DensityOfHeatFlowRateUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    /* ISO-80000-5 item 5-9 thermal conductivity */
    attribute def ThermalConductivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-9 thermal conductivity
         * symbol(s): `λ_l`, `(ϰ)`
         * application domain: generic
         * name: ThermalConductivity
         * quantity dimension: L^1*M^1*T^-3*Θ^-1
         * measurement unit(s): W/(m*K), kg*m*s^-3*K^-1
         * tensor order: 0
         * definition: quotient of density of heat flow rate (item 5-8) and thermodynamic temperature gradient that has the same direction as the heat flow
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalConductivityUnit[1];
    }

    attribute thermalConductivity: ThermalConductivityValue[*] nonunique :> scalarQuantities;

    attribute def ThermalConductivityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-10.1 coefficient of heat transfer */
    attribute def CoefficientOfHeatTransferValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-10.1 coefficient of heat transfer
         * symbol(s): `K`, `(k)`
         * application domain: generic
         * name: CoefficientOfHeatTransfer
         * quantity dimension: M^1*T^-3*Θ^-1
         * measurement unit(s): W/(m^2*K), kg*s^-3*K^-1
         * tensor order: 0
         * definition: quotient of density of heat flow rate (item 5-8) and thermodynamic temperature (item 5-1) difference
         * remarks: In building technology, the coefficient of heat transfer is often called thermal transmittance, with the symbol U (no longer recommended). See remark to item 5-13.
         */
        attribute :>> num: Real;
        attribute :>> mRef: CoefficientOfHeatTransferUnit[1];
    }

    attribute coefficientOfHeatTransfer: CoefficientOfHeatTransferValue[*] nonunique :> scalarQuantities;

    attribute def CoefficientOfHeatTransferUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-10.2 surface coefficient of heat transfer */
    attribute def SurfaceCoefficientOfHeatTransferValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-10.2 surface coefficient of heat transfer
         * symbol(s): `h`, `(α)`
         * application domain: generic
         * name: SurfaceCoefficientOfHeatTransfer
         * quantity dimension: M^1*T^-3*Θ^-1
         * measurement unit(s): W/(m^2*K), kg*s^-3*K^-1
         * tensor order: 0
         * definition: quotient of density of heat flow rate and the difference of the temperature at the surface and a reference temperature: `h = q / (T_s - T_r)` where q is density of heat flow rate (item 5-8), `T_s` is the thermodynamic temperature (item 5-1) at the surface, and `T_r` is a reference thermodynamic temperature characterizing the adjacent surroundings
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SurfaceCoefficientOfHeatTransferUnit[1];
    }

    attribute surfaceCoefficientOfHeatTransfer: SurfaceCoefficientOfHeatTransferValue[*] nonunique :> scalarQuantities;

    attribute def SurfaceCoefficientOfHeatTransferUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-11 thermal insulance, coefficient of thermal insulance */
    attribute def ThermalInsulanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-11 thermal insulance, coefficient of thermal insulance
         * symbol(s): `M`
         * application domain: generic
         * name: ThermalInsulance
         * quantity dimension: M^-1*T^3*Θ^1
         * measurement unit(s): m^2*K/W, kg^-1*s^3*K
         * tensor order: 0
         * definition: inverse of coefficient of heat transfer `K`: `M = 1/K` where `K` is coefficient of heat transfer (item 5-10.1)
         * remarks: In building technology, this quantity is often called thermal resistance, with the symbol R.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalInsulanceUnit[1];
    }

    attribute thermalInsulance: ThermalInsulanceValue[*] nonunique :> scalarQuantities;

    attribute def ThermalInsulanceUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF, thermodynamicTemperaturePF); }
    }

    alias CoefficientOfThermalInsulanceUnit for ThermalInsulanceUnit;
    alias CoefficientOfThermalInsulanceValue for ThermalInsulanceValue;
    alias coefficientOfThermalInsulance for thermalInsulance;

    /* ISO-80000-5 item 5-12 thermal resistance */
    attribute def ThermalResistanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-12 thermal resistance
         * symbol(s): `R`
         * application domain: generic
         * name: ThermalResistance
         * quantity dimension: L^-2*M^-1*T^3*Θ^1
         * measurement unit(s): K/W, kg^-1*m^-2*s^3*K
         * tensor order: 0
         * definition: quotient of thermodynamic temperature (item 5-1) difference and heat flow rate (item 5-7)
         * remarks: See remark to item 5-11.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalResistanceUnit[1];
    }

    attribute thermalResistance: ThermalResistanceValue[*] nonunique :> scalarQuantities;

    attribute def ThermalResistanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 3; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-13 thermal conductance */
    attribute def ThermalConductanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-13 thermal conductance
         * symbol(s): `G`, `(H)`
         * application domain: generic
         * name: ThermalConductance
         * quantity dimension: L^2*M^1*T^-3*Θ^-1
         * measurement unit(s): W/K, kg*m^2*s^-3*K^-1
         * tensor order: 0
         * definition: inverse of thermal resistance `R`: `G = 1/R` where `R` is thermal resistance (item 5-12)
         * remarks: See remark to item 5-11. This quantity is also called heat transfer coefficient. See item 5-10.1.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalConductanceUnit[1];
    }

    attribute thermalConductance: ThermalConductanceValue[*] nonunique :> scalarQuantities;

    attribute def ThermalConductanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-14 thermal diffusivity */
    attribute def ThermalDiffusivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-14 thermal diffusivity
         * symbol(s): `a`
         * application domain: generic
         * name: ThermalDiffusivity
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: quotient of thermal conductivity and the product of mass density and specific heat capacity: `a = λ / (ρ C_p)` where `λ` is thermal conductivity (item 5-9), `ρ` is mass density (ISO 80000-4), and `c_p` is specific heat capacity at constant pressure (item 5-16.2)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: ThermalDiffusivityUnit[1];
    }

    attribute thermalDiffusivity: ThermalDiffusivityValue[*] nonunique :> scalarQuantities;

    attribute def ThermalDiffusivityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-5 item 5-15 heat capacity */
    attribute def HeatCapacityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-15 heat capacity
         * symbol(s): `C`
         * application domain: generic
         * name: HeatCapacity
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: derivative of added heat with respect to thermodynamic temperature of a system: `C = (dQ)/(dT)` where `Q` is amount of heat (item 5-6.1) and `T` is thermodynamic temperature (item 5-1)
         * remarks: Heat capacity is not completely defined unless specified as seen in items 5-16.2, 5-16.3 and 5-16.4.
         */
        attribute :>> num: Real;
        attribute :>> mRef: HeatCapacityUnit[1];
    }

    attribute heatCapacity: HeatCapacityValue[*] nonunique :> scalarQuantities;

    attribute def HeatCapacityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-16.1 specific heat capacity */
    attribute def SpecificHeatCapacityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-16.1 specific heat capacity
         * symbol(s): `c`
         * application domain: generic
         * name: SpecificHeatCapacity
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of heat capacity and mass: `c = C/m` where `C` is heat capacity (item 5-15) and `m` is mass (ISO 80000-4)
         * remarks: For the corresponding quantities related to the amount of substance, see ISO 80000-9.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificHeatCapacityUnit[1];
    }

    attribute specificHeatCapacity: SpecificHeatCapacityValue[*] nonunique :> scalarQuantities;

    attribute def SpecificHeatCapacityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-16.2 specific heat capacity at constant pressure */
    attribute def SpecificHeatCapacityAtConstantPressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-16.2 specific heat capacity at constant pressure
         * symbol(s): `c_p`
         * application domain: generic
         * name: SpecificHeatCapacityAtConstantPressure
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: specific heat capacity (item 5-16.1) at constant pressure (ISO 80000-4)
         * remarks: Also called specific isobaric heat capacity.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificHeatCapacityAtConstantPressureUnit[1];
    }

    attribute specificHeatCapacityAtConstantPressure: SpecificHeatCapacityAtConstantPressureValue[*] nonunique :> scalarQuantities;

    attribute def SpecificHeatCapacityAtConstantPressureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-16.3 specific heat capacity at constant volume */
    attribute def SpecificHeatCapacityAtConstantVolumeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-16.3 specific heat capacity at constant volume
         * symbol(s): `c_V`
         * application domain: generic
         * name: SpecificHeatCapacityAtConstantVolume
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: specific heat capacity (item 5-16.1) at constant volume (ISO 80000-3)
         * remarks: Also called specific isochoric heat capacity.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificHeatCapacityAtConstantVolumeUnit[1];
    }

    attribute specificHeatCapacityAtConstantVolume: SpecificHeatCapacityAtConstantVolumeValue[*] nonunique :> scalarQuantities;

    attribute def SpecificHeatCapacityAtConstantVolumeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-16.4 specific heat capacity at saturated vapour pressure */
    attribute def SpecificHeatCapacityAtSaturatedVapourPressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-16.4 specific heat capacity at saturated vapour pressure
         * symbol(s): `c_"sat"`
         * application domain: generic
         * name: SpecificHeatCapacityAtSaturatedVapourPressure
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: specific heat capacity (item 5-16.1) at saturated vapour pressure (ISO 80000-4)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificHeatCapacityAtSaturatedVapourPressureUnit[1];
    }

    attribute specificHeatCapacityAtSaturatedVapourPressure: SpecificHeatCapacityAtSaturatedVapourPressureValue[*] nonunique :> scalarQuantities;

    attribute def SpecificHeatCapacityAtSaturatedVapourPressureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-17.1 ratio of specific heat capacities */
    attribute def RatioOfSpecificHeatCapacitiesValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-17.1 ratio of specific heat capacities
         * symbol(s): `γ`
         * application domain: generic
         * name: RatioOfSpecificHeatCapacities (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of specific heat capacity at constant pressure and specific heat capacity at constant volume: `γ = c_p/c_V` where `c_p` is specific heat capacity at constant pressure (item 5-16.2) and `c_V` is specific heat capacity at constant volume (item 5-16.3)
         * remarks: This quantity can also be expressed by `γ = C_p/C_V` where `C_p` is heat capacity at constant pressure and `C_V` is heat capacity at constant volume.
         */
    }
    attribute ratioOfSpecificHeatCapacities: RatioOfSpecificHeatCapacitiesValue :> scalarQuantities;

    /* ISO-80000-5 item 5-17.2 isentropic exponent, isentropic expansion factor */
    attribute def IsentropicExponentValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-17.2 isentropic exponent, isentropic expansion factor
         * symbol(s): `ϰ`
         * application domain: generic
         * name: IsentropicExponent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: the negative of relative pressure change, divided by relative volume change, at constant entropy: `ϰ = -V/p * ((partial p)/(partial V))_S` where `V` is volume (ISO 80000-3), `p` is pressure (ISO 80000-4), and `S` is entropy (item 5-18)
         * remarks: For an ideal gas, `ϰ` is equal to `γ` (item 5-17.1).
         */
    }
    attribute isentropicExponent: IsentropicExponentValue :> scalarQuantities;

    alias isentropicExpansionFactor for isentropicExponent;

    /* ISO-80000-5 item 5-18 entropy */
    attribute def EntropyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-18 entropy
         * symbol(s): `S`
         * application domain: generic
         * name: Entropy
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: natural logarithm of number of equally probable microscopic configurations in a macroscopic system, multiplied by the Boltzmann constant: `S = k lnW` where `W` is number of configurations and `k` is the Boltzmann constant (ISO 80000-1)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EntropyUnit[1];
    }

    attribute entropy: EntropyValue[*] nonunique :> scalarQuantities;

    attribute def EntropyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-19 specific entropy */
    attribute def SpecificEntropyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-19 specific entropy
         * symbol(s): `s`
         * application domain: generic
         * name: SpecificEntropy
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of entropy and mass: `s = S/m` where `S` is entropy (item 5-18) and `m` is mass (ISO 80000-4)
         * remarks: For the corresponding quantity related to amount of substance, see ISO 80000-9.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificEntropyUnit[1];
    }

    attribute specificEntropy: SpecificEntropyValue[*] nonunique :> scalarQuantities;

    attribute def SpecificEntropyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-20.1 energy */
    attribute def EnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-20.1 energy
         * symbol(s): `E`
         * application domain: thermodynamics
         * name: Energy
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: ability of a system to do work (ISO 80000-4)
         * remarks: Energy exists in different forms that are mutually transformable into each other, either totally or partially. In contrast to internal energy (item 5-20.2), energy is not a state function.
         */
        attribute :>> num: Real;
        attribute :>> mRef: EnergyUnit[1];
    }

    attribute energy: EnergyValue[*] nonunique :> scalarQuantities;

    attribute def EnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-5 item 5-20.2 internal energy, thermodynamic energy */
    attribute internalEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-20.2 internal energy, thermodynamic energy
         * symbol(s): `U`
         * application domain: generic
         * name: InternalEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy of a system whose change is given by the amount of the heat (item 5-6.1) transferred to the system and the work (ISO 80000-4) done on the system, provided that the system is closed and no chemical reactions occur
         * remarks: In thermodynamic text books, usually the formula `ΔU = Q + W` is used. Note that the zero of the energy is undefined.
         */
    }

    alias thermodynamicEnergy for internalEnergy;

    /* ISO-80000-5 item 5-20.3 enthalpy */
    attribute enthalpy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-20.3 enthalpy
         * symbol(s): `H`
         * application domain: generic
         * name: Enthalpy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of internal energy of the system and the product of pressure and volume of the system: `H = U + p*V` where U is internal energy (item 5-20.2), `p` is pressure (ISO 80000-4), and `V` is volume (ISO 80000-3)
         * remarks: None.
         */
    }

    /* ISO-80000-5 item 5-20.4 Helmholtz energy, Helmholtz function */
    attribute helmholtzEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-20.4 Helmholtz energy, Helmholtz function
         * symbol(s): `A`, `F`
         * application domain: generic
         * name: HelmholtzEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference of internal energy of the system and the product of thermodynamic temperature and entropy of the system: `A = U - TS` where `U` is internal energy (item 5-20.2), `T` is thermodynamic temperature (item 5-1), and `S` is entropy (item 5-18)
         * remarks: The name Helmholtz free energy is also used. However, this term is not recommended.
         */
    }

    alias helmholtzFunction for helmholtzEnergy;

    /* ISO-80000-5 item 5-20.5 Gibbs energy, Gibbs function */
    attribute gibbsEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-20.5 Gibbs energy, Gibbs function
         * symbol(s): `G`
         * application domain: generic
         * name: GibbsEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference of the enthalpy and the product of thermodynamic temperature and entropy of the system: `G = H - T*S` where H is enthalpy (item 5-20.3), `T` is thermodynamic temperature (item 5-1), and `S` is entropy (item 5-18)
         * remarks: The name Gibbs free energy is also used. However, this term is not recommended.
         */
    }

    alias gibbsFunction for gibbsEnergy;

    /* ISO-80000-5 item 5-21.1 specific energy */
    attribute def SpecificEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-21.1 specific energy
         * symbol(s): `e`
         * application domain: generic
         * name: SpecificEnergy
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of energy and mass: `e = E/m` where `E` is energy (item 5-20.1) and `m` is mass (ISO 80000-4)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificEnergyUnit[1];
    }

    attribute specificEnergy: SpecificEnergyValue[*] nonunique :> scalarQuantities;

    attribute def SpecificEnergyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-5 item 5-21.2 specific internal energy, specific thermodynamic energy */
    attribute specificInternalEnergy: SpecificEnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-21.2 specific internal energy, specific thermodynamic energy
         * symbol(s): `u`
         * application domain: generic
         * name: SpecificInternalEnergy (specializes SpecificEnergy)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of internal energy and mass: `u = U/m` where `U` is internal energy (item 5-20.2) and `m` is mass (ISO 80000-4)
         * remarks: None.
         */
    }

    alias specificThermodynamicEnergy for specificInternalEnergy;

    /* ISO-80000-5 item 5-21.3 specific enthalpy */
    attribute def SpecificEnthalpyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-21.3 specific enthalpy
         * symbol(s): `h`
         * application domain: generic
         * name: SpecificEnthalpy
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of enthalpy and mass: `h = H/m` where `H` is enthalpy (item 5-20.3) and `m` is mass (ISO 80000-4)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificEnthalpyUnit[1];
    }

    attribute specificEnthalpy: SpecificEnthalpyValue[*] nonunique :> scalarQuantities;

    attribute def SpecificEnthalpyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-5 item 5-21.4 specific Helmholtz energy, specific Helmholtz function */
    attribute specificHelmholtzEnergy: SpecificEnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-21.4 specific Helmholtz energy, specific Helmholtz function
         * symbol(s): `a`, `f`
         * application domain: generic
         * name: SpecificHelmholtzEnergy (specializes SpecificEnergy)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of Helmholtz energy and mass: `a = A/m` where A is Helmholtz energy (item 5-20.4) and m is mass (ISO 80000-4)
         * remarks: The name specific Helmholtz free energy is also used. However, this term is not recommended.
         */
    }

    alias specificHelmholtzFunction for specificHelmholtzEnergy;

    /* ISO-80000-5 item 5-21.5 specific Gibbs energy, specific Gibbs function */
    attribute specificGibbsEnergy: SpecificEnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 5-21.5 specific Gibbs energy, specific Gibbs function
         * symbol(s): `g`
         * application domain: generic
         * name: SpecificGibbsEnergy (specializes SpecificEnergy)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of Gibbs energy and mass: `g = G/m` where `G` is Gibbs energy (item 5-20.5) and `m` is mass (ISO 80000-4)
         * remarks: The name specific Gibbs free energy is also used. However, this term is not recommended.
         */
    }

    alias specificGibbsFunction for specificGibbsEnergy;

    /* ISO-80000-5 item 5-22 Massieu function */
    attribute def MassieuFunctionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-22 Massieu function
         * symbol(s): `J`
         * application domain: generic
         * name: MassieuFunction
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of the negative of Helmholtz energy and temperature: `J = -A/T` where `A` is Helmholtz energy (item 5-20.4) and `T` is thermodynamic temperature (item 5-1)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassieuFunctionUnit[1];
    }

    attribute massieuFunction: MassieuFunctionValue[*] nonunique :> scalarQuantities;

    attribute def MassieuFunctionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-23 Planck function */
    attribute def PlanckFunctionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-23 Planck function
         * symbol(s): `Y`
         * application domain: generic
         * name: PlanckFunction
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of the negative of Gibbs energy and temperature: `Y = -G/T` where G is Gibbs energy (item 5-20.5) and `T` is thermodynamic temperature (item 5-1)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PlanckFunctionUnit[1];
    }

    attribute planckFunction: PlanckFunctionValue[*] nonunique :> scalarQuantities;

    attribute def PlanckFunctionUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-24 Joule-Thomson coefficient */
    attribute def JouleThomsonCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-24 Joule-Thomson coefficient
         * symbol(s): `μ_"JT"`
         * application domain: generic
         * name: JouleThomsonCoefficient
         * quantity dimension: L^1*M^-1*T^2*Θ^1
         * measurement unit(s): K/Pa, kg^-1*m*s^2*K
         * tensor order: 0
         * definition: change of thermodynamic temperature with respect to pressure in a Joule-Thomson process at constant enthalpy: `μ_(JT) = ((partial T)/(partial p))_H` where `T` is thermodynamic temperature (item 5-1), `p` is pressure (ISO 80000-4) and H is enthalpy (item 5-20.3)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: JouleThomsonCoefficientUnit[1];
    }

    attribute jouleThomsonCoefficient: JouleThomsonCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def JouleThomsonCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = -1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = 2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-25.1 thermal efficiency */
    attribute def ThermalEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-25.1 thermal efficiency
         * symbol(s): `η`
         * application domain: thermodynamics
         * name: ThermalEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of work (ISO 80000-4) delivered by a heat engine and supplied heat: `η = W/Q` where `W` is work (ISO 80000-4) and `Q` is heat (item 5-6.1)
         * remarks: None.
         */
    }
    attribute thermalEfficiency: ThermalEfficiencyValue :> scalarQuantities;

    /* ISO-80000-5 item 5-25.2 maximum thermal efficiency */
    attribute def MaximumThermalEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-25.2 maximum thermal efficiency
         * symbol(s): `η_"max"`
         * application domain: generic
         * name: MaximumThermalEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: efficiency determined by the quotient of the temperatures of the hot source and the cold sink: `η_max = 1 - T_c/T_h` where `T_c` is the thermodynamic temperature (item 5-1) of the cold sink and `T_h` is the thermodynamic temperature (item 5-1) of the hot source
         * remarks: An ideal heat engine operating according to the Carnot process is delivering the maximum efficiency.
         */
    }
    attribute maximumThermalEfficiency: MaximumThermalEfficiencyValue :> scalarQuantities;

    /* ISO-80000-5 item 5-26 specific gas constant */
    attribute def SpecificGasConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-26 specific gas constant
         * symbol(s): `R_s`
         * application domain: generic
         * name: SpecificGasConstant
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of the Boltzmann constant `k` (ISO 80000-1) and the mass `m` (ISO 80000-4) of the gas particle
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpecificGasConstantUnit[1];
    }

    attribute specificGasConstant: SpecificGasConstantValue[*] nonunique :> scalarQuantities;

    attribute def SpecificGasConstantUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        private attribute thermodynamicTemperaturePF: QuantityPowerFactor[1] { :>> quantity = isq.'Θ'; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF); }
    }

    /* ISO-80000-5 item 5-27 mass concentration of water */
    attribute def MassConcentrationOfWaterValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-27 mass concentration of water
         * symbol(s): `w`
         * application domain: generic
         * name: MassConcentrationOfWater
         * quantity dimension: L^-3*M^1
         * measurement unit(s): kg*m^-3
         * tensor order: 0
         * definition: quotient of mass of water and a specified volume: `w = m/V` where `m` is mass (ISO 80000-4) of water, irrespective of the form of aggregation state, and `V` is volume (ISO 80000-3)
         * remarks: Mass concentration of water at saturation is denoted `w_"sat"`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassConcentrationOfWaterUnit[1];
    }

    attribute massConcentrationOfWater: MassConcentrationOfWaterValue[*] nonunique :> scalarQuantities;

    attribute def MassConcentrationOfWaterUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-5 item 5-28 mass concentration of water vapour absolute humidity */
    attribute def MassConcentrationOfWaterVapourAbsoluteHumidityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-28 mass concentration of water vapour absolute humidity
         * symbol(s): `v`
         * application domain: generic
         * name: MassConcentrationOfWaterVapourAbsoluteHumidity
         * quantity dimension: L^-3*M^1
         * measurement unit(s): kg*m^-3
         * tensor order: 0
         * definition: quotient of mass of water vapour and a specified volume: `v = m/V` where m is mass (ISO 80000-4) of water vapour and `V` is volume (ISO 80000-3)
         * remarks: Mass concentration of water vapour at saturation is denoted `v_"sat"`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: MassConcentrationOfWaterVapourAbsoluteHumidityUnit[1];
    }

    attribute massConcentrationOfWaterVapourAbsoluteHumidity: MassConcentrationOfWaterVapourAbsoluteHumidityValue[*] nonunique :> scalarQuantities;

    attribute def MassConcentrationOfWaterVapourAbsoluteHumidityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF); }
    }

    /* ISO-80000-5 item 5-29 mass ratio of water to dry matter */
    attribute def MassRatioOfWaterToDryMatterValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-29 mass ratio of water to dry matter
         * symbol(s): `u`
         * application domain: generic
         * name: MassRatioOfWaterToDryMatter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass of water and mass of dry matter: `u = m/m_d` where `m` is mass (ISO 80000-4) of water and `m_d` is mass of dry matter
         * remarks: Mass ratio of water to dry matter at saturation is denoted `u_"sat"`.
         */
    }
    attribute massRatioOfWaterToDryMatter: MassRatioOfWaterToDryMatterValue :> scalarQuantities;

    /* ISO-80000-5 item 5-30 mass ratio of water vapour to dry gas */
    attribute def MassRatioOfWaterVapourToDryGasValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-30 mass ratio of water vapour to dry gas
         * symbol(s): `r`, `(x)`
         * application domain: generic
         * name: MassRatioOfWaterVapourToDryGas (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass of water vapour and mass of dry gas: `r = m/m_d` where `m` is mass (ISO 80000-4) of water vapour and `m_d` is mass of dry gas
         * remarks: Mass ratio of water vapour to dry gas at saturation is denoted `r_"sat"`. Mass ratio of water vapour to dry gas is also called mixing ratio.
         */
    }
    attribute massRatioOfWaterVapourToDryGas: MassRatioOfWaterVapourToDryGasValue :> scalarQuantities;

    /* ISO-80000-5 item 5-31 mass fraction of water */
    attribute def MassFractionOfWaterValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-31 mass fraction of water
         * symbol(s): `w_(H_(2)O)`
         * application domain: generic
         * name: MassFractionOfWater (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `w_(H_(2)O) = u/(1+u)` where `u` is mass ratio of water to dry matter (item 5-29)
         * remarks: None.
         */
    }
    attribute massFractionOfWater: MassFractionOfWaterValue :> scalarQuantities;

    /* ISO-80000-5 item 5-32 mass fraction of dry matter */
    attribute def MassFractionOfDryMatterValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-32 mass fraction of dry matter
         * symbol(s): `w_d`
         * application domain: generic
         * name: MassFractionOfDryMatter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `w_d = 1 - w_(H_(2)O)` where `w_(H_(2)O)` is mass fraction of water (item 5-31)
         * remarks: None.
         */
    }
    attribute massFractionOfDryMatter: MassFractionOfDryMatterValue :> scalarQuantities;

    /* ISO-80000-5 item 5-33 relative humidity */
    attribute def RelativeHumidityValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-33 relative humidity
         * symbol(s): `φ`
         * application domain: generic
         * name: RelativeHumidity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of partial pressure of water vapour and partial pressure at its saturation: `φ = p/p_"sat"` where `p` is partial pressure (ISO 80000-4) of vapour and `p_"sat"` is its partial pressure at saturation at the same temperature
         * remarks: Relative humidity is often referred to as RH and expressed in percent. See also remark in item 5-35.
         */
    }
    attribute relativeHumidity: RelativeHumidityValue :> scalarQuantities;

    /* ISO-80000-5 item 5-34 relative mass concentration of vapour */
    attribute def RelativeMassConcentrationOfVapourValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-34 relative mass concentration of vapour
         * symbol(s): `φ`
         * application domain: generic
         * name: RelativeMassConcentrationOfVapour (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass concentration of water vapour and mass concentration at its saturation: `φ = v/v_"sat"` where `v` is mass concentration of water vapour (item 5-28) and `v_"sat"` is its mass concentration of water vapour at saturation of the same temperature
         * remarks: For water vapour concentrations up to 1 kg/m^3, the relative humidity (item 5-33) is assumed to be equal to relative mass concentration of vapour. For details see Reference [8].
         */
    }
    attribute relativeMassConcentrationOfVapour: RelativeMassConcentrationOfVapourValue :> scalarQuantities;

    /* ISO-80000-5 item 5-35 relative mass ratio of vapour */
    attribute def RelativeMassRatioOfVapourValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-35 relative mass ratio of vapour
         * symbol(s): `ψ`
         * application domain: generic
         * name: RelativeMassRatioOfVapour (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass ratio of water vapour to dry gas and mass ratio of water vapour to dry gas at saturation: `ψ = r/r_"sat"` where `r` is mass ratio of water vapour to dry gas (item 5-30) and `r_"sat"` is its mass ratio of water vapour to dry gas at saturation of the same temperature
         * remarks: This quantity is also used as an approximation of relative humidity (item 5-33).
         */
    }
    attribute relativeMassRatioOfVapour: RelativeMassRatioOfVapourValue :> scalarQuantities;

    /* ISO-80000-5 item 5-36 dew-point temperature */
    attribute dewPointTemperature: ThermodynamicTemperatureValue :> scalarQuantities {
        doc
        /*
         * source: item 5-36 dew-point temperature
         * symbol(s): `T_d`
         * application domain: generic
         * name: DewPointTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: temperature at which water vapour in the air reaches saturation under isobaric conditions
         * remarks: The corresponding Celsius temperature, denoted `t_d`, is still called dew-point temperature. The unit for the corresponding Celsius temperature is degree Celsius, symbol °C.
         */
    }

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_thermodynamics.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package ISQThermodynamics {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-5:2019 "Thermodynamics"
     * see also https://www.iso.org/standard/64976.html
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
    alias TemperatureUnit for ThermodynamicTemperatureUnit;
    alias TemperatureValue for ThermodynamicTemperatureValue;
    alias temperature for thermodynamicTemperature;
    attribute def CelsiusTemperatureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-2 Celsius temperature
         * symbol(s): `t`, `θ`
         * application domain: generic
         * name: CelsiusTemperature
         * quantity dimension: Θ^1
         * measurement unit(s): °C
         * tensor order: 0
         * definition: temperature difference from the thermodynamic temperature of the ice point is called the Celsius temperature t, which is defined by the quantity equation: `t = T - T_0` where `T` is thermodynamic temperature (item 5-1) and `T_0 = 273,15 K`
         * remarks: The unit degree Celsius is a special name for the kelvin for use in stating values of Celsius temperature. The unit degree Celsius is by definition equal in magnitude to the kelvin. A difference or interval of temperature may be expressed in kelvin or in degrees Celsius. The thermodynamic temperature `T_0` is 0,01 K below the thermodynamic temperature of the triple point of water. The symbol °C for the degree Celsius shall be preceded by a space (see ISO 80000-1). Prefixes are not allowed in combination with the unit °C.
         */
        attribute :>> num : Real;
        attribute :>> mRef : CelsiusTemperatureUnit[1];
    }
    attribute def celsiusTemperature : CelsiusTemperatureValue[*] nonunique;
    attribute def CelsiusTemperatureUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = thermodynamicTemperaturePF;
        }
    }
    attribute def LinearExpansionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-3.1 linear expansion coefficient
         * symbol(s): `α_l`
         * application domain: generic
         * name: LinearExpansionCoefficient
         * quantity dimension: Θ^-1
         * measurement unit(s): K^-1
         * tensor order: 0
         * definition: relative change of length with temperature: `α_l = 1/l * (dl)/(dT)` where l is length (ISO 80000-3) and `T` is thermodynamic temperature (item 5-1)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LinearExpansionCoefficientUnit[1];
    }
    attribute def linearExpansionCoefficient : LinearExpansionCoefficientValue[*] nonunique;
    attribute def LinearExpansionCoefficientUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = thermodynamicTemperaturePF;
        }
    }
    attribute def CubicExpansionCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-3.2 cubic expansion coefficient
         * symbol(s): `α_V`, `γ`
         * application domain: generic
         * name: CubicExpansionCoefficient
         * quantity dimension: Θ^-1
         * measurement unit(s): K^-1
         * tensor order: 0
         * definition: relative change of volume with temperature: `α_V = 1/V * (dV)/(dT)` where `V` is volume (ISO 80000-3) and `T` is thermodynamic temperature (item 5-1)
         * remarks: Also called volumetric expansion coefficient. The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num : Real;
        attribute :>> mRef : CubicExpansionCoefficientUnit[1];
    }
    attribute def cubicExpansionCoefficient : CubicExpansionCoefficientValue[*] nonunique;
    attribute def CubicExpansionCoefficientUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = thermodynamicTemperaturePF;
        }
    }
    attribute def RelativePressureCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-3.3 relative pressure coefficient
         * symbol(s): `α_p`
         * application domain: generic
         * name: RelativePressureCoefficient
         * quantity dimension: Θ^-1
         * measurement unit(s): K^-1
         * tensor order: 0
         * definition: relative change of pressure with temperature at constant volume: `α_p = 1/p * ((partial p)/(partial T))_V` where `p` is pressure (ISO 80000-4), `T` is thermodynamic temperature (item 5-1), and `V` is volume (ISO 80000-3)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num : Real;
        attribute :>> mRef : RelativePressureCoefficientUnit[1];
    }
    attribute def relativePressureCoefficient : RelativePressureCoefficientValue[*] nonunique;
    attribute def RelativePressureCoefficientUnit :> DerivedUnit {
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = thermodynamicTemperaturePF;
        }
    }
    attribute def PressureCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-4 pressure coefficient
         * symbol(s): `β`
         * application domain: generic
         * name: PressureCoefficient
         * quantity dimension: L^-1*M^1*T^-2*Θ^-1
         * measurement unit(s): Pa/K, kg*m^-1*s^-2*K^-1
         * tensor order: 0
         * definition: change of pressure with temperature at constant volume: `β = ((partial p)/(partial T))_V` where `p` is pressure (ISO 80000-4), `T` is thermodynamic temperature (item 5-1), and `V` is volume (ISO 80000-3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PressureCoefficientUnit[1];
    }
    attribute def pressureCoefficient : PressureCoefficientValue[*] nonunique;
    attribute def PressureCoefficientUnit :> DerivedUnit {
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
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def IsothermalCompressibilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-5.1 isothermal compressibility
         * symbol(s): `ϰ_T`
         * application domain: generic
         * name: IsothermalCompressibility
         * quantity dimension: L^1*M^-1*T^2
         * measurement unit(s): Pa^-1, kg^-1*m*s^2
         * tensor order: 0
         * definition: negative relative change of volume with pressure at constant temperature: `ϰ_T = -1/V * ((partial V)/(partial p))_T` where `V` is volume (ISO 80000-3), `p` is pressure (ISO 80000-4), and `T` is thermodynamic temperature (item 5-1)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num : Real;
        attribute :>> mRef : IsothermalCompressibilityUnit[1];
    }
    attribute def isothermalCompressibility : IsothermalCompressibilityValue[*] nonunique;
    attribute def IsothermalCompressibilityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 1;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def IsentropicCompressibilityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-5.2 isentropic compressibility
         * symbol(s): `ϰ_S`
         * application domain: generic
         * name: IsentropicCompressibility
         * quantity dimension: L^1*M^-1*T^2
         * measurement unit(s): Pa^-1, kg^-1*m*s^2
         * tensor order: 0
         * definition: negative relative change of volume with pressure at constant entropy: `ϰ_S = -1/V * ((partial V)/(partial p))_S` where `V` is volume (ISO 80000-3), `p` is pressure (ISO 80000-4), and `S` is entropy (item 5-18)
         * remarks: The subscripts in the symbols may be omitted when there is no risk of confusion.
         */
        attribute :>> num : Real;
        attribute :>> mRef : IsentropicCompressibilityUnit[1];
    }
    attribute def isentropicCompressibility : IsentropicCompressibilityValue[*] nonunique;
    attribute def IsentropicCompressibilityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 1;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def heat : EnergyValue {
        doc
        /*
         * source: item 5-6.1 heat, amount of heat
         * symbol(s): `Q`
         * application domain: generic
         * name: Heat (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference between the increase in the internal energy (item 5-20.2) of a system and the work (ISO 80000-4) done on the system, provided that the amounts of substances within the system are not changed
         * remarks: The heat transferred in an isothermal phase transformation should be expressed as the change in the appropriate state functions, e.g. `T ΔS`, where `T` is thermodynamic temperature (item 5-1) and `S` is entropy (item 5-18), or `ΔH`, where `H` is enthalpy (item 5-20.3). NOTE A supply of heat can correspond to an increase in thermodynamic temperature or to other effects, such as phase change or chemical processes; see item 5-6.2.
         */
    }
    alias amountOfHeat for heat;
    attribute def latentHeat : EnergyValue {
        doc
        /*
         * source: item 5-6.2 latent heat
         * symbol(s): `Q`
         * application domain: generic
         * name: LatentHeat (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy released or absorbed by a system during a constant-temperature process
         * remarks: Examples of latent heat are latent heat of fusion (melting) and latent heat of vaporization (boiling).
         */
    }
    attribute def HeatFlowRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-7 heat flow rate
         * symbol(s): `dot(Q)`
         * application domain: generic
         * name: HeatFlowRate
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, J/s, kg*m^2*s^-3
         * tensor order: 0
         * definition: time rate at which heat (item 5-6.1) crosses a given surface
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : HeatFlowRateUnit[1];
    }
    attribute def heatFlowRate : HeatFlowRateValue[*] nonunique;
    attribute def HeatFlowRateUnit :> DerivedUnit {
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
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def DensityOfHeatFlowRateValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-8 density of heat flow rate
         * symbol(s): `q`, `φ`
         * application domain: generic
         * name: DensityOfHeatFlowRate
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: quotient of heat flow rate and area: `q = dot Q / A` where `dot Q` is heat flow rate (item 5-7) and A is area (ISO 80000-3) of a given surface
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DensityOfHeatFlowRateUnit[1];
    }
    attribute def densityOfHeatFlowRate : DensityOfHeatFlowRateValue[*] nonunique;
    attribute def DensityOfHeatFlowRateUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF);
        }
    }
    attribute def ThermalConductivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-9 thermal conductivity
         * symbol(s): `λ_l`, `(ϰ)`
         * application domain: generic
         * name: ThermalConductivity
         * quantity dimension: L^1*M^1*T^-3*Θ^-1
         * measurement unit(s): W/(m*K), kg*m*s^-3*K^-1
         * tensor order: 0
         * definition: quotient of density of heat flow rate (item 5-8) and thermodynamic temperature gradient that has the same direction as the heat flow
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThermalConductivityUnit[1];
    }
    attribute def thermalConductivity : ThermalConductivityValue[*] nonunique;
    attribute def ThermalConductivityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 1;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -3;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def CoefficientOfHeatTransferValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-10.1 coefficient of heat transfer
         * symbol(s): `K`, `(k)`
         * application domain: generic
         * name: CoefficientOfHeatTransfer
         * quantity dimension: M^1*T^-3*Θ^-1
         * measurement unit(s): W/(m^2*K), kg*s^-3*K^-1
         * tensor order: 0
         * definition: quotient of density of heat flow rate (item 5-8) and thermodynamic temperature (item 5-1) difference
         * remarks: In building technology, the coefficient of heat transfer is often called thermal transmittance, with the symbol U (no longer recommended). See remark to item 5-13.
         */
        attribute :>> num : Real;
        attribute :>> mRef : CoefficientOfHeatTransferUnit[1];
    }
    attribute def coefficientOfHeatTransfer : CoefficientOfHeatTransferValue[*] nonunique;
    attribute def CoefficientOfHeatTransferUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -3;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def SurfaceCoefficientOfHeatTransferValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-10.2 surface coefficient of heat transfer
         * symbol(s): `h`, `(α)`
         * application domain: generic
         * name: SurfaceCoefficientOfHeatTransfer
         * quantity dimension: M^1*T^-3*Θ^-1
         * measurement unit(s): W/(m^2*K), kg*s^-3*K^-1
         * tensor order: 0
         * definition: quotient of density of heat flow rate and the difference of the temperature at the surface and a reference temperature: `h = q / (T_s - T_r)` where q is density of heat flow rate (item 5-8), `T_s` is the thermodynamic temperature (item 5-1) at the surface, and `T_r` is a reference thermodynamic temperature characterizing the adjacent surroundings
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SurfaceCoefficientOfHeatTransferUnit[1];
    }
    attribute def surfaceCoefficientOfHeatTransfer : SurfaceCoefficientOfHeatTransferValue[*] nonunique;
    attribute def SurfaceCoefficientOfHeatTransferUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -3;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def ThermalInsulanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-11 thermal insulance, coefficient of thermal insulance
         * symbol(s): `M`
         * application domain: generic
         * name: ThermalInsulance
         * quantity dimension: M^-1*T^3*Θ^1
         * measurement unit(s): m^2*K/W, kg^-1*s^3*K
         * tensor order: 0
         * definition: inverse of coefficient of heat transfer `K`: `M = 1/K` where `K` is coefficient of heat transfer (item 5-10.1)
         * remarks: In building technology, this quantity is often called thermal resistance, with the symbol R.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThermalInsulanceUnit[1];
    }
    attribute def thermalInsulance : ThermalInsulanceValue[*] nonunique;
    attribute def ThermalInsulanceUnit :> DerivedUnit {
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 3;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (massPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    alias CoefficientOfThermalInsulanceUnit for ThermalInsulanceUnit;
    alias CoefficientOfThermalInsulanceValue for ThermalInsulanceValue;
    alias coefficientOfThermalInsulance for thermalInsulance;
    attribute def ThermalResistanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-12 thermal resistance
         * symbol(s): `R`
         * application domain: generic
         * name: ThermalResistance
         * quantity dimension: L^-2*M^-1*T^3*Θ^1
         * measurement unit(s): K/W, kg^-1*m^-2*s^3*K
         * tensor order: 0
         * definition: quotient of thermodynamic temperature (item 5-1) difference and heat flow rate (item 5-7)
         * remarks: See remark to item 5-11.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThermalResistanceUnit[1];
    }
    attribute def thermalResistance : ThermalResistanceValue[*] nonunique;
    attribute def ThermalResistanceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 3;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def ThermalConductanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-13 thermal conductance
         * symbol(s): `G`, `(H)`
         * application domain: generic
         * name: ThermalConductance
         * quantity dimension: L^2*M^1*T^-3*Θ^-1
         * measurement unit(s): W/K, kg*m^2*s^-3*K^-1
         * tensor order: 0
         * definition: inverse of thermal resistance `R`: `G = 1/R` where `R` is thermal resistance (item 5-12)
         * remarks: See remark to item 5-11. This quantity is also called heat transfer coefficient. See item 5-10.1.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThermalConductanceUnit[1];
    }
    attribute def thermalConductance : ThermalConductanceValue[*] nonunique;
    attribute def ThermalConductanceUnit :> DerivedUnit {
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
            attribute :>> exponent = -3;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def ThermalDiffusivityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-14 thermal diffusivity
         * symbol(s): `a`
         * application domain: generic
         * name: ThermalDiffusivity
         * quantity dimension: L^2*T^-1
         * measurement unit(s): m^2*s^-1
         * tensor order: 0
         * definition: quotient of thermal conductivity and the product of mass density and specific heat capacity: `a = λ / (ρ C_p)` where `λ` is thermal conductivity (item 5-9), `ρ` is mass density (ISO 80000-4), and `c_p` is specific heat capacity at constant pressure (item 5-16.2)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : ThermalDiffusivityUnit[1];
    }
    attribute def thermalDiffusivity : ThermalDiffusivityValue[*] nonunique;
    attribute def ThermalDiffusivityUnit :> DerivedUnit {
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
    attribute def HeatCapacityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-15 heat capacity
         * symbol(s): `C`
         * application domain: generic
         * name: HeatCapacity
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: derivative of added heat with respect to thermodynamic temperature of a system: `C = (dQ)/(dT)` where `Q` is amount of heat (item 5-6.1) and `T` is thermodynamic temperature (item 5-1)
         * remarks: Heat capacity is not completely defined unless specified as seen in items 5-16.2, 5-16.3 and 5-16.4.
         */
        attribute :>> num : Real;
        attribute :>> mRef : HeatCapacityUnit[1];
    }
    attribute def heatCapacity : HeatCapacityValue[*] nonunique;
    attribute def HeatCapacityUnit :> DerivedUnit {
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
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def SpecificHeatCapacityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-16.1 specific heat capacity
         * symbol(s): `c`
         * application domain: generic
         * name: SpecificHeatCapacity
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of heat capacity and mass: `c = C/m` where `C` is heat capacity (item 5-15) and `m` is mass (ISO 80000-4)
         * remarks: For the corresponding quantities related to the amount of substance, see ISO 80000-9.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificHeatCapacityUnit[1];
    }
    attribute def specificHeatCapacity : SpecificHeatCapacityValue[*] nonunique;
    attribute def SpecificHeatCapacityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def SpecificHeatCapacityAtConstantPressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-16.2 specific heat capacity at constant pressure
         * symbol(s): `c_p`
         * application domain: generic
         * name: SpecificHeatCapacityAtConstantPressure
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: specific heat capacity (item 5-16.1) at constant pressure (ISO 80000-4)
         * remarks: Also called specific isobaric heat capacity.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificHeatCapacityAtConstantPressureUnit[1];
    }
    attribute def specificHeatCapacityAtConstantPressure : SpecificHeatCapacityAtConstantPressureValue[*] nonunique;
    attribute def SpecificHeatCapacityAtConstantPressureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def SpecificHeatCapacityAtConstantVolumeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-16.3 specific heat capacity at constant volume
         * symbol(s): `c_V`
         * application domain: generic
         * name: SpecificHeatCapacityAtConstantVolume
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: specific heat capacity (item 5-16.1) at constant volume (ISO 80000-3)
         * remarks: Also called specific isochoric heat capacity.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificHeatCapacityAtConstantVolumeUnit[1];
    }
    attribute def specificHeatCapacityAtConstantVolume : SpecificHeatCapacityAtConstantVolumeValue[*] nonunique;
    attribute def SpecificHeatCapacityAtConstantVolumeUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def SpecificHeatCapacityAtSaturatedVapourPressureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-16.4 specific heat capacity at saturated vapour pressure
         * symbol(s): `c_"sat"`
         * application domain: generic
         * name: SpecificHeatCapacityAtSaturatedVapourPressure
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: specific heat capacity (item 5-16.1) at saturated vapour pressure (ISO 80000-4)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificHeatCapacityAtSaturatedVapourPressureUnit[1];
    }
    attribute def specificHeatCapacityAtSaturatedVapourPressure : SpecificHeatCapacityAtSaturatedVapourPressureValue[*] nonunique;
    attribute def SpecificHeatCapacityAtSaturatedVapourPressureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def RatioOfSpecificHeatCapacitiesValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-17.1 ratio of specific heat capacities
         * symbol(s): `γ`
         * application domain: generic
         * name: RatioOfSpecificHeatCapacities (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of specific heat capacity at constant pressure and specific heat capacity at constant volume: `γ = c_p/c_V` where `c_p` is specific heat capacity at constant pressure (item 5-16.2) and `c_V` is specific heat capacity at constant volume (item 5-16.3)
         * remarks: This quantity can also be expressed by `γ = C_p/C_V` where `C_p` is heat capacity at constant pressure and `C_V` is heat capacity at constant volume.
         */
    }
    attribute def ratioOfSpecificHeatCapacities : RatioOfSpecificHeatCapacitiesValue;
    attribute def IsentropicExponentValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-17.2 isentropic exponent, isentropic expansion factor
         * symbol(s): `ϰ`
         * application domain: generic
         * name: IsentropicExponent (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: the negative of relative pressure change, divided by relative volume change, at constant entropy: `ϰ = -V/p * ((partial p)/(partial V))_S` where `V` is volume (ISO 80000-3), `p` is pressure (ISO 80000-4), and `S` is entropy (item 5-18)
         * remarks: For an ideal gas, `ϰ` is equal to `γ` (item 5-17.1).
         */
    }
    attribute def isentropicExponent : IsentropicExponentValue;
    alias isentropicExpansionFactor for isentropicExponent;
    attribute def EntropyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-18 entropy
         * symbol(s): `S`
         * application domain: generic
         * name: Entropy
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: natural logarithm of number of equally probable microscopic configurations in a macroscopic system, multiplied by the Boltzmann constant: `S = k lnW` where `W` is number of configurations and `k` is the Boltzmann constant (ISO 80000-1)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : EntropyUnit[1];
    }
    attribute def entropy : EntropyValue[*] nonunique;
    attribute def EntropyUnit :> DerivedUnit {
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
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def SpecificEntropyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-19 specific entropy
         * symbol(s): `s`
         * application domain: generic
         * name: SpecificEntropy
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of entropy and mass: `s = S/m` where `S` is entropy (item 5-18) and `m` is mass (ISO 80000-4)
         * remarks: For the corresponding quantity related to amount of substance, see ISO 80000-9.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificEntropyUnit[1];
    }
    attribute def specificEntropy : SpecificEntropyValue[*] nonunique;
    attribute def SpecificEntropyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def EnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-20.1 energy
         * symbol(s): `E`
         * application domain: thermodynamics
         * name: Energy
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: ability of a system to do work (ISO 80000-4)
         * remarks: Energy exists in different forms that are mutually transformable into each other, either totally or partially. In contrast to internal energy (item 5-20.2), energy is not a state function.
         */
        attribute :>> num : Real;
        attribute :>> mRef : EnergyUnit[1];
    }
    attribute def energy : EnergyValue[*] nonunique;
    attribute def EnergyUnit :> DerivedUnit {
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
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def internalEnergy : EnergyValue {
        doc
        /*
         * source: item 5-20.2 internal energy, thermodynamic energy
         * symbol(s): `U`
         * application domain: generic
         * name: InternalEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: energy of a system whose change is given by the amount of the heat (item 5-6.1) transferred to the system and the work (ISO 80000-4) done on the system, provided that the system is closed and no chemical reactions occur
         * remarks: In thermodynamic text books, usually the formula `ΔU = Q + W` is used. Note that the zero of the energy is undefined.
         */
    }
    alias thermodynamicEnergy for internalEnergy;
    attribute def enthalpy : EnergyValue {
        doc
        /*
         * source: item 5-20.3 enthalpy
         * symbol(s): `H`
         * application domain: generic
         * name: Enthalpy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: sum of internal energy of the system and the product of pressure and volume of the system: `H = U + p*V` where U is internal energy (item 5-20.2), `p` is pressure (ISO 80000-4), and `V` is volume (ISO 80000-3)
         * remarks: None.
         */
    }
    attribute def helmholtzEnergy : EnergyValue {
        doc
        /*
         * source: item 5-20.4 Helmholtz energy, Helmholtz function
         * symbol(s): `A`, `F`
         * application domain: generic
         * name: HelmholtzEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference of internal energy of the system and the product of thermodynamic temperature and entropy of the system: `A = U - TS` where `U` is internal energy (item 5-20.2), `T` is thermodynamic temperature (item 5-1), and `S` is entropy (item 5-18)
         * remarks: The name Helmholtz free energy is also used. However, this term is not recommended.
         */
    }
    alias helmholtzFunction for helmholtzEnergy;
    attribute def gibbsEnergy : EnergyValue {
        doc
        /*
         * source: item 5-20.5 Gibbs energy, Gibbs function
         * symbol(s): `G`
         * application domain: generic
         * name: GibbsEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: difference of the enthalpy and the product of thermodynamic temperature and entropy of the system: `G = H - T*S` where H is enthalpy (item 5-20.3), `T` is thermodynamic temperature (item 5-1), and `S` is entropy (item 5-18)
         * remarks: The name Gibbs free energy is also used. However, this term is not recommended.
         */
    }
    alias gibbsFunction for gibbsEnergy;
    attribute def SpecificEnergyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-21.1 specific energy
         * symbol(s): `e`
         * application domain: generic
         * name: SpecificEnergy
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of energy and mass: `e = E/m` where `E` is energy (item 5-20.1) and `m` is mass (ISO 80000-4)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificEnergyUnit[1];
    }
    attribute def specificEnergy : SpecificEnergyValue[*] nonunique;
    attribute def SpecificEnergyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    attribute def specificInternalEnergy : SpecificEnergyValue {
        doc
        /*
         * source: item 5-21.2 specific internal energy, specific thermodynamic energy
         * symbol(s): `u`
         * application domain: generic
         * name: SpecificInternalEnergy (specializes SpecificEnergy)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of internal energy and mass: `u = U/m` where `U` is internal energy (item 5-20.2) and `m` is mass (ISO 80000-4)
         * remarks: None.
         */
    }
    alias specificThermodynamicEnergy for specificInternalEnergy;
    attribute def SpecificEnthalpyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-21.3 specific enthalpy
         * symbol(s): `h`
         * application domain: generic
         * name: SpecificEnthalpy
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of enthalpy and mass: `h = H/m` where `H` is enthalpy (item 5-20.3) and `m` is mass (ISO 80000-4)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificEnthalpyUnit[1];
    }
    attribute def specificEnthalpy : SpecificEnthalpyValue[*] nonunique;
    attribute def SpecificEnthalpyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    attribute def specificHelmholtzEnergy : SpecificEnergyValue {
        doc
        /*
         * source: item 5-21.4 specific Helmholtz energy, specific Helmholtz function
         * symbol(s): `a`, `f`
         * application domain: generic
         * name: SpecificHelmholtzEnergy (specializes SpecificEnergy)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of Helmholtz energy and mass: `a = A/m` where A is Helmholtz energy (item 5-20.4) and m is mass (ISO 80000-4)
         * remarks: The name specific Helmholtz free energy is also used. However, this term is not recommended.
         */
    }
    alias specificHelmholtzFunction for specificHelmholtzEnergy;
    attribute def specificGibbsEnergy : SpecificEnergyValue {
        doc
        /*
         * source: item 5-21.5 specific Gibbs energy, specific Gibbs function
         * symbol(s): `g`
         * application domain: generic
         * name: SpecificGibbsEnergy (specializes SpecificEnergy)
         * quantity dimension: L^2*T^-2
         * measurement unit(s): J/kg, m^2*s^-2
         * tensor order: 0
         * definition: quotient of Gibbs energy and mass: `g = G/m` where `G` is Gibbs energy (item 5-20.5) and `m` is mass (ISO 80000-4)
         * remarks: The name specific Gibbs free energy is also used. However, this term is not recommended.
         */
    }
    alias specificGibbsFunction for specificGibbsEnergy;
    attribute def MassieuFunctionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-22 Massieu function
         * symbol(s): `J`
         * application domain: generic
         * name: MassieuFunction
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of the negative of Helmholtz energy and temperature: `J = -A/T` where `A` is Helmholtz energy (item 5-20.4) and `T` is thermodynamic temperature (item 5-1)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassieuFunctionUnit[1];
    }
    attribute def massieuFunction : MassieuFunctionValue[*] nonunique;
    attribute def MassieuFunctionUnit :> DerivedUnit {
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
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def PlanckFunctionValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-23 Planck function
         * symbol(s): `Y`
         * application domain: generic
         * name: PlanckFunction
         * quantity dimension: L^2*M^1*T^-2*Θ^-1
         * measurement unit(s): J/K, kg*m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of the negative of Gibbs energy and temperature: `Y = -G/T` where G is Gibbs energy (item 5-20.5) and `T` is thermodynamic temperature (item 5-1)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PlanckFunctionUnit[1];
    }
    attribute def planckFunction : PlanckFunctionValue[*] nonunique;
    attribute def PlanckFunctionUnit :> DerivedUnit {
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
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def JouleThomsonCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-24 Joule-Thomson coefficient
         * symbol(s): `μ_"JT"`
         * application domain: generic
         * name: JouleThomsonCoefficient
         * quantity dimension: L^1*M^-1*T^2*Θ^1
         * measurement unit(s): K/Pa, kg^-1*m*s^2*K
         * tensor order: 0
         * definition: change of thermodynamic temperature with respect to pressure in a Joule-Thomson process at constant enthalpy: `μ_(JT) = ((partial T)/(partial p))_H` where `T` is thermodynamic temperature (item 5-1), `p` is pressure (ISO 80000-4) and H is enthalpy (item 5-20.3)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : JouleThomsonCoefficientUnit[1];
    }
    attribute def jouleThomsonCoefficient : JouleThomsonCoefficientValue[*] nonunique;
    attribute def JouleThomsonCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 1;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = -1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = 2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = 1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def ThermalEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-25.1 thermal efficiency
         * symbol(s): `η`
         * application domain: thermodynamics
         * name: ThermalEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of work (ISO 80000-4) delivered by a heat engine and supplied heat: `η = W/Q` where `W` is work (ISO 80000-4) and `Q` is heat (item 5-6.1)
         * remarks: None.
         */
    }
    attribute def thermalEfficiency : ThermalEfficiencyValue;
    attribute def MaximumThermalEfficiencyValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-25.2 maximum thermal efficiency
         * symbol(s): `η_"max"`
         * application domain: generic
         * name: MaximumThermalEfficiency (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: efficiency determined by the quotient of the temperatures of the hot source and the cold sink: `η_max = 1 - T_c/T_h` where `T_c` is the thermodynamic temperature (item 5-1) of the cold sink and `T_h` is the thermodynamic temperature (item 5-1) of the hot source
         * remarks: An ideal heat engine operating according to the Carnot process is delivering the maximum efficiency.
         */
    }
    attribute def maximumThermalEfficiency : MaximumThermalEfficiencyValue;
    attribute def SpecificGasConstantValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-26 specific gas constant
         * symbol(s): `R_s`
         * application domain: generic
         * name: SpecificGasConstant
         * quantity dimension: L^2*T^-2*Θ^-1
         * measurement unit(s): J/(kg*K), m^2*s^-2*K^-1
         * tensor order: 0
         * definition: quotient of the Boltzmann constant `k` (ISO 80000-1) and the mass `m` (ISO 80000-4) of the gas particle
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpecificGasConstantUnit[1];
    }
    attribute def specificGasConstant : SpecificGasConstantValue[*] nonunique;
    attribute def SpecificGasConstantUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        private attribute thermodynamicTemperaturePF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.'Θ';
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF, thermodynamicTemperaturePF);
        }
    }
    attribute def MassConcentrationOfWaterValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-27 mass concentration of water
         * symbol(s): `w`
         * application domain: generic
         * name: MassConcentrationOfWater
         * quantity dimension: L^-3*M^1
         * measurement unit(s): kg*m^-3
         * tensor order: 0
         * definition: quotient of mass of water and a specified volume: `w = m/V` where `m` is mass (ISO 80000-4) of water, irrespective of the form of aggregation state, and `V` is volume (ISO 80000-3)
         * remarks: Mass concentration of water at saturation is denoted `w_"sat"`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassConcentrationOfWaterUnit[1];
    }
    attribute def massConcentrationOfWater : MassConcentrationOfWaterValue[*] nonunique;
    attribute def MassConcentrationOfWaterUnit :> DerivedUnit {
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
    attribute def MassConcentrationOfWaterVapourAbsoluteHumidityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 5-28 mass concentration of water vapour absolute humidity
         * symbol(s): `v`
         * application domain: generic
         * name: MassConcentrationOfWaterVapourAbsoluteHumidity
         * quantity dimension: L^-3*M^1
         * measurement unit(s): kg*m^-3
         * tensor order: 0
         * definition: quotient of mass of water vapour and a specified volume: `v = m/V` where m is mass (ISO 80000-4) of water vapour and `V` is volume (ISO 80000-3)
         * remarks: Mass concentration of water vapour at saturation is denoted `v_"sat"`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : MassConcentrationOfWaterVapourAbsoluteHumidityUnit[1];
    }
    attribute def massConcentrationOfWaterVapourAbsoluteHumidity : MassConcentrationOfWaterVapourAbsoluteHumidityValue[*] nonunique;
    attribute def MassConcentrationOfWaterVapourAbsoluteHumidityUnit :> DerivedUnit {
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
    attribute def MassRatioOfWaterToDryMatterValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-29 mass ratio of water to dry matter
         * symbol(s): `u`
         * application domain: generic
         * name: MassRatioOfWaterToDryMatter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass of water and mass of dry matter: `u = m/m_d` where `m` is mass (ISO 80000-4) of water and `m_d` is mass of dry matter
         * remarks: Mass ratio of water to dry matter at saturation is denoted `u_"sat"`.
         */
    }
    attribute def massRatioOfWaterToDryMatter : MassRatioOfWaterToDryMatterValue;
    attribute def MassRatioOfWaterVapourToDryGasValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-30 mass ratio of water vapour to dry gas
         * symbol(s): `r`, `(x)`
         * application domain: generic
         * name: MassRatioOfWaterVapourToDryGas (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass of water vapour and mass of dry gas: `r = m/m_d` where `m` is mass (ISO 80000-4) of water vapour and `m_d` is mass of dry gas
         * remarks: Mass ratio of water vapour to dry gas at saturation is denoted `r_"sat"`. Mass ratio of water vapour to dry gas is also called mixing ratio.
         */
    }
    attribute def massRatioOfWaterVapourToDryGas : MassRatioOfWaterVapourToDryGasValue;
    attribute def MassFractionOfWaterValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-31 mass fraction of water
         * symbol(s): `w_(H_(2)O)`
         * application domain: generic
         * name: MassFractionOfWater (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `w_(H_(2)O) = u/(1+u)` where `u` is mass ratio of water to dry matter (item 5-29)
         * remarks: None.
         */
    }
    attribute def massFractionOfWater : MassFractionOfWaterValue;
    attribute def MassFractionOfDryMatterValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-32 mass fraction of dry matter
         * symbol(s): `w_d`
         * application domain: generic
         * name: MassFractionOfDryMatter (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quantity given by: `w_d = 1 - w_(H_(2)O)` where `w_(H_(2)O)` is mass fraction of water (item 5-31)
         * remarks: None.
         */
    }
    attribute def massFractionOfDryMatter : MassFractionOfDryMatterValue;
    attribute def RelativeHumidityValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-33 relative humidity
         * symbol(s): `φ`
         * application domain: generic
         * name: RelativeHumidity (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of partial pressure of water vapour and partial pressure at its saturation: `φ = p/p_"sat"` where `p` is partial pressure (ISO 80000-4) of vapour and `p_"sat"` is its partial pressure at saturation at the same temperature
         * remarks: Relative humidity is often referred to as RH and expressed in percent. See also remark in item 5-35.
         */
    }
    attribute def relativeHumidity : RelativeHumidityValue;
    attribute def RelativeMassConcentrationOfVapourValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-34 relative mass concentration of vapour
         * symbol(s): `φ`
         * application domain: generic
         * name: RelativeMassConcentrationOfVapour (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass concentration of water vapour and mass concentration at its saturation: `φ = v/v_"sat"` where `v` is mass concentration of water vapour (item 5-28) and `v_"sat"` is its mass concentration of water vapour at saturation of the same temperature
         * remarks: For water vapour concentrations up to 1 kg/m^3, the relative humidity (item 5-33) is assumed to be equal to relative mass concentration of vapour. For details see Reference [8].
         */
    }
    attribute def relativeMassConcentrationOfVapour : RelativeMassConcentrationOfVapourValue;
    attribute def RelativeMassRatioOfVapourValue :> DimensionOneValue {
        doc
        /*
         * source: item 5-35 relative mass ratio of vapour
         * symbol(s): `ψ`
         * application domain: generic
         * name: RelativeMassRatioOfVapour (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: quotient of mass ratio of water vapour to dry gas and mass ratio of water vapour to dry gas at saturation: `ψ = r/r_"sat"` where `r` is mass ratio of water vapour to dry gas (item 5-30) and `r_"sat"` is its mass ratio of water vapour to dry gas at saturation of the same temperature
         * remarks: This quantity is also used as an approximation of relative humidity (item 5-33).
         */
    }
    attribute def relativeMassRatioOfVapour : RelativeMassRatioOfVapourValue;
    attribute def dewPointTemperature : ThermodynamicTemperatureValue {
        doc
        /*
         * source: item 5-36 dew-point temperature
         * symbol(s): `T_d`
         * application domain: generic
         * name: DewPointTemperature (specializes ThermodynamicTemperature)
         * quantity dimension: Θ^1
         * measurement unit(s): K
         * tensor order: 0
         * definition: temperature at which water vapour in the air reaches saturation under isobaric conditions
         * remarks: The corresponding Celsius temperature, denoted `t_d`, is still called dew-point temperature. The unit for the corresponding Celsius temperature is degree Celsius, symbol °C.
         */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 789) (line 15) (column 20) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 789) (line 15) (column 20) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 803) (line 15) (column 34) (len 4)))))
    (reference r1 (scope relative) (span (offset 828) (line 16) (column 20) (len 10)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 828) (line 16) (column 20) (len 10)))))
    (reference r2 (scope relative) (span (offset 862) (line 17) (column 20) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 862) (line 17) (column 20) (len 21)))))
    (reference r3 (scope relative) (span (offset 907) (line 18) (column 20) (len 7)) (segments (segment 0 (token "ISQBase") (name "ISQBase") (separator none) (span (offset 907) (line 18) (column 20) (len 7)))))
    (reference r4 (scope relative) (span (offset 1206) (line 26) (column 31) (len 28)) (segments (segment 0 (token "ThermodynamicTemperatureUnit") (name "ThermodynamicTemperatureUnit") (separator none) (span (offset 1206) (line 26) (column 31) (len 28)))))
    (reference r5 (scope relative) (span (offset 1267) (line 27) (column 32) (len 29)) (segments (segment 0 (token "ThermodynamicTemperatureValue") (name "ThermodynamicTemperatureValue") (separator none) (span (offset 1267) (line 27) (column 32) (len 29)))))
    (reference r6 (scope relative) (span (offset 1324) (line 28) (column 27) (len 24)) (segments (segment 0 (token "thermodynamicTemperature") (name "thermodynamicTemperature") (separator none) (span (offset 1324) (line 28) (column 27) (len 24)))))
    (reference r7 (scope relative) (span (offset 1447) (line 31) (column 46) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 1447) (line 31) (column 46) (len 19)))))
    (reference r8 (scope relative) (span (offset 2597) (line 44) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 2597) (line 44) (column 28) (len 4)))))
    (reference r9 (scope relative) (span (offset 2592) (line 44) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 2592) (line 44) (column 23) (len 3)))))
    (reference r10 (scope relative) (span (offset 2631) (line 45) (column 29) (len 22)) (segments (segment 0 (token "CelsiusTemperatureUnit") (name "CelsiusTemperatureUnit") (separator none) (span (offset 2631) (line 45) (column 29) (len 22)))))
    (reference r11 (scope relative) (span (offset 2625) (line 45) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 2625) (line 45) (column 23) (len 4)))))
    (reference r12 (scope relative) (span (offset 2699) (line 48) (column 35) (len 23)) (segments (segment 0 (token "CelsiusTemperatureValue") (name "CelsiusTemperatureValue") (separator none) (span (offset 2699) (line 48) (column 35) (len 23)))))
    (reference r13 (scope relative) (span (offset 2802) (line 50) (column 45) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 2802) (line 50) (column 45) (len 11)))))
    (reference r14 (scope relative) (span (offset 2870) (line 51) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 2870) (line 51) (column 55) (len 19)))))
    (reference r15 (scope relative) (span (offset 2899) (line 51) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 2899) (line 51) (column 84) (len 8)))))
    (reference r16 (scope relative) (span (offset 2910) (line 51) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 2910) (line 51) (column 95) (len 3)))))
    (reference r17 (scope relative) (span (offset 2914) (line 51) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 2914) (line 51) (column 99) (len 4)))))
    (reference r18 (scope relative) (span (offset 2924) (line 51) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 2924) (line 51) (column 109) (len 8)))))
    (reference r19 (scope relative) (span (offset 2962) (line 52) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 2962) (line 52) (column 23) (len 17)))))
    (reference r20 (scope relative) (span (offset 2986) (line 52) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 2986) (line 52) (column 47) (len 20)))))
    (reference r21 (scope relative) (span (offset 3009) (line 52) (column 70) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 3009) (line 52) (column 70) (len 26)))))
    (reference r22 (scope relative) (span (offset 3161) (line 56) (column 54) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 3161) (line 56) (column 54) (len 19)))))
    (reference r23 (scope relative) (span (offset 3790) (line 69) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 3790) (line 69) (column 28) (len 4)))))
    (reference r24 (scope relative) (span (offset 3785) (line 69) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 3785) (line 69) (column 23) (len 3)))))
    (reference r25 (scope relative) (span (offset 3824) (line 70) (column 29) (len 30)) (segments (segment 0 (token "LinearExpansionCoefficientUnit") (name "LinearExpansionCoefficientUnit") (separator none) (span (offset 3824) (line 70) (column 29) (len 30)))))
    (reference r26 (scope relative) (span (offset 3818) (line 70) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 3818) (line 70) (column 23) (len 4)))))
    (reference r27 (scope relative) (span (offset 3908) (line 73) (column 43) (len 31)) (segments (segment 0 (token "LinearExpansionCoefficientValue") (name "LinearExpansionCoefficientValue") (separator none) (span (offset 3908) (line 73) (column 43) (len 31)))))
    (reference r28 (scope relative) (span (offset 4027) (line 75) (column 53) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 4027) (line 75) (column 53) (len 11)))))
    (reference r29 (scope relative) (span (offset 4095) (line 76) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 4095) (line 76) (column 55) (len 19)))))
    (reference r30 (scope relative) (span (offset 4124) (line 76) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 4124) (line 76) (column 84) (len 8)))))
    (reference r31 (scope relative) (span (offset 4135) (line 76) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 4135) (line 76) (column 95) (len 3)))))
    (reference r32 (scope relative) (span (offset 4139) (line 76) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 4139) (line 76) (column 99) (len 4)))))
    (reference r33 (scope relative) (span (offset 4149) (line 76) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 4149) (line 76) (column 109) (len 8)))))
    (reference r34 (scope relative) (span (offset 4188) (line 77) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 4188) (line 77) (column 23) (len 17)))))
    (reference r35 (scope relative) (span (offset 4212) (line 77) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 4212) (line 77) (column 47) (len 20)))))
    (reference r36 (scope relative) (span (offset 4235) (line 77) (column 70) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 4235) (line 77) (column 70) (len 26)))))
    (reference r37 (scope relative) (span (offset 4385) (line 81) (column 53) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 4385) (line 81) (column 53) (len 19)))))
    (reference r38 (scope relative) (span (offset 5066) (line 94) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 5066) (line 94) (column 28) (len 4)))))
    (reference r39 (scope relative) (span (offset 5061) (line 94) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 5061) (line 94) (column 23) (len 3)))))
    (reference r40 (scope relative) (span (offset 5100) (line 95) (column 29) (len 29)) (segments (segment 0 (token "CubicExpansionCoefficientUnit") (name "CubicExpansionCoefficientUnit") (separator none) (span (offset 5100) (line 95) (column 29) (len 29)))))
    (reference r41 (scope relative) (span (offset 5094) (line 95) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 5094) (line 95) (column 23) (len 4)))))
    (reference r42 (scope relative) (span (offset 5182) (line 98) (column 42) (len 30)) (segments (segment 0 (token "CubicExpansionCoefficientValue") (name "CubicExpansionCoefficientValue") (separator none) (span (offset 5182) (line 98) (column 42) (len 30)))))
    (reference r43 (scope relative) (span (offset 5299) (line 100) (column 52) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 5299) (line 100) (column 52) (len 11)))))
    (reference r44 (scope relative) (span (offset 5367) (line 101) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 5367) (line 101) (column 55) (len 19)))))
    (reference r45 (scope relative) (span (offset 5396) (line 101) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 5396) (line 101) (column 84) (len 8)))))
    (reference r46 (scope relative) (span (offset 5407) (line 101) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 5407) (line 101) (column 95) (len 3)))))
    (reference r47 (scope relative) (span (offset 5411) (line 101) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 5411) (line 101) (column 99) (len 4)))))
    (reference r48 (scope relative) (span (offset 5421) (line 101) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 5421) (line 101) (column 109) (len 8)))))
    (reference r49 (scope relative) (span (offset 5460) (line 102) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 5460) (line 102) (column 23) (len 17)))))
    (reference r50 (scope relative) (span (offset 5484) (line 102) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 5484) (line 102) (column 47) (len 20)))))
    (reference r51 (scope relative) (span (offset 5507) (line 102) (column 70) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 5507) (line 102) (column 70) (len 26)))))
    (reference r52 (scope relative) (span (offset 5661) (line 106) (column 55) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 5661) (line 106) (column 55) (len 19)))))
    (reference r53 (scope relative) (span (offset 6365) (line 119) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 6365) (line 119) (column 28) (len 4)))))
    (reference r54 (scope relative) (span (offset 6360) (line 119) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 6360) (line 119) (column 23) (len 3)))))
    (reference r55 (scope relative) (span (offset 6399) (line 120) (column 29) (len 31)) (segments (segment 0 (token "RelativePressureCoefficientUnit") (name "RelativePressureCoefficientUnit") (separator none) (span (offset 6399) (line 120) (column 29) (len 31)))))
    (reference r56 (scope relative) (span (offset 6393) (line 120) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 6393) (line 120) (column 23) (len 4)))))
    (reference r57 (scope relative) (span (offset 6485) (line 123) (column 44) (len 32)) (segments (segment 0 (token "RelativePressureCoefficientValue") (name "RelativePressureCoefficientValue") (separator none) (span (offset 6485) (line 123) (column 44) (len 32)))))
    (reference r58 (scope relative) (span (offset 6606) (line 125) (column 54) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 6606) (line 125) (column 54) (len 11)))))
    (reference r59 (scope relative) (span (offset 6674) (line 126) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 6674) (line 126) (column 55) (len 19)))))
    (reference r60 (scope relative) (span (offset 6703) (line 126) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 6703) (line 126) (column 84) (len 8)))))
    (reference r61 (scope relative) (span (offset 6714) (line 126) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 6714) (line 126) (column 95) (len 3)))))
    (reference r62 (scope relative) (span (offset 6718) (line 126) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 6718) (line 126) (column 99) (len 4)))))
    (reference r63 (scope relative) (span (offset 6728) (line 126) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 6728) (line 126) (column 109) (len 8)))))
    (reference r64 (scope relative) (span (offset 6767) (line 127) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 6767) (line 127) (column 23) (len 17)))))
    (reference r65 (scope relative) (span (offset 6791) (line 127) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 6791) (line 127) (column 47) (len 20)))))
    (reference r66 (scope relative) (span (offset 6814) (line 127) (column 70) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 6814) (line 127) (column 70) (len 26)))))
    (reference r67 (scope relative) (span (offset 6949) (line 131) (column 47) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 6949) (line 131) (column 47) (len 19)))))
    (reference r68 (scope relative) (span (offset 7573) (line 144) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 7573) (line 144) (column 28) (len 4)))))
    (reference r69 (scope relative) (span (offset 7568) (line 144) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 7568) (line 144) (column 23) (len 3)))))
    (reference r70 (scope relative) (span (offset 7607) (line 145) (column 29) (len 23)) (segments (segment 0 (token "PressureCoefficientUnit") (name "PressureCoefficientUnit") (separator none) (span (offset 7607) (line 145) (column 29) (len 23)))))
    (reference r71 (scope relative) (span (offset 7601) (line 145) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 7601) (line 145) (column 23) (len 4)))))
    (reference r72 (scope relative) (span (offset 7677) (line 148) (column 36) (len 24)) (segments (segment 0 (token "PressureCoefficientValue") (name "PressureCoefficientValue") (separator none) (span (offset 7677) (line 148) (column 36) (len 24)))))
    (reference r73 (scope relative) (span (offset 7782) (line 150) (column 46) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 7782) (line 150) (column 46) (len 11)))))
    (reference r74 (scope relative) (span (offset 7832) (line 151) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 7832) (line 151) (column 37) (len 19)))))
    (reference r75 (scope relative) (span (offset 7861) (line 151) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 7861) (line 151) (column 66) (len 8)))))
    (reference r76 (scope relative) (span (offset 7872) (line 151) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 7872) (line 151) (column 77) (len 3)))))
    (reference r77 (scope relative) (span (offset 7876) (line 151) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 7876) (line 151) (column 81) (len 1)))))
    (reference r78 (scope relative) (span (offset 7883) (line 151) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 7883) (line 151) (column 88) (len 8)))))
    (reference r79 (scope relative) (span (offset 7934) (line 152) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 7934) (line 152) (column 35) (len 19)))))
    (reference r80 (scope relative) (span (offset 7963) (line 152) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 7963) (line 152) (column 64) (len 8)))))
    (reference r81 (scope relative) (span (offset 7974) (line 152) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 7974) (line 152) (column 75) (len 3)))))
    (reference r82 (scope relative) (span (offset 7978) (line 152) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 7978) (line 152) (column 79) (len 1)))))
    (reference r83 (scope relative) (span (offset 7985) (line 152) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 7985) (line 152) (column 86) (len 8)))))
    (reference r84 (scope relative) (span (offset 8039) (line 153) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 8039) (line 153) (column 39) (len 19)))))
    (reference r85 (scope relative) (span (offset 8068) (line 153) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 8068) (line 153) (column 68) (len 8)))))
    (reference r86 (scope relative) (span (offset 8079) (line 153) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 8079) (line 153) (column 79) (len 3)))))
    (reference r87 (scope relative) (span (offset 8083) (line 153) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 8083) (line 153) (column 83) (len 1)))))
    (reference r88 (scope relative) (span (offset 8090) (line 153) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 8090) (line 153) (column 90) (len 8)))))
    (reference r89 (scope relative) (span (offset 8161) (line 154) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 8161) (line 154) (column 55) (len 19)))))
    (reference r90 (scope relative) (span (offset 8190) (line 154) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 8190) (line 154) (column 84) (len 8)))))
    (reference r91 (scope relative) (span (offset 8201) (line 154) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 8201) (line 154) (column 95) (len 3)))))
    (reference r92 (scope relative) (span (offset 8205) (line 154) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 8205) (line 154) (column 99) (len 4)))))
    (reference r93 (scope relative) (span (offset 8215) (line 154) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 8215) (line 154) (column 109) (len 8)))))
    (reference r94 (scope relative) (span (offset 8254) (line 155) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 8254) (line 155) (column 23) (len 17)))))
    (reference r95 (scope relative) (span (offset 8278) (line 155) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 8278) (line 155) (column 47) (len 20)))))
    (reference r96 (scope relative) (span (offset 8302) (line 155) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 8302) (line 155) (column 71) (len 8)))))
    (reference r97 (scope relative) (span (offset 8312) (line 155) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 8312) (line 155) (column 81) (len 6)))))
    (reference r98 (scope relative) (span (offset 8320) (line 155) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 8320) (line 155) (column 89) (len 10)))))
    (reference r99 (scope relative) (span (offset 8332) (line 155) (column 101) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 8332) (line 155) (column 101) (len 26)))))
    (reference r100 (scope relative) (span (offset 8482) (line 159) (column 53) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 8482) (line 159) (column 53) (len 19)))))
    (reference r101 (scope relative) (span (offset 9212) (line 172) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 9212) (line 172) (column 28) (len 4)))))
    (reference r102 (scope relative) (span (offset 9207) (line 172) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 9207) (line 172) (column 23) (len 3)))))
    (reference r103 (scope relative) (span (offset 9246) (line 173) (column 29) (len 29)) (segments (segment 0 (token "IsothermalCompressibilityUnit") (name "IsothermalCompressibilityUnit") (separator none) (span (offset 9246) (line 173) (column 29) (len 29)))))
    (reference r104 (scope relative) (span (offset 9240) (line 173) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 9240) (line 173) (column 23) (len 4)))))
    (reference r105 (scope relative) (span (offset 9328) (line 176) (column 42) (len 30)) (segments (segment 0 (token "IsothermalCompressibilityValue") (name "IsothermalCompressibilityValue") (separator none) (span (offset 9328) (line 176) (column 42) (len 30)))))
    (reference r106 (scope relative) (span (offset 9445) (line 178) (column 52) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 9445) (line 178) (column 52) (len 11)))))
    (reference r107 (scope relative) (span (offset 9495) (line 179) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 9495) (line 179) (column 37) (len 19)))))
    (reference r108 (scope relative) (span (offset 9524) (line 179) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 9524) (line 179) (column 66) (len 8)))))
    (reference r109 (scope relative) (span (offset 9535) (line 179) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 9535) (line 179) (column 77) (len 3)))))
    (reference r110 (scope relative) (span (offset 9539) (line 179) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 9539) (line 179) (column 81) (len 1)))))
    (reference r111 (scope relative) (span (offset 9546) (line 179) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 9546) (line 179) (column 88) (len 8)))))
    (reference r112 (scope relative) (span (offset 9596) (line 180) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 9596) (line 180) (column 35) (len 19)))))
    (reference r113 (scope relative) (span (offset 9625) (line 180) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 9625) (line 180) (column 64) (len 8)))))
    (reference r114 (scope relative) (span (offset 9636) (line 180) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 9636) (line 180) (column 75) (len 3)))))
    (reference r115 (scope relative) (span (offset 9640) (line 180) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 9640) (line 180) (column 79) (len 1)))))
    (reference r116 (scope relative) (span (offset 9647) (line 180) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 9647) (line 180) (column 86) (len 8)))))
    (reference r117 (scope relative) (span (offset 9702) (line 181) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 9702) (line 181) (column 39) (len 19)))))
    (reference r118 (scope relative) (span (offset 9731) (line 181) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 9731) (line 181) (column 68) (len 8)))))
    (reference r119 (scope relative) (span (offset 9742) (line 181) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 9742) (line 181) (column 79) (len 3)))))
    (reference r120 (scope relative) (span (offset 9746) (line 181) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 9746) (line 181) (column 83) (len 1)))))
    (reference r121 (scope relative) (span (offset 9753) (line 181) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 9753) (line 181) (column 90) (len 8)))))
    (reference r122 (scope relative) (span (offset 9791) (line 182) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 9791) (line 182) (column 23) (len 17)))))
    (reference r123 (scope relative) (span (offset 9815) (line 182) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 9815) (line 182) (column 47) (len 20)))))
    (reference r124 (scope relative) (span (offset 9839) (line 182) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 9839) (line 182) (column 71) (len 8)))))
    (reference r125 (scope relative) (span (offset 9849) (line 182) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 9849) (line 182) (column 81) (len 6)))))
    (reference r126 (scope relative) (span (offset 9857) (line 182) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 9857) (line 182) (column 89) (len 10)))))
    (reference r127 (scope relative) (span (offset 9991) (line 186) (column 53) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 9991) (line 186) (column 53) (len 19)))))
    (reference r128 (scope relative) (span (offset 10700) (line 199) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 10700) (line 199) (column 28) (len 4)))))
    (reference r129 (scope relative) (span (offset 10695) (line 199) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 10695) (line 199) (column 23) (len 3)))))
    (reference r130 (scope relative) (span (offset 10734) (line 200) (column 29) (len 29)) (segments (segment 0 (token "IsentropicCompressibilityUnit") (name "IsentropicCompressibilityUnit") (separator none) (span (offset 10734) (line 200) (column 29) (len 29)))))
    (reference r131 (scope relative) (span (offset 10728) (line 200) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 10728) (line 200) (column 23) (len 4)))))
    (reference r132 (scope relative) (span (offset 10816) (line 203) (column 42) (len 30)) (segments (segment 0 (token "IsentropicCompressibilityValue") (name "IsentropicCompressibilityValue") (separator none) (span (offset 10816) (line 203) (column 42) (len 30)))))
    (reference r133 (scope relative) (span (offset 10933) (line 205) (column 52) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 10933) (line 205) (column 52) (len 11)))))
    (reference r134 (scope relative) (span (offset 10983) (line 206) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 10983) (line 206) (column 37) (len 19)))))
    (reference r135 (scope relative) (span (offset 11012) (line 206) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 11012) (line 206) (column 66) (len 8)))))
    (reference r136 (scope relative) (span (offset 11023) (line 206) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 11023) (line 206) (column 77) (len 3)))))
    (reference r137 (scope relative) (span (offset 11027) (line 206) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 11027) (line 206) (column 81) (len 1)))))
    (reference r138 (scope relative) (span (offset 11034) (line 206) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 11034) (line 206) (column 88) (len 8)))))
    (reference r139 (scope relative) (span (offset 11084) (line 207) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 11084) (line 207) (column 35) (len 19)))))
    (reference r140 (scope relative) (span (offset 11113) (line 207) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 11113) (line 207) (column 64) (len 8)))))
    (reference r141 (scope relative) (span (offset 11124) (line 207) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 11124) (line 207) (column 75) (len 3)))))
    (reference r142 (scope relative) (span (offset 11128) (line 207) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 11128) (line 207) (column 79) (len 1)))))
    (reference r143 (scope relative) (span (offset 11135) (line 207) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 11135) (line 207) (column 86) (len 8)))))
    (reference r144 (scope relative) (span (offset 11190) (line 208) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 11190) (line 208) (column 39) (len 19)))))
    (reference r145 (scope relative) (span (offset 11219) (line 208) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 11219) (line 208) (column 68) (len 8)))))
    (reference r146 (scope relative) (span (offset 11230) (line 208) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 11230) (line 208) (column 79) (len 3)))))
    (reference r147 (scope relative) (span (offset 11234) (line 208) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 11234) (line 208) (column 83) (len 1)))))
    (reference r148 (scope relative) (span (offset 11241) (line 208) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 11241) (line 208) (column 90) (len 8)))))
    (reference r149 (scope relative) (span (offset 11279) (line 209) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 11279) (line 209) (column 23) (len 17)))))
    (reference r150 (scope relative) (span (offset 11303) (line 209) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 11303) (line 209) (column 47) (len 20)))))
    (reference r151 (scope relative) (span (offset 11327) (line 209) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 11327) (line 209) (column 71) (len 8)))))
    (reference r152 (scope relative) (span (offset 11337) (line 209) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 11337) (line 209) (column 81) (len 6)))))
    (reference r153 (scope relative) (span (offset 11345) (line 209) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 11345) (line 209) (column 89) (len 10)))))
    (reference r154 (scope relative) (span (offset 11441) (line 213) (column 21) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 11441) (line 213) (column 21) (len 11)))))
    (reference r155 (scope relative) (span (offset 12500) (line 228) (column 28) (len 4)) (segments (segment 0 (token "heat") (name "heat") (separator none) (span (offset 12500) (line 228) (column 28) (len 4)))))
    (reference r156 (scope relative) (span (offset 12578) (line 231) (column 27) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 12578) (line 231) (column 27) (len 11)))))
    (reference r157 (scope relative) (span (offset 13237) (line 247) (column 40) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 13237) (line 247) (column 40) (len 19)))))
    (reference r158 (scope relative) (span (offset 13697) (line 260) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 13697) (line 260) (column 28) (len 4)))))
    (reference r159 (scope relative) (span (offset 13692) (line 260) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 13692) (line 260) (column 23) (len 3)))))
    (reference r160 (scope relative) (span (offset 13731) (line 261) (column 29) (len 16)) (segments (segment 0 (token "HeatFlowRateUnit") (name "HeatFlowRateUnit") (separator none) (span (offset 13731) (line 261) (column 29) (len 16)))))
    (reference r161 (scope relative) (span (offset 13725) (line 261) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 13725) (line 261) (column 23) (len 4)))))
    (reference r162 (scope relative) (span (offset 13787) (line 264) (column 29) (len 17)) (segments (segment 0 (token "HeatFlowRateValue") (name "HeatFlowRateValue") (separator none) (span (offset 13787) (line 264) (column 29) (len 17)))))
    (reference r163 (scope relative) (span (offset 13878) (line 266) (column 39) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 13878) (line 266) (column 39) (len 11)))))
    (reference r164 (scope relative) (span (offset 13928) (line 267) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 13928) (line 267) (column 37) (len 19)))))
    (reference r165 (scope relative) (span (offset 13957) (line 267) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 13957) (line 267) (column 66) (len 8)))))
    (reference r166 (scope relative) (span (offset 13968) (line 267) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 13968) (line 267) (column 77) (len 3)))))
    (reference r167 (scope relative) (span (offset 13972) (line 267) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 13972) (line 267) (column 81) (len 1)))))
    (reference r168 (scope relative) (span (offset 13979) (line 267) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 13979) (line 267) (column 88) (len 8)))))
    (reference r169 (scope relative) (span (offset 14029) (line 268) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 14029) (line 268) (column 35) (len 19)))))
    (reference r170 (scope relative) (span (offset 14058) (line 268) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 14058) (line 268) (column 64) (len 8)))))
    (reference r171 (scope relative) (span (offset 14069) (line 268) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 14069) (line 268) (column 75) (len 3)))))
    (reference r172 (scope relative) (span (offset 14073) (line 268) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 14073) (line 268) (column 79) (len 1)))))
    (reference r173 (scope relative) (span (offset 14080) (line 268) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 14080) (line 268) (column 86) (len 8)))))
    (reference r174 (scope relative) (span (offset 14134) (line 269) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 14134) (line 269) (column 39) (len 19)))))
    (reference r175 (scope relative) (span (offset 14163) (line 269) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 14163) (line 269) (column 68) (len 8)))))
    (reference r176 (scope relative) (span (offset 14174) (line 269) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 14174) (line 269) (column 79) (len 3)))))
    (reference r177 (scope relative) (span (offset 14178) (line 269) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 14178) (line 269) (column 83) (len 1)))))
    (reference r178 (scope relative) (span (offset 14185) (line 269) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 14185) (line 269) (column 90) (len 8)))))
    (reference r179 (scope relative) (span (offset 14224) (line 270) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 14224) (line 270) (column 23) (len 17)))))
    (reference r180 (scope relative) (span (offset 14248) (line 270) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 14248) (line 270) (column 47) (len 20)))))
    (reference r181 (scope relative) (span (offset 14272) (line 270) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 14272) (line 270) (column 71) (len 8)))))
    (reference r182 (scope relative) (span (offset 14282) (line 270) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 14282) (line 270) (column 81) (len 6)))))
    (reference r183 (scope relative) (span (offset 14290) (line 270) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 14290) (line 270) (column 89) (len 10)))))
    (reference r184 (scope relative) (span (offset 14417) (line 274) (column 49) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 14417) (line 274) (column 49) (len 19)))))
    (reference r185 (scope relative) (span (offset 14971) (line 287) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 14971) (line 287) (column 28) (len 4)))))
    (reference r186 (scope relative) (span (offset 14966) (line 287) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 14966) (line 287) (column 23) (len 3)))))
    (reference r187 (scope relative) (span (offset 15005) (line 288) (column 29) (len 25)) (segments (segment 0 (token "DensityOfHeatFlowRateUnit") (name "DensityOfHeatFlowRateUnit") (separator none) (span (offset 15005) (line 288) (column 29) (len 25)))))
    (reference r188 (scope relative) (span (offset 14999) (line 288) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 14999) (line 288) (column 23) (len 4)))))
    (reference r189 (scope relative) (span (offset 15079) (line 291) (column 38) (len 26)) (segments (segment 0 (token "DensityOfHeatFlowRateValue") (name "DensityOfHeatFlowRateValue") (separator none) (span (offset 15079) (line 291) (column 38) (len 26)))))
    (reference r190 (scope relative) (span (offset 15188) (line 293) (column 48) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 15188) (line 293) (column 48) (len 11)))))
    (reference r191 (scope relative) (span (offset 15236) (line 294) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 15236) (line 294) (column 35) (len 19)))))
    (reference r192 (scope relative) (span (offset 15265) (line 294) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 15265) (line 294) (column 64) (len 8)))))
    (reference r193 (scope relative) (span (offset 15276) (line 294) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 15276) (line 294) (column 75) (len 3)))))
    (reference r194 (scope relative) (span (offset 15280) (line 294) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 15280) (line 294) (column 79) (len 1)))))
    (reference r195 (scope relative) (span (offset 15287) (line 294) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 15287) (line 294) (column 86) (len 8)))))
    (reference r196 (scope relative) (span (offset 15341) (line 295) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 15341) (line 295) (column 39) (len 19)))))
    (reference r197 (scope relative) (span (offset 15370) (line 295) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 15370) (line 295) (column 68) (len 8)))))
    (reference r198 (scope relative) (span (offset 15381) (line 295) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 15381) (line 295) (column 79) (len 3)))))
    (reference r199 (scope relative) (span (offset 15385) (line 295) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 15385) (line 295) (column 83) (len 1)))))
    (reference r200 (scope relative) (span (offset 15392) (line 295) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 15392) (line 295) (column 90) (len 8)))))
    (reference r201 (scope relative) (span (offset 15431) (line 296) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 15431) (line 296) (column 23) (len 17)))))
    (reference r202 (scope relative) (span (offset 15455) (line 296) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 15455) (line 296) (column 47) (len 20)))))
    (reference r203 (scope relative) (span (offset 15479) (line 296) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 15479) (line 296) (column 71) (len 6)))))
    (reference r204 (scope relative) (span (offset 15487) (line 296) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 15487) (line 296) (column 79) (len 10)))))
    (reference r205 (scope relative) (span (offset 15607) (line 300) (column 47) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 15607) (line 300) (column 47) (len 19)))))
    (reference r206 (scope relative) (span (offset 16168) (line 313) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 16168) (line 313) (column 28) (len 4)))))
    (reference r207 (scope relative) (span (offset 16163) (line 313) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 16163) (line 313) (column 23) (len 3)))))
    (reference r208 (scope relative) (span (offset 16202) (line 314) (column 29) (len 23)) (segments (segment 0 (token "ThermalConductivityUnit") (name "ThermalConductivityUnit") (separator none) (span (offset 16202) (line 314) (column 29) (len 23)))))
    (reference r209 (scope relative) (span (offset 16196) (line 314) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 16196) (line 314) (column 23) (len 4)))))
    (reference r210 (scope relative) (span (offset 16272) (line 317) (column 36) (len 24)) (segments (segment 0 (token "ThermalConductivityValue") (name "ThermalConductivityValue") (separator none) (span (offset 16272) (line 317) (column 36) (len 24)))))
    (reference r211 (scope relative) (span (offset 16377) (line 319) (column 46) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 16377) (line 319) (column 46) (len 11)))))
    (reference r212 (scope relative) (span (offset 16427) (line 320) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 16427) (line 320) (column 37) (len 19)))))
    (reference r213 (scope relative) (span (offset 16456) (line 320) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 16456) (line 320) (column 66) (len 8)))))
    (reference r214 (scope relative) (span (offset 16467) (line 320) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 16467) (line 320) (column 77) (len 3)))))
    (reference r215 (scope relative) (span (offset 16471) (line 320) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 16471) (line 320) (column 81) (len 1)))))
    (reference r216 (scope relative) (span (offset 16478) (line 320) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 16478) (line 320) (column 88) (len 8)))))
    (reference r217 (scope relative) (span (offset 16528) (line 321) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 16528) (line 321) (column 35) (len 19)))))
    (reference r218 (scope relative) (span (offset 16557) (line 321) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 16557) (line 321) (column 64) (len 8)))))
    (reference r219 (scope relative) (span (offset 16568) (line 321) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 16568) (line 321) (column 75) (len 3)))))
    (reference r220 (scope relative) (span (offset 16572) (line 321) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 16572) (line 321) (column 79) (len 1)))))
    (reference r221 (scope relative) (span (offset 16579) (line 321) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 16579) (line 321) (column 86) (len 8)))))
    (reference r222 (scope relative) (span (offset 16633) (line 322) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 16633) (line 322) (column 39) (len 19)))))
    (reference r223 (scope relative) (span (offset 16662) (line 322) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 16662) (line 322) (column 68) (len 8)))))
    (reference r224 (scope relative) (span (offset 16673) (line 322) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 16673) (line 322) (column 79) (len 3)))))
    (reference r225 (scope relative) (span (offset 16677) (line 322) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 16677) (line 322) (column 83) (len 1)))))
    (reference r226 (scope relative) (span (offset 16684) (line 322) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 16684) (line 322) (column 90) (len 8)))))
    (reference r227 (scope relative) (span (offset 16755) (line 323) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 16755) (line 323) (column 55) (len 19)))))
    (reference r228 (scope relative) (span (offset 16784) (line 323) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 16784) (line 323) (column 84) (len 8)))))
    (reference r229 (scope relative) (span (offset 16795) (line 323) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 16795) (line 323) (column 95) (len 3)))))
    (reference r230 (scope relative) (span (offset 16799) (line 323) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 16799) (line 323) (column 99) (len 4)))))
    (reference r231 (scope relative) (span (offset 16809) (line 323) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 16809) (line 323) (column 109) (len 8)))))
    (reference r232 (scope relative) (span (offset 16848) (line 324) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 16848) (line 324) (column 23) (len 17)))))
    (reference r233 (scope relative) (span (offset 16872) (line 324) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 16872) (line 324) (column 47) (len 20)))))
    (reference r234 (scope relative) (span (offset 16896) (line 324) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 16896) (line 324) (column 71) (len 8)))))
    (reference r235 (scope relative) (span (offset 16906) (line 324) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 16906) (line 324) (column 81) (len 6)))))
    (reference r236 (scope relative) (span (offset 16914) (line 324) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 16914) (line 324) (column 89) (len 10)))))
    (reference r237 (scope relative) (span (offset 16926) (line 324) (column 101) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 16926) (line 324) (column 101) (len 26)))))
    (reference r238 (scope relative) (span (offset 17079) (line 328) (column 53) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 17079) (line 328) (column 53) (len 19)))))
    (reference r239 (scope relative) (span (offset 17775) (line 341) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 17775) (line 341) (column 28) (len 4)))))
    (reference r240 (scope relative) (span (offset 17770) (line 341) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 17770) (line 341) (column 23) (len 3)))))
    (reference r241 (scope relative) (span (offset 17809) (line 342) (column 29) (len 29)) (segments (segment 0 (token "CoefficientOfHeatTransferUnit") (name "CoefficientOfHeatTransferUnit") (separator none) (span (offset 17809) (line 342) (column 29) (len 29)))))
    (reference r242 (scope relative) (span (offset 17803) (line 342) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 17803) (line 342) (column 23) (len 4)))))
    (reference r243 (scope relative) (span (offset 17891) (line 345) (column 42) (len 30)) (segments (segment 0 (token "CoefficientOfHeatTransferValue") (name "CoefficientOfHeatTransferValue") (separator none) (span (offset 17891) (line 345) (column 42) (len 30)))))
    (reference r244 (scope relative) (span (offset 18008) (line 347) (column 52) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 18008) (line 347) (column 52) (len 11)))))
    (reference r245 (scope relative) (span (offset 18056) (line 348) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 18056) (line 348) (column 35) (len 19)))))
    (reference r246 (scope relative) (span (offset 18085) (line 348) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 18085) (line 348) (column 64) (len 8)))))
    (reference r247 (scope relative) (span (offset 18096) (line 348) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 18096) (line 348) (column 75) (len 3)))))
    (reference r248 (scope relative) (span (offset 18100) (line 348) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 18100) (line 348) (column 79) (len 1)))))
    (reference r249 (scope relative) (span (offset 18107) (line 348) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 18107) (line 348) (column 86) (len 8)))))
    (reference r250 (scope relative) (span (offset 18161) (line 349) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 18161) (line 349) (column 39) (len 19)))))
    (reference r251 (scope relative) (span (offset 18190) (line 349) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 18190) (line 349) (column 68) (len 8)))))
    (reference r252 (scope relative) (span (offset 18201) (line 349) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 18201) (line 349) (column 79) (len 3)))))
    (reference r253 (scope relative) (span (offset 18205) (line 349) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 18205) (line 349) (column 83) (len 1)))))
    (reference r254 (scope relative) (span (offset 18212) (line 349) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 18212) (line 349) (column 90) (len 8)))))
    (reference r255 (scope relative) (span (offset 18283) (line 350) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 18283) (line 350) (column 55) (len 19)))))
    (reference r256 (scope relative) (span (offset 18312) (line 350) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 18312) (line 350) (column 84) (len 8)))))
    (reference r257 (scope relative) (span (offset 18323) (line 350) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 18323) (line 350) (column 95) (len 3)))))
    (reference r258 (scope relative) (span (offset 18327) (line 350) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 18327) (line 350) (column 99) (len 4)))))
    (reference r259 (scope relative) (span (offset 18337) (line 350) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 18337) (line 350) (column 109) (len 8)))))
    (reference r260 (scope relative) (span (offset 18376) (line 351) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 18376) (line 351) (column 23) (len 17)))))
    (reference r261 (scope relative) (span (offset 18400) (line 351) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 18400) (line 351) (column 47) (len 20)))))
    (reference r262 (scope relative) (span (offset 18424) (line 351) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 18424) (line 351) (column 71) (len 6)))))
    (reference r263 (scope relative) (span (offset 18432) (line 351) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 18432) (line 351) (column 79) (len 10)))))
    (reference r264 (scope relative) (span (offset 18444) (line 351) (column 91) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 18444) (line 351) (column 91) (len 26)))))
    (reference r265 (scope relative) (span (offset 18612) (line 355) (column 60) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 18612) (line 355) (column 60) (len 19)))))
    (reference r266 (scope relative) (span (offset 19414) (line 368) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 19414) (line 368) (column 28) (len 4)))))
    (reference r267 (scope relative) (span (offset 19409) (line 368) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 19409) (line 368) (column 23) (len 3)))))
    (reference r268 (scope relative) (span (offset 19448) (line 369) (column 29) (len 36)) (segments (segment 0 (token "SurfaceCoefficientOfHeatTransferUnit") (name "SurfaceCoefficientOfHeatTransferUnit") (separator none) (span (offset 19448) (line 369) (column 29) (len 36)))))
    (reference r269 (scope relative) (span (offset 19442) (line 369) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 19442) (line 369) (column 23) (len 4)))))
    (reference r270 (scope relative) (span (offset 19544) (line 372) (column 49) (len 37)) (segments (segment 0 (token "SurfaceCoefficientOfHeatTransferValue") (name "SurfaceCoefficientOfHeatTransferValue") (separator none) (span (offset 19544) (line 372) (column 49) (len 37)))))
    (reference r271 (scope relative) (span (offset 19675) (line 374) (column 59) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 19675) (line 374) (column 59) (len 11)))))
    (reference r272 (scope relative) (span (offset 19723) (line 375) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 19723) (line 375) (column 35) (len 19)))))
    (reference r273 (scope relative) (span (offset 19752) (line 375) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 19752) (line 375) (column 64) (len 8)))))
    (reference r274 (scope relative) (span (offset 19763) (line 375) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 19763) (line 375) (column 75) (len 3)))))
    (reference r275 (scope relative) (span (offset 19767) (line 375) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 19767) (line 375) (column 79) (len 1)))))
    (reference r276 (scope relative) (span (offset 19774) (line 375) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 19774) (line 375) (column 86) (len 8)))))
    (reference r277 (scope relative) (span (offset 19828) (line 376) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 19828) (line 376) (column 39) (len 19)))))
    (reference r278 (scope relative) (span (offset 19857) (line 376) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 19857) (line 376) (column 68) (len 8)))))
    (reference r279 (scope relative) (span (offset 19868) (line 376) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 19868) (line 376) (column 79) (len 3)))))
    (reference r280 (scope relative) (span (offset 19872) (line 376) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 19872) (line 376) (column 83) (len 1)))))
    (reference r281 (scope relative) (span (offset 19879) (line 376) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 19879) (line 376) (column 90) (len 8)))))
    (reference r282 (scope relative) (span (offset 19950) (line 377) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 19950) (line 377) (column 55) (len 19)))))
    (reference r283 (scope relative) (span (offset 19979) (line 377) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 19979) (line 377) (column 84) (len 8)))))
    (reference r284 (scope relative) (span (offset 19990) (line 377) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 19990) (line 377) (column 95) (len 3)))))
    (reference r285 (scope relative) (span (offset 19994) (line 377) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 19994) (line 377) (column 99) (len 4)))))
    (reference r286 (scope relative) (span (offset 20004) (line 377) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 20004) (line 377) (column 109) (len 8)))))
    (reference r287 (scope relative) (span (offset 20043) (line 378) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 20043) (line 378) (column 23) (len 17)))))
    (reference r288 (scope relative) (span (offset 20067) (line 378) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 20067) (line 378) (column 47) (len 20)))))
    (reference r289 (scope relative) (span (offset 20091) (line 378) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 20091) (line 378) (column 71) (len 6)))))
    (reference r290 (scope relative) (span (offset 20099) (line 378) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 20099) (line 378) (column 79) (len 10)))))
    (reference r291 (scope relative) (span (offset 20111) (line 378) (column 91) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 20111) (line 378) (column 91) (len 26)))))
    (reference r292 (scope relative) (span (offset 20276) (line 382) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 20276) (line 382) (column 44) (len 19)))))
    (reference r293 (scope relative) (span (offset 20912) (line 395) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 20912) (line 395) (column 28) (len 4)))))
    (reference r294 (scope relative) (span (offset 20907) (line 395) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 20907) (line 395) (column 23) (len 3)))))
    (reference r295 (scope relative) (span (offset 20946) (line 396) (column 29) (len 20)) (segments (segment 0 (token "ThermalInsulanceUnit") (name "ThermalInsulanceUnit") (separator none) (span (offset 20946) (line 396) (column 29) (len 20)))))
    (reference r296 (scope relative) (span (offset 20940) (line 396) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 20940) (line 396) (column 23) (len 4)))))
    (reference r297 (scope relative) (span (offset 21010) (line 399) (column 33) (len 21)) (segments (segment 0 (token "ThermalInsulanceValue") (name "ThermalInsulanceValue") (separator none) (span (offset 21010) (line 399) (column 33) (len 21)))))
    (reference r298 (scope relative) (span (offset 21109) (line 401) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 21109) (line 401) (column 43) (len 11)))))
    (reference r299 (scope relative) (span (offset 21157) (line 402) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 21157) (line 402) (column 35) (len 19)))))
    (reference r300 (scope relative) (span (offset 21186) (line 402) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 21186) (line 402) (column 64) (len 8)))))
    (reference r301 (scope relative) (span (offset 21197) (line 402) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 21197) (line 402) (column 75) (len 3)))))
    (reference r302 (scope relative) (span (offset 21201) (line 402) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 21201) (line 402) (column 79) (len 1)))))
    (reference r303 (scope relative) (span (offset 21208) (line 402) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 21208) (line 402) (column 86) (len 8)))))
    (reference r304 (scope relative) (span (offset 21263) (line 403) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 21263) (line 403) (column 39) (len 19)))))
    (reference r305 (scope relative) (span (offset 21292) (line 403) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 21292) (line 403) (column 68) (len 8)))))
    (reference r306 (scope relative) (span (offset 21303) (line 403) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 21303) (line 403) (column 79) (len 3)))))
    (reference r307 (scope relative) (span (offset 21307) (line 403) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 21307) (line 403) (column 83) (len 1)))))
    (reference r308 (scope relative) (span (offset 21314) (line 403) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 21314) (line 403) (column 90) (len 8)))))
    (reference r309 (scope relative) (span (offset 21384) (line 404) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 21384) (line 404) (column 55) (len 19)))))
    (reference r310 (scope relative) (span (offset 21413) (line 404) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 21413) (line 404) (column 84) (len 8)))))
    (reference r311 (scope relative) (span (offset 21424) (line 404) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 21424) (line 404) (column 95) (len 3)))))
    (reference r312 (scope relative) (span (offset 21428) (line 404) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 21428) (line 404) (column 99) (len 4)))))
    (reference r313 (scope relative) (span (offset 21438) (line 404) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 21438) (line 404) (column 109) (len 8)))))
    (reference r314 (scope relative) (span (offset 21476) (line 405) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 21476) (line 405) (column 23) (len 17)))))
    (reference r315 (scope relative) (span (offset 21500) (line 405) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 21500) (line 405) (column 47) (len 20)))))
    (reference r316 (scope relative) (span (offset 21524) (line 405) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 21524) (line 405) (column 71) (len 6)))))
    (reference r317 (scope relative) (span (offset 21532) (line 405) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 21532) (line 405) (column 79) (len 10)))))
    (reference r318 (scope relative) (span (offset 21544) (line 405) (column 91) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 21544) (line 405) (column 91) (len 26)))))
    (reference r319 (scope relative) (span (offset 21630) (line 408) (column 49) (len 20)) (segments (segment 0 (token "ThermalInsulanceUnit") (name "ThermalInsulanceUnit") (separator none) (span (offset 21630) (line 408) (column 49) (len 20)))))
    (reference r320 (scope relative) (span (offset 21701) (line 409) (column 50) (len 21)) (segments (segment 0 (token "ThermalInsulanceValue") (name "ThermalInsulanceValue") (separator none) (span (offset 21701) (line 409) (column 50) (len 21)))))
    (reference r321 (scope relative) (span (offset 21768) (line 410) (column 45) (len 16)) (segments (segment 0 (token "thermalInsulance") (name "thermalInsulance") (separator none) (span (offset 21768) (line 410) (column 45) (len 16)))))
    (reference r322 (scope relative) (span (offset 21882) (line 413) (column 45) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 21882) (line 413) (column 45) (len 19)))))
    (reference r323 (scope relative) (span (offset 22403) (line 426) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 22403) (line 426) (column 28) (len 4)))))
    (reference r324 (scope relative) (span (offset 22398) (line 426) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 22398) (line 426) (column 23) (len 3)))))
    (reference r325 (scope relative) (span (offset 22437) (line 427) (column 29) (len 21)) (segments (segment 0 (token "ThermalResistanceUnit") (name "ThermalResistanceUnit") (separator none) (span (offset 22437) (line 427) (column 29) (len 21)))))
    (reference r326 (scope relative) (span (offset 22431) (line 427) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 22431) (line 427) (column 23) (len 4)))))
    (reference r327 (scope relative) (span (offset 22503) (line 430) (column 34) (len 22)) (segments (segment 0 (token "ThermalResistanceValue") (name "ThermalResistanceValue") (separator none) (span (offset 22503) (line 430) (column 34) (len 22)))))
    (reference r328 (scope relative) (span (offset 22604) (line 432) (column 44) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 22604) (line 432) (column 44) (len 11)))))
    (reference r329 (scope relative) (span (offset 22654) (line 433) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 22654) (line 433) (column 37) (len 19)))))
    (reference r330 (scope relative) (span (offset 22683) (line 433) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 22683) (line 433) (column 66) (len 8)))))
    (reference r331 (scope relative) (span (offset 22694) (line 433) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 22694) (line 433) (column 77) (len 3)))))
    (reference r332 (scope relative) (span (offset 22698) (line 433) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 22698) (line 433) (column 81) (len 1)))))
    (reference r333 (scope relative) (span (offset 22705) (line 433) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 22705) (line 433) (column 88) (len 8)))))
    (reference r334 (scope relative) (span (offset 22756) (line 434) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 22756) (line 434) (column 35) (len 19)))))
    (reference r335 (scope relative) (span (offset 22785) (line 434) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 22785) (line 434) (column 64) (len 8)))))
    (reference r336 (scope relative) (span (offset 22796) (line 434) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 22796) (line 434) (column 75) (len 3)))))
    (reference r337 (scope relative) (span (offset 22800) (line 434) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 22800) (line 434) (column 79) (len 1)))))
    (reference r338 (scope relative) (span (offset 22807) (line 434) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 22807) (line 434) (column 86) (len 8)))))
    (reference r339 (scope relative) (span (offset 22862) (line 435) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 22862) (line 435) (column 39) (len 19)))))
    (reference r340 (scope relative) (span (offset 22891) (line 435) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 22891) (line 435) (column 68) (len 8)))))
    (reference r341 (scope relative) (span (offset 22902) (line 435) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 22902) (line 435) (column 79) (len 3)))))
    (reference r342 (scope relative) (span (offset 22906) (line 435) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 22906) (line 435) (column 83) (len 1)))))
    (reference r343 (scope relative) (span (offset 22913) (line 435) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 22913) (line 435) (column 90) (len 8)))))
    (reference r344 (scope relative) (span (offset 22983) (line 436) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 22983) (line 436) (column 55) (len 19)))))
    (reference r345 (scope relative) (span (offset 23012) (line 436) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 23012) (line 436) (column 84) (len 8)))))
    (reference r346 (scope relative) (span (offset 23023) (line 436) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 23023) (line 436) (column 95) (len 3)))))
    (reference r347 (scope relative) (span (offset 23027) (line 436) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 23027) (line 436) (column 99) (len 4)))))
    (reference r348 (scope relative) (span (offset 23037) (line 436) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 23037) (line 436) (column 109) (len 8)))))
    (reference r349 (scope relative) (span (offset 23075) (line 437) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 23075) (line 437) (column 23) (len 17)))))
    (reference r350 (scope relative) (span (offset 23099) (line 437) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 23099) (line 437) (column 47) (len 20)))))
    (reference r351 (scope relative) (span (offset 23123) (line 437) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 23123) (line 437) (column 71) (len 8)))))
    (reference r352 (scope relative) (span (offset 23133) (line 437) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 23133) (line 437) (column 81) (len 6)))))
    (reference r353 (scope relative) (span (offset 23141) (line 437) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 23141) (line 437) (column 89) (len 10)))))
    (reference r354 (scope relative) (span (offset 23153) (line 437) (column 101) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 23153) (line 437) (column 101) (len 26)))))
    (reference r355 (scope relative) (span (offset 23288) (line 441) (column 46) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 23288) (line 441) (column 46) (len 19)))))
    (reference r356 (scope relative) (span (offset 23890) (line 454) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 23890) (line 454) (column 28) (len 4)))))
    (reference r357 (scope relative) (span (offset 23885) (line 454) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 23885) (line 454) (column 23) (len 3)))))
    (reference r358 (scope relative) (span (offset 23924) (line 455) (column 29) (len 22)) (segments (segment 0 (token "ThermalConductanceUnit") (name "ThermalConductanceUnit") (separator none) (span (offset 23924) (line 455) (column 29) (len 22)))))
    (reference r359 (scope relative) (span (offset 23918) (line 455) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 23918) (line 455) (column 23) (len 4)))))
    (reference r360 (scope relative) (span (offset 23992) (line 458) (column 35) (len 23)) (segments (segment 0 (token "ThermalConductanceValue") (name "ThermalConductanceValue") (separator none) (span (offset 23992) (line 458) (column 35) (len 23)))))
    (reference r361 (scope relative) (span (offset 24095) (line 460) (column 45) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 24095) (line 460) (column 45) (len 11)))))
    (reference r362 (scope relative) (span (offset 24145) (line 461) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 24145) (line 461) (column 37) (len 19)))))
    (reference r363 (scope relative) (span (offset 24174) (line 461) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 24174) (line 461) (column 66) (len 8)))))
    (reference r364 (scope relative) (span (offset 24185) (line 461) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 24185) (line 461) (column 77) (len 3)))))
    (reference r365 (scope relative) (span (offset 24189) (line 461) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 24189) (line 461) (column 81) (len 1)))))
    (reference r366 (scope relative) (span (offset 24196) (line 461) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 24196) (line 461) (column 88) (len 8)))))
    (reference r367 (scope relative) (span (offset 24246) (line 462) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 24246) (line 462) (column 35) (len 19)))))
    (reference r368 (scope relative) (span (offset 24275) (line 462) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 24275) (line 462) (column 64) (len 8)))))
    (reference r369 (scope relative) (span (offset 24286) (line 462) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 24286) (line 462) (column 75) (len 3)))))
    (reference r370 (scope relative) (span (offset 24290) (line 462) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 24290) (line 462) (column 79) (len 1)))))
    (reference r371 (scope relative) (span (offset 24297) (line 462) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 24297) (line 462) (column 86) (len 8)))))
    (reference r372 (scope relative) (span (offset 24351) (line 463) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 24351) (line 463) (column 39) (len 19)))))
    (reference r373 (scope relative) (span (offset 24380) (line 463) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 24380) (line 463) (column 68) (len 8)))))
    (reference r374 (scope relative) (span (offset 24391) (line 463) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 24391) (line 463) (column 79) (len 3)))))
    (reference r375 (scope relative) (span (offset 24395) (line 463) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 24395) (line 463) (column 83) (len 1)))))
    (reference r376 (scope relative) (span (offset 24402) (line 463) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 24402) (line 463) (column 90) (len 8)))))
    (reference r377 (scope relative) (span (offset 24473) (line 464) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 24473) (line 464) (column 55) (len 19)))))
    (reference r378 (scope relative) (span (offset 24502) (line 464) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 24502) (line 464) (column 84) (len 8)))))
    (reference r379 (scope relative) (span (offset 24513) (line 464) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 24513) (line 464) (column 95) (len 3)))))
    (reference r380 (scope relative) (span (offset 24517) (line 464) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 24517) (line 464) (column 99) (len 4)))))
    (reference r381 (scope relative) (span (offset 24527) (line 464) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 24527) (line 464) (column 109) (len 8)))))
    (reference r382 (scope relative) (span (offset 24566) (line 465) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 24566) (line 465) (column 23) (len 17)))))
    (reference r383 (scope relative) (span (offset 24590) (line 465) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 24590) (line 465) (column 47) (len 20)))))
    (reference r384 (scope relative) (span (offset 24614) (line 465) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 24614) (line 465) (column 71) (len 8)))))
    (reference r385 (scope relative) (span (offset 24624) (line 465) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 24624) (line 465) (column 81) (len 6)))))
    (reference r386 (scope relative) (span (offset 24632) (line 465) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 24632) (line 465) (column 89) (len 10)))))
    (reference r387 (scope relative) (span (offset 24644) (line 465) (column 101) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 24644) (line 465) (column 101) (len 26)))))
    (reference r388 (scope relative) (span (offset 24779) (line 469) (column 46) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 24779) (line 469) (column 46) (len 19)))))
    (reference r389 (scope relative) (span (offset 25437) (line 482) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 25437) (line 482) (column 28) (len 4)))))
    (reference r390 (scope relative) (span (offset 25432) (line 482) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 25432) (line 482) (column 23) (len 3)))))
    (reference r391 (scope relative) (span (offset 25471) (line 483) (column 29) (len 22)) (segments (segment 0 (token "ThermalDiffusivityUnit") (name "ThermalDiffusivityUnit") (separator none) (span (offset 25471) (line 483) (column 29) (len 22)))))
    (reference r392 (scope relative) (span (offset 25465) (line 483) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 25465) (line 483) (column 23) (len 4)))))
    (reference r393 (scope relative) (span (offset 25539) (line 486) (column 35) (len 23)) (segments (segment 0 (token "ThermalDiffusivityValue") (name "ThermalDiffusivityValue") (separator none) (span (offset 25539) (line 486) (column 35) (len 23)))))
    (reference r394 (scope relative) (span (offset 25642) (line 488) (column 45) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 25642) (line 488) (column 45) (len 11)))))
    (reference r395 (scope relative) (span (offset 25692) (line 489) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 25692) (line 489) (column 37) (len 19)))))
    (reference r396 (scope relative) (span (offset 25721) (line 489) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 25721) (line 489) (column 66) (len 8)))))
    (reference r397 (scope relative) (span (offset 25732) (line 489) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 25732) (line 489) (column 77) (len 3)))))
    (reference r398 (scope relative) (span (offset 25736) (line 489) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 25736) (line 489) (column 81) (len 1)))))
    (reference r399 (scope relative) (span (offset 25743) (line 489) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 25743) (line 489) (column 88) (len 8)))))
    (reference r400 (scope relative) (span (offset 25797) (line 490) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 25797) (line 490) (column 39) (len 19)))))
    (reference r401 (scope relative) (span (offset 25826) (line 490) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 25826) (line 490) (column 68) (len 8)))))
    (reference r402 (scope relative) (span (offset 25837) (line 490) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 25837) (line 490) (column 79) (len 3)))))
    (reference r403 (scope relative) (span (offset 25841) (line 490) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 25841) (line 490) (column 83) (len 1)))))
    (reference r404 (scope relative) (span (offset 25848) (line 490) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 25848) (line 490) (column 90) (len 8)))))
    (reference r405 (scope relative) (span (offset 25887) (line 491) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 25887) (line 491) (column 23) (len 17)))))
    (reference r406 (scope relative) (span (offset 25911) (line 491) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 25911) (line 491) (column 47) (len 20)))))
    (reference r407 (scope relative) (span (offset 25935) (line 491) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 25935) (line 491) (column 71) (len 8)))))
    (reference r408 (scope relative) (span (offset 25945) (line 491) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 25945) (line 491) (column 81) (len 10)))))
    (reference r409 (scope relative) (span (offset 26052) (line 495) (column 40) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 26052) (line 495) (column 40) (len 19)))))
    (reference r410 (scope relative) (span (offset 26734) (line 508) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 26734) (line 508) (column 28) (len 4)))))
    (reference r411 (scope relative) (span (offset 26729) (line 508) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 26729) (line 508) (column 23) (len 3)))))
    (reference r412 (scope relative) (span (offset 26768) (line 509) (column 29) (len 16)) (segments (segment 0 (token "HeatCapacityUnit") (name "HeatCapacityUnit") (separator none) (span (offset 26768) (line 509) (column 29) (len 16)))))
    (reference r413 (scope relative) (span (offset 26762) (line 509) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 26762) (line 509) (column 23) (len 4)))))
    (reference r414 (scope relative) (span (offset 26824) (line 512) (column 29) (len 17)) (segments (segment 0 (token "HeatCapacityValue") (name "HeatCapacityValue") (separator none) (span (offset 26824) (line 512) (column 29) (len 17)))))
    (reference r415 (scope relative) (span (offset 26915) (line 514) (column 39) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 26915) (line 514) (column 39) (len 11)))))
    (reference r416 (scope relative) (span (offset 26965) (line 515) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 26965) (line 515) (column 37) (len 19)))))
    (reference r417 (scope relative) (span (offset 26994) (line 515) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 26994) (line 515) (column 66) (len 8)))))
    (reference r418 (scope relative) (span (offset 27005) (line 515) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 27005) (line 515) (column 77) (len 3)))))
    (reference r419 (scope relative) (span (offset 27009) (line 515) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 27009) (line 515) (column 81) (len 1)))))
    (reference r420 (scope relative) (span (offset 27016) (line 515) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 27016) (line 515) (column 88) (len 8)))))
    (reference r421 (scope relative) (span (offset 27066) (line 516) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 27066) (line 516) (column 35) (len 19)))))
    (reference r422 (scope relative) (span (offset 27095) (line 516) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 27095) (line 516) (column 64) (len 8)))))
    (reference r423 (scope relative) (span (offset 27106) (line 516) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 27106) (line 516) (column 75) (len 3)))))
    (reference r424 (scope relative) (span (offset 27110) (line 516) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 27110) (line 516) (column 79) (len 1)))))
    (reference r425 (scope relative) (span (offset 27117) (line 516) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 27117) (line 516) (column 86) (len 8)))))
    (reference r426 (scope relative) (span (offset 27171) (line 517) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 27171) (line 517) (column 39) (len 19)))))
    (reference r427 (scope relative) (span (offset 27200) (line 517) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 27200) (line 517) (column 68) (len 8)))))
    (reference r428 (scope relative) (span (offset 27211) (line 517) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 27211) (line 517) (column 79) (len 3)))))
    (reference r429 (scope relative) (span (offset 27215) (line 517) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 27215) (line 517) (column 83) (len 1)))))
    (reference r430 (scope relative) (span (offset 27222) (line 517) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 27222) (line 517) (column 90) (len 8)))))
    (reference r431 (scope relative) (span (offset 27293) (line 518) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 27293) (line 518) (column 55) (len 19)))))
    (reference r432 (scope relative) (span (offset 27322) (line 518) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 27322) (line 518) (column 84) (len 8)))))
    (reference r433 (scope relative) (span (offset 27333) (line 518) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 27333) (line 518) (column 95) (len 3)))))
    (reference r434 (scope relative) (span (offset 27337) (line 518) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 27337) (line 518) (column 99) (len 4)))))
    (reference r435 (scope relative) (span (offset 27347) (line 518) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 27347) (line 518) (column 109) (len 8)))))
    (reference r436 (scope relative) (span (offset 27386) (line 519) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 27386) (line 519) (column 23) (len 17)))))
    (reference r437 (scope relative) (span (offset 27410) (line 519) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 27410) (line 519) (column 47) (len 20)))))
    (reference r438 (scope relative) (span (offset 27434) (line 519) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 27434) (line 519) (column 71) (len 8)))))
    (reference r439 (scope relative) (span (offset 27444) (line 519) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 27444) (line 519) (column 81) (len 6)))))
    (reference r440 (scope relative) (span (offset 27452) (line 519) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 27452) (line 519) (column 89) (len 10)))))
    (reference r441 (scope relative) (span (offset 27464) (line 519) (column 101) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 27464) (line 519) (column 101) (len 26)))))
    (reference r442 (scope relative) (span (offset 27606) (line 523) (column 48) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 27606) (line 523) (column 48) (len 19)))))
    (reference r443 (scope relative) (span (offset 28220) (line 536) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 28220) (line 536) (column 28) (len 4)))))
    (reference r444 (scope relative) (span (offset 28215) (line 536) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 28215) (line 536) (column 23) (len 3)))))
    (reference r445 (scope relative) (span (offset 28254) (line 537) (column 29) (len 24)) (segments (segment 0 (token "SpecificHeatCapacityUnit") (name "SpecificHeatCapacityUnit") (separator none) (span (offset 28254) (line 537) (column 29) (len 24)))))
    (reference r446 (scope relative) (span (offset 28248) (line 537) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 28248) (line 537) (column 23) (len 4)))))
    (reference r447 (scope relative) (span (offset 28326) (line 540) (column 37) (len 25)) (segments (segment 0 (token "SpecificHeatCapacityValue") (name "SpecificHeatCapacityValue") (separator none) (span (offset 28326) (line 540) (column 37) (len 25)))))
    (reference r448 (scope relative) (span (offset 28433) (line 542) (column 47) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 28433) (line 542) (column 47) (len 11)))))
    (reference r449 (scope relative) (span (offset 28483) (line 543) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 28483) (line 543) (column 37) (len 19)))))
    (reference r450 (scope relative) (span (offset 28512) (line 543) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 28512) (line 543) (column 66) (len 8)))))
    (reference r451 (scope relative) (span (offset 28523) (line 543) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 28523) (line 543) (column 77) (len 3)))))
    (reference r452 (scope relative) (span (offset 28527) (line 543) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 28527) (line 543) (column 81) (len 1)))))
    (reference r453 (scope relative) (span (offset 28534) (line 543) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 28534) (line 543) (column 88) (len 8)))))
    (reference r454 (scope relative) (span (offset 28588) (line 544) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 28588) (line 544) (column 39) (len 19)))))
    (reference r455 (scope relative) (span (offset 28617) (line 544) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 28617) (line 544) (column 68) (len 8)))))
    (reference r456 (scope relative) (span (offset 28628) (line 544) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 28628) (line 544) (column 79) (len 3)))))
    (reference r457 (scope relative) (span (offset 28632) (line 544) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 28632) (line 544) (column 83) (len 1)))))
    (reference r458 (scope relative) (span (offset 28639) (line 544) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 28639) (line 544) (column 90) (len 8)))))
    (reference r459 (scope relative) (span (offset 28710) (line 545) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 28710) (line 545) (column 55) (len 19)))))
    (reference r460 (scope relative) (span (offset 28739) (line 545) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 28739) (line 545) (column 84) (len 8)))))
    (reference r461 (scope relative) (span (offset 28750) (line 545) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 28750) (line 545) (column 95) (len 3)))))
    (reference r462 (scope relative) (span (offset 28754) (line 545) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 28754) (line 545) (column 99) (len 4)))))
    (reference r463 (scope relative) (span (offset 28764) (line 545) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 28764) (line 545) (column 109) (len 8)))))
    (reference r464 (scope relative) (span (offset 28803) (line 546) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 28803) (line 546) (column 23) (len 17)))))
    (reference r465 (scope relative) (span (offset 28827) (line 546) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 28827) (line 546) (column 47) (len 20)))))
    (reference r466 (scope relative) (span (offset 28851) (line 546) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 28851) (line 546) (column 71) (len 8)))))
    (reference r467 (scope relative) (span (offset 28861) (line 546) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 28861) (line 546) (column 81) (len 10)))))
    (reference r468 (scope relative) (span (offset 28873) (line 546) (column 93) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 28873) (line 546) (column 93) (len 26)))))
    (reference r469 (scope relative) (span (offset 29054) (line 550) (column 66) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 29054) (line 550) (column 66) (len 19)))))
    (reference r470 (scope relative) (span (offset 29625) (line 563) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 29625) (line 563) (column 28) (len 4)))))
    (reference r471 (scope relative) (span (offset 29620) (line 563) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 29620) (line 563) (column 23) (len 3)))))
    (reference r472 (scope relative) (span (offset 29659) (line 564) (column 29) (len 42)) (segments (segment 0 (token "SpecificHeatCapacityAtConstantPressureUnit") (name "SpecificHeatCapacityAtConstantPressureUnit") (separator none) (span (offset 29659) (line 564) (column 29) (len 42)))))
    (reference r473 (scope relative) (span (offset 29653) (line 564) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 29653) (line 564) (column 23) (len 4)))))
    (reference r474 (scope relative) (span (offset 29767) (line 567) (column 55) (len 43)) (segments (segment 0 (token "SpecificHeatCapacityAtConstantPressureValue") (name "SpecificHeatCapacityAtConstantPressureValue") (separator none) (span (offset 29767) (line 567) (column 55) (len 43)))))
    (reference r475 (scope relative) (span (offset 29910) (line 569) (column 65) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 29910) (line 569) (column 65) (len 11)))))
    (reference r476 (scope relative) (span (offset 29960) (line 570) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 29960) (line 570) (column 37) (len 19)))))
    (reference r477 (scope relative) (span (offset 29989) (line 570) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 29989) (line 570) (column 66) (len 8)))))
    (reference r478 (scope relative) (span (offset 30000) (line 570) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 30000) (line 570) (column 77) (len 3)))))
    (reference r479 (scope relative) (span (offset 30004) (line 570) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 30004) (line 570) (column 81) (len 1)))))
    (reference r480 (scope relative) (span (offset 30011) (line 570) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 30011) (line 570) (column 88) (len 8)))))
    (reference r481 (scope relative) (span (offset 30065) (line 571) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 30065) (line 571) (column 39) (len 19)))))
    (reference r482 (scope relative) (span (offset 30094) (line 571) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 30094) (line 571) (column 68) (len 8)))))
    (reference r483 (scope relative) (span (offset 30105) (line 571) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 30105) (line 571) (column 79) (len 3)))))
    (reference r484 (scope relative) (span (offset 30109) (line 571) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 30109) (line 571) (column 83) (len 1)))))
    (reference r485 (scope relative) (span (offset 30116) (line 571) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 30116) (line 571) (column 90) (len 8)))))
    (reference r486 (scope relative) (span (offset 30187) (line 572) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 30187) (line 572) (column 55) (len 19)))))
    (reference r487 (scope relative) (span (offset 30216) (line 572) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 30216) (line 572) (column 84) (len 8)))))
    (reference r488 (scope relative) (span (offset 30227) (line 572) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 30227) (line 572) (column 95) (len 3)))))
    (reference r489 (scope relative) (span (offset 30231) (line 572) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 30231) (line 572) (column 99) (len 4)))))
    (reference r490 (scope relative) (span (offset 30241) (line 572) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 30241) (line 572) (column 109) (len 8)))))
    (reference r491 (scope relative) (span (offset 30280) (line 573) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 30280) (line 573) (column 23) (len 17)))))
    (reference r492 (scope relative) (span (offset 30304) (line 573) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 30304) (line 573) (column 47) (len 20)))))
    (reference r493 (scope relative) (span (offset 30328) (line 573) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 30328) (line 573) (column 71) (len 8)))))
    (reference r494 (scope relative) (span (offset 30338) (line 573) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 30338) (line 573) (column 81) (len 10)))))
    (reference r495 (scope relative) (span (offset 30350) (line 573) (column 93) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 30350) (line 573) (column 93) (len 26)))))
    (reference r496 (scope relative) (span (offset 30527) (line 577) (column 64) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 30527) (line 577) (column 64) (len 19)))))
    (reference r497 (scope relative) (span (offset 31093) (line 590) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 31093) (line 590) (column 28) (len 4)))))
    (reference r498 (scope relative) (span (offset 31088) (line 590) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 31088) (line 590) (column 23) (len 3)))))
    (reference r499 (scope relative) (span (offset 31127) (line 591) (column 29) (len 40)) (segments (segment 0 (token "SpecificHeatCapacityAtConstantVolumeUnit") (name "SpecificHeatCapacityAtConstantVolumeUnit") (separator none) (span (offset 31127) (line 591) (column 29) (len 40)))))
    (reference r500 (scope relative) (span (offset 31121) (line 591) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 31121) (line 591) (column 23) (len 4)))))
    (reference r501 (scope relative) (span (offset 31231) (line 594) (column 53) (len 41)) (segments (segment 0 (token "SpecificHeatCapacityAtConstantVolumeValue") (name "SpecificHeatCapacityAtConstantVolumeValue") (separator none) (span (offset 31231) (line 594) (column 53) (len 41)))))
    (reference r502 (scope relative) (span (offset 31370) (line 596) (column 63) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 31370) (line 596) (column 63) (len 11)))))
    (reference r503 (scope relative) (span (offset 31420) (line 597) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 31420) (line 597) (column 37) (len 19)))))
    (reference r504 (scope relative) (span (offset 31449) (line 597) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 31449) (line 597) (column 66) (len 8)))))
    (reference r505 (scope relative) (span (offset 31460) (line 597) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 31460) (line 597) (column 77) (len 3)))))
    (reference r506 (scope relative) (span (offset 31464) (line 597) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 31464) (line 597) (column 81) (len 1)))))
    (reference r507 (scope relative) (span (offset 31471) (line 597) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 31471) (line 597) (column 88) (len 8)))))
    (reference r508 (scope relative) (span (offset 31525) (line 598) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 31525) (line 598) (column 39) (len 19)))))
    (reference r509 (scope relative) (span (offset 31554) (line 598) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 31554) (line 598) (column 68) (len 8)))))
    (reference r510 (scope relative) (span (offset 31565) (line 598) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 31565) (line 598) (column 79) (len 3)))))
    (reference r511 (scope relative) (span (offset 31569) (line 598) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 31569) (line 598) (column 83) (len 1)))))
    (reference r512 (scope relative) (span (offset 31576) (line 598) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 31576) (line 598) (column 90) (len 8)))))
    (reference r513 (scope relative) (span (offset 31647) (line 599) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 31647) (line 599) (column 55) (len 19)))))
    (reference r514 (scope relative) (span (offset 31676) (line 599) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 31676) (line 599) (column 84) (len 8)))))
    (reference r515 (scope relative) (span (offset 31687) (line 599) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 31687) (line 599) (column 95) (len 3)))))
    (reference r516 (scope relative) (span (offset 31691) (line 599) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 31691) (line 599) (column 99) (len 4)))))
    (reference r517 (scope relative) (span (offset 31701) (line 599) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 31701) (line 599) (column 109) (len 8)))))
    (reference r518 (scope relative) (span (offset 31740) (line 600) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 31740) (line 600) (column 23) (len 17)))))
    (reference r519 (scope relative) (span (offset 31764) (line 600) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 31764) (line 600) (column 47) (len 20)))))
    (reference r520 (scope relative) (span (offset 31788) (line 600) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 31788) (line 600) (column 71) (len 8)))))
    (reference r521 (scope relative) (span (offset 31798) (line 600) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 31798) (line 600) (column 81) (len 10)))))
    (reference r522 (scope relative) (span (offset 31810) (line 600) (column 93) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 31810) (line 600) (column 93) (len 26)))))
    (reference r523 (scope relative) (span (offset 32006) (line 604) (column 73) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 32006) (line 604) (column 73) (len 19)))))
    (reference r524 (scope relative) (span (offset 32565) (line 617) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 32565) (line 617) (column 28) (len 4)))))
    (reference r525 (scope relative) (span (offset 32560) (line 617) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 32560) (line 617) (column 23) (len 3)))))
    (reference r526 (scope relative) (span (offset 32599) (line 618) (column 29) (len 49)) (segments (segment 0 (token "SpecificHeatCapacityAtSaturatedVapourPressureUnit") (name "SpecificHeatCapacityAtSaturatedVapourPressureUnit") (separator none) (span (offset 32599) (line 618) (column 29) (len 49)))))
    (reference r527 (scope relative) (span (offset 32593) (line 618) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 32593) (line 618) (column 23) (len 4)))))
    (reference r528 (scope relative) (span (offset 32721) (line 621) (column 62) (len 50)) (segments (segment 0 (token "SpecificHeatCapacityAtSaturatedVapourPressureValue") (name "SpecificHeatCapacityAtSaturatedVapourPressureValue") (separator none) (span (offset 32721) (line 621) (column 62) (len 50)))))
    (reference r529 (scope relative) (span (offset 32878) (line 623) (column 72) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 32878) (line 623) (column 72) (len 11)))))
    (reference r530 (scope relative) (span (offset 32928) (line 624) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 32928) (line 624) (column 37) (len 19)))))
    (reference r531 (scope relative) (span (offset 32957) (line 624) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 32957) (line 624) (column 66) (len 8)))))
    (reference r532 (scope relative) (span (offset 32968) (line 624) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 32968) (line 624) (column 77) (len 3)))))
    (reference r533 (scope relative) (span (offset 32972) (line 624) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 32972) (line 624) (column 81) (len 1)))))
    (reference r534 (scope relative) (span (offset 32979) (line 624) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 32979) (line 624) (column 88) (len 8)))))
    (reference r535 (scope relative) (span (offset 33033) (line 625) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 33033) (line 625) (column 39) (len 19)))))
    (reference r536 (scope relative) (span (offset 33062) (line 625) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 33062) (line 625) (column 68) (len 8)))))
    (reference r537 (scope relative) (span (offset 33073) (line 625) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 33073) (line 625) (column 79) (len 3)))))
    (reference r538 (scope relative) (span (offset 33077) (line 625) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 33077) (line 625) (column 83) (len 1)))))
    (reference r539 (scope relative) (span (offset 33084) (line 625) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 33084) (line 625) (column 90) (len 8)))))
    (reference r540 (scope relative) (span (offset 33155) (line 626) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 33155) (line 626) (column 55) (len 19)))))
    (reference r541 (scope relative) (span (offset 33184) (line 626) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 33184) (line 626) (column 84) (len 8)))))
    (reference r542 (scope relative) (span (offset 33195) (line 626) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 33195) (line 626) (column 95) (len 3)))))
    (reference r543 (scope relative) (span (offset 33199) (line 626) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 33199) (line 626) (column 99) (len 4)))))
    (reference r544 (scope relative) (span (offset 33209) (line 626) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 33209) (line 626) (column 109) (len 8)))))
    (reference r545 (scope relative) (span (offset 33248) (line 627) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 33248) (line 627) (column 23) (len 17)))))
    (reference r546 (scope relative) (span (offset 33272) (line 627) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 33272) (line 627) (column 47) (len 20)))))
    (reference r547 (scope relative) (span (offset 33296) (line 627) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 33296) (line 627) (column 71) (len 8)))))
    (reference r548 (scope relative) (span (offset 33306) (line 627) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 33306) (line 627) (column 81) (len 10)))))
    (reference r549 (scope relative) (span (offset 33318) (line 627) (column 93) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 33318) (line 627) (column 93) (len 26)))))
    (reference r550 (scope relative) (span (offset 33480) (line 631) (column 57) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 33480) (line 631) (column 57) (len 17)))))
    (reference r551 (scope relative) (span (offset 34347) (line 645) (column 46) (len 34)) (segments (segment 0 (token "RatioOfSpecificHeatCapacitiesValue") (name "RatioOfSpecificHeatCapacitiesValue") (separator none) (span (offset 34347) (line 645) (column 46) (len 34)))))
    (reference r552 (scope relative) (span (offset 34532) (line 648) (column 46) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 34532) (line 648) (column 46) (len 17)))))
    (reference r553 (scope relative) (span (offset 35273) (line 662) (column 35) (len 23)) (segments (segment 0 (token "IsentropicExponentValue") (name "IsentropicExponentValue") (separator none) (span (offset 35273) (line 662) (column 35) (len 23)))))
    (reference r554 (scope relative) (span (offset 35359) (line 664) (column 41) (len 18)) (segments (segment 0 (token "isentropicExponent") (name "isentropicExponent") (separator none) (span (offset 35359) (line 664) (column 41) (len 18)))))
    (reference r555 (scope relative) (span (offset 35454) (line 667) (column 35) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 35454) (line 667) (column 35) (len 19)))))
    (reference r556 (scope relative) (span (offset 36081) (line 680) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 36081) (line 680) (column 28) (len 4)))))
    (reference r557 (scope relative) (span (offset 36076) (line 680) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 36076) (line 680) (column 23) (len 3)))))
    (reference r558 (scope relative) (span (offset 36115) (line 681) (column 29) (len 11)) (segments (segment 0 (token "EntropyUnit") (name "EntropyUnit") (separator none) (span (offset 36115) (line 681) (column 29) (len 11)))))
    (reference r559 (scope relative) (span (offset 36109) (line 681) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 36109) (line 681) (column 23) (len 4)))))
    (reference r560 (scope relative) (span (offset 36161) (line 684) (column 24) (len 12)) (segments (segment 0 (token "EntropyValue") (name "EntropyValue") (separator none) (span (offset 36161) (line 684) (column 24) (len 12)))))
    (reference r561 (scope relative) (span (offset 36242) (line 686) (column 34) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 36242) (line 686) (column 34) (len 11)))))
    (reference r562 (scope relative) (span (offset 36292) (line 687) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 36292) (line 687) (column 37) (len 19)))))
    (reference r563 (scope relative) (span (offset 36321) (line 687) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 36321) (line 687) (column 66) (len 8)))))
    (reference r564 (scope relative) (span (offset 36332) (line 687) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 36332) (line 687) (column 77) (len 3)))))
    (reference r565 (scope relative) (span (offset 36336) (line 687) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 36336) (line 687) (column 81) (len 1)))))
    (reference r566 (scope relative) (span (offset 36343) (line 687) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 36343) (line 687) (column 88) (len 8)))))
    (reference r567 (scope relative) (span (offset 36393) (line 688) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 36393) (line 688) (column 35) (len 19)))))
    (reference r568 (scope relative) (span (offset 36422) (line 688) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 36422) (line 688) (column 64) (len 8)))))
    (reference r569 (scope relative) (span (offset 36433) (line 688) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 36433) (line 688) (column 75) (len 3)))))
    (reference r570 (scope relative) (span (offset 36437) (line 688) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 36437) (line 688) (column 79) (len 1)))))
    (reference r571 (scope relative) (span (offset 36444) (line 688) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 36444) (line 688) (column 86) (len 8)))))
    (reference r572 (scope relative) (span (offset 36498) (line 689) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 36498) (line 689) (column 39) (len 19)))))
    (reference r573 (scope relative) (span (offset 36527) (line 689) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 36527) (line 689) (column 68) (len 8)))))
    (reference r574 (scope relative) (span (offset 36538) (line 689) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 36538) (line 689) (column 79) (len 3)))))
    (reference r575 (scope relative) (span (offset 36542) (line 689) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 36542) (line 689) (column 83) (len 1)))))
    (reference r576 (scope relative) (span (offset 36549) (line 689) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 36549) (line 689) (column 90) (len 8)))))
    (reference r577 (scope relative) (span (offset 36620) (line 690) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 36620) (line 690) (column 55) (len 19)))))
    (reference r578 (scope relative) (span (offset 36649) (line 690) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 36649) (line 690) (column 84) (len 8)))))
    (reference r579 (scope relative) (span (offset 36660) (line 690) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 36660) (line 690) (column 95) (len 3)))))
    (reference r580 (scope relative) (span (offset 36664) (line 690) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 36664) (line 690) (column 99) (len 4)))))
    (reference r581 (scope relative) (span (offset 36674) (line 690) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 36674) (line 690) (column 109) (len 8)))))
    (reference r582 (scope relative) (span (offset 36713) (line 691) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 36713) (line 691) (column 23) (len 17)))))
    (reference r583 (scope relative) (span (offset 36737) (line 691) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 36737) (line 691) (column 47) (len 20)))))
    (reference r584 (scope relative) (span (offset 36761) (line 691) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 36761) (line 691) (column 71) (len 8)))))
    (reference r585 (scope relative) (span (offset 36771) (line 691) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 36771) (line 691) (column 81) (len 6)))))
    (reference r586 (scope relative) (span (offset 36779) (line 691) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 36779) (line 691) (column 89) (len 10)))))
    (reference r587 (scope relative) (span (offset 36791) (line 691) (column 101) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 36791) (line 691) (column 101) (len 26)))))
    (reference r588 (scope relative) (span (offset 36920) (line 695) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 36920) (line 695) (column 43) (len 19)))))
    (reference r589 (scope relative) (span (offset 37503) (line 708) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 37503) (line 708) (column 28) (len 4)))))
    (reference r590 (scope relative) (span (offset 37498) (line 708) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 37498) (line 708) (column 23) (len 3)))))
    (reference r591 (scope relative) (span (offset 37537) (line 709) (column 29) (len 19)) (segments (segment 0 (token "SpecificEntropyUnit") (name "SpecificEntropyUnit") (separator none) (span (offset 37537) (line 709) (column 29) (len 19)))))
    (reference r592 (scope relative) (span (offset 37531) (line 709) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 37531) (line 709) (column 23) (len 4)))))
    (reference r593 (scope relative) (span (offset 37599) (line 712) (column 32) (len 20)) (segments (segment 0 (token "SpecificEntropyValue") (name "SpecificEntropyValue") (separator none) (span (offset 37599) (line 712) (column 32) (len 20)))))
    (reference r594 (scope relative) (span (offset 37696) (line 714) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 37696) (line 714) (column 42) (len 11)))))
    (reference r595 (scope relative) (span (offset 37746) (line 715) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 37746) (line 715) (column 37) (len 19)))))
    (reference r596 (scope relative) (span (offset 37775) (line 715) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 37775) (line 715) (column 66) (len 8)))))
    (reference r597 (scope relative) (span (offset 37786) (line 715) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 37786) (line 715) (column 77) (len 3)))))
    (reference r598 (scope relative) (span (offset 37790) (line 715) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 37790) (line 715) (column 81) (len 1)))))
    (reference r599 (scope relative) (span (offset 37797) (line 715) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 37797) (line 715) (column 88) (len 8)))))
    (reference r600 (scope relative) (span (offset 37851) (line 716) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 37851) (line 716) (column 39) (len 19)))))
    (reference r601 (scope relative) (span (offset 37880) (line 716) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 37880) (line 716) (column 68) (len 8)))))
    (reference r602 (scope relative) (span (offset 37891) (line 716) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 37891) (line 716) (column 79) (len 3)))))
    (reference r603 (scope relative) (span (offset 37895) (line 716) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 37895) (line 716) (column 83) (len 1)))))
    (reference r604 (scope relative) (span (offset 37902) (line 716) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 37902) (line 716) (column 90) (len 8)))))
    (reference r605 (scope relative) (span (offset 37973) (line 717) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 37973) (line 717) (column 55) (len 19)))))
    (reference r606 (scope relative) (span (offset 38002) (line 717) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 38002) (line 717) (column 84) (len 8)))))
    (reference r607 (scope relative) (span (offset 38013) (line 717) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 38013) (line 717) (column 95) (len 3)))))
    (reference r608 (scope relative) (span (offset 38017) (line 717) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 38017) (line 717) (column 99) (len 4)))))
    (reference r609 (scope relative) (span (offset 38027) (line 717) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 38027) (line 717) (column 109) (len 8)))))
    (reference r610 (scope relative) (span (offset 38066) (line 718) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 38066) (line 718) (column 23) (len 17)))))
    (reference r611 (scope relative) (span (offset 38090) (line 718) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 38090) (line 718) (column 47) (len 20)))))
    (reference r612 (scope relative) (span (offset 38114) (line 718) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 38114) (line 718) (column 71) (len 8)))))
    (reference r613 (scope relative) (span (offset 38124) (line 718) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 38124) (line 718) (column 81) (len 10)))))
    (reference r614 (scope relative) (span (offset 38136) (line 718) (column 93) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 38136) (line 718) (column 93) (len 26)))))
    (reference r615 (scope relative) (span (offset 38248) (line 722) (column 34) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 38248) (line 722) (column 34) (len 19)))))
    (reference r616 (scope relative) (span (offset 38861) (line 735) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 38861) (line 735) (column 28) (len 4)))))
    (reference r617 (scope relative) (span (offset 38856) (line 735) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 38856) (line 735) (column 23) (len 3)))))
    (reference r618 (scope relative) (span (offset 38895) (line 736) (column 29) (len 10)) (segments (segment 0 (token "EnergyUnit") (name "EnergyUnit") (separator none) (span (offset 38895) (line 736) (column 29) (len 10)))))
    (reference r619 (scope relative) (span (offset 38889) (line 736) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 38889) (line 736) (column 23) (len 4)))))
    (reference r620 (scope relative) (span (offset 38939) (line 739) (column 23) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 38939) (line 739) (column 23) (len 11)))))
    (reference r621 (scope relative) (span (offset 39018) (line 741) (column 33) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 39018) (line 741) (column 33) (len 11)))))
    (reference r622 (scope relative) (span (offset 39068) (line 742) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 39068) (line 742) (column 37) (len 19)))))
    (reference r623 (scope relative) (span (offset 39097) (line 742) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 39097) (line 742) (column 66) (len 8)))))
    (reference r624 (scope relative) (span (offset 39108) (line 742) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 39108) (line 742) (column 77) (len 3)))))
    (reference r625 (scope relative) (span (offset 39112) (line 742) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 39112) (line 742) (column 81) (len 1)))))
    (reference r626 (scope relative) (span (offset 39119) (line 742) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 39119) (line 742) (column 88) (len 8)))))
    (reference r627 (scope relative) (span (offset 39169) (line 743) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 39169) (line 743) (column 35) (len 19)))))
    (reference r628 (scope relative) (span (offset 39198) (line 743) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 39198) (line 743) (column 64) (len 8)))))
    (reference r629 (scope relative) (span (offset 39209) (line 743) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 39209) (line 743) (column 75) (len 3)))))
    (reference r630 (scope relative) (span (offset 39213) (line 743) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 39213) (line 743) (column 79) (len 1)))))
    (reference r631 (scope relative) (span (offset 39220) (line 743) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 39220) (line 743) (column 86) (len 8)))))
    (reference r632 (scope relative) (span (offset 39274) (line 744) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 39274) (line 744) (column 39) (len 19)))))
    (reference r633 (scope relative) (span (offset 39303) (line 744) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 39303) (line 744) (column 68) (len 8)))))
    (reference r634 (scope relative) (span (offset 39314) (line 744) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 39314) (line 744) (column 79) (len 3)))))
    (reference r635 (scope relative) (span (offset 39318) (line 744) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 39318) (line 744) (column 83) (len 1)))))
    (reference r636 (scope relative) (span (offset 39325) (line 744) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 39325) (line 744) (column 90) (len 8)))))
    (reference r637 (scope relative) (span (offset 39364) (line 745) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 39364) (line 745) (column 23) (len 17)))))
    (reference r638 (scope relative) (span (offset 39388) (line 745) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 39388) (line 745) (column 47) (len 20)))))
    (reference r639 (scope relative) (span (offset 39412) (line 745) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 39412) (line 745) (column 71) (len 8)))))
    (reference r640 (scope relative) (span (offset 39422) (line 745) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 39422) (line 745) (column 81) (len 6)))))
    (reference r641 (scope relative) (span (offset 39430) (line 745) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 39430) (line 745) (column 89) (len 10)))))
    (reference r642 (scope relative) (span (offset 39554) (line 749) (column 31) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 39554) (line 749) (column 31) (len 11)))))
    (reference r643 (scope relative) (span (offset 40351) (line 764) (column 35) (len 14)) (segments (segment 0 (token "internalEnergy") (name "internalEnergy") (separator none) (span (offset 40351) (line 764) (column 35) (len 14)))))
    (reference r644 (scope relative) (span (offset 40435) (line 767) (column 25) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 40435) (line 767) (column 25) (len 11)))))
    (reference r645 (scope relative) (span (offset 41142) (line 783) (column 32) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 41142) (line 783) (column 32) (len 11)))))
    (reference r646 (scope relative) (span (offset 41936) (line 798) (column 33) (len 15)) (segments (segment 0 (token "helmholtzEnergy") (name "helmholtzEnergy") (separator none) (span (offset 41936) (line 798) (column 33) (len 15)))))
    (reference r647 (scope relative) (span (offset 42044) (line 801) (column 28) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 42044) (line 801) (column 28) (len 11)))))
    (reference r648 (scope relative) (span (offset 42788) (line 816) (column 29) (len 11)) (segments (segment 0 (token "gibbsEnergy") (name "gibbsEnergy") (separator none) (span (offset 42788) (line 816) (column 29) (len 11)))))
    (reference r649 (scope relative) (span (offset 42893) (line 819) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 42893) (line 819) (column 42) (len 19)))))
    (reference r650 (scope relative) (span (offset 43387) (line 832) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 43387) (line 832) (column 28) (len 4)))))
    (reference r651 (scope relative) (span (offset 43382) (line 832) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 43382) (line 832) (column 23) (len 3)))))
    (reference r652 (scope relative) (span (offset 43421) (line 833) (column 29) (len 18)) (segments (segment 0 (token "SpecificEnergyUnit") (name "SpecificEnergyUnit") (separator none) (span (offset 43421) (line 833) (column 29) (len 18)))))
    (reference r653 (scope relative) (span (offset 43415) (line 833) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 43415) (line 833) (column 23) (len 4)))))
    (reference r654 (scope relative) (span (offset 43481) (line 836) (column 31) (len 19)) (segments (segment 0 (token "SpecificEnergyValue") (name "SpecificEnergyValue") (separator none) (span (offset 43481) (line 836) (column 31) (len 19)))))
    (reference r655 (scope relative) (span (offset 43576) (line 838) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 43576) (line 838) (column 41) (len 11)))))
    (reference r656 (scope relative) (span (offset 43626) (line 839) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 43626) (line 839) (column 37) (len 19)))))
    (reference r657 (scope relative) (span (offset 43655) (line 839) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 43655) (line 839) (column 66) (len 8)))))
    (reference r658 (scope relative) (span (offset 43666) (line 839) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 43666) (line 839) (column 77) (len 3)))))
    (reference r659 (scope relative) (span (offset 43670) (line 839) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 43670) (line 839) (column 81) (len 1)))))
    (reference r660 (scope relative) (span (offset 43677) (line 839) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 43677) (line 839) (column 88) (len 8)))))
    (reference r661 (scope relative) (span (offset 43731) (line 840) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 43731) (line 840) (column 39) (len 19)))))
    (reference r662 (scope relative) (span (offset 43760) (line 840) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 43760) (line 840) (column 68) (len 8)))))
    (reference r663 (scope relative) (span (offset 43771) (line 840) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 43771) (line 840) (column 79) (len 3)))))
    (reference r664 (scope relative) (span (offset 43775) (line 840) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 43775) (line 840) (column 83) (len 1)))))
    (reference r665 (scope relative) (span (offset 43782) (line 840) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 43782) (line 840) (column 90) (len 8)))))
    (reference r666 (scope relative) (span (offset 43821) (line 841) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 43821) (line 841) (column 23) (len 17)))))
    (reference r667 (scope relative) (span (offset 43845) (line 841) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 43845) (line 841) (column 47) (len 20)))))
    (reference r668 (scope relative) (span (offset 43869) (line 841) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 43869) (line 841) (column 71) (len 8)))))
    (reference r669 (scope relative) (span (offset 43879) (line 841) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 43879) (line 841) (column 81) (len 10)))))
    (reference r670 (scope relative) (span (offset 44029) (line 845) (column 39) (len 19)) (segments (segment 0 (token "SpecificEnergyValue") (name "SpecificEnergyValue") (separator none) (span (offset 44029) (line 845) (column 39) (len 19)))))
    (reference r671 (scope relative) (span (offset 44660) (line 860) (column 43) (len 22)) (segments (segment 0 (token "specificInternalEnergy") (name "specificInternalEnergy") (separator none) (span (offset 44660) (line 860) (column 43) (len 22)))))
    (reference r672 (scope relative) (span (offset 44780) (line 863) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 44780) (line 863) (column 44) (len 19)))))
    (reference r673 (scope relative) (span (offset 45282) (line 876) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 45282) (line 876) (column 28) (len 4)))))
    (reference r674 (scope relative) (span (offset 45277) (line 876) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 45277) (line 876) (column 23) (len 3)))))
    (reference r675 (scope relative) (span (offset 45316) (line 877) (column 29) (len 20)) (segments (segment 0 (token "SpecificEnthalpyUnit") (name "SpecificEnthalpyUnit") (separator none) (span (offset 45316) (line 877) (column 29) (len 20)))))
    (reference r676 (scope relative) (span (offset 45310) (line 877) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 45310) (line 877) (column 23) (len 4)))))
    (reference r677 (scope relative) (span (offset 45380) (line 880) (column 33) (len 21)) (segments (segment 0 (token "SpecificEnthalpyValue") (name "SpecificEnthalpyValue") (separator none) (span (offset 45380) (line 880) (column 33) (len 21)))))
    (reference r678 (scope relative) (span (offset 45479) (line 882) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 45479) (line 882) (column 43) (len 11)))))
    (reference r679 (scope relative) (span (offset 45529) (line 883) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 45529) (line 883) (column 37) (len 19)))))
    (reference r680 (scope relative) (span (offset 45558) (line 883) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 45558) (line 883) (column 66) (len 8)))))
    (reference r681 (scope relative) (span (offset 45569) (line 883) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 45569) (line 883) (column 77) (len 3)))))
    (reference r682 (scope relative) (span (offset 45573) (line 883) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 45573) (line 883) (column 81) (len 1)))))
    (reference r683 (scope relative) (span (offset 45580) (line 883) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 45580) (line 883) (column 88) (len 8)))))
    (reference r684 (scope relative) (span (offset 45634) (line 884) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 45634) (line 884) (column 39) (len 19)))))
    (reference r685 (scope relative) (span (offset 45663) (line 884) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 45663) (line 884) (column 68) (len 8)))))
    (reference r686 (scope relative) (span (offset 45674) (line 884) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 45674) (line 884) (column 79) (len 3)))))
    (reference r687 (scope relative) (span (offset 45678) (line 884) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 45678) (line 884) (column 83) (len 1)))))
    (reference r688 (scope relative) (span (offset 45685) (line 884) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 45685) (line 884) (column 90) (len 8)))))
    (reference r689 (scope relative) (span (offset 45724) (line 885) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 45724) (line 885) (column 23) (len 17)))))
    (reference r690 (scope relative) (span (offset 45748) (line 885) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 45748) (line 885) (column 47) (len 20)))))
    (reference r691 (scope relative) (span (offset 45772) (line 885) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 45772) (line 885) (column 71) (len 8)))))
    (reference r692 (scope relative) (span (offset 45782) (line 885) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 45782) (line 885) (column 81) (len 10)))))
    (reference r693 (scope relative) (span (offset 45932) (line 889) (column 40) (len 19)) (segments (segment 0 (token "SpecificEnergyValue") (name "SpecificEnergyValue") (separator none) (span (offset 45932) (line 889) (column 40) (len 19)))))
    (reference r694 (scope relative) (span (offset 46651) (line 904) (column 41) (len 23)) (segments (segment 0 (token "specificHelmholtzEnergy") (name "specificHelmholtzEnergy") (separator none) (span (offset 46651) (line 904) (column 41) (len 23)))))
    (reference r695 (scope relative) (span (offset 46793) (line 907) (column 36) (len 19)) (segments (segment 0 (token "SpecificEnergyValue") (name "SpecificEnergyValue") (separator none) (span (offset 46793) (line 907) (column 36) (len 19)))))
    (reference r696 (scope relative) (span (offset 47483) (line 922) (column 37) (len 19)) (segments (segment 0 (token "specificGibbsEnergy") (name "specificGibbsEnergy") (separator none) (span (offset 47483) (line 922) (column 37) (len 19)))))
    (reference r697 (scope relative) (span (offset 47596) (line 925) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 47596) (line 925) (column 43) (len 19)))))
    (reference r698 (scope relative) (span (offset 48169) (line 938) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 48169) (line 938) (column 28) (len 4)))))
    (reference r699 (scope relative) (span (offset 48164) (line 938) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 48164) (line 938) (column 23) (len 3)))))
    (reference r700 (scope relative) (span (offset 48203) (line 939) (column 29) (len 19)) (segments (segment 0 (token "MassieuFunctionUnit") (name "MassieuFunctionUnit") (separator none) (span (offset 48203) (line 939) (column 29) (len 19)))))
    (reference r701 (scope relative) (span (offset 48197) (line 939) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 48197) (line 939) (column 23) (len 4)))))
    (reference r702 (scope relative) (span (offset 48265) (line 942) (column 32) (len 20)) (segments (segment 0 (token "MassieuFunctionValue") (name "MassieuFunctionValue") (separator none) (span (offset 48265) (line 942) (column 32) (len 20)))))
    (reference r703 (scope relative) (span (offset 48362) (line 944) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 48362) (line 944) (column 42) (len 11)))))
    (reference r704 (scope relative) (span (offset 48412) (line 945) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 48412) (line 945) (column 37) (len 19)))))
    (reference r705 (scope relative) (span (offset 48441) (line 945) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 48441) (line 945) (column 66) (len 8)))))
    (reference r706 (scope relative) (span (offset 48452) (line 945) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 48452) (line 945) (column 77) (len 3)))))
    (reference r707 (scope relative) (span (offset 48456) (line 945) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 48456) (line 945) (column 81) (len 1)))))
    (reference r708 (scope relative) (span (offset 48463) (line 945) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 48463) (line 945) (column 88) (len 8)))))
    (reference r709 (scope relative) (span (offset 48513) (line 946) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 48513) (line 946) (column 35) (len 19)))))
    (reference r710 (scope relative) (span (offset 48542) (line 946) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 48542) (line 946) (column 64) (len 8)))))
    (reference r711 (scope relative) (span (offset 48553) (line 946) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 48553) (line 946) (column 75) (len 3)))))
    (reference r712 (scope relative) (span (offset 48557) (line 946) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 48557) (line 946) (column 79) (len 1)))))
    (reference r713 (scope relative) (span (offset 48564) (line 946) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 48564) (line 946) (column 86) (len 8)))))
    (reference r714 (scope relative) (span (offset 48618) (line 947) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 48618) (line 947) (column 39) (len 19)))))
    (reference r715 (scope relative) (span (offset 48647) (line 947) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 48647) (line 947) (column 68) (len 8)))))
    (reference r716 (scope relative) (span (offset 48658) (line 947) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 48658) (line 947) (column 79) (len 3)))))
    (reference r717 (scope relative) (span (offset 48662) (line 947) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 48662) (line 947) (column 83) (len 1)))))
    (reference r718 (scope relative) (span (offset 48669) (line 947) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 48669) (line 947) (column 90) (len 8)))))
    (reference r719 (scope relative) (span (offset 48740) (line 948) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 48740) (line 948) (column 55) (len 19)))))
    (reference r720 (scope relative) (span (offset 48769) (line 948) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 48769) (line 948) (column 84) (len 8)))))
    (reference r721 (scope relative) (span (offset 48780) (line 948) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 48780) (line 948) (column 95) (len 3)))))
    (reference r722 (scope relative) (span (offset 48784) (line 948) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 48784) (line 948) (column 99) (len 4)))))
    (reference r723 (scope relative) (span (offset 48794) (line 948) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 48794) (line 948) (column 109) (len 8)))))
    (reference r724 (scope relative) (span (offset 48833) (line 949) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 48833) (line 949) (column 23) (len 17)))))
    (reference r725 (scope relative) (span (offset 48857) (line 949) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 48857) (line 949) (column 47) (len 20)))))
    (reference r726 (scope relative) (span (offset 48881) (line 949) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 48881) (line 949) (column 71) (len 8)))))
    (reference r727 (scope relative) (span (offset 48891) (line 949) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 48891) (line 949) (column 81) (len 6)))))
    (reference r728 (scope relative) (span (offset 48899) (line 949) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 48899) (line 949) (column 89) (len 10)))))
    (reference r729 (scope relative) (span (offset 48911) (line 949) (column 101) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 48911) (line 949) (column 101) (len 26)))))
    (reference r730 (scope relative) (span (offset 49038) (line 953) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 49038) (line 953) (column 42) (len 19)))))
    (reference r731 (scope relative) (span (offset 49599) (line 966) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 49599) (line 966) (column 28) (len 4)))))
    (reference r732 (scope relative) (span (offset 49594) (line 966) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 49594) (line 966) (column 23) (len 3)))))
    (reference r733 (scope relative) (span (offset 49633) (line 967) (column 29) (len 18)) (segments (segment 0 (token "PlanckFunctionUnit") (name "PlanckFunctionUnit") (separator none) (span (offset 49633) (line 967) (column 29) (len 18)))))
    (reference r734 (scope relative) (span (offset 49627) (line 967) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 49627) (line 967) (column 23) (len 4)))))
    (reference r735 (scope relative) (span (offset 49693) (line 970) (column 31) (len 19)) (segments (segment 0 (token "PlanckFunctionValue") (name "PlanckFunctionValue") (separator none) (span (offset 49693) (line 970) (column 31) (len 19)))))
    (reference r736 (scope relative) (span (offset 49788) (line 972) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 49788) (line 972) (column 41) (len 11)))))
    (reference r737 (scope relative) (span (offset 49838) (line 973) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 49838) (line 973) (column 37) (len 19)))))
    (reference r738 (scope relative) (span (offset 49867) (line 973) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 49867) (line 973) (column 66) (len 8)))))
    (reference r739 (scope relative) (span (offset 49878) (line 973) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 49878) (line 973) (column 77) (len 3)))))
    (reference r740 (scope relative) (span (offset 49882) (line 973) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 49882) (line 973) (column 81) (len 1)))))
    (reference r741 (scope relative) (span (offset 49889) (line 973) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 49889) (line 973) (column 88) (len 8)))))
    (reference r742 (scope relative) (span (offset 49939) (line 974) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 49939) (line 974) (column 35) (len 19)))))
    (reference r743 (scope relative) (span (offset 49968) (line 974) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 49968) (line 974) (column 64) (len 8)))))
    (reference r744 (scope relative) (span (offset 49979) (line 974) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 49979) (line 974) (column 75) (len 3)))))
    (reference r745 (scope relative) (span (offset 49983) (line 974) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 49983) (line 974) (column 79) (len 1)))))
    (reference r746 (scope relative) (span (offset 49990) (line 974) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 49990) (line 974) (column 86) (len 8)))))
    (reference r747 (scope relative) (span (offset 50044) (line 975) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 50044) (line 975) (column 39) (len 19)))))
    (reference r748 (scope relative) (span (offset 50073) (line 975) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 50073) (line 975) (column 68) (len 8)))))
    (reference r749 (scope relative) (span (offset 50084) (line 975) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 50084) (line 975) (column 79) (len 3)))))
    (reference r750 (scope relative) (span (offset 50088) (line 975) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 50088) (line 975) (column 83) (len 1)))))
    (reference r751 (scope relative) (span (offset 50095) (line 975) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 50095) (line 975) (column 90) (len 8)))))
    (reference r752 (scope relative) (span (offset 50166) (line 976) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 50166) (line 976) (column 55) (len 19)))))
    (reference r753 (scope relative) (span (offset 50195) (line 976) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 50195) (line 976) (column 84) (len 8)))))
    (reference r754 (scope relative) (span (offset 50206) (line 976) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 50206) (line 976) (column 95) (len 3)))))
    (reference r755 (scope relative) (span (offset 50210) (line 976) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 50210) (line 976) (column 99) (len 4)))))
    (reference r756 (scope relative) (span (offset 50220) (line 976) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 50220) (line 976) (column 109) (len 8)))))
    (reference r757 (scope relative) (span (offset 50259) (line 977) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 50259) (line 977) (column 23) (len 17)))))
    (reference r758 (scope relative) (span (offset 50283) (line 977) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 50283) (line 977) (column 47) (len 20)))))
    (reference r759 (scope relative) (span (offset 50307) (line 977) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 50307) (line 977) (column 71) (len 8)))))
    (reference r760 (scope relative) (span (offset 50317) (line 977) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 50317) (line 977) (column 81) (len 6)))))
    (reference r761 (scope relative) (span (offset 50325) (line 977) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 50325) (line 977) (column 89) (len 10)))))
    (reference r762 (scope relative) (span (offset 50337) (line 977) (column 101) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 50337) (line 977) (column 101) (len 26)))))
    (reference r763 (scope relative) (span (offset 50483) (line 981) (column 51) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 50483) (line 981) (column 51) (len 19)))))
    (reference r764 (scope relative) (span (offset 51174) (line 994) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 51174) (line 994) (column 28) (len 4)))))
    (reference r765 (scope relative) (span (offset 51169) (line 994) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 51169) (line 994) (column 23) (len 3)))))
    (reference r766 (scope relative) (span (offset 51208) (line 995) (column 29) (len 27)) (segments (segment 0 (token "JouleThomsonCoefficientUnit") (name "JouleThomsonCoefficientUnit") (separator none) (span (offset 51208) (line 995) (column 29) (len 27)))))
    (reference r767 (scope relative) (span (offset 51202) (line 995) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 51202) (line 995) (column 23) (len 4)))))
    (reference r768 (scope relative) (span (offset 51286) (line 998) (column 40) (len 28)) (segments (segment 0 (token "JouleThomsonCoefficientValue") (name "JouleThomsonCoefficientValue") (separator none) (span (offset 51286) (line 998) (column 40) (len 28)))))
    (reference r769 (scope relative) (span (offset 51399) (line 1000) (column 50) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 51399) (line 1000) (column 50) (len 11)))))
    (reference r770 (scope relative) (span (offset 51449) (line 1001) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 51449) (line 1001) (column 37) (len 19)))))
    (reference r771 (scope relative) (span (offset 51478) (line 1001) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 51478) (line 1001) (column 66) (len 8)))))
    (reference r772 (scope relative) (span (offset 51489) (line 1001) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 51489) (line 1001) (column 77) (len 3)))))
    (reference r773 (scope relative) (span (offset 51493) (line 1001) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 51493) (line 1001) (column 81) (len 1)))))
    (reference r774 (scope relative) (span (offset 51500) (line 1001) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 51500) (line 1001) (column 88) (len 8)))))
    (reference r775 (scope relative) (span (offset 51550) (line 1002) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 51550) (line 1002) (column 35) (len 19)))))
    (reference r776 (scope relative) (span (offset 51579) (line 1002) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 51579) (line 1002) (column 64) (len 8)))))
    (reference r777 (scope relative) (span (offset 51590) (line 1002) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 51590) (line 1002) (column 75) (len 3)))))
    (reference r778 (scope relative) (span (offset 51594) (line 1002) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 51594) (line 1002) (column 79) (len 1)))))
    (reference r779 (scope relative) (span (offset 51601) (line 1002) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 51601) (line 1002) (column 86) (len 8)))))
    (reference r780 (scope relative) (span (offset 51656) (line 1003) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 51656) (line 1003) (column 39) (len 19)))))
    (reference r781 (scope relative) (span (offset 51685) (line 1003) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 51685) (line 1003) (column 68) (len 8)))))
    (reference r782 (scope relative) (span (offset 51696) (line 1003) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 51696) (line 1003) (column 79) (len 3)))))
    (reference r783 (scope relative) (span (offset 51700) (line 1003) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 51700) (line 1003) (column 83) (len 1)))))
    (reference r784 (scope relative) (span (offset 51707) (line 1003) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 51707) (line 1003) (column 90) (len 8)))))
    (reference r785 (scope relative) (span (offset 51777) (line 1004) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 51777) (line 1004) (column 55) (len 19)))))
    (reference r786 (scope relative) (span (offset 51806) (line 1004) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 51806) (line 1004) (column 84) (len 8)))))
    (reference r787 (scope relative) (span (offset 51817) (line 1004) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 51817) (line 1004) (column 95) (len 3)))))
    (reference r788 (scope relative) (span (offset 51821) (line 1004) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 51821) (line 1004) (column 99) (len 4)))))
    (reference r789 (scope relative) (span (offset 51831) (line 1004) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 51831) (line 1004) (column 109) (len 8)))))
    (reference r790 (scope relative) (span (offset 51869) (line 1005) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 51869) (line 1005) (column 23) (len 17)))))
    (reference r791 (scope relative) (span (offset 51893) (line 1005) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 51893) (line 1005) (column 47) (len 20)))))
    (reference r792 (scope relative) (span (offset 51917) (line 1005) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 51917) (line 1005) (column 71) (len 8)))))
    (reference r793 (scope relative) (span (offset 51927) (line 1005) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 51927) (line 1005) (column 81) (len 6)))))
    (reference r794 (scope relative) (span (offset 51935) (line 1005) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 51935) (line 1005) (column 89) (len 10)))))
    (reference r795 (scope relative) (span (offset 51947) (line 1005) (column 101) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 51947) (line 1005) (column 101) (len 26)))))
    (reference r796 (scope relative) (span (offset 52082) (line 1009) (column 45) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 52082) (line 1009) (column 45) (len 17)))))
    (reference r797 (scope relative) (span (offset 52661) (line 1023) (column 34) (len 22)) (segments (segment 0 (token "ThermalEfficiencyValue") (name "ThermalEfficiencyValue") (separator none) (span (offset 52661) (line 1023) (column 34) (len 22)))))
    (reference r798 (scope relative) (span (offset 52818) (line 1026) (column 52) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 52818) (line 1026) (column 52) (len 17)))))
    (reference r799 (scope relative) (span (offset 53627) (line 1040) (column 41) (len 29)) (segments (segment 0 (token "MaximumThermalEfficiencyValue") (name "MaximumThermalEfficiencyValue") (separator none) (span (offset 53627) (line 1040) (column 41) (len 29)))))
    (reference r800 (scope relative) (span (offset 53779) (line 1043) (column 47) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 53779) (line 1043) (column 47) (len 19)))))
    (reference r801 (scope relative) (span (offset 54300) (line 1056) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 54300) (line 1056) (column 28) (len 4)))))
    (reference r802 (scope relative) (span (offset 54295) (line 1056) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 54295) (line 1056) (column 23) (len 3)))))
    (reference r803 (scope relative) (span (offset 54334) (line 1057) (column 29) (len 23)) (segments (segment 0 (token "SpecificGasConstantUnit") (name "SpecificGasConstantUnit") (separator none) (span (offset 54334) (line 1057) (column 29) (len 23)))))
    (reference r804 (scope relative) (span (offset 54328) (line 1057) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 54328) (line 1057) (column 23) (len 4)))))
    (reference r805 (scope relative) (span (offset 54404) (line 1060) (column 36) (len 24)) (segments (segment 0 (token "SpecificGasConstantValue") (name "SpecificGasConstantValue") (separator none) (span (offset 54404) (line 1060) (column 36) (len 24)))))
    (reference r806 (scope relative) (span (offset 54509) (line 1062) (column 46) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 54509) (line 1062) (column 46) (len 11)))))
    (reference r807 (scope relative) (span (offset 54559) (line 1063) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 54559) (line 1063) (column 37) (len 19)))))
    (reference r808 (scope relative) (span (offset 54588) (line 1063) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 54588) (line 1063) (column 66) (len 8)))))
    (reference r809 (scope relative) (span (offset 54599) (line 1063) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 54599) (line 1063) (column 77) (len 3)))))
    (reference r810 (scope relative) (span (offset 54603) (line 1063) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 54603) (line 1063) (column 81) (len 1)))))
    (reference r811 (scope relative) (span (offset 54610) (line 1063) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 54610) (line 1063) (column 88) (len 8)))))
    (reference r812 (scope relative) (span (offset 54664) (line 1064) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 54664) (line 1064) (column 39) (len 19)))))
    (reference r813 (scope relative) (span (offset 54693) (line 1064) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 54693) (line 1064) (column 68) (len 8)))))
    (reference r814 (scope relative) (span (offset 54704) (line 1064) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 54704) (line 1064) (column 79) (len 3)))))
    (reference r815 (scope relative) (span (offset 54708) (line 1064) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 54708) (line 1064) (column 83) (len 1)))))
    (reference r816 (scope relative) (span (offset 54715) (line 1064) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 54715) (line 1064) (column 90) (len 8)))))
    (reference r817 (scope relative) (span (offset 54786) (line 1065) (column 55) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 54786) (line 1065) (column 55) (len 19)))))
    (reference r818 (scope relative) (span (offset 54815) (line 1065) (column 84) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 54815) (line 1065) (column 84) (len 8)))))
    (reference r819 (scope relative) (span (offset 54826) (line 1065) (column 95) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 54826) (line 1065) (column 95) (len 3)))))
    (reference r820 (scope relative) (span (offset 54830) (line 1065) (column 99) (len 4)) (segments (segment 0 (token "'Θ'") (name "Θ") (separator none) (span (offset 54830) (line 1065) (column 99) (len 4)))))
    (reference r821 (scope relative) (span (offset 54840) (line 1065) (column 109) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 54840) (line 1065) (column 109) (len 8)))))
    (reference r822 (scope relative) (span (offset 54879) (line 1066) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 54879) (line 1066) (column 23) (len 17)))))
    (reference r823 (scope relative) (span (offset 54903) (line 1066) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 54903) (line 1066) (column 47) (len 20)))))
    (reference r824 (scope relative) (span (offset 54927) (line 1066) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 54927) (line 1066) (column 71) (len 8)))))
    (reference r825 (scope relative) (span (offset 54937) (line 1066) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 54937) (line 1066) (column 81) (len 10)))))
    (reference r826 (scope relative) (span (offset 54949) (line 1066) (column 93) (len 26)) (segments (segment 0 (token "thermodynamicTemperaturePF") (name "thermodynamicTemperaturePF") (separator none) (span (offset 54949) (line 1066) (column 93) (len 26)))))
    (reference r827 (scope relative) (span (offset 55098) (line 1070) (column 52) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 55098) (line 1070) (column 52) (len 19)))))
    (reference r828 (scope relative) (span (offset 55741) (line 1083) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 55741) (line 1083) (column 28) (len 4)))))
    (reference r829 (scope relative) (span (offset 55736) (line 1083) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 55736) (line 1083) (column 23) (len 3)))))
    (reference r830 (scope relative) (span (offset 55775) (line 1084) (column 29) (len 28)) (segments (segment 0 (token "MassConcentrationOfWaterUnit") (name "MassConcentrationOfWaterUnit") (separator none) (span (offset 55775) (line 1084) (column 29) (len 28)))))
    (reference r831 (scope relative) (span (offset 55769) (line 1084) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 55769) (line 1084) (column 23) (len 4)))))
    (reference r832 (scope relative) (span (offset 55855) (line 1087) (column 41) (len 29)) (segments (segment 0 (token "MassConcentrationOfWaterValue") (name "MassConcentrationOfWaterValue") (separator none) (span (offset 55855) (line 1087) (column 41) (len 29)))))
    (reference r833 (scope relative) (span (offset 55970) (line 1089) (column 51) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 55970) (line 1089) (column 51) (len 11)))))
    (reference r834 (scope relative) (span (offset 56020) (line 1090) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 56020) (line 1090) (column 37) (len 19)))))
    (reference r835 (scope relative) (span (offset 56049) (line 1090) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 56049) (line 1090) (column 66) (len 8)))))
    (reference r836 (scope relative) (span (offset 56060) (line 1090) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 56060) (line 1090) (column 77) (len 3)))))
    (reference r837 (scope relative) (span (offset 56064) (line 1090) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 56064) (line 1090) (column 81) (len 1)))))
    (reference r838 (scope relative) (span (offset 56071) (line 1090) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 56071) (line 1090) (column 88) (len 8)))))
    (reference r839 (scope relative) (span (offset 56122) (line 1091) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 56122) (line 1091) (column 35) (len 19)))))
    (reference r840 (scope relative) (span (offset 56151) (line 1091) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 56151) (line 1091) (column 64) (len 8)))))
    (reference r841 (scope relative) (span (offset 56162) (line 1091) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 56162) (line 1091) (column 75) (len 3)))))
    (reference r842 (scope relative) (span (offset 56166) (line 1091) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 56166) (line 1091) (column 79) (len 1)))))
    (reference r843 (scope relative) (span (offset 56173) (line 1091) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 56173) (line 1091) (column 86) (len 8)))))
    (reference r844 (scope relative) (span (offset 56211) (line 1092) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 56211) (line 1092) (column 23) (len 17)))))
    (reference r845 (scope relative) (span (offset 56235) (line 1092) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 56235) (line 1092) (column 47) (len 20)))))
    (reference r846 (scope relative) (span (offset 56259) (line 1092) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 56259) (line 1092) (column 71) (len 8)))))
    (reference r847 (scope relative) (span (offset 56269) (line 1092) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 56269) (line 1092) (column 81) (len 6)))))
    (reference r848 (scope relative) (span (offset 56445) (line 1096) (column 74) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 56445) (line 1096) (column 74) (len 19)))))
    (reference r849 (scope relative) (span (offset 57106) (line 1109) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 57106) (line 1109) (column 28) (len 4)))))
    (reference r850 (scope relative) (span (offset 57101) (line 1109) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 57101) (line 1109) (column 23) (len 3)))))
    (reference r851 (scope relative) (span (offset 57140) (line 1110) (column 29) (len 50)) (segments (segment 0 (token "MassConcentrationOfWaterVapourAbsoluteHumidityUnit") (name "MassConcentrationOfWaterVapourAbsoluteHumidityUnit") (separator none) (span (offset 57140) (line 1110) (column 29) (len 50)))))
    (reference r852 (scope relative) (span (offset 57134) (line 1110) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 57134) (line 1110) (column 23) (len 4)))))
    (reference r853 (scope relative) (span (offset 57264) (line 1113) (column 63) (len 51)) (segments (segment 0 (token "MassConcentrationOfWaterVapourAbsoluteHumidityValue") (name "MassConcentrationOfWaterVapourAbsoluteHumidityValue") (separator none) (span (offset 57264) (line 1113) (column 63) (len 51)))))
    (reference r854 (scope relative) (span (offset 57423) (line 1115) (column 73) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 57423) (line 1115) (column 73) (len 11)))))
    (reference r855 (scope relative) (span (offset 57473) (line 1116) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 57473) (line 1116) (column 37) (len 19)))))
    (reference r856 (scope relative) (span (offset 57502) (line 1116) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 57502) (line 1116) (column 66) (len 8)))))
    (reference r857 (scope relative) (span (offset 57513) (line 1116) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 57513) (line 1116) (column 77) (len 3)))))
    (reference r858 (scope relative) (span (offset 57517) (line 1116) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 57517) (line 1116) (column 81) (len 1)))))
    (reference r859 (scope relative) (span (offset 57524) (line 1116) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 57524) (line 1116) (column 88) (len 8)))))
    (reference r860 (scope relative) (span (offset 57575) (line 1117) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 57575) (line 1117) (column 35) (len 19)))))
    (reference r861 (scope relative) (span (offset 57604) (line 1117) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 57604) (line 1117) (column 64) (len 8)))))
    (reference r862 (scope relative) (span (offset 57615) (line 1117) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 57615) (line 1117) (column 75) (len 3)))))
    (reference r863 (scope relative) (span (offset 57619) (line 1117) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 57619) (line 1117) (column 79) (len 1)))))
    (reference r864 (scope relative) (span (offset 57626) (line 1117) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 57626) (line 1117) (column 86) (len 8)))))
    (reference r865 (scope relative) (span (offset 57664) (line 1118) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 57664) (line 1118) (column 23) (len 17)))))
    (reference r866 (scope relative) (span (offset 57688) (line 1118) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 57688) (line 1118) (column 47) (len 20)))))
    (reference r867 (scope relative) (span (offset 57712) (line 1118) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 57712) (line 1118) (column 71) (len 8)))))
    (reference r868 (scope relative) (span (offset 57722) (line 1118) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 57722) (line 1118) (column 81) (len 6)))))
    (reference r869 (scope relative) (span (offset 57860) (line 1122) (column 55) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 57860) (line 1122) (column 55) (len 17)))))
    (reference r870 (scope relative) (span (offset 58514) (line 1136) (column 44) (len 32)) (segments (segment 0 (token "MassRatioOfWaterToDryMatterValue") (name "MassRatioOfWaterToDryMatterValue") (separator none) (span (offset 58514) (line 1136) (column 44) (len 32)))))
    (reference r871 (scope relative) (span (offset 58696) (line 1139) (column 58) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 58696) (line 1139) (column 58) (len 17)))))
    (reference r872 (scope relative) (span (offset 59446) (line 1153) (column 47) (len 35)) (segments (segment 0 (token "MassRatioOfWaterVapourToDryGasValue") (name "MassRatioOfWaterVapourToDryGasValue") (separator none) (span (offset 59446) (line 1153) (column 47) (len 35)))))
    (reference r873 (scope relative) (span (offset 59605) (line 1156) (column 47) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 59605) (line 1156) (column 47) (len 17)))))
    (reference r874 (scope relative) (span (offset 60143) (line 1170) (column 36) (len 24)) (segments (segment 0 (token "MassFractionOfWaterValue") (name "MassFractionOfWaterValue") (separator none) (span (offset 60143) (line 1170) (column 36) (len 24)))))
    (reference r875 (scope relative) (span (offset 60300) (line 1173) (column 51) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 60300) (line 1173) (column 51) (len 17)))))
    (reference r876 (scope relative) (span (offset 60842) (line 1187) (column 40) (len 28)) (segments (segment 0 (token "MassFractionOfDryMatterValue") (name "MassFractionOfDryMatterValue") (separator none) (span (offset 60842) (line 1187) (column 40) (len 28)))))
    (reference r877 (scope relative) (span (offset 60986) (line 1190) (column 44) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 60986) (line 1190) (column 44) (len 17)))))
    (reference r878 (scope relative) (span (offset 61731) (line 1204) (column 33) (len 21)) (segments (segment 0 (token "RelativeHumidityValue") (name "RelativeHumidityValue") (separator none) (span (offset 61731) (line 1204) (column 33) (len 21)))))
    (reference r879 (scope relative) (span (offset 61905) (line 1207) (column 61) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 61905) (line 1207) (column 61) (len 17)))))
    (reference r880 (scope relative) (span (offset 62809) (line 1221) (column 50) (len 38)) (segments (segment 0 (token "RelativeMassConcentrationOfVapourValue") (name "RelativeMassConcentrationOfVapourValue") (separator none) (span (offset 62809) (line 1221) (column 50) (len 38)))))
    (reference r881 (scope relative) (span (offset 62984) (line 1224) (column 53) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 62984) (line 1224) (column 53) (len 17)))))
    (reference r882 (scope relative) (span (offset 63791) (line 1238) (column 42) (len 30)) (segments (segment 0 (token "RelativeMassRatioOfVapourValue") (name "RelativeMassRatioOfVapourValue") (separator none) (span (offset 63791) (line 1238) (column 42) (len 30)))))
    (reference r883 (scope relative) (span (offset 63933) (line 1241) (column 36) (len 29)) (segments (segment 0 (token "ThermodynamicTemperatureValue") (name "ThermodynamicTemperatureValue") (separator none) (span (offset 63933) (line 1241) (column 36) (len 29)))))
  )
  (root (library-package (name "ISQThermodynamics") (standard true) (body brace (doc) (import (target (span (span (offset 789) (line 15) (column 20) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 828) (line 16) (column 20) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 838) (line 16) (column 30) (len 3))) (separator (span (offset 838) (line 16) (column 30) (len 2))) (marker (span (offset 840) (line 16) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 862) (line 17) (column 20) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 883) (line 17) (column 41) (len 3))) (separator (span (offset 883) (line 17) (column 41) (len 2))) (marker (span (offset 885) (line 17) (column 43) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 907) (line 18) (column 20) (len 10))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 914) (line 18) (column 27) (len 3))) (separator (span (offset 914) (line 18) (column 27) (len 2))) (marker (span (offset 916) (line 18) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (alias (name "TemperatureUnit") (target (ref r4)) (body semicolon)) (alias (name "TemperatureValue") (target (ref r5)) (body semicolon)) (alias (name "temperature") (target (ref r6)) (body semicolon)) (attribute-def (declaration-name "CelsiusTemperatureValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r9)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r11)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "celsiusTemperature") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "CelsiusTemperatureUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r13)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r15)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2910) (line 51) (column 95) (len 8)) (member-access (base (expression (span (offset 2910) (line 51) (column 95) (len 3)) (ref r16))) (separator dot) (member (ref r17))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r18)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2935) (line 51) (column 120) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r19)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3009) (line 52) (column 70) (len 26)) (ref r21))))) (body semicolon)))))) (attribute-def (declaration-name "LinearExpansionCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r22)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r24)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r26)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "linearExpansionCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r27)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "LinearExpansionCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r28)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r29)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r30)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4135) (line 76) (column 95) (len 8)) (member-access (base (expression (span (offset 4135) (line 76) (column 95) (len 3)) (ref r31))) (separator dot) (member (ref r32))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r33)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4160) (line 76) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 4161) (line 76) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r34)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r35)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 4235) (line 77) (column 70) (len 26)) (ref r36))))) (body semicolon)))))) (attribute-def (declaration-name "CubicExpansionCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r37)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r38)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r39)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r40)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r41)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cubicExpansionCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r42)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "CubicExpansionCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r43)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r44)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r45)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5407) (line 101) (column 95) (len 8)) (member-access (base (expression (span (offset 5407) (line 101) (column 95) (len 3)) (ref r46))) (separator dot) (member (ref r47))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r48)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5432) (line 101) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 5433) (line 101) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r49)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r50)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5507) (line 102) (column 70) (len 26)) (ref r51))))) (body semicolon)))))) (attribute-def (declaration-name "RelativePressureCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r52)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r53)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r54)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r55)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r56)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "relativePressureCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r57)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "RelativePressureCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r58)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r59)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r60)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6714) (line 126) (column 95) (len 8)) (member-access (base (expression (span (offset 6714) (line 126) (column 95) (len 3)) (ref r61))) (separator dot) (member (ref r62))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r63)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6739) (line 126) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 6740) (line 126) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r64)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r65)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6814) (line 127) (column 70) (len 26)) (ref r66))))) (body semicolon)))))) (attribute-def (declaration-name "PressureCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r67)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r68)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r69)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r70)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r71)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "pressureCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r72)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "PressureCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r73)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r74)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r75)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7872) (line 151) (column 77) (len 5)) (member-access (base (expression (span (offset 7872) (line 151) (column 77) (len 3)) (ref r76))) (separator dot) (member (ref r77))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r78)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7894) (line 151) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 7895) (line 151) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r79)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r80)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7974) (line 152) (column 75) (len 5)) (member-access (base (expression (span (offset 7974) (line 152) (column 75) (len 3)) (ref r81))) (separator dot) (member (ref r82))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r83)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7996) (line 152) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r84)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r85)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8079) (line 153) (column 79) (len 5)) (member-access (base (expression (span (offset 8079) (line 153) (column 79) (len 3)) (ref r86))) (separator dot) (member (ref r87))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r88)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8101) (line 153) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 8102) (line 153) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r89)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r90)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8201) (line 154) (column 95) (len 8)) (member-access (base (expression (span (offset 8201) (line 154) (column 95) (len 3)) (ref r91))) (separator dot) (member (ref r92))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r93)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8226) (line 154) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 8227) (line 154) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r94)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r95)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 8301) (line 155) (column 70) (len 58)) (tuple (expression (span (offset 8302) (line 155) (column 71) (len 8)) (ref r96)) (expression (span (offset 8312) (line 155) (column 81) (len 6)) (ref r97)) (expression (span (offset 8320) (line 155) (column 89) (len 10)) (ref r98)) (expression (span (offset 8332) (line 155) (column 101) (len 26)) (ref r99))))))) (body semicolon)))))) (attribute-def (declaration-name "IsothermalCompressibilityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r100)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r101)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r102)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r103)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r104)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "isothermalCompressibility") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r105)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "IsothermalCompressibilityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r106)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r107)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r108)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9535) (line 179) (column 77) (len 5)) (member-access (base (expression (span (offset 9535) (line 179) (column 77) (len 3)) (ref r109))) (separator dot) (member (ref r110))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r111)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9557) (line 179) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r112)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r113)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9636) (line 180) (column 75) (len 5)) (member-access (base (expression (span (offset 9636) (line 180) (column 75) (len 3)) (ref r114))) (separator dot) (member (ref r115))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r116)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9658) (line 180) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 9659) (line 180) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r117)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r118)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9742) (line 181) (column 79) (len 5)) (member-access (base (expression (span (offset 9742) (line 181) (column 79) (len 3)) (ref r119))) (separator dot) (member (ref r120))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r121)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9764) (line 181) (column 101) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r122)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r123)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9838) (line 182) (column 70) (len 30)) (tuple (expression (span (offset 9839) (line 182) (column 71) (len 8)) (ref r124)) (expression (span (offset 9849) (line 182) (column 81) (len 6)) (ref r125)) (expression (span (offset 9857) (line 182) (column 89) (len 10)) (ref r126))))))) (body semicolon)))))) (attribute-def (declaration-name "IsentropicCompressibilityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r127)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r128)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r129)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r130)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r131)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "isentropicCompressibility") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r132)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "IsentropicCompressibilityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r133)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r134)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r135)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11023) (line 206) (column 77) (len 5)) (member-access (base (expression (span (offset 11023) (line 206) (column 77) (len 3)) (ref r136))) (separator dot) (member (ref r137))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r138)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11045) (line 206) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r139)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r140)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11124) (line 207) (column 75) (len 5)) (member-access (base (expression (span (offset 11124) (line 207) (column 75) (len 3)) (ref r141))) (separator dot) (member (ref r142))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r143)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11146) (line 207) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 11147) (line 207) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r144)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r145)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11230) (line 208) (column 79) (len 5)) (member-access (base (expression (span (offset 11230) (line 208) (column 79) (len 3)) (ref r146))) (separator dot) (member (ref r147))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r148)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11252) (line 208) (column 101) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r149)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r150)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11326) (line 209) (column 70) (len 30)) (tuple (expression (span (offset 11327) (line 209) (column 71) (len 8)) (ref r151)) (expression (span (offset 11337) (line 209) (column 81) (len 6)) (ref r152)) (expression (span (offset 11345) (line 209) (column 89) (len 10)) (ref r153))))))) (body semicolon)))))) (attribute-def (declaration-name "heat") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r154)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (alias (name "amountOfHeat") (target (ref r155)) (body semicolon)) (attribute-def (declaration-name "latentHeat") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r156)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "HeatFlowRateValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r157)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r158)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r159)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r160)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r161)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "heatFlowRate") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r162)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "HeatFlowRateUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r163)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r164)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r165)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13968) (line 267) (column 77) (len 5)) (member-access (base (expression (span (offset 13968) (line 267) (column 77) (len 3)) (ref r166))) (separator dot) (member (ref r167))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r168)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13990) (line 267) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r169)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r170)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14069) (line 268) (column 75) (len 5)) (member-access (base (expression (span (offset 14069) (line 268) (column 75) (len 3)) (ref r171))) (separator dot) (member (ref r172))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r173)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14091) (line 268) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r174)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r175)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14174) (line 269) (column 79) (len 5)) (member-access (base (expression (span (offset 14174) (line 269) (column 79) (len 3)) (ref r176))) (separator dot) (member (ref r177))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r178)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14196) (line 269) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 14197) (line 269) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r179)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r180)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14271) (line 270) (column 70) (len 30)) (tuple (expression (span (offset 14272) (line 270) (column 71) (len 8)) (ref r181)) (expression (span (offset 14282) (line 270) (column 81) (len 6)) (ref r182)) (expression (span (offset 14290) (line 270) (column 89) (len 10)) (ref r183))))))) (body semicolon)))))) (attribute-def (declaration-name "DensityOfHeatFlowRateValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r184)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r185)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r186)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r187)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r188)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "densityOfHeatFlowRate") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r189)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "DensityOfHeatFlowRateUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r190)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r191)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r192)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15276) (line 294) (column 75) (len 5)) (member-access (base (expression (span (offset 15276) (line 294) (column 75) (len 3)) (ref r193))) (separator dot) (member (ref r194))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r195)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15298) (line 294) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r196)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r197)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15381) (line 295) (column 79) (len 5)) (member-access (base (expression (span (offset 15381) (line 295) (column 79) (len 3)) (ref r198))) (separator dot) (member (ref r199))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r200)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15403) (line 295) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 15404) (line 295) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r201)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r202)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15478) (line 296) (column 70) (len 20)) (tuple (expression (span (offset 15479) (line 296) (column 71) (len 6)) (ref r203)) (expression (span (offset 15487) (line 296) (column 79) (len 10)) (ref r204))))))) (body semicolon)))))) (attribute-def (declaration-name "ThermalConductivityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r205)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r206)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r207)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r208)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r209)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "thermalConductivity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r210)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "ThermalConductivityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r211)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r212)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r213)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16467) (line 320) (column 77) (len 5)) (member-access (base (expression (span (offset 16467) (line 320) (column 77) (len 3)) (ref r214))) (separator dot) (member (ref r215))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r216)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16489) (line 320) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r217)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r218)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16568) (line 321) (column 75) (len 5)) (member-access (base (expression (span (offset 16568) (line 321) (column 75) (len 3)) (ref r219))) (separator dot) (member (ref r220))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r221)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16590) (line 321) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r222)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r223)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16673) (line 322) (column 79) (len 5)) (member-access (base (expression (span (offset 16673) (line 322) (column 79) (len 3)) (ref r224))) (separator dot) (member (ref r225))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r226)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16695) (line 322) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 16696) (line 322) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r227)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r228)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16795) (line 323) (column 95) (len 8)) (member-access (base (expression (span (offset 16795) (line 323) (column 95) (len 3)) (ref r229))) (separator dot) (member (ref r230))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r231)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16820) (line 323) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 16821) (line 323) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r232)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r233)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16895) (line 324) (column 70) (len 58)) (tuple (expression (span (offset 16896) (line 324) (column 71) (len 8)) (ref r234)) (expression (span (offset 16906) (line 324) (column 81) (len 6)) (ref r235)) (expression (span (offset 16914) (line 324) (column 89) (len 10)) (ref r236)) (expression (span (offset 16926) (line 324) (column 101) (len 26)) (ref r237))))))) (body semicolon)))))) (attribute-def (declaration-name "CoefficientOfHeatTransferValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r238)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r239)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r240)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r241)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r242)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "coefficientOfHeatTransfer") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r243)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "CoefficientOfHeatTransferUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r244)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r245)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r246)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18096) (line 348) (column 75) (len 5)) (member-access (base (expression (span (offset 18096) (line 348) (column 75) (len 3)) (ref r247))) (separator dot) (member (ref r248))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r249)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18118) (line 348) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r250)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r251)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18201) (line 349) (column 79) (len 5)) (member-access (base (expression (span (offset 18201) (line 349) (column 79) (len 3)) (ref r252))) (separator dot) (member (ref r253))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r254)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18223) (line 349) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 18224) (line 349) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r255)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r256)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18323) (line 350) (column 95) (len 8)) (member-access (base (expression (span (offset 18323) (line 350) (column 95) (len 3)) (ref r257))) (separator dot) (member (ref r258))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r259)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18348) (line 350) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 18349) (line 350) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r260)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r261)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18423) (line 351) (column 70) (len 48)) (tuple (expression (span (offset 18424) (line 351) (column 71) (len 6)) (ref r262)) (expression (span (offset 18432) (line 351) (column 79) (len 10)) (ref r263)) (expression (span (offset 18444) (line 351) (column 91) (len 26)) (ref r264))))))) (body semicolon)))))) (attribute-def (declaration-name "SurfaceCoefficientOfHeatTransferValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r265)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r266)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r267)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r268)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r269)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "surfaceCoefficientOfHeatTransfer") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r270)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "SurfaceCoefficientOfHeatTransferUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r271)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r272)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r273)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19763) (line 375) (column 75) (len 5)) (member-access (base (expression (span (offset 19763) (line 375) (column 75) (len 3)) (ref r274))) (separator dot) (member (ref r275))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r276)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19785) (line 375) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r277)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r278)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19868) (line 376) (column 79) (len 5)) (member-access (base (expression (span (offset 19868) (line 376) (column 79) (len 3)) (ref r279))) (separator dot) (member (ref r280))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r281)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19890) (line 376) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 19891) (line 376) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r282)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r283)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19990) (line 377) (column 95) (len 8)) (member-access (base (expression (span (offset 19990) (line 377) (column 95) (len 3)) (ref r284))) (separator dot) (member (ref r285))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r286)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20015) (line 377) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 20016) (line 377) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r287)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r288)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20090) (line 378) (column 70) (len 48)) (tuple (expression (span (offset 20091) (line 378) (column 71) (len 6)) (ref r289)) (expression (span (offset 20099) (line 378) (column 79) (len 10)) (ref r290)) (expression (span (offset 20111) (line 378) (column 91) (len 26)) (ref r291))))))) (body semicolon)))))) (attribute-def (declaration-name "ThermalInsulanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r292)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r293)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r294)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r295)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r296)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "thermalInsulance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r297)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "ThermalInsulanceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r298)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r299)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r300)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21197) (line 402) (column 75) (len 5)) (member-access (base (expression (span (offset 21197) (line 402) (column 75) (len 3)) (ref r301))) (separator dot) (member (ref r302))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r303)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21219) (line 402) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 21220) (line 402) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r304)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r305)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21303) (line 403) (column 79) (len 5)) (member-access (base (expression (span (offset 21303) (line 403) (column 79) (len 3)) (ref r306))) (separator dot) (member (ref r307))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r308)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21325) (line 403) (column 101) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r309)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r310)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21424) (line 404) (column 95) (len 8)) (member-access (base (expression (span (offset 21424) (line 404) (column 95) (len 3)) (ref r311))) (separator dot) (member (ref r312))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r313)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21449) (line 404) (column 120) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r314)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r315)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21523) (line 405) (column 70) (len 48)) (tuple (expression (span (offset 21524) (line 405) (column 71) (len 6)) (ref r316)) (expression (span (offset 21532) (line 405) (column 79) (len 10)) (ref r317)) (expression (span (offset 21544) (line 405) (column 91) (len 26)) (ref r318))))))) (body semicolon)))))) (alias (name "CoefficientOfThermalInsulanceUnit") (target (ref r319)) (body semicolon)) (alias (name "CoefficientOfThermalInsulanceValue") (target (ref r320)) (body semicolon)) (alias (name "coefficientOfThermalInsulance") (target (ref r321)) (body semicolon)) (attribute-def (declaration-name "ThermalResistanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r322)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r323)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r324)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r325)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r326)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "thermalResistance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r327)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "ThermalResistanceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r328)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r329)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r330)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22694) (line 433) (column 77) (len 5)) (member-access (base (expression (span (offset 22694) (line 433) (column 77) (len 3)) (ref r331))) (separator dot) (member (ref r332))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r333)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22716) (line 433) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 22717) (line 433) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r334)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r335)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22796) (line 434) (column 75) (len 5)) (member-access (base (expression (span (offset 22796) (line 434) (column 75) (len 3)) (ref r336))) (separator dot) (member (ref r337))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r338)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22818) (line 434) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 22819) (line 434) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r339)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r340)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22902) (line 435) (column 79) (len 5)) (member-access (base (expression (span (offset 22902) (line 435) (column 79) (len 3)) (ref r341))) (separator dot) (member (ref r342))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r343)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22924) (line 435) (column 101) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r344)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r345)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23023) (line 436) (column 95) (len 8)) (member-access (base (expression (span (offset 23023) (line 436) (column 95) (len 3)) (ref r346))) (separator dot) (member (ref r347))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r348)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23048) (line 436) (column 120) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r349)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r350)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23122) (line 437) (column 70) (len 58)) (tuple (expression (span (offset 23123) (line 437) (column 71) (len 8)) (ref r351)) (expression (span (offset 23133) (line 437) (column 81) (len 6)) (ref r352)) (expression (span (offset 23141) (line 437) (column 89) (len 10)) (ref r353)) (expression (span (offset 23153) (line 437) (column 101) (len 26)) (ref r354))))))) (body semicolon)))))) (attribute-def (declaration-name "ThermalConductanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r355)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r356)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r357)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r358)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r359)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "thermalConductance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r360)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "ThermalConductanceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r361)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r362)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r363)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24185) (line 461) (column 77) (len 5)) (member-access (base (expression (span (offset 24185) (line 461) (column 77) (len 3)) (ref r364))) (separator dot) (member (ref r365))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r366)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24207) (line 461) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r367)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r368)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24286) (line 462) (column 75) (len 5)) (member-access (base (expression (span (offset 24286) (line 462) (column 75) (len 3)) (ref r369))) (separator dot) (member (ref r370))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r371)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24308) (line 462) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r372)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r373)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24391) (line 463) (column 79) (len 5)) (member-access (base (expression (span (offset 24391) (line 463) (column 79) (len 3)) (ref r374))) (separator dot) (member (ref r375))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r376)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24413) (line 463) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 24414) (line 463) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r377)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r378)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24513) (line 464) (column 95) (len 8)) (member-access (base (expression (span (offset 24513) (line 464) (column 95) (len 3)) (ref r379))) (separator dot) (member (ref r380))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r381)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24538) (line 464) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 24539) (line 464) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r382)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r383)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24613) (line 465) (column 70) (len 58)) (tuple (expression (span (offset 24614) (line 465) (column 71) (len 8)) (ref r384)) (expression (span (offset 24624) (line 465) (column 81) (len 6)) (ref r385)) (expression (span (offset 24632) (line 465) (column 89) (len 10)) (ref r386)) (expression (span (offset 24644) (line 465) (column 101) (len 26)) (ref r387))))))) (body semicolon)))))) (attribute-def (declaration-name "ThermalDiffusivityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r388)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r389)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r390)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r391)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r392)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "thermalDiffusivity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r393)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "ThermalDiffusivityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r394)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r395)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r396)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 25732) (line 489) (column 77) (len 5)) (member-access (base (expression (span (offset 25732) (line 489) (column 77) (len 3)) (ref r397))) (separator dot) (member (ref r398))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r399)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 25754) (line 489) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r400)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r401)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 25837) (line 490) (column 79) (len 5)) (member-access (base (expression (span (offset 25837) (line 490) (column 79) (len 3)) (ref r402))) (separator dot) (member (ref r403))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r404)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 25859) (line 490) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 25860) (line 490) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r405)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r406)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 25934) (line 491) (column 70) (len 22)) (tuple (expression (span (offset 25935) (line 491) (column 71) (len 8)) (ref r407)) (expression (span (offset 25945) (line 491) (column 81) (len 10)) (ref r408))))))) (body semicolon)))))) (attribute-def (declaration-name "HeatCapacityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r409)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r410)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r411)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r412)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r413)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "heatCapacity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r414)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "HeatCapacityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r415)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r416)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r417)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27005) (line 515) (column 77) (len 5)) (member-access (base (expression (span (offset 27005) (line 515) (column 77) (len 3)) (ref r418))) (separator dot) (member (ref r419))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r420)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27027) (line 515) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r421)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r422)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27106) (line 516) (column 75) (len 5)) (member-access (base (expression (span (offset 27106) (line 516) (column 75) (len 3)) (ref r423))) (separator dot) (member (ref r424))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r425)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27128) (line 516) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r426)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r427)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27211) (line 517) (column 79) (len 5)) (member-access (base (expression (span (offset 27211) (line 517) (column 79) (len 3)) (ref r428))) (separator dot) (member (ref r429))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r430)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27233) (line 517) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 27234) (line 517) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r431)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r432)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27333) (line 518) (column 95) (len 8)) (member-access (base (expression (span (offset 27333) (line 518) (column 95) (len 3)) (ref r433))) (separator dot) (member (ref r434))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r435)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27358) (line 518) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 27359) (line 518) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r436)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r437)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 27433) (line 519) (column 70) (len 58)) (tuple (expression (span (offset 27434) (line 519) (column 71) (len 8)) (ref r438)) (expression (span (offset 27444) (line 519) (column 81) (len 6)) (ref r439)) (expression (span (offset 27452) (line 519) (column 89) (len 10)) (ref r440)) (expression (span (offset 27464) (line 519) (column 101) (len 26)) (ref r441))))))) (body semicolon)))))) (attribute-def (declaration-name "SpecificHeatCapacityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r442)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r443)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r444)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r445)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r446)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "specificHeatCapacity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r447)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "SpecificHeatCapacityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r448)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r449)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r450)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 28523) (line 543) (column 77) (len 5)) (member-access (base (expression (span (offset 28523) (line 543) (column 77) (len 3)) (ref r451))) (separator dot) (member (ref r452))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r453)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 28545) (line 543) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r454)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r455)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 28628) (line 544) (column 79) (len 5)) (member-access (base (expression (span (offset 28628) (line 544) (column 79) (len 3)) (ref r456))) (separator dot) (member (ref r457))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r458)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 28650) (line 544) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 28651) (line 544) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r459)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r460)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 28750) (line 545) (column 95) (len 8)) (member-access (base (expression (span (offset 28750) (line 545) (column 95) (len 3)) (ref r461))) (separator dot) (member (ref r462))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r463)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 28775) (line 545) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 28776) (line 545) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r464)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r465)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 28850) (line 546) (column 70) (len 50)) (tuple (expression (span (offset 28851) (line 546) (column 71) (len 8)) (ref r466)) (expression (span (offset 28861) (line 546) (column 81) (len 10)) (ref r467)) (expression (span (offset 28873) (line 546) (column 93) (len 26)) (ref r468))))))) (body semicolon)))))) (attribute-def (declaration-name "SpecificHeatCapacityAtConstantPressureValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r469)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r470)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r471)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r472)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r473)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "specificHeatCapacityAtConstantPressure") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r474)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "SpecificHeatCapacityAtConstantPressureUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r475)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r476)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r477)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30000) (line 570) (column 77) (len 5)) (member-access (base (expression (span (offset 30000) (line 570) (column 77) (len 3)) (ref r478))) (separator dot) (member (ref r479))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r480)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30022) (line 570) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r481)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r482)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30105) (line 571) (column 79) (len 5)) (member-access (base (expression (span (offset 30105) (line 571) (column 79) (len 3)) (ref r483))) (separator dot) (member (ref r484))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r485)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30127) (line 571) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 30128) (line 571) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r486)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r487)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30227) (line 572) (column 95) (len 8)) (member-access (base (expression (span (offset 30227) (line 572) (column 95) (len 3)) (ref r488))) (separator dot) (member (ref r489))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r490)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30252) (line 572) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 30253) (line 572) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r491)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r492)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30327) (line 573) (column 70) (len 50)) (tuple (expression (span (offset 30328) (line 573) (column 71) (len 8)) (ref r493)) (expression (span (offset 30338) (line 573) (column 81) (len 10)) (ref r494)) (expression (span (offset 30350) (line 573) (column 93) (len 26)) (ref r495))))))) (body semicolon)))))) (attribute-def (declaration-name "SpecificHeatCapacityAtConstantVolumeValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r496)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r497)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r498)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r499)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r500)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "specificHeatCapacityAtConstantVolume") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r501)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "SpecificHeatCapacityAtConstantVolumeUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r502)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r503)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r504)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31460) (line 597) (column 77) (len 5)) (member-access (base (expression (span (offset 31460) (line 597) (column 77) (len 3)) (ref r505))) (separator dot) (member (ref r506))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r507)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31482) (line 597) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r508)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r509)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31565) (line 598) (column 79) (len 5)) (member-access (base (expression (span (offset 31565) (line 598) (column 79) (len 3)) (ref r510))) (separator dot) (member (ref r511))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r512)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31587) (line 598) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 31588) (line 598) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r513)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r514)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31687) (line 599) (column 95) (len 8)) (member-access (base (expression (span (offset 31687) (line 599) (column 95) (len 3)) (ref r515))) (separator dot) (member (ref r516))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r517)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31712) (line 599) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 31713) (line 599) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r518)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r519)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31787) (line 600) (column 70) (len 50)) (tuple (expression (span (offset 31788) (line 600) (column 71) (len 8)) (ref r520)) (expression (span (offset 31798) (line 600) (column 81) (len 10)) (ref r521)) (expression (span (offset 31810) (line 600) (column 93) (len 26)) (ref r522))))))) (body semicolon)))))) (attribute-def (declaration-name "SpecificHeatCapacityAtSaturatedVapourPressureValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r523)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r524)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r525)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r526)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r527)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "specificHeatCapacityAtSaturatedVapourPressure") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r528)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "SpecificHeatCapacityAtSaturatedVapourPressureUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r529)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r530)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r531)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32968) (line 624) (column 77) (len 5)) (member-access (base (expression (span (offset 32968) (line 624) (column 77) (len 3)) (ref r532))) (separator dot) (member (ref r533))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r534)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32990) (line 624) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r535)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r536)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33073) (line 625) (column 79) (len 5)) (member-access (base (expression (span (offset 33073) (line 625) (column 79) (len 3)) (ref r537))) (separator dot) (member (ref r538))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r539)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33095) (line 625) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 33096) (line 625) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r540)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r541)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33195) (line 626) (column 95) (len 8)) (member-access (base (expression (span (offset 33195) (line 626) (column 95) (len 3)) (ref r542))) (separator dot) (member (ref r543))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r544)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33220) (line 626) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 33221) (line 626) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r545)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r546)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33295) (line 627) (column 70) (len 50)) (tuple (expression (span (offset 33296) (line 627) (column 71) (len 8)) (ref r547)) (expression (span (offset 33306) (line 627) (column 81) (len 10)) (ref r548)) (expression (span (offset 33318) (line 627) (column 93) (len 26)) (ref r549))))))) (body semicolon)))))) (attribute-def (declaration-name "RatioOfSpecificHeatCapacitiesValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r550)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "ratioOfSpecificHeatCapacities") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r551)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "IsentropicExponentValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r552)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "isentropicExponent") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r553)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (alias (name "isentropicExpansionFactor") (target (ref r554)) (body semicolon)) (attribute-def (declaration-name "EntropyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r555)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r556)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r557)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r558)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r559)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "entropy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r560)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "EntropyUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r561)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r562)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r563)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36332) (line 687) (column 77) (len 5)) (member-access (base (expression (span (offset 36332) (line 687) (column 77) (len 3)) (ref r564))) (separator dot) (member (ref r565))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r566)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36354) (line 687) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r567)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r568)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36433) (line 688) (column 75) (len 5)) (member-access (base (expression (span (offset 36433) (line 688) (column 75) (len 3)) (ref r569))) (separator dot) (member (ref r570))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r571)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36455) (line 688) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r572)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r573)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36538) (line 689) (column 79) (len 5)) (member-access (base (expression (span (offset 36538) (line 689) (column 79) (len 3)) (ref r574))) (separator dot) (member (ref r575))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r576)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36560) (line 689) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 36561) (line 689) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r577)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r578)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36660) (line 690) (column 95) (len 8)) (member-access (base (expression (span (offset 36660) (line 690) (column 95) (len 3)) (ref r579))) (separator dot) (member (ref r580))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r581)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36685) (line 690) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 36686) (line 690) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r582)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r583)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36760) (line 691) (column 70) (len 58)) (tuple (expression (span (offset 36761) (line 691) (column 71) (len 8)) (ref r584)) (expression (span (offset 36771) (line 691) (column 81) (len 6)) (ref r585)) (expression (span (offset 36779) (line 691) (column 89) (len 10)) (ref r586)) (expression (span (offset 36791) (line 691) (column 101) (len 26)) (ref r587))))))) (body semicolon)))))) (attribute-def (declaration-name "SpecificEntropyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r588)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r589)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r590)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r591)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r592)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "specificEntropy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r593)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "SpecificEntropyUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r594)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r595)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r596)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37786) (line 715) (column 77) (len 5)) (member-access (base (expression (span (offset 37786) (line 715) (column 77) (len 3)) (ref r597))) (separator dot) (member (ref r598))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r599)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37808) (line 715) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r600)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r601)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37891) (line 716) (column 79) (len 5)) (member-access (base (expression (span (offset 37891) (line 716) (column 79) (len 3)) (ref r602))) (separator dot) (member (ref r603))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r604)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37913) (line 716) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 37914) (line 716) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r605)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r606)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38013) (line 717) (column 95) (len 8)) (member-access (base (expression (span (offset 38013) (line 717) (column 95) (len 3)) (ref r607))) (separator dot) (member (ref r608))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r609)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38038) (line 717) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 38039) (line 717) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r610)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r611)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38113) (line 718) (column 70) (len 50)) (tuple (expression (span (offset 38114) (line 718) (column 71) (len 8)) (ref r612)) (expression (span (offset 38124) (line 718) (column 81) (len 10)) (ref r613)) (expression (span (offset 38136) (line 718) (column 93) (len 26)) (ref r614))))))) (body semicolon)))))) (attribute-def (declaration-name "EnergyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r615)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r616)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r617)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r618)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r619)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "energy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r620)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "EnergyUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r621)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r622)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r623)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 39108) (line 742) (column 77) (len 5)) (member-access (base (expression (span (offset 39108) (line 742) (column 77) (len 3)) (ref r624))) (separator dot) (member (ref r625))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r626)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 39130) (line 742) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r627)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r628)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 39209) (line 743) (column 75) (len 5)) (member-access (base (expression (span (offset 39209) (line 743) (column 75) (len 3)) (ref r629))) (separator dot) (member (ref r630))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r631)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 39231) (line 743) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r632)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r633)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 39314) (line 744) (column 79) (len 5)) (member-access (base (expression (span (offset 39314) (line 744) (column 79) (len 3)) (ref r634))) (separator dot) (member (ref r635))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r636)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 39336) (line 744) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 39337) (line 744) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r637)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r638)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 39411) (line 745) (column 70) (len 30)) (tuple (expression (span (offset 39412) (line 745) (column 71) (len 8)) (ref r639)) (expression (span (offset 39422) (line 745) (column 81) (len 6)) (ref r640)) (expression (span (offset 39430) (line 745) (column 89) (len 10)) (ref r641))))))) (body semicolon)))))) (attribute-def (declaration-name "internalEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r642)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (alias (name "thermodynamicEnergy") (target (ref r643)) (body semicolon)) (attribute-def (declaration-name "enthalpy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r644)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "helmholtzEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r645)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (alias (name "helmholtzFunction") (target (ref r646)) (body semicolon)) (attribute-def (declaration-name "gibbsEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r647)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (alias (name "gibbsFunction") (target (ref r648)) (body semicolon)) (attribute-def (declaration-name "SpecificEnergyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r649)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r650)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r651)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r652)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r653)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "specificEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r654)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "SpecificEnergyUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r655)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r656)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r657)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43666) (line 839) (column 77) (len 5)) (member-access (base (expression (span (offset 43666) (line 839) (column 77) (len 3)) (ref r658))) (separator dot) (member (ref r659))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r660)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43688) (line 839) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r661)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r662)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43771) (line 840) (column 79) (len 5)) (member-access (base (expression (span (offset 43771) (line 840) (column 79) (len 3)) (ref r663))) (separator dot) (member (ref r664))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r665)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43793) (line 840) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 43794) (line 840) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r666)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r667)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43868) (line 841) (column 70) (len 22)) (tuple (expression (span (offset 43869) (line 841) (column 71) (len 8)) (ref r668)) (expression (span (offset 43879) (line 841) (column 81) (len 10)) (ref r669))))))) (body semicolon)))))) (attribute-def (declaration-name "specificInternalEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r670)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (alias (name "specificThermodynamicEnergy") (target (ref r671)) (body semicolon)) (attribute-def (declaration-name "SpecificEnthalpyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r672)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r673)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r674)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r675)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r676)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "specificEnthalpy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r677)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "SpecificEnthalpyUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r678)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r679)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r680)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45569) (line 883) (column 77) (len 5)) (member-access (base (expression (span (offset 45569) (line 883) (column 77) (len 3)) (ref r681))) (separator dot) (member (ref r682))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r683)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45591) (line 883) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r684)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r685)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45674) (line 884) (column 79) (len 5)) (member-access (base (expression (span (offset 45674) (line 884) (column 79) (len 3)) (ref r686))) (separator dot) (member (ref r687))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r688)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45696) (line 884) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 45697) (line 884) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r689)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r690)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45771) (line 885) (column 70) (len 22)) (tuple (expression (span (offset 45772) (line 885) (column 71) (len 8)) (ref r691)) (expression (span (offset 45782) (line 885) (column 81) (len 10)) (ref r692))))))) (body semicolon)))))) (attribute-def (declaration-name "specificHelmholtzEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r693)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (alias (name "specificHelmholtzFunction") (target (ref r694)) (body semicolon)) (attribute-def (declaration-name "specificGibbsEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r695)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (alias (name "specificGibbsFunction") (target (ref r696)) (body semicolon)) (attribute-def (declaration-name "MassieuFunctionValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r697)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r698)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r699)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r700)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r701)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "massieuFunction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r702)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "MassieuFunctionUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r703)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r704)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r705)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48452) (line 945) (column 77) (len 5)) (member-access (base (expression (span (offset 48452) (line 945) (column 77) (len 3)) (ref r706))) (separator dot) (member (ref r707))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r708)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48474) (line 945) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r709)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r710)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48553) (line 946) (column 75) (len 5)) (member-access (base (expression (span (offset 48553) (line 946) (column 75) (len 3)) (ref r711))) (separator dot) (member (ref r712))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r713)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48575) (line 946) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r714)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r715)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48658) (line 947) (column 79) (len 5)) (member-access (base (expression (span (offset 48658) (line 947) (column 79) (len 3)) (ref r716))) (separator dot) (member (ref r717))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r718)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48680) (line 947) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 48681) (line 947) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r719)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r720)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48780) (line 948) (column 95) (len 8)) (member-access (base (expression (span (offset 48780) (line 948) (column 95) (len 3)) (ref r721))) (separator dot) (member (ref r722))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r723)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48805) (line 948) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 48806) (line 948) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r724)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r725)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48880) (line 949) (column 70) (len 58)) (tuple (expression (span (offset 48881) (line 949) (column 71) (len 8)) (ref r726)) (expression (span (offset 48891) (line 949) (column 81) (len 6)) (ref r727)) (expression (span (offset 48899) (line 949) (column 89) (len 10)) (ref r728)) (expression (span (offset 48911) (line 949) (column 101) (len 26)) (ref r729))))))) (body semicolon)))))) (attribute-def (declaration-name "PlanckFunctionValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r730)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r731)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r732)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r733)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r734)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "planckFunction") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r735)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "PlanckFunctionUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r736)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r737)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r738)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 49878) (line 973) (column 77) (len 5)) (member-access (base (expression (span (offset 49878) (line 973) (column 77) (len 3)) (ref r739))) (separator dot) (member (ref r740))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r741)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 49900) (line 973) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r742)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r743)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 49979) (line 974) (column 75) (len 5)) (member-access (base (expression (span (offset 49979) (line 974) (column 75) (len 3)) (ref r744))) (separator dot) (member (ref r745))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r746)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50001) (line 974) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r747)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r748)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50084) (line 975) (column 79) (len 5)) (member-access (base (expression (span (offset 50084) (line 975) (column 79) (len 3)) (ref r749))) (separator dot) (member (ref r750))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r751)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50106) (line 975) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 50107) (line 975) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r752)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r753)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50206) (line 976) (column 95) (len 8)) (member-access (base (expression (span (offset 50206) (line 976) (column 95) (len 3)) (ref r754))) (separator dot) (member (ref r755))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r756)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50231) (line 976) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 50232) (line 976) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r757)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r758)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50306) (line 977) (column 70) (len 58)) (tuple (expression (span (offset 50307) (line 977) (column 71) (len 8)) (ref r759)) (expression (span (offset 50317) (line 977) (column 81) (len 6)) (ref r760)) (expression (span (offset 50325) (line 977) (column 89) (len 10)) (ref r761)) (expression (span (offset 50337) (line 977) (column 101) (len 26)) (ref r762))))))) (body semicolon)))))) (attribute-def (declaration-name "JouleThomsonCoefficientValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r763)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r764)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r765)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r766)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r767)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "jouleThomsonCoefficient") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r768)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "JouleThomsonCoefficientUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r769)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r770)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r771)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51489) (line 1001) (column 77) (len 5)) (member-access (base (expression (span (offset 51489) (line 1001) (column 77) (len 3)) (ref r772))) (separator dot) (member (ref r773))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r774)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51511) (line 1001) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r775)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r776)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51590) (line 1002) (column 75) (len 5)) (member-access (base (expression (span (offset 51590) (line 1002) (column 75) (len 3)) (ref r777))) (separator dot) (member (ref r778))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r779)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51612) (line 1002) (column 97) (len 2)) (unary (operator "-") (operand (expression (span (offset 51613) (line 1002) (column 98) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r780)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r781)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51696) (line 1003) (column 79) (len 5)) (member-access (base (expression (span (offset 51696) (line 1003) (column 79) (len 3)) (ref r782))) (separator dot) (member (ref r783))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r784)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51718) (line 1003) (column 101) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r785)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r786)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51817) (line 1004) (column 95) (len 8)) (member-access (base (expression (span (offset 51817) (line 1004) (column 95) (len 3)) (ref r787))) (separator dot) (member (ref r788))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r789)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51842) (line 1004) (column 120) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r790)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r791)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 51916) (line 1005) (column 70) (len 58)) (tuple (expression (span (offset 51917) (line 1005) (column 71) (len 8)) (ref r792)) (expression (span (offset 51927) (line 1005) (column 81) (len 6)) (ref r793)) (expression (span (offset 51935) (line 1005) (column 89) (len 10)) (ref r794)) (expression (span (offset 51947) (line 1005) (column 101) (len 26)) (ref r795))))))) (body semicolon)))))) (attribute-def (declaration-name "ThermalEfficiencyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r796)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "thermalEfficiency") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r797)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "MaximumThermalEfficiencyValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r798)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "maximumThermalEfficiency") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r799)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "SpecificGasConstantValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r800)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r801)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r802)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r803)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r804)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "specificGasConstant") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r805)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "SpecificGasConstantUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r806)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r807)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r808)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 54599) (line 1063) (column 77) (len 5)) (member-access (base (expression (span (offset 54599) (line 1063) (column 77) (len 3)) (ref r809))) (separator dot) (member (ref r810))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r811)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 54621) (line 1063) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r812)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r813)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 54704) (line 1064) (column 79) (len 5)) (member-access (base (expression (span (offset 54704) (line 1064) (column 79) (len 3)) (ref r814))) (separator dot) (member (ref r815))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r816)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 54726) (line 1064) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 54727) (line 1064) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "thermodynamicTemperaturePF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r817)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r818)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 54826) (line 1065) (column 95) (len 8)) (member-access (base (expression (span (offset 54826) (line 1065) (column 95) (len 3)) (ref r819))) (separator dot) (member (ref r820))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r821)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 54851) (line 1065) (column 120) (len 2)) (unary (operator "-") (operand (expression (span (offset 54852) (line 1065) (column 121) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r822)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r823)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 54926) (line 1066) (column 70) (len 50)) (tuple (expression (span (offset 54927) (line 1066) (column 71) (len 8)) (ref r824)) (expression (span (offset 54937) (line 1066) (column 81) (len 10)) (ref r825)) (expression (span (offset 54949) (line 1066) (column 93) (len 26)) (ref r826))))))) (body semicolon)))))) (attribute-def (declaration-name "MassConcentrationOfWaterValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r827)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r828)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r829)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r830)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r831)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "massConcentrationOfWater") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r832)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "MassConcentrationOfWaterUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r833)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r834)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r835)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56060) (line 1090) (column 77) (len 5)) (member-access (base (expression (span (offset 56060) (line 1090) (column 77) (len 3)) (ref r836))) (separator dot) (member (ref r837))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r838)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56082) (line 1090) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 56083) (line 1090) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r839)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r840)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56162) (line 1091) (column 75) (len 5)) (member-access (base (expression (span (offset 56162) (line 1091) (column 75) (len 3)) (ref r841))) (separator dot) (member (ref r842))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r843)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56184) (line 1091) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r844)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r845)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 56258) (line 1092) (column 70) (len 18)) (tuple (expression (span (offset 56259) (line 1092) (column 71) (len 8)) (ref r846)) (expression (span (offset 56269) (line 1092) (column 81) (len 6)) (ref r847))))))) (body semicolon)))))) (attribute-def (declaration-name "MassConcentrationOfWaterVapourAbsoluteHumidityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r848)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r849)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r850)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r851)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r852)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "massConcentrationOfWaterVapourAbsoluteHumidity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r853)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique true)) (value none) (body semicolon)) (attribute-def (declaration-name "MassConcentrationOfWaterVapourAbsoluteHumidityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r854)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r855)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r856)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57513) (line 1116) (column 77) (len 5)) (member-access (base (expression (span (offset 57513) (line 1116) (column 77) (len 3)) (ref r857))) (separator dot) (member (ref r858))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r859)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57535) (line 1116) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 57536) (line 1116) (column 100) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r860)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r861)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57615) (line 1117) (column 75) (len 5)) (member-access (base (expression (span (offset 57615) (line 1117) (column 75) (len 3)) (ref r862))) (separator dot) (member (ref r863))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r864)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57637) (line 1117) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r865)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r866)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 57711) (line 1118) (column 70) (len 18)) (tuple (expression (span (offset 57712) (line 1118) (column 71) (len 8)) (ref r867)) (expression (span (offset 57722) (line 1118) (column 81) (len 6)) (ref r868))))))) (body semicolon)))))) (attribute-def (declaration-name "MassRatioOfWaterToDryMatterValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r869)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "massRatioOfWaterToDryMatter") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r870)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "MassRatioOfWaterVapourToDryGasValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r871)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "massRatioOfWaterVapourToDryGas") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r872)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "MassFractionOfWaterValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r873)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "massFractionOfWater") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r874)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "MassFractionOfDryMatterValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r875)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "massFractionOfDryMatter") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r876)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "RelativeHumidityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r877)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "relativeHumidity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r878)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "RelativeMassConcentrationOfVapourValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r879)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "relativeMassConcentrationOfVapour") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r880)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "RelativeMassRatioOfVapourValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r881)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))) (attribute-def (declaration-name "relativeMassRatioOfVapour") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r882)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body semicolon)) (attribute-def (declaration-name "dewPointTemperature") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r883)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (value none) (body brace (doc))))))
)
~~~
