# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Vehicle): VehicleDefinitions"))
~~~
# SOURCE
~~~sysml
package VehicleDefinitions {
	doc
	/*
	 * Example vehicle definitions model.
	 */

	private import ScalarValues::*;
	private import Quantities::*;
	private import MeasurementReferences::*;
	private import ISQ::*;
	private import SI::*;
	
	/* PART DEFINITIONS */
	
	part def Vehicle {
		attribute mass :> ISQ::mass;
	}
	part def Transmission;	
	part def AxleAssembly;
	part def Axle {
		port leftMountingPoint: AxleMountIF;
		port rightMountingPoint: AxleMountIF;
	}
	part def Wheel {
		port hub: WheelHubIF;
	}
	part def Lugbolt {
		attribute tighteningTorque :> ISQ::torque;
	}
	
	/* PORT DEFINITIONS */
	
	port def DriveIF { 
		in driveTorque :> ISQ::torque;
	}
	
	port def AxleMountIF { 
		out transferredTorque :> ISQ::torque;
	}
	
	port def WheelHubIF { 
		in appliedTorque :> ISQ::torque;
	}
	
	/* INTERFACE DEFINITIONS */
	
	interface def Mounting {
		doc /* The definition of the interface for mounting a Wheel to an Axle. */
		end axleMount: AxleMountIF;
		end hub: WheelHubIF;
		
		flow axleMount.transferredTorque to hub.appliedTorque;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicle_definitions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package VehicleDefinitions {
    doc
    /*
	 * Example vehicle definitions model.
	 */
    private import ScalarValues::*;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import ISQ::*;
    private import SI::*;
    /* PART DEFINITIONS */
    part def Vehicle {
        attribute mass :> ISQ::mass;
    }
    part def Transmission;
    part def AxleAssembly;
    part def Axle {
        port leftMountingPoint : AxleMountIF;
        port rightMountingPoint : AxleMountIF;
    }
    part def Wheel {
        port hub : WheelHubIF;
    }
    part def Lugbolt {
        attribute tighteningTorque :> ISQ::torque;
    }
    /* PORT DEFINITIONS */
    port def DriveIF {
        in driveTorque :> ISQ::torque;
    }
    port def AxleMountIF {
        out transferredTorque :> ISQ::torque;
    }
    port def WheelHubIF {
        in appliedTorque :> ISQ::torque;
    }
    /* INTERFACE DEFINITIONS */
    interface def Mounting {
        doc
        /* The definition of the interface for mounting a Wheel to an Axle. */
        end axleMount : AxleMountIF;
        end hub : WheelHubIF;
        flow from axleMount.transferredTorque to hub.appliedTorque;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 99) (line 7) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 99) (line 7) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 132) (line 8) (column 17) (len 10)) (segments (segment 0 (token "Quantities") (name "Quantities") (separator none) (span (offset 132) (line 8) (column 17) (len 10)))))
    (reference r2 (scope relative) (span (offset 163) (line 9) (column 17) (len 21)) (segments (segment 0 (token "MeasurementReferences") (name "MeasurementReferences") (separator none) (span (offset 163) (line 9) (column 17) (len 21)))))
    (reference r3 (scope relative) (span (offset 205) (line 10) (column 17) (len 3)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 205) (line 10) (column 17) (len 3)))))
    (reference r4 (scope relative) (span (offset 229) (line 11) (column 17) (len 2)) (segments (segment 0 (token "SI") (name "SI") (separator none) (span (offset 229) (line 11) (column 17) (len 2)))))
    (reference r5 (scope relative) (span (offset 304) (line 16) (column 21) (len 9)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 304) (line 16) (column 21) (len 3))) (segment 1 (token "mass") (name "mass") (separator colon-colon) (span (offset 309) (line 16) (column 26) (len 4)))))
    (reference r6 (scope relative) (span (offset 410) (line 21) (column 27) (len 11)) (segments (segment 0 (token "AxleMountIF") (name "AxleMountIF") (separator none) (span (offset 410) (line 21) (column 27) (len 11)))))
    (reference r7 (scope relative) (span (offset 450) (line 22) (column 28) (len 11)) (segments (segment 0 (token "AxleMountIF") (name "AxleMountIF") (separator none) (span (offset 450) (line 22) (column 28) (len 11)))))
    (reference r8 (scope relative) (span (offset 496) (line 25) (column 13) (len 10)) (segments (segment 0 (token "WheelHubIF") (name "WheelHubIF") (separator none) (span (offset 496) (line 25) (column 13) (len 10)))))
    (reference r9 (scope relative) (span (offset 563) (line 28) (column 33) (len 11)) (segments (segment 0 (token "ISQ") (name "ISQ") (separator none) (span (offset 563) (line 28) (column 33) (len 3))) (segment 1 (token "torque") (name "torque") (separator colon-colon) (span (offset 568) (line 28) (column 38) (len 6)))))
    (reference r10 (scope relative) (span (offset 951) (line 49) (column 18) (len 11)) (segments (segment 0 (token "AxleMountIF") (name "AxleMountIF") (separator none) (span (offset 951) (line 49) (column 18) (len 11)))))
    (reference r11 (scope relative) (span (offset 975) (line 50) (column 12) (len 10)) (segments (segment 0 (token "WheelHubIF") (name "WheelHubIF") (separator none) (span (offset 975) (line 50) (column 12) (len 10)))))
    (reference r12 (scope relative) (span (offset 997) (line 52) (column 8) (len 27)) (segments (segment 0 (token "axleMount") (name "axleMount") (separator none) (span (offset 997) (line 52) (column 8) (len 9))) (segment 1 (token "transferredTorque") (name "transferredTorque") (separator dot) (span (offset 1007) (line 52) (column 18) (len 17)))))
    (reference r13 (scope relative) (span (offset 1028) (line 52) (column 39) (len 17)) (segments (segment 0 (token "hub") (name "hub") (separator none) (span (offset 1028) (line 52) (column 39) (len 3))) (segment 1 (token "appliedTorque") (name "appliedTorque") (separator dot) (span (offset 1032) (line 52) (column 43) (len 13)))))
  )
  (root (package (name "VehicleDefinitions") (body brace (doc (name none) (locale none) (body (span (offset 37) (line 3) (column 4) (len 42)) (normalized "Example vehicle definitions model.\n"))) (import (target (span (span (offset 99) (line 7) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 111) (line 7) (column 29) (len 3))) (separator (span (offset 111) (line 7) (column 29) (len 2))) (marker (span (offset 113) (line 7) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 132) (line 8) (column 17) (len 13))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 142) (line 8) (column 27) (len 3))) (separator (span (offset 142) (line 8) (column 27) (len 2))) (marker (span (offset 144) (line 8) (column 29) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 163) (line 9) (column 17) (len 24))) (all none) (ref r2) (shape (namespace (wildcard-suffix (span (span (offset 184) (line 9) (column 38) (len 3))) (separator (span (offset 184) (line 9) (column 38) (len 2))) (marker (span (offset 186) (line 9) (column 40) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 205) (line 10) (column 17) (len 6))) (all none) (ref r3) (shape (namespace (wildcard-suffix (span (span (offset 208) (line 10) (column 20) (len 3))) (separator (span (offset 208) (line 10) (column 20) (len 2))) (marker (span (offset 210) (line 10) (column 22) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 229) (line 11) (column 17) (len 5))) (all none) (ref r4) (shape (namespace (wildcard-suffix (span (span (offset 231) (line 11) (column 19) (len 3))) (separator (span (offset 231) (line 11) (column 19) (len 2))) (marker (span (offset 233) (line 11) (column 21) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 241) (line 13) (column 4) (len 18)) (normalized "PART DEFINITIONS "))) (part-def (name "Vehicle") (modifiers) (body brace (attribute-usage (declaration-name "mass") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r5)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Transmission") (modifiers) (body semicolon)) (part-def (name "AxleAssembly") (modifiers) (body semicolon)) (part-def (name "Axle") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "leftMountingPoint") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "rightMountingPoint") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Wheel") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "hub") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Lugbolt") (modifiers) (body brace (attribute-usage (declaration-name "tighteningTorque") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets (relationship (kind subsets) (implied false) (targets (ref r9)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 584) (line 31) (column 4) (len 18)) (normalized "PORT DEFINITIONS "))) (port-def (name "DriveIF") (modifiers) (specializes none) (body brace (in-out-declaration))) (port-def (name "AxleMountIF") (modifiers) (specializes none) (body brace (in-out-declaration))) (port-def (name "WheelHubIF") (modifiers) (specializes none) (body brace (in-out-declaration))) (comment (keyword none) (name none) (about) (locale none) (body (span (offset 803) (line 45) (column 4) (len 23)) (normalized "INTERFACE DEFINITIONS "))) (interface-def (name "Mounting") (modifiers) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 865) (line 48) (column 9) (len 66)) (normalized "The definition of the interface for mounting a Wheel to an Axle. "))) (end (introducer bare) (short-name none) (identity (declaration (name "axleMount") (span (offset 940) (line 49) (column 7) (len 9)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)) (end (introducer bare) (short-name none) (identity (declaration (name "hub") (span (offset 970) (line 50) (column 7) (len 3)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (references none) (multiplicity none) (redefines none) (crosses none) (nested-usage none)) (flow-usage (kind flow) (visibility none) (declaration (endpoint-only (from (connector-end (multiplicity none) (target (ref r12)) (references none))) (to (connector-end (multiplicity none) (target (ref r13)) (references none))))) (body (body semicolon))))))))
)
~~~
