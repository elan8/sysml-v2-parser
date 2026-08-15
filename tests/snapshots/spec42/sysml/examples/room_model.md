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
  )
  (root (package (name "RoomModel") (body brace (package (name "RoomDefinitionModelLibrary") (body brace (import (target (span (span (offset 151) (line 4) (column 24) (len 19))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 167) (line 4) (column 40) (len 3))) (separator (span (offset 167) (line 4) (column 40) (len 2))) (marker (span (offset 169) (line 4) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 195) (line 5) (column 24) (len 19))) (all none) (ref r1) (shape (namespace (wildcard-suffix (span (span (offset 211) (line 5) (column 40) (len 3))) (separator (span (offset 211) (line 5) (column 40) (len 2))) (marker (span (offset 213) (line 5) (column 42) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (package (name "Part_Definitions") (body brace (part-def (name "Classroom") (body brace (port-usage (declaration-name "classEntry") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r2)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Storageroom") (body brace (port-usage (declaration-name "storageEntry") (direction none) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r3)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))) (part-def (name "Hallway") (body brace (port-usage (declaration-name "hallExit_to_Classroom") (direction none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r4)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)) (port-usage (declaration-name "hallExit_to_Storageroom") (direction none) (typing (typing (kind typing) (conjugated true) (implied false) (targets (ref r5)))) (subsets none) (redefines none) (references none) (crosses none) (intersects none) (value none) (body semicolon)))))) (package (name "Port_Definitions") (body brace (port-def (name "EntryWay_to_Classroom") (specializes none) (body brace (in-out-declaration) (in-out-declaration) (in-out-declaration) (in-out-declaration))) (port-def (name "EntryWay_to_Storageroom") (specializes none) (body brace (in-out-declaration) (in-out-declaration))))) (package (name "Flow_Definitions") (body brace (part-def (name "Air") (body semicolon)) (part-def (name "Furniture") (body semicolon)) (part-def (name "Student") (body semicolon)) (part-def (name "Teacher") (body semicolon)))))) (package (name "Room_Configuration") (body brace (import (target (span (span (offset 1573) (line 44) (column 24) (len 29))) (all none) (ref r6) (shape (namespace (wildcard-suffix (span (span (offset 1599) (line 44) (column 50) (len 3))) (separator (span (offset 1599) (line 44) (column 50) (len 2))) (marker (span (offset 1601) (line 44) (column 52) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1627) (line 45) (column 24) (len 47))) (all none) (ref r7) (shape (namespace (wildcard-suffix (span (span (offset 1671) (line 45) (column 68) (len 3))) (separator (span (offset 1671) (line 45) (column 68) (len 2))) (marker (span (offset 1673) (line 45) (column 70) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1699) (line 46) (column 24) (len 47))) (all none) (ref r8) (shape (namespace (wildcard-suffix (span (span (offset 1743) (line 46) (column 68) (len 3))) (separator (span (offset 1743) (line 46) (column 68) (len 2))) (marker (span (offset 1745) (line 46) (column 70) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (import (target (span (span (offset 1771) (line 47) (column 24) (len 47))) (all none) (ref r9) (shape (namespace (wildcard-suffix (span (span (offset 1815) (line 47) (column 68) (len 3))) (separator (span (offset 1815) (line 47) (column 68) (len 2))) (marker (span (offset 1817) (line 47) (column 70) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (part-usage))))))
)
~~~
