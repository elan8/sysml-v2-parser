# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Systems Library/Connections"))
~~~
# SOURCE
~~~sysml
standard library package Connections {
    doc
    /*
     * This package defines the base types for connections and related structural elements 
     * in the SysML language.
     */

    private import Base::Anything;
    private import Occurrences::Occurrence;
    private import Occurrences::HappensDuring;
    private import Objects::LinkObject;
    private import Objects::linkObjects;
    private import Objects::BinaryLinkObject;
    private import Objects::binaryLinkObjects;
    private import Transfers::Transfer;
    private import Transfers::transfers;
    private import Transfers::FlowTransfer;
    private import Transfers::flowTransfers;
    private import Transfers::FlowTransferBefore;
    private import Transfers::flowTransfersBefore;
    private import ScalarValues::Natural;
    private import Parts::Part;
    private import Parts::parts;
    private import Actions::Action;
    private import Actions::actions;

    abstract connection def Connection :> LinkObject, Part {
        doc
        /*
         * Connection is the most general class of links between things within some 
         * containing structure. Connection is the base type of all ConnectionDefinitions.
         */
    }
     
    abstract connection def BinaryConnection :> BinaryLinkObject, Connection {
        doc
        /*
         * BinaryConnection is the most general class of binary links between two things 
         * within some containing structure. BinaryConnection is the base type of all 
         * ConnectionDefinitions with exactly two ends.
         */
    
        end source: Anything :>> BinaryLinkObject::source;
        end target: Anything :>> BinaryLinkObject::target;
    }
    
    abstract connection connections: Connection[0..*] nonunique :> linkObjects, parts {
        doc
        /*
         * connections is the base feature of all ConnectionUsages.
         */
    }
    
    abstract connection binaryConnections: Connection[0..*] nonunique :> connections, binaryLinkObjects {
        doc
        /*
         * binaryConnections is the base feature of all binary ConnectionUsages.
         */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "connections.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package Connections {
    doc
    /*
     * This package defines the base types for connections and related structural elements 
     * in the SysML language.
     */
    private import Base::Anything;
    private import Occurrences::Occurrence;
    private import Occurrences::HappensDuring;
    private import Objects::LinkObject;
    private import Objects::linkObjects;
    private import Objects::BinaryLinkObject;
    private import Objects::binaryLinkObjects;
    private import Transfers::Transfer;
    private import Transfers::transfers;
    private import Transfers::FlowTransfer;
    private import Transfers::flowTransfers;
    private import Transfers::FlowTransferBefore;
    private import Transfers::flowTransfersBefore;
    private import ScalarValues::Natural;
    private import Parts::Part;
    private import Parts::parts;
    private import Actions::Action;
    private import Actions::actions;
    abstract connection def Connection :> LinkObject, Part {
        doc
        /*
         * Connection is the most general class of links between things within some 
         * containing structure. Connection is the base type of all ConnectionDefinitions.
         */
    }
    abstract connection def BinaryConnection :> BinaryLinkObject, Connection {
        doc
        /*
         * BinaryConnection is the most general class of binary links between two things 
         * within some containing structure. BinaryConnection is the base type of all 
         * ConnectionDefinitions with exactly two ends.
         */
        end source : Anything :>> BinaryLinkObject::source;
        end target : Anything :>> BinaryLinkObject::target;
    }
    abstract connection def connections :> linkObjects, parts {
        doc
        /*
         * connections is the base feature of all ConnectionUsages.
         */
    }
    abstract connection def binaryConnections :> connections, binaryLinkObjects {
        doc
        /*
         * binaryConnections is the base feature of all binary ConnectionUsages.
         */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 204) (line 8) (column 20) (len 14)) (segments (segment 0 (token "Base") (name "Base") (separator none) (span (offset 204) (line 8) (column 20) (len 4))) (segment 1 (token "Anything") (name "Anything") (separator colon-colon) (span (offset 210) (line 8) (column 26) (len 8)))))
    (reference r1 (scope relative) (span (offset 239) (line 9) (column 20) (len 23)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 239) (line 9) (column 20) (len 11))) (segment 1 (token "Occurrence") (name "Occurrence") (separator colon-colon) (span (offset 252) (line 9) (column 33) (len 10)))))
    (reference r2 (scope relative) (span (offset 283) (line 10) (column 20) (len 26)) (segments (segment 0 (token "Occurrences") (name "Occurrences") (separator none) (span (offset 283) (line 10) (column 20) (len 11))) (segment 1 (token "HappensDuring") (name "HappensDuring") (separator colon-colon) (span (offset 296) (line 10) (column 33) (len 13)))))
    (reference r3 (scope relative) (span (offset 330) (line 11) (column 20) (len 19)) (segments (segment 0 (token "Objects") (name "Objects") (separator none) (span (offset 330) (line 11) (column 20) (len 7))) (segment 1 (token "LinkObject") (name "LinkObject") (separator colon-colon) (span (offset 339) (line 11) (column 29) (len 10)))))
    (reference r4 (scope relative) (span (offset 370) (line 12) (column 20) (len 20)) (segments (segment 0 (token "Objects") (name "Objects") (separator none) (span (offset 370) (line 12) (column 20) (len 7))) (segment 1 (token "linkObjects") (name "linkObjects") (separator colon-colon) (span (offset 379) (line 12) (column 29) (len 11)))))
    (reference r5 (scope relative) (span (offset 411) (line 13) (column 20) (len 25)) (segments (segment 0 (token "Objects") (name "Objects") (separator none) (span (offset 411) (line 13) (column 20) (len 7))) (segment 1 (token "BinaryLinkObject") (name "BinaryLinkObject") (separator colon-colon) (span (offset 420) (line 13) (column 29) (len 16)))))
    (reference r6 (scope relative) (span (offset 457) (line 14) (column 20) (len 26)) (segments (segment 0 (token "Objects") (name "Objects") (separator none) (span (offset 457) (line 14) (column 20) (len 7))) (segment 1 (token "binaryLinkObjects") (name "binaryLinkObjects") (separator colon-colon) (span (offset 466) (line 14) (column 29) (len 17)))))
    (reference r7 (scope relative) (span (offset 504) (line 15) (column 20) (len 19)) (segments (segment 0 (token "Transfers") (name "Transfers") (separator none) (span (offset 504) (line 15) (column 20) (len 9))) (segment 1 (token "Transfer") (name "Transfer") (separator colon-colon) (span (offset 515) (line 15) (column 31) (len 8)))))
    (reference r8 (scope relative) (span (offset 544) (line 16) (column 20) (len 20)) (segments (segment 0 (token "Transfers") (name "Transfers") (separator none) (span (offset 544) (line 16) (column 20) (len 9))) (segment 1 (token "transfers") (name "transfers") (separator colon-colon) (span (offset 555) (line 16) (column 31) (len 9)))))
    (reference r9 (scope relative) (span (offset 585) (line 17) (column 20) (len 23)) (segments (segment 0 (token "Transfers") (name "Transfers") (separator none) (span (offset 585) (line 17) (column 20) (len 9))) (segment 1 (token "FlowTransfer") (name "FlowTransfer") (separator colon-colon) (span (offset 596) (line 17) (column 31) (len 12)))))
    (reference r10 (scope relative) (span (offset 629) (line 18) (column 20) (len 24)) (segments (segment 0 (token "Transfers") (name "Transfers") (separator none) (span (offset 629) (line 18) (column 20) (len 9))) (segment 1 (token "flowTransfers") (name "flowTransfers") (separator colon-colon) (span (offset 640) (line 18) (column 31) (len 13)))))
    (reference r11 (scope relative) (span (offset 674) (line 19) (column 20) (len 29)) (segments (segment 0 (token "Transfers") (name "Transfers") (separator none) (span (offset 674) (line 19) (column 20) (len 9))) (segment 1 (token "FlowTransferBefore") (name "FlowTransferBefore") (separator colon-colon) (span (offset 685) (line 19) (column 31) (len 18)))))
    (reference r12 (scope relative) (span (offset 724) (line 20) (column 20) (len 30)) (segments (segment 0 (token "Transfers") (name "Transfers") (separator none) (span (offset 724) (line 20) (column 20) (len 9))) (segment 1 (token "flowTransfersBefore") (name "flowTransfersBefore") (separator colon-colon) (span (offset 735) (line 20) (column 31) (len 19)))))
    (reference r13 (scope relative) (span (offset 775) (line 21) (column 20) (len 21)) (segments (segment 0 (token "ScalarValues") (name "ScalarValues") (separator none) (span (offset 775) (line 21) (column 20) (len 12))) (segment 1 (token "Natural") (name "Natural") (separator colon-colon) (span (offset 789) (line 21) (column 34) (len 7)))))
    (reference r14 (scope relative) (span (offset 817) (line 22) (column 20) (len 11)) (segments (segment 0 (token "Parts") (name "Parts") (separator none) (span (offset 817) (line 22) (column 20) (len 5))) (segment 1 (token "Part") (name "Part") (separator colon-colon) (span (offset 824) (line 22) (column 27) (len 4)))))
    (reference r15 (scope relative) (span (offset 849) (line 23) (column 20) (len 12)) (segments (segment 0 (token "Parts") (name "Parts") (separator none) (span (offset 849) (line 23) (column 20) (len 5))) (segment 1 (token "parts") (name "parts") (separator colon-colon) (span (offset 856) (line 23) (column 27) (len 5)))))
    (reference r16 (scope relative) (span (offset 882) (line 24) (column 20) (len 15)) (segments (segment 0 (token "Actions") (name "Actions") (separator none) (span (offset 882) (line 24) (column 20) (len 7))) (segment 1 (token "Action") (name "Action") (separator colon-colon) (span (offset 891) (line 24) (column 29) (len 6)))))
    (reference r17 (scope relative) (span (offset 918) (line 25) (column 20) (len 16)) (segments (segment 0 (token "Actions") (name "Actions") (separator none) (span (offset 918) (line 25) (column 20) (len 7))) (segment 1 (token "actions") (name "actions") (separator colon-colon) (span (offset 927) (line 25) (column 29) (len 7)))))
    (reference r18 (scope relative) (span (offset 979) (line 27) (column 43) (len 10)) (segments (segment 0 (token "LinkObject") (name "LinkObject") (separator none) (span (offset 979) (line 27) (column 43) (len 10)))))
    (reference r19 (scope relative) (span (offset 991) (line 27) (column 55) (len 4)) (segments (segment 0 (token "Part") (name "Part") (separator none) (span (offset 991) (line 27) (column 55) (len 4)))))
    (reference r20 (scope relative) (span (offset 1269) (line 35) (column 49) (len 16)) (segments (segment 0 (token "BinaryLinkObject") (name "BinaryLinkObject") (separator none) (span (offset 1269) (line 35) (column 49) (len 16)))))
    (reference r21 (scope relative) (span (offset 1287) (line 35) (column 67) (len 10)) (segments (segment 0 (token "Connection") (name "Connection") (separator none) (span (offset 1287) (line 35) (column 67) (len 10)))))
    (reference r22 (scope relative) (span (offset 1593) (line 43) (column 21) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 1593) (line 43) (column 21) (len 8)))))
    (reference r23 (scope relative) (span (offset 1606) (line 43) (column 34) (len 24)) (segments (segment 0 (token "BinaryLinkObject") (name "BinaryLinkObject") (separator none) (span (offset 1606) (line 43) (column 34) (len 16))) (segment 1 (token "source") (name "source") (separator colon-colon) (span (offset 1624) (line 43) (column 52) (len 6)))))
    (reference r24 (scope relative) (span (offset 1652) (line 44) (column 21) (len 8)) (segments (segment 0 (token "Anything") (name "Anything") (separator none) (span (offset 1652) (line 44) (column 21) (len 8)))))
    (reference r25 (scope relative) (span (offset 1665) (line 44) (column 34) (len 24)) (segments (segment 0 (token "BinaryLinkObject") (name "BinaryLinkObject") (separator none) (span (offset 1665) (line 44) (column 34) (len 16))) (segment 1 (token "target") (name "target") (separator colon-colon) (span (offset 1683) (line 44) (column 52) (len 6)))))
    (reference r26 (scope relative) (span (offset 1769) (line 47) (column 68) (len 11)) (segments (segment 0 (token "linkObjects") (name "linkObjects") (separator none) (span (offset 1769) (line 47) (column 68) (len 11)))))
    (reference r27 (scope relative) (span (offset 1782) (line 47) (column 81) (len 5)) (segments (segment 0 (token "parts") (name "parts") (separator none) (span (offset 1782) (line 47) (column 81) (len 5)))))
    (reference r28 (scope relative) (span (offset 1977) (line 54) (column 74) (len 11)) (segments (segment 0 (token "connections") (name "connections") (separator none) (span (offset 1977) (line 54) (column 74) (len 11)))))
    (reference r29 (scope relative) (span (offset 1990) (line 54) (column 87) (len 17)) (segments (segment 0 (token "binaryLinkObjects") (name "binaryLinkObjects") (separator none) (span (offset 1990) (line 54) (column 87) (len 17)))))
  )
  (root (library-package (name "Connections") (standard true) (body brace (doc) (import (target (span (span (offset 204) (line 8) (column 20) (len 14))) (all none) (ref r0) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 239) (line 9) (column 20) (len 23))) (all none) (ref r1) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 283) (line 10) (column 20) (len 26))) (all none) (ref r2) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 330) (line 11) (column 20) (len 19))) (all none) (ref r3) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 370) (line 12) (column 20) (len 20))) (all none) (ref r4) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 411) (line 13) (column 20) (len 25))) (all none) (ref r5) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 457) (line 14) (column 20) (len 26))) (all none) (ref r6) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 504) (line 15) (column 20) (len 19))) (all none) (ref r7) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 544) (line 16) (column 20) (len 20))) (all none) (ref r8) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 585) (line 17) (column 20) (len 23))) (all none) (ref r9) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 629) (line 18) (column 20) (len 24))) (all none) (ref r10) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 674) (line 19) (column 20) (len 29))) (all none) (ref r11) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 724) (line 20) (column 20) (len 30))) (all none) (ref r12) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 775) (line 21) (column 20) (len 21))) (all none) (ref r13) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 817) (line 22) (column 20) (len 11))) (all none) (ref r14) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 849) (line 23) (column 20) (len 12))) (all none) (ref r15) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 882) (line 24) (column 20) (len 15))) (all none) (ref r16) (shape (membership (recursive-suffix none))))) (import (target (span (span (offset 918) (line 25) (column 20) (len 16))) (all none) (ref r17) (shape (membership (recursive-suffix none))))) (connection-def (name "Connection") (modifiers abstract) (role ordinary) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r18) (ref r19)))) (body brace (doc))) (connection-def (name "BinaryConnection") (modifiers abstract) (role ordinary) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r20) (ref r21)))) (body brace (doc) (end (short-name none) (identity (declaration (name "source") (span (offset 1585) (line 43) (column 13) (len 6)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r22)))) (references none) (redefines (relationship (kind redefines) (implied false) (targets (ref r23)))) (crosses none)) (end (short-name none) (identity (declaration (name "target") (span (offset 1644) (line 44) (column 13) (len 6)))) (typing (typing (kind typing) (conjugated false) (implied false) (targets (ref r24)))) (references none) (redefines (relationship (kind redefines) (implied false) (targets (ref r25)))) (crosses none)))) (connection-def (name "connections") (modifiers abstract) (role ordinary) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r26) (ref r27)))) (body brace (doc))) (connection-def (name "binaryConnections") (modifiers abstract) (role ordinary) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r28) (ref r29)))) (body brace (doc))))))
)
~~~
