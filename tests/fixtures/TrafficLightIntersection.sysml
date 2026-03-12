/*
 * Traffic Light Intersection - SysML v2 model.
 * Derived from intersection/README.md: §4 structure, §5 state machine, §6 use cases, §7 requirements, §8 interfaces.
 * Central artifact: intersection controller state machine (phase cycle + Normal/Flash modes).
 */
package TrafficLightIntersection {
	doc /* Root package for the traffic light intersection example. */

	// ========== Value types ==========
	attribute def Real;
	attribute def TimeValue;
	attribute def String;
	attribute def Boolean;

	// ========== Port definitions (README §8) ==========
	port def LampCommandPort {
		out indication : String;
	}

	port def DetectorOutputPort {
		out vehiclePresent : Boolean;
		out pedestrianCall : Boolean;
	}

	port def OperatorCommandPort {
		in modeCommand : String;
	}

	// ========== Part definitions (README §4) ==========
	part def SignalHead {
		doc /* Display Red, Yellow, Green to road users. */
		attribute approachName : String;
		port cmd : ~LampCommandPort;
	}

	part def Detector {
		doc /* Vehicle presence or pedestrian demand. */
		attribute coverage : String;
		port output : DetectorOutputPort;
	}

	part def IntersectionController {
		doc /* Logic and timing; runs phase state machine. */
		attribute minGreen : TimeValue;
		attribute maxGreen : TimeValue;
		attribute yellowTime : TimeValue;
		attribute allRedTime : TimeValue;
		attribute cycleLength : TimeValue;
		port mainLampOut : LampCommandPort;
		port sideLampOut : LampCommandPort;
		port detectorIn : ~DetectorOutputPort;
		port operatorIn : ~OperatorCommandPort;
		exhibit state controllerMode : ControllerStateMachine;
	}

	// ========== Top-level system ==========
	part def TrafficLightIntersection {
		doc /* 4-way intersection with controller, signal heads, optional detector. */
		part controller : IntersectionController;
		part mainStreetSignals : SignalHead;
		part sideStreetSignals : SignalHead;
		part detector : Detector;

		connect controller.mainLampOut to mainStreetSignals.cmd;
		connect controller.sideLampOut to sideStreetSignals.cmd;
		connect detector.output to controller.detectorIn;
	}

	// ========== State machine - Phase states (README §5.1, §5.2) ==========
	state def PhaseA {
		doc /* Main Street green; Side Street red. */
	}

	state def PhaseB {
		doc /* Main Street yellow; Side Street red. */
	}

	state def PhaseC {
		doc /* Side Street green; Main Street red. */
	}

	state def PhaseD {
		doc /* Side Street yellow; Main Street red. */
	}

	item def PhaseTimerElapsed;
	item def FlashCommand;
	item def NormalCommand;

	state def Normal {
		doc /* Running main cycle A -> B -> C -> D -> A. */
		entry;
		then phaseA;
		state phaseA : PhaseA;
		state phaseB : PhaseB;
		state phaseC : PhaseC;
		state phaseD : PhaseD;
		transition a_to_b first phaseA accept PhaseTimerElapsed then phaseB;
		transition b_to_c first phaseB accept PhaseTimerElapsed then phaseC;
		transition c_to_d first phaseC accept PhaseTimerElapsed then phaseD;
		transition d_to_a first phaseD accept PhaseTimerElapsed then phaseA;
	}

	state def Flash {
		doc /* All approaches flashing; no cycle. */
	}

	state def ControllerStateMachine {
		doc /* Top-level: Normal cycle or Flash mode. */
		entry;
		then normal;
		state normal : Normal;
		state flash : Flash;
		transition to_flash first normal accept FlashCommand then flash;
		transition to_normal first flash accept NormalCommand then normal;
	}

	// ========== Requirements (README §7) ==========
	requirement def NoConflictingGreensReq {
		subject intersection : TrafficLightIntersection;
		doc /* No two conflicting movements shall have green simultaneously. */
		require constraint { doc /* Only one approach group has green at a time. */ }
	}

	requirement def ClearanceTimeReq {
		subject intersection : TrafficLightIntersection;
		doc /* Yellow and all-red shall be long enough for clearance per approach speed. */
		require constraint { doc /* yellowTime and allRedTime meet clearance requirements. */ }
	}

	requirement def FailSafeReq {
		subject intersection : TrafficLightIntersection;
		doc /* On fault or loss of controller, transition to flash or safe state. */
		require constraint { doc /* Fault triggers transition to Flash. */ }
	}

	requirement def PhaseSequenceReq {
		subject controller : IntersectionController;
		doc /* Controller shall run defined phase sequence A -> B -> C -> D. */
		require constraint { doc /* Cycle runs PhaseA, PhaseB, PhaseC, PhaseD in order. */ }
	}

	requirement def ModesReq {
		subject controller : IntersectionController;
		doc /* Support at least Normal and Flash modes. */
		require constraint { doc /* Normal and Flash modes supported. */ }
	}

	requirement def VisibilityReq {
		subject intersection : TrafficLightIntersection;
		doc /* Signal indications shall meet standard visibility. */
		require constraint { doc /* Luminance and position per standards. */ }
	}

	part intersectionInstance : TrafficLightIntersection;
	satisfy NoConflictingGreensReq by intersectionInstance;
	satisfy ClearanceTimeReq by intersectionInstance;
	satisfy FailSafeReq by intersectionInstance;
	satisfy PhaseSequenceReq by intersectionInstance.controller;
	satisfy ModesReq by intersectionInstance.controller;
	satisfy VisibilityReq by intersectionInstance;

	// ========== Use cases (README §6) ==========
	item def Operator;
	item def RoadUser;

	use case def NormalOperation {
		subject intersection : TrafficLightIntersection;
		actor driver : RoadUser;
		doc /* Controller runs phase sequence; vehicles and pedestrians obey signals. */
		objective { doc /* Safe, predictable right-of-way assignment. */ }
	}

	use case def SwitchToFlash {
		subject intersection : TrafficLightIntersection;
		actor operator : Operator;
		doc /* Operator or fault triggers flash mode. */
		objective { doc /* All approaches flash for maintenance or failure. */ }
	}

	use case def ReturnToNormal {
		subject intersection : TrafficLightIntersection;
		actor operator : Operator;
		doc /* Flash ends; controller resumes normal cycle. */
		objective { doc /* Resume phase sequence from defined phase. */ }
	}

	use case def ActuatedGreen {
		subject intersection : TrafficLightIntersection;
		actor driver : RoadUser;
		doc /* Green extends on vehicle demand until max green or gap-out. */
		objective { doc /* Responsive green based on detection. */ }
	}

	use case def PedestrianCrossing {
		subject intersection : TrafficLightIntersection;
		actor pedestrian : RoadUser;
		doc /* Button press registers; walk phase served in cycle. */
		objective { doc /* Adequate walk and clearance time. */ }
	}

	// ========== Optional: actions for lamp commands (placeholder) ==========
	action def SetMainGreen {
		doc /* Command Main Street to green. */
	}

	action def SetMainYellow {
		doc /* Command Main Street to yellow. */
	}

	action def SetSideGreen {
		doc /* Command Side Street to green. */
	}

	action def SetSideYellow {
		doc /* Command Side Street to yellow. */
	}

	action def SetAllFlash {
		doc /* Command all approaches to flash. */
	}

	// ========== Optional: parametrics ==========
	constraint def ClearanceTimeConstraint {
		doc /* Yellow time >= f(speed, distance) for clearance. */
		in approachSpeed : Real;
		in clearanceDistance : Real;
		in yellowTime : TimeValue;
		yellowTime >= clearanceDistance / approachSpeed
	}

}
