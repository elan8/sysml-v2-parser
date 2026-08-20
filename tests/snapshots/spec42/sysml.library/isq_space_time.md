# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Domain Libraries/Quantities and Units/ISQSpaceTime"))
~~~
# SOURCE
~~~sysml
standard library package ISQSpaceTime {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-3:2019 "Space and Time"
     * see also https://www.iso.org/standard/64974.html
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

    /* ISO-80000-3 item 3-1.1 length */
    /* See package ISQBase for the declarations of LengthValue and LengthUnit */

    /* ISO-80000-3 item 3-1.2 width, breadth */
    attribute width: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.2 width, breadth
         * symbol(s): `b`, `B`
         * application domain: generic
         * name: Width (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: minimum length of a straight line segment between two parallel straight lines (in two dimensions) or planes (in three dimensions) that enclose a given geometrical shape
         * remarks: This quantity is non-negative.
         */
    }

    alias breadth for width;

    /* ISO-80000-3 item 3-1.3 height, depth, altitude */
    attribute height: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.3 height, depth, altitude
         * symbol(s): `h`, `H`
         * application domain: generic
         * name: Height (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: minimum length of a straight line segment between a point and a reference line or reference surface
         * remarks: This quantity is usually signed. The sign expresses the position of the particular point with respect to the reference line or surface and is chosen by convention. The symbol `H` is often used to denote altitude, i.e. height above sea level.
         */
    }

    alias depth for height;

    alias altitude for height;

    /* ISO-80000-3 item 3-1.4 thickness */
    attribute thickness: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.4 thickness
         * symbol(s): `d`, `δ`
         * application domain: generic
         * name: Thickness (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: width (item 3-1.2)
         * remarks: This quantity is non-negative.
         */
    }

    /* ISO-80000-3 item 3-1.5 diameter */
    attribute diameter: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.5 diameter
         * symbol(s): `d`, `D`
         * application domain: generic
         * name: Diameter (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: width (item 3-1.2) of a circle, cylinder or sphere
         * remarks: This quantity is non-negative.
         */
    }

    /* ISO-80000-3 item 3-1.6 radius */
    attribute radius: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.6 radius
         * symbol(s): `r`, `R`
         * application domain: generic
         * name: Radius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: half of a diameter (item 3-1.5)
         * remarks: This quantity is non-negative.
         */
    }

    /* ISO-80000-3 item 3-1.7 path length, arc length */
    attribute pathLength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.7 path length, arc length
         * symbol(s): `s`
         * application domain: generic
         * name: PathLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: length of a rectifiable curve between two of its points
         * remarks: The differential path length at a given point of a curve is: `ds = sqrt(dx^2 + dy^2 + dz^2)` where `x`, `y`, and `z` denote the Cartesian coordinates (ISO 80000-2) of the particular point. There are curves which are not rectifiable, for example fractal curves.
         */
    }

    alias arcLength for pathLength;

    /* ISO-80000-3 item 3-1.8 distance */
    attribute distance: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.8 distance
         * symbol(s): `d`, `r`
         * application domain: generic
         * name: Distance (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: shortest path length (item 3-1.7) between two points in a metric space
         * remarks: A metric space might be curved. An example of a curved metric space is the surface of the Earth. In this case, distances are measured along great circles. A metric is not necessarily Euclidean.
         */
    }

    /* ISO-80000-3 item 3-1.9 radial distance */
    attribute radialDistance: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.9 radial distance
         * symbol(s): `r_Q`, `ρ`
         * application domain: generic
         * name: RadialDistance (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (item 3-1.8), where one point is located on an axis or within a closed non self-intersecting curve or surface
         * remarks: The subscript Q denotes the point from which the radial distance is measured. Examples of closed non self-intersecting curves are circles or ellipses. Examples of closed non self-intersecting surfaces are surfaces of spheres or egg-shaped objects.
         */
    }

    /* Spatial coordinate frames */
    
    attribute def Spatial3dCoordinateFrame :> '3dCoordinateFrame' {
        doc
        /*
         * Most general spatial 3D coordinate frame
         */
        attribute :>> isBound = true;
    }

    attribute def CartesianSpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc
        /*
         * Cartesian spatial 3D coordinate frame
         *
         * source: ISO 80000-2 item 2-17.1 Cartesian coordinates
         *
         * The components of a vector expressed on a Cartesian spatial coordinate frame are all LengthValues, and denoted with symbols `x`, `y`, `z`.
         *
         * Note 1: The Cartesian basis vectors `vec(e_x)`, `vec(e_y)` and `vec(e_z)` form an orthonormal right-handed coordinate frame.
         * Note 2: The measurement units for the 3 dimensions are typically the same, but may be different.
         */
        attribute xUnit : LengthUnit = mRefs#(1);
        attribute yUnit : LengthUnit = mRefs#(2);
        attribute zUnit : LengthUnit = mRefs#(3);
        attribute :>> mRefs : LengthUnit[3];
        attribute :>> isOrthogonal = true;
    }

    attribute universalCartesianSpatial3dCoordinateFrame : CartesianSpatial3dCoordinateFrame[1] {
        doc
        /*
         * A singleton CartesianSpatial3dCoordinateFrame that can be used as a default universal Cartesian 3D coordinate frame.
         */
         
        attribute :>> mRefs default (SI::m, SI::m, SI::m) {
            doc /*
             * By default, the universalCartesianSpatial3dCoordinateFrame uses meters as the units on all three axes.
             */
        }
        
        attribute :>> transformation[0..0] {
            doc /*
             * The universalCartesianSpatial3dCoordinateFrame is the "top-level" coordinate frame, not nested in any other frame.
             */
        }
        
    }

    attribute def CylindricalSpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc
        /*
         * Cylindrical spatial 3D coordinate frame
         *
         * source: ISO 80000-2 item 2-17.2 cylindrical coordinates
         *
         * The components of a (position) vector to a point P in a cylindrical coordinate frame are:
         * - radialDistance (symbol `ρ`) defined by LengthValue, that is the radial distance from the cylinder axis to P
         * - azimuth (symbol `φ`) defined by AngularMeasure, that is the angle between the azimuth reference direction and the line segment
         * from the cylinder axis, in the plane that is orthogonal to the cylinder axis and intersects P
         * - z coordinate (symbol `z`) defined by LengthValue, the coordinate along the clyinder axis.
         *
         * Note 1: The basis vectors `vec(e_ρ)(φ)`, `vec(e_φ)(φ)` and `vec(e_z)` form an orthonormal right-handed coordinate frame, where
         * `vec(e_φ)` is tangent to the circular arc in the `φ` direction.
         * Note 2: In order to enable transformation to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned
         * with the `φ=0` direction in the cylindrical frame, and the `vec(e_z)` Cartesian basis vector is aligned with
         * the `vec(e_z)` cylindrical basis vector.
         * Note 3: If `z = 0`, then `ρ` and `φ` are polar coordinates in the XY-plane.
         * Note 4: See also https://en.wikipedia.org/wiki/Cylindrical_coordinate_system .
         */
        attribute radialDistanceUnit : LengthUnit;
        attribute azimuthUnit : AngularMeasureUnit;
        attribute zUnit : LengthUnit;
        attribute :>> mRefs = (radialDistanceUnit, azimuthUnit, zUnit);
        attribute :>> isOrthogonal = true;
    }

    attribute def SphericalSpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc
        /*
         * Spherical spatial 3D coordinate frame
         *
         * source: ISO 80000-2 item 2-17.3 spherical coordinates
         *
         * The components of a (position) vector to a point P specified in a spherical coordinate frame are:
         * - radialDistance (symbol `r`) defined by LengthValue, that is the distance from the origin to P
         * - inclination (symbol `θ`) defined by AngularMeasure, that is the angle between the zenith direction and the line segment from origin to P
         * - azimuth (symbol `φ`) defined by AngularMeasure, that is the angle between the azimuth reference direction and the line segment
         * from the origin to the orthogonal projection of P on the reference plane, normal to the zenith direction.
         *
         * Note 1: The basis vectors `vec(e_r)(θ,φ)`, `vec(e_θ)(θ,φ)` and `vec(e_φ)(φ)` form an orthonormal right-handed frame, where
         * `vec(e_θ)` and `vec(e_φ)` are tangent to the respective circular arcs in the `θ` and `φ` directions.
         * Note 2: In order to transform to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned
         * with the `θ=π/4` and `φ=0` direction in the spherical frame, and the `vec(e_z)` Cartesian basis vector is aligned
         * with the `θ=0` zenith direction in the spherical frame.
         * Note 3: If `θ = π/4`, then `ρ` and `φ` are polar coordinates in the XY-plane.
         * Note 4: See also https://en.wikipedia.org/wiki/Spherical_coordinate_system .
         */
        attribute radialDistanceUnit : LengthUnit;
        attribute inclinationUnit : AngularMeasureUnit;
        attribute azimuthUnit : AngularMeasureUnit;
        attribute :>> mRefs = (radialDistanceUnit, inclinationUnit, azimuthUnit);
        attribute :>> isOrthogonal = true;
    }

     attribute def PlanetarySpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc
        /*
         * Planetary spatial 3D coordinate frame
         *
         * A planetary spatial 3D coordinate frame is a generalization for any planet of the geographic coordinate frame and geocentric coordinate
         * for Earth. In such coordinate frames, typically the origin is located at the planet's centre of gravity, and the surface of the planet
         * is approximated by a reference ellipsoid centred on the origin, with its major axes oriented along the south to north pole vector and
         * the equatorial plane.
         *
         * The components of a (position) vector to a point P specified in a planetary coordinate frame are:
         * - latitude (symbol `lat` or `φ`) defined by AngularMeasure, that is the angle between the equatorial plane and the vector from
         *   the origin to P, similar to the inclination in a spherical spatial coordinate frame. Typically, the zero reference latitude is chosen
         *   for positions in the equatorial plane, with positive latitude for positions in the northern hemisphere and negative latitude for positions
         *   in the southern hemisphere.
         * - longitude (symbol `long` or `λ`) defined by AngularMeasure, that is the angle between a reference meridian and the meridian
         *   passing through P, similar to the azimuth of a spherical spatial coordinate frame. The convention is to connotate positive longitude
         *   with eastward direction and negative longitude with westward direction. The reference meridian for `long=0` is chosen to pass
         *   through a particular feature of the planet, e.g., for Earth typically the position of the British Royal Observatory in Greenwich, UK.
         * - altitude (symbol `h`) defined by LengthValue, that is the distance between P and the reference ellipsoid
         *   in the normal direction to the ellipsoid. Positive altitude specifies a position above the reference ellipsoid surface,
         *   while a negative value specifies a position below.
         *
         * Note 1: The reference meridian is also called prime meridian.
         * Note 2: The basis vectors `vec(e_φ)(φ)`, `vec(e_λ)(λ)` and `vec(e_h)(φ,λ)` form an orthonormal right-handed frame, where
         * `vec(e_φ)` and `vec(e_λ)` are tangent to the reference ellipsoid in the respective latitude and longitude directions,
         * and `vec(e_h)` is normal to the reference ellipsoid.
         * Note 3: In order to transform to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned
         * with the `φ=0` and `λ=0` direction in the planetary frame, and the `vec(e_z)` Cartesian basis vector is aligned
         * with the `λ=π/2` (north pole) direction in the planetary frame.
         * Note 4: See also https://en.wikipedia.org/wiki/Planetary_coordinate_system .
         */
        attribute latitudeUnit : AngularMeasureUnit;
        attribute longitudeUnit : AngularMeasureUnit;
        attribute altitudeUnit : LengthUnit;
        attribute :>> mRefs = (longitudeUnit, latitudeUnit, altitudeUnit);
        attribute :>> isOrthogonal = true;
    }

    /* ISO-80000-3 item 3-1.10 position vector */
    attribute def Position3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-1.10 position vector
         * symbol(s): `vec(r)`
         * application domain: generic
         * name: PositionVector
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity from the origin of a coordinate system to a point in space
         * remarks: Position vectors are so-called bounded vectors, i.e. their magnitude (ISO 80000-2) and direction depend on the particular coordinate system used.
         */
        attribute :>> isBound = true;
        attribute :>> mRef: Spatial3dCoordinateFrame[1];
    }

    attribute position3dVector: Position3dVector :> vectorQuantities;

    attribute def CartesianPosition3dVector :> Position3dVector {
        attribute x : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute y : LengthValue = num#(2) [mRef.mRefs#(2)];
        attribute z : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame[1];
    }
    attribute cartesianPosition3dVector : CartesianPosition3dVector :> position3dVector;

    attribute def CylindricalPosition3dVector :> Position3dVector {
        attribute <'ρ'> radialDistance : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute <'φ'> azimuth : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <h> height : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : CylindricalSpatial3dCoordinateFrame[1];
    }
    attribute cylindricalPosition3dVector : CylindricalPosition3dVector :> position3dVector;

    attribute def SphericalPosition3dVector :> Position3dVector {
        attribute <r> radialDistance : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute <'θ'> inclination : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <'φ'> azimuth : AngularMeasureUnit = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : SphericalSpatial3dCoordinateFrame[1];
    }
    attribute sphericalPosition3dVector : SphericalPosition3dVector :> position3dVector;

    attribute def PlanetaryPosition3dVector :> Position3dVector {
        attribute <lat> latitude : AngularMeasureUnit = num#(1) [mRef.mRefs#(1)];
        attribute <long> longitude : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <h> altitude : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : PlanetarySpatial3dCoordinateFrame[1];
    }
    attribute planetaryPosition3dVector : PlanetaryPosition3dVector :> position3dVector;

    /* ISO-80000-3 item 3-1.11 displacement */
    attribute def Displacement3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-1.11 displacement
         * symbol(s): `vec(Δr)`
         * application domain: generic
         * name: Displacement
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity between any two points in space
         * remarks: Displacement vectors are so-called free vectors, i.e. their magnitude (ISO 80000-2) and direction do not depend on a particular coordinate system. The magnitude of this vector is also called displacement.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: Spatial3dCoordinateFrame[1];
    }

    attribute displacement3dVector: Displacement3dVector :> vectorQuantities;

    attribute def CartesianDisplacement3dVector :> Displacement3dVector {
        attribute x : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute y : LengthValue = num#(2) [mRef.mRefs#(2)];
        attribute z : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame[1];
    }
    attribute cartesianDisplacement3dVector : CartesianDisplacement3dVector :> displacement3dVector;

    attribute def CylindricalDisplacement3dVector :> Displacement3dVector {
        attribute <'ρ'> radialDistance : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute <'φ'> azimuth : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <h> height : LengthValue = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : CylindricalSpatial3dCoordinateFrame[1];
    }
    attribute cylindricalDisplacement3dVector : CylindricalDisplacement3dVector :> displacement3dVector;

    attribute def SphericalDisplacement3dVector :> Displacement3dVector {
        attribute <r> radialDistance : LengthValue = num#(1) [mRef.mRefs#(1)];
        attribute <'θ'> inclination : AngularMeasureUnit = num#(2) [mRef.mRefs#(2)];
        attribute <'φ'> azimuth : AngularMeasureUnit = num#(3) [mRef.mRefs#(3)];
        attribute :>> mRef : SphericalSpatial3dCoordinateFrame[1];
    }
    attribute sphericalDisplacement3dVector : SphericalDisplacement3dVector :> displacement3dVector;

    /* ISO-80000-3 item 3-1.12 radius of curvature */
    attribute radiusOfCurvature: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-1.12 radius of curvature
         * symbol(s): `ρ`
         * application domain: generic
         * name: RadiusOfCurvature (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: radius (item 3-1.6) of the osculating circle of a planar curve at a particular point of the curve
         * remarks: The radius of curvature is only defined for curves which are at least twice continuously differentiable.
         */
    }

    /* ISO-80000-3 item 3-2 curvature */
    attribute def CurvatureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-2 curvature
         * symbol(s): `κ`
         * application domain: generic
         * name: Curvature
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: inverse of the radius of curvature (item 3-1.12)
         * remarks: The curvature is given by: `κ = 1/ρ` where `ρ` denotes the radius of curvature (item 3-1.12).
         */
        attribute :>> num: Real;
        attribute :>> mRef: CurvatureUnit[1];
    }

    attribute curvature: CurvatureValue[*] nonunique :> scalarQuantities;

    attribute def CurvatureUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-3 item 3-3 area */
    attribute def AreaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-3 area
         * symbol(s): `A`, `S`
         * application domain: generic
         * name: Area
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: extent of a two-dimensional geometrical shape
         * remarks: The surface element at a given point of a surface is given by: `dA = g du dv` where `u` and `v` denote the Gaussian surface coordinates and `g` denotes the determinant of the metric tensor (ISO 80000-2) at the particular point. The symbol `dσ` is also used for the surface element.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AreaUnit[1];
    }

    attribute area: AreaValue[*] nonunique :> scalarQuantities;

    attribute def AreaUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-3 item 3-4 volume */
    attribute def VolumeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-4 volume
         * symbol(s): `V`, `(S)`
         * application domain: generic
         * name: Volume
         * quantity dimension: L^3
         * measurement unit(s): m^3
         * tensor order: 0
         * definition: extent of a three-dimensional geometrical shape
         * remarks: The volume element in Euclidean space is given by: `dV = dx dy dz` where `dx`, `dy`, and `dz` denote the differentials of the Cartesian coordinates (ISO 80000-2). The symbol `dτ` is also used for the volume element.
         */
        attribute :>> num: Real;
        attribute :>> mRef: VolumeUnit[1];
    }

    attribute volume: VolumeValue[*] nonunique :> scalarQuantities;

    attribute def VolumeUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 3; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-3 item 3-5 angular measure, plane angle */
    attribute def AngularMeasureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-5 angular measure, plane angle
         * symbol(s): `α`, `β`, `γ`
         * application domain: generic
         * name: AngularMeasure
         * quantity dimension: 1
         * measurement unit(s): rad, 1
         * tensor order: 0
         * definition: measure of a geometric figure, called plane angle, formed by two rays, called the sides of the plane angle, emanating from a common point, called the vertex of the plane angle
         * remarks: The angular measure is given by: `α = s/r` where `s` denotes the arc length (item 3-1.7) of the included arc of a circle, centred at the vertex of the plane angle, and `r` the radius (item 3-1.6) of that circle. Other symbols are also used.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularMeasureUnit[1];
    }

    attribute angularMeasure: AngularMeasureValue[*] nonunique :> scalarQuantities;

    attribute def AngularMeasureUnit :> DimensionOneUnit {
    }

    alias PlaneAngleUnit for AngularMeasureUnit;
    alias PlaneAngleValue for AngularMeasureValue;
    alias planeAngle for angularMeasure;

    /* ISO-80000-3 item 3-6 rotational displacement, angular displacement */
    attribute rotationalDisplacement: AngularMeasureValue :> scalarQuantities {
        doc
        /*
         * source: item 3-6 rotational displacement, angular displacement
         * symbol(s): `ϑ`, `φ`
         * application domain: generic
         * name: RotationalDisplacement (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): rad, 1
         * tensor order: 0
         * definition: quotient of the traversed circular path length (item 3-1.7) of a point in space during a rotation and its distance (item 3-1.8) from the axis or centre of rotation
         * remarks: The rotational displacement is given by: `φ = s/r` where `s` denotes the traversed path length (item 3-1.7) along the periphery of a circle, centred at the vertex of the plane angle, and `r` the radius (item 3-1.6) of that circle. The rotational displacement is signed. The sign denotes the direction of rotation and is chosen by convention. Other symbols are also used.
         */
    }

    alias angularDisplacement for rotationalDisplacement;

    /* ISO-80000-3 item 3-7 phase angle */
    attribute phaseAngle: AngularMeasureValue :> scalarQuantities {
        doc
        /*
         * source: item 3-7 phase angle
         * symbol(s): `φ`, `ϕ`
         * application domain: generic
         * name: PhaseAngle (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): rad, 1
         * tensor order: 0
         * definition: angular measure (item 3-5) between the positive real axis and the radius of the polar representation of the complex number in the complex plane
         * remarks: The phase angle (often imprecisely referred to as the "phase") is the argument of a complex number. Other symbols are also used.
         */
    }

    /* ISO-80000-3 item 3-8 solid angular measure */
    attribute def SolidAngularMeasureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-8 solid angular measure
         * symbol(s): `Ω`
         * application domain: generic
         * name: SolidAngularMeasure
         * quantity dimension: 1
         * measurement unit(s): sr, 1
         * tensor order: 0
         * definition: measure of a conical geometric figure, called solid angle, formed by all rays, originating from a common point, called the vertex of the solid angle, and passing through the points of a closed, non-self-intersecting curve in space considered as the border of a surface
         * remarks: The differential solid angular measure expressed in spherical coordinates (ISO 80000-2) is given by: `dΩ = A/r^2 * sin(θ * dθ * dφ)` where `A` is area, `r` is radius, `θ` and `φ` are spherical coordinates.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SolidAngularMeasureUnit[1];
    }

    attribute solidAngularMeasure: SolidAngularMeasureValue[*] nonunique :> scalarQuantities;

    attribute def SolidAngularMeasureUnit :> DimensionOneUnit {
    }

    /* ISO-80000-3 item 3-9 duration, time */
    /* See package ISQBase for the declarations of DurationValue and DurationUnit */

    alias TimeUnit for DurationUnit;
    alias TimeValue for DurationValue;
    alias time for duration;

    /* ISO-80000-3 item 3-10.1 velocity */
    attribute def CartesianVelocity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-10.1 velocity
         * symbol(s): `vec(v)`, `u,v,w`
         * application domain: generic
         * name: Velocity
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m/s, m*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of a position vector (item 3-1.10)
         * remarks: The velocity vector is given by: `vec(v) = (d vec(r)) / (dt)` where `vec(r)` denotes the position vector (item 3-1.10) and `t` the duration (item 3-9). When the general symbol `vec(v)` is not used for the velocity, the symbols `u`, `v`, `w` may be used for the components (ISO 80000-2) of the velocity.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianVelocity3dCoordinateFrame[1];
    }

    attribute cartesianVelocity3dVector: CartesianVelocity3dVector :> vectorQuantities;

    attribute def CartesianVelocity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: SpeedUnit[3];
    }

    /* ISO-80000-3 item 3-10.2 speed */
    attribute def SpeedValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-10.2 speed
         * symbol(s): `v`
         * application domain: generic
         * name: Speed
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m/s, m*s^-1
         * tensor order: 0
         * definition: magnitude (ISO 80000-2) of the velocity (item 3-10.1)
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: SpeedUnit[1];
    }

    attribute speed: SpeedValue[*] nonunique :> scalarQuantities;

    attribute def SpeedUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    /* ISO-80000-3 item 3-11 acceleration */
    attribute def AccelerationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-11 acceleration (magnitude)
         * symbol(s): `a`
         * application domain: generic
         * name: Acceleration
         * quantity dimension: L^1*T^-2
         * measurement unit(s): m*s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity giving the rate of change of velocity (item 3-10)
         * remarks: The acceleration vector is given by: `vec(a) = (d vec(v))/(dt)` where `vec(v)` denotes the velocity (item 3-10.1) and `t` the duration (item 3-9). The magnitude (ISO 80000-2) of the acceleration of free fall is usually denoted by `g`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AccelerationUnit[1];
    }

    attribute acceleration: AccelerationValue[*] nonunique :> scalarQuantities;

    attribute def AccelerationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    attribute def CartesianAcceleration3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-11 acceleration (vector)
         * symbol(s): `vec(a)`
         * application domain: generic
         * name: Acceleration
         * quantity dimension: L^1*T^-2
         * measurement unit(s): m*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of velocity (item 3-10)
         * remarks: The acceleration vector is given by: `vec(a) = (d vec(v))/(dt)` where `vec(v)` denotes the velocity (item 3-10.1) and `t` the duration (item 3-9). The magnitude (ISO 80000-2) of the acceleration of free fall is usually denoted by `g`.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianAcceleration3dCoordinateFrame[1];
    }

    attribute cartesianAcceleration3dVector: CartesianAcceleration3dVector :> vectorQuantities;

    attribute def CartesianAcceleration3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: AccelerationUnit[3];
    }

    /* ISO-80000-3 item 3-12 angular velocity */
    attribute def AngularVelocityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-12 angular velocity (magnitude)
         * symbol(s): `ω`
         * application domain: generic
         * name: AngularVelocity
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity giving the rate of change of the rotational displacement (item 3-6) as its magnitude (ISO 80000-2) and with a direction equal to the direction of the axis of rotation
         * remarks: The angular velocity vector is given by: `vec(ω) = (d φ) / (dt) vec(u)` where `φ` denotes the angular displacement (item 3-6), `t` the duration (item 3-9), and `vec(u)` the unit vector (ISO 80000-2) along the axis of rotation in the direction for which the rotation corresponds to a right-hand spiral.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularVelocityUnit[1];
    }

    attribute angularVelocity: AngularVelocityValue[*] nonunique :> scalarQuantities;

    attribute def AngularVelocityUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    attribute def CartesianAngularVelocity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-12 angular velocity (vector)
         * symbol(s): `vec(ω)`
         * application domain: generic
         * name: AngularVelocity
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of the rotational displacement (item 3-6) as its magnitude (ISO 80000-2) and with a direction equal to the direction of the axis of rotation
         * remarks: The angular velocity vector is given by: `vec(ω) = (d φ) / (dt) vec(u)` where `φ` denotes the angular displacement (item 3-6), `t` the duration (item 3-9), and `vec(u)` the unit vector (ISO 80000-2) along the axis of rotation in the direction for which the rotation corresponds to a right-hand spiral.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianAngularVelocity3dCoordinateFrame[1];
    }

    attribute cartesianAngularVelocity3dVector: CartesianAngularVelocity3dVector :> vectorQuantities;

    attribute def CartesianAngularVelocity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: AngularVelocityUnit[3];
    }

    /* ISO-80000-3 item 3-13 angular acceleration */
    attribute def AngularAccelerationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-13 angular acceleration (magnitude)
         * symbol(s): `α`
         * application domain: generic
         * name: AngularAcceleration
         * quantity dimension: T^-2
         * measurement unit(s): rad*s^-2, s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity giving the rate of change of angular velocity (item 3-12)
         * remarks: The angular acceleration vector is given by: `vec α = (d vec(ω))/(dt)` Where `vec(ω)` denotes the angular velocity (item 3-12) and `t` the duration (item 3-9).
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularAccelerationUnit[1];
    }

    attribute angularAcceleration: AngularAccelerationValue[*] nonunique :> scalarQuantities;

    attribute def AngularAccelerationUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    attribute def CartesianAngularAcceleration3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-13 angular acceleration (vector)
         * symbol(s): `vec(α)`
         * application domain: generic
         * name: AngularAcceleration
         * quantity dimension: T^-2
         * measurement unit(s): rad*s^-2, s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of angular velocity (item 3-12)
         * remarks: The angular acceleration vector is given by: `vec α = (d vec(ω))/(dt)` Where `vec(ω)` denotes the angular velocity (item 3-12) and `t` the duration (item 3-9).
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianAngularAcceleration3dCoordinateFrame[1];
    }

    attribute cartesianAngularAcceleration3dVector: CartesianAngularAcceleration3dVector :> vectorQuantities;

    attribute def CartesianAngularAcceleration3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: AngularAccelerationUnit[3];
    }

    /* ISO-80000-3 item 3-14 period duration, period */
    attribute periodDuration: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 3-14 period duration, period
         * symbol(s): `T`
         * application domain: generic
         * name: PeriodDuration (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: duration (item 3-9) of one cycle of a periodic event
         * remarks: A periodic event is an event that occurs regularly with a fixed time interval.
         */
    }

    alias period for periodDuration;

    /* ISO-80000-3 item 3-15 time constant */
    attribute timeConstant: DurationValue :> scalarQuantities {
        doc
        /*
         * source: item 3-15 time constant
         * symbol(s): `τ`, `T`
         * application domain: generic
         * name: TimeConstant (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: parameter characterizing the response to a step input of a first-order, linear time-invariant system
         * remarks: If a quantity is a function of the duration (item 3-9) expressed by: `F(t) prop e^(-t/τ)` where `t` denotes the duration (item 3-9), then `τ` denotes the time constant. Here the time constant `τ` applies to an exponentially decaying quantity.
         */
    }

    /* ISO-80000-3 item 3-16 rotation */
    attribute rotation: CountValue :> scalarQuantities {
        doc
        /*
         * source: item 3-16 rotation
         * symbol(s): `N`
         * application domain: generic
         * name: Rotation (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of revolutions
         * remarks: `N` is the number (not necessarily an integer) of revolutions, for example, of a rotating body about a given axis. Its value is given by: `N = φ/(2 π)` where `φ` denotes the measure of rotational displacement (item 3-6).
         */
    }

    /* ISO-80000-3 item 3-17.1 frequency */
    attribute def FrequencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-17.1 frequency
         * symbol(s): `f`, `ν`
         * application domain: generic
         * name: Frequency
         * quantity dimension: T^-1
         * measurement unit(s): Hz, s^-1
         * tensor order: 0
         * definition: inverse of period duration (item 3-14)
         * remarks: The frequency is given by: `f = 1/T` where `T` denotes the period duration (item 3-14).
         */
        attribute :>> num: Real;
        attribute :>> mRef: FrequencyUnit[1];
    }

    attribute frequency: FrequencyValue[*] nonunique :> scalarQuantities;

    attribute def FrequencyUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-3 item 3-17.2 rotational frequency */
    attribute rotationalFrequency: FrequencyValue :> scalarQuantities {
        doc
        /*
         * source: item 3-17.2 rotational frequency
         * symbol(s): `n`
         * application domain: generic
         * name: RotationalFrequency (specializes Frequency)
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: duration (item 3-9) of one cycle of a periodic event
         * remarks: The rotational frequency is given by: `n = (dN) / (dt)` where `N` denotes the rotation (item 3-16) and `t` is the duration (item 3-9).
         */
    }

    /* ISO-80000-3 item 3-18 angular frequency */
    attribute def AngularFrequencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-18 angular frequency
         * symbol(s): `ω`
         * application domain: generic
         * name: AngularFrequency
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: rate of change of the phase angle (item 3-7)
         * remarks: The angular frequency is given by: `ω = 2 π f` where `f` denotes the frequency (item 3-17.1).
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularFrequencyUnit[1];
    }

    attribute angularFrequency: AngularFrequencyValue[*] nonunique :> scalarQuantities;

    attribute def AngularFrequencyUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-3 item 3-19 wavelength */
    attribute wavelength: LengthValue :> scalarQuantities {
        doc
        /*
         * source: item 3-19 wavelength
         * symbol(s): `λ`
         * application domain: generic
         * name: Wavelength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: length (item 3-1.1) of the repetition interval of a wave
         * remarks: None.
         */
    }

    /* ISO-80000-3 item 3-20 repetency, wavenumber */
    attribute def RepetencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-20 repetency, wavenumber
         * symbol(s): `σ`, `ṽ`
         * application domain: generic
         * name: Repetency
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: inverse of the wavelength (item 3-19)
         * remarks: The repetency is given by: `σ = 1 / λ` where `λ` denotes the wavelength (item 3-19).
         */
        attribute :>> num: Real;
        attribute :>> mRef: RepetencyUnit[1];
    }

    attribute repetency: RepetencyValue[*] nonunique :> scalarQuantities;

    attribute def RepetencyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias WavenumberUnit for RepetencyUnit;
    alias WavenumberValue for RepetencyValue;
    alias wavenumber for repetency;

    /* ISO-80000-3 item 3-21 wave vector */
    attribute def CartesianWave3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-21 wave vector
         * symbol(s): `vec(k)`
         * application domain: generic
         * name: WaveVector
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 1
         * definition: vector normal to the surfaces of constant phase angle (item 3-7) of a wave, with the magnitude (ISO 80000-2) of repetency (item 3-20)
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef: CartesianWaveVector3dCoordinateFrame[1];
    }

    attribute cartesianWave3dVector: CartesianWave3dVector :> vectorQuantities;

    attribute def CartesianWaveVector3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs: RepetencyUnit[3];
    }

    /* ISO-80000-3 item 3-22 angular repetency, angular wavenumber */
    attribute def AngularRepetencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-22 angular repetency, angular wavenumber
         * symbol(s): `k`
         * application domain: generic
         * name: AngularRepetency
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: magnitude (ISO 80000-2) of the wave vector (item 3-21)
         * remarks: The angular repetency is given by: `κ = (2 π)/λ` where `λ` denotes the wavelength (item 3-19).
         */
        attribute :>> num: Real;
        attribute :>> mRef: AngularRepetencyUnit[1];
    }

    attribute angularRepetency: AngularRepetencyValue[*] nonunique :> scalarQuantities;

    attribute def AngularRepetencyUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias AngularWavenumberUnit for AngularRepetencyUnit;
    alias AngularWavenumberValue for AngularRepetencyValue;
    alias angularWavenumber for angularRepetency;

    /* ISO-80000-3 item 3-23.1 phase velocity, phase speed */
    attribute def PhaseVelocityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-23.1 phase velocity, phase speed
         * symbol(s): `c`, `v`, `(ν)`, `c_φ`, `v_φ`, `(ν_φ)`
         * application domain: generic
         * name: PhaseVelocity
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 0
         * definition: speed with which the phase angle (item 3-7) of a wave propagates in space
         * remarks: The phase velocity is given by: `c = ω/κ` where `ω` denotes the angular frequency (item 3-18) and `k` the angular repetency (item 3-22). If phase velocities of electromagnetic waves and other phase velocities are both involved, then `c` should be used for the former and `υ` for the latter. Phase velocity can also be written as `c = λ f`.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhaseVelocityUnit[1];
    }

    attribute phaseVelocity: PhaseVelocityValue[*] nonunique :> scalarQuantities;

    attribute def PhaseVelocityUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, durationPF); }
    }

    alias PhaseSpeedUnit for PhaseVelocityUnit;
    alias PhaseSpeedValue for PhaseVelocityValue;
    alias phaseSpeed for phaseVelocity;

    /* ISO-80000-3 item 3-23.2 group velocity, group speed */
    attribute groupVelocity: SpeedValue :> scalarQuantities {
        doc
        /*
         * source: item 3-23.2 group velocity, group speed
         * symbol(s): `c_g`, `v_g`, `(ν_g)`
         * application domain: generic
         * name: GroupVelocity (specializes Speed)
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 0
         * definition: speed with which the envelope of a wave propagates in space
         * remarks: The group velocity is given by: `c_g = (d ω)/ (dk)` where `ω` denotes the angular frequency (item 3-18) and `k` the angular repetency (item 3-22).
         */
    }

    alias groupSpeed for groupVelocity;

    /* ISO-80000-3 item 3-24 damping coefficient */
    attribute def DampingCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-24 damping coefficient
         * symbol(s): `δ`
         * application domain: generic
         * name: DampingCoefficient
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: inverse of the time constant (item 3-15) of an exponentially varying quantity
         * remarks: None.
         */
        attribute :>> num: Real;
        attribute :>> mRef: DampingCoefficientUnit[1];
    }

    attribute dampingCoefficient: DampingCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def DampingCoefficientUnit :> DerivedUnit {
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = durationPF; }
    }

    /* ISO-80000-3 item 3-25 logarithmic decrement */
    attribute def LogarithmicDecrementValue :> DimensionOneValue {
        doc
        /*
         * source: item 3-25 logarithmic decrement
         * symbol(s): `Λ`
         * application domain: generic
         * name: LogarithmicDecrement (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: product of damping coefficient (item 3-24) and period duration (item 3-14)
         * remarks: None.
         */
    }
    attribute logarithmicDecrement: LogarithmicDecrementValue :> scalarQuantities;

    /* ISO-80000-3 item 3-26.1 attenuation, extinction */
    attribute def AttenuationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-26.1 attenuation, extinction
         * symbol(s): `α`
         * application domain: generic
         * name: Attenuation
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: gradual decrease in magnitude (ISO 80000-2) of any kind of flux through a medium
         * remarks: If a quantity is a function of distance (item 3-1.8) expressed by: `f(x) prop e^(-α x)` where `x` denotes distance (item 3-1.8), then `α` denotes attenuation. The inverse of attenuation is called attenuation length.
         */
        attribute :>> num: Real;
        attribute :>> mRef: AttenuationUnit[1];
    }

    attribute attenuation: AttenuationValue[*] nonunique :> scalarQuantities;

    attribute def AttenuationUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    alias ExtinctionUnit for AttenuationUnit;
    alias ExtinctionValue for AttenuationValue;
    alias extinction for attenuation;

    /* ISO-80000-3 item 3-26.2 phase coefficient */
    attribute def PhaseCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-26.2 phase coefficient
         * symbol(s): `β`
         * application domain: generic
         * name: PhaseCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): rad/m, m^-1
         * tensor order: 0
         * definition: change of phase angle (item 3-7) with the length (item 3-1.1) along the path travelled by a plane wave
         * remarks: If a quantity is a function of distance expressed by: `f(x) prop cos(β(x-x_0))` where `x` denotes distance (item 3-1.8), then `β` denotes the phase coefficient.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PhaseCoefficientUnit[1];
    }

    attribute phaseCoefficient: PhaseCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def PhaseCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

    /* ISO-80000-3 item 3-26.3 propagation coefficient */
    attribute def PropagationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-26.3 propagation coefficient
         * symbol(s): `γ`
         * application domain: generic
         * name: PropagationCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: measure of the change of amplitude and phase angle (item 3-7) of a plane wave propagating in a given direction
         * remarks: The propagation coefficient is given by: `γ = α + iβ` where `α` denotes attenuation (item 3-26.1) and `β` the phase coefficient (item 3-26.2) of a plane wave.
         */
        attribute :>> num: Real;
        attribute :>> mRef: PropagationCoefficientUnit[1];
    }

    attribute propagationCoefficient: PropagationCoefficientValue[*] nonunique :> scalarQuantities;

    attribute def PropagationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = lengthPF; }
    }

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "isq_space_time.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package ISQSpaceTime {
    doc
    /*
     * International System of Quantities and Units
     * Generated on 2025-03-13T15:00:05Z from standard ISO-80000-3:2019 "Space and Time"
     * see also https://www.iso.org/standard/64974.html
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
    /* ISO-80000-3 item 3-1.1 length */
    /* See package ISQBase for the declarations of LengthValue and LengthUnit */
    /* ISO-80000-3 item 3-1.2 width, breadth */
    attribute def width : LengthValue {
        doc
        /*
         * source: item 3-1.2 width, breadth
         * symbol(s): `b`, `B`
         * application domain: generic
         * name: Width (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: minimum length of a straight line segment between two parallel straight lines (in two dimensions) or planes (in three dimensions) that enclose a given geometrical shape
         * remarks: This quantity is non-negative.
         */
    }
    alias breadth for width;
    /* ISO-80000-3 item 3-1.3 height, depth, altitude */
    attribute def height : LengthValue {
        doc
        /*
         * source: item 3-1.3 height, depth, altitude
         * symbol(s): `h`, `H`
         * application domain: generic
         * name: Height (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: minimum length of a straight line segment between a point and a reference line or reference surface
         * remarks: This quantity is usually signed. The sign expresses the position of the particular point with respect to the reference line or surface and is chosen by convention. The symbol `H` is often used to denote altitude, i.e. height above sea level.
         */
    }
    alias depth for height;
    alias altitude for height;
    /* ISO-80000-3 item 3-1.4 thickness */
    attribute def thickness : LengthValue {
        doc
        /*
         * source: item 3-1.4 thickness
         * symbol(s): `d`, `δ`
         * application domain: generic
         * name: Thickness (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: width (item 3-1.2)
         * remarks: This quantity is non-negative.
         */
    }
    /* ISO-80000-3 item 3-1.5 diameter */
    attribute def diameter : LengthValue {
        doc
        /*
         * source: item 3-1.5 diameter
         * symbol(s): `d`, `D`
         * application domain: generic
         * name: Diameter (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: width (item 3-1.2) of a circle, cylinder or sphere
         * remarks: This quantity is non-negative.
         */
    }
    /* ISO-80000-3 item 3-1.6 radius */
    attribute def radius : LengthValue {
        doc
        /*
         * source: item 3-1.6 radius
         * symbol(s): `r`, `R`
         * application domain: generic
         * name: Radius (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: half of a diameter (item 3-1.5)
         * remarks: This quantity is non-negative.
         */
    }
    /* ISO-80000-3 item 3-1.7 path length, arc length */
    attribute def pathLength : LengthValue {
        doc
        /*
         * source: item 3-1.7 path length, arc length
         * symbol(s): `s`
         * application domain: generic
         * name: PathLength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: length of a rectifiable curve between two of its points
         * remarks: The differential path length at a given point of a curve is: `ds = sqrt(dx^2 + dy^2 + dz^2)` where `x`, `y`, and `z` denote the Cartesian coordinates (ISO 80000-2) of the particular point. There are curves which are not rectifiable, for example fractal curves.
         */
    }
    alias arcLength for pathLength;
    /* ISO-80000-3 item 3-1.8 distance */
    attribute def distance : LengthValue {
        doc
        /*
         * source: item 3-1.8 distance
         * symbol(s): `d`, `r`
         * application domain: generic
         * name: Distance (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: shortest path length (item 3-1.7) between two points in a metric space
         * remarks: A metric space might be curved. An example of a curved metric space is the surface of the Earth. In this case, distances are measured along great circles. A metric is not necessarily Euclidean.
         */
    }
    /* ISO-80000-3 item 3-1.9 radial distance */
    attribute def radialDistance : LengthValue {
        doc
        /*
         * source: item 3-1.9 radial distance
         * symbol(s): `r_Q`, `ρ`
         * application domain: generic
         * name: RadialDistance (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: distance (item 3-1.8), where one point is located on an axis or within a closed non self-intersecting curve or surface
         * remarks: The subscript Q denotes the point from which the radial distance is measured. Examples of closed non self-intersecting curves are circles or ellipses. Examples of closed non self-intersecting surfaces are surfaces of spheres or egg-shaped objects.
         */
    }
    /* Spatial coordinate frames */
    attribute def Spatial3dCoordinateFrame :> '3dCoordinateFrame' {
        doc
        /*
         * Most general spatial 3D coordinate frame
         */
        attribute :>> isBound = true;
    }
    attribute def CartesianSpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc
        /*
         * Cartesian spatial 3D coordinate frame
         *
         * source: ISO 80000-2 item 2-17.1 Cartesian coordinates
         *
         * The components of a vector expressed on a Cartesian spatial coordinate frame are all LengthValues, and denoted with symbols `x`, `y`, `z`.
         *
         * Note 1: The Cartesian basis vectors `vec(e_x)`, `vec(e_y)` and `vec(e_z)` form an orthonormal right-handed coordinate frame.
         * Note 2: The measurement units for the 3 dimensions are typically the same, but may be different.
         */
        attribute xUnit : LengthUnit = mRefs#(1);
        attribute yUnit : LengthUnit = mRefs#(2);
        attribute zUnit : LengthUnit = mRefs#(3);
        attribute :>> mRefs : LengthUnit[3];
        attribute :>> isOrthogonal = true;
    }
    attribute def universalCartesianSpatial3dCoordinateFrame : CartesianSpatial3dCoordinateFrame[1] {
        doc
        /*
         * A singleton CartesianSpatial3dCoordinateFrame that can be used as a default universal Cartesian 3D coordinate frame.
         */
        attribute :>> mRefs default (SI::m, SI::m, SI::m) {
            doc
            /*
             * By default, the universalCartesianSpatial3dCoordinateFrame uses meters as the units on all three axes.
             */
        }
        attribute :>> transformation[0] {
            doc
            /*
             * The universalCartesianSpatial3dCoordinateFrame is the "top-level" coordinate frame, not nested in any other frame.
             */
        }
    }
    attribute def CylindricalSpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc
        /*
         * Cylindrical spatial 3D coordinate frame
         *
         * source: ISO 80000-2 item 2-17.2 cylindrical coordinates
         *
         * The components of a (position) vector to a point P in a cylindrical coordinate frame are:
         * - radialDistance (symbol `ρ`) defined by LengthValue, that is the radial distance from the cylinder axis to P
         * - azimuth (symbol `φ`) defined by AngularMeasure, that is the angle between the azimuth reference direction and the line segment
         * from the cylinder axis, in the plane that is orthogonal to the cylinder axis and intersects P
         * - z coordinate (symbol `z`) defined by LengthValue, the coordinate along the clyinder axis.
         *
         * Note 1: The basis vectors `vec(e_ρ)(φ)`, `vec(e_φ)(φ)` and `vec(e_z)` form an orthonormal right-handed coordinate frame, where
         * `vec(e_φ)` is tangent to the circular arc in the `φ` direction.
         * Note 2: In order to enable transformation to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned
         * with the `φ=0` direction in the cylindrical frame, and the `vec(e_z)` Cartesian basis vector is aligned with
         * the `vec(e_z)` cylindrical basis vector.
         * Note 3: If `z = 0`, then `ρ` and `φ` are polar coordinates in the XY-plane.
         * Note 4: See also https://en.wikipedia.org/wiki/Cylindrical_coordinate_system .
         */
        attribute radialDistanceUnit : LengthUnit;
        attribute azimuthUnit : AngularMeasureUnit;
        attribute zUnit : LengthUnit;
        attribute :>> mRefs = (radialDistanceUnit, azimuthUnit, zUnit);
        attribute :>> isOrthogonal = true;
    }
    attribute def SphericalSpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc
        /*
         * Spherical spatial 3D coordinate frame
         *
         * source: ISO 80000-2 item 2-17.3 spherical coordinates
         *
         * The components of a (position) vector to a point P specified in a spherical coordinate frame are:
         * - radialDistance (symbol `r`) defined by LengthValue, that is the distance from the origin to P
         * - inclination (symbol `θ`) defined by AngularMeasure, that is the angle between the zenith direction and the line segment from origin to P
         * - azimuth (symbol `φ`) defined by AngularMeasure, that is the angle between the azimuth reference direction and the line segment
         * from the origin to the orthogonal projection of P on the reference plane, normal to the zenith direction.
         *
         * Note 1: The basis vectors `vec(e_r)(θ,φ)`, `vec(e_θ)(θ,φ)` and `vec(e_φ)(φ)` form an orthonormal right-handed frame, where
         * `vec(e_θ)` and `vec(e_φ)` are tangent to the respective circular arcs in the `θ` and `φ` directions.
         * Note 2: In order to transform to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned
         * with the `θ=π/4` and `φ=0` direction in the spherical frame, and the `vec(e_z)` Cartesian basis vector is aligned
         * with the `θ=0` zenith direction in the spherical frame.
         * Note 3: If `θ = π/4`, then `ρ` and `φ` are polar coordinates in the XY-plane.
         * Note 4: See also https://en.wikipedia.org/wiki/Spherical_coordinate_system .
         */
        attribute radialDistanceUnit : LengthUnit;
        attribute inclinationUnit : AngularMeasureUnit;
        attribute azimuthUnit : AngularMeasureUnit;
        attribute :>> mRefs = (radialDistanceUnit, inclinationUnit, azimuthUnit);
        attribute :>> isOrthogonal = true;
    }
    attribute def PlanetarySpatial3dCoordinateFrame :> Spatial3dCoordinateFrame {
        doc
        /*
         * Planetary spatial 3D coordinate frame
         *
         * A planetary spatial 3D coordinate frame is a generalization for any planet of the geographic coordinate frame and geocentric coordinate
         * for Earth. In such coordinate frames, typically the origin is located at the planet's centre of gravity, and the surface of the planet
         * is approximated by a reference ellipsoid centred on the origin, with its major axes oriented along the south to north pole vector and
         * the equatorial plane.
         *
         * The components of a (position) vector to a point P specified in a planetary coordinate frame are:
         * - latitude (symbol `lat` or `φ`) defined by AngularMeasure, that is the angle between the equatorial plane and the vector from
         *   the origin to P, similar to the inclination in a spherical spatial coordinate frame. Typically, the zero reference latitude is chosen
         *   for positions in the equatorial plane, with positive latitude for positions in the northern hemisphere and negative latitude for positions
         *   in the southern hemisphere.
         * - longitude (symbol `long` or `λ`) defined by AngularMeasure, that is the angle between a reference meridian and the meridian
         *   passing through P, similar to the azimuth of a spherical spatial coordinate frame. The convention is to connotate positive longitude
         *   with eastward direction and negative longitude with westward direction. The reference meridian for `long=0` is chosen to pass
         *   through a particular feature of the planet, e.g., for Earth typically the position of the British Royal Observatory in Greenwich, UK.
         * - altitude (symbol `h`) defined by LengthValue, that is the distance between P and the reference ellipsoid
         *   in the normal direction to the ellipsoid. Positive altitude specifies a position above the reference ellipsoid surface,
         *   while a negative value specifies a position below.
         *
         * Note 1: The reference meridian is also called prime meridian.
         * Note 2: The basis vectors `vec(e_φ)(φ)`, `vec(e_λ)(λ)` and `vec(e_h)(φ,λ)` form an orthonormal right-handed frame, where
         * `vec(e_φ)` and `vec(e_λ)` are tangent to the reference ellipsoid in the respective latitude and longitude directions,
         * and `vec(e_h)` is normal to the reference ellipsoid.
         * Note 3: In order to transform to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned
         * with the `φ=0` and `λ=0` direction in the planetary frame, and the `vec(e_z)` Cartesian basis vector is aligned
         * with the `λ=π/2` (north pole) direction in the planetary frame.
         * Note 4: See also https://en.wikipedia.org/wiki/Planetary_coordinate_system .
         */
        attribute latitudeUnit : AngularMeasureUnit;
        attribute longitudeUnit : AngularMeasureUnit;
        attribute altitudeUnit : LengthUnit;
        attribute :>> mRefs = (longitudeUnit, latitudeUnit, altitudeUnit);
        attribute :>> isOrthogonal = true;
    }
    /* ISO-80000-3 item 3-1.10 position vector */
    attribute def Position3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-1.10 position vector
         * symbol(s): `vec(r)`
         * application domain: generic
         * name: PositionVector
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity from the origin of a coordinate system to a point in space
         * remarks: Position vectors are so-called bounded vectors, i.e. their magnitude (ISO 80000-2) and direction depend on the particular coordinate system used.
         */
        attribute :>> isBound = true;
        attribute :>> mRef : Spatial3dCoordinateFrame[1];
    }
    attribute def position3dVector : Position3dVector;
    attribute def CartesianPosition3dVector :> Position3dVector {
        attribute x : LengthValue[mRef.mRefs#(1)] = num#(1);
        attribute y : LengthValue[mRef.mRefs#(2)] = num#(2);
        attribute z : LengthValue[mRef.mRefs#(3)] = num#(3);
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame[1];
    }
    attribute def cartesianPosition3dVector : CartesianPosition3dVector;
    attribute def CylindricalPosition3dVector :> Position3dVector {
        attribute <'ρ'> radialDistance : LengthValue[mRef.mRefs#(1)] = num#(1);
        attribute <'φ'> azimuth : AngularMeasureUnit[mRef.mRefs#(2)] = num#(2);
        attribute <h> height : LengthValue[mRef.mRefs#(3)] = num#(3);
        attribute :>> mRef : CylindricalSpatial3dCoordinateFrame[1];
    }
    attribute def cylindricalPosition3dVector : CylindricalPosition3dVector;
    attribute def SphericalPosition3dVector :> Position3dVector {
        attribute <r> radialDistance : LengthValue[mRef.mRefs#(1)] = num#(1);
        attribute <'θ'> inclination : AngularMeasureUnit[mRef.mRefs#(2)] = num#(2);
        attribute <'φ'> azimuth : AngularMeasureUnit[mRef.mRefs#(3)] = num#(3);
        attribute :>> mRef : SphericalSpatial3dCoordinateFrame[1];
    }
    attribute def sphericalPosition3dVector : SphericalPosition3dVector;
    attribute def PlanetaryPosition3dVector :> Position3dVector {
        attribute <lat> latitude : AngularMeasureUnit[mRef.mRefs#(1)] = num#(1);
        attribute <long> longitude : AngularMeasureUnit[mRef.mRefs#(2)] = num#(2);
        attribute <h> altitude : LengthValue[mRef.mRefs#(3)] = num#(3);
        attribute :>> mRef : PlanetarySpatial3dCoordinateFrame[1];
    }
    attribute def planetaryPosition3dVector : PlanetaryPosition3dVector;
    /* ISO-80000-3 item 3-1.11 displacement */
    attribute def Displacement3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-1.11 displacement
         * symbol(s): `vec(Δr)`
         * application domain: generic
         * name: Displacement
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity between any two points in space
         * remarks: Displacement vectors are so-called free vectors, i.e. their magnitude (ISO 80000-2) and direction do not depend on a particular coordinate system. The magnitude of this vector is also called displacement.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : Spatial3dCoordinateFrame[1];
    }
    attribute def displacement3dVector : Displacement3dVector;
    attribute def CartesianDisplacement3dVector :> Displacement3dVector {
        attribute x : LengthValue[mRef.mRefs#(1)] = num#(1);
        attribute y : LengthValue[mRef.mRefs#(2)] = num#(2);
        attribute z : LengthValue[mRef.mRefs#(3)] = num#(3);
        attribute :>> mRef : CartesianSpatial3dCoordinateFrame[1];
    }
    attribute def cartesianDisplacement3dVector : CartesianDisplacement3dVector;
    attribute def CylindricalDisplacement3dVector :> Displacement3dVector {
        attribute <'ρ'> radialDistance : LengthValue[mRef.mRefs#(1)] = num#(1);
        attribute <'φ'> azimuth : AngularMeasureUnit[mRef.mRefs#(2)] = num#(2);
        attribute <h> height : LengthValue[mRef.mRefs#(3)] = num#(3);
        attribute :>> mRef : CylindricalSpatial3dCoordinateFrame[1];
    }
    attribute def cylindricalDisplacement3dVector : CylindricalDisplacement3dVector;
    attribute def SphericalDisplacement3dVector :> Displacement3dVector {
        attribute <r> radialDistance : LengthValue[mRef.mRefs#(1)] = num#(1);
        attribute <'θ'> inclination : AngularMeasureUnit[mRef.mRefs#(2)] = num#(2);
        attribute <'φ'> azimuth : AngularMeasureUnit[mRef.mRefs#(3)] = num#(3);
        attribute :>> mRef : SphericalSpatial3dCoordinateFrame[1];
    }
    attribute def sphericalDisplacement3dVector : SphericalDisplacement3dVector;
    /* ISO-80000-3 item 3-1.12 radius of curvature */
    attribute def radiusOfCurvature : LengthValue {
        doc
        /*
         * source: item 3-1.12 radius of curvature
         * symbol(s): `ρ`
         * application domain: generic
         * name: RadiusOfCurvature (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: radius (item 3-1.6) of the osculating circle of a planar curve at a particular point of the curve
         * remarks: The radius of curvature is only defined for curves which are at least twice continuously differentiable.
         */
    }
    /* ISO-80000-3 item 3-2 curvature */
    attribute def CurvatureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-2 curvature
         * symbol(s): `κ`
         * application domain: generic
         * name: Curvature
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: inverse of the radius of curvature (item 3-1.12)
         * remarks: The curvature is given by: `κ = 1/ρ` where `ρ` denotes the radius of curvature (item 3-1.12).
         */
        attribute :>> num : Real;
        attribute :>> mRef : CurvatureUnit[1];
    }
    attribute def curvature : CurvatureValue[*] nonunique;
    attribute def CurvatureUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-3 item 3-3 area */
    attribute def AreaValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-3 area
         * symbol(s): `A`, `S`
         * application domain: generic
         * name: Area
         * quantity dimension: L^2
         * measurement unit(s): m^2
         * tensor order: 0
         * definition: extent of a two-dimensional geometrical shape
         * remarks: The surface element at a given point of a surface is given by: `dA = g du dv` where `u` and `v` denote the Gaussian surface coordinates and `g` denotes the determinant of the metric tensor (ISO 80000-2) at the particular point. The symbol `dσ` is also used for the surface element.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AreaUnit[1];
    }
    attribute def area : AreaValue[*] nonunique;
    attribute def AreaUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-3 item 3-4 volume */
    attribute def VolumeValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-4 volume
         * symbol(s): `V`, `(S)`
         * application domain: generic
         * name: Volume
         * quantity dimension: L^3
         * measurement unit(s): m^3
         * tensor order: 0
         * definition: extent of a three-dimensional geometrical shape
         * remarks: The volume element in Euclidean space is given by: `dV = dx dy dz` where `dx`, `dy`, and `dz` denote the differentials of the Cartesian coordinates (ISO 80000-2). The symbol `dτ` is also used for the volume element.
         */
        attribute :>> num : Real;
        attribute :>> mRef : VolumeUnit[1];
    }
    attribute def volume : VolumeValue[*] nonunique;
    attribute def VolumeUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 3;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-3 item 3-5 angular measure, plane angle */
    attribute def AngularMeasureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-5 angular measure, plane angle
         * symbol(s): `α`, `β`, `γ`
         * application domain: generic
         * name: AngularMeasure
         * quantity dimension: 1
         * measurement unit(s): rad, 1
         * tensor order: 0
         * definition: measure of a geometric figure, called plane angle, formed by two rays, called the sides of the plane angle, emanating from a common point, called the vertex of the plane angle
         * remarks: The angular measure is given by: `α = s/r` where `s` denotes the arc length (item 3-1.7) of the included arc of a circle, centred at the vertex of the plane angle, and `r` the radius (item 3-1.6) of that circle. Other symbols are also used.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularMeasureUnit[1];
    }
    attribute def angularMeasure : AngularMeasureValue[*] nonunique;
    attribute def AngularMeasureUnit :> DimensionOneUnit {
    }
    alias PlaneAngleUnit for AngularMeasureUnit;
    alias PlaneAngleValue for AngularMeasureValue;
    alias planeAngle for angularMeasure;
    /* ISO-80000-3 item 3-6 rotational displacement, angular displacement */
    attribute def rotationalDisplacement : AngularMeasureValue {
        doc
        /*
         * source: item 3-6 rotational displacement, angular displacement
         * symbol(s): `ϑ`, `φ`
         * application domain: generic
         * name: RotationalDisplacement (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): rad, 1
         * tensor order: 0
         * definition: quotient of the traversed circular path length (item 3-1.7) of a point in space during a rotation and its distance (item 3-1.8) from the axis or centre of rotation
         * remarks: The rotational displacement is given by: `φ = s/r` where `s` denotes the traversed path length (item 3-1.7) along the periphery of a circle, centred at the vertex of the plane angle, and `r` the radius (item 3-1.6) of that circle. The rotational displacement is signed. The sign denotes the direction of rotation and is chosen by convention. Other symbols are also used.
         */
    }
    alias angularDisplacement for rotationalDisplacement;
    /* ISO-80000-3 item 3-7 phase angle */
    attribute def phaseAngle : AngularMeasureValue {
        doc
        /*
         * source: item 3-7 phase angle
         * symbol(s): `φ`, `ϕ`
         * application domain: generic
         * name: PhaseAngle (specializes AngularMeasure)
         * quantity dimension: 1
         * measurement unit(s): rad, 1
         * tensor order: 0
         * definition: angular measure (item 3-5) between the positive real axis and the radius of the polar representation of the complex number in the complex plane
         * remarks: The phase angle (often imprecisely referred to as the "phase") is the argument of a complex number. Other symbols are also used.
         */
    }
    /* ISO-80000-3 item 3-8 solid angular measure */
    attribute def SolidAngularMeasureValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-8 solid angular measure
         * symbol(s): `Ω`
         * application domain: generic
         * name: SolidAngularMeasure
         * quantity dimension: 1
         * measurement unit(s): sr, 1
         * tensor order: 0
         * definition: measure of a conical geometric figure, called solid angle, formed by all rays, originating from a common point, called the vertex of the solid angle, and passing through the points of a closed, non-self-intersecting curve in space considered as the border of a surface
         * remarks: The differential solid angular measure expressed in spherical coordinates (ISO 80000-2) is given by: `dΩ = A/r^2 * sin(θ * dθ * dφ)` where `A` is area, `r` is radius, `θ` and `φ` are spherical coordinates.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SolidAngularMeasureUnit[1];
    }
    attribute def solidAngularMeasure : SolidAngularMeasureValue[*] nonunique;
    attribute def SolidAngularMeasureUnit :> DimensionOneUnit {
    }
    /* ISO-80000-3 item 3-9 duration, time */
    /* See package ISQBase for the declarations of DurationValue and DurationUnit */
    alias TimeUnit for DurationUnit;
    alias TimeValue for DurationValue;
    alias time for duration;
    /* ISO-80000-3 item 3-10.1 velocity */
    attribute def CartesianVelocity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-10.1 velocity
         * symbol(s): `vec(v)`, `u,v,w`
         * application domain: generic
         * name: Velocity
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m/s, m*s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of a position vector (item 3-1.10)
         * remarks: The velocity vector is given by: `vec(v) = (d vec(r)) / (dt)` where `vec(r)` denotes the position vector (item 3-1.10) and `t` the duration (item 3-9). When the general symbol `vec(v)` is not used for the velocity, the symbols `u`, `v`, `w` may be used for the components (ISO 80000-2) of the velocity.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianVelocity3dCoordinateFrame[1];
    }
    attribute def cartesianVelocity3dVector : CartesianVelocity3dVector;
    attribute def CartesianVelocity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : SpeedUnit[3];
    }
    /* ISO-80000-3 item 3-10.2 speed */
    attribute def SpeedValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-10.2 speed
         * symbol(s): `v`
         * application domain: generic
         * name: Speed
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m/s, m*s^-1
         * tensor order: 0
         * definition: magnitude (ISO 80000-2) of the velocity (item 3-10.1)
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : SpeedUnit[1];
    }
    attribute def speed : SpeedValue[*] nonunique;
    attribute def SpeedUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    /* ISO-80000-3 item 3-11 acceleration */
    attribute def AccelerationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-11 acceleration (magnitude)
         * symbol(s): `a`
         * application domain: generic
         * name: Acceleration
         * quantity dimension: L^1*T^-2
         * measurement unit(s): m*s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity giving the rate of change of velocity (item 3-10)
         * remarks: The acceleration vector is given by: `vec(a) = (d vec(v))/(dt)` where `vec(v)` denotes the velocity (item 3-10.1) and `t` the duration (item 3-9). The magnitude (ISO 80000-2) of the acceleration of free fall is usually denoted by `g`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AccelerationUnit[1];
    }
    attribute def acceleration : AccelerationValue[*] nonunique;
    attribute def AccelerationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    attribute def CartesianAcceleration3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-11 acceleration (vector)
         * symbol(s): `vec(a)`
         * application domain: generic
         * name: Acceleration
         * quantity dimension: L^1*T^-2
         * measurement unit(s): m*s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of velocity (item 3-10)
         * remarks: The acceleration vector is given by: `vec(a) = (d vec(v))/(dt)` where `vec(v)` denotes the velocity (item 3-10.1) and `t` the duration (item 3-9). The magnitude (ISO 80000-2) of the acceleration of free fall is usually denoted by `g`.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianAcceleration3dCoordinateFrame[1];
    }
    attribute def cartesianAcceleration3dVector : CartesianAcceleration3dVector;
    attribute def CartesianAcceleration3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : AccelerationUnit[3];
    }
    /* ISO-80000-3 item 3-12 angular velocity */
    attribute def AngularVelocityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-12 angular velocity (magnitude)
         * symbol(s): `ω`
         * application domain: generic
         * name: AngularVelocity
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity giving the rate of change of the rotational displacement (item 3-6) as its magnitude (ISO 80000-2) and with a direction equal to the direction of the axis of rotation
         * remarks: The angular velocity vector is given by: `vec(ω) = (d φ) / (dt) vec(u)` where `φ` denotes the angular displacement (item 3-6), `t` the duration (item 3-9), and `vec(u)` the unit vector (ISO 80000-2) along the axis of rotation in the direction for which the rotation corresponds to a right-hand spiral.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularVelocityUnit[1];
    }
    attribute def angularVelocity : AngularVelocityValue[*] nonunique;
    attribute def AngularVelocityUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = durationPF;
        }
    }
    attribute def CartesianAngularVelocity3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-12 angular velocity (vector)
         * symbol(s): `vec(ω)`
         * application domain: generic
         * name: AngularVelocity
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of the rotational displacement (item 3-6) as its magnitude (ISO 80000-2) and with a direction equal to the direction of the axis of rotation
         * remarks: The angular velocity vector is given by: `vec(ω) = (d φ) / (dt) vec(u)` where `φ` denotes the angular displacement (item 3-6), `t` the duration (item 3-9), and `vec(u)` the unit vector (ISO 80000-2) along the axis of rotation in the direction for which the rotation corresponds to a right-hand spiral.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianAngularVelocity3dCoordinateFrame[1];
    }
    attribute def cartesianAngularVelocity3dVector : CartesianAngularVelocity3dVector;
    attribute def CartesianAngularVelocity3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : AngularVelocityUnit[3];
    }
    /* ISO-80000-3 item 3-13 angular acceleration */
    attribute def AngularAccelerationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-13 angular acceleration (magnitude)
         * symbol(s): `α`
         * application domain: generic
         * name: AngularAcceleration
         * quantity dimension: T^-2
         * measurement unit(s): rad*s^-2, s^-2
         * tensor order: 0
         * definition: vector (ISO 80000-2) quantity giving the rate of change of angular velocity (item 3-12)
         * remarks: The angular acceleration vector is given by: `vec α = (d vec(ω))/(dt)` Where `vec(ω)` denotes the angular velocity (item 3-12) and `t` the duration (item 3-9).
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularAccelerationUnit[1];
    }
    attribute def angularAcceleration : AngularAccelerationValue[*] nonunique;
    attribute def AngularAccelerationUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = durationPF;
        }
    }
    attribute def CartesianAngularAcceleration3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-13 angular acceleration (vector)
         * symbol(s): `vec(α)`
         * application domain: generic
         * name: AngularAcceleration
         * quantity dimension: T^-2
         * measurement unit(s): rad*s^-2, s^-2
         * tensor order: 1
         * definition: vector (ISO 80000-2) quantity giving the rate of change of angular velocity (item 3-12)
         * remarks: The angular acceleration vector is given by: `vec α = (d vec(ω))/(dt)` Where `vec(ω)` denotes the angular velocity (item 3-12) and `t` the duration (item 3-9).
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianAngularAcceleration3dCoordinateFrame[1];
    }
    attribute def cartesianAngularAcceleration3dVector : CartesianAngularAcceleration3dVector;
    attribute def CartesianAngularAcceleration3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : AngularAccelerationUnit[3];
    }
    /* ISO-80000-3 item 3-14 period duration, period */
    attribute def periodDuration : DurationValue {
        doc
        /*
         * source: item 3-14 period duration, period
         * symbol(s): `T`
         * application domain: generic
         * name: PeriodDuration (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: duration (item 3-9) of one cycle of a periodic event
         * remarks: A periodic event is an event that occurs regularly with a fixed time interval.
         */
    }
    alias period for periodDuration;
    /* ISO-80000-3 item 3-15 time constant */
    attribute def timeConstant : DurationValue {
        doc
        /*
         * source: item 3-15 time constant
         * symbol(s): `τ`, `T`
         * application domain: generic
         * name: TimeConstant (specializes Duration)
         * quantity dimension: T^1
         * measurement unit(s): s
         * tensor order: 0
         * definition: parameter characterizing the response to a step input of a first-order, linear time-invariant system
         * remarks: If a quantity is a function of the duration (item 3-9) expressed by: `F(t) prop e^(-t/τ)` where `t` denotes the duration (item 3-9), then `τ` denotes the time constant. Here the time constant `τ` applies to an exponentially decaying quantity.
         */
    }
    /* ISO-80000-3 item 3-16 rotation */
    attribute def rotation : CountValue {
        doc
        /*
         * source: item 3-16 rotation
         * symbol(s): `N`
         * application domain: generic
         * name: Rotation (specializes Count)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: number of revolutions
         * remarks: `N` is the number (not necessarily an integer) of revolutions, for example, of a rotating body about a given axis. Its value is given by: `N = φ/(2 π)` where `φ` denotes the measure of rotational displacement (item 3-6).
         */
    }
    /* ISO-80000-3 item 3-17.1 frequency */
    attribute def FrequencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-17.1 frequency
         * symbol(s): `f`, `ν`
         * application domain: generic
         * name: Frequency
         * quantity dimension: T^-1
         * measurement unit(s): Hz, s^-1
         * tensor order: 0
         * definition: inverse of period duration (item 3-14)
         * remarks: The frequency is given by: `f = 1/T` where `T` denotes the period duration (item 3-14).
         */
        attribute :>> num : Real;
        attribute :>> mRef : FrequencyUnit[1];
    }
    attribute def frequency : FrequencyValue[*] nonunique;
    attribute def FrequencyUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = durationPF;
        }
    }
    /* ISO-80000-3 item 3-17.2 rotational frequency */
    attribute def rotationalFrequency : FrequencyValue {
        doc
        /*
         * source: item 3-17.2 rotational frequency
         * symbol(s): `n`
         * application domain: generic
         * name: RotationalFrequency (specializes Frequency)
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: duration (item 3-9) of one cycle of a periodic event
         * remarks: The rotational frequency is given by: `n = (dN) / (dt)` where `N` denotes the rotation (item 3-16) and `t` is the duration (item 3-9).
         */
    }
    /* ISO-80000-3 item 3-18 angular frequency */
    attribute def AngularFrequencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-18 angular frequency
         * symbol(s): `ω`
         * application domain: generic
         * name: AngularFrequency
         * quantity dimension: T^-1
         * measurement unit(s): rad*s^-1, s^-1
         * tensor order: 0
         * definition: rate of change of the phase angle (item 3-7)
         * remarks: The angular frequency is given by: `ω = 2 π f` where `f` denotes the frequency (item 3-17.1).
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularFrequencyUnit[1];
    }
    attribute def angularFrequency : AngularFrequencyValue[*] nonunique;
    attribute def AngularFrequencyUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = durationPF;
        }
    }
    /* ISO-80000-3 item 3-19 wavelength */
    attribute def wavelength : LengthValue {
        doc
        /*
         * source: item 3-19 wavelength
         * symbol(s): `λ`
         * application domain: generic
         * name: Wavelength (specializes Length)
         * quantity dimension: L^1
         * measurement unit(s): m
         * tensor order: 0
         * definition: length (item 3-1.1) of the repetition interval of a wave
         * remarks: None.
         */
    }
    /* ISO-80000-3 item 3-20 repetency, wavenumber */
    attribute def RepetencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-20 repetency, wavenumber
         * symbol(s): `σ`, `ṽ`
         * application domain: generic
         * name: Repetency
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: inverse of the wavelength (item 3-19)
         * remarks: The repetency is given by: `σ = 1 / λ` where `λ` denotes the wavelength (item 3-19).
         */
        attribute :>> num : Real;
        attribute :>> mRef : RepetencyUnit[1];
    }
    attribute def repetency : RepetencyValue[*] nonunique;
    attribute def RepetencyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    alias WavenumberUnit for RepetencyUnit;
    alias WavenumberValue for RepetencyValue;
    alias wavenumber for repetency;
    /* ISO-80000-3 item 3-21 wave vector */
    attribute def CartesianWave3dVector :> '3dVectorQuantityValue' {
        doc
        /*
         * source: item 3-21 wave vector
         * symbol(s): `vec(k)`
         * application domain: generic
         * name: WaveVector
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 1
         * definition: vector normal to the surfaces of constant phase angle (item 3-7) of a wave, with the magnitude (ISO 80000-2) of repetency (item 3-20)
         * remarks: None.
         */
        attribute :>> isBound = false;
        attribute :>> mRef : CartesianWaveVector3dCoordinateFrame[1];
    }
    attribute def cartesianWave3dVector : CartesianWave3dVector;
    attribute def CartesianWaveVector3dCoordinateFrame :> '3dCoordinateFrame' {
        attribute :>> isBound = false;
        attribute :>> isOrthogonal = true;
        attribute :>> mRefs : RepetencyUnit[3];
    }
    /* ISO-80000-3 item 3-22 angular repetency, angular wavenumber */
    attribute def AngularRepetencyValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-22 angular repetency, angular wavenumber
         * symbol(s): `k`
         * application domain: generic
         * name: AngularRepetency
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: magnitude (ISO 80000-2) of the wave vector (item 3-21)
         * remarks: The angular repetency is given by: `κ = (2 π)/λ` where `λ` denotes the wavelength (item 3-19).
         */
        attribute :>> num : Real;
        attribute :>> mRef : AngularRepetencyUnit[1];
    }
    attribute def angularRepetency : AngularRepetencyValue[*] nonunique;
    attribute def AngularRepetencyUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    alias AngularWavenumberUnit for AngularRepetencyUnit;
    alias AngularWavenumberValue for AngularRepetencyValue;
    alias angularWavenumber for angularRepetency;
    /* ISO-80000-3 item 3-23.1 phase velocity, phase speed */
    attribute def PhaseVelocityValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-23.1 phase velocity, phase speed
         * symbol(s): `c`, `v`, `(ν)`, `c_φ`, `v_φ`, `(ν_φ)`
         * application domain: generic
         * name: PhaseVelocity
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 0
         * definition: speed with which the phase angle (item 3-7) of a wave propagates in space
         * remarks: The phase velocity is given by: `c = ω/κ` where `ω` denotes the angular frequency (item 3-18) and `k` the angular repetency (item 3-22). If phase velocities of electromagnetic waves and other phase velocities are both involved, then `c` should be used for the former and `υ` for the latter. Phase velocity can also be written as `c = λ f`.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PhaseVelocityUnit[1];
    }
    attribute def phaseVelocity : PhaseVelocityValue[*] nonunique;
    attribute def PhaseVelocityUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = (lengthPF, durationPF);
        }
    }
    alias PhaseSpeedUnit for PhaseVelocityUnit;
    alias PhaseSpeedValue for PhaseVelocityValue;
    alias phaseSpeed for phaseVelocity;
    /* ISO-80000-3 item 3-23.2 group velocity, group speed */
    attribute def groupVelocity : SpeedValue {
        doc
        /*
         * source: item 3-23.2 group velocity, group speed
         * symbol(s): `c_g`, `v_g`, `(ν_g)`
         * application domain: generic
         * name: GroupVelocity (specializes Speed)
         * quantity dimension: L^1*T^-1
         * measurement unit(s): m*s^-1
         * tensor order: 0
         * definition: speed with which the envelope of a wave propagates in space
         * remarks: The group velocity is given by: `c_g = (d ω)/ (dk)` where `ω` denotes the angular frequency (item 3-18) and `k` the angular repetency (item 3-22).
         */
    }
    alias groupSpeed for groupVelocity;
    /* ISO-80000-3 item 3-24 damping coefficient */
    attribute def DampingCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-24 damping coefficient
         * symbol(s): `δ`
         * application domain: generic
         * name: DampingCoefficient
         * quantity dimension: T^-1
         * measurement unit(s): s^-1
         * tensor order: 0
         * definition: inverse of the time constant (item 3-15) of an exponentially varying quantity
         * remarks: None.
         */
        attribute :>> num : Real;
        attribute :>> mRef : DampingCoefficientUnit[1];
    }
    attribute def dampingCoefficient : DampingCoefficientValue[*] nonunique;
    attribute def DampingCoefficientUnit :> DerivedUnit {
        private attribute durationPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.T;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = durationPF;
        }
    }
    /* ISO-80000-3 item 3-25 logarithmic decrement */
    attribute def LogarithmicDecrementValue :> DimensionOneValue {
        doc
        /*
         * source: item 3-25 logarithmic decrement
         * symbol(s): `Λ`
         * application domain: generic
         * name: LogarithmicDecrement (specializes DimensionOneQuantity)
         * quantity dimension: 1
         * measurement unit(s): 1
         * tensor order: 0
         * definition: product of damping coefficient (item 3-24) and period duration (item 3-14)
         * remarks: None.
         */
    }
    attribute def logarithmicDecrement : LogarithmicDecrementValue;
    /* ISO-80000-3 item 3-26.1 attenuation, extinction */
    attribute def AttenuationValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-26.1 attenuation, extinction
         * symbol(s): `α`
         * application domain: generic
         * name: Attenuation
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: gradual decrease in magnitude (ISO 80000-2) of any kind of flux through a medium
         * remarks: If a quantity is a function of distance (item 3-1.8) expressed by: `f(x) prop e^(-α x)` where `x` denotes distance (item 3-1.8), then `α` denotes attenuation. The inverse of attenuation is called attenuation length.
         */
        attribute :>> num : Real;
        attribute :>> mRef : AttenuationUnit[1];
    }
    attribute def attenuation : AttenuationValue[*] nonunique;
    attribute def AttenuationUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    alias ExtinctionUnit for AttenuationUnit;
    alias ExtinctionValue for AttenuationValue;
    alias extinction for attenuation;
    /* ISO-80000-3 item 3-26.2 phase coefficient */
    attribute def PhaseCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-26.2 phase coefficient
         * symbol(s): `β`
         * application domain: generic
         * name: PhaseCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): rad/m, m^-1
         * tensor order: 0
         * definition: change of phase angle (item 3-7) with the length (item 3-1.1) along the path travelled by a plane wave
         * remarks: If a quantity is a function of distance expressed by: `f(x) prop cos(β(x-x_0))` where `x` denotes distance (item 3-1.8), then `β` denotes the phase coefficient.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PhaseCoefficientUnit[1];
    }
    attribute def phaseCoefficient : PhaseCoefficientValue[*] nonunique;
    attribute def PhaseCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
    /* ISO-80000-3 item 3-26.3 propagation coefficient */
    attribute def PropagationCoefficientValue :> ScalarQuantityValue {
        doc
        /*
         * source: item 3-26.3 propagation coefficient
         * symbol(s): `γ`
         * application domain: generic
         * name: PropagationCoefficient
         * quantity dimension: L^-1
         * measurement unit(s): m^-1
         * tensor order: 0
         * definition: measure of the change of amplitude and phase angle (item 3-7) of a plane wave propagating in a given direction
         * remarks: The propagation coefficient is given by: `γ = α + iβ` where `α` denotes attenuation (item 3-26.1) and `β` the phase coefficient (item 3-26.2) of a plane wave.
         */
        attribute :>> num : Real;
        attribute :>> mRef : PropagationCoefficientUnit[1];
    }
    attribute def propagationCoefficient : PropagationCoefficientValue[*] nonunique;
    attribute def PropagationCoefficientUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor[1] {
            attribute :>> quantity = isq.L;
            attribute :>> exponent = -1;
        }
        attribute :>> quantityDimension {
            attribute :>> quantityPowerFactors = lengthPF;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 784) (line 15) (column 20) (len 18)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 784) (line 15) (column 20) (len 12))) (segment 1 (token "Real") (name "Real") (separator colon-colon) (span (offset 798) (line 15) (column 34) (len 4)))))
    (reference r1 (scope relative) (span (offset 823) (line 16) (column 20) (len 10)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 823) (line 16) (column 20) (len 10)))))
    (reference r2 (scope relative) (span (offset 857) (line 17) (column 20) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 857) (line 17) (column 20) (len 21)))))
    (reference r3 (scope relative) (span (offset 902) (line 18) (column 20) (len 7)) (segments (segment 0 (token "ISQBase") (name "ISQBase") (separator none) (span (offset 902) (line 18) (column 20) (len 7)))))
    (reference r4 (scope relative) (span (offset 1106) (line 24) (column 22) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 1106) (line 24) (column 22) (len 11)))))
    (reference r5 (scope relative) (span (offset 1702) (line 39) (column 23) (len 5)) (segments (segment 0 (token "width") (name "width") (separator none) (span (offset 1702) (line 39) (column 23) (len 5)))))
    (reference r6 (scope relative) (span (offset 1789) (line 42) (column 23) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 1789) (line 42) (column 23) (len 11)))))
    (reference r7 (scope relative) (span (offset 2535) (line 57) (column 21) (len 6)) (segments (segment 0 (token "height") (name "height") (separator none) (span (offset 2535) (line 57) (column 21) (len 6)))))
    (reference r8 (scope relative) (span (offset 2567) (line 59) (column 24) (len 6)) (segments (segment 0 (token "height") (name "height") (separator none) (span (offset 2567) (line 59) (column 24) (len 6)))))
    (reference r9 (scope relative) (span (offset 2644) (line 62) (column 26) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 2644) (line 62) (column 26) (len 11)))))
    (reference r10 (scope relative) (span (offset 3134) (line 78) (column 25) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 3134) (line 78) (column 25) (len 11)))))
    (reference r11 (scope relative) (span (offset 3649) (line 94) (column 23) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 3649) (line 94) (column 23) (len 11)))))
    (reference r12 (scope relative) (span (offset 4162) (line 110) (column 27) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 4162) (line 110) (column 27) (len 11)))))
    (reference r13 (scope relative) (span (offset 4886) (line 125) (column 25) (len 10)) (segments (segment 0 (token "pathLength") (name "pathLength") (separator none) (span (offset 4886) (line 125) (column 25) (len 10)))))
    (reference r14 (scope relative) (span (offset 4965) (line 128) (column 25) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 4965) (line 128) (column 25) (len 11)))))
    (reference r15 (scope relative) (span (offset 5680) (line 144) (column 31) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 5680) (line 144) (column 31) (len 11)))))
    (reference r16 (scope relative) (span (offset 6521) (line 161) (column 47) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 6521) (line 161) (column 47) (len 19)))))
    (reference r17 (scope relative) (span (offset 6652) (line 166) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 6652) (line 166) (column 23) (len 7)))))
    (reference r18 (scope relative) (span (offset 6730) (line 169) (column 56) (len 24)) (segments (segment 0 (token "Spatial3dCoordinateFrame") (name "Spatial3dCoordinateFrame") (separator none) (span (offset 6730) (line 169) (column 56) (len 24)))))
    (reference r19 (scope relative) (span (offset 7359) (line 181) (column 27) (len 10)) (segments (segment 0 (token "LengthUnit") (name "LengthUnit") (separator none) (span (offset 7359) (line 181) (column 27) (len 10)))))
    (reference r20 (scope relative) (span (offset 7372) (line 181) (column 40) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 7372) (line 181) (column 40) (len 5)))))
    (reference r21 (scope relative) (span (offset 7409) (line 182) (column 27) (len 10)) (segments (segment 0 (token "LengthUnit") (name "LengthUnit") (separator none) (span (offset 7409) (line 182) (column 27) (len 10)))))
    (reference r22 (scope relative) (span (offset 7422) (line 182) (column 40) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 7422) (line 182) (column 40) (len 5)))))
    (reference r23 (scope relative) (span (offset 7459) (line 183) (column 27) (len 10)) (segments (segment 0 (token "LengthUnit") (name "LengthUnit") (separator none) (span (offset 7459) (line 183) (column 27) (len 10)))))
    (reference r24 (scope relative) (span (offset 7472) (line 183) (column 40) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 7472) (line 183) (column 40) (len 5)))))
    (reference r25 (scope relative) (span (offset 7513) (line 184) (column 31) (len 10)) (segments (segment 0 (token "LengthUnit") (name "LengthUnit") (separator none) (span (offset 7513) (line 184) (column 31) (len 10)))))
    (reference r26 (scope relative) (span (offset 7505) (line 184) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 7505) (line 184) (column 23) (len 5)))))
    (reference r27 (scope relative) (span (offset 7550) (line 185) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 7550) (line 185) (column 23) (len 12)))))
    (reference r28 (scope relative) (span (offset 7637) (line 188) (column 60) (len 33)) (segments (segment 0 (token "CartesianSpatial3dCoordinateFrame") (name "CartesianSpatial3dCoordinateFrame") (separator none) (span (offset 7637) (line 188) (column 60) (len 33)))))
    (reference r29 (scope relative) (span (offset 7871) (line 194) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 7871) (line 194) (column 23) (len 5)))))
    (reference r30 (scope relative) (span (offset 7886) (line 194) (column 38) (len 5)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 7886) (line 194) (column 38) (len 2))) (segment 1 (token "m") (name "m") (separator colon-colon) (span (offset 7890) (line 194) (column 42) (len 1)))))
    (reference r31 (scope relative) (span (offset 7893) (line 194) (column 45) (len 5)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 7893) (line 194) (column 45) (len 2))) (segment 1 (token "m") (name "m") (separator colon-colon) (span (offset 7897) (line 194) (column 49) (len 1)))))
    (reference r32 (scope relative) (span (offset 7900) (line 194) (column 52) (len 5)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 7900) (line 194) (column 52) (len 2))) (segment 1 (token "m") (name "m") (separator colon-colon) (span (offset 7904) (line 194) (column 56) (len 1)))))
    (reference r33 (scope relative) (span (offset 8103) (line 200) (column 23) (len 14)) (segments (segment 0 (token "transformation") (name "transformation") (separator none) (span (offset 8103) (line 200) (column 23) (len 14)))))
    (reference r34 (scope relative) (span (offset 8374) (line 208) (column 58) (len 24)) (segments (segment 0 (token "Spatial3dCoordinateFrame") (name "Spatial3dCoordinateFrame") (separator none) (span (offset 8374) (line 208) (column 58) (len 24)))))
    (reference r35 (scope relative) (span (offset 9919) (line 229) (column 40) (len 10)) (segments (segment 0 (token "LengthUnit") (name "LengthUnit") (separator none) (span (offset 9919) (line 229) (column 40) (len 10)))))
    (reference r36 (scope relative) (span (offset 9963) (line 230) (column 33) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 9963) (line 230) (column 33) (len 18)))))
    (reference r37 (scope relative) (span (offset 10009) (line 231) (column 27) (len 10)) (segments (segment 0 (token "LengthUnit") (name "LengthUnit") (separator none) (span (offset 10009) (line 231) (column 27) (len 10)))))
    (reference r38 (scope relative) (span (offset 10043) (line 232) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 10043) (line 232) (column 23) (len 5)))))
    (reference r39 (scope relative) (span (offset 10052) (line 232) (column 32) (len 18)) (segments (segment 0 (token "radialDistanceUnit") (name "radialDistanceUnit") (separator none) (span (offset 10052) (line 232) (column 32) (len 18)))))
    (reference r40 (scope relative) (span (offset 10072) (line 232) (column 52) (len 11)) (segments (segment 0 (token "azimuthUnit") (name "azimuthUnit") (separator none) (span (offset 10072) (line 232) (column 52) (len 11)))))
    (reference r41 (scope relative) (span (offset 10085) (line 232) (column 65) (len 5)) (segments (segment 0 (token "zUnit") (name "zUnit") (separator none) (span (offset 10085) (line 232) (column 65) (len 5)))))
    (reference r42 (scope relative) (span (offset 10115) (line 233) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 10115) (line 233) (column 23) (len 12)))))
    (reference r43 (scope relative) (span (offset 10198) (line 236) (column 56) (len 24)) (segments (segment 0 (token "Spatial3dCoordinateFrame") (name "Spatial3dCoordinateFrame") (separator none) (span (offset 10198) (line 236) (column 56) (len 24)))))
    (reference r44 (scope relative) (span (offset 11843) (line 257) (column 40) (len 10)) (segments (segment 0 (token "LengthUnit") (name "LengthUnit") (separator none) (span (offset 11843) (line 257) (column 40) (len 10)))))
    (reference r45 (scope relative) (span (offset 11891) (line 258) (column 37) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 11891) (line 258) (column 37) (len 18)))))
    (reference r46 (scope relative) (span (offset 11943) (line 259) (column 33) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 11943) (line 259) (column 33) (len 18)))))
    (reference r47 (scope relative) (span (offset 11985) (line 260) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 11985) (line 260) (column 23) (len 5)))))
    (reference r48 (scope relative) (span (offset 11994) (line 260) (column 32) (len 18)) (segments (segment 0 (token "radialDistanceUnit") (name "radialDistanceUnit") (separator none) (span (offset 11994) (line 260) (column 32) (len 18)))))
    (reference r49 (scope relative) (span (offset 12014) (line 260) (column 52) (len 15)) (segments (segment 0 (token "inclinationUnit") (name "inclinationUnit") (separator none) (span (offset 12014) (line 260) (column 52) (len 15)))))
    (reference r50 (scope relative) (span (offset 12031) (line 260) (column 69) (len 11)) (segments (segment 0 (token "azimuthUnit") (name "azimuthUnit") (separator none) (span (offset 12031) (line 260) (column 69) (len 11)))))
    (reference r51 (scope relative) (span (offset 12067) (line 261) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 12067) (line 261) (column 23) (len 12)))))
    (reference r52 (scope relative) (span (offset 12151) (line 264) (column 57) (len 24)) (segments (segment 0 (token "Spatial3dCoordinateFrame") (name "Spatial3dCoordinateFrame") (separator none) (span (offset 12151) (line 264) (column 57) (len 24)))))
    (reference r53 (scope relative) (span (offset 15106) (line 296) (column 34) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 15106) (line 296) (column 34) (len 18)))))
    (reference r54 (scope relative) (span (offset 15160) (line 297) (column 35) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 15160) (line 297) (column 35) (len 18)))))
    (reference r55 (scope relative) (span (offset 15213) (line 298) (column 34) (len 10)) (segments (segment 0 (token "LengthUnit") (name "LengthUnit") (separator none) (span (offset 15213) (line 298) (column 34) (len 10)))))
    (reference r56 (scope relative) (span (offset 15247) (line 299) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 15247) (line 299) (column 23) (len 5)))))
    (reference r57 (scope relative) (span (offset 15256) (line 299) (column 32) (len 13)) (segments (segment 0 (token "longitudeUnit") (name "longitudeUnit") (separator none) (span (offset 15256) (line 299) (column 32) (len 13)))))
    (reference r58 (scope relative) (span (offset 15271) (line 299) (column 47) (len 12)) (segments (segment 0 (token "latitudeUnit") (name "latitudeUnit") (separator none) (span (offset 15271) (line 299) (column 47) (len 12)))))
    (reference r59 (scope relative) (span (offset 15285) (line 299) (column 61) (len 12)) (segments (segment 0 (token "altitudeUnit") (name "altitudeUnit") (separator none) (span (offset 15285) (line 299) (column 61) (len 12)))))
    (reference r60 (scope relative) (span (offset 15322) (line 300) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 15322) (line 300) (column 23) (len 12)))))
    (reference r61 (scope relative) (span (offset 15438) (line 304) (column 39) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 15438) (line 304) (column 39) (len 23)))))
    (reference r62 (scope relative) (span (offset 16044) (line 317) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 16044) (line 317) (column 23) (len 7)))))
    (reference r63 (scope relative) (span (offset 16088) (line 318) (column 29) (len 24)) (segments (segment 0 (token "Spatial3dCoordinateFrame") (name "Spatial3dCoordinateFrame") (separator none) (span (offset 16088) (line 318) (column 29) (len 24)))))
    (reference r64 (scope relative) (span (offset 16082) (line 318) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 16082) (line 318) (column 23) (len 4)))))
    (reference r65 (scope relative) (span (offset 16156) (line 321) (column 33) (len 16)) (segments (segment 0 (token "Position3dVector") (name "Position3dVector") (separator none) (span (offset 16156) (line 321) (column 33) (len 16)))))
    (reference r66 (scope relative) (span (offset 16242) (line 323) (column 48) (len 16)) (segments (segment 0 (token "Position3dVector") (name "Position3dVector") (separator none) (span (offset 16242) (line 323) (column 48) (len 16)))))
    (reference r67 (scope relative) (span (offset 16283) (line 324) (column 23) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 16283) (line 324) (column 23) (len 11)))))
    (reference r68 (scope relative) (span (offset 16297) (line 324) (column 37) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 16297) (line 324) (column 37) (len 3)))))
    (reference r69 (scope relative) (span (offset 16345) (line 325) (column 23) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 16345) (line 325) (column 23) (len 11)))))
    (reference r70 (scope relative) (span (offset 16359) (line 325) (column 37) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 16359) (line 325) (column 37) (len 3)))))
    (reference r71 (scope relative) (span (offset 16407) (line 326) (column 23) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 16407) (line 326) (column 23) (len 11)))))
    (reference r72 (scope relative) (span (offset 16421) (line 326) (column 37) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 16421) (line 326) (column 37) (len 3)))))
    (reference r73 (scope relative) (span (offset 16476) (line 327) (column 30) (len 33)) (segments (segment 0 (token "CartesianSpatial3dCoordinateFrame") (name "CartesianSpatial3dCoordinateFrame") (separator none) (span (offset 16476) (line 327) (column 30) (len 33)))))
    (reference r74 (scope relative) (span (offset 16469) (line 327) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 16469) (line 327) (column 23) (len 4)))))
    (reference r75 (scope relative) (span (offset 16562) (line 329) (column 43) (len 25)) (segments (segment 0 (token "CartesianPosition3dVector") (name "CartesianPosition3dVector") (separator none) (span (offset 16562) (line 329) (column 43) (len 25)))))
    (reference r76 (scope relative) (span (offset 16659) (line 331) (column 50) (len 16)) (segments (segment 0 (token "Position3dVector") (name "Position3dVector") (separator none) (span (offset 16659) (line 331) (column 50) (len 16)))))
    (reference r77 (scope relative) (span (offset 16720) (line 332) (column 43) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 16720) (line 332) (column 43) (len 11)))))
    (reference r78 (scope relative) (span (offset 16734) (line 332) (column 57) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 16734) (line 332) (column 57) (len 3)))))
    (reference r79 (scope relative) (span (offset 16795) (line 333) (column 36) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 16795) (line 333) (column 36) (len 18)))))
    (reference r80 (scope relative) (span (offset 16816) (line 333) (column 57) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 16816) (line 333) (column 57) (len 3)))))
    (reference r81 (scope relative) (span (offset 16873) (line 334) (column 32) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 16873) (line 334) (column 32) (len 11)))))
    (reference r82 (scope relative) (span (offset 16887) (line 334) (column 46) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 16887) (line 334) (column 46) (len 3)))))
    (reference r83 (scope relative) (span (offset 16942) (line 335) (column 30) (len 35)) (segments (segment 0 (token "CylindricalSpatial3dCoordinateFrame") (name "CylindricalSpatial3dCoordinateFrame") (separator none) (span (offset 16942) (line 335) (column 30) (len 35)))))
    (reference r84 (scope relative) (span (offset 16935) (line 335) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 16935) (line 335) (column 23) (len 4)))))
    (reference r85 (scope relative) (span (offset 17032) (line 337) (column 45) (len 27)) (segments (segment 0 (token "CylindricalPosition3dVector") (name "CylindricalPosition3dVector") (separator none) (span (offset 17032) (line 337) (column 45) (len 27)))))
    (reference r86 (scope relative) (span (offset 17129) (line 339) (column 48) (len 16)) (segments (segment 0 (token "Position3dVector") (name "Position3dVector") (separator none) (span (offset 17129) (line 339) (column 48) (len 16)))))
    (reference r87 (scope relative) (span (offset 17187) (line 340) (column 40) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 17187) (line 340) (column 40) (len 11)))))
    (reference r88 (scope relative) (span (offset 17201) (line 340) (column 54) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 17201) (line 340) (column 54) (len 3)))))
    (reference r89 (scope relative) (span (offset 17266) (line 341) (column 40) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 17266) (line 341) (column 40) (len 18)))))
    (reference r90 (scope relative) (span (offset 17287) (line 341) (column 61) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 17287) (line 341) (column 61) (len 3)))))
    (reference r91 (scope relative) (span (offset 17348) (line 342) (column 36) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 17348) (line 342) (column 36) (len 18)))))
    (reference r92 (scope relative) (span (offset 17369) (line 342) (column 57) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 17369) (line 342) (column 57) (len 3)))))
    (reference r93 (scope relative) (span (offset 17424) (line 343) (column 30) (len 33)) (segments (segment 0 (token "SphericalSpatial3dCoordinateFrame") (name "SphericalSpatial3dCoordinateFrame") (separator none) (span (offset 17424) (line 343) (column 30) (len 33)))))
    (reference r94 (scope relative) (span (offset 17417) (line 343) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 17417) (line 343) (column 23) (len 4)))))
    (reference r95 (scope relative) (span (offset 17510) (line 345) (column 43) (len 25)) (segments (segment 0 (token "SphericalPosition3dVector") (name "SphericalPosition3dVector") (separator none) (span (offset 17510) (line 345) (column 43) (len 25)))))
    (reference r96 (scope relative) (span (offset 17605) (line 347) (column 48) (len 16)) (segments (segment 0 (token "Position3dVector") (name "Position3dVector") (separator none) (span (offset 17605) (line 347) (column 48) (len 16)))))
    (reference r97 (scope relative) (span (offset 17659) (line 348) (column 36) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 17659) (line 348) (column 36) (len 18)))))
    (reference r98 (scope relative) (span (offset 17680) (line 348) (column 57) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 17680) (line 348) (column 57) (len 3)))))
    (reference r99 (scope relative) (span (offset 17743) (line 349) (column 38) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 17743) (line 349) (column 38) (len 18)))))
    (reference r100 (scope relative) (span (offset 17764) (line 349) (column 59) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 17764) (line 349) (column 59) (len 3)))))
    (reference r101 (scope relative) (span (offset 17823) (line 350) (column 34) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 17823) (line 350) (column 34) (len 11)))))
    (reference r102 (scope relative) (span (offset 17837) (line 350) (column 48) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 17837) (line 350) (column 48) (len 3)))))
    (reference r103 (scope relative) (span (offset 17892) (line 351) (column 30) (len 33)) (segments (segment 0 (token "PlanetarySpatial3dCoordinateFrame") (name "PlanetarySpatial3dCoordinateFrame") (separator none) (span (offset 17892) (line 351) (column 30) (len 33)))))
    (reference r104 (scope relative) (span (offset 17885) (line 351) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 17885) (line 351) (column 23) (len 4)))))
    (reference r105 (scope relative) (span (offset 17978) (line 353) (column 43) (len 25)) (segments (segment 0 (token "PlanetaryPosition3dVector") (name "PlanetaryPosition3dVector") (separator none) (span (offset 17978) (line 353) (column 43) (len 25)))))
    (reference r106 (scope relative) (span (offset 18115) (line 356) (column 43) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 18115) (line 356) (column 43) (len 23)))))
    (reference r107 (scope relative) (span (offset 18750) (line 369) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 18750) (line 369) (column 23) (len 7)))))
    (reference r108 (scope relative) (span (offset 18795) (line 370) (column 29) (len 24)) (segments (segment 0 (token "Spatial3dCoordinateFrame") (name "Spatial3dCoordinateFrame") (separator none) (span (offset 18795) (line 370) (column 29) (len 24)))))
    (reference r109 (scope relative) (span (offset 18789) (line 370) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 18789) (line 370) (column 23) (len 4)))))
    (reference r110 (scope relative) (span (offset 18867) (line 373) (column 37) (len 20)) (segments (segment 0 (token "Displacement3dVector") (name "Displacement3dVector") (separator none) (span (offset 18867) (line 373) (column 37) (len 20)))))
    (reference r111 (scope relative) (span (offset 18961) (line 375) (column 52) (len 20)) (segments (segment 0 (token "Displacement3dVector") (name "Displacement3dVector") (separator none) (span (offset 18961) (line 375) (column 52) (len 20)))))
    (reference r112 (scope relative) (span (offset 19006) (line 376) (column 23) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 19006) (line 376) (column 23) (len 11)))))
    (reference r113 (scope relative) (span (offset 19020) (line 376) (column 37) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 19020) (line 376) (column 37) (len 3)))))
    (reference r114 (scope relative) (span (offset 19068) (line 377) (column 23) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 19068) (line 377) (column 23) (len 11)))))
    (reference r115 (scope relative) (span (offset 19082) (line 377) (column 37) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 19082) (line 377) (column 37) (len 3)))))
    (reference r116 (scope relative) (span (offset 19130) (line 378) (column 23) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 19130) (line 378) (column 23) (len 11)))))
    (reference r117 (scope relative) (span (offset 19144) (line 378) (column 37) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 19144) (line 378) (column 37) (len 3)))))
    (reference r118 (scope relative) (span (offset 19199) (line 379) (column 30) (len 33)) (segments (segment 0 (token "CartesianSpatial3dCoordinateFrame") (name "CartesianSpatial3dCoordinateFrame") (separator none) (span (offset 19199) (line 379) (column 30) (len 33)))))
    (reference r119 (scope relative) (span (offset 19192) (line 379) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 19192) (line 379) (column 23) (len 4)))))
    (reference r120 (scope relative) (span (offset 19289) (line 381) (column 47) (len 29)) (segments (segment 0 (token "CartesianDisplacement3dVector") (name "CartesianDisplacement3dVector") (separator none) (span (offset 19289) (line 381) (column 47) (len 29)))))
    (reference r121 (scope relative) (span (offset 19398) (line 383) (column 54) (len 20)) (segments (segment 0 (token "Displacement3dVector") (name "Displacement3dVector") (separator none) (span (offset 19398) (line 383) (column 54) (len 20)))))
    (reference r122 (scope relative) (span (offset 19463) (line 384) (column 43) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 19463) (line 384) (column 43) (len 11)))))
    (reference r123 (scope relative) (span (offset 19477) (line 384) (column 57) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 19477) (line 384) (column 57) (len 3)))))
    (reference r124 (scope relative) (span (offset 19538) (line 385) (column 36) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 19538) (line 385) (column 36) (len 18)))))
    (reference r125 (scope relative) (span (offset 19559) (line 385) (column 57) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 19559) (line 385) (column 57) (len 3)))))
    (reference r126 (scope relative) (span (offset 19616) (line 386) (column 32) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 19616) (line 386) (column 32) (len 11)))))
    (reference r127 (scope relative) (span (offset 19630) (line 386) (column 46) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 19630) (line 386) (column 46) (len 3)))))
    (reference r128 (scope relative) (span (offset 19685) (line 387) (column 30) (len 35)) (segments (segment 0 (token "CylindricalSpatial3dCoordinateFrame") (name "CylindricalSpatial3dCoordinateFrame") (separator none) (span (offset 19685) (line 387) (column 30) (len 35)))))
    (reference r129 (scope relative) (span (offset 19678) (line 387) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 19678) (line 387) (column 23) (len 4)))))
    (reference r130 (scope relative) (span (offset 19779) (line 389) (column 49) (len 31)) (segments (segment 0 (token "CylindricalDisplacement3dVector") (name "CylindricalDisplacement3dVector") (separator none) (span (offset 19779) (line 389) (column 49) (len 31)))))
    (reference r131 (scope relative) (span (offset 19888) (line 391) (column 52) (len 20)) (segments (segment 0 (token "Displacement3dVector") (name "Displacement3dVector") (separator none) (span (offset 19888) (line 391) (column 52) (len 20)))))
    (reference r132 (scope relative) (span (offset 19950) (line 392) (column 40) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 19950) (line 392) (column 40) (len 11)))))
    (reference r133 (scope relative) (span (offset 19964) (line 392) (column 54) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 19964) (line 392) (column 54) (len 3)))))
    (reference r134 (scope relative) (span (offset 20029) (line 393) (column 40) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 20029) (line 393) (column 40) (len 18)))))
    (reference r135 (scope relative) (span (offset 20050) (line 393) (column 61) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 20050) (line 393) (column 61) (len 3)))))
    (reference r136 (scope relative) (span (offset 20111) (line 394) (column 36) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 20111) (line 394) (column 36) (len 18)))))
    (reference r137 (scope relative) (span (offset 20132) (line 394) (column 57) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 20132) (line 394) (column 57) (len 3)))))
    (reference r138 (scope relative) (span (offset 20187) (line 395) (column 30) (len 33)) (segments (segment 0 (token "SphericalSpatial3dCoordinateFrame") (name "SphericalSpatial3dCoordinateFrame") (separator none) (span (offset 20187) (line 395) (column 30) (len 33)))))
    (reference r139 (scope relative) (span (offset 20180) (line 395) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 20180) (line 395) (column 23) (len 4)))))
    (reference r140 (scope relative) (span (offset 20277) (line 397) (column 47) (len 29)) (segments (segment 0 (token "SphericalDisplacement3dVector") (name "SphericalDisplacement3dVector") (separator none) (span (offset 20277) (line 397) (column 47) (len 29)))))
    (reference r141 (scope relative) (span (offset 20420) (line 400) (column 34) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 20420) (line 400) (column 34) (len 11)))))
    (reference r142 (scope relative) (span (offset 21088) (line 416) (column 37) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 21088) (line 416) (column 37) (len 19)))))
    (reference r143 (scope relative) (span (offset 21592) (line 429) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 21592) (line 429) (column 28) (len 4)))))
    (reference r144 (scope relative) (span (offset 21587) (line 429) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 21587) (line 429) (column 23) (len 3)))))
    (reference r145 (scope relative) (span (offset 21626) (line 430) (column 29) (len 13)) (segments (segment 0 (token "CurvatureUnit") (name "CurvatureUnit") (separator none) (span (offset 21626) (line 430) (column 29) (len 13)))))
    (reference r146 (scope relative) (span (offset 21620) (line 430) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 21620) (line 430) (column 23) (len 4)))))
    (reference r147 (scope relative) (span (offset 21676) (line 433) (column 26) (len 14)) (segments (segment 0 (token "CurvatureValue") (name "CurvatureValue") (separator none) (span (offset 21676) (line 433) (column 26) (len 14)))))
    (reference r148 (scope relative) (span (offset 21761) (line 435) (column 36) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 21761) (line 435) (column 36) (len 11)))))
    (reference r149 (scope relative) (span (offset 21811) (line 436) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 21811) (line 436) (column 37) (len 19)))))
    (reference r150 (scope relative) (span (offset 21840) (line 436) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 21840) (line 436) (column 66) (len 8)))))
    (reference r151 (scope relative) (span (offset 21851) (line 436) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 21851) (line 436) (column 77) (len 3)))))
    (reference r152 (scope relative) (span (offset 21855) (line 436) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 21855) (line 436) (column 81) (len 1)))))
    (reference r153 (scope relative) (span (offset 21862) (line 436) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 21862) (line 436) (column 88) (len 8)))))
    (reference r154 (scope relative) (span (offset 21901) (line 437) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 21901) (line 437) (column 23) (len 17)))))
    (reference r155 (scope relative) (span (offset 21925) (line 437) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 21925) (line 437) (column 47) (len 20)))))
    (reference r156 (scope relative) (span (offset 21948) (line 437) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 21948) (line 437) (column 70) (len 8)))))
    (reference r157 (scope relative) (span (offset 22034) (line 441) (column 32) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 22034) (line 441) (column 32) (len 19)))))
    (reference r158 (scope relative) (span (offset 22713) (line 454) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 22713) (line 454) (column 28) (len 4)))))
    (reference r159 (scope relative) (span (offset 22708) (line 454) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 22708) (line 454) (column 23) (len 3)))))
    (reference r160 (scope relative) (span (offset 22747) (line 455) (column 29) (len 8)) (segments (segment 0 (token "AreaUnit") (name "AreaUnit") (separator none) (span (offset 22747) (line 455) (column 29) (len 8)))))
    (reference r161 (scope relative) (span (offset 22741) (line 455) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 22741) (line 455) (column 23) (len 4)))))
    (reference r162 (scope relative) (span (offset 22787) (line 458) (column 21) (len 9)) (segments (segment 0 (token "AreaValue") (name "AreaValue") (separator none) (span (offset 22787) (line 458) (column 21) (len 9)))))
    (reference r163 (scope relative) (span (offset 22862) (line 460) (column 31) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 22862) (line 460) (column 31) (len 11)))))
    (reference r164 (scope relative) (span (offset 22912) (line 461) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 22912) (line 461) (column 37) (len 19)))))
    (reference r165 (scope relative) (span (offset 22941) (line 461) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 22941) (line 461) (column 66) (len 8)))))
    (reference r166 (scope relative) (span (offset 22952) (line 461) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 22952) (line 461) (column 77) (len 3)))))
    (reference r167 (scope relative) (span (offset 22956) (line 461) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 22956) (line 461) (column 81) (len 1)))))
    (reference r168 (scope relative) (span (offset 22963) (line 461) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 22963) (line 461) (column 88) (len 8)))))
    (reference r169 (scope relative) (span (offset 23001) (line 462) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 23001) (line 462) (column 23) (len 17)))))
    (reference r170 (scope relative) (span (offset 23025) (line 462) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 23025) (line 462) (column 47) (len 20)))))
    (reference r171 (scope relative) (span (offset 23048) (line 462) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 23048) (line 462) (column 70) (len 8)))))
    (reference r172 (scope relative) (span (offset 23138) (line 466) (column 34) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 23138) (line 466) (column 34) (len 19)))))
    (reference r173 (scope relative) (span (offset 23759) (line 479) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 23759) (line 479) (column 28) (len 4)))))
    (reference r174 (scope relative) (span (offset 23754) (line 479) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 23754) (line 479) (column 23) (len 3)))))
    (reference r175 (scope relative) (span (offset 23793) (line 480) (column 29) (len 10)) (segments (segment 0 (token "VolumeUnit") (name "VolumeUnit") (separator none) (span (offset 23793) (line 480) (column 29) (len 10)))))
    (reference r176 (scope relative) (span (offset 23787) (line 480) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 23787) (line 480) (column 23) (len 4)))))
    (reference r177 (scope relative) (span (offset 23837) (line 483) (column 23) (len 11)) (segments (segment 0 (token "VolumeValue") (name "VolumeValue") (separator none) (span (offset 23837) (line 483) (column 23) (len 11)))))
    (reference r178 (scope relative) (span (offset 23916) (line 485) (column 33) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 23916) (line 485) (column 33) (len 11)))))
    (reference r179 (scope relative) (span (offset 23966) (line 486) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 23966) (line 486) (column 37) (len 19)))))
    (reference r180 (scope relative) (span (offset 23995) (line 486) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 23995) (line 486) (column 66) (len 8)))))
    (reference r181 (scope relative) (span (offset 24006) (line 486) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 24006) (line 486) (column 77) (len 3)))))
    (reference r182 (scope relative) (span (offset 24010) (line 486) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 24010) (line 486) (column 81) (len 1)))))
    (reference r183 (scope relative) (span (offset 24017) (line 486) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 24017) (line 486) (column 88) (len 8)))))
    (reference r184 (scope relative) (span (offset 24055) (line 487) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 24055) (line 487) (column 23) (len 17)))))
    (reference r185 (scope relative) (span (offset 24079) (line 487) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 24079) (line 487) (column 47) (len 20)))))
    (reference r186 (scope relative) (span (offset 24102) (line 487) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 24102) (line 487) (column 70) (len 8)))))
    (reference r187 (scope relative) (span (offset 24222) (line 491) (column 42) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 24222) (line 491) (column 42) (len 19)))))
    (reference r188 (scope relative) (span (offset 25033) (line 504) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 25033) (line 504) (column 28) (len 4)))))
    (reference r189 (scope relative) (span (offset 25028) (line 504) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 25028) (line 504) (column 23) (len 3)))))
    (reference r190 (scope relative) (span (offset 25067) (line 505) (column 29) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 25067) (line 505) (column 29) (len 18)))))
    (reference r191 (scope relative) (span (offset 25061) (line 505) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 25061) (line 505) (column 23) (len 4)))))
    (reference r192 (scope relative) (span (offset 25127) (line 508) (column 31) (len 19)) (segments (segment 0 (token "AngularMeasureValue") (name "AngularMeasureValue") (separator none) (span (offset 25127) (line 508) (column 31) (len 19)))))
    (reference r193 (scope relative) (span (offset 25222) (line 510) (column 41) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 25222) (line 510) (column 41) (len 16)))))
    (reference r194 (scope relative) (span (offset 25277) (line 513) (column 30) (len 18)) (segments (segment 0 (token "AngularMeasureUnit") (name "AngularMeasureUnit") (separator none) (span (offset 25277) (line 513) (column 30) (len 18)))))
    (reference r195 (scope relative) (span (offset 25327) (line 514) (column 31) (len 19)) (segments (segment 0 (token "AngularMeasureValue") (name "AngularMeasureValue") (separator none) (span (offset 25327) (line 514) (column 31) (len 19)))))
    (reference r196 (scope relative) (span (offset 25373) (line 515) (column 26) (len 14)) (segments (segment 0 (token "angularMeasure") (name "angularMeasure") (separator none) (span (offset 25373) (line 515) (column 26) (len 14)))))
    (reference r197 (scope relative) (span (offset 25505) (line 518) (column 39) (len 19)) (segments (segment 0 (token "AngularMeasureValue") (name "AngularMeasureValue") (separator none) (span (offset 25505) (line 518) (column 39) (len 19)))))
    (reference r198 (scope relative) (span (offset 26516) (line 533) (column 35) (len 22)) (segments (segment 0 (token "rotationalDisplacement") (name "rotationalDisplacement") (separator none) (span (offset 26516) (line 533) (column 35) (len 22)))))
    (reference r199 (scope relative) (span (offset 26610) (line 536) (column 27) (len 19)) (segments (segment 0 (token "AngularMeasureValue") (name "AngularMeasureValue") (separator none) (span (offset 26610) (line 536) (column 27) (len 19)))))
    (reference r200 (scope relative) (span (offset 27377) (line 552) (column 47) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 27377) (line 552) (column 47) (len 19)))))
    (reference r201 (scope relative) (span (offset 28236) (line 565) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 28236) (line 565) (column 28) (len 4)))))
    (reference r202 (scope relative) (span (offset 28231) (line 565) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 28231) (line 565) (column 23) (len 3)))))
    (reference r203 (scope relative) (span (offset 28270) (line 566) (column 29) (len 23)) (segments (segment 0 (token "SolidAngularMeasureUnit") (name "SolidAngularMeasureUnit") (separator none) (span (offset 28270) (line 566) (column 29) (len 23)))))
    (reference r204 (scope relative) (span (offset 28264) (line 566) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 28264) (line 566) (column 23) (len 4)))))
    (reference r205 (scope relative) (span (offset 28340) (line 569) (column 36) (len 24)) (segments (segment 0 (token "SolidAngularMeasureValue") (name "SolidAngularMeasureValue") (separator none) (span (offset 28340) (line 569) (column 36) (len 24)))))
    (reference r206 (scope relative) (span (offset 28445) (line 571) (column 46) (len 16)) (segments (segment 0 (token "DimensionOneUnit") (name "DimensionOneUnit") (separator none) (span (offset 28445) (line 571) (column 46) (len 16)))))
    (reference r207 (scope relative) (span (offset 28626) (line 577) (column 24) (len 12)) (segments (segment 0 (token "DurationUnit") (name "DurationUnit") (separator none) (span (offset 28626) (line 577) (column 24) (len 12)))))
    (reference r208 (scope relative) (span (offset 28664) (line 578) (column 25) (len 13)) (segments (segment 0 (token "DurationValue") (name "DurationValue") (separator none) (span (offset 28664) (line 578) (column 25) (len 13)))))
    (reference r209 (scope relative) (span (offset 28698) (line 579) (column 20) (len 8)) (segments (segment 0 (token "duration") (name "duration") (separator none) (span (offset 28698) (line 579) (column 20) (len 8)))))
    (reference r210 (scope relative) (span (offset 28799) (line 582) (column 48) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 28799) (line 582) (column 48) (len 23)))))
    (reference r211 (scope relative) (span (offset 29575) (line 595) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 29575) (line 595) (column 23) (len 7)))))
    (reference r212 (scope relative) (span (offset 29620) (line 596) (column 29) (len 34)) (segments (segment 0 (token "CartesianVelocity3dCoordinateFrame") (name "CartesianVelocity3dCoordinateFrame") (separator none) (span (offset 29620) (line 596) (column 29) (len 34)))))
    (reference r213 (scope relative) (span (offset 29614) (line 596) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 29614) (line 596) (column 23) (len 4)))))
    (reference r214 (scope relative) (span (offset 29707) (line 599) (column 42) (len 25)) (segments (segment 0 (token "CartesianVelocity3dVector") (name "CartesianVelocity3dVector") (separator none) (span (offset 29707) (line 599) (column 42) (len 25)))))
    (reference r215 (scope relative) (span (offset 29811) (line 601) (column 57) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 29811) (line 601) (column 57) (len 19)))))
    (reference r216 (scope relative) (span (offset 29855) (line 602) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 29855) (line 602) (column 23) (len 7)))))
    (reference r217 (scope relative) (span (offset 29894) (line 603) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 29894) (line 603) (column 23) (len 12)))))
    (reference r218 (scope relative) (span (offset 29944) (line 604) (column 30) (len 9)) (segments (segment 0 (token "SpeedUnit") (name "SpeedUnit") (separator none) (span (offset 29944) (line 604) (column 30) (len 9)))))
    (reference r219 (scope relative) (span (offset 29937) (line 604) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 29937) (line 604) (column 23) (len 5)))))
    (reference r220 (scope relative) (span (offset 30037) (line 608) (column 33) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 30037) (line 608) (column 33) (len 19)))))
    (reference r221 (scope relative) (span (offset 30460) (line 621) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 30460) (line 621) (column 28) (len 4)))))
    (reference r222 (scope relative) (span (offset 30455) (line 621) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 30455) (line 621) (column 23) (len 3)))))
    (reference r223 (scope relative) (span (offset 30494) (line 622) (column 29) (len 9)) (segments (segment 0 (token "SpeedUnit") (name "SpeedUnit") (separator none) (span (offset 30494) (line 622) (column 29) (len 9)))))
    (reference r224 (scope relative) (span (offset 30488) (line 622) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 30488) (line 622) (column 23) (len 4)))))
    (reference r225 (scope relative) (span (offset 30536) (line 625) (column 22) (len 10)) (segments (segment 0 (token "SpeedValue") (name "SpeedValue") (separator none) (span (offset 30536) (line 625) (column 22) (len 10)))))
    (reference r226 (scope relative) (span (offset 30613) (line 627) (column 32) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 30613) (line 627) (column 32) (len 11)))))
    (reference r227 (scope relative) (span (offset 30663) (line 628) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 30663) (line 628) (column 37) (len 19)))))
    (reference r228 (scope relative) (span (offset 30692) (line 628) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 30692) (line 628) (column 66) (len 8)))))
    (reference r229 (scope relative) (span (offset 30703) (line 628) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 30703) (line 628) (column 77) (len 3)))))
    (reference r230 (scope relative) (span (offset 30707) (line 628) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 30707) (line 628) (column 81) (len 1)))))
    (reference r231 (scope relative) (span (offset 30714) (line 628) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 30714) (line 628) (column 88) (len 8)))))
    (reference r232 (scope relative) (span (offset 30768) (line 629) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 30768) (line 629) (column 39) (len 19)))))
    (reference r233 (scope relative) (span (offset 30797) (line 629) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 30797) (line 629) (column 68) (len 8)))))
    (reference r234 (scope relative) (span (offset 30808) (line 629) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 30808) (line 629) (column 79) (len 3)))))
    (reference r235 (scope relative) (span (offset 30812) (line 629) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 30812) (line 629) (column 83) (len 1)))))
    (reference r236 (scope relative) (span (offset 30819) (line 629) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 30819) (line 629) (column 90) (len 8)))))
    (reference r237 (scope relative) (span (offset 30858) (line 630) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 30858) (line 630) (column 23) (len 17)))))
    (reference r238 (scope relative) (span (offset 30882) (line 630) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 30882) (line 630) (column 47) (len 20)))))
    (reference r239 (scope relative) (span (offset 30906) (line 630) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 30906) (line 630) (column 71) (len 8)))))
    (reference r240 (scope relative) (span (offset 30916) (line 630) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 30916) (line 630) (column 81) (len 10)))))
    (reference r241 (scope relative) (span (offset 31022) (line 634) (column 40) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 31022) (line 634) (column 40) (len 19)))))
    (reference r242 (scope relative) (span (offset 31719) (line 647) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 31719) (line 647) (column 28) (len 4)))))
    (reference r243 (scope relative) (span (offset 31714) (line 647) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 31714) (line 647) (column 23) (len 3)))))
    (reference r244 (scope relative) (span (offset 31753) (line 648) (column 29) (len 16)) (segments (segment 0 (token "AccelerationUnit") (name "AccelerationUnit") (separator none) (span (offset 31753) (line 648) (column 29) (len 16)))))
    (reference r245 (scope relative) (span (offset 31747) (line 648) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 31747) (line 648) (column 23) (len 4)))))
    (reference r246 (scope relative) (span (offset 31809) (line 651) (column 29) (len 17)) (segments (segment 0 (token "AccelerationValue") (name "AccelerationValue") (separator none) (span (offset 31809) (line 651) (column 29) (len 17)))))
    (reference r247 (scope relative) (span (offset 31900) (line 653) (column 39) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 31900) (line 653) (column 39) (len 11)))))
    (reference r248 (scope relative) (span (offset 31950) (line 654) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 31950) (line 654) (column 37) (len 19)))))
    (reference r249 (scope relative) (span (offset 31979) (line 654) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 31979) (line 654) (column 66) (len 8)))))
    (reference r250 (scope relative) (span (offset 31990) (line 654) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 31990) (line 654) (column 77) (len 3)))))
    (reference r251 (scope relative) (span (offset 31994) (line 654) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 31994) (line 654) (column 81) (len 1)))))
    (reference r252 (scope relative) (span (offset 32001) (line 654) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 32001) (line 654) (column 88) (len 8)))))
    (reference r253 (scope relative) (span (offset 32055) (line 655) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 32055) (line 655) (column 39) (len 19)))))
    (reference r254 (scope relative) (span (offset 32084) (line 655) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 32084) (line 655) (column 68) (len 8)))))
    (reference r255 (scope relative) (span (offset 32095) (line 655) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 32095) (line 655) (column 79) (len 3)))))
    (reference r256 (scope relative) (span (offset 32099) (line 655) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 32099) (line 655) (column 83) (len 1)))))
    (reference r257 (scope relative) (span (offset 32106) (line 655) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 32106) (line 655) (column 90) (len 8)))))
    (reference r258 (scope relative) (span (offset 32145) (line 656) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 32145) (line 656) (column 23) (len 17)))))
    (reference r259 (scope relative) (span (offset 32169) (line 656) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 32169) (line 656) (column 47) (len 20)))))
    (reference r260 (scope relative) (span (offset 32193) (line 656) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 32193) (line 656) (column 71) (len 8)))))
    (reference r261 (scope relative) (span (offset 32203) (line 656) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 32203) (line 656) (column 81) (len 10)))))
    (reference r262 (scope relative) (span (offset 32276) (line 659) (column 52) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 32276) (line 659) (column 52) (len 23)))))
    (reference r263 (scope relative) (span (offset 32974) (line 672) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 32974) (line 672) (column 23) (len 7)))))
    (reference r264 (scope relative) (span (offset 33019) (line 673) (column 29) (len 38)) (segments (segment 0 (token "CartesianAcceleration3dCoordinateFrame") (name "CartesianAcceleration3dCoordinateFrame") (separator none) (span (offset 33019) (line 673) (column 29) (len 38)))))
    (reference r265 (scope relative) (span (offset 33013) (line 673) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 33013) (line 673) (column 23) (len 4)))))
    (reference r266 (scope relative) (span (offset 33114) (line 676) (column 46) (len 29)) (segments (segment 0 (token "CartesianAcceleration3dVector") (name "CartesianAcceleration3dVector") (separator none) (span (offset 33114) (line 676) (column 46) (len 29)))))
    (reference r267 (scope relative) (span (offset 33226) (line 678) (column 61) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 33226) (line 678) (column 61) (len 19)))))
    (reference r268 (scope relative) (span (offset 33270) (line 679) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 33270) (line 679) (column 23) (len 7)))))
    (reference r269 (scope relative) (span (offset 33309) (line 680) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 33309) (line 680) (column 23) (len 12)))))
    (reference r270 (scope relative) (span (offset 33359) (line 681) (column 30) (len 16)) (segments (segment 0 (token "AccelerationUnit") (name "AccelerationUnit") (separator none) (span (offset 33359) (line 681) (column 30) (len 16)))))
    (reference r271 (scope relative) (span (offset 33352) (line 681) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 33352) (line 681) (column 23) (len 5)))))
    (reference r272 (scope relative) (span (offset 33478) (line 685) (column 43) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 33478) (line 685) (column 43) (len 19)))))
    (reference r273 (scope relative) (span (offset 34374) (line 698) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 34374) (line 698) (column 28) (len 4)))))
    (reference r274 (scope relative) (span (offset 34369) (line 698) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 34369) (line 698) (column 23) (len 3)))))
    (reference r275 (scope relative) (span (offset 34408) (line 699) (column 29) (len 19)) (segments (segment 0 (token "AngularVelocityUnit") (name "AngularVelocityUnit") (separator none) (span (offset 34408) (line 699) (column 29) (len 19)))))
    (reference r276 (scope relative) (span (offset 34402) (line 699) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 34402) (line 699) (column 23) (len 4)))))
    (reference r277 (scope relative) (span (offset 34470) (line 702) (column 32) (len 20)) (segments (segment 0 (token "AngularVelocityValue") (name "AngularVelocityValue") (separator none) (span (offset 34470) (line 702) (column 32) (len 20)))))
    (reference r278 (scope relative) (span (offset 34567) (line 704) (column 42) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 34567) (line 704) (column 42) (len 11)))))
    (reference r279 (scope relative) (span (offset 34619) (line 705) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 34619) (line 705) (column 39) (len 19)))))
    (reference r280 (scope relative) (span (offset 34648) (line 705) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 34648) (line 705) (column 68) (len 8)))))
    (reference r281 (scope relative) (span (offset 34659) (line 705) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 34659) (line 705) (column 79) (len 3)))))
    (reference r282 (scope relative) (span (offset 34663) (line 705) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 34663) (line 705) (column 83) (len 1)))))
    (reference r283 (scope relative) (span (offset 34670) (line 705) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 34670) (line 705) (column 90) (len 8)))))
    (reference r284 (scope relative) (span (offset 34709) (line 706) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 34709) (line 706) (column 23) (len 17)))))
    (reference r285 (scope relative) (span (offset 34733) (line 706) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 34733) (line 706) (column 47) (len 20)))))
    (reference r286 (scope relative) (span (offset 34756) (line 706) (column 70) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 34756) (line 706) (column 70) (len 10)))))
    (reference r287 (scope relative) (span (offset 34831) (line 709) (column 55) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 34831) (line 709) (column 55) (len 23)))))
    (reference r288 (scope relative) (span (offset 35728) (line 722) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 35728) (line 722) (column 23) (len 7)))))
    (reference r289 (scope relative) (span (offset 35773) (line 723) (column 29) (len 41)) (segments (segment 0 (token "CartesianAngularVelocity3dCoordinateFrame") (name "CartesianAngularVelocity3dCoordinateFrame") (separator none) (span (offset 35773) (line 723) (column 29) (len 41)))))
    (reference r290 (scope relative) (span (offset 35767) (line 723) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 35767) (line 723) (column 23) (len 4)))))
    (reference r291 (scope relative) (span (offset 35874) (line 726) (column 49) (len 32)) (segments (segment 0 (token "CartesianAngularVelocity3dVector") (name "CartesianAngularVelocity3dVector") (separator none) (span (offset 35874) (line 726) (column 49) (len 32)))))
    (reference r292 (scope relative) (span (offset 35992) (line 728) (column 64) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 35992) (line 728) (column 64) (len 19)))))
    (reference r293 (scope relative) (span (offset 36036) (line 729) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 36036) (line 729) (column 23) (len 7)))))
    (reference r294 (scope relative) (span (offset 36075) (line 730) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 36075) (line 730) (column 23) (len 12)))))
    (reference r295 (scope relative) (span (offset 36125) (line 731) (column 30) (len 19)) (segments (segment 0 (token "AngularVelocityUnit") (name "AngularVelocityUnit") (separator none) (span (offset 36125) (line 731) (column 30) (len 19)))))
    (reference r296 (scope relative) (span (offset 36118) (line 731) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 36118) (line 731) (column 23) (len 5)))))
    (reference r297 (scope relative) (span (offset 36255) (line 735) (column 47) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 36255) (line 735) (column 47) (len 19)))))
    (reference r298 (scope relative) (span (offset 36908) (line 748) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 36908) (line 748) (column 28) (len 4)))))
    (reference r299 (scope relative) (span (offset 36903) (line 748) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 36903) (line 748) (column 23) (len 3)))))
    (reference r300 (scope relative) (span (offset 36942) (line 749) (column 29) (len 23)) (segments (segment 0 (token "AngularAccelerationUnit") (name "AngularAccelerationUnit") (separator none) (span (offset 36942) (line 749) (column 29) (len 23)))))
    (reference r301 (scope relative) (span (offset 36936) (line 749) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 36936) (line 749) (column 23) (len 4)))))
    (reference r302 (scope relative) (span (offset 37012) (line 752) (column 36) (len 24)) (segments (segment 0 (token "AngularAccelerationValue") (name "AngularAccelerationValue") (separator none) (span (offset 37012) (line 752) (column 36) (len 24)))))
    (reference r303 (scope relative) (span (offset 37117) (line 754) (column 46) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 37117) (line 754) (column 46) (len 11)))))
    (reference r304 (scope relative) (span (offset 37169) (line 755) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 37169) (line 755) (column 39) (len 19)))))
    (reference r305 (scope relative) (span (offset 37198) (line 755) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 37198) (line 755) (column 68) (len 8)))))
    (reference r306 (scope relative) (span (offset 37209) (line 755) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 37209) (line 755) (column 79) (len 3)))))
    (reference r307 (scope relative) (span (offset 37213) (line 755) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 37213) (line 755) (column 83) (len 1)))))
    (reference r308 (scope relative) (span (offset 37220) (line 755) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 37220) (line 755) (column 90) (len 8)))))
    (reference r309 (scope relative) (span (offset 37259) (line 756) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 37259) (line 756) (column 23) (len 17)))))
    (reference r310 (scope relative) (span (offset 37283) (line 756) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 37283) (line 756) (column 47) (len 20)))))
    (reference r311 (scope relative) (span (offset 37306) (line 756) (column 70) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 37306) (line 756) (column 70) (len 10)))))
    (reference r312 (scope relative) (span (offset 37385) (line 759) (column 59) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 37385) (line 759) (column 59) (len 23)))))
    (reference r313 (scope relative) (span (offset 38039) (line 772) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 38039) (line 772) (column 23) (len 7)))))
    (reference r314 (scope relative) (span (offset 38084) (line 773) (column 29) (len 45)) (segments (segment 0 (token "CartesianAngularAcceleration3dCoordinateFrame") (name "CartesianAngularAcceleration3dCoordinateFrame") (separator none) (span (offset 38084) (line 773) (column 29) (len 45)))))
    (reference r315 (scope relative) (span (offset 38078) (line 773) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 38078) (line 773) (column 23) (len 4)))))
    (reference r316 (scope relative) (span (offset 38193) (line 776) (column 53) (len 36)) (segments (segment 0 (token "CartesianAngularAcceleration3dVector") (name "CartesianAngularAcceleration3dVector") (separator none) (span (offset 38193) (line 776) (column 53) (len 36)))))
    (reference r317 (scope relative) (span (offset 38319) (line 778) (column 68) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 38319) (line 778) (column 68) (len 19)))))
    (reference r318 (scope relative) (span (offset 38363) (line 779) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 38363) (line 779) (column 23) (len 7)))))
    (reference r319 (scope relative) (span (offset 38402) (line 780) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 38402) (line 780) (column 23) (len 12)))))
    (reference r320 (scope relative) (span (offset 38452) (line 781) (column 30) (len 23)) (segments (segment 0 (token "AngularAccelerationUnit") (name "AngularAccelerationUnit") (separator none) (span (offset 38452) (line 781) (column 30) (len 23)))))
    (reference r321 (scope relative) (span (offset 38445) (line 781) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 38445) (line 781) (column 23) (len 5)))))
    (reference r322 (scope relative) (span (offset 38573) (line 785) (column 31) (len 13)) (segments (segment 0 (token "DurationValue") (name "DurationValue") (separator none) (span (offset 38573) (line 785) (column 31) (len 13)))))
    (reference r323 (scope relative) (span (offset 39116) (line 800) (column 22) (len 14)) (segments (segment 0 (token "periodDuration") (name "periodDuration") (separator none) (span (offset 39116) (line 800) (column 22) (len 14)))))
    (reference r324 (scope relative) (span (offset 39207) (line 803) (column 29) (len 13)) (segments (segment 0 (token "DurationValue") (name "DurationValue") (separator none) (span (offset 39207) (line 803) (column 29) (len 13)))))
    (reference r325 (scope relative) (span (offset 40003) (line 819) (column 25) (len 10)) (segments (segment 0 (token "CountValue") (name "CountValue") (separator none) (span (offset 40003) (line 819) (column 25) (len 10)))))
    (reference r326 (scope relative) (span (offset 40690) (line 835) (column 37) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 40690) (line 835) (column 37) (len 19)))))
    (reference r327 (scope relative) (span (offset 41187) (line 848) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 41187) (line 848) (column 28) (len 4)))))
    (reference r328 (scope relative) (span (offset 41182) (line 848) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 41182) (line 848) (column 23) (len 3)))))
    (reference r329 (scope relative) (span (offset 41221) (line 849) (column 29) (len 13)) (segments (segment 0 (token "FrequencyUnit") (name "FrequencyUnit") (separator none) (span (offset 41221) (line 849) (column 29) (len 13)))))
    (reference r330 (scope relative) (span (offset 41215) (line 849) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 41215) (line 849) (column 23) (len 4)))))
    (reference r331 (scope relative) (span (offset 41271) (line 852) (column 26) (len 14)) (segments (segment 0 (token "FrequencyValue") (name "FrequencyValue") (separator none) (span (offset 41271) (line 852) (column 26) (len 14)))))
    (reference r332 (scope relative) (span (offset 41356) (line 854) (column 36) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 41356) (line 854) (column 36) (len 11)))))
    (reference r333 (scope relative) (span (offset 41408) (line 855) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 41408) (line 855) (column 39) (len 19)))))
    (reference r334 (scope relative) (span (offset 41437) (line 855) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 41437) (line 855) (column 68) (len 8)))))
    (reference r335 (scope relative) (span (offset 41448) (line 855) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 41448) (line 855) (column 79) (len 3)))))
    (reference r336 (scope relative) (span (offset 41452) (line 855) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 41452) (line 855) (column 83) (len 1)))))
    (reference r337 (scope relative) (span (offset 41459) (line 855) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 41459) (line 855) (column 90) (len 8)))))
    (reference r338 (scope relative) (span (offset 41498) (line 856) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 41498) (line 856) (column 23) (len 17)))))
    (reference r339 (scope relative) (span (offset 41522) (line 856) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 41522) (line 856) (column 47) (len 20)))))
    (reference r340 (scope relative) (span (offset 41545) (line 856) (column 70) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 41545) (line 856) (column 70) (len 10)))))
    (reference r341 (scope relative) (span (offset 41656) (line 860) (column 36) (len 14)) (segments (segment 0 (token "FrequencyValue") (name "FrequencyValue") (separator none) (span (offset 41656) (line 860) (column 36) (len 14)))))
    (reference r342 (scope relative) (span (offset 42337) (line 876) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 42337) (line 876) (column 44) (len 19)))))
    (reference r343 (scope relative) (span (offset 42862) (line 889) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 42862) (line 889) (column 28) (len 4)))))
    (reference r344 (scope relative) (span (offset 42857) (line 889) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 42857) (line 889) (column 23) (len 3)))))
    (reference r345 (scope relative) (span (offset 42896) (line 890) (column 29) (len 20)) (segments (segment 0 (token "AngularFrequencyUnit") (name "AngularFrequencyUnit") (separator none) (span (offset 42896) (line 890) (column 29) (len 20)))))
    (reference r346 (scope relative) (span (offset 42890) (line 890) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 42890) (line 890) (column 23) (len 4)))))
    (reference r347 (scope relative) (span (offset 42960) (line 893) (column 33) (len 21)) (segments (segment 0 (token "AngularFrequencyValue") (name "AngularFrequencyValue") (separator none) (span (offset 42960) (line 893) (column 33) (len 21)))))
    (reference r348 (scope relative) (span (offset 43059) (line 895) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 43059) (line 895) (column 43) (len 11)))))
    (reference r349 (scope relative) (span (offset 43111) (line 896) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 43111) (line 896) (column 39) (len 19)))))
    (reference r350 (scope relative) (span (offset 43140) (line 896) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 43140) (line 896) (column 68) (len 8)))))
    (reference r351 (scope relative) (span (offset 43151) (line 896) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 43151) (line 896) (column 79) (len 3)))))
    (reference r352 (scope relative) (span (offset 43155) (line 896) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 43155) (line 896) (column 83) (len 1)))))
    (reference r353 (scope relative) (span (offset 43162) (line 896) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 43162) (line 896) (column 90) (len 8)))))
    (reference r354 (scope relative) (span (offset 43201) (line 897) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 43201) (line 897) (column 23) (len 17)))))
    (reference r355 (scope relative) (span (offset 43225) (line 897) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 43225) (line 897) (column 47) (len 20)))))
    (reference r356 (scope relative) (span (offset 43248) (line 897) (column 70) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 43248) (line 897) (column 70) (len 10)))))
    (reference r357 (scope relative) (span (offset 43338) (line 901) (column 27) (len 11)) (segments (segment 0 (token "LengthValue") (name "LengthValue") (separator none) (span (offset 43338) (line 901) (column 27) (len 11)))))
    (reference r358 (scope relative) (span (offset 43861) (line 917) (column 37) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 43861) (line 917) (column 37) (len 19)))))
    (reference r359 (scope relative) (span (offset 44365) (line 930) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 44365) (line 930) (column 28) (len 4)))))
    (reference r360 (scope relative) (span (offset 44360) (line 930) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 44360) (line 930) (column 23) (len 3)))))
    (reference r361 (scope relative) (span (offset 44399) (line 931) (column 29) (len 13)) (segments (segment 0 (token "RepetencyUnit") (name "RepetencyUnit") (separator none) (span (offset 44399) (line 931) (column 29) (len 13)))))
    (reference r362 (scope relative) (span (offset 44393) (line 931) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 44393) (line 931) (column 23) (len 4)))))
    (reference r363 (scope relative) (span (offset 44449) (line 934) (column 26) (len 14)) (segments (segment 0 (token "RepetencyValue") (name "RepetencyValue") (separator none) (span (offset 44449) (line 934) (column 26) (len 14)))))
    (reference r364 (scope relative) (span (offset 44534) (line 936) (column 36) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 44534) (line 936) (column 36) (len 11)))))
    (reference r365 (scope relative) (span (offset 44584) (line 937) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 44584) (line 937) (column 37) (len 19)))))
    (reference r366 (scope relative) (span (offset 44613) (line 937) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 44613) (line 937) (column 66) (len 8)))))
    (reference r367 (scope relative) (span (offset 44624) (line 937) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 44624) (line 937) (column 77) (len 3)))))
    (reference r368 (scope relative) (span (offset 44628) (line 937) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 44628) (line 937) (column 81) (len 1)))))
    (reference r369 (scope relative) (span (offset 44635) (line 937) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 44635) (line 937) (column 88) (len 8)))))
    (reference r370 (scope relative) (span (offset 44674) (line 938) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 44674) (line 938) (column 23) (len 17)))))
    (reference r371 (scope relative) (span (offset 44698) (line 938) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 44698) (line 938) (column 47) (len 20)))))
    (reference r372 (scope relative) (span (offset 44721) (line 938) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 44721) (line 938) (column 70) (len 8)))))
    (reference r373 (scope relative) (span (offset 44769) (line 941) (column 30) (len 13)) (segments (segment 0 (token "RepetencyUnit") (name "RepetencyUnit") (separator none) (span (offset 44769) (line 941) (column 30) (len 13)))))
    (reference r374 (scope relative) (span (offset 44814) (line 942) (column 31) (len 14)) (segments (segment 0 (token "RepetencyValue") (name "RepetencyValue") (separator none) (span (offset 44814) (line 942) (column 31) (len 14)))))
    (reference r375 (scope relative) (span (offset 44855) (line 943) (column 26) (len 9)) (segments (segment 0 (token "repetency") (name "repetency") (separator none) (span (offset 44855) (line 943) (column 26) (len 9)))))
    (reference r376 (scope relative) (span (offset 44954) (line 946) (column 44) (len 23)) (segments (segment 0 (token "'3dVectorQuantityValue'") (name "3dVectorQuantityValue") (separator none) (span (offset 44954) (line 946) (column 44) (len 23)))))
    (reference r377 (scope relative) (span (offset 45459) (line 959) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 45459) (line 959) (column 23) (len 7)))))
    (reference r378 (scope relative) (span (offset 45504) (line 960) (column 29) (len 36)) (segments (segment 0 (token "CartesianWaveVector3dCoordinateFrame") (name "CartesianWaveVector3dCoordinateFrame") (separator none) (span (offset 45504) (line 960) (column 29) (len 36)))))
    (reference r379 (scope relative) (span (offset 45498) (line 960) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 45498) (line 960) (column 23) (len 4)))))
    (reference r380 (scope relative) (span (offset 45589) (line 963) (column 38) (len 21)) (segments (segment 0 (token "CartesianWave3dVector") (name "CartesianWave3dVector") (separator none) (span (offset 45589) (line 963) (column 38) (len 21)))))
    (reference r381 (scope relative) (span (offset 45691) (line 965) (column 59) (len 19)) (segments (segment 0 (token "'3dCoordinateFrame'") (name "3dCoordinateFrame") (separator none) (span (offset 45691) (line 965) (column 59) (len 19)))))
    (reference r382 (scope relative) (span (offset 45735) (line 966) (column 23) (len 7)) (segments (segment 0 (token "isBound") (name "isBound") (separator none) (span (offset 45735) (line 966) (column 23) (len 7)))))
    (reference r383 (scope relative) (span (offset 45774) (line 967) (column 23) (len 12)) (segments (segment 0 (token "isOrthogonal") (name "isOrthogonal") (separator none) (span (offset 45774) (line 967) (column 23) (len 12)))))
    (reference r384 (scope relative) (span (offset 45824) (line 968) (column 30) (len 13)) (segments (segment 0 (token "RepetencyUnit") (name "RepetencyUnit") (separator none) (span (offset 45824) (line 968) (column 30) (len 13)))))
    (reference r385 (scope relative) (span (offset 45817) (line 968) (column 23) (len 5)) (segments (segment 0 (token "mRefs") (name "mRefs") (separator none) (span (offset 45817) (line 968) (column 23) (len 5)))))
    (reference r386 (scope relative) (span (offset 45962) (line 972) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 45962) (line 972) (column 44) (len 19)))))
    (reference r387 (scope relative) (span (offset 46509) (line 985) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 46509) (line 985) (column 28) (len 4)))))
    (reference r388 (scope relative) (span (offset 46504) (line 985) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 46504) (line 985) (column 23) (len 3)))))
    (reference r389 (scope relative) (span (offset 46543) (line 986) (column 29) (len 20)) (segments (segment 0 (token "AngularRepetencyUnit") (name "AngularRepetencyUnit") (separator none) (span (offset 46543) (line 986) (column 29) (len 20)))))
    (reference r390 (scope relative) (span (offset 46537) (line 986) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 46537) (line 986) (column 23) (len 4)))))
    (reference r391 (scope relative) (span (offset 46607) (line 989) (column 33) (len 21)) (segments (segment 0 (token "AngularRepetencyValue") (name "AngularRepetencyValue") (separator none) (span (offset 46607) (line 989) (column 33) (len 21)))))
    (reference r392 (scope relative) (span (offset 46706) (line 991) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 46706) (line 991) (column 43) (len 11)))))
    (reference r393 (scope relative) (span (offset 46756) (line 992) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 46756) (line 992) (column 37) (len 19)))))
    (reference r394 (scope relative) (span (offset 46785) (line 992) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 46785) (line 992) (column 66) (len 8)))))
    (reference r395 (scope relative) (span (offset 46796) (line 992) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 46796) (line 992) (column 77) (len 3)))))
    (reference r396 (scope relative) (span (offset 46800) (line 992) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 46800) (line 992) (column 81) (len 1)))))
    (reference r397 (scope relative) (span (offset 46807) (line 992) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 46807) (line 992) (column 88) (len 8)))))
    (reference r398 (scope relative) (span (offset 46846) (line 993) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 46846) (line 993) (column 23) (len 17)))))
    (reference r399 (scope relative) (span (offset 46870) (line 993) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 46870) (line 993) (column 47) (len 20)))))
    (reference r400 (scope relative) (span (offset 46893) (line 993) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 46893) (line 993) (column 70) (len 8)))))
    (reference r401 (scope relative) (span (offset 46948) (line 996) (column 37) (len 20)) (segments (segment 0 (token "AngularRepetencyUnit") (name "AngularRepetencyUnit") (separator none) (span (offset 46948) (line 996) (column 37) (len 20)))))
    (reference r402 (scope relative) (span (offset 47007) (line 997) (column 38) (len 21)) (segments (segment 0 (token "AngularRepetencyValue") (name "AngularRepetencyValue") (separator none) (span (offset 47007) (line 997) (column 38) (len 21)))))
    (reference r403 (scope relative) (span (offset 47062) (line 998) (column 33) (len 16)) (segments (segment 0 (token "angularRepetency") (name "angularRepetency") (separator none) (span (offset 47062) (line 998) (column 33) (len 16)))))
    (reference r404 (scope relative) (span (offset 47183) (line 1001) (column 41) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 47183) (line 1001) (column 41) (len 19)))))
    (reference r405 (scope relative) (span (offset 48030) (line 1014) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 48030) (line 1014) (column 28) (len 4)))))
    (reference r406 (scope relative) (span (offset 48025) (line 1014) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 48025) (line 1014) (column 23) (len 3)))))
    (reference r407 (scope relative) (span (offset 48064) (line 1015) (column 29) (len 17)) (segments (segment 0 (token "PhaseVelocityUnit") (name "PhaseVelocityUnit") (separator none) (span (offset 48064) (line 1015) (column 29) (len 17)))))
    (reference r408 (scope relative) (span (offset 48058) (line 1015) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 48058) (line 1015) (column 23) (len 4)))))
    (reference r409 (scope relative) (span (offset 48122) (line 1018) (column 30) (len 18)) (segments (segment 0 (token "PhaseVelocityValue") (name "PhaseVelocityValue") (separator none) (span (offset 48122) (line 1018) (column 30) (len 18)))))
    (reference r410 (scope relative) (span (offset 48215) (line 1020) (column 40) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 48215) (line 1020) (column 40) (len 11)))))
    (reference r411 (scope relative) (span (offset 48265) (line 1021) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 48265) (line 1021) (column 37) (len 19)))))
    (reference r412 (scope relative) (span (offset 48294) (line 1021) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 48294) (line 1021) (column 66) (len 8)))))
    (reference r413 (scope relative) (span (offset 48305) (line 1021) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 48305) (line 1021) (column 77) (len 3)))))
    (reference r414 (scope relative) (span (offset 48309) (line 1021) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 48309) (line 1021) (column 81) (len 1)))))
    (reference r415 (scope relative) (span (offset 48316) (line 1021) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 48316) (line 1021) (column 88) (len 8)))))
    (reference r416 (scope relative) (span (offset 48370) (line 1022) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 48370) (line 1022) (column 39) (len 19)))))
    (reference r417 (scope relative) (span (offset 48399) (line 1022) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 48399) (line 1022) (column 68) (len 8)))))
    (reference r418 (scope relative) (span (offset 48410) (line 1022) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 48410) (line 1022) (column 79) (len 3)))))
    (reference r419 (scope relative) (span (offset 48414) (line 1022) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 48414) (line 1022) (column 83) (len 1)))))
    (reference r420 (scope relative) (span (offset 48421) (line 1022) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 48421) (line 1022) (column 90) (len 8)))))
    (reference r421 (scope relative) (span (offset 48460) (line 1023) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 48460) (line 1023) (column 23) (len 17)))))
    (reference r422 (scope relative) (span (offset 48484) (line 1023) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 48484) (line 1023) (column 47) (len 20)))))
    (reference r423 (scope relative) (span (offset 48508) (line 1023) (column 71) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 48508) (line 1023) (column 71) (len 8)))))
    (reference r424 (scope relative) (span (offset 48518) (line 1023) (column 81) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 48518) (line 1023) (column 81) (len 10)))))
    (reference r425 (scope relative) (span (offset 48569) (line 1026) (column 30) (len 17)) (segments (segment 0 (token "PhaseVelocityUnit") (name "PhaseVelocityUnit") (separator none) (span (offset 48569) (line 1026) (column 30) (len 17)))))
    (reference r426 (scope relative) (span (offset 48618) (line 1027) (column 31) (len 18)) (segments (segment 0 (token "PhaseVelocityValue") (name "PhaseVelocityValue") (separator none) (span (offset 48618) (line 1027) (column 31) (len 18)))))
    (reference r427 (scope relative) (span (offset 48663) (line 1028) (column 26) (len 13)) (segments (segment 0 (token "phaseVelocity") (name "phaseVelocity") (separator none) (span (offset 48663) (line 1028) (column 26) (len 13)))))
    (reference r428 (scope relative) (span (offset 48770) (line 1031) (column 30) (len 10)) (segments (segment 0 (token "SpeedValue") (name "SpeedValue") (separator none) (span (offset 48770) (line 1031) (column 30) (len 10)))))
    (reference r429 (scope relative) (span (offset 49422) (line 1046) (column 26) (len 13)) (segments (segment 0 (token "groupVelocity") (name "groupVelocity") (separator none) (span (offset 49422) (line 1046) (column 26) (len 13)))))
    (reference r430 (scope relative) (span (offset 49535) (line 1049) (column 46) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 49535) (line 1049) (column 46) (len 19)))))
    (reference r431 (scope relative) (span (offset 49997) (line 1062) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 49997) (line 1062) (column 28) (len 4)))))
    (reference r432 (scope relative) (span (offset 49992) (line 1062) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 49992) (line 1062) (column 23) (len 3)))))
    (reference r433 (scope relative) (span (offset 50031) (line 1063) (column 29) (len 22)) (segments (segment 0 (token "DampingCoefficientUnit") (name "DampingCoefficientUnit") (separator none) (span (offset 50031) (line 1063) (column 29) (len 22)))))
    (reference r434 (scope relative) (span (offset 50025) (line 1063) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 50025) (line 1063) (column 23) (len 4)))))
    (reference r435 (scope relative) (span (offset 50099) (line 1066) (column 35) (len 23)) (segments (segment 0 (token "DampingCoefficientValue") (name "DampingCoefficientValue") (separator none) (span (offset 50099) (line 1066) (column 35) (len 23)))))
    (reference r436 (scope relative) (span (offset 50202) (line 1068) (column 45) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 50202) (line 1068) (column 45) (len 11)))))
    (reference r437 (scope relative) (span (offset 50254) (line 1069) (column 39) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 50254) (line 1069) (column 39) (len 19)))))
    (reference r438 (scope relative) (span (offset 50283) (line 1069) (column 68) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 50283) (line 1069) (column 68) (len 8)))))
    (reference r439 (scope relative) (span (offset 50294) (line 1069) (column 79) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 50294) (line 1069) (column 79) (len 3)))))
    (reference r440 (scope relative) (span (offset 50298) (line 1069) (column 83) (len 1)) (segments (segment 0 (token "T") (name "T") (separator none) (span (offset 50298) (line 1069) (column 83) (len 1)))))
    (reference r441 (scope relative) (span (offset 50305) (line 1069) (column 90) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 50305) (line 1069) (column 90) (len 8)))))
    (reference r442 (scope relative) (span (offset 50344) (line 1070) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 50344) (line 1070) (column 23) (len 17)))))
    (reference r443 (scope relative) (span (offset 50368) (line 1070) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 50368) (line 1070) (column 47) (len 20)))))
    (reference r444 (scope relative) (span (offset 50391) (line 1070) (column 70) (len 10)) (segments (segment 0 (token "durationPF") (name "durationPF") (separator none) (span (offset 50391) (line 1070) (column 70) (len 10)))))
    (reference r445 (scope relative) (span (offset 50513) (line 1074) (column 48) (len 17)) (segments (segment 0 (token "DimensionOneValue") (name "DimensionOneValue") (separator none) (span (offset 50513) (line 1074) (column 48) (len 17)))))
    (reference r446 (scope relative) (span (offset 51018) (line 1088) (column 37) (len 25)) (segments (segment 0 (token "LogarithmicDecrementValue") (name "LogarithmicDecrementValue") (separator none) (span (offset 51018) (line 1088) (column 37) (len 25)))))
    (reference r447 (scope relative) (span (offset 51162) (line 1091) (column 39) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 51162) (line 1091) (column 39) (len 19)))))
    (reference r448 (scope relative) (span (offset 51838) (line 1104) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 51838) (line 1104) (column 28) (len 4)))))
    (reference r449 (scope relative) (span (offset 51833) (line 1104) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 51833) (line 1104) (column 23) (len 3)))))
    (reference r450 (scope relative) (span (offset 51872) (line 1105) (column 29) (len 15)) (segments (segment 0 (token "AttenuationUnit") (name "AttenuationUnit") (separator none) (span (offset 51872) (line 1105) (column 29) (len 15)))))
    (reference r451 (scope relative) (span (offset 51866) (line 1105) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 51866) (line 1105) (column 23) (len 4)))))
    (reference r452 (scope relative) (span (offset 51926) (line 1108) (column 28) (len 16)) (segments (segment 0 (token "AttenuationValue") (name "AttenuationValue") (separator none) (span (offset 51926) (line 1108) (column 28) (len 16)))))
    (reference r453 (scope relative) (span (offset 52015) (line 1110) (column 38) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 52015) (line 1110) (column 38) (len 11)))))
    (reference r454 (scope relative) (span (offset 52065) (line 1111) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 52065) (line 1111) (column 37) (len 19)))))
    (reference r455 (scope relative) (span (offset 52094) (line 1111) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 52094) (line 1111) (column 66) (len 8)))))
    (reference r456 (scope relative) (span (offset 52105) (line 1111) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 52105) (line 1111) (column 77) (len 3)))))
    (reference r457 (scope relative) (span (offset 52109) (line 1111) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 52109) (line 1111) (column 81) (len 1)))))
    (reference r458 (scope relative) (span (offset 52116) (line 1111) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 52116) (line 1111) (column 88) (len 8)))))
    (reference r459 (scope relative) (span (offset 52155) (line 1112) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 52155) (line 1112) (column 23) (len 17)))))
    (reference r460 (scope relative) (span (offset 52179) (line 1112) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 52179) (line 1112) (column 47) (len 20)))))
    (reference r461 (scope relative) (span (offset 52202) (line 1112) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 52202) (line 1112) (column 70) (len 8)))))
    (reference r462 (scope relative) (span (offset 52250) (line 1115) (column 30) (len 15)) (segments (segment 0 (token "AttenuationUnit") (name "AttenuationUnit") (separator none) (span (offset 52250) (line 1115) (column 30) (len 15)))))
    (reference r463 (scope relative) (span (offset 52297) (line 1116) (column 31) (len 16)) (segments (segment 0 (token "AttenuationValue") (name "AttenuationValue") (separator none) (span (offset 52297) (line 1116) (column 31) (len 16)))))
    (reference r464 (scope relative) (span (offset 52340) (line 1117) (column 26) (len 11)) (segments (segment 0 (token "attenuation") (name "attenuation") (separator none) (span (offset 52340) (line 1117) (column 26) (len 11)))))
    (reference r465 (scope relative) (span (offset 52449) (line 1120) (column 44) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 52449) (line 1120) (column 44) (len 19)))))
    (reference r466 (scope relative) (span (offset 53098) (line 1133) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 53098) (line 1133) (column 28) (len 4)))))
    (reference r467 (scope relative) (span (offset 53093) (line 1133) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 53093) (line 1133) (column 23) (len 3)))))
    (reference r468 (scope relative) (span (offset 53132) (line 1134) (column 29) (len 20)) (segments (segment 0 (token "PhaseCoefficientUnit") (name "PhaseCoefficientUnit") (separator none) (span (offset 53132) (line 1134) (column 29) (len 20)))))
    (reference r469 (scope relative) (span (offset 53126) (line 1134) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 53126) (line 1134) (column 23) (len 4)))))
    (reference r470 (scope relative) (span (offset 53196) (line 1137) (column 33) (len 21)) (segments (segment 0 (token "PhaseCoefficientValue") (name "PhaseCoefficientValue") (separator none) (span (offset 53196) (line 1137) (column 33) (len 21)))))
    (reference r471 (scope relative) (span (offset 53295) (line 1139) (column 43) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 53295) (line 1139) (column 43) (len 11)))))
    (reference r472 (scope relative) (span (offset 53345) (line 1140) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 53345) (line 1140) (column 37) (len 19)))))
    (reference r473 (scope relative) (span (offset 53374) (line 1140) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 53374) (line 1140) (column 66) (len 8)))))
    (reference r474 (scope relative) (span (offset 53385) (line 1140) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 53385) (line 1140) (column 77) (len 3)))))
    (reference r475 (scope relative) (span (offset 53389) (line 1140) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 53389) (line 1140) (column 81) (len 1)))))
    (reference r476 (scope relative) (span (offset 53396) (line 1140) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 53396) (line 1140) (column 88) (len 8)))))
    (reference r477 (scope relative) (span (offset 53435) (line 1141) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 53435) (line 1141) (column 23) (len 17)))))
    (reference r478 (scope relative) (span (offset 53459) (line 1141) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 53459) (line 1141) (column 47) (len 20)))))
    (reference r479 (scope relative) (span (offset 53482) (line 1141) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 53482) (line 1141) (column 70) (len 8)))))
    (reference r480 (scope relative) (span (offset 53608) (line 1145) (column 50) (len 19)) (segments (segment 0 (token "ScalarQuantityValue") (name "ScalarQuantityValue") (separator none) (span (offset 53608) (line 1145) (column 50) (len 19)))))
    (reference r481 (scope relative) (span (offset 54271) (line 1158) (column 28) (len 4)) (segments (segment 0 (token "Real") (name "Real") (separator none) (span (offset 54271) (line 1158) (column 28) (len 4)))))
    (reference r482 (scope relative) (span (offset 54266) (line 1158) (column 23) (len 3)) (segments (segment 0 (token "num") (name "num") (separator none) (span (offset 54266) (line 1158) (column 23) (len 3)))))
    (reference r483 (scope relative) (span (offset 54305) (line 1159) (column 29) (len 26)) (segments (segment 0 (token "PropagationCoefficientUnit") (name "PropagationCoefficientUnit") (separator none) (span (offset 54305) (line 1159) (column 29) (len 26)))))
    (reference r484 (scope relative) (span (offset 54299) (line 1159) (column 23) (len 4)) (segments (segment 0 (token "mRef") (name "mRef") (separator none) (span (offset 54299) (line 1159) (column 23) (len 4)))))
    (reference r485 (scope relative) (span (offset 54381) (line 1162) (column 39) (len 27)) (segments (segment 0 (token "PropagationCoefficientValue") (name "PropagationCoefficientValue") (separator none) (span (offset 54381) (line 1162) (column 39) (len 27)))))
    (reference r486 (scope relative) (span (offset 54492) (line 1164) (column 49) (len 11)) (segments (segment 0 (token "DerivedUnit") (name "DerivedUnit") (separator none) (span (offset 54492) (line 1164) (column 49) (len 11)))))
    (reference r487 (scope relative) (span (offset 54542) (line 1165) (column 37) (len 19)) (segments (segment 0 (token "QuantityPowerFactor") (name "QuantityPowerFactor") (separator none) (span (offset 54542) (line 1165) (column 37) (len 19)))))
    (reference r488 (scope relative) (span (offset 54571) (line 1165) (column 66) (len 8)) (segments (segment 0 (token "quantity") (name "quantity") (separator none) (span (offset 54571) (line 1165) (column 66) (len 8)))))
    (reference r489 (scope relative) (span (offset 54582) (line 1165) (column 77) (len 3)) (segments (segment 0 (token "isq") (name "isq") (separator none) (span (offset 54582) (line 1165) (column 77) (len 3)))))
    (reference r490 (scope relative) (span (offset 54586) (line 1165) (column 81) (len 1)) (segments (segment 0 (token "L") (name "L") (separator none) (span (offset 54586) (line 1165) (column 81) (len 1)))))
    (reference r491 (scope relative) (span (offset 54593) (line 1165) (column 88) (len 8)) (segments (segment 0 (token "exponent") (name "exponent") (separator none) (span (offset 54593) (line 1165) (column 88) (len 8)))))
    (reference r492 (scope relative) (span (offset 54632) (line 1166) (column 23) (len 17)) (segments (segment 0 (token "quantityDimension") (name "quantityDimension") (separator none) (span (offset 54632) (line 1166) (column 23) (len 17)))))
    (reference r493 (scope relative) (span (offset 54656) (line 1166) (column 47) (len 20)) (segments (segment 0 (token "quantityPowerFactors") (name "quantityPowerFactors") (separator none) (span (offset 54656) (line 1166) (column 47) (len 20)))))
    (reference r494 (scope relative) (span (offset 54679) (line 1166) (column 70) (len 8)) (segments (segment 0 (token "lengthPF") (name "lengthPF") (separator none) (span (offset 54679) (line 1166) (column 70) (len 8)))))
  )
  (root (library-package (name "ISQSpaceTime") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 54) (line 3) (column 7) (len 707)) (normalized "International System of Quantities and Units\nGenerated on 2025-03-13T15:00:05Z from standard ISO-80000-3:2019 \"Space and Time\"\nsee also https://www.iso.org/standard/64974.html\n\nNote 1: In documentation comments, AsciiMath notation (see http://asciimath.org/) is used for mathematical concepts,\nwith Greek letters in Unicode encoding. In running text, AsciiMath is placed between backticks.\nNote 2: For vector and tensor quantities currently the unit and quantity value type for their (scalar) magnitude is \ndefined, as well as their typical Cartesian 3d VectorMeasurementReference (i.e. coordinate system) \nor TensorMeasurementReference.\n"))) (import (target (span (span (offset 784) (line 15) (column 20) (len 18))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 823) (line 16) (column 20) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 833) (line 16) (column 30) (len 3))) (separator (span (offset 833) (line 16) (column 30) (len 2))) (marker (span (offset 835) (line 16) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 857) (line 17) (column 20) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 878) (line 17) (column 41) (len 3))) (separator (span (offset 878) (line 17) (column 41) (len 2))) (marker (span (offset 880) (line 17) (column 43) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 902) (line 18) (column 20) (len 10))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 909) (line 18) (column 27) (len 3))) (separator (span (offset 909) (line 18) (column 27) (len 2))) (marker (span (offset 911) (line 18) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 921) (line 20) (column 7) (len 31)) (normalized "ISO-80000-3 item 3-1.1 length "))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 961) (line 21) (column 7) (len 72)) (normalized "See package ISQBase for the declarations of LengthValue and LengthUnit "))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1043) (line 23) (column 7) (len 39)) (normalized "ISO-80000-3 item 3-1.2 width, breadth "))) (attribute-def (declaration-name "width") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1162) (line 26) (column 11) (len 508)) (normalized "source: item 3-1.2 width, breadth\nsymbol(s): `b`, `B`\napplication domain: generic\nname: Width (specializes Length)\nquantity dimension: L^1\nmeasurement unit(s): m\ntensor order: 0\ndefinition: minimum length of a straight line segment between two parallel straight lines (in two dimensions) or planes (in three dimensions) that enclose a given geometrical shape\nremarks: This quantity is non-negative.\n"))))) (alias (name "breadth") (target (ref r5)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 1716) (line 41) (column 7) (len 48)) (normalized "ISO-80000-3 item 3-1.3 height, depth, altitude "))) (attribute-def (declaration-name "height") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 1845) (line 44) (column 11) (len 660)) (normalized "source: item 3-1.3 height, depth, altitude\nsymbol(s): `h`, `H`\napplication domain: generic\nname: Height (specializes Length)\nquantity dimension: L^1\nmeasurement unit(s): m\ntensor order: 0\ndefinition: minimum length of a straight line segment between a point and a reference line or reference surface\nremarks: This quantity is usually signed. The sign expresses the position of the particular point with respect to the reference line or surface and is chosen by convention. The symbol `H` is often used to denote altitude, i.e. height above sea level.\n"))))) (alias (name "depth") (target (ref r7)) (body semicolon)) (alias (name "altitude") (target (ref r8)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 2582) (line 61) (column 7) (len 34)) (normalized "ISO-80000-3 item 3-1.4 thickness "))) (attribute-def (declaration-name "thickness") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 2700) (line 64) (column 11) (len 358)) (normalized "source: item 3-1.4 thickness\nsymbol(s): `d`, `δ`\napplication domain: generic\nname: Thickness (specializes Length)\nquantity dimension: L^1\nmeasurement unit(s): m\ntensor order: 0\ndefinition: width (item 3-1.2)\nremarks: This quantity is non-negative.\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 3074) (line 77) (column 7) (len 33)) (normalized "ISO-80000-3 item 3-1.5 diameter "))) (attribute-def (declaration-name "diameter") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 3190) (line 80) (column 11) (len 387)) (normalized "source: item 3-1.5 diameter\nsymbol(s): `d`, `D`\napplication domain: generic\nname: Diameter (specializes Length)\nquantity dimension: L^1\nmeasurement unit(s): m\ntensor order: 0\ndefinition: width (item 3-1.2) of a circle, cylinder or sphere\nremarks: This quantity is non-negative.\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 3593) (line 93) (column 7) (len 31)) (normalized "ISO-80000-3 item 3-1.6 radius "))) (attribute-def (declaration-name "radius") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 3705) (line 96) (column 11) (len 364)) (normalized "source: item 3-1.6 radius\nsymbol(s): `r`, `R`\napplication domain: generic\nname: Radius (specializes Length)\nquantity dimension: L^1\nmeasurement unit(s): m\ntensor order: 0\ndefinition: half of a diameter (item 3-1.5)\nremarks: This quantity is non-negative.\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 4085) (line 109) (column 7) (len 48)) (normalized "ISO-80000-3 item 3-1.7 path length, arc length "))) (attribute-def (declaration-name "pathLength") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 4218) (line 112) (column 11) (len 634)) (normalized "source: item 3-1.7 path length, arc length\nsymbol(s): `s`\napplication domain: generic\nname: PathLength (specializes Length)\nquantity dimension: L^1\nmeasurement unit(s): m\ntensor order: 0\ndefinition: length of a rectifiable curve between two of its points\nremarks: The differential path length at a given point of a curve is: `ds = sqrt(dx^2 + dy^2 + dz^2)` where `x`, `y`, and `z` denote the Cartesian coordinates (ISO 80000-2) of the particular point. There are curves which are not rectifiable, for example fractal curves.\n"))))) (alias (name "arcLength") (target (ref r13)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 4905) (line 127) (column 7) (len 33)) (normalized "ISO-80000-3 item 3-1.8 distance "))) (attribute-def (declaration-name "distance") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 5021) (line 130) (column 11) (len 570)) (normalized "source: item 3-1.8 distance\nsymbol(s): `d`, `r`\napplication domain: generic\nname: Distance (specializes Length)\nquantity dimension: L^1\nmeasurement unit(s): m\ntensor order: 0\ndefinition: shortest path length (item 3-1.7) between two points in a metric space\nremarks: A metric space might be curved. An example of a curved metric space is the surface of the Earth. In this case, distances are measured along great circles. A metric is not necessarily Euclidean.\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 5607) (line 143) (column 7) (len 40)) (normalized "ISO-80000-3 item 3-1.9 radial distance "))) (attribute-def (declaration-name "radialDistance") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 5736) (line 146) (column 11) (len 688)) (normalized "source: item 3-1.9 radial distance\nsymbol(s): `r_Q`, `ρ`\napplication domain: generic\nname: RadialDistance (specializes Length)\nquantity dimension: L^1\nmeasurement unit(s): m\ntensor order: 0\ndefinition: distance (item 3-1.8), where one point is located on an axis or within a closed non self-intersecting curve or surface\nremarks: The subscript Q denotes the point from which the radial distance is measured. Examples of closed non self-intersecting curves are circles or ellipses. Examples of closed non self-intersecting surfaces are surfaces of spheres or egg-shaped objects.\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 6440) (line 159) (column 7) (len 27)) (normalized "Spatial coordinate frames "))) (attribute-def (declaration-name "Spatial3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r16)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 6565) (line 163) (column 11) (len 62)) (normalized "Most general spatial 3D coordinate frame\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r17)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 6662) (line 166) (column 33) (len 4)) (boolean true))))) (body semicolon)))) (attribute-def (declaration-name "CartesianSpatial3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r18)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 6779) (line 171) (column 11) (len 551)) (normalized "Cartesian spatial 3D coordinate frame\n\nsource: ISO 80000-2 item 2-17.1 Cartesian coordinates\n\nThe components of a vector expressed on a Cartesian spatial coordinate frame are all LengthValues, and denoted with symbols `x`, `y`, `z`.\n\nNote 1: The Cartesian basis vectors `vec(e_x)`, `vec(e_y)` and `vec(e_z)` form an orthonormal right-handed coordinate frame.\nNote 2: The measurement units for the 3 dimensions are typically the same, but may be different.\n"))) (attribute-usage (declaration-name "xUnit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7372) (line 181) (column 40) (len 9)) (index (base (expression (span (offset 7372) (line 181) (column 40) (len 5)) (ref r20))) (index (expression (span (offset 7379) (line 181) (column 47) (len 1)) (integer 1)))))))) (body semicolon)) (attribute-usage (declaration-name "yUnit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r21)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7422) (line 182) (column 40) (len 9)) (index (base (expression (span (offset 7422) (line 182) (column 40) (len 5)) (ref r22))) (index (expression (span (offset 7429) (line 182) (column 47) (len 1)) (integer 2)))))))) (body semicolon)) (attribute-usage (declaration-name "zUnit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7472) (line 183) (column 40) (len 9)) (index (base (expression (span (offset 7472) (line 183) (column 40) (len 5)) (ref r24))) (index (expression (span (offset 7479) (line 183) (column 47) (len 1)) (integer 3)))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r25)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r26)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r27)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 7565) (line 185) (column 38) (len 4)) (boolean true))))) (body semicolon)))) (attribute-def (declaration-name "universalCartesianSpatial3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r28)))) (multiplicity (lower (expression (span (offset 7671) (line 188) (column 94) (len 1)) (integer 1))) (upper (expression (span (offset 7671) (line 188) (column 94) (len 1)) (integer 1)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 7698) (line 190) (column 11) (len 138)) (normalized "A singleton CartesianSpatial3dCoordinateFrame that can be used as a default universal Cartesian 3D coordinate frame.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r29)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 7885) (line 194) (column 37) (len 21)) (tuple (expression (span (offset 7886) (line 194) (column 38) (len 5)) (ref r30)) (expression (span (offset 7893) (line 194) (column 45) (len 5)) (ref r31)) (expression (span (offset 7900) (line 194) (column 52) (len 5)) (ref r32))))))) (body brace (doc (name none) (locale none) (body (span (offset 7927) (line 195) (column 19) (len 132)) (normalized "By default, the universalCartesianSpatial3dCoordinateFrame uses meters as the units on all three axes.\n"))))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r33)))) (references none) (crosses none) (intersects none) (value none) (body brace (doc (name none) (locale none) (body (span (offset 8144) (line 201) (column 19) (len 144)) (normalized "The universalCartesianSpatial3dCoordinateFrame is the \"top-level\" coordinate frame, not nested in any other frame.\n"))))))) (attribute-def (declaration-name "CylindricalSpatial3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r34)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 8423) (line 210) (column 11) (len 1454)) (normalized "Cylindrical spatial 3D coordinate frame\n\nsource: ISO 80000-2 item 2-17.2 cylindrical coordinates\n\nThe components of a (position) vector to a point P in a cylindrical coordinate frame are:\n- radialDistance (symbol `ρ`) defined by LengthValue, that is the radial distance from the cylinder axis to P\n- azimuth (symbol `φ`) defined by AngularMeasure, that is the angle between the azimuth reference direction and the line segment\nfrom the cylinder axis, in the plane that is orthogonal to the cylinder axis and intersects P\n- z coordinate (symbol `z`) defined by LengthValue, the coordinate along the clyinder axis.\n\nNote 1: The basis vectors `vec(e_ρ)(φ)`, `vec(e_φ)(φ)` and `vec(e_z)` form an orthonormal right-handed coordinate frame, where\n`vec(e_φ)` is tangent to the circular arc in the `φ` direction.\nNote 2: In order to enable transformation to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned\nwith the `φ=0` direction in the cylindrical frame, and the `vec(e_z)` Cartesian basis vector is aligned with\nthe `vec(e_z)` cylindrical basis vector.\nNote 3: If `z = 0`, then `ρ` and `φ` are polar coordinates in the XY-plane.\nNote 4: See also https://en.wikipedia.org/wiki/Cylindrical_coordinate_system .\n"))) (attribute-usage (declaration-name "radialDistanceUnit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r35)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "azimuthUnit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r36)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "zUnit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r37)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r38)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10051) (line 232) (column 31) (len 40)) (tuple (expression (span (offset 10052) (line 232) (column 32) (len 18)) (ref r39)) (expression (span (offset 10072) (line 232) (column 52) (len 11)) (ref r40)) (expression (span (offset 10085) (line 232) (column 65) (len 5)) (ref r41))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r42)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 10130) (line 233) (column 38) (len 4)) (boolean true))))) (body semicolon)))) (attribute-def (declaration-name "SphericalSpatial3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r43)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 10247) (line 238) (column 11) (len 1554)) (normalized "Spherical spatial 3D coordinate frame\n\nsource: ISO 80000-2 item 2-17.3 spherical coordinates\n\nThe components of a (position) vector to a point P specified in a spherical coordinate frame are:\n- radialDistance (symbol `r`) defined by LengthValue, that is the distance from the origin to P\n- inclination (symbol `θ`) defined by AngularMeasure, that is the angle between the zenith direction and the line segment from origin to P\n- azimuth (symbol `φ`) defined by AngularMeasure, that is the angle between the azimuth reference direction and the line segment\nfrom the origin to the orthogonal projection of P on the reference plane, normal to the zenith direction.\n\nNote 1: The basis vectors `vec(e_r)(θ,φ)`, `vec(e_θ)(θ,φ)` and `vec(e_φ)(φ)` form an orthonormal right-handed frame, where\n`vec(e_θ)` and `vec(e_φ)` are tangent to the respective circular arcs in the `θ` and `φ` directions.\nNote 2: In order to transform to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned\nwith the `θ=π/4` and `φ=0` direction in the spherical frame, and the `vec(e_z)` Cartesian basis vector is aligned\nwith the `θ=0` zenith direction in the spherical frame.\nNote 3: If `θ = π/4`, then `ρ` and `φ` are polar coordinates in the XY-plane.\nNote 4: See also https://en.wikipedia.org/wiki/Spherical_coordinate_system .\n"))) (attribute-usage (declaration-name "radialDistanceUnit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r44)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "inclinationUnit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r45)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "azimuthUnit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r46)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r47)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 11993) (line 260) (column 31) (len 50)) (tuple (expression (span (offset 11994) (line 260) (column 32) (len 18)) (ref r48)) (expression (span (offset 12014) (line 260) (column 52) (len 15)) (ref r49)) (expression (span (offset 12031) (line 260) (column 69) (len 11)) (ref r50))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r51)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 12082) (line 261) (column 38) (len 4)) (boolean true))))) (body semicolon)))) (attribute-def (declaration-name "PlanetarySpatial3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r52)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 12200) (line 266) (column 11) (len 2870)) (normalized "Planetary spatial 3D coordinate frame\n\nA planetary spatial 3D coordinate frame is a generalization for any planet of the geographic coordinate frame and geocentric coordinate\nfor Earth. In such coordinate frames, typically the origin is located at the planet's centre of gravity, and the surface of the planet\nis approximated by a reference ellipsoid centred on the origin, with its major axes oriented along the south to north pole vector and\nthe equatorial plane.\n\nThe components of a (position) vector to a point P specified in a planetary coordinate frame are:\n- latitude (symbol `lat` or `φ`) defined by AngularMeasure, that is the angle between the equatorial plane and the vector from\n  the origin to P, similar to the inclination in a spherical spatial coordinate frame. Typically, the zero reference latitude is chosen\n  for positions in the equatorial plane, with positive latitude for positions in the northern hemisphere and negative latitude for positions\n  in the southern hemisphere.\n- longitude (symbol `long` or `λ`) defined by AngularMeasure, that is the angle between a reference meridian and the meridian\n  passing through P, similar to the azimuth of a spherical spatial coordinate frame. The convention is to connotate positive longitude\n  with eastward direction and negative longitude with westward direction. The reference meridian for `long=0` is chosen to pass\n  through a particular feature of the planet, e.g., for Earth typically the position of the British Royal Observatory in Greenwich, UK.\n- altitude (symbol `h`) defined by LengthValue, that is the distance between P and the reference ellipsoid\n  in the normal direction to the ellipsoid. Positive altitude specifies a position above the reference ellipsoid surface,\n  while a negative value specifies a position below.\n\nNote 1: The reference meridian is also called prime meridian.\nNote 2: The basis vectors `vec(e_φ)(φ)`, `vec(e_λ)(λ)` and `vec(e_h)(φ,λ)` form an orthonormal right-handed frame, where\n`vec(e_φ)` and `vec(e_λ)` are tangent to the reference ellipsoid in the respective latitude and longitude directions,\nand `vec(e_h)` is normal to the reference ellipsoid.\nNote 3: In order to transform to and from a CartesianSpatial3dCoordinateFrame the `vec(e_x)` Cartesian basis vector is aligned\nwith the `φ=0` and `λ=0` direction in the planetary frame, and the `vec(e_z)` Cartesian basis vector is aligned\nwith the `λ=π/2` (north pole) direction in the planetary frame.\nNote 4: See also https://en.wikipedia.org/wiki/Planetary_coordinate_system .\n"))) (attribute-usage (declaration-name "latitudeUnit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r53)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "longitudeUnit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r54)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "altitudeUnit") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r55)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r56)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15255) (line 299) (column 31) (len 43)) (tuple (expression (span (offset 15256) (line 299) (column 32) (len 13)) (ref r57)) (expression (span (offset 15271) (line 299) (column 47) (len 12)) (ref r58)) (expression (span (offset 15285) (line 299) (column 61) (len 12)) (ref r59))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r60)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 15337) (line 300) (column 38) (len 4)) (boolean true))))) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 15356) (line 303) (column 7) (len 41)) (normalized "ISO-80000-3 item 3-1.10 position vector "))) (attribute-def (declaration-name "Position3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r61)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 15486) (line 306) (column 11) (len 533)) (normalized "source: item 3-1.10 position vector\nsymbol(s): `vec(r)`\napplication domain: generic\nname: PositionVector\nquantity dimension: L^1\nmeasurement unit(s): m\ntensor order: 1\ndefinition: vector (ISO 80000-2) quantity from the origin of a coordinate system to a point in space\nremarks: Position vectors are so-called bounded vectors, i.e. their magnitude (ISO 80000-2) and direction depend on the particular coordinate system used.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r62)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16054) (line 317) (column 33) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r63)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r64)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "position3dVector") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r65)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianPosition3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r66)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "x") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r67)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16297) (line 324) (column 37) (len 7)) (index (base (expression (span (offset 16297) (line 324) (column 37) (len 3)) (ref r68))) (index (expression (span (offset 16302) (line 324) (column 42) (len 1)) (integer 1)))))))) (body semicolon)) (attribute-usage (declaration-name "y") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r69)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16359) (line 325) (column 37) (len 7)) (index (base (expression (span (offset 16359) (line 325) (column 37) (len 3)) (ref r70))) (index (expression (span (offset 16364) (line 325) (column 42) (len 1)) (integer 2)))))))) (body semicolon)) (attribute-usage (declaration-name "z") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r71)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16421) (line 326) (column 37) (len 7)) (index (base (expression (span (offset 16421) (line 326) (column 37) (len 3)) (ref r72))) (index (expression (span (offset 16426) (line 326) (column 42) (len 1)) (integer 3)))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r73)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r74)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianPosition3dVector") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r75)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "CylindricalPosition3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r76)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "radialDistance") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r77)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16734) (line 332) (column 57) (len 7)) (index (base (expression (span (offset 16734) (line 332) (column 57) (len 3)) (ref r78))) (index (expression (span (offset 16739) (line 332) (column 62) (len 1)) (integer 1)))))))) (body semicolon)) (attribute-usage (declaration-name "azimuth") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r79)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16816) (line 333) (column 57) (len 7)) (index (base (expression (span (offset 16816) (line 333) (column 57) (len 3)) (ref r80))) (index (expression (span (offset 16821) (line 333) (column 62) (len 1)) (integer 2)))))))) (body semicolon)) (attribute-usage (declaration-name "height") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r81)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 16887) (line 334) (column 46) (len 7)) (index (base (expression (span (offset 16887) (line 334) (column 46) (len 3)) (ref r82))) (index (expression (span (offset 16892) (line 334) (column 51) (len 1)) (integer 3)))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r83)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r84)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cylindricalPosition3dVector") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r85)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "SphericalPosition3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r86)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "radialDistance") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r87)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17201) (line 340) (column 54) (len 7)) (index (base (expression (span (offset 17201) (line 340) (column 54) (len 3)) (ref r88))) (index (expression (span (offset 17206) (line 340) (column 59) (len 1)) (integer 1)))))))) (body semicolon)) (attribute-usage (declaration-name "inclination") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r89)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17287) (line 341) (column 61) (len 7)) (index (base (expression (span (offset 17287) (line 341) (column 61) (len 3)) (ref r90))) (index (expression (span (offset 17292) (line 341) (column 66) (len 1)) (integer 2)))))))) (body semicolon)) (attribute-usage (declaration-name "azimuth") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r91)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17369) (line 342) (column 57) (len 7)) (index (base (expression (span (offset 17369) (line 342) (column 57) (len 3)) (ref r92))) (index (expression (span (offset 17374) (line 342) (column 62) (len 1)) (integer 3)))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r93)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r94)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "sphericalPosition3dVector") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r95)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "PlanetaryPosition3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r96)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "latitude") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r97)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17680) (line 348) (column 57) (len 7)) (index (base (expression (span (offset 17680) (line 348) (column 57) (len 3)) (ref r98))) (index (expression (span (offset 17685) (line 348) (column 62) (len 1)) (integer 1)))))))) (body semicolon)) (attribute-usage (declaration-name "longitude") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r99)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17764) (line 349) (column 59) (len 7)) (index (base (expression (span (offset 17764) (line 349) (column 59) (len 3)) (ref r100))) (index (expression (span (offset 17769) (line 349) (column 64) (len 1)) (integer 2)))))))) (body semicolon)) (attribute-usage (declaration-name "altitude") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r101)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 17837) (line 350) (column 48) (len 7)) (index (base (expression (span (offset 17837) (line 350) (column 48) (len 3)) (ref r102))) (index (expression (span (offset 17842) (line 350) (column 53) (len 1)) (integer 3)))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r103)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r104)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "planetaryPosition3dVector") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r105)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 18032) (line 355) (column 7) (len 38)) (normalized "ISO-80000-3 item 3-1.11 displacement "))) (attribute-def (declaration-name "Displacement3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r106)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 18163) (line 358) (column 11) (len 562)) (normalized "source: item 3-1.11 displacement\nsymbol(s): `vec(Δr)`\napplication domain: generic\nname: Displacement\nquantity dimension: L^1\nmeasurement unit(s): m\ntensor order: 1\ndefinition: vector (ISO 80000-2) quantity between any two points in space\nremarks: Displacement vectors are so-called free vectors, i.e. their magnitude (ISO 80000-2) and direction do not depend on a particular coordinate system. The magnitude of this vector is also called displacement.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r107)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 18760) (line 369) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r108)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r109)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "displacement3dVector") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r110)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianDisplacement3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r111)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "x") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r112)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19020) (line 376) (column 37) (len 7)) (index (base (expression (span (offset 19020) (line 376) (column 37) (len 3)) (ref r113))) (index (expression (span (offset 19025) (line 376) (column 42) (len 1)) (integer 1)))))))) (body semicolon)) (attribute-usage (declaration-name "y") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r114)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19082) (line 377) (column 37) (len 7)) (index (base (expression (span (offset 19082) (line 377) (column 37) (len 3)) (ref r115))) (index (expression (span (offset 19087) (line 377) (column 42) (len 1)) (integer 2)))))))) (body semicolon)) (attribute-usage (declaration-name "z") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r116)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19144) (line 378) (column 37) (len 7)) (index (base (expression (span (offset 19144) (line 378) (column 37) (len 3)) (ref r117))) (index (expression (span (offset 19149) (line 378) (column 42) (len 1)) (integer 3)))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r118)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r119)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianDisplacement3dVector") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r120)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "CylindricalDisplacement3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r121)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "radialDistance") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r122)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19477) (line 384) (column 57) (len 7)) (index (base (expression (span (offset 19477) (line 384) (column 57) (len 3)) (ref r123))) (index (expression (span (offset 19482) (line 384) (column 62) (len 1)) (integer 1)))))))) (body semicolon)) (attribute-usage (declaration-name "azimuth") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r124)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19559) (line 385) (column 57) (len 7)) (index (base (expression (span (offset 19559) (line 385) (column 57) (len 3)) (ref r125))) (index (expression (span (offset 19564) (line 385) (column 62) (len 1)) (integer 2)))))))) (body semicolon)) (attribute-usage (declaration-name "height") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r126)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19630) (line 386) (column 46) (len 7)) (index (base (expression (span (offset 19630) (line 386) (column 46) (len 3)) (ref r127))) (index (expression (span (offset 19635) (line 386) (column 51) (len 1)) (integer 3)))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r128)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r129)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cylindricalDisplacement3dVector") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r130)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "SphericalDisplacement3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r131)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "radialDistance") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r132)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 19964) (line 392) (column 54) (len 7)) (index (base (expression (span (offset 19964) (line 392) (column 54) (len 3)) (ref r133))) (index (expression (span (offset 19969) (line 392) (column 59) (len 1)) (integer 1)))))))) (body semicolon)) (attribute-usage (declaration-name "inclination") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r134)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20050) (line 393) (column 61) (len 7)) (index (base (expression (span (offset 20050) (line 393) (column 61) (len 3)) (ref r135))) (index (expression (span (offset 20055) (line 393) (column 66) (len 1)) (integer 2)))))))) (body semicolon)) (attribute-usage (declaration-name "azimuth") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r136)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 20132) (line 394) (column 57) (len 7)) (index (base (expression (span (offset 20132) (line 394) (column 57) (len 3)) (ref r137))) (index (expression (span (offset 20137) (line 394) (column 62) (len 1)) (integer 3)))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r138)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r139)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "sphericalDisplacement3dVector") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r140)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 20339) (line 399) (column 7) (len 45)) (normalized "ISO-80000-3 item 3-1.12 radius of curvature "))) (attribute-def (declaration-name "radiusOfCurvature") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r141)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 20476) (line 402) (column 11) (len 525)) (normalized "source: item 3-1.12 radius of curvature\nsymbol(s): `ρ`\napplication domain: generic\nname: RadiusOfCurvature (specializes Length)\nquantity dimension: L^1\nmeasurement unit(s): m\ntensor order: 0\ndefinition: radius (item 3-1.6) of the osculating circle of a planar curve at a particular point of the curve\nremarks: The radius of curvature is only defined for curves which are at least twice continuously differentiable.\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 21017) (line 415) (column 7) (len 32)) (normalized "ISO-80000-3 item 3-2 curvature "))) (attribute-def (declaration-name "CurvatureValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r142)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 21132) (line 418) (column 11) (len 430)) (normalized "source: item 3-2 curvature\nsymbol(s): `κ`\napplication domain: generic\nname: Curvature\nquantity dimension: L^-1\nmeasurement unit(s): m^-1\ntensor order: 0\ndefinition: inverse of the radius of curvature (item 3-1.12)\nremarks: The curvature is given by: `κ = 1/ρ` where `ρ` denotes the radius of curvature (item 3-1.12).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r143)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r144)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r145)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r146)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "curvature") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r147)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "CurvatureUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r148)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r149)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r150)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21851) (line 436) (column 77) (len 5)) (member-access (base (expression (span (offset 21851) (line 436) (column 77) (len 3)) (ref r151))) (separator dot) (member (ref r152))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r153)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21873) (line 436) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 21874) (line 436) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r154)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r155)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 21948) (line 437) (column 70) (len 8)) (ref r156))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 21973) (line 440) (column 7) (len 27)) (normalized "ISO-80000-3 item 3-3 area "))) (attribute-def (declaration-name "AreaValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r157)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 22078) (line 443) (column 11) (len 605)) (normalized "source: item 3-3 area\nsymbol(s): `A`, `S`\napplication domain: generic\nname: Area\nquantity dimension: L^2\nmeasurement unit(s): m^2\ntensor order: 0\ndefinition: extent of a two-dimensional geometrical shape\nremarks: The surface element at a given point of a surface is given by: `dA = g du dv` where `u` and `v` denote the Gaussian surface coordinates and `g` denotes the determinant of the metric tensor (ISO 80000-2) at the particular point. The symbol `dσ` is also used for the surface element.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r158)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r159)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r160)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r161)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "area") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r162)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "AreaUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r163)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r164)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r165)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22952) (line 461) (column 77) (len 5)) (member-access (base (expression (span (offset 22952) (line 461) (column 77) (len 3)) (ref r166))) (separator dot) (member (ref r167))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r168)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 22974) (line 461) (column 99) (len 1)) (integer 2))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r169)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r170)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 23048) (line 462) (column 70) (len 8)) (ref r171))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 23073) (line 465) (column 7) (len 29)) (normalized "ISO-80000-3 item 3-4 volume "))) (attribute-def (declaration-name "VolumeValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r172)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 23182) (line 468) (column 11) (len 547)) (normalized "source: item 3-4 volume\nsymbol(s): `V`, `(S)`\napplication domain: generic\nname: Volume\nquantity dimension: L^3\nmeasurement unit(s): m^3\ntensor order: 0\ndefinition: extent of a three-dimensional geometrical shape\nremarks: The volume element in Euclidean space is given by: `dV = dx dy dz` where `dx`, `dy`, and `dz` denote the differentials of the Cartesian coordinates (ISO 80000-2). The symbol `dτ` is also used for the volume element.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r173)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r174)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r175)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r176)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "volume") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r177)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "VolumeUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r178)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r179)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r180)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24006) (line 486) (column 77) (len 5)) (member-access (base (expression (span (offset 24006) (line 486) (column 77) (len 3)) (ref r181))) (separator dot) (member (ref r182))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r183)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24028) (line 486) (column 99) (len 1)) (integer 3))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r184)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r185)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 24102) (line 487) (column 70) (len 8)) (ref r186))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 24127) (line 490) (column 7) (len 51)) (normalized "ISO-80000-3 item 3-5 angular measure, plane angle "))) (attribute-def (declaration-name "AngularMeasureValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r187)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 24266) (line 493) (column 11) (len 737)) (normalized "source: item 3-5 angular measure, plane angle\nsymbol(s): `α`, `β`, `γ`\napplication domain: generic\nname: AngularMeasure\nquantity dimension: 1\nmeasurement unit(s): rad, 1\ntensor order: 0\ndefinition: measure of a geometric figure, called plane angle, formed by two rays, called the sides of the plane angle, emanating from a common point, called the vertex of the plane angle\nremarks: The angular measure is given by: `α = s/r` where `s` denotes the arc length (item 3-1.7) of the included arc of a circle, centred at the vertex of the plane angle, and `r` the radius (item 3-1.6) of that circle. Other symbols are also used.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r188)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r189)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r190)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r191)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "angularMeasure") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r192)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "AngularMeasureUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r193)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace)) (alias (name "PlaneAngleUnit") (target (ref r194)) (body semicolon)) (alias (name "PlaneAngleValue") (target (ref r195)) (body semicolon)) (alias (name "planeAngle") (target (ref r196)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 25396) (line 517) (column 7) (len 68)) (normalized "ISO-80000-3 item 3-6 rotational displacement, angular displacement "))) (attribute-def (declaration-name "rotationalDisplacement") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r197)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 25569) (line 520) (column 11) (len 903)) (normalized "source: item 3-6 rotational displacement, angular displacement\nsymbol(s): `ϑ`, `φ`\napplication domain: generic\nname: RotationalDisplacement (specializes AngularMeasure)\nquantity dimension: 1\nmeasurement unit(s): rad, 1\ntensor order: 0\ndefinition: quotient of the traversed circular path length (item 3-1.7) of a point in space during a rotation and its distance (item 3-1.8) from the axis or centre of rotation\nremarks: The rotational displacement is given by: `φ = s/r` where `s` denotes the traversed path length (item 3-1.7) along the periphery of a circle, centred at the vertex of the plane angle, and `r` the radius (item 3-1.6) of that circle. The rotational displacement is signed. The sign denotes the direction of rotation and is chosen by convention. Other symbols are also used.\n"))))) (alias (name "angularDisplacement") (target (ref r198)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 26547) (line 535) (column 7) (len 34)) (normalized "ISO-80000-3 item 3-7 phase angle "))) (attribute-def (declaration-name "phaseAngle") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r199)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 26674) (line 538) (column 11) (len 594)) (normalized "source: item 3-7 phase angle\nsymbol(s): `φ`, `ϕ`\napplication domain: generic\nname: PhaseAngle (specializes AngularMeasure)\nquantity dimension: 1\nmeasurement unit(s): rad, 1\ntensor order: 0\ndefinition: angular measure (item 3-5) between the positive real axis and the radius of the polar representation of the complex number in the complex plane\nremarks: The phase angle (often imprecisely referred to as the \"phase\") is the argument of a complex number. Other symbols are also used.\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 27284) (line 551) (column 7) (len 44)) (normalized "ISO-80000-3 item 3-8 solid angular measure "))) (attribute-def (declaration-name "SolidAngularMeasureValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r200)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 27421) (line 554) (column 11) (len 785)) (normalized "source: item 3-8 solid angular measure\nsymbol(s): `Ω`\napplication domain: generic\nname: SolidAngularMeasure\nquantity dimension: 1\nmeasurement unit(s): sr, 1\ntensor order: 0\ndefinition: measure of a conical geometric figure, called solid angle, formed by all rays, originating from a common point, called the vertex of the solid angle, and passing through the points of a closed, non-self-intersecting curve in space considered as the border of a surface\nremarks: The differential solid angular measure expressed in spherical coordinates (ISO 80000-2) is given by: `dΩ = A/r^2 * sin(θ * dθ * dφ)` where `A` is area, `r` is radius, `θ` and `φ` are spherical coordinates.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r201)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r202)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r203)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r204)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "solidAngularMeasure") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r205)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SolidAngularMeasureUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r206)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 28477) (line 574) (column 7) (len 37)) (normalized "ISO-80000-3 item 3-9 duration, time "))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 28523) (line 575) (column 7) (len 76)) (normalized "See package ISQBase for the declarations of DurationValue and DurationUnit "))) (alias (name "TimeUnit") (target (ref r207)) (body semicolon)) (alias (name "TimeValue") (target (ref r208)) (body semicolon)) (alias (name "time") (target (ref r209)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 28715) (line 581) (column 7) (len 34)) (normalized "ISO-80000-3 item 3-10.1 velocity "))) (attribute-def (declaration-name "CartesianVelocity3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r210)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 28847) (line 584) (column 11) (len 703)) (normalized "source: item 3-10.1 velocity\nsymbol(s): `vec(v)`, `u,v,w`\napplication domain: generic\nname: Velocity\nquantity dimension: L^1*T^-1\nmeasurement unit(s): m/s, m*s^-1\ntensor order: 1\ndefinition: vector (ISO 80000-2) quantity giving the rate of change of a position vector (item 3-1.10)\nremarks: The velocity vector is given by: `vec(v) = (d vec(r)) / (dt)` where `vec(r)` denotes the position vector (item 3-1.10) and `t` the duration (item 3-9). When the general symbol `vec(v)` is not used for the velocity, the symbols `u`, `v`, `w` may be used for the components (ISO 80000-2) of the velocity.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r211)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29585) (line 595) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r212)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r213)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianVelocity3dVector") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r214)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianVelocity3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r215)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r216)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29865) (line 602) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r217)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 29909) (line 603) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r218)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r219)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 29971) (line 607) (column 7) (len 31)) (normalized "ISO-80000-3 item 3-10.2 speed "))) (attribute-def (declaration-name "SpeedValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r220)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 30081) (line 610) (column 11) (len 349)) (normalized "source: item 3-10.2 speed\nsymbol(s): `v`\napplication domain: generic\nname: Speed\nquantity dimension: L^1*T^-1\nmeasurement unit(s): m/s, m*s^-1\ntensor order: 0\ndefinition: magnitude (ISO 80000-2) of the velocity (item 3-10.1)\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r221)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r222)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r223)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r224)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "speed") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r225)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "SpeedUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r226)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r227)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r228)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30703) (line 628) (column 77) (len 5)) (member-access (base (expression (span (offset 30703) (line 628) (column 77) (len 3)) (ref r229))) (separator dot) (member (ref r230))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r231)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30725) (line 628) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r232)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r233)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30808) (line 629) (column 79) (len 5)) (member-access (base (expression (span (offset 30808) (line 629) (column 79) (len 3)) (ref r234))) (separator dot) (member (ref r235))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r236)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30830) (line 629) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 30831) (line 629) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r237)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r238)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 30905) (line 630) (column 70) (len 22)) (tuple (expression (span (offset 30906) (line 630) (column 71) (len 8)) (ref r239)) (expression (span (offset 30916) (line 630) (column 81) (len 10)) (ref r240))))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 30944) (line 633) (column 7) (len 36)) (normalized "ISO-80000-3 item 3-11 acceleration "))) (attribute-def (declaration-name "AccelerationValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r241)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 31066) (line 636) (column 11) (len 623)) (normalized "source: item 3-11 acceleration (magnitude)\nsymbol(s): `a`\napplication domain: generic\nname: Acceleration\nquantity dimension: L^1*T^-2\nmeasurement unit(s): m*s^-2\ntensor order: 0\ndefinition: vector (ISO 80000-2) quantity giving the rate of change of velocity (item 3-10)\nremarks: The acceleration vector is given by: `vec(a) = (d vec(v))/(dt)` where `vec(v)` denotes the velocity (item 3-10.1) and `t` the duration (item 3-9). The magnitude (ISO 80000-2) of the acceleration of free fall is usually denoted by `g`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r242)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r243)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r244)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r245)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "acceleration") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r246)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "AccelerationUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r247)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r248)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r249)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 31990) (line 654) (column 77) (len 5)) (member-access (base (expression (span (offset 31990) (line 654) (column 77) (len 3)) (ref r250))) (separator dot) (member (ref r251))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r252)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32012) (line 654) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r253)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r254)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32095) (line 655) (column 79) (len 5)) (member-access (base (expression (span (offset 32095) (line 655) (column 79) (len 3)) (ref r255))) (separator dot) (member (ref r256))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r257)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32117) (line 655) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 32118) (line 655) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r258)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r259)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32192) (line 656) (column 70) (len 22)) (tuple (expression (span (offset 32193) (line 656) (column 71) (len 8)) (ref r260)) (expression (span (offset 32203) (line 656) (column 81) (len 10)) (ref r261))))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianAcceleration3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r262)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 32324) (line 661) (column 11) (len 625)) (normalized "source: item 3-11 acceleration (vector)\nsymbol(s): `vec(a)`\napplication domain: generic\nname: Acceleration\nquantity dimension: L^1*T^-2\nmeasurement unit(s): m*s^-2\ntensor order: 1\ndefinition: vector (ISO 80000-2) quantity giving the rate of change of velocity (item 3-10)\nremarks: The acceleration vector is given by: `vec(a) = (d vec(v))/(dt)` where `vec(v)` denotes the velocity (item 3-10.1) and `t` the duration (item 3-9). The magnitude (ISO 80000-2) of the acceleration of free fall is usually denoted by `g`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r263)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 32984) (line 672) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r264)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r265)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianAcceleration3dVector") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r266)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianAcceleration3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r267)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r268)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33280) (line 679) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r269)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 33324) (line 680) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r270)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r271)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 33393) (line 684) (column 7) (len 40)) (normalized "ISO-80000-3 item 3-12 angular velocity "))) (attribute-def (declaration-name "AngularVelocityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r272)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 33522) (line 687) (column 11) (len 822)) (normalized "source: item 3-12 angular velocity (magnitude)\nsymbol(s): `ω`\napplication domain: generic\nname: AngularVelocity\nquantity dimension: T^-1\nmeasurement unit(s): rad*s^-1, s^-1\ntensor order: 0\ndefinition: vector (ISO 80000-2) quantity giving the rate of change of the rotational displacement (item 3-6) as its magnitude (ISO 80000-2) and with a direction equal to the direction of the axis of rotation\nremarks: The angular velocity vector is given by: `vec(ω) = (d φ) / (dt) vec(u)` where `φ` denotes the angular displacement (item 3-6), `t` the duration (item 3-9), and `vec(u)` the unit vector (ISO 80000-2) along the axis of rotation in the direction for which the rotation corresponds to a right-hand spiral.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r273)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r274)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r275)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r276)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "angularVelocity") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r277)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "AngularVelocityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r278)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r279)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r280)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34659) (line 705) (column 79) (len 5)) (member-access (base (expression (span (offset 34659) (line 705) (column 79) (len 3)) (ref r281))) (separator dot) (member (ref r282))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r283)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34681) (line 705) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 34682) (line 705) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r284)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r285)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 34756) (line 706) (column 70) (len 10)) (ref r286))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianAngularVelocity3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r287)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 34879) (line 711) (column 11) (len 824)) (normalized "source: item 3-12 angular velocity (vector)\nsymbol(s): `vec(ω)`\napplication domain: generic\nname: AngularVelocity\nquantity dimension: T^-1\nmeasurement unit(s): rad*s^-1, s^-1\ntensor order: 1\ndefinition: vector (ISO 80000-2) quantity giving the rate of change of the rotational displacement (item 3-6) as its magnitude (ISO 80000-2) and with a direction equal to the direction of the axis of rotation\nremarks: The angular velocity vector is given by: `vec(ω) = (d φ) / (dt) vec(u)` where `φ` denotes the angular displacement (item 3-6), `t` the duration (item 3-9), and `vec(u)` the unit vector (ISO 80000-2) along the axis of rotation in the direction for which the rotation corresponds to a right-hand spiral.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r288)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 35738) (line 722) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r289)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r290)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianAngularVelocity3dVector") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r291)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianAngularVelocity3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r292)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r293)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36046) (line 729) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r294)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 36090) (line 730) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r295)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r296)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 36162) (line 734) (column 7) (len 44)) (normalized "ISO-80000-3 item 3-13 angular acceleration "))) (attribute-def (declaration-name "AngularAccelerationValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r297)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 36299) (line 737) (column 11) (len 579)) (normalized "source: item 3-13 angular acceleration (magnitude)\nsymbol(s): `α`\napplication domain: generic\nname: AngularAcceleration\nquantity dimension: T^-2\nmeasurement unit(s): rad*s^-2, s^-2\ntensor order: 0\ndefinition: vector (ISO 80000-2) quantity giving the rate of change of angular velocity (item 3-12)\nremarks: The angular acceleration vector is given by: `vec α = (d vec(ω))/(dt)` Where `vec(ω)` denotes the angular velocity (item 3-12) and `t` the duration (item 3-9).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r298)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r299)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r300)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r301)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "angularAcceleration") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r302)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "AngularAccelerationUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r303)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r304)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r305)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37209) (line 755) (column 79) (len 5)) (member-access (base (expression (span (offset 37209) (line 755) (column 79) (len 3)) (ref r306))) (separator dot) (member (ref r307))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r308)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37231) (line 755) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 37232) (line 755) (column 102) (len 1)) (integer 2)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r309)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r310)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 37306) (line 756) (column 70) (len 10)) (ref r311))))) (body semicolon)))))) (attribute-def (declaration-name "CartesianAngularAcceleration3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r312)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 37433) (line 761) (column 11) (len 581)) (normalized "source: item 3-13 angular acceleration (vector)\nsymbol(s): `vec(α)`\napplication domain: generic\nname: AngularAcceleration\nquantity dimension: T^-2\nmeasurement unit(s): rad*s^-2, s^-2\ntensor order: 1\ndefinition: vector (ISO 80000-2) quantity giving the rate of change of angular velocity (item 3-12)\nremarks: The angular acceleration vector is given by: `vec α = (d vec(ω))/(dt)` Where `vec(ω)` denotes the angular velocity (item 3-12) and `t` the duration (item 3-9).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r313)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38049) (line 772) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r314)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r315)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianAngularAcceleration3dVector") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r316)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianAngularAcceleration3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r317)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r318)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38373) (line 779) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r319)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 38417) (line 780) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r320)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r321)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 38493) (line 784) (column 7) (len 47)) (normalized "ISO-80000-3 item 3-14 period duration, period "))) (attribute-def (declaration-name "periodDuration") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r322)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 38631) (line 787) (column 11) (len 454)) (normalized "source: item 3-14 period duration, period\nsymbol(s): `T`\napplication domain: generic\nname: PeriodDuration (specializes Duration)\nquantity dimension: T^1\nmeasurement unit(s): s\ntensor order: 0\ndefinition: duration (item 3-9) of one cycle of a periodic event\nremarks: A periodic event is an event that occurs regularly with a fixed time interval.\n"))))) (alias (name "period") (target (ref r323)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 39139) (line 802) (column 7) (len 37)) (normalized "ISO-80000-3 item 3-15 time constant "))) (attribute-def (declaration-name "timeConstant") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r324)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 39265) (line 805) (column 11) (len 663)) (normalized "source: item 3-15 time constant\nsymbol(s): `τ`, `T`\napplication domain: generic\nname: TimeConstant (specializes Duration)\nquantity dimension: T^1\nmeasurement unit(s): s\ntensor order: 0\ndefinition: parameter characterizing the response to a step input of a first-order, linear time-invariant system\nremarks: If a quantity is a function of the duration (item 3-9) expressed by: `F(t) prop e^(-t/τ)` where `t` denotes the duration (item 3-9), then `τ` denotes the time constant. Here the time constant `τ` applies to an exponentially decaying quantity.\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 39944) (line 818) (column 7) (len 32)) (normalized "ISO-80000-3 item 3-16 rotation "))) (attribute-def (declaration-name "rotation") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r325)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 40058) (line 821) (column 11) (len 542)) (normalized "source: item 3-16 rotation\nsymbol(s): `N`\napplication domain: generic\nname: Rotation (specializes Count)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: number of revolutions\nremarks: `N` is the number (not necessarily an integer) of revolutions, for example, of a rotating body about a given axis. Its value is given by: `N = φ/(2 π)` where `φ` denotes the measure of rotational displacement (item 3-6).\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 40616) (line 834) (column 7) (len 35)) (normalized "ISO-80000-3 item 3-17.1 frequency "))) (attribute-def (declaration-name "FrequencyValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r326)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 40734) (line 837) (column 11) (len 423)) (normalized "source: item 3-17.1 frequency\nsymbol(s): `f`, `ν`\napplication domain: generic\nname: Frequency\nquantity dimension: T^-1\nmeasurement unit(s): Hz, s^-1\ntensor order: 0\ndefinition: inverse of period duration (item 3-14)\nremarks: The frequency is given by: `f = 1/T` where `T` denotes the period duration (item 3-14).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r327)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r328)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r329)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r330)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "frequency") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r331)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "FrequencyUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r332)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r333)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r334)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 41448) (line 855) (column 79) (len 5)) (member-access (base (expression (span (offset 41448) (line 855) (column 79) (len 3)) (ref r335))) (separator dot) (member (ref r336))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r337)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 41470) (line 855) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 41471) (line 855) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r338)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r339)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 41545) (line 856) (column 70) (len 10)) (ref r340))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 41572) (line 859) (column 7) (len 46)) (normalized "ISO-80000-3 item 3-17.2 rotational frequency "))) (attribute-def (declaration-name "rotationalFrequency") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r341)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 41715) (line 862) (column 11) (len 519)) (normalized "source: item 3-17.2 rotational frequency\nsymbol(s): `n`\napplication domain: generic\nname: RotationalFrequency (specializes Frequency)\nquantity dimension: T^-1\nmeasurement unit(s): s^-1\ntensor order: 0\ndefinition: duration (item 3-9) of one cycle of a periodic event\nremarks: The rotational frequency is given by: `n = (dN) / (dt)` where `N` denotes the rotation (item 3-16) and `t` is the duration (item 3-9).\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 42250) (line 875) (column 7) (len 41)) (normalized "ISO-80000-3 item 3-18 angular frequency "))) (attribute-def (declaration-name "AngularFrequencyValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r342)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 42381) (line 878) (column 11) (len 451)) (normalized "source: item 3-18 angular frequency\nsymbol(s): `ω`\napplication domain: generic\nname: AngularFrequency\nquantity dimension: T^-1\nmeasurement unit(s): rad*s^-1, s^-1\ntensor order: 0\ndefinition: rate of change of the phase angle (item 3-7)\nremarks: The angular frequency is given by: `ω = 2 π f` where `f` denotes the frequency (item 3-17.1).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r343)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r344)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r345)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r346)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "angularFrequency") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r347)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "AngularFrequencyUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r348)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r349)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r350)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43151) (line 896) (column 79) (len 5)) (member-access (base (expression (span (offset 43151) (line 896) (column 79) (len 3)) (ref r351))) (separator dot) (member (ref r352))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r353)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43173) (line 896) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 43174) (line 896) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r354)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r355)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 43248) (line 897) (column 70) (len 10)) (ref r356))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 43275) (line 900) (column 7) (len 34)) (normalized "ISO-80000-3 item 3-19 wavelength "))) (attribute-def (declaration-name "wavelength") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r357)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 43394) (line 903) (column 11) (len 367)) (normalized "source: item 3-19 wavelength\nsymbol(s): `λ`\napplication domain: generic\nname: Wavelength (specializes Length)\nquantity dimension: L^1\nmeasurement unit(s): m\ntensor order: 0\ndefinition: length (item 3-1.1) of the repetition interval of a wave\nremarks: None.\n"))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 43777) (line 916) (column 7) (len 45)) (normalized "ISO-80000-3 item 3-20 repetency, wavenumber "))) (attribute-def (declaration-name "RepetencyValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r358)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 43905) (line 919) (column 11) (len 430)) (normalized "source: item 3-20 repetency, wavenumber\nsymbol(s): `σ`, `ṽ`\napplication domain: generic\nname: Repetency\nquantity dimension: L^-1\nmeasurement unit(s): m^-1\ntensor order: 0\ndefinition: inverse of the wavelength (item 3-19)\nremarks: The repetency is given by: `σ = 1 / λ` where `λ` denotes the wavelength (item 3-19).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r359)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r360)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r361)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r362)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "repetency") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r363)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "RepetencyUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r364)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r365)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r366)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 44624) (line 937) (column 77) (len 5)) (member-access (base (expression (span (offset 44624) (line 937) (column 77) (len 3)) (ref r367))) (separator dot) (member (ref r368))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r369)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 44646) (line 937) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 44647) (line 937) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r370)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r371)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 44721) (line 938) (column 70) (len 8)) (ref r372))))) (body semicolon)))))) (alias (name "WavenumberUnit") (target (ref r373)) (body semicolon)) (alias (name "WavenumberValue") (target (ref r374)) (body semicolon)) (alias (name "wavenumber") (target (ref r375)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 44873) (line 945) (column 7) (len 35)) (normalized "ISO-80000-3 item 3-21 wave vector "))) (attribute-def (declaration-name "CartesianWave3dVector") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r376)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 45002) (line 948) (column 11) (len 432)) (normalized "source: item 3-21 wave vector\nsymbol(s): `vec(k)`\napplication domain: generic\nname: WaveVector\nquantity dimension: L^-1\nmeasurement unit(s): m^-1\ntensor order: 1\ndefinition: vector normal to the surfaces of constant phase angle (item 3-7) of a wave, with the magnitude (ISO 80000-2) of repetency (item 3-20)\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r377)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45469) (line 959) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r378)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r379)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "cartesianWave3dVector") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r380)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (attribute-def (declaration-name "CartesianWaveVector3dCoordinateFrame") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r381)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r382)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45745) (line 966) (column 33) (len 5)) (boolean false))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r383)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 45789) (line 967) (column 38) (len 4)) (boolean true))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r384)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r385)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 45855) (line 971) (column 7) (len 61)) (normalized "ISO-80000-3 item 3-22 angular repetency, angular wavenumber "))) (attribute-def (declaration-name "AngularRepetencyValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r386)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 46006) (line 974) (column 11) (len 473)) (normalized "source: item 3-22 angular repetency, angular wavenumber\nsymbol(s): `k`\napplication domain: generic\nname: AngularRepetency\nquantity dimension: L^-1\nmeasurement unit(s): m^-1\ntensor order: 0\ndefinition: magnitude (ISO 80000-2) of the wave vector (item 3-21)\nremarks: The angular repetency is given by: `κ = (2 π)/λ` where `λ` denotes the wavelength (item 3-19).\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r387)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r388)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r389)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r390)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "angularRepetency") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r391)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "AngularRepetencyUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r392)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r393)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r394)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 46796) (line 992) (column 77) (len 5)) (member-access (base (expression (span (offset 46796) (line 992) (column 77) (len 3)) (ref r395))) (separator dot) (member (ref r396))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r397)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 46818) (line 992) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 46819) (line 992) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r398)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r399)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 46893) (line 993) (column 70) (len 8)) (ref r400))))) (body semicolon)))))) (alias (name "AngularWavenumberUnit") (target (ref r401)) (body semicolon)) (alias (name "AngularWavenumberValue") (target (ref r402)) (body semicolon)) (alias (name "angularWavenumber") (target (ref r403)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 47087) (line 1000) (column 7) (len 53)) (normalized "ISO-80000-3 item 3-23.1 phase velocity, phase speed "))) (attribute-def (declaration-name "PhaseVelocityValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r404)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 47227) (line 1003) (column 11) (len 773)) (normalized "source: item 3-23.1 phase velocity, phase speed\nsymbol(s): `c`, `v`, `(ν)`, `c_φ`, `v_φ`, `(ν_φ)`\napplication domain: generic\nname: PhaseVelocity\nquantity dimension: L^1*T^-1\nmeasurement unit(s): m*s^-1\ntensor order: 0\ndefinition: speed with which the phase angle (item 3-7) of a wave propagates in space\nremarks: The phase velocity is given by: `c = ω/κ` where `ω` denotes the angular frequency (item 3-18) and `k` the angular repetency (item 3-22). If phase velocities of electromagnetic waves and other phase velocities are both involved, then `c` should be used for the former and `υ` for the latter. Phase velocity can also be written as `c = λ f`.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r405)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r406)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r407)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r408)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "phaseVelocity") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r409)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "PhaseVelocityUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r410)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r411)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r412)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48305) (line 1021) (column 77) (len 5)) (member-access (base (expression (span (offset 48305) (line 1021) (column 77) (len 3)) (ref r413))) (separator dot) (member (ref r414))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r415)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48327) (line 1021) (column 99) (len 1)) (integer 1))))) (body semicolon)))) (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r416)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r417)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48410) (line 1022) (column 79) (len 5)) (member-access (base (expression (span (offset 48410) (line 1022) (column 79) (len 3)) (ref r418))) (separator dot) (member (ref r419))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r420)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48432) (line 1022) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 48433) (line 1022) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r421)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r422)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 48507) (line 1023) (column 70) (len 22)) (tuple (expression (span (offset 48508) (line 1023) (column 71) (len 8)) (ref r423)) (expression (span (offset 48518) (line 1023) (column 81) (len 10)) (ref r424))))))) (body semicolon)))))) (alias (name "PhaseSpeedUnit") (target (ref r425)) (body semicolon)) (alias (name "PhaseSpeedValue") (target (ref r426)) (body semicolon)) (alias (name "phaseSpeed") (target (ref r427)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 48685) (line 1030) (column 7) (len 53)) (normalized "ISO-80000-3 item 3-23.2 group velocity, group speed "))) (attribute-def (declaration-name "groupVelocity") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r428)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 48825) (line 1033) (column 11) (len 562)) (normalized "source: item 3-23.2 group velocity, group speed\nsymbol(s): `c_g`, `v_g`, `(ν_g)`\napplication domain: generic\nname: GroupVelocity (specializes Speed)\nquantity dimension: L^1*T^-1\nmeasurement unit(s): m*s^-1\ntensor order: 0\ndefinition: speed with which the envelope of a wave propagates in space\nremarks: The group velocity is given by: `c_g = (d ω)/ (dk)` where `ω` denotes the angular frequency (item 3-18) and `k` the angular repetency (item 3-22).\n"))))) (alias (name "groupSpeed") (target (ref r429)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 49444) (line 1048) (column 7) (len 43)) (normalized "ISO-80000-3 item 3-24 damping coefficient "))) (attribute-def (declaration-name "DampingCoefficientValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r430)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 49579) (line 1051) (column 11) (len 388)) (normalized "source: item 3-24 damping coefficient\nsymbol(s): `δ`\napplication domain: generic\nname: DampingCoefficient\nquantity dimension: T^-1\nmeasurement unit(s): s^-1\ntensor order: 0\ndefinition: inverse of the time constant (item 3-15) of an exponentially varying quantity\nremarks: None.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r431)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r432)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r433)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r434)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "dampingCoefficient") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r435)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "DampingCoefficientUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r436)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "durationPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r437)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r438)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50294) (line 1069) (column 79) (len 5)) (member-access (base (expression (span (offset 50294) (line 1069) (column 79) (len 3)) (ref r439))) (separator dot) (member (ref r440))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r441)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50316) (line 1069) (column 101) (len 2)) (unary (operator "-") (operand (expression (span (offset 50317) (line 1069) (column 102) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r442)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r443)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 50391) (line 1070) (column 70) (len 10)) (ref r444))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 50418) (line 1073) (column 7) (len 45)) (normalized "ISO-80000-3 item 3-25 logarithmic decrement "))) (attribute-def (declaration-name "LogarithmicDecrementValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r445)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 50555) (line 1076) (column 11) (len 418)) (normalized "source: item 3-25 logarithmic decrement\nsymbol(s): `Λ`\napplication domain: generic\nname: LogarithmicDecrement (specializes DimensionOneQuantity)\nquantity dimension: 1\nmeasurement unit(s): 1\ntensor order: 0\ndefinition: product of damping coefficient (item 3-24) and period duration (item 3-14)\nremarks: None.\n"))))) (attribute-def (declaration-name "logarithmicDecrement") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r446)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 51072) (line 1090) (column 7) (len 49)) (normalized "ISO-80000-3 item 3-26.1 attenuation, extinction "))) (attribute-def (declaration-name "AttenuationValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r447)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 51206) (line 1093) (column 11) (len 602)) (normalized "source: item 3-26.1 attenuation, extinction\nsymbol(s): `α`\napplication domain: generic\nname: Attenuation\nquantity dimension: L^-1\nmeasurement unit(s): m^-1\ntensor order: 0\ndefinition: gradual decrease in magnitude (ISO 80000-2) of any kind of flux through a medium\nremarks: If a quantity is a function of distance (item 3-1.8) expressed by: `f(x) prop e^(-α x)` where `x` denotes distance (item 3-1.8), then `α` denotes attenuation. The inverse of attenuation is called attenuation length.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r448)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r449)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r450)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r451)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "attenuation") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r452)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "AttenuationUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r453)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r454)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r455)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 52105) (line 1111) (column 77) (len 5)) (member-access (base (expression (span (offset 52105) (line 1111) (column 77) (len 3)) (ref r456))) (separator dot) (member (ref r457))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r458)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 52127) (line 1111) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 52128) (line 1111) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r459)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r460)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 52202) (line 1112) (column 70) (len 8)) (ref r461))))) (body semicolon)))))) (alias (name "ExtinctionUnit") (target (ref r462)) (body semicolon)) (alias (name "ExtinctionValue") (target (ref r463)) (body semicolon)) (alias (name "extinction") (target (ref r464)) (body semicolon)) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 52360) (line 1119) (column 7) (len 43)) (normalized "ISO-80000-3 item 3-26.2 phase coefficient "))) (attribute-def (declaration-name "PhaseCoefficientValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r465)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 52493) (line 1122) (column 11) (len 575)) (normalized "source: item 3-26.2 phase coefficient\nsymbol(s): `β`\napplication domain: generic\nname: PhaseCoefficient\nquantity dimension: L^-1\nmeasurement unit(s): rad/m, m^-1\ntensor order: 0\ndefinition: change of phase angle (item 3-7) with the length (item 3-1.1) along the path travelled by a plane wave\nremarks: If a quantity is a function of distance expressed by: `f(x) prop cos(β(x-x_0))` where `x` denotes distance (item 3-1.8), then `β` denotes the phase coefficient.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r466)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r467)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r468)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r469)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "phaseCoefficient") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r470)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "PhaseCoefficientUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r471)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r472)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r473)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 53385) (line 1140) (column 77) (len 5)) (member-access (base (expression (span (offset 53385) (line 1140) (column 77) (len 3)) (ref r474))) (separator dot) (member (ref r475))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r476)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 53407) (line 1140) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 53408) (line 1140) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r477)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r478)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 53482) (line 1141) (column 70) (len 8)) (ref r479))))) (body semicolon)))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 53507) (line 1144) (column 7) (len 49)) (normalized "ISO-80000-3 item 3-26.3 propagation coefficient "))) (attribute-def (declaration-name "PropagationCoefficientValue") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r480)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (doc (name none) (locale none) (body (span (offset 53652) (line 1147) (column 11) (len 589)) (normalized "source: item 3-26.3 propagation coefficient\nsymbol(s): `γ`\napplication domain: generic\nname: PropagationCoefficient\nquantity dimension: L^-1\nmeasurement unit(s): m^-1\ntensor order: 0\ndefinition: measure of the change of amplitude and phase angle (item 3-7) of a plane wave propagating in a given direction\nremarks: The propagation coefficient is given by: `γ = α + iβ` where `α` denotes attenuation (item 3-26.1) and `β` the phase coefficient (item 3-26.2) of a plane wave.\n"))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r481)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r482)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r483)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r484)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (attribute-def (declaration-name "propagationCoefficient") (short-name none) (modifiers) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r485)))) (multiplicity (lower unbounded) (upper unbounded)) (multiplicity-modifiers (ordering none) (uniqueness nonunique)) (value none) (body semicolon)) (attribute-def (declaration-name "PropagationCoefficientUnit") (short-name none) (modifiers) (typing (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r486)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (value none) (body brace (attribute-usage (declaration-name "lengthPF") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r487)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r488)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 54582) (line 1165) (column 77) (len 5)) (member-access (base (expression (span (offset 54582) (line 1165) (column 77) (len 3)) (ref r489))) (separator dot) (member (ref r490))))))) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r491)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 54604) (line 1165) (column 99) (len 2)) (unary (operator "-") (operand (expression (span (offset 54605) (line 1165) (column 100) (len 1)) (integer 1)))))))) (body semicolon)))) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r492)))) (references none) (crosses none) (intersects none) (value none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r493)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 54679) (line 1166) (column 70) (len 8)) (ref r494))))) (body semicolon)))))))))
)
~~~
