# META
~~~sexpr
(snapshot (type semantic) (description "SysML Training 39 (Metadata): Metadata Example-1"))
~~~
# SOURCE
~~~sysml
package 'Metadata Example-1' {
	
	metadata def SafetyFeature;
	metadata def SecurityFeature {
		:> annotatedElement : SysML::PartDefinition;
		:> annotatedElement : SysML::PartUsage;
	}
	
	metadata SafetyFeature about 
		vehicle::interior::seatBelt,
		vehicle::interior::driverAirBag,
		vehicle::bodyAssy::bumper;
	
	metadata SecurityFeature about
		vehicle::interior::alarm,
		vehicle::bodyAssy::keylessEntry;
		
	part vehicle {
		part interior {
			part alarm;
			part seatBelt[2];
			part frontSeat[2];
			part driverAirBag;
		}
		part bodyAssy {
			part body;
			part bumper;
			part keylessEntry;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "39_metadata_example_1.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Metadata Example-1' {
    metadata def SafetyFeature;
    metadata def SecurityFeature {
        attribute :> annotatedElement : SysML::PartDefinition;
        attribute :> annotatedElement : SysML::PartUsage;
    }
    metadata SafetyFeature about vehicle::interior::seatBelt, vehicle::interior::driverAirBag, vehicle::bodyAssy::bumper;
    metadata SecurityFeature about vehicle::interior::alarm, vehicle::bodyAssy::keylessEntry;
    part vehicle {
        part interior {
            part alarm;
            part seatBelt[2];
            part frontSeat[2];
            part driverAirBag;
        }
        part bodyAssy {
            part body;
            part bumper;
            part keylessEntry;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 118) (line 5) (column 25) (len 21)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 118) (line 5) (column 25) (len 5))) (segment 1 (token "PartDefinition") (name "PartDefinition") (separator colon-colon) (span (offset 125) (line 5) (column 32) (len 14)))))
    (reference r1 (scope relative) (span (offset 99) (line 5) (column 6) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 99) (line 5) (column 6) (len 16)))))
    (reference r2 (scope relative) (span (offset 165) (line 6) (column 25) (len 16)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 165) (line 6) (column 25) (len 5))) (segment 1 (token "PartUsage") (name "PartUsage") (separator colon-colon) (span (offset 172) (line 6) (column 32) (len 9)))))
    (reference r3 (scope relative) (span (offset 146) (line 6) (column 6) (len 16)) (segments (segment 0 (token "annotatedElement") (name "annotatedElement") (separator none) (span (offset 146) (line 6) (column 6) (len 16)))))
    (reference r4 (scope relative) (span (offset 221) (line 10) (column 3) (len 27)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 221) (line 10) (column 3) (len 7))) (segment 1 (token "interior") (name "interior") (separator colon-colon) (span (offset 230) (line 10) (column 12) (len 8))) (segment 2 (token "seatBelt") (name "seatBelt") (separator colon-colon) (span (offset 240) (line 10) (column 22) (len 8)))))
    (reference r5 (scope relative) (span (offset 252) (line 11) (column 3) (len 31)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 252) (line 11) (column 3) (len 7))) (segment 1 (token "interior") (name "interior") (separator colon-colon) (span (offset 261) (line 11) (column 12) (len 8))) (segment 2 (token "driverAirBag") (name "driverAirBag") (separator colon-colon) (span (offset 271) (line 11) (column 22) (len 12)))))
    (reference r6 (scope relative) (span (offset 287) (line 12) (column 3) (len 25)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 287) (line 12) (column 3) (len 7))) (segment 1 (token "bodyAssy") (name "bodyAssy") (separator colon-colon) (span (offset 296) (line 12) (column 12) (len 8))) (segment 2 (token "bumper") (name "bumper") (separator colon-colon) (span (offset 306) (line 12) (column 22) (len 6)))))
    (reference r7 (scope relative) (span (offset 350) (line 15) (column 3) (len 24)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 350) (line 15) (column 3) (len 7))) (segment 1 (token "interior") (name "interior") (separator colon-colon) (span (offset 359) (line 15) (column 12) (len 8))) (segment 2 (token "alarm") (name "alarm") (separator colon-colon) (span (offset 369) (line 15) (column 22) (len 5)))))
    (reference r8 (scope relative) (span (offset 378) (line 16) (column 3) (len 31)) (segments (segment 0 (token "vehicle") (name "vehicle") (separator none) (span (offset 378) (line 16) (column 3) (len 7))) (segment 1 (token "bodyAssy") (name "bodyAssy") (separator colon-colon) (span (offset 387) (line 16) (column 12) (len 8))) (segment 2 (token "keylessEntry") (name "keylessEntry") (separator colon-colon) (span (offset 397) (line 16) (column 22) (len 12)))))
  )
  (root (package (name "Metadata Example-1") (body brace (metadata-def (name "SafetyFeature") (abstract false) (specializes none) (body semicolon)) (metadata-def (name "SecurityFeature") (abstract false) (specializes none) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r0)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r1)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets (relationship (kind subsets) (implied false) (targets (ref r3)))) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (metadata-usage (declaration-name "SafetyFeature") (type none) (about (ref r4) (ref r5) (ref r6)) (body semicolon)) (metadata-usage (declaration-name "SecurityFeature") (type none) (about (ref r7) (ref r8)) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "vehicle") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "interior") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "alarm") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "seatBelt") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 480) (line 21) (column 18) (len 1)) (integer 2))) (upper (expression (span (offset 480) (line 21) (column 18) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "frontSeat") (short-name none) (typing none) (multiplicity (lower (expression (span (offset 502) (line 22) (column 19) (len 1)) (integer 2))) (upper (expression (span (offset 502) (line 22) (column 19) (len 1)) (integer 2)))) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "driverAirBag") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bodyAssy") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "body") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "bumper") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "keylessEntry") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)))))))))
)
~~~
