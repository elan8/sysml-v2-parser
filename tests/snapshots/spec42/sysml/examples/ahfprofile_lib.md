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
        part :>> locclouds : LocalCloudDD[1..*];
    }
    part def LocalCloudDD :> LocalCloudDesign {
        part :>> systems : SysDD[1..*];
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
    (reference r4 (scope relative) (span (offset 459) (line 18) (column 18) (len 16)) (segments (segment 0 (token "LocalCloudDesign") (name "LocalCloudDesign") (separator none) (span (offset 459) (line 18) (column 18) (len 16)))))
    (reference r5 (scope relative) (span (offset 512) (line 21) (column 25) (len 16)) (segments (segment 0 (token "SysLocalCloudsDD") (name "SysLocalCloudsDD") (separator none) (span (offset 512) (line 21) (column 25) (len 16)))))
    (reference r6 (scope relative) (span (offset 642) (line 26) (column 16) (len 4)) (segments (segment 0 (token "SysD") (name "SysD") (separator none) (span (offset 642) (line 26) (column 16) (len 4)))))
    (reference r7 (scope relative) (span (offset 726) (line 32) (column 18) (len 2)) (segments (segment 0 (token "SD") (name "SD") (separator none) (span (offset 726) (line 32) (column 18) (len 2)))))
    (reference r8 (scope relative) (span (offset 760) (line 33) (column 25) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 760) (line 33) (column 25) (len 6)))))
    (reference r9 (scope relative) (span (offset 789) (line 34) (column 22) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 789) (line 34) (column 22) (len 6)))))
    (reference r10 (scope relative) (span (offset 817) (line 35) (column 21) (len 7)) (segments (segment 0 (token "Integer") (name "Integer") (separator none) (span (offset 817) (line 35) (column 21) (len 7)))))
    (reference r11 (scope relative) (span (offset 877) (line 39) (column 18) (len 2)) (segments (segment 0 (token "SD") (name "SD") (separator none) (span (offset 877) (line 39) (column 18) (len 2)))))
    (reference r12 (scope relative) (span (offset 963) (line 42) (column 27) (len 6)) (segments (segment 0 (token "String") (name "String") (separator none) (span (offset 963) (line 42) (column 27) (len 6)))))
    (reference r13 (scope relative) (span (offset 994) (line 45) (column 19) (len 2)) (segments (segment 0 (token "SD") (name "SD") (separator none) (span (offset 994) (line 45) (column 19) (len 2)))))
    (reference r14 (scope relative) (span (offset 1063) (line 48) (column 13) (len 3)) (segments (segment 0 (token "IDD") (name "IDD") (separator none) (span (offset 1063) (line 48) (column 13) (len 3)))))
    (reference r15 (scope relative) (span (offset 1240) (line 54) (column 22) (len 12)) (segments (segment 0 (token "LocalCloudDD") (name "LocalCloudDD") (separator none) (span (offset 1240) (line 54) (column 22) (len 12)))))
    (reference r16 (scope relative) (span (offset 1230) (line 54) (column 12) (len 9)) (segments (segment 0 (token "locclouds") (name "locclouds") (separator none) (span (offset 1230) (line 54) (column 12) (len 9)))))
    (reference r17 (scope relative) (span (offset 1348) (line 58) (column 20) (len 5)) (segments (segment 0 (token "SysDD") (name "SysDD") (separator none) (span (offset 1348) (line 58) (column 20) (len 5)))))
    (reference r18 (scope relative) (span (offset 1340) (line 58) (column 12) (len 7)) (segments (segment 0 (token "systems") (name "systems") (separator none) (span (offset 1340) (line 58) (column 12) (len 7)))))
    (reference r19 (scope relative) (span (offset 1451) (line 64) (column 21) (len 4)) (segments (segment 0 (token "SDDD") (name "SDDD") (separator none) (span (offset 1451) (line 64) (column 21) (len 4)))))
    (reference r20 (scope relative) (span (offset 1442) (line 64) (column 12) (len 8)) (segments (segment 0 (token "services") (name "services") (separator none) (span (offset 1442) (line 64) (column 12) (len 8)))))
    (reference r21 (scope relative) (span (offset 1571) (line 70) (column 17) (len 29)) (segments (segment 0 (token "Metaobjects") (name "Metaobjects") (separator none) (span (offset 1571) (line 70) (column 17) (len 11))) (segment 1 (token "SemanticMetadata") (name "SemanticMetadata") (separator colon-colon) (span (offset 1584) (line 70) (column 30) (len 16)))))
    (reference r22 (scope relative) (span (offset 1618) (line 71) (column 17) (len 13)) (segments (segment 0 (token "AHFProfileLib") (name "AHFProfileLib") (separator none) (span (offset 1618) (line 71) (column 17) (len 13)))))
    (reference r23 (scope relative) (span (offset 1653) (line 73) (column 17) (len 2)) (segments (segment 0 (token "SD") (name "SD") (separator none) (span (offset 1653) (line 73) (column 17) (len 2)))))
    (reference r24 (scope relative) (span (offset 1695) (line 74) (column 39) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 1695) (line 74) (column 39) (len 16)))))
    (reference r25 (scope relative) (span (offset 1865) (line 77) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 1865) (line 77) (column 7) (len 8)))))
    (reference r26 (scope relative) (span (offset 1882) (line 77) (column 24) (len 9)) (segments (segment 0 (token "global_sd") (name "global_sd") (separator none) (span (offset 1882) (line 77) (column 24) (len 9)))))
    (reference r27 (scope relative) (span (offset 1897) (line 77) (column 39) (len 16)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 1897) (line 77) (column 39) (len 5))) (segment 1 (token "PortUsage") (name "PortUsage") (separator colon-colon) (span (offset 1904) (line 77) (column 46) (len 9)))))
    (reference r28 (scope relative) (span (offset 1966) (line 80) (column 47) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 1966) (line 80) (column 47) (len 16)))))
    (reference r29 (scope relative) (span (offset 1990) (line 81) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 1990) (line 81) (column 7) (len 8)))))
    (reference r30 (scope relative) (span (offset 2001) (line 81) (column 18) (len 17)) (segments (segment 0 (token "system_of_systems") (name "system_of_systems") (separator none) (span (offset 2001) (line 81) (column 18) (len 17)))))
    (reference r31 (scope relative) (span (offset 2024) (line 81) (column 41) (len 16)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2024) (line 81) (column 41) (len 5))) (segment 1 (token "PartUsage") (name "PartUsage") (separator colon-colon) (span (offset 2031) (line 81) (column 48) (len 9)))))
    (reference r32 (scope relative) (span (offset 2092) (line 84) (column 46) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 2092) (line 84) (column 46) (len 16)))))
    (reference r33 (scope relative) (span (offset 2116) (line 85) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 2116) (line 85) (column 7) (len 8)))))
    (reference r34 (scope relative) (span (offset 2133) (line 85) (column 24) (len 28)) (segments (segment 0 (token "system_of_systems") (name "system_of_systems") (separator none) (span (offset 2133) (line 85) (column 24) (len 17))) (segment 1 (token "locclouds") (name "locclouds") (separator colon-colon) (span (offset 2152) (line 85) (column 43) (len 9)))))
    (reference r35 (scope relative) (span (offset 2167) (line 85) (column 58) (len 16)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2167) (line 85) (column 58) (len 5))) (segment 1 (token "PartUsage") (name "PartUsage") (separator colon-colon) (span (offset 2174) (line 85) (column 65) (len 9)))))
    (reference r36 (scope relative) (span (offset 2229) (line 88) (column 40) (len 16)) (segments (segment 0 (token "SemanticMetadata") (name "SemanticMetadata") (separator none) (span (offset 2229) (line 88) (column 40) (len 16)))))
    (reference r37 (scope relative) (span (offset 2253) (line 89) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 2253) (line 89) (column 7) (len 8)))))
    (reference r38 (scope relative) (span (offset 2270) (line 89) (column 24) (len 37)) (segments (segment 0 (token "system_of_systems") (name "system_of_systems") (separator none) (span (offset 2270) (line 89) (column 24) (len 17))) (segment 1 (token "locclouds") (name "locclouds") (separator colon-colon) (span (offset 2289) (line 89) (column 43) (len 9))) (segment 2 (token "systems") (name "systems") (separator colon-colon) (span (offset 2300) (line 89) (column 54) (len 7)))))
    (reference r39 (scope relative) (span (offset 2313) (line 89) (column 67) (len 16)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2313) (line 89) (column 67) (len 5))) (segment 1 (token "PartUsage") (name "PartUsage") (separator colon-colon) (span (offset 2320) (line 89) (column 74) (len 9)))))
    (reference r40 (scope relative) (span (offset 2445) (line 93) (column 36) (len 10)) (segments (segment 0 (token "SDMetadata") (name "SDMetadata") (separator none) (span (offset 2445) (line 93) (column 36) (len 10)))))
    (reference r41 (scope relative) (span (offset 2555) (line 95) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 2555) (line 95) (column 7) (len 8)))))
    (reference r42 (scope relative) (span (offset 2566) (line 95) (column 18) (len 10)) (segments (segment 0 (token "SDDD") (name "SDDD") (separator none) (span (offset 2566) (line 95) (column 18) (len 4))) (segment 1 (token "idds") (name "idds") (separator colon-colon) (span (offset 2572) (line 95) (column 24) (len 4)))))
    (reference r43 (scope relative) (span (offset 2582) (line 95) (column 34) (len 16)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2582) (line 95) (column 34) (len 5))) (segment 1 (token "PortUsage") (name "PortUsage") (separator colon-colon) (span (offset 2589) (line 95) (column 41) (len 9)))))
    (reference r44 (scope relative) (span (offset 2648) (line 99) (column 19) (len 4)) (segments (segment 0 (token "SDDD") (name "SDDD") (separator none) (span (offset 2648) (line 99) (column 19) (len 4)))))
    (reference r45 (scope relative) (span (offset 2696) (line 100) (column 43) (len 10)) (segments (segment 0 (token "SDMetadata") (name "SDMetadata") (separator none) (span (offset 2696) (line 100) (column 43) (len 10)))))
    (reference r46 (scope relative) (span (offset 2803) (line 102) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 2803) (line 102) (column 7) (len 8)))))
    (reference r47 (scope relative) (span (offset 2814) (line 102) (column 18) (len 11)) (segments (segment 0 (token "global_sddd") (name "global_sddd") (separator none) (span (offset 2814) (line 102) (column 18) (len 11)))))
    (reference r48 (scope relative) (span (offset 2831) (line 102) (column 35) (len 16)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2831) (line 102) (column 35) (len 5))) (segment 1 (token "PortUsage") (name "PortUsage") (separator colon-colon) (span (offset 2838) (line 102) (column 42) (len 9)))))
    (reference r49 (scope relative) (span (offset 2903) (line 105) (column 50) (len 19)) (segments (segment 0 (token "LocalCloudsMetadata") (name "LocalCloudsMetadata") (separator none) (span (offset 2903) (line 105) (column 50) (len 19)))))
    (reference r50 (scope relative) (span (offset 2930) (line 106) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 2930) (line 106) (column 7) (len 8)))))
    (reference r51 (scope relative) (span (offset 2941) (line 106) (column 18) (len 28)) (segments (segment 0 (token "system_of_systems") (name "system_of_systems") (separator none) (span (offset 2941) (line 106) (column 18) (len 17))) (segment 1 (token "locclouds") (name "locclouds") (separator colon-colon) (span (offset 2960) (line 106) (column 37) (len 9)))))
    (reference r52 (scope relative) (span (offset 2975) (line 106) (column 52) (len 16)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 2975) (line 106) (column 52) (len 5))) (segment 1 (token "PartUsage") (name "PartUsage") (separator colon-colon) (span (offset 2982) (line 106) (column 59) (len 9)))))
    (reference r53 (scope relative) (span (offset 3019) (line 109) (column 22) (len 12)) (segments (segment 0 (token "LocalCloudDD") (name "LocalCloudDD") (separator none) (span (offset 3019) (line 109) (column 22) (len 12)))))
    (reference r54 (scope relative) (span (offset 3056) (line 110) (column 24) (len 5)) (segments (segment 0 (token "SysDD") (name "SysDD") (separator none) (span (offset 3056) (line 110) (column 24) (len 5)))))
    (reference r55 (scope relative) (span (offset 3105) (line 111) (column 43) (len 12)) (segments (segment 0 (token "SysDMetadata") (name "SysDMetadata") (separator none) (span (offset 3105) (line 111) (column 43) (len 12)))))
    (reference r56 (scope relative) (span (offset 3268) (line 114) (column 7) (len 8)) (segments (segment 0 (token "baseType") (name "baseType") (separator none) (span (offset 3268) (line 114) (column 7) (len 8)))))
    (reference r57 (scope relative) (span (offset 3279) (line 114) (column 18) (len 16)) (segments (segment 0 (token "global_systemsdd") (name "global_systemsdd") (separator none) (span (offset 3279) (line 114) (column 18) (len 16)))))
    (reference r58 (scope relative) (span (offset 3301) (line 114) (column 40) (len 16)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 3301) (line 114) (column 40) (len 5))) (segment 1 (token "PartUsage") (name "PartUsage") (separator colon-colon) (span (offset 3308) (line 114) (column 47) (len 9)))))
  )
  (root (library-package (name "AHFProfileLib") (standard false) (body brace (import (target (span (span (offset 99) (line 3) (column 17) (len 15))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 111) (line 3) (column 29) (len 3))) (separator (span (offset 111) (line 3) (column 29) (len 2))) (marker (span (offset 113) (line 3) (column 31) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (port-def (name "SD") (specializes none) (body brace (doc) (attribute-usage (declaration-name "serviceDefinition") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r1)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "serviceURL") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "intrfce_protocol") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "SysLocalCloudsDesign") (body brace (doc) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "locclouds") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r4)))) (multiplicity (lower (expression (span (offset 476) (line 18) (column 35) (len 1)) (integer 1))) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "system_of_systems") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-def (name "LocalCloudDesign") (body brace (doc) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "systems") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r6)))) (multiplicity (lower (expression (span (offset 647) (line 26) (column 21) (len 1)) (integer 1))) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)))) (part-def (name "SysD") (body brace (doc) (port-usage (declaration-name "services") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r7)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "systemname") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r8)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "address") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r9)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (attribute-usage (declaration-name "portno") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (port-def (name "IDD") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r11)))) (body brace (doc) (attribute-usage (declaration-name "encoding_kind") (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (port-def (name "SDDD") (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r13)))) (body brace (doc) (port-usage (declaration-name "idds") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r14)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "SysLocalCloudsDD") (body brace (doc) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r15)))) (multiplicity (lower (expression (span (offset 1253) (line 54) (column 35) (len 1)) (integer 1))) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r16)))) (value none) (body semicolon)))) (part-def (name "LocalCloudDD") (body brace (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name none) (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r17)))) (multiplicity (lower (expression (span (offset 1354) (line 58) (column 26) (len 1)) (integer 1))) (upper unbounded)) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r18)))) (value none) (body semicolon)))) (part-def (name "SysDD") (body brace (doc) (port-usage (declaration-name none) (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r19)))) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r20)))) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (action-usage))))) (library-package (name "AHFProfileMetadata") (standard false) (body brace (import (target (span (span (offset 1571) (line 70) (column 17) (len 29))) (all none) (ref r21) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 1618) (line 71) (column 17) (len 16))) (all none) (ref r22) (shape (namespace (wildcard-suffix (span (span (offset 1631) (line 71) (column 30) (len 3))) (separator (span (offset 1631) (line 71) (column 30) (len 2))) (marker (span (offset 1633) (line 71) (column 32) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (port-def (name "global_sd") (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r23)))) (body semicolon)) (metadata-def (name "SDMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r24)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r25)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 1882) (line 77) (column 24) (len 31)) (meta-cast (base (expression (span (offset 1882) (line 77) (column 24) (len 9)) (ref r26))) (metaclass (ref r27))))))) (body semicolon)))) (metadata-def (name "SysLocalCloudsMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r28)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r29)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2001) (line 81) (column 18) (len 39)) (meta-cast (base (expression (span (offset 2001) (line 81) (column 18) (len 17)) (ref r30))) (metaclass (ref r31))))))) (body semicolon)))) (metadata-def (name "LocalCloudsMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r32)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r33)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 2133) (line 85) (column 24) (len 50)) (meta-cast (base (expression (span (offset 2133) (line 85) (column 24) (len 28)) (ref r34))) (metaclass (ref r35))))))) (body semicolon)))) (metadata-def (name "SysDMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r36)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r37)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default true) (expression (expression (span (offset 2270) (line 89) (column 24) (len 59)) (meta-cast (base (expression (span (offset 2270) (line 89) (column 24) (len 37)) (ref r38))) (metaclass (ref r39))))))) (body semicolon)))) (metadata-def (name "IDDMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r40)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r41)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2566) (line 95) (column 18) (len 32)) (meta-cast (base (expression (span (offset 2566) (line 95) (column 18) (len 10)) (ref r42))) (metaclass (ref r43))))))) (body semicolon)))) (port-def (name "global_sddd") (specializes (typing (kind typing) (conjugated false) (implied false) (targets (ref r44)))) (body semicolon)) (metadata-def (name "SDDDMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r45)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r46)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2814) (line 102) (column 18) (len 33)) (meta-cast (base (expression (span (offset 2814) (line 102) (column 18) (len 11)) (ref r47))) (metaclass (ref r48))))))) (body semicolon)))) (metadata-def (name "LocalCloudsDDMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r49)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r50)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 2941) (line 106) (column 18) (len 50)) (meta-cast (base (expression (span (offset 2941) (line 106) (column 18) (len 28)) (ref r51))) (metaclass (ref r52))))))) (body semicolon)))) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "global_clouddd") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r53)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "global_systemsdd") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r54)))) (multiplicity none) (multiplicity-modifiers (ordered false) (nonunique false)) (subsets none) (redefines none) (value none) (body semicolon)) (metadata-def (name "SysDDMetadata") (abstract false) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r55)))) (body brace (attribute-usage (declaration-name none) (direction none) (derived false) (usage-prefix none) (constant false) (reference false) (end false) (typing none) (subsets none) (redefines (relationship (kind redefines) (implied false) (targets (ref r56)))) (references none) (crosses none) (intersects none) (value (feature-value (kind bind) (default false) (expression (expression (span (offset 3279) (line 114) (column 18) (len 38)) (meta-cast (base (expression (span (offset 3279) (line 114) (column 18) (len 16)) (ref r57))) (metaclass (ref r58))))))) (body semicolon)))))))
)
~~~
