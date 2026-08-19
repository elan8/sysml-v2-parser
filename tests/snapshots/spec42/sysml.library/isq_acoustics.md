# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/ISQAcoustics"))
~~~
# SOURCE
~~~sysml
standard library package ISQAcoustics {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-8:2020 "Acoustics"
     * see also https://www.iso.org/standard/64978.html
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
    private import ISQMechanics::PowerValue;
    private import ISQMechanics::PressureValue;
    private import ISQSpaceTime::CartesianSpatial3dCoordinateFrame;
    private import ISQSpaceTime::SpeedValue;
    private import ISQSpaceTime::CartesianVelocity3dCoordinateFrame;
    private import ISQSpaceTime::AccelerationValue;
    private import ISQSpaceTime::CartesianAcceleration3dCoordinateFrame;
    private import ISQThermodynamics::EnergyValue;

    /* ISO-80000-8 item 8-1 logarithmic frequency range */
    attribute def LogarithmicFrequencyRangeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-1 logarithmic frequency range
         * symbol(s): `G`
         * application domain: generic
         * name: LogarithmicFrequencyRange
         * quantity dimension: 1
         * measurement unit(s): oct, dec
         * tensor order: 0
         * definition: quantity given by: `G = log_2(f_2/f_1) "[oct]" = log_10(f_2/f_1) "[dec]"`, where `f_1` and `f_2` are two frequencies (ISO 80000-3)
         * remarks: One octave (oct) is the logarithmic frequency range between `f_1` and `f_2` when `f_2/f_1 = 2`. Similarly, one decade (dec) is the logarithmic frequency range between `f_1` and `f_2` when `f_2/f_1 = 10`; thus `1 "[dec]" = log_2(10) "[oct]" ≈ 3.322 "[oct]"`. ISO 266 specifies preferred frequencies for acoustics separated by logarithmic frequency ranges equal to one tenth of a decade (`0.1 "[dec]"`). Each `0.1 "[dec]"` logarithmic frequency range is referred to in ISO 266 as a "one-third-octave interval" because `0.1 "[dec]"` is approximately equal to `1/3 "[oct]"`. Similarly, a logarithmic frequency range of `0.3 "[dec]"` is referred to as a "one-octave interval" because `0.3 "[dec]"` is approximately equal to `1 "[oct]"`. A logarithmic frequency range equal to one tenth of a decade can be referred to as a decidecade.
         */
        attribute :>> num: Real;
        attribute :>> mRef: LogarithmicFrequencyRangeUnit[1];
    }

    attribute logarithmicFrequencyRange: LogarithmicFrequencyRangeValue[*] nonunique :> scalarQuantities;

    attribute def LogarithmicFrequencyRangeUnit :> DimensionOneUnit {
    }

    /* ISO-80000-8 item 8-2.1 static pressure */
    attribute staticPressure: PressureValue :> scalarQuantities {
        doc
        /*
         * source: item 8-2.1 static pressure
         * symbol(s): `p_s`
         * application domain: generic
         * name: StaticPressure (specializes Pressure)
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: pressure (ISO 80000-4) in a medium when no sound wave is present
         * remarks: This definition applies to a medium with zero flow.
         */
    }

    /* ISO-80000-8 item 8-2.2 sound pressure */
    attribute soundPressure: PressureValue :> scalarQuantities {
        doc
        /*
         * source: item 8-2.2 sound pressure
         * symbol(s): `p`
         * application domain: generic
         * name: SoundPressure (specializes Pressure)
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: difference between instantaneous total pressure and static pressure (item 8-2.1)
         * remarks: None.
         */
    }

    /* ISO-80000-8 item 8-3 sound particle displacement */
    attribute def CartesianSoundParticleDisplacement3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 8-3 sound particle displacement
         * symbol(s): `vec(δ)`
         * application domain: generic
         * name: SoundParticleDisplacement (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the instantaneous displacement (ISO 80000-3) of a particle in a medium from what would be its position in the absence of sound waves
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSpatial3dCoordinateFrame[1];
    }

    attribute cartesianSoundParticleDisplacement3dVector: CartesianSoundParticleDisplacement3dVector :> vectorQuantities;

    /* ISO-80000-8 item 8-4 sound particle velocity */
    attribute def CartesianSoundParticleVelocity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 8-4 sound particle velocity
         * symbol(s): `vec(u)`, `(vec(v))`
         * application domain: generic
         * name: SoundParticleVelocity (specializes Velocity)
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity given by: `vec(u) = del(vec(δ))/del(t)`, where `vec(δ)` is sound particle displacement (item 8-3) and `t` is time (ISO 80000-3)
         * remarks: The definition is limited to small-amplitude acoustic disturbances such that the magnitude of `vec(u)` is small relative to the phase speed (ISO 80000-3) of sound.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianVelocity3dCoordinateFrame[1];
    }

    attribute cartesianSoundParticleVelocity3dVector: CartesianSoundParticleVelocity3dVector :> vectorQuantities;

    /* ISO-80000-8 item 8-5 sound particle acceleration */
    attribute def CartesianSoundParticleAcceleration3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 8-5 sound particle acceleration
         * symbol(s): `vec(a)`
         * application domain: generic
         * name: SoundParticleAcceleration (specializes Acceleration)
         * quantity dimension: L^1*T^-2
         * measurement unit(s): m*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity given by: `vec(a) = (del(vec(u)))/(del(t))`, where `vec(u)` is sound particle velocity (item 8-4) and `t` is time
         * remarks: The definition is limited to small-amplitude acoustic disturbances such that the magnitude of `vec(u)` is small relative to the phase speed (ISO 80000-3) of sound.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianAcceleration3dCoordinateFrame[1];
    }

    attribute cartesianSoundParticleAcceleration3dVector: CartesianSoundParticleAcceleration3dVector :> vectorQuantities;

    /* ISO-80000-8 item 8-6 volume velocity, volume flow rate */
    attribute volumeVelocity: SpeedValue :> scalarQuantities {
        doc
        /*
         * source: item 8-6 volume velocity, volume flow rate
         * symbol(s): `q`, `q_v`
         * application domain: generic
         * name: VolumeVelocity (specializes Speed)
         * quantity dimension: L^3*T^-1
         * measurement unit(s): m^3*s^-1
         * tensor order: 0
         * definition: surface integral of the normal component of the sound particle velocity (item 8-4) over a defined surface
         * remarks: None.
         */
    }

    alias volumeFlowRate for volumeVelocity;

    /* ISO-80000-8 item 8-7 sound energy density */
    attribute def SoundEnergyDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-7 sound energy density
         * symbol(s): `w`
         * application domain: generic
         * name: SoundEnergyDensity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): J/m^3, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quantity given by: `w = 1/2 ρ_m u^2 + 1/2 p^2/(ρ_m c^2)`, where `ρ_m` is mean density (ISO 80000-4), `u` is the magnitude of the sound particle velocity (item 8-4), `p` is sound pressure (item 8-2.2), and `c` is the phase speed (ISO 80000-3) of sound
         * remarks: In formula form: `E = int_(t_1)^(t_2) p^2 dt`, where `t_1` and `t_2` are the starting and ending times for the integral and `p` is sound pressure (item 8-2.2). In airborne acoustics, the sound pressure is frequency-weighted and frequency-band-limited. If frequency weightings as specified in IEC 61672-1 are applied, this should be indicated by appropriate subscripts to the symbol `E`. In underwater acoustics, the term ""sound exposure"" indicates an unweighted quantity unless indicated otherwise.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SoundEnergyDensityUnit[1];
    }

    attribute soundEnergyDensity: SoundEnergyDensityValue[*] nonunique :> scalarQuantities;

    attribute def SoundEnergyDensityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-8 item 8-8 sound energy */
    attribute soundEnergy: EnergyValue :> scalarQuantities {
        doc
        /*
         * source: item 8-8 sound energy
         * symbol(s): `Q`
         * application domain: generic
         * name: SoundEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: integral of sound energy density (item 8-7) over a specified volume
         * remarks: The sound energy in region `R` can be expressed by: `Q = oint_R w(x) d^3x`, where `d^3x` is an element of volume.
         */
    }

    /* ISO-80000-8 item 8-9 sound power */
    attribute soundPower: PowerValue :> scalarQuantities {
        doc
        /*
         * source: item 8-9 sound power
         * symbol(s): `P`, `W`
         * application domain: generic
         * name: SoundPower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, kg*m^2*s^-3
         * tensor order: 0
         * definition: integral over a surface of the product of sound pressure, `p` (item 8-2.2), and the component `u_n` of the particle velocity (item 8-4) in the direction normal to the surface, at a point on the surface
         * remarks: This definition holds for waves in the volume of homogenous fluids or gases. This definition can become inapplicable in situations with a high mean fluid flow. Sound power is for example used to indicate the rate at which energy is radiated by a sound source. Sound power is an oscillatory quantity that can be positive or negative. A positive sound power indicates that the sound power is radiated out of the surface. A negative sound power indicates that the sound power is absorbed into the surface.
         */
    }

    /* ISO-80000-8 item 8-10 sound intensity */
    attribute def SoundIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-10 sound intensity (magnitude)
         * symbol(s): `I`
         * application domain: generic
         * name: SoundIntensity
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity given by: `vec(I) = p vec(u)`, where `p` is sound pressure (item 8-2.2) and `vec(u)` is sound particle velocity (item 8-4)
         * remarks: This definition can become inapplicable in situations with a high mean fluid flow.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SoundIntensityUnit[1];
    }

    attribute soundIntensity: SoundIntensityValue[*] nonunique :> scalarQuantities;

    attribute def SoundIntensityUnit :> DerivedUnit {
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (massPF, durationPF); }
    }

    attribute def CartesianSoundIntensity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 8-10 sound intensity (vector)
         * symbol(s): `vec(I)`
         * application domain: generic
         * name: SoundIntensity
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity given by: `vec(I) = p vec(u)`, where `p` is sound pressure (item 8-2.2) and `vec(u)` is sound particle velocity (item 8-4)
         * remarks: This definition can become inapplicable in situations with a high mean fluid flow.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianSoundIntensity3dCoordinateFrame[1];
    }

    attribute cartesianSoundIntensity3dVector: CartesianSoundIntensity3dVector :> vectorQuantities;

    attribute def CartesianSoundIntensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: SoundIntensityUnit[3];
    }

    /* ISO-80000-8 item 8-11 sound exposure */
    attribute def SoundExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-11 sound exposure
         * symbol(s): `E`
         * application domain: generic
         * name: SoundExposure
         * quantity dimension: L^-2*M^2*T^-3
         * measurement unit(s): Pa^2*s, kg^2*m^-2*s^-3
         * tensor order: 0
         * definition: time-integrated squared sound pressure (item 8-2.2)
         * remarks: In formula form: `E = int_(t_1)^(t_2) p^2 dt`, where `t_1` and `t_2` are the starting and ending times for the integral and `p` is sound pressure (item 8-2.2). In airborne acoustics, the sound pressure is frequency-weighted and frequency-band-limited. If frequency weightings as specified in IEC 61672-1 are applied, this should be indicated by appropriate subscripts to the symbol `E`. In underwater acoustics, the term "sound exposure" indicates an unweighted quantity unless indicated otherwise.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SoundExposureUnit[1];
    }

    attribute soundExposure: SoundExposureValue[*] nonunique :> scalarQuantities;

    attribute def SoundExposureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 2; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-8 item 8-12 characteristic impedance of a medium for longitudinal waves */
    attribute def CharacteristicImpedanceOfAMediumForLongitudinalWavesValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-12 characteristic impedance of a medium for longitudinal waves
         * symbol(s): `Z_c`
         * application domain: generic
         * name: CharacteristicImpedanceOfAMediumForLongitudinalWaves
         * quantity dimension: L^-2*M^1*T^-1
         * measurement unit(s): Pa*s/m, kg*m^-2*s^-1
         * tensor order: 0
         * definition: quotient of sound pressure (item 8-2.2) and the component of the sound particle velocity (item 8-4) in the direction of the wave propagation
         * remarks: The definition is limited to a progressive plane wave in a non-dissipative homogenous gas or fluid. Characteristic impedance is a property of the medium and is equal to `ρ c` where `ρ` is the time-averaged density (ISO 80000-4) of the medium and `c` the phase speed of sound (ISO 80000-3). Longitudinal waves are waves in which the displacement of the medium is in the same direction as, or the opposite direction to, the direction of propagation of the wave.
         */
        attribute :>> num: Real;
        attribute :>> mRef: CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit[1];
    }

    attribute characteristicImpedanceOfAMediumForLongitudinalWaves: CharacteristicImpedanceOfAMediumForLongitudinalWavesValue[*] nonunique :> scalarQuantities;

    attribute def CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -2; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-8 item 8-13 acoustic impedance */
    attribute def AcousticImpedanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-13 acoustic impedance
         * symbol(s): `Z_a`
         * application domain: generic
         * name: AcousticImpedance
         * quantity dimension: L^-4*M^1*T^-1
         * measurement unit(s): Pa*s/m^3, kg*m^-4*s^-1
         * tensor order: 0
         * definition: at a surface, quotient of the average sound pressure (item 8-2.2) over that surface and the sound volume flow rate (item 8-6) through that surface
         * remarks: This definition applies to a sound pressure that is in phase with the volume flow rate. In this situation, the acoustic impedance is real. Both the sound pressure, `p`, and sound volume flow rate, `q`, are real quantities that fluctuate with time. If the fluctuations are in phase (phase difference equal to zero), the quotient `p/q` is a constant. If they are out of phase (phase difference not equal to zero), they can be represented by complex quantities in the frequency domain, the quotient of which is also complex.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AcousticImpedanceUnit[1];
    }

    attribute acousticImpedance: AcousticImpedanceValue[*] nonunique :> scalarQuantities;

    attribute def AcousticImpedanceUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -4; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }
    }

    /* ISO-80000-8 item 8-14 sound pressure level */
    attribute def SoundPressureLevelValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-14 sound pressure level
         * symbol(s): `L_p`
         * application domain: generic
         * name: SoundPressureLevel
         * quantity dimension: 1
         * measurement unit(s): dB
         * tensor order: 0
         * definition: quantity given by: `L_p = 10 log_10((p_"RMS"^2)/p_0^2) "[dB]"`, where `p_"RMS"` is the root-mean-square sound pressure in the time domain and `p_0` is the reference value of sound pressure
         * remarks: For sound in air and other gases, the reference value of sound pressure is given by `p_0 = 20 "[μPa]"`. For sound in water and other liquids, the reference value of sound pressure is given by `p_0 = 1 "[μPa]"`. When stating a value of sound pressure level, the reference value shall be specified. The value of sound pressure level depends on the selected frequency range and time duration. When stating a value of sound pressure level, the frequency range and time duration shall be specified. In accordance with ISO 80000-1, any attachment to the unit symbol as a means of giving information about the special nature of the quantity or context of measurement under consideration is not permitted. If specific frequency and time weightings as specified in IEC 61672-1 or specific frequency bands or time duration are applied, this should be indicated by appropriate subscripts to the quantity symbol. In some applications the level of the peak sound pressure is required. This is obtained by replacing the root-mean-square sound pressure, with the instantaneous sound pressure having the greatest absolute value during a stated time interval, in the definition of sound pressure level.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SoundPressureLevelUnit[1];
    }

    attribute soundPressureLevel: SoundPressureLevelValue[*] nonunique :> scalarQuantities;

    attribute def SoundPressureLevelUnit :> DimensionOneUnit {
    }

    /* ISO-80000-8 item 8-15 sound power level */
    attribute def SoundPowerLevelValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-15 sound power level
         * symbol(s): `L_P`, `L_W`
         * application domain: generic
         * name: SoundPowerLevel
         * quantity dimension: 1
         * measurement unit(s): dB
         * tensor order: 0
         * definition: quantity given by: `L_P = 10 log_10 ((P_m)/P_0) "[dB]"`, where `P_m` is the magnitude of the time-averaged sound power (item 8-9) and `P_0` is the reference value of sound power
         * remarks: The reference value of sound power is given by `P_0 = 1 "[pW]"`. When stating a value of sound power level, the reference value shall be specified. The value of sound power level depends on the selected frequency range and time duration. When stating a value of sound power level, the frequency range and time duration shall be specified. In accordance with ISO 80000-1, any attachment to the unit symbol as a means of giving information about the special nature of the quantity or context of measurement under consideration is not permitted. If specific frequency and time weightings as specified in IEC 61672-1 or specific frequency bands or time duration are applied, this should be indicated by appropriate subscripts to the quantity symbol.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SoundPowerLevelUnit[1];
    }

    attribute soundPowerLevel: SoundPowerLevelValue[*] nonunique :> scalarQuantities;

    attribute def SoundPowerLevelUnit :> DimensionOneUnit {
    }

    /* ISO-80000-8 item 8-16 sound exposure level */
    attribute def SoundExposureLevelValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-16 sound exposure level
         * symbol(s): `L_E`
         * application domain: generic
         * name: SoundExposureLevel
         * quantity dimension: 1
         * measurement unit(s): dB
         * tensor order: 0
         * definition: quantity given by: `L_E = 10 log_10(E/E_0) "[dB]"`, where `E` is sound exposure (item 8-11) and `E_0` is the reference value of sound exposure
         * remarks: For sound in air and other gases, the reference value of sound exposure is given by `E_0 = 400 "@"["μPa"^2*"s"]`. For sound in water and other liquids, the reference value of sound exposure is given by `E_0 = 1"@"["μPa"^2*"s"]`. When stating a value of sound exposure level, the reference value shall be specified. The value of sound exposure level depends on the selected frequency range and time duration. When stating a value of sound exposure level, the frequency range and time duration shall be specified. In accordance with ISO 80000-1, any attachment to the unit symbol as a means of giving information about the special nature of the quantity or context of measurement under consideration is not permitted. If specific frequency and time weightings as specified in IEC 61672-1 or specific frequency bands or time duration are applied, this should be indicated by appropriate subscripts to the quantity symbol.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SoundExposureLevelUnit[1];
    }

    attribute soundExposureLevel: SoundExposureLevelValue[*] nonunique :> scalarQuantities;

    attribute def SoundExposureLevelUnit :> DimensionOneUnit {
    }

    /* ISO-80000-8 item 8-17 reverberation time */
    attribute reverberationTime: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 8-17 reverberation time
         * symbol(s): `T`
         * application domain: generic
         * name: ReverberationTime (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: time duration (ISO 80000-3) required for the space-averaged sound energy density (item 8-7) to decrease to `10^(−6)` of its initial value (i.e. for its level to decrease by `60 "[dB]"`) after the source emission has stopped
         * remarks: The reverberation time can be evaluated based on a dynamic range smaller than `60 "[dB]"` and extrapolated to a decay time of `60 "[dB]"`. It is then labelled accordingly `T_n`, where `n` is the dynamic range in `"[dB]"`. See also ISO 3382-1.
         */
    }

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_acoustics.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package ISQAcoustics {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-8:2020 "Acoustics"
     * see also https://www.iso.org/standard/64978.html
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
    private import ISQMechanics::PowerValue;
    private import ISQMechanics::PressureValue;
    private import ISQSpaceTime::CartesianSpatial3dCoordinateFrame;
    private import ISQSpaceTime::SpeedValue;
    private import ISQSpaceTime::CartesianVelocity3dCoordinateFrame;
    private import ISQSpaceTime::AccelerationValue;
    private import ISQSpaceTime::CartesianAcceleration3dCoordinateFrame;
    private import ISQThermodynamics::EnergyValue;
    attribute def LogarithmicFrequencyRangeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-1 logarithmic frequency range
         * symbol(s): `G`
         * application domain: generic
         * name: LogarithmicFrequencyRange
         * quantity dimension: 1
         * measurement unit(s): oct, dec
         * tensor order: 0
         * definition: quantity given by: `G = log_2(f_2/f_1) "[oct]" = log_10(f_2/f_1) "[dec]"`, where `f_1` and `f_2` are two frequencies (ISO 80000-3)
         * remarks: One octave (oct) is the logarithmic frequency range between `f_1` and `f_2` when `f_2/f_1 = 2`. Similarly, one decade (dec) is the logarithmic frequency range between `f_1` and `f_2` when `f_2/f_1 = 10`; thus `1 "[dec]" = log_2(10) "[oct]" ≈ 3.322 "[oct]"`. ISO 266 specifies preferred frequencies for acoustics separated by logarithmic frequency ranges equal to one tenth of a decade (`0.1 "[dec]"`). Each `0.1 "[dec]"` logarithmic frequency range is referred to in ISO 266 as a "one-third-octave interval" because `0.1 "[dec]"` is approximately equal to `1/3 "[oct]"`. Similarly, a logarithmic frequency range of `0.3 "[dec]"` is referred to as a "one-octave interval" because `0.3 "[dec]"` is approximately equal to `1 "[oct]"`. A logarithmic frequency range equal to one tenth of a decade can be referred to as a decidecade.
         */
        attribute :>> num : Real;
        attribute :>> mRef : LogarithmicFrequencyRangeUnit[1];
    }
    attribute def logarithmicFrequencyRange : LogarithmicFrequencyRangeValue[*] nonunique;
    attribute def LogarithmicFrequencyRangeUnit :> DimensionOneUnit {
    }
    attribute def staticPressure : PressureValue {
        doc
        /*
         * source: item 8-2.1 static pressure
         * symbol(s): `p_s`
         * application domain: generic
         * name: StaticPressure (specializes Pressure)
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: pressure (ISO 80000-4) in a medium when no sound wave is present
         * remarks: This definition applies to a medium with zero flow.
         */
    }
    attribute def soundPressure : PressureValue {
        doc
        /*
         * source: item 8-2.2 sound pressure
         * symbol(s): `p`
         * application domain: generic
         * name: SoundPressure (specializes Pressure)
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): Pa, kg*m^-1*s^-2
         * tensor order: 0
         * definition: difference between instantaneous total pressure and static pressure (item 8-2.1)
         * remarks: None.
         */
    }
    attribute def CartesianSoundParticleDisplacement3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 8-3 sound particle displacement
         * symbol(s): `vec(δ)`
         * application domain: generic
         * name: SoundParticleDisplacement (specializes Displacement)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the instantaneous displacement (ISO 80000-3) of a particle in a medium from what would be its position in the absence of sound waves
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame[1];
    }
    attribute def cartesianSoundParticleDisplacement3dVector : CartesianSoundParticleDisplacement3dVector;
    attribute def CartesianSoundParticleVelocity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 8-4 sound particle velocity
         * symbol(s): `vec(u)`, `(vec(v))`
         * application domain: generic
         * name: SoundParticleVelocity (specializes Velocity)
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity given by: `vec(u) = del(vec(δ))/del(t)`, where `vec(δ)` is sound particle displacement (item 8-3) and `t` is time (ISO 80000-3)
         * remarks: The definition is limited to small-amplitude acoustic disturbances such that the magnitude of `vec(u)` is small relative to the phase speed (ISO 80000-3) of sound.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianVelocity3dCoordinateFrame[1];
    }
    attribute def cartesianSoundParticleVelocity3dVector : CartesianSoundParticleVelocity3dVector;
    attribute def CartesianSoundParticleAcceleration3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 8-5 sound particle acceleration
         * symbol(s): `vec(a)`
         * application domain: generic
         * name: SoundParticleAcceleration (specializes Acceleration)
         * quantity dimension: L^1*T^-2
         * measurement unit(s): m*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity given by: `vec(a) = (del(vec(u)))/(del(t))`, where `vec(u)` is sound particle velocity (item 8-4) and `t` is time
         * remarks: The definition is limited to small-amplitude acoustic disturbances such that the magnitude of `vec(u)` is small relative to the phase speed (ISO 80000-3) of sound.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianAcceleration3dCoordinateFrame[1];
    }
    attribute def cartesianSoundParticleAcceleration3dVector : CartesianSoundParticleAcceleration3dVector;
    attribute def volumeVelocity : SpeedValue {
        doc
        /*
         * source: item 8-6 volume velocity, volume flow rate
         * symbol(s): `q`, `q_v`
         * application domain: generic
         * name: VolumeVelocity (specializes Speed)
         * quantity dimension: L^3*T^-1
         * measurement unit(s): m^3*s^-1
         * tensor order: 0
         * definition: surface integral of the normal component of the sound particle velocity (item 8-4) over a defined surface
         * remarks: None.
         */
    }
    alias volumeFlowRate for volumeVelocity;
    attribute def SoundEnergyDensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-7 sound energy density
         * symbol(s): `w`
         * application domain: generic
         * name: SoundEnergyDensity
         * quantity dimension: L^-1*M^1*T^-2
         * measurement unit(s): J/m^3, kg*m^-1*s^-2
         * tensor order: 0
         * definition: quantity given by: `w = 1/2 ρ_m u^2 + 1/2 p^2/(ρ_m c^2)`, where `ρ_m` is mean density (ISO 80000-4), `u` is the magnitude of the sound particle velocity (item 8-4), `p` is sound pressure (item 8-2.2), and `c` is the phase speed (ISO 80000-3) of sound
         * remarks: In formula form: `E = int_(t_1)^(t_2) p^2 dt`, where `t_1` and `t_2` are the starting and ending times for the integral and `p` is sound pressure (item 8-2.2). In airborne acoustics, the sound pressure is frequency-weighted and frequency-band-limited. If frequency weightings as specified in IEC 61672-1 are applied, this should be indicated by appropriate subscripts to the symbol `E`. In underwater acoustics, the term ""sound exposure"" indicates an unweighted quantity unless indicated otherwise.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SoundEnergyDensityUnit[1];
    }
    attribute def soundEnergyDensity : SoundEnergyDensityValue[*] nonunique;
    attribute def SoundEnergyDensityUnit :> DerivedUnit {
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
    attribute def soundEnergy : EnergyValue {
        doc
        /*
         * source: item 8-8 sound energy
         * symbol(s): `Q`
         * application domain: generic
         * name: SoundEnergy (specializes Energy)
         * quantity dimension: L^2*M^1*T^-2
         * measurement unit(s): J, kg*m^2*s^-2
         * tensor order: 0
         * definition: integral of sound energy density (item 8-7) over a specified volume
         * remarks: The sound energy in region `R` can be expressed by: `Q = oint_R w(x) d^3x`, where `d^3x` is an element of volume.
         */
    }
    attribute def soundPower : PowerValue {
        doc
        /*
         * source: item 8-9 sound power
         * symbol(s): `P`, `W`
         * application domain: generic
         * name: SoundPower (specializes Power)
         * quantity dimension: L^2*M^1*T^-3
         * measurement unit(s): W, kg*m^2*s^-3
         * tensor order: 0
         * definition: integral over a surface of the product of sound pressure, `p` (item 8-2.2), and the component `u_n` of the particle velocity (item 8-4) in the direction normal to the surface, at a point on the surface
         * remarks: This definition holds for waves in the volume of homogenous fluids or gases. This definition can become inapplicable in situations with a high mean fluid flow. Sound power is for example used to indicate the rate at which energy is radiated by a sound source. Sound power is an oscillatory quantity that can be positive or negative. A positive sound power indicates that the sound power is radiated out of the surface. A negative sound power indicates that the sound power is absorbed into the surface.
         */
    }
    attribute def SoundIntensityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-10 sound intensity (magnitude)
         * symbol(s): `I`
         * application domain: generic
         * name: SoundIntensity
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity given by: `vec(I) = p vec(u)`, where `p` is sound pressure (item 8-2.2) and `vec(u)` is sound particle velocity (item 8-4)
         * remarks: This definition can become inapplicable in situations with a high mean fluid flow.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SoundIntensityUnit[1];
    }
    attribute def soundIntensity : SoundIntensityValue[*] nonunique;
    attribute def SoundIntensityUnit :> DerivedUnit {
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
    attribute def CartesianSoundIntensity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 8-10 sound intensity (vector)
         * symbol(s): `vec(I)`
         * application domain: generic
         * name: SoundIntensity
         * quantity dimension: M^1*T^-3
         * measurement unit(s): W/m^2, kg*s^-3
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity given by: `vec(I) = p vec(u)`, where `p` is sound pressure (item 8-2.2) and `vec(u)` is sound particle velocity (item 8-4)
         * remarks: This definition can become inapplicable in situations with a high mean fluid flow.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianSoundIntensity3dCoordinateFrame[1];
    }
    attribute def cartesianSoundIntensity3dVector : CartesianSoundIntensity3dVector;
    attribute def CartesianSoundIntensity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : SoundIntensityUnit[3];
    }
    attribute def SoundExposureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-11 sound exposure
         * symbol(s): `E`
         * application domain: generic
         * name: SoundExposure
         * quantity dimension: L^-2*M^2*T^-3
         * measurement unit(s): Pa^2*s, kg^2*m^-2*s^-3
         * tensor order: 0
         * definition: time-integrated squared sound pressure (item 8-2.2)
         * remarks: In formula form: `E = int_(t_1)^(t_2) p^2 dt`, where `t_1` and `t_2` are the starting and ending times for the integral and `p` is sound pressure (item 8-2.2). In airborne acoustics, the sound pressure is frequency-weighted and frequency-band-limited. If frequency weightings as specified in IEC 61672-1 are applied, this should be indicated by appropriate subscripts to the symbol `E`. In underwater acoustics, the term "sound exposure" indicates an unweighted quantity unless indicated otherwise.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SoundExposureUnit[1];
    }
    attribute def soundExposure : SoundExposureValue[*] nonunique;
    attribute def SoundExposureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 2;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def CharacteristicImpedanceOfAMediumForLongitudinalWavesValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-12 characteristic impedance of a medium for longitudinal waves
         * symbol(s): `Z_c`
         * application domain: generic
         * name: CharacteristicImpedanceOfAMediumForLongitudinalWaves
         * quantity dimension: L^-2*M^1*T^-1
         * measurement unit(s): Pa*s/m, kg*m^-2*s^-1
         * tensor order: 0
         * definition: quotient of sound pressure (item 8-2.2) and the component of the sound particle velocity (item 8-4) in the direction of the wave propagation
         * remarks: The definition is limited to a progressive plane wave in a non-dissipative homogenous gas or fluid. Characteristic impedance is a property of the medium and is equal to `ρ c` where `ρ` is the time-averaged density (ISO 80000-4) of the medium and `c` the phase speed of sound (ISO 80000-3). Longitudinal waves are waves in which the displacement of the medium is in the same direction as, or the opposite direction to, the direction of propagation of the wave.
         */
        attribute :>> num : Real;
        attribute :>> mRef : CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit[1];
    }
    attribute def characteristicImpedanceOfAMediumForLongitudinalWaves : CharacteristicImpedanceOfAMediumForLongitudinalWavesValue[*] nonunique;
    attribute def CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -2;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def AcousticImpedanceValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-13 acoustic impedance
         * symbol(s): `Z_a`
         * application domain: generic
         * name: AcousticImpedance
         * quantity dimension: L^-4*M^1*T^-1
         * measurement unit(s): Pa*s/m^3, kg*m^-4*s^-1
         * tensor order: 0
         * definition: at a surface, quotient of the average sound pressure (item 8-2.2) over that surface and the sound volume flow rate (item 8-6) through that surface
         * remarks: This definition applies to a sound pressure that is in phase with the volume flow rate. In this situation, the acoustic impedance is real. Both the sound pressure, `p`, and sound volume flow rate, `q`, are real quantities that fluctuate with time. If the fluctuations are in phase (phase difference equal to zero), the quotient `p/q` is a constant. If they are out of phase (phase difference not equal to zero), they can be represented by complex quantities in the frequency domain, the quotient of which is also complex.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AcousticImpedanceUnit[1];
    }
    attribute def acousticImpedance : AcousticImpedanceValue[*] nonunique;
    attribute def AcousticImpedanceUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -4;
        }
        private attribute massPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.M;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }
    attribute def SoundPressureLevelValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-14 sound pressure level
         * symbol(s): `L_p`
         * application domain: generic
         * name: SoundPressureLevel
         * quantity dimension: 1
         * measurement unit(s): dB
         * tensor order: 0
         * definition: quantity given by: `L_p = 10 log_10((p_"RMS"^2)/p_0^2) "[dB]"`, where `p_"RMS"` is the root-mean-square sound pressure in the time domain and `p_0` is the reference value of sound pressure
         * remarks: For sound in air and other gases, the reference value of sound pressure is given by `p_0 = 20 "[μPa]"`. For sound in water and other liquids, the reference value of sound pressure is given by `p_0 = 1 "[μPa]"`. When stating a value of sound pressure level, the reference value shall be specified. The value of sound pressure level depends on the selected frequency range and time duration. When stating a value of sound pressure level, the frequency range and time duration shall be specified. In accordance with ISO 80000-1, any attachment to the unit symbol as a means of giving information about the special nature of the quantity or context of measurement under consideration is not permitted. If specific frequency and time weightings as specified in IEC 61672-1 or specific frequency bands or time duration are applied, this should be indicated by appropriate subscripts to the quantity symbol. In some applications the level of the peak sound pressure is required. This is obtained by replacing the root-mean-square sound pressure, with the instantaneous sound pressure having the greatest absolute value during a stated time interval, in the definition of sound pressure level.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SoundPressureLevelUnit[1];
    }
    attribute def soundPressureLevel : SoundPressureLevelValue[*] nonunique;
    attribute def SoundPressureLevelUnit :> DimensionOneUnit {
    }
    attribute def SoundPowerLevelValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-15 sound power level
         * symbol(s): `L_P`, `L_W`
         * application domain: generic
         * name: SoundPowerLevel
         * quantity dimension: 1
         * measurement unit(s): dB
         * tensor order: 0
         * definition: quantity given by: `L_P = 10 log_10 ((P_m)/P_0) "[dB]"`, where `P_m` is the magnitude of the time-averaged sound power (item 8-9) and `P_0` is the reference value of sound power
         * remarks: The reference value of sound power is given by `P_0 = 1 "[pW]"`. When stating a value of sound power level, the reference value shall be specified. The value of sound power level depends on the selected frequency range and time duration. When stating a value of sound power level, the frequency range and time duration shall be specified. In accordance with ISO 80000-1, any attachment to the unit symbol as a means of giving information about the special nature of the quantity or context of measurement under consideration is not permitted. If specific frequency and time weightings as specified in IEC 61672-1 or specific frequency bands or time duration are applied, this should be indicated by appropriate subscripts to the quantity symbol.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SoundPowerLevelUnit[1];
    }
    attribute def soundPowerLevel : SoundPowerLevelValue[*] nonunique;
    attribute def SoundPowerLevelUnit :> DimensionOneUnit {
    }
    attribute def SoundExposureLevelValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 8-16 sound exposure level
         * symbol(s): `L_E`
         * application domain: generic
         * name: SoundExposureLevel
         * quantity dimension: 1
         * measurement unit(s): dB
         * tensor order: 0
         * definition: quantity given by: `L_E = 10 log_10(E/E_0) "[dB]"`, where `E` is sound exposure (item 8-11) and `E_0` is the reference value of sound exposure
         * remarks: For sound in air and other gases, the reference value of sound exposure is given by `E_0 = 400 "@"["μPa"^2*"s"]`. For sound in water and other liquids, the reference value of sound exposure is given by `E_0 = 1"@"["μPa"^2*"s"]`. When stating a value of sound exposure level, the reference value shall be specified. The value of sound exposure level depends on the selected frequency range and time duration. When stating a value of sound exposure level, the frequency range and time duration shall be specified. In accordance with ISO 80000-1, any attachment to the unit symbol as a means of giving information about the special nature of the quantity or context of measurement under consideration is not permitted. If specific frequency and time weightings as specified in IEC 61672-1 or specific frequency bands or time duration are applied, this should be indicated by appropriate subscripts to the quantity symbol.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SoundExposureLevelUnit[1];
    }
    attribute def soundExposureLevel : SoundExposureLevelValue[*] nonunique;
    attribute def SoundExposureLevelUnit :> DimensionOneUnit {
    }
    attribute def reverberationTime : DurationValue {
        doc
        /*
         * source: item 8-17 reverberation time
         * symbol(s): `T`
         * application domain: generic
         * name: ReverberationTime (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: time duration (ISO 80000-3) required for the space-averaged sound energy density (item 8-7) to decrease to `10^(−6)` of its initial value (i.e. for its level to decrease by `60 "[dB]"`) after the source emission has stopped
         * remarks: The reverberation time can be evaluated based on a dynamic range smaller than `60 "[dB]"` and extrapolated to a decay time of `60 "[dB]"`. It is then labelled accordingly `T_n`, where `n` is the dynamic range in `"[dB]"`. See also ISO 3382-1.
         */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 779) (line 15) (column 20) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 779) (line 15) (column 20) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 793) (line 15) (column 34) (len 4)))))
    (reference r1 (scope relative) (span (offset 818) (line 16) (column 20) (len 10)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 818) (line 16) (column 20) (len 10)))))
    (reference r2 (scope relative) (span (offset 852) (line 17) (column 20) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 852) (line 17) (column 20) (len 21)))))
    (reference r3 (scope relative) (span (offset 897) (line 18) (column 20) (len 7)) (segments (segment 0 (token "ISQBase") (name "ISQBase") (separator none) (span (offset 897) (line 18) (column 20) (len 7)))))
    (reference r4 (scope relative) (span (offset 995) (line 21) (column 20) (len 24)) (segments (segment 0 (token "ISQMechanics") (name "ISQMechanics") (separator none) (span (offset 995) (line 21) (column 20) (len 12))) (segment 1 (token "PowerValue") (name "PowerValue") (separator colon-colon) (span (offset 1009) (line 21) (column 34) (len 10)))))
    (reference r5 (scope relative) (span (offset 1040) (line 22) (column 20) (len 27)) (segments (segment 0 (token "ISQMechanics") (name "ISQMechanics") (separator none) (span (offset 1040) (line 22) (column 20) (len 12))) (segment 1 (token "PressureValue") (name "PressureValue") (separator colon-colon) (span (offset 1054) (line 22) (column 34) (len 13)))))
    (reference r6 (scope relative) (span (offset 1088) (line 23) (column 20) (len 47)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 1088) (line 23) (column 20) (len 12))) (segment 1 (token "CartesianSpatial3dCoordinateFrame") (name "CartesianSpatial3dCoordinateFrame") (separator colon-colon) (span (offset 1102) (line 23) (column 34) (len 33)))))
    (reference r7 (scope relative) (span (offset 1156) (line 24) (column 20) (len 24)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 1156) (line 24) (column 20) (len 12))) (segment 1 (token "SpeedValue") (name "SpeedValue") (separator colon-colon) (span (offset 1170) (line 24) (column 34) (len 10)))))
    (reference r8 (scope relative) (span (offset 1201) (line 25) (column 20) (len 48)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 1201) (line 25) (column 20) (len 12))) (segment 1 (token "CartesianVelocity3dCoordinateFrame") (name "CartesianVelocity3dCoordinateFrame") (separator colon-colon) (span (offset 1215) (line 25) (column 34) (len 34)))))
    (reference r9 (scope relative) (span (offset 1270) (line 26) (column 20) (len 31)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 1270) (line 26) (column 20) (len 12))) (segment 1 (token "AccelerationValue") (name "AccelerationValue") (separator colon-colon) (span (offset 1284) (line 26) (column 34) (len 17)))))
    (reference r10 (scope relative) (span (offset 1322) (line 27) (column 20) (len 52)) (segments (segment 0 (token "ISQSpaceTime") (name "ISQSpaceTime") (separator none) (span (offset 1322) (line 27) (column 20) (len 12))) (segment 1 (token "CartesianAcceleration3dCoordinateFrame") (name "CartesianAcceleration3dCoordinateFrame") (separator colon-colon) (span (offset 1336) (line 27) (column 34) (len 38)))))
    (reference r11 (scope relative) (span (offset 1395) (line 28) (column 20) (len 30)) (segments (segment 0 (token "ISQThermodynamics") (name "ISQThermodynamics") (separator none) (span (offset 1395) (line 28) (column 20) (len 17))) (segment 1 (token "EnergyValue") (name "EnergyValue") (separator colon-colon) (span (offset 1414) (line 28) (column 39) (len 11)))))
    (reference r12 (scope relative) (span (offset 1539) (line 31) (column 53) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 1539) (line 31) (column 53) (len 19)))))
    (reference r13 (scope relative) (span (offset 2893) (line 44) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 2893) (line 44) (column 28) (len 4)))))
    (reference r14 (scope relative) (span (offset 2888) (line 44) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 2888) (line 44) (column 23) (len 3)))))
    (reference r15 (scope relative) (span (offset 2927) (line 45) (column 29) (len 29)) (segments (segment 0 (token "LogarithmicFrequencyRangeUnit") (name "LogarithmicFrequencyRangeUnit") (separator none) (span (offset 2927) (line 45) (column 29) (len 29)))))
    (reference r16 (scope relative) (span (offset 2921) (line 45) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 2921) (line 45) (column 23) (len 4)))))
    (reference r17 (scope relative) (span (offset 3009) (line 48) (column 42) (len 30)) (segments (segment 0 (token "LogarithmicFrequencyRangeValue") (name "LogarithmicFrequencyRangeValue") (separator none) (span (offset 3009) (line 48) (column 42) (len 30)))))
    (reference r18 (scope relative) (span (offset 3126) (line 50) (column 52) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 3126) (line 50) (column 52) (len 16)))))
    (reference r19 (scope relative) (span (offset 3231) (line 54) (column 31) (len 13)) (segments (segment 0 (token "PressureValue") (name "PressureValue") (separator none) (span (offset 3231) (line 54) (column 31) (len 13)))))
    (reference r20 (scope relative) (span (offset 3835) (line 70) (column 30) (len 13)) (segments (segment 0 (token "PressureValue") (name "PressureValue") (separator none) (span (offset 3835) (line 70) (column 30) (len 13)))))
    (reference r21 (scope relative) (span (offset 4451) (line 86) (column 65) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 4451) (line 86) (column 65) (len 23)))))
    (reference r22 (scope relative) (span (offset 5046) (line 99) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 5046) (line 99) (column 23) (len 7)))))
    (reference r23 (scope relative) (span (offset 5091) (line 100) (column 29) (len 33)) (segments (segment 0 (token "CartesianSpatial3dCoordinateFrame") (name "CartesianSpatial3dCoordinateFrame") (separator none) (span (offset 5091) (line 100) (column 29) (len 33)))))
    (reference r24 (scope relative) (span (offset 5085) (line 100) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 5085) (line 100) (column 23) (len 4)))))
    (reference r25 (scope relative) (span (offset 5194) (line 103) (column 59) (len 42)) (segments (segment 0 (token "CartesianSoundParticleDisplacement3dVector") (name "CartesianSoundParticleDisplacement3dVector") (separator none) (span (offset 5194) (line 103) (column 59) (len 42)))))
    (reference r26 (scope relative) (span (offset 5374) (line 106) (column 61) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 5374) (line 106) (column 61) (len 23)))))
    (reference r27 (scope relative) (span (offset 6126) (line 119) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 6126) (line 119) (column 23) (len 7)))))
    (reference r28 (scope relative) (span (offset 6171) (line 120) (column 29) (len 34)) (segments (segment 0 (token "CartesianVelocity3dCoordinateFrame") (name "CartesianVelocity3dCoordinateFrame") (separator none) (span (offset 6171) (line 120) (column 29) (len 34)))))
    (reference r29 (scope relative) (span (offset 6165) (line 120) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 6165) (line 120) (column 23) (len 4)))))
    (reference r30 (scope relative) (span (offset 6271) (line 123) (column 55) (len 38)) (segments (segment 0 (token "CartesianSoundParticleVelocity3dVector") (name "CartesianSoundParticleVelocity3dVector") (separator none) (span (offset 6271) (line 123) (column 55) (len 38)))))
    (reference r31 (scope relative) (span (offset 6455) (line 126) (column 65) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 6455) (line 126) (column 65) (len 23)))))
    (reference r32 (scope relative) (span (offset 7191) (line 139) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 7191) (line 139) (column 23) (len 7)))))
    (reference r33 (scope relative) (span (offset 7236) (line 140) (column 29) (len 38)) (segments (segment 0 (token "CartesianAcceleration3dCoordinateFrame") (name "CartesianAcceleration3dCoordinateFrame") (separator none) (span (offset 7236) (line 140) (column 29) (len 38)))))
    (reference r34 (scope relative) (span (offset 7230) (line 140) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 7230) (line 140) (column 23) (len 4)))))
    (reference r35 (scope relative) (span (offset 7344) (line 143) (column 59) (len 42)) (segments (segment 0 (token "CartesianSoundParticleAcceleration3dVector") (name "CartesianSoundParticleAcceleration3dVector") (separator none) (span (offset 7344) (line 143) (column 59) (len 42)))))
    (reference r36 (scope relative) (span (offset 7504) (line 146) (column 31) (len 10)) (segments (segment 0 (token "SpeedValue") (name "SpeedValue") (separator none) (span (offset 7504) (line 146) (column 31) (len 10)))))
    (reference r37 (scope relative) (span (offset 8057) (line 161) (column 30) (len 14)) (segments (segment 0 (token "volumeVelocity") (name "volumeVelocity") (separator none) (span (offset 8057) (line 161) (column 30) (len 14)))))
    (reference r38 (scope relative) (span (offset 8171) (line 164) (column 46) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 8171) (line 164) (column 46) (len 19)))))
    (reference r39 (scope relative) (span (offset 9327) (line 177) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 9327) (line 177) (column 28) (len 4)))))
    (reference r40 (scope relative) (span (offset 9322) (line 177) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 9322) (line 177) (column 23) (len 3)))))
    (reference r41 (scope relative) (span (offset 9361) (line 178) (column 29) (len 22)) (segments (segment 0 (token "SoundEnergyDensityUnit") (name "SoundEnergyDensityUnit") (separator none) (span (offset 9361) (line 178) (column 29) (len 22)))))
    (reference r42 (scope relative) (span (offset 9355) (line 178) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 9355) (line 178) (column 23) (len 4)))))
    (reference r43 (scope relative) (span (offset 9429) (line 181) (column 35) (len 23)) (segments (segment 0 (token "SoundEnergyDensityValue") (name "SoundEnergyDensityValue") (separator none) (span (offset 9429) (line 181) (column 35) (len 23)))))
    (reference r44 (scope relative) (span (offset 9532) (line 183) (column 45) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 9532) (line 183) (column 45) (len 11)))))
    (reference r45 (scope relative) (span (offset 9582) (line 184) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 9582) (line 184) (column 37) (len 19)))))
    (reference r46 (scope relative) (span (offset 9611) (line 184) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 9611) (line 184) (column 66) (len 8)))))
    (reference r47 (scope relative) (span (offset 9622) (line 184) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 9622) (line 184) (column 77) (len 3)))))
    (reference r48 (scope relative) (span (offset 9626) (line 184) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 9626) (line 184) (column 81) (len 1)))))
    (reference r49 (scope relative) (span (offset 9633) (line 184) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 9633) (line 184) (column 88) (len 8)))))
    (reference r50 (scope relative) (span (offset 9684) (line 185) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 9684) (line 185) (column 35) (len 19)))))
    (reference r51 (scope relative) (span (offset 9713) (line 185) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 9713) (line 185) (column 64) (len 8)))))
    (reference r52 (scope relative) (span (offset 9724) (line 185) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 9724) (line 185) (column 75) (len 3)))))
    (reference r53 (scope relative) (span (offset 9728) (line 185) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 9728) (line 185) (column 79) (len 1)))))
    (reference r54 (scope relative) (span (offset 9735) (line 185) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 9735) (line 185) (column 86) (len 8)))))
    (reference r55 (scope relative) (span (offset 9789) (line 186) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 9789) (line 186) (column 39) (len 19)))))
    (reference r56 (scope relative) (span (offset 9818) (line 186) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 9818) (line 186) (column 68) (len 8)))))
    (reference r57 (scope relative) (span (offset 9829) (line 186) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 9829) (line 186) (column 79) (len 3)))))
    (reference r58 (scope relative) (span (offset 9833) (line 186) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 9833) (line 186) (column 83) (len 1)))))
    (reference r59 (scope relative) (span (offset 9840) (line 186) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 9840) (line 186) (column 90) (len 8)))))
    (reference r60 (scope relative) (span (offset 9879) (line 187) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 9879) (line 187) (column 23) (len 17)))))
    (reference r61 (scope relative) (span (offset 9903) (line 187) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 9903) (line 187) (column 47) (len 20)))))
    (reference r62 (scope relative) (span (offset 9927) (line 187) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 9927) (line 187) (column 71) (len 8)))))
    (reference r63 (scope relative) (span (offset 9937) (line 187) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 9937) (line 187) (column 81) (len 6)))))
    (reference r64 (scope relative) (span (offset 9945) (line 187) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 9945) (line 187) (column 89) (len 10)))))
    (reference r65 (scope relative) (span (offset 10038) (line 191) (column 28) (len 11)) (segments (segment 0 (token "EnergyValue") (name "EnergyValue") (separator none) (span (offset 10038) (line 191) (column 28) (len 11)))))
    (reference r66 (scope relative) (span (offset 10682) (line 207) (column 27) (len 10)) (segments (segment 0 (token "PowerValue") (name "PowerValue") (separator none) (span (offset 10682) (line 207) (column 27) (len 10)))))
    (reference r67 (scope relative) (span (offset 11870) (line 223) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 11870) (line 223) (column 42) (len 19)))))
    (reference r68 (scope relative) (span (offset 12501) (line 236) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 12501) (line 236) (column 28) (len 4)))))
    (reference r69 (scope relative) (span (offset 12496) (line 236) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 12496) (line 236) (column 23) (len 3)))))
    (reference r70 (scope relative) (span (offset 12535) (line 237) (column 29) (len 18)) (segments (segment 0 (token "SoundIntensityUnit") (name "SoundIntensityUnit") (separator none) (span (offset 12535) (line 237) (column 29) (len 18)))))
    (reference r71 (scope relative) (span (offset 12529) (line 237) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 12529) (line 237) (column 23) (len 4)))))
    (reference r72 (scope relative) (span (offset 12595) (line 240) (column 31) (len 19)) (segments (segment 0 (token "SoundIntensityValue") (name "SoundIntensityValue") (separator none) (span (offset 12595) (line 240) (column 31) (len 19)))))
    (reference r73 (scope relative) (span (offset 12690) (line 242) (column 41) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 12690) (line 242) (column 41) (len 11)))))
    (reference r74 (scope relative) (span (offset 12738) (line 243) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12738) (line 243) (column 35) (len 19)))))
    (reference r75 (scope relative) (span (offset 12767) (line 243) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12767) (line 243) (column 64) (len 8)))))
    (reference r76 (scope relative) (span (offset 12778) (line 243) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12778) (line 243) (column 75) (len 3)))))
    (reference r77 (scope relative) (span (offset 12782) (line 243) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 12782) (line 243) (column 79) (len 1)))))
    (reference r78 (scope relative) (span (offset 12789) (line 243) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12789) (line 243) (column 86) (len 8)))))
    (reference r79 (scope relative) (span (offset 12843) (line 244) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 12843) (line 244) (column 39) (len 19)))))
    (reference r80 (scope relative) (span (offset 12872) (line 244) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 12872) (line 244) (column 68) (len 8)))))
    (reference r81 (scope relative) (span (offset 12883) (line 244) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 12883) (line 244) (column 79) (len 3)))))
    (reference r82 (scope relative) (span (offset 12887) (line 244) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 12887) (line 244) (column 83) (len 1)))))
    (reference r83 (scope relative) (span (offset 12894) (line 244) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 12894) (line 244) (column 90) (len 8)))))
    (reference r84 (scope relative) (span (offset 12933) (line 245) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 12933) (line 245) (column 23) (len 17)))))
    (reference r85 (scope relative) (span (offset 12957) (line 245) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 12957) (line 245) (column 47) (len 20)))))
    (reference r86 (scope relative) (span (offset 12981) (line 245) (column 71) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 12981) (line 245) (column 71) (len 6)))))
    (reference r87 (scope relative) (span (offset 12989) (line 245) (column 79) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 12989) (line 245) (column 79) (len 10)))))
    (reference r88 (scope relative) (span (offset 13064) (line 248) (column 54) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 13064) (line 248) (column 54) (len 23)))))
    (reference r89 (scope relative) (span (offset 13696) (line 261) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 13696) (line 261) (column 23) (len 7)))))
    (reference r90 (scope relative) (span (offset 13741) (line 262) (column 29) (len 40)) (segments (segment 0 (token "CartesianSoundIntensity3dCoordinateFrame") (name "CartesianSoundIntensity3dCoordinateFrame") (separator none) (span (offset 13741) (line 262) (column 29) (len 40)))))
    (reference r91 (scope relative) (span (offset 13735) (line 262) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 13735) (line 262) (column 23) (len 4)))))
    (reference r92 (scope relative) (span (offset 13840) (line 265) (column 48) (len 31)) (segments (segment 0 (token "CartesianSoundIntensity3dVector") (name "CartesianSoundIntensity3dVector") (separator none) (span (offset 13840) (line 265) (column 48) (len 31)))))
    (reference r93 (scope relative) (span (offset 13956) (line 267) (column 63) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 13956) (line 267) (column 63) (len 19)))))
    (reference r94 (scope relative) (span (offset 14000) (line 268) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 14000) (line 268) (column 23) (len 7)))))
    (reference r95 (scope relative) (span (offset 14039) (line 269) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 14039) (line 269) (column 23) (len 12)))))
    (reference r96 (scope relative) (span (offset 14089) (line 270) (column 30) (len 18)) (segments (segment 0 (token "SoundIntensityUnit") (name "SoundIntensityUnit") (separator none) (span (offset 14089) (line 270) (column 30) (len 18)))))
    (reference r97 (scope relative) (span (offset 14082) (line 270) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 14082) (line 270) (column 23) (len 5)))))
    (reference r98 (scope relative) (span (offset 14206) (line 274) (column 41) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 14206) (line 274) (column 41) (len 19)))))
    (reference r99 (scope relative) (span (offset 15151) (line 287) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 15151) (line 287) (column 28) (len 4)))))
    (reference r100 (scope relative) (span (offset 15146) (line 287) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 15146) (line 287) (column 23) (len 3)))))
    (reference r101 (scope relative) (span (offset 15185) (line 288) (column 29) (len 17)) (segments (segment 0 (token "SoundExposureUnit") (name "SoundExposureUnit") (separator none) (span (offset 15185) (line 288) (column 29) (len 17)))))
    (reference r102 (scope relative) (span (offset 15179) (line 288) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 15179) (line 288) (column 23) (len 4)))))
    (reference r103 (scope relative) (span (offset 15243) (line 291) (column 30) (len 18)) (segments (segment 0 (token "SoundExposureValue") (name "SoundExposureValue") (separator none) (span (offset 15243) (line 291) (column 30) (len 18)))))
    (reference r104 (scope relative) (span (offset 15336) (line 293) (column 40) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 15336) (line 293) (column 40) (len 11)))))
    (reference r105 (scope relative) (span (offset 15386) (line 294) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 15386) (line 294) (column 37) (len 19)))))
    (reference r106 (scope relative) (span (offset 15415) (line 294) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 15415) (line 294) (column 66) (len 8)))))
    (reference r107 (scope relative) (span (offset 15426) (line 294) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 15426) (line 294) (column 77) (len 3)))))
    (reference r108 (scope relative) (span (offset 15430) (line 294) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 15430) (line 294) (column 81) (len 1)))))
    (reference r109 (scope relative) (span (offset 15437) (line 294) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 15437) (line 294) (column 88) (len 8)))))
    (reference r110 (scope relative) (span (offset 15488) (line 295) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 15488) (line 295) (column 35) (len 19)))))
    (reference r111 (scope relative) (span (offset 15517) (line 295) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 15517) (line 295) (column 64) (len 8)))))
    (reference r112 (scope relative) (span (offset 15528) (line 295) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 15528) (line 295) (column 75) (len 3)))))
    (reference r113 (scope relative) (span (offset 15532) (line 295) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 15532) (line 295) (column 79) (len 1)))))
    (reference r114 (scope relative) (span (offset 15539) (line 295) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 15539) (line 295) (column 86) (len 8)))))
    (reference r115 (scope relative) (span (offset 15593) (line 296) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 15593) (line 296) (column 39) (len 19)))))
    (reference r116 (scope relative) (span (offset 15622) (line 296) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 15622) (line 296) (column 68) (len 8)))))
    (reference r117 (scope relative) (span (offset 15633) (line 296) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 15633) (line 296) (column 79) (len 3)))))
    (reference r118 (scope relative) (span (offset 15637) (line 296) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 15637) (line 296) (column 83) (len 1)))))
    (reference r119 (scope relative) (span (offset 15644) (line 296) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 15644) (line 296) (column 90) (len 8)))))
    (reference r120 (scope relative) (span (offset 15683) (line 297) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 15683) (line 297) (column 23) (len 17)))))
    (reference r121 (scope relative) (span (offset 15707) (line 297) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 15707) (line 297) (column 47) (len 20)))))
    (reference r122 (scope relative) (span (offset 15731) (line 297) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 15731) (line 297) (column 71) (len 8)))))
    (reference r123 (scope relative) (span (offset 15741) (line 297) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 15741) (line 297) (column 81) (len 6)))))
    (reference r124 (scope relative) (span (offset 15749) (line 297) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 15749) (line 297) (column 89) (len 10)))))
    (reference r125 (scope relative) (span (offset 15942) (line 301) (column 80) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 15942) (line 301) (column 80) (len 19)))))
    (reference r126 (scope relative) (span (offset 17023) (line 314) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 17023) (line 314) (column 28) (len 4)))))
    (reference r127 (scope relative) (span (offset 17018) (line 314) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 17018) (line 314) (column 23) (len 3)))))
    (reference r128 (scope relative) (span (offset 17057) (line 315) (column 29) (len 56)) (segments (segment 0 (token "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit") (name "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit") (separator none) (span (offset 17057) (line 315) (column 29) (len 56)))))
    (reference r129 (scope relative) (span (offset 17051) (line 315) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 17051) (line 315) (column 23) (len 4)))))
    (reference r130 (scope relative) (span (offset 17193) (line 318) (column 69) (len 57)) (segments (segment 0 (token "CharacteristicImpedanceOfAMediumForLongitudinalWavesValue") (name "CharacteristicImpedanceOfAMediumForLongitudinalWavesValue") (separator none) (span (offset 17193) (line 318) (column 69) (len 57)))))
    (reference r131 (scope relative) (span (offset 17364) (line 320) (column 79) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 17364) (line 320) (column 79) (len 11)))))
    (reference r132 (scope relative) (span (offset 17414) (line 321) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 17414) (line 321) (column 37) (len 19)))))
    (reference r133 (scope relative) (span (offset 17443) (line 321) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 17443) (line 321) (column 66) (len 8)))))
    (reference r134 (scope relative) (span (offset 17454) (line 321) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 17454) (line 321) (column 77) (len 3)))))
    (reference r135 (scope relative) (span (offset 17458) (line 321) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 17458) (line 321) (column 81) (len 1)))))
    (reference r136 (scope relative) (span (offset 17465) (line 321) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 17465) (line 321) (column 88) (len 8)))))
    (reference r137 (scope relative) (span (offset 17516) (line 322) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 17516) (line 322) (column 35) (len 19)))))
    (reference r138 (scope relative) (span (offset 17545) (line 322) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 17545) (line 322) (column 64) (len 8)))))
    (reference r139 (scope relative) (span (offset 17556) (line 322) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 17556) (line 322) (column 75) (len 3)))))
    (reference r140 (scope relative) (span (offset 17560) (line 322) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 17560) (line 322) (column 79) (len 1)))))
    (reference r141 (scope relative) (span (offset 17567) (line 322) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 17567) (line 322) (column 86) (len 8)))))
    (reference r142 (scope relative) (span (offset 17621) (line 323) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 17621) (line 323) (column 39) (len 19)))))
    (reference r143 (scope relative) (span (offset 17650) (line 323) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 17650) (line 323) (column 68) (len 8)))))
    (reference r144 (scope relative) (span (offset 17661) (line 323) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 17661) (line 323) (column 79) (len 3)))))
    (reference r145 (scope relative) (span (offset 17665) (line 323) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 17665) (line 323) (column 83) (len 1)))))
    (reference r146 (scope relative) (span (offset 17672) (line 323) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 17672) (line 323) (column 90) (len 8)))))
    (reference r147 (scope relative) (span (offset 17711) (line 324) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 17711) (line 324) (column 23) (len 17)))))
    (reference r148 (scope relative) (span (offset 17735) (line 324) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 17735) (line 324) (column 47) (len 20)))))
    (reference r149 (scope relative) (span (offset 17759) (line 324) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 17759) (line 324) (column 71) (len 8)))))
    (reference r150 (scope relative) (span (offset 17769) (line 324) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 17769) (line 324) (column 81) (len 6)))))
    (reference r151 (scope relative) (span (offset 17777) (line 324) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 17777) (line 324) (column 89) (len 10)))))
    (reference r152 (scope relative) (span (offset 17894) (line 328) (column 45) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 17894) (line 328) (column 45) (len 19)))))
    (reference r153 (scope relative) (span (offset 18967) (line 341) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 18967) (line 341) (column 28) (len 4)))))
    (reference r154 (scope relative) (span (offset 18962) (line 341) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 18962) (line 341) (column 23) (len 3)))))
    (reference r155 (scope relative) (span (offset 19001) (line 342) (column 29) (len 21)) (segments (segment 0 (token "AcousticImpedanceUnit") (name "AcousticImpedanceUnit") (separator none) (span (offset 19001) (line 342) (column 29) (len 21)))))
    (reference r156 (scope relative) (span (offset 18995) (line 342) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 18995) (line 342) (column 23) (len 4)))))
    (reference r157 (scope relative) (span (offset 19067) (line 345) (column 34) (len 22)) (segments (segment 0 (token "AcousticImpedanceValue") (name "AcousticImpedanceValue") (separator none) (span (offset 19067) (line 345) (column 34) (len 22)))))
    (reference r158 (scope relative) (span (offset 19168) (line 347) (column 44) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 19168) (line 347) (column 44) (len 11)))))
    (reference r159 (scope relative) (span (offset 19218) (line 348) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 19218) (line 348) (column 37) (len 19)))))
    (reference r160 (scope relative) (span (offset 19247) (line 348) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 19247) (line 348) (column 66) (len 8)))))
    (reference r161 (scope relative) (span (offset 19258) (line 348) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 19258) (line 348) (column 77) (len 3)))))
    (reference r162 (scope relative) (span (offset 19262) (line 348) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 19262) (line 348) (column 81) (len 1)))))
    (reference r163 (scope relative) (span (offset 19269) (line 348) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 19269) (line 348) (column 88) (len 8)))))
    (reference r164 (scope relative) (span (offset 19320) (line 349) (column 35) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 19320) (line 349) (column 35) (len 19)))))
    (reference r165 (scope relative) (span (offset 19349) (line 349) (column 64) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 19349) (line 349) (column 64) (len 8)))))
    (reference r166 (scope relative) (span (offset 19360) (line 349) (column 75) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 19360) (line 349) (column 75) (len 3)))))
    (reference r167 (scope relative) (span (offset 19364) (line 349) (column 79) (len 1)) (segments (segment 0 (token "M") (name "M") (separator none) (span (offset 19364) (line 349) (column 79) (len 1)))))
    (reference r168 (scope relative) (span (offset 19371) (line 349) (column 86) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 19371) (line 349) (column 86) (len 8)))))
    (reference r169 (scope relative) (span (offset 19425) (line 350) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 19425) (line 350) (column 39) (len 19)))))
    (reference r170 (scope relative) (span (offset 19454) (line 350) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 19454) (line 350) (column 68) (len 8)))))
    (reference r171 (scope relative) (span (offset 19465) (line 350) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 19465) (line 350) (column 79) (len 3)))))
    (reference r172 (scope relative) (span (offset 19469) (line 350) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 19469) (line 350) (column 83) (len 1)))))
    (reference r173 (scope relative) (span (offset 19476) (line 350) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 19476) (line 350) (column 90) (len 8)))))
    (reference r174 (scope relative) (span (offset 19515) (line 351) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 19515) (line 351) (column 23) (len 17)))))
    (reference r175 (scope relative) (span (offset 19539) (line 351) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 19539) (line 351) (column 47) (len 20)))))
    (reference r176 (scope relative) (span (offset 19563) (line 351) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 19563) (line 351) (column 71) (len 8)))))
    (reference r177 (scope relative) (span (offset 19573) (line 351) (column 81) (len 6)) (segments (segment 0 (token "massPF") (name "massPF") (separator none) (span (offset 19573) (line 351) (column 81) (len 6)))))
    (reference r178 (scope relative) (span (offset 19581) (line 351) (column 89) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 19581) (line 351) (column 89) (len 10)))))
    (reference r179 (scope relative) (span (offset 19701) (line 355) (column 46) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 19701) (line 355) (column 46) (len 19)))))
    (reference r180 (scope relative) (span (offset 21453) (line 368) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 21453) (line 368) (column 28) (len 4)))))
    (reference r181 (scope relative) (span (offset 21448) (line 368) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 21448) (line 368) (column 23) (len 3)))))
    (reference r182 (scope relative) (span (offset 21487) (line 369) (column 29) (len 22)) (segments (segment 0 (token "SoundPressureLevelUnit") (name "SoundPressureLevelUnit") (separator none) (span (offset 21487) (line 369) (column 29) (len 22)))))
    (reference r183 (scope relative) (span (offset 21481) (line 369) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 21481) (line 369) (column 23) (len 4)))))
    (reference r184 (scope relative) (span (offset 21555) (line 372) (column 35) (len 23)) (segments (segment 0 (token "SoundPressureLevelValue") (name "SoundPressureLevelValue") (separator none) (span (offset 21555) (line 372) (column 35) (len 23)))))
    (reference r185 (scope relative) (span (offset 21658) (line 374) (column 45) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 21658) (line 374) (column 45) (len 16)))))
    (reference r186 (scope relative) (span (offset 21776) (line 378) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 21776) (line 378) (column 43) (len 19)))))
    (reference r187 (scope relative) (span (offset 23076) (line 391) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 23076) (line 391) (column 28) (len 4)))))
    (reference r188 (scope relative) (span (offset 23071) (line 391) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 23071) (line 391) (column 23) (len 3)))))
    (reference r189 (scope relative) (span (offset 23110) (line 392) (column 29) (len 19)) (segments (segment 0 (token "SoundPowerLevelUnit") (name "SoundPowerLevelUnit") (separator none) (span (offset 23110) (line 392) (column 29) (len 19)))))
    (reference r190 (scope relative) (span (offset 23104) (line 392) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 23104) (line 392) (column 23) (len 4)))))
    (reference r191 (scope relative) (span (offset 23172) (line 395) (column 32) (len 20)) (segments (segment 0 (token "SoundPowerLevelValue") (name "SoundPowerLevelValue") (separator none) (span (offset 23172) (line 395) (column 32) (len 20)))))
    (reference r192 (scope relative) (span (offset 23269) (line 397) (column 42) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 23269) (line 397) (column 42) (len 16)))))
    (reference r193 (scope relative) (span (offset 23393) (line 401) (column 46) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 23393) (line 401) (column 46) (len 19)))))
    (reference r194 (scope relative) (span (offset 24832) (line 414) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 24832) (line 414) (column 28) (len 4)))))
    (reference r195 (scope relative) (span (offset 24827) (line 414) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 24827) (line 414) (column 23) (len 3)))))
    (reference r196 (scope relative) (span (offset 24866) (line 415) (column 29) (len 22)) (segments (segment 0 (token "SoundExposureLevelUnit") (name "SoundExposureLevelUnit") (separator none) (span (offset 24866) (line 415) (column 29) (len 22)))))
    (reference r197 (scope relative) (span (offset 24860) (line 415) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 24860) (line 415) (column 23) (len 4)))))
    (reference r198 (scope relative) (span (offset 24934) (line 418) (column 35) (len 23)) (segments (segment 0 (token "SoundExposureLevelValue") (name "SoundExposureLevelValue") (separator none) (span (offset 24934) (line 418) (column 35) (len 23)))))
    (reference r199 (scope relative) (span (offset 25037) (line 420) (column 45) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 25037) (line 420) (column 45) (len 16)))))
    (reference r200 (scope relative) (span (offset 25147) (line 424) (column 34) (len 13)) (segments (segment 0 (token "DurationValue") (name "DurationValue") (separator none) (span (offset 25147) (line 424) (column 34) (len 13)))))
  )
  (root (library-package (name "ISQAcoustics") (standard true) (body brace (doc) (import (target (span (span (offset 779) (line 15) (column 20) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 818) (line 16) (column 20) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 828) (line 16) (column 30) (len 3))) (separator (span (offset 828) (line 16) (column 30) (len 2))) (marker (span (offset 830) (line 16) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 852) (line 17) (column 20) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 873) (line 17) (column 41) (len 3))) (separator (span (offset 873) (line 17) (column 41) (len 2))) (marker (span (offset 875) (line 17) (column 43) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 897) (line 18) (column 20) (len 10))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 904) (line 18) (column 27) (len 3))) (separator (span (offset 904) (line 18) (column 27) (len 2))) (marker (span (offset 906) (line 18) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 995) (line 21) (column 20) (len 24))) (all none) (ref r4) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1040) (line 22) (column 20) (len 27))) (all none) (ref r5) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1088) (line 23) (column 20) (len 47))) (all none) (ref r6) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1156) (line 24) (column 20) (len 24))) (all none) (ref r7) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1201) (line 25) (column 20) (len 48))) (all none) (ref r8) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1270) (line 26) (column 20) (len 31))) (all none) (ref r9) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1322) (line 27) (column 20) (len 52))) (all none) (ref r10) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1395) (line 28) (column 20) (len 30))) (all none) (ref r11) (shape (membership (recursive-suffix none))))) (attribute-def (declaration-name "LogarithmicFrequencyRangeValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r14)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "logarithmicFrequencyRange") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "LogarithmicFrequencyRangeUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace)) (attribute-def (declaration-name "staticPressure") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "soundPressure") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "CartesianSoundParticleDisplacement3dVector") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r21)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r22)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 5056) (line 99) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r24)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianSoundParticleDisplacement3dVector") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianSoundParticleVelocity3dVector") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r26)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r27)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6136) (line 119) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r28)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r29)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianSoundParticleVelocity3dVector") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r30)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianSoundParticleAcceleration3dVector") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r31)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r32)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7201) (line 139) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r33)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r34)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianSoundParticleAcceleration3dVector") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r35)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "volumeVelocity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r36)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (alias (name "volumeFlowRate") (target (ref r37)) (body semicolon)) (attribute-def (declaration-name "SoundEnergyDensityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r38)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r39)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r40)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r41)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r42)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "soundEnergyDensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r43)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SoundEnergyDensityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r44)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r45)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r46)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9622) (line 184) (column 77) (len 5)) (member-access (base (expression (span (offset 9622) (line 184) (column 77) (len 3)) (ref r47))) (separator dot) (member (ref r48))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r49)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9644) (line 184) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 9645) (line 184) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r50)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r51)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9724) (line 185) (column 75) (len 5)) (member-access (base (expression (span (offset 9724) (line 185) (column 75) (len 3)) (ref r52))) (separator dot) (member (ref r53))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r54)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9746) (line 185) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r55)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r56)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9829) (line 186) (column 79) (len 5)) (member-access (base (expression (span (offset 9829) (line 186) (column 79) (len 3)) (ref r57))) (separator dot) (member (ref r58))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r59)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9851) (line 186) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 9852) (line 186) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r60)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r61)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 9926) (line 187) (column 70) (len 30)) (tuple (expression (span (offset 9927) (line 187) (column 71) (len 8)) (ref r62)) (expression (span (offset 9937) (line 187) (column 81) (len 6)) (ref r63)) (expression (span (offset 9945) (line 187) (column 89) (len 10)) (ref r64))))))) (body semicolon)))))) (attribute-def (declaration-name "soundEnergy") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r65)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "soundPower") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r66)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))) (attribute-def (declaration-name "SoundIntensityValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r67)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r68)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r69)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r70)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r71)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "soundIntensity") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r72)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SoundIntensityUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r73)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r74)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r75)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12778) (line 243) (column 75) (len 5)) (member-access (base (expression (span (offset 12778) (line 243) (column 75) (len 3)) (ref r76))) (separator dot) (member (ref r77))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r78)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12800) (line 243) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r79)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r80)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12883) (line 244) (column 79) (len 5)) (member-access (base (expression (span (offset 12883) (line 244) (column 79) (len 3)) (ref r81))) (separator dot) (member (ref r82))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r83)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12905) (line 244) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 12906) (line 244) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r84)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r85)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12980) (line 245) (column 70) (len 20)) (tuple (expression (span (offset 12981) (line 245) (column 71) (len 6)) (ref r86)) (expression (span (offset 12989) (line 245) (column 79) (len 10)) (ref r87))))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianSoundIntensity3dVector") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r88)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r89)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 13706) (line 261) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r90)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r91)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianSoundIntensity3dVector") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r92)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianSoundIntensity3dCoordinateFrame") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r93)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r94)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14010) (line 268) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r95)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 14054) (line 269) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r96)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r97)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "SoundExposureValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r98)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r99)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r100)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r101)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r102)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "soundExposure") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r103)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SoundExposureUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r104)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r105)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r106)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15426) (line 294) (column 77) (len 5)) (member-access (base (expression (span (offset 15426) (line 294) (column 77) (len 3)) (ref r107))) (separator dot) (member (ref r108))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r109)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15448) (line 294) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 15449) (line 294) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r110)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r111)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15528) (line 295) (column 75) (len 5)) (member-access (base (expression (span (offset 15528) (line 295) (column 75) (len 3)) (ref r112))) (separator dot) (member (ref r113))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r114)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15550) (line 295) (column 97) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r115)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r116)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15633) (line 296) (column 79) (len 5)) (member-access (base (expression (span (offset 15633) (line 296) (column 79) (len 3)) (ref r117))) (separator dot) (member (ref r118))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r119)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15655) (line 296) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 15656) (line 296) (column 102) (len 1)) (integer 3)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r120)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r121)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15730) (line 297) (column 70) (len 30)) (tuple (expression (span (offset 15731) (line 297) (column 71) (len 8)) (ref r122)) (expression (span (offset 15741) (line 297) (column 81) (len 6)) (ref r123)) (expression (span (offset 15749) (line 297) (column 89) (len 10)) (ref r124))))))) (body semicolon)))))) (attribute-def (declaration-name "CharacteristicImpedanceOfAMediumForLongitudinalWavesValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r125)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r126)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r127)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r128)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r129)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "characteristicImpedanceOfAMediumForLongitudinalWaves") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r130)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "CharacteristicImpedanceOfAMediumForLongitudinalWavesUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r131)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r132)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r133)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17454) (line 321) (column 77) (len 5)) (member-access (base (expression (span (offset 17454) (line 321) (column 77) (len 3)) (ref r134))) (separator dot) (member (ref r135))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r136)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17476) (line 321) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 17477) (line 321) (column 100) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r137)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r138)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17556) (line 322) (column 75) (len 5)) (member-access (base (expression (span (offset 17556) (line 322) (column 75) (len 3)) (ref r139))) (separator dot) (member (ref r140))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r141)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17578) (line 322) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r142)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r143)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17661) (line 323) (column 79) (len 5)) (member-access (base (expression (span (offset 17661) (line 323) (column 79) (len 3)) (ref r144))) (separator dot) (member (ref r145))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r146)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17683) (line 323) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 17684) (line 323) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r147)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r148)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17758) (line 324) (column 70) (len 30)) (tuple (expression (span (offset 17759) (line 324) (column 71) (len 8)) (ref r149)) (expression (span (offset 17769) (line 324) (column 81) (len 6)) (ref r150)) (expression (span (offset 17777) (line 324) (column 89) (len 10)) (ref r151))))))) (body semicolon)))))) (attribute-def (declaration-name "AcousticImpedanceValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r152)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r153)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r154)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r155)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r156)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "acousticImpedance") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r157)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "AcousticImpedanceUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r158)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r159)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r160)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19258) (line 348) (column 77) (len 5)) (member-access (base (expression (span (offset 19258) (line 348) (column 77) (len 3)) (ref r161))) (separator dot) (member (ref r162))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r163)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19280) (line 348) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 19281) (line 348) (column 100) (len 1)) (integer 4)))))))) (body semicolon)))) (attribute-usage (declaration-name "massPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r164)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r165)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19360) (line 349) (column 75) (len 5)) (member-access (base (expression (span (offset 19360) (line 349) (column 75) (len 3)) (ref r166))) (separator dot) (member (ref r167))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r168)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19382) (line 349) (column 97) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r169)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r170)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19465) (line 350) (column 79) (len 5)) (member-access (base (expression (span (offset 19465) (line 350) (column 79) (len 3)) (ref r171))) (separator dot) (member (ref r172))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r173)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19487) (line 350) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 19488) (line 350) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r174)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r175)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19562) (line 351) (column 70) (len 30)) (tuple (expression (span (offset 19563) (line 351) (column 71) (len 8)) (ref r176)) (expression (span (offset 19573) (line 351) (column 81) (len 6)) (ref r177)) (expression (span (offset 19581) (line 351) (column 89) (len 10)) (ref r178))))))) (body semicolon)))))) (attribute-def (declaration-name "SoundPressureLevelValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r179)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r180)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r181)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r182)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r183)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "soundPressureLevel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r184)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SoundPressureLevelUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r185)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace)) (attribute-def (declaration-name "SoundPowerLevelValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r186)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r187)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r188)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r189)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r190)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "soundPowerLevel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r191)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SoundPowerLevelUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r192)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace)) (attribute-def (declaration-name "SoundExposureLevelValue") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r193)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r194)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r195)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r196)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r197)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "soundExposureLevel") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r198)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SoundExposureLevelUnit") (short-name none) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r199)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace)) (attribute-def (declaration-name "reverberationTime") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r200)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc))))))
)
~~~
