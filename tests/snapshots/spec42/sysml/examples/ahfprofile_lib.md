# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Arrowhead Framework): AHFProfileLib"))
~~~
# SOURCE
~~~sysml
library package AHFProfileLib {
	// Systems and Services and their functionalities
	private import ScalarValues::*;
	
	// Design level
	port def SD{
		doc /* Service definition */
		
		attribute serviceDefinition:String;
		attribute serviceURL:String;
		attribute intrfce_protocol:String; // which may be "REST" or "MQTT" etc.		 
	}	
		
	part def SysLocalCloudsDesign {
		doc /* System of Systems Definition */	

		// System of Local Clouds 
		part locclouds:LocalCloudDesign[1..*];
	}
	
	part system_of_systems:SysLocalCloudsDD; // defining a top level usage
	
	part def LocalCloudDesign {
		doc /* Local Cloud definition */

		part systems:SysD[1..*];	
	}

	part def SysD {
		doc /* System definitions */	

		port services: SD[1..*];
		attribute systemname: String;
		attribute address: String;
		attribute portno: Integer;
	}	

	// Design Description level
	port def IDD :> SD{
		doc /* Interface Design Description of services */
		
		attribute encoding_kind:String;
	}
	
	port def SDDD :> SD{
		doc /* Service Definition Design Description */
		
		port idds:IDD[*]; // nested protocol-specific services
	}	

	part def SysLocalCloudsDD :> SysLocalCloudsDesign {
		doc /* System of Systems Detailed Description */	

		part :>> locclouds:LocalCloudDD[1..*]; // the descriptions
	}

	part def LocalCloudDD :> LocalCloudDesign {
		part :>> systems:SysDD[1..*];
	}

	part def SysDD :> SysD{
		doc /* System Detailed Description */

		port :>> services:SDDD;
		action ServiceMethod[1..*]; //means general behaviors
	}
}

library package AHFProfileMetadata{
	private import Metaobjects::SemanticMetadata;
	private import AHFProfileLib::*;

	port global_sd:SD;
	metadata def <service> SDMetadata :> SemanticMetadata{
		// :>> baseType = system_of_systems.locclouds.systems.services meta SysML::PortUsage;
		// :>> baseType = SysD::services meta SysML::PortUsage;
		:>> baseType default global_sd meta SysML::PortUsage;
	}
	
	metadata def <sos> SysLocalCloudsMetadata :> SemanticMetadata{
		:>> baseType = system_of_systems meta SysML::PartUsage;
	}
	
	metadata def <cloud> LocalCloudsMetadata :> SemanticMetadata{
		:>> baseType default system_of_systems::locclouds meta SysML::PartUsage;
	}
	
	metadata def <system> SysDMetadata :> SemanticMetadata{
		:>> baseType default system_of_systems::locclouds::systems meta SysML::PartUsage;
		// :>> baseType default LocalCloudDesign::systems meta SysML::PartUsage;
	}

	metadata def <idd> IDDMetadata :> SDMetadata{
		// :>> baseType = system_of_systems.locclouds.systems.services.idd meta SysML::PortUsage;
		:>> baseType = SDDD::idds meta SysML::PortUsage;
		// :>> global_sddd.idd;
	}

	port global_sddd:SDDD;
	metadata def <servicedd> SDDDMetadata :> SDMetadata {
		// :>> baseType = system_of_systems.locclouds.systems.services meta SysML::PortUsage;
		:>> baseType = global_sddd meta SysML::PortUsage;
	}
	
	metadata def <clouddd> LocalCloudsDDMetadata :> LocalCloudsMetadata{
		:>> baseType = system_of_systems::locclouds meta SysML::PartUsage;
	}
	
	part global_clouddd:LocalCloudDD;
	part global_systemsdd:SysDD;
	metadata def <systemdd> SysDDMetadata :> SysDMetadata{
		// :>> baseType = system_of_systems.locclouds.systems meta SysML::PartUsage;
		//:>> baseType = LocalCloudDD::systems meta SysML::PartUsage;
		:>> baseType = global_systemsdd meta SysML::PartUsage;
	}	
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ahfprofile_lib.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
library package AHFProfileLib {
    private import ScalarValues::*;
    port def SD {
        doc
        /* Service definition */
        attribute serviceDefinition : String;
        attribute serviceURL : String;
        attribute intrfce_protocol : String;
    }
    part def SysLocalCloudsDesign {
        doc
        /* System of Systems Definition */
        part locclouds : LocalCloudDesign[1..*];
    }
    part system_of_systems : SysLocalCloudsDD;
    part def LocalCloudDesign {
        doc
        /* Local Cloud definition */
        part systems : SysD[1..*];
    }
    part def SysD {
        doc
        /* System definitions */
        port services : SD[1..*];
        attribute systemname : String;
        attribute address : String;
        attribute portno : Integer;
    }
    port def IDD :> SD {
        doc
        /* Interface Design Description of services */
        attribute encoding_kind : String;
    }
    port def SDDD :> SD {
        doc
        /* Service Definition Design Description */
        port idds : IDD[*];
    }
    part def SysLocalCloudsDD :> SysLocalCloudsDesign {
        doc
        /* System of Systems Detailed Description */
        part  :>> locclouds : LocalCloudDD[1..*];
    }
    part def LocalCloudDD :> LocalCloudDesign {
        part  :>> systems : SysDD[1..*];
    }
    part def SysDD :> SysD {
        doc
        /* System Detailed Description */
        port  :>> services : SDDD;
        action ServiceMethod[1..*];
    }
}

library package AHFProfileMetadata {
    private import Metaobjects::SemanticMetadata;
    private import AHFProfileLib::*;
    port def global_sd : SD;
    metadata def <service> SDMetadata :> SemanticMetadata {
        attribute :>> baseType default global_sd meta SysML::PortUsage;
    }
    metadata def <sos> SysLocalCloudsMetadata :> SemanticMetadata {
        attribute :>> baseType = system_of_systems meta SysML::PartUsage;
    }
    metadata def <cloud> LocalCloudsMetadata :> SemanticMetadata {
        attribute :>> baseType default system_of_systems::locclouds meta SysML::PartUsage;
    }
    metadata def <system> SysDMetadata :> SemanticMetadata {
        attribute :>> baseType default system_of_systems::locclouds::systems meta SysML::PartUsage;
    }
    metadata def <idd> IDDMetadata :> SDMetadata {
        attribute :>> baseType = SDDD::idds meta SysML::PortUsage;
    }
    port def global_sddd : SDDD;
    metadata def <servicedd> SDDDMetadata :> SDMetadata {
        attribute :>> baseType = global_sddd meta SysML::PortUsage;
    }
    metadata def <clouddd> LocalCloudsDDMetadata :> LocalCloudsMetadata {
        attribute :>> baseType = system_of_systems::locclouds meta SysML::PartUsage;
    }
    part global_clouddd : LocalCloudDD;
    part global_systemsdd : SysDD;
    metadata def <systemdd> SysDDMetadata :> SysDMetadata {
        attribute :>> baseType = global_systemsdd meta SysML::PartUsage;
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 99) (line 3) (column 17) (len 12)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 99) (line 3) (column 17) (len 12)))))
    (reference r1 (scope relative) (span (offset 213) (line 9) (column 31) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 213) (line 9) (column 31) (len 6)))))
    (reference r2 (scope relative) (span (offset 244) (line 10) (column 24) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 244) (line 10) (column 24) (len 6)))))
    (reference r3 (scope relative) (span (offset 281) (line 11) (column 30) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 281) (line 11) (column 30) (len 6)))))
    (reference r4 (scope relative) (span (offset 512) (line 21) (column 25) (len 16)) (segments (segment 0 (token "SysLocalCloudsDD") (name "SysLocalCloudsDD") (separator none) (span (offset 512) (line 21) (column 25) (len 16)))))
    (reference r5 (scope relative) (span (offset 726) (line 32) (column 18) (len 2)) (segments (segment 0 (token "SD") (name "SD") (separator none) (span (offset 726) (line 32) (column 18) (len 2)))))
    (reference r6 (scope relative) (span (offset 760) (line 33) (column 25) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 760) (line 33) (column 25) (len 6)))))
    (reference r7 (scope relative) (span (offset 789) (line 34) (column 22) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 789) (line 34) (column 22) (len 6)))))
    (reference r8 (scope relative) (span (offset 817) (line 35) (column 21) (len 7)) (segments (segment 0 (token "Integer") (name "Integer") (separator none) (span (offset 817) (line 35) (column 21) (len 7)))))
    (reference r9 (scope relative) (span (offset 877) (line 39) (column 18) (len 2)) (segments (segment 0 (token "SD") (name "SD") (separator none) (span (offset 877) (line 39) (column 18) (len 2)))))
    (reference r10 (scope relative) (span (offset 963) (line 42) (column 27) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 963) (line 42) (column 27) (len 6)))))
    (reference r11 (scope relative) (span (offset 994) (line 45) (column 19) (len 2)) (segments (segment 0 (token "SD") (name "SD") (separator none) (span (offset 994) (line 45) (column 19) (len 2)))))
    (reference r12 (scope relative) (span (offset 1063) (line 48) (column 13) (len 3)) (segments (segment 0 (token "IDD") (name "IDD") (separator none) (span (offset 1063) (line 48) (column 13) (len 3)))))
    (reference r13 (scope relative) (span (offset 1451) (line 64) (column 21) (len 4)) (segments (segment 0 (token "SDDD") (name "SDDD") (separator none) (span (offset 1451) (line 64) (column 21) (len 4)))))
    (reference r14 (scope relative) (span (offset 1442) (line 64) (column 12) (len 8)) (segments (segment 0 (token "services") (name "services") (separator none) (span (offset 1442) (line 64) (column 12) (len 8)))))
    (reference r15 (scope relative) (span (offset 1571) (line 70) (column 17) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 1571) (line 70) (column 17) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 1584) (line 70) (column 30) (len 16)))))
    (reference r16 (scope relative) (span (offset 1618) (line 71) (column 17) (len 13)) (segments (segment 0 (token "AHFProfileLib") (name "AHFProfileLib") (separator none) (span (offset 1618) (line 71) (column 17) (len 13)))))
    (reference r17 (scope relative) (span (offset 1653) (line 73) (column 17) (len 2)) (segments (segment 0 (token "SD") (name "SD") (separator none) (span (offset 1653) (line 73) (column 17) (len 2)))))
    (reference r18 (scope relative) (span (offset 2648) (line 99) (column 19) (len 4)) (segments (segment 0 (token "SDDD") (name "SDDD") (separator none) (span (offset 2648) (line 99) (column 19) (len 4)))))
    (reference r19 (scope relative) (span (offset 3019) (line 109) (column 22) (len 12)) (segments (segment 0 (token "LocalCloudDD") (name "LocalCloudDD") (separator none) (span (offset 3019) (line 109) (column 22) (len 12)))))
    (reference r20 (scope relative) (span (offset 3056) (line 110) (column 24) (len 5)) (segments (segment 0 (token "SysDD") (name "SysDD") (separator none) (span (offset 3056) (line 110) (column 24) (len 5)))))
  )
  (root (library-package (name "AHFProfileLib") (standard false) (body brace (import (target (span (span (offset 99) (line 3) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 111) (line 3) (column 29) (len 3))) (separator (span (offset 111) (line 3) (column 29) (len 2))) (marker (span (offset 113) (line 3) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (port-def (name "SD") (specializes none) (body brace (doc) (attribute-usage (declaration-name "serviceDefinition") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "serviceURL") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "intrfce_protocol") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "SysLocalCloudsDesign") (body brace (doc) (part-usage))) (part-usage (declaration-name "system_of_systems") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-def (name "LocalCloudDesign") (body brace (doc) (part-usage))) (part-def (name "SysD") (body brace (doc) (port-usage (declaration-name "services") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "systemname") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "address") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "portno") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (port-def (name "IDD") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r9)))) (body brace (doc) (attribute-usage (declaration-name "encoding_kind") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (port-def (name "SDDD") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r11)))) (body brace (doc) (port-usage (declaration-name "idds") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "SysLocalCloudsDD") (body brace (doc) (part-usage))) (part-def (name "LocalCloudDD") (body brace (part-usage))) (part-def (name "SysDD") (body brace (doc) (port-usage (declaration-name none) (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r13)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r14)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (action-usage))))) (library-package (name "AHFProfileMetadata") (standard false) (body brace (import (target (span (span (offset 1571) (line 70) (column 17) (len 29))) (all none) (ref r15) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1618) (line 71) (column 17) (len 16))) (all none) (ref r16) (shape (namespace (wildcard-suffix (span (span (offset 1631) (line 71) (column 30) (len 3))) (separator (span (offset 1631) (line 71) (column 30) (len 2))) (marker (span (offset 1633) (line 71) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (port-def (name "global_sd") (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (body semicolon)) (metadata-def) (metadata-def) (metadata-def) (metadata-def) (metadata-def) (port-def (name "global_sddd") (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r18)))) (body semicolon)) (metadata-def) (metadata-def) (part-usage (declaration-name "global_clouddd") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (part-usage (declaration-name "global_systemsdd") (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r20)))) (multiplicity-modifiers (ordered false) (nonunique false)) (body semicolon)) (metadata-def))))
)
~~~
