# META
~~~sexpr
(snapshot (type semantic) (description "SysML Example (Room Model): RoomModel"))
~~~
# SOURCE
~~~sysml
// SysML v2 Interpretation of the SysML v1 Room Connection Example
package RoomModel { 
    package RoomDefinitionModelLibrary{
        private import Port_Definitions::*;
        private import Flow_Definitions::*;
        package Part_Definitions{
            // Rooms
            part def Classroom {
                port classEntry: EntryWay_to_Classroom;
            }
            part def Storageroom {
                port storageEntry: EntryWay_to_Storageroom;
            }
            part def Hallway {
                // conjugate ports with ~
                port hallExit_to_Classroom: ~EntryWay_to_Classroom;
                port hallExit_to_Storageroom: ~EntryWay_to_Storageroom;
            }
        }
        package Port_Definitions{
            port def EntryWay_to_Classroom {
                //flow properties
                in ref student:Student;
                in ref teacher:Teacher;
                in ref furniture:Furniture;
                in ref air:Air;
            }
            port def EntryWay_to_Storageroom {
                //flow properties
                in ref furniture:  Furniture;
                in ref air: Air;
            }
        }
        package Flow_Definitions {
                // Conveyed items between Hallway, Classroom, and Storageroom
                part def Air;
                part def Furniture;
                part def Student;
                part def Teacher;
        }
    }
    package Room_Configuration{
        // defining the parts and their interconnection in context 
        private import RoomDefinitionModelLibrary::*;
        private import RoomDefinitionModelLibrary::Part_Definitions::*;
        private import RoomDefinitionModelLibrary::Port_Definitions::*;
        private import RoomDefinitionModelLibrary::Flow_Definitions::*;
        part roomContext{
            part c:Classroom;
            part s:Storageroom;
            part h:Hallway;
            
    		//  Connectors and item flows between hallway and classroom
            flow HallToClassroom_Air
                from h.hallExit_to_Classroom.air
                to c.classEntry.air;
            flow HallToClassroom_Furniture
                from h.hallExit_to_Classroom.furniture
                to c.classEntry.furniture;
            flow HallToClassroom_Student
                from h.hallExit_to_Classroom.student
                to c.classEntry.student;
            flow HallToClassroom_Teacher
                from h.hallExit_to_Classroom.teacher
                to c.classEntry.teacher;
            flow HallToStorageroom_Air
                from h.hallExit_to_Storageroom.air
                to s.storageEntry.air;
            flow HallToStorageroom_Furniture
                from h.hallExit_to_Storageroom.furniture
                to s.storageEntry.furniture;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "room_model.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package RoomModel {
    package RoomDefinitionModelLibrary {
        private import Port_Definitions::*;
        private import Flow_Definitions::*;
        package Part_Definitions {
            part def Classroom {
                port classEntry : EntryWay_to_Classroom;
            }
            part def Storageroom {
                port storageEntry : EntryWay_to_Storageroom;
            }
            part def Hallway {
                port hallExit_to_Classroom : ~EntryWay_to_Classroom;
                port hallExit_to_Storageroom : ~EntryWay_to_Storageroom;
            }
        }
        package Port_Definitions {
            port def EntryWay_to_Classroom {
                in ref student : Student;
                in ref teacher : Teacher;
                in ref furniture : Furniture;
                in ref air : Air;
            }
            port def EntryWay_to_Storageroom {
                in ref furniture : Furniture;
                in ref air : Air;
            }
        }
        package Flow_Definitions {
            part def Air;
            part def Furniture;
            part def Student;
            part def Teacher;
        }
    }
    package Room_Configuration {
        private import RoomDefinitionModelLibrary::*;
        private import RoomDefinitionModelLibrary::Part_Definitions::*;
        private import RoomDefinitionModelLibrary::Port_Definitions::*;
        private import RoomDefinitionModelLibrary::Flow_Definitions::*;
        part roomContext {
            part c : Classroom;
            part s : Storageroom;
            part h : Hallway;
            flow HallToClassroom_Air from h.hallExit_to_Classroom.air to c.classEntry.air;
            flow HallToClassroom_Furniture from h.hallExit_to_Classroom.furniture to c.classEntry.furniture;
            flow HallToClassroom_Student from h.hallExit_to_Classroom.student to c.classEntry.student;
            flow HallToClassroom_Teacher from h.hallExit_to_Classroom.teacher to c.classEntry.teacher;
            flow HallToStorageroom_Air from h.hallExit_to_Storageroom.air to s.storageEntry.air;
            flow HallToStorageroom_Furniture from h.hallExit_to_Storageroom.furniture to s.storageEntry.furniture;
        }
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 151) (line 4) (column 24) (len 16)) (segments (segment 0 (token "Port_Definitions") (name "Port_Definitions") (separator none) (span (offset 151) (line 4) (column 24) (len 16)))))
    (reference r1 (scope relative) (span (offset 195) (line 5) (column 24) (len 16)) (segments (segment 0 (token "Flow_Definitions") (name "Flow_Definitions") (separator none) (span (offset 195) (line 5) (column 24) (len 16)))))
    (reference r2 (scope relative) (span (offset 337) (line 9) (column 34) (len 21)) (segments (segment 0 (token "EntryWay_to_Classroom") (name "EntryWay_to_Classroom") (separator none) (span (offset 337) (line 9) (column 34) (len 21)))))
    (reference r3 (scope relative) (span (offset 444) (line 12) (column 36) (len 23)) (segments (segment 0 (token "EntryWay_to_Storageroom") (name "EntryWay_to_Storageroom") (separator none) (span (offset 444) (line 12) (column 36) (len 23)))))
    (reference r4 (scope relative) (span (offset 601) (line 16) (column 46) (len 21)) (segments (segment 0 (token "EntryWay_to_Classroom") (name "EntryWay_to_Classroom") (separator none) (span (offset 601) (line 16) (column 46) (len 21)))))
    (reference r5 (scope relative) (span (offset 671) (line 17) (column 48) (len 23)) (segments (segment 0 (token "EntryWay_to_Storageroom") (name "EntryWay_to_Storageroom") (separator none) (span (offset 671) (line 17) (column 48) (len 23)))))
    (reference r6 (scope relative) (span (offset 1573) (line 44) (column 24) (len 26)) (segments (segment 0 (token "RoomDefinitionModelLibrary") (name "RoomDefinitionModelLibrary") (separator none) (span (offset 1573) (line 44) (column 24) (len 26)))))
    (reference r7 (scope relative) (span (offset 1627) (line 45) (column 24) (len 44)) (segments (segment 0 (token "RoomDefinitionModelLibrary") (name "RoomDefinitionModelLibrary") (separator none) (span (offset 1627) (line 45) (column 24) (len 26))) (segment 1 (token "Part_Definitions") (name "Part_Definitions") (separator colon-colon) (span (offset 1655) (line 45) (column 52) (len 16)))))
    (reference r8 (scope relative) (span (offset 1699) (line 46) (column 24) (len 44)) (segments (segment 0 (token "RoomDefinitionModelLibrary") (name "RoomDefinitionModelLibrary") (separator none) (span (offset 1699) (line 46) (column 24) (len 26))) (segment 1 (token "Port_Definitions") (name "Port_Definitions") (separator colon-colon) (span (offset 1727) (line 46) (column 52) (len 16)))))
    (reference r9 (scope relative) (span (offset 1771) (line 47) (column 24) (len 44)) (segments (segment 0 (token "RoomDefinitionModelLibrary") (name "RoomDefinitionModelLibrary") (separator none) (span (offset 1771) (line 47) (column 24) (len 26))) (segment 1 (token "Flow_Definitions") (name "Flow_Definitions") (separator colon-colon) (span (offset 1799) (line 47) (column 52) (len 16)))))
    (reference r10 (scope relative) (span (offset 1865) (line 49) (column 20) (len 9)) (segments (segment 0 (token "Classroom") (name "Classroom") (separator none) (span (offset 1865) (line 49) (column 20) (len 9)))))
    (reference r11 (scope relative) (span (offset 1895) (line 50) (column 20) (len 11)) (segments (segment 0 (token "Storageroom") (name "Storageroom") (separator none) (span (offset 1895) (line 50) (column 20) (len 11)))))
    (reference r12 (scope relative) (span (offset 1927) (line 51) (column 20) (len 7)) (segments (segment 0 (token "Hallway") (name "Hallway") (separator none) (span (offset 1927) (line 51) (column 20) (len 7)))))
    (reference r13 (scope relative) (span (offset 2073) (line 55) (column 22) (len 27)) (segments (segment 0 (token "h") (name "h") (separator none) (span (offset 2073) (line 55) (column 22) (len 1))) (segment 1 (token "hallExit_to_Classroom") (name "hallExit_to_Classroom") (separator dot) (span (offset 2075) (line 55) (column 24) (len 21))) (segment 2 (token "air") (name "air") (separator dot) (span (offset 2097) (line 55) (column 46) (len 3)))))
    (reference r14 (scope relative) (span (offset 2120) (line 56) (column 20) (len 16)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 2120) (line 56) (column 20) (len 1))) (segment 1 (token "classEntry") (name "classEntry") (separator dot) (span (offset 2122) (line 56) (column 22) (len 10))) (segment 2 (token "air") (name "air") (separator dot) (span (offset 2133) (line 56) (column 33) (len 3)))))
    (reference r15 (scope relative) (span (offset 2202) (line 58) (column 22) (len 33)) (segments (segment 0 (token "h") (name "h") (separator none) (span (offset 2202) (line 58) (column 22) (len 1))) (segment 1 (token "hallExit_to_Classroom") (name "hallExit_to_Classroom") (separator dot) (span (offset 2204) (line 58) (column 24) (len 21))) (segment 2 (token "furniture") (name "furniture") (separator dot) (span (offset 2226) (line 58) (column 46) (len 9)))))
    (reference r16 (scope relative) (span (offset 2255) (line 59) (column 20) (len 22)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 2255) (line 59) (column 20) (len 1))) (segment 1 (token "classEntry") (name "classEntry") (separator dot) (span (offset 2257) (line 59) (column 22) (len 10))) (segment 2 (token "furniture") (name "furniture") (separator dot) (span (offset 2268) (line 59) (column 33) (len 9)))))
    (reference r17 (scope relative) (span (offset 2341) (line 61) (column 22) (len 31)) (segments (segment 0 (token "h") (name "h") (separator none) (span (offset 2341) (line 61) (column 22) (len 1))) (segment 1 (token "hallExit_to_Classroom") (name "hallExit_to_Classroom") (separator dot) (span (offset 2343) (line 61) (column 24) (len 21))) (segment 2 (token "student") (name "student") (separator dot) (span (offset 2365) (line 61) (column 46) (len 7)))))
    (reference r18 (scope relative) (span (offset 2392) (line 62) (column 20) (len 20)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 2392) (line 62) (column 20) (len 1))) (segment 1 (token "classEntry") (name "classEntry") (separator dot) (span (offset 2394) (line 62) (column 22) (len 10))) (segment 2 (token "student") (name "student") (separator dot) (span (offset 2405) (line 62) (column 33) (len 7)))))
    (reference r19 (scope relative) (span (offset 2476) (line 64) (column 22) (len 31)) (segments (segment 0 (token "h") (name "h") (separator none) (span (offset 2476) (line 64) (column 22) (len 1))) (segment 1 (token "hallExit_to_Classroom") (name "hallExit_to_Classroom") (separator dot) (span (offset 2478) (line 64) (column 24) (len 21))) (segment 2 (token "teacher") (name "teacher") (separator dot) (span (offset 2500) (line 64) (column 46) (len 7)))))
    (reference r20 (scope relative) (span (offset 2527) (line 65) (column 20) (len 20)) (segments (segment 0 (token "c") (name "c") (separator none) (span (offset 2527) (line 65) (column 20) (len 1))) (segment 1 (token "classEntry") (name "classEntry") (separator dot) (span (offset 2529) (line 65) (column 22) (len 10))) (segment 2 (token "teacher") (name "teacher") (separator dot) (span (offset 2540) (line 65) (column 33) (len 7)))))
    (reference r21 (scope relative) (span (offset 2609) (line 67) (column 22) (len 29)) (segments (segment 0 (token "h") (name "h") (separator none) (span (offset 2609) (line 67) (column 22) (len 1))) (segment 1 (token "hallExit_to_Storageroom") (name "hallExit_to_Storageroom") (separator dot) (span (offset 2611) (line 67) (column 24) (len 23))) (segment 2 (token "air") (name "air") (separator dot) (span (offset 2635) (line 67) (column 48) (len 3)))))
    (reference r22 (scope relative) (span (offset 2658) (line 68) (column 20) (len 18)) (segments (segment 0 (token "s") (name "s") (separator none) (span (offset 2658) (line 68) (column 20) (len 1))) (segment 1 (token "storageEntry") (name "storageEntry") (separator dot) (span (offset 2660) (line 68) (column 22) (len 12))) (segment 2 (token "air") (name "air") (separator dot) (span (offset 2673) (line 68) (column 35) (len 3)))))
    (reference r23 (scope relative) (span (offset 2744) (line 70) (column 22) (len 35)) (segments (segment 0 (token "h") (name "h") (separator none) (span (offset 2744) (line 70) (column 22) (len 1))) (segment 1 (token "hallExit_to_Storageroom") (name "hallExit_to_Storageroom") (separator dot) (span (offset 2746) (line 70) (column 24) (len 23))) (segment 2 (token "furniture") (name "furniture") (separator dot) (span (offset 2770) (line 70) (column 48) (len 9)))))
    (reference r24 (scope relative) (span (offset 2799) (line 71) (column 20) (len 24)) (segments (segment 0 (token "s") (name "s") (separator none) (span (offset 2799) (line 71) (column 20) (len 1))) (segment 1 (token "storageEntry") (name "storageEntry") (separator dot) (span (offset 2801) (line 71) (column 22) (len 12))) (segment 2 (token "furniture") (name "furniture") (separator dot) (span (offset 2814) (line 71) (column 35) (len 9)))))
  )
  (root (package (name "RoomModel") (body brace (package (name "RoomDefinitionModelLibrary") (body brace (import (target (span (span (offset 151) (line 4) (column 24) (len 19))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 167) (line 4) (column 40) (len 3))) (separator (span (offset 167) (line 4) (column 40) (len 2))) (marker (span (offset 169) (line 4) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 195) (line 5) (column 24) (len 19))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 211) (line 5) (column 40) (len 3))) (separator (span (offset 211) (line 5) (column 40) (len 2))) (marker (span (offset 213) (line 5) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Part_Definitions") (body brace (part-def (name "Classroom") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "classEntry") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Storageroom") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "storageEntry") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Hallway") (modifiers) (body brace (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "hallExit_to_Classroom") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r4)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "hallExit_to_Storageroom") (short-name none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r5)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (package (name "Port_Definitions") (body brace (port-def (name "EntryWay_to_Classroom") (modifiers) (specializes none) (body brace (in-out-declaration) (in-out-declaration) (in-out-declaration) (in-out-declaration))) (port-def (name "EntryWay_to_Storageroom") (modifiers) (specializes none) (body brace (in-out-declaration) (in-out-declaration))))) (package (name "Flow_Definitions") (body brace (part-def (name "Air") (modifiers) (body semicolon)) (part-def (name "Furniture") (modifiers) (body semicolon)) (part-def (name "Student") (modifiers) (body semicolon)) (part-def (name "Teacher") (modifiers) (body semicolon)))))) (package (name "Room_Configuration") (body brace (import (target (span (span (offset 1573) (line 44) (column 24) (len 29))) (all none) (ref r6) (shape (namespace (wildcard-suffix (span (span (offset 1599) (line 44) (column 50) (len 3))) (separator (span (offset 1599) (line 44) (column 50) (len 2))) (marker (span (offset 1601) (line 44) (column 52) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1627) (line 45) (column 24) (len 47))) (all none) (ref r7) (shape (namespace (wildcard-suffix (span (span (offset 1671) (line 45) (column 68) (len 3))) (separator (span (offset 1671) (line 45) (column 68) (len 2))) (marker (span (offset 1673) (line 45) (column 70) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1699) (line 46) (column 24) (len 47))) (all none) (ref r8) (shape (namespace (wildcard-suffix (span (span (offset 1743) (line 46) (column 68) (len 3))) (separator (span (offset 1743) (line 46) (column 68) (len 2))) (marker (span (offset 1745) (line 46) (column 70) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1771) (line 47) (column 24) (len 47))) (all none) (ref r9) (shape (namespace (wildcard-suffix (span (span (offset 1815) (line 47) (column 68) (len 3))) (separator (span (offset 1815) (line 47) (column 68) (len 2))) (marker (span (offset 1817) (line 47) (column 70) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "roomContext") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body brace (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "c") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r10)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "s") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r11)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (part-usage (then false) (prefix (direction none) (derived false) (variance none) (constant false) (reference false) (individual false) (portion none) (extensions)) (declaration-name "h") (short-name none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r12)))) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (value none) (body semicolon)) (flow-usage (kind flow) (visibility none) (declaration (declared (name "HallToClassroom_Air") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints (from (connector-end (multiplicity none) (target (ref r13)) (references none))) (to (connector-end (multiplicity none) (target (ref r14)) (references none)))))) (body (body semicolon))) (flow-usage (kind flow) (visibility none) (declaration (declared (name "HallToClassroom_Furniture") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints (from (connector-end (multiplicity none) (target (ref r15)) (references none))) (to (connector-end (multiplicity none) (target (ref r16)) (references none)))))) (body (body semicolon))) (flow-usage (kind flow) (visibility none) (declaration (declared (name "HallToClassroom_Student") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints (from (connector-end (multiplicity none) (target (ref r17)) (references none))) (to (connector-end (multiplicity none) (target (ref r18)) (references none)))))) (body (body semicolon))) (flow-usage (kind flow) (visibility none) (declaration (declared (name "HallToClassroom_Teacher") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints (from (connector-end (multiplicity none) (target (ref r19)) (references none))) (to (connector-end (multiplicity none) (target (ref r20)) (references none)))))) (body (body semicolon))) (flow-usage (kind flow) (visibility none) (declaration (declared (name "HallToStorageroom_Air") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints (from (connector-end (multiplicity none) (target (ref r21)) (references none))) (to (connector-end (multiplicity none) (target (ref r22)) (references none)))))) (body (body semicolon))) (flow-usage (kind flow) (visibility none) (declaration (declared (name "HallToStorageroom_Furniture") (short-name none) (typing none) (multiplicity none) (multiplicity-modifiers (ordering none) (uniqueness none)) (subsets none) (redefines none) (references none) (crosses none) (intersects none)) (value none) (payload none) (endpoints (from (connector-end (multiplicity none) (target (ref r23)) (references none))) (to (connector-end (multiplicity none) (target (ref r24)) (references none)))))) (body (body semicolon))))))))))
)
~~~
