# META
~~~sexpr
(snapshot (type semantic) (description "Standard Library: Systems Library/StandardViewDefinitions"))
~~~
# SOURCE
~~~sysml
standard library package StandardViewDefinitions {
    doc /*
         * This package defines the standard view definitions for the SysML language.
         */
    public import SysML::*;

    view def <gv> GeneralView {
        doc /*
             * View definition to present any members of exposed model element(s).
             * This is the most general view, enabling presentation of any model element.
             * The typical rendering in graphical notation is as a graph of nodes and edges.
             * Specializations of GeneralView can be specified through appropriate selection of filters, e.g.:
             * - package view, filtering on Package, Package containment, package Import
             * - definition and usage view, filtering on Definition, Usage, Specialization, FeatureTyping (covering defined by)
             * - requirement view, filtering on RequirementDefinition, RequirementUsage, Specialization, FeatureTyping, 
             *   SatisfyRequirementUsage, AllocationDefinition, AllocationUsage,
             * - view and viewpoint view, filtering on ViewDefinition, ViewUsage, ViewpointDefinition, ViewpointUsage, 
             *   RenderingDefinition, RenderingUsage, ConcernDefinition, ConcernUsage, StakeholderMembership, ...
             * - language extension view, filtering on Metaclass, MetadataFeature, MetadataAccessExpression, ...
             * Note: filters are specified by referencing concepts from the KerML.kerml and SysML.sysml standard library packages.
             */
    }

    view def <iv> InterconnectionView {
        doc /*
             * View definition to present exposed features as nodes, nested features as
             * nested nodes, and connections between features
             * as edges between (nested) nodes. Nested nodes may present boundary features
             * (e.g., ports, parameters).
             */
    }

    view def <afv> ActionFlowView specializes InterconnectionView {
        doc /*
             * View definition to present connections between actions.
             * Valid nodes and edges in an ActionFlowView are:
             * - Actions with nested actions
             * - Parameters with direction
             * - Flow connection usages (e.g., kinds of transfers from output to input)
             * - Binding connections between parameters (e.g., delegate a parameter from
             *   one level of nesting to another)
             * - Proxy connection points
             * - Swim lanes
             * - Conditional succession
             * - Control nodes (fork, join, decision, merge)
             * - Control structures, e.g., if-then-else, until-while-loop, for-loop
             * - Send and accept actions
             * - Change and time triggers
             * - Compartments on actions and parameters
             */
    }

    view def <stv> StateTransitionView specializes InterconnectionView {
        doc /*
             * View definition to present states and their transitions.
             * Valid nodes and edges in a StateTransitionView are:
             * - States with nested states
             * - Entry, do, and exit actions
             * - Transition usages with triggers, guards, and actions
             * - Compartments on states
             */
    }

    view def <sv> SequenceView {
        doc /*
             * View definition to present time ordering of event occurrences on lifelines
             * of exposed features.
             * Valid nodes and edges in a SequenceView are:
             * - Features such as parts with their lifelines
             * - Event occurrences on the lifelines
             * - Messages sent from one part to another with and without a type of flow
             * - Succession between event occurrences
             * - Nested sequence view (e.g., a reference to a view)
             * - Compartments
             * The typical rendering in graphical notation depicts the exposed features
             * horizontally along the top, with vertical lifelines. The time axis is
             * vertical, with time increasing from top to bottom.
             */
    }

    view def <gev> GeometryView {
        doc /*
             * View definition to present a visualization of exposed spatial items in two
             * or three dimensions
             * Valid nodes and edges in a GeometryView are:
             * - Spatial item, including shape
             * - Coordinate frame
             * - Feature related to spatial item, such as a quantity (e.g. temperature)
             *   of which values are to be rendered on a color scale
             * The typical rendering in graphical notation would include a number of
             * visualization parameters, such as:
             * - 2D or 3D view
             * - viewing direction
             * - zoom level
             * - light sources
             * - object projection mode, e.g., isometric, perspective, orthographic
             * - object rendering mode, e.g., shaded, wireframe, hidden line
             * - object pan (placement) and rotate (orientation) settings
             * - color maps
             */
    }

    view def <grv> GridView {
        doc /*
             * View definition to present exposed model elements and their relationships,
             * arranged in a rectangular grid.
             * GridView is the generalization of the following more specialized views:
             * - Tabular view
             * - Data value tabular view
             * - Relationship matrix view, e.g. presenting allocation or dependency relationships
             */
    }

    view def <bv> BrowserView {
        doc /*
             * View definition to present the hierarchical membership structure of model
             * elements starting from one or more exposed root elements.
             * The typical rendering in graphical notation is as an indented list of rows,
             * consisting of dynamically collapsible-expandable nodes that represent
             * branches and leaves of the tree, as in graphical user interface widgets.
             */
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "standard_view_definitions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package StandardViewDefinitions {
    doc
    /*
         * This package defines the standard view definitions for the SysML language.
         */
    public import SysML::*;
    view def <gv> GeneralView {
        doc
        /*
             * View definition to present any members of exposed model element(s).
             * This is the most general view, enabling presentation of any model element.
             * The typical rendering in graphical notation is as a graph of nodes and edges.
             * Specializations of GeneralView can be specified through appropriate selection of filters, e.g.:
             * - package view, filtering on Package, Package containment, package Import
             * - definition and usage view, filtering on Definition, Usage, Specialization, FeatureTyping (covering defined by)
             * - requirement view, filtering on RequirementDefinition, RequirementUsage, Specialization, FeatureTyping, 
             *   SatisfyRequirementUsage, AllocationDefinition, AllocationUsage,
             * - view and viewpoint view, filtering on ViewDefinition, ViewUsage, ViewpointDefinition, ViewpointUsage, 
             *   RenderingDefinition, RenderingUsage, ConcernDefinition, ConcernUsage, StakeholderMembership, ...
             * - language extension view, filtering on Metaclass, MetadataFeature, MetadataAccessExpression, ...
             * Note: filters are specified by referencing concepts from the KerML.kerml and SysML.sysml standard library packages.
             */
    }
    view def <iv> InterconnectionView {
        doc
        /*
             * View definition to present exposed features as nodes, nested features as
             * nested nodes, and connections between features
             * as edges between (nested) nodes. Nested nodes may present boundary features
             * (e.g., ports, parameters).
             */
    }
    view def <afv> ActionFlowView specializes InterconnectionView {
        doc
        /*
             * View definition to present connections between actions.
             * Valid nodes and edges in an ActionFlowView are:
             * - Actions with nested actions
             * - Parameters with direction
             * - Flow connection usages (e.g., kinds of transfers from output to input)
             * - Binding connections between parameters (e.g., delegate a parameter from
             *   one level of nesting to another)
             * - Proxy connection points
             * - Swim lanes
             * - Conditional succession
             * - Control nodes (fork, join, decision, merge)
             * - Control structures, e.g., if-then-else, until-while-loop, for-loop
             * - Send and accept actions
             * - Change and time triggers
             * - Compartments on actions and parameters
             */
    }
    view def <stv> StateTransitionView specializes InterconnectionView {
        doc
        /*
             * View definition to present states and their transitions.
             * Valid nodes and edges in a StateTransitionView are:
             * - States with nested states
             * - Entry, do, and exit actions
             * - Transition usages with triggers, guards, and actions
             * - Compartments on states
             */
    }
    view def <sv> SequenceView {
        doc
        /*
             * View definition to present time ordering of event occurrences on lifelines
             * of exposed features.
             * Valid nodes and edges in a SequenceView are:
             * - Features such as parts with their lifelines
             * - Event occurrences on the lifelines
             * - Messages sent from one part to another with and without a type of flow
             * - Succession between event occurrences
             * - Nested sequence view (e.g., a reference to a view)
             * - Compartments
             * The typical rendering in graphical notation depicts the exposed features
             * horizontally along the top, with vertical lifelines. The time axis is
             * vertical, with time increasing from top to bottom.
             */
    }
    view def <gev> GeometryView {
        doc
        /*
             * View definition to present a visualization of exposed spatial items in two
             * or three dimensions
             * Valid nodes and edges in a GeometryView are:
             * - Spatial item, including shape
             * - Coordinate frame
             * - Feature related to spatial item, such as a quantity (e.g. temperature)
             *   of which values are to be rendered on a color scale
             * The typical rendering in graphical notation would include a number of
             * visualization parameters, such as:
             * - 2D or 3D view
             * - viewing direction
             * - zoom level
             * - light sources
             * - object projection mode, e.g., isometric, perspective, orthographic
             * - object rendering mode, e.g., shaded, wireframe, hidden line
             * - object pan (placement) and rotate (orientation) settings
             * - color maps
             */
    }
    view def <grv> GridView {
        doc
        /*
             * View definition to present exposed model elements and their relationships,
             * arranged in a rectangular grid.
             * GridView is the generalization of the following more specialized views:
             * - Tabular view
             * - Data value tabular view
             * - Relationship matrix view, e.g. presenting allocation or dependency relationships
             */
    }
    view def <bv> BrowserView {
        doc
        /*
             * View definition to present the hierarchical membership structure of model
             * elements starting from one or more exposed root elements.
             * The typical rendering in graphical notation is as an indented list of rows,
             * consisting of dynamically collapsible-expandable nodes that represent
             * branches and leaves of the tree, as in graphical user interface widgets.
             */
    }
}
~~~
# AST
~~~sexpr
(parsed-document
  (references
    (reference r0 (scope relative) (span (offset 178) (line 5) (column 19) (len 5)) (segments (segment 0 (token "SysML") (name "SysML") (separator none) (span (offset 178) (line 5) (column 19) (len 5)))))
    (reference r1 (scope relative) (span (offset 1940) (line 33) (column 47) (len 19)) (segments (segment 0 (token "InterconnectionView") (name "InterconnectionView") (separator none) (span (offset 1940) (line 33) (column 47) (len 19)))))
    (reference r2 (scope relative) (span (offset 2893) (line 53) (column 52) (len 19)) (segments (segment 0 (token "InterconnectionView") (name "InterconnectionView") (separator none) (span (offset 2893) (line 53) (column 52) (len 19)))))
  )
  (root (library-package (name "StandardViewDefinitions") (standard true) (body brace (doc (name none) (locale none) (body (span (offset 61) (line 2) (column 11) (len 96)) (normalized "This package defines the standard view definitions for the SysML language.\n"))) (import (target (span (span (offset 178) (line 5) (column 19) (len 8))) (all none) (ref r0) (shape (namespace (wildcard-suffix (span (span (offset 183) (line 5) (column 24) (len 3))) (separator (span (offset 183) (line 5) (column 24) (len 2))) (marker (span (offset 185) (line 5) (column 26) (len 1)))) (recursive-suffix none) (combined-recursive-suffix-span none))))) (view-def (name "GeneralView") (short-name "gv") (modifiers) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 235) (line 8) (column 15) (len 1288)) (normalized "View definition to present any members of exposed model element(s).\nThis is the most general view, enabling presentation of any model element.\nThe typical rendering in graphical notation is as a graph of nodes and edges.\nSpecializations of GeneralView can be specified through appropriate selection of filters, e.g.:\n- package view, filtering on Package, Package containment, package Import\n- definition and usage view, filtering on Definition, Usage, Specialization, FeatureTyping (covering defined by)\n- requirement view, filtering on RequirementDefinition, RequirementUsage, Specialization, FeatureTyping, \n  SatisfyRequirementUsage, AllocationDefinition, AllocationUsage,\n- view and viewpoint view, filtering on ViewDefinition, ViewUsage, ViewpointDefinition, ViewpointUsage, \n  RenderingDefinition, RenderingUsage, ConcernDefinition, ConcernUsage, StakeholderMembership, ...\n- language extension view, filtering on Metaclass, MetadataFeature, MetadataAccessExpression, ...\nNote: filters are specified by referencing concepts from the KerML.kerml and SysML.sysml standard library packages.\n"))))) (view-def (name "InterconnectionView") (short-name "iv") (modifiers) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 1587) (line 25) (column 15) (len 297)) (normalized "View definition to present exposed features as nodes, nested features as\nnested nodes, and connections between features\nas edges between (nested) nodes. Nested nodes may present boundary features\n(e.g., ports, parameters).\n"))))) (view-def (name "ActionFlowView") (short-name "afv") (modifiers) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r1)))) (body brace (doc (name none) (locale none) (body (span (offset 1976) (line 34) (column 15) (len 856)) (normalized "View definition to present connections between actions.\nValid nodes and edges in an ActionFlowView are:\n- Actions with nested actions\n- Parameters with direction\n- Flow connection usages (e.g., kinds of transfers from output to input)\n- Binding connections between parameters (e.g., delegate a parameter from\n  one level of nesting to another)\n- Proxy connection points\n- Swim lanes\n- Conditional succession\n- Control nodes (fork, join, decision, merge)\n- Control structures, e.g., if-then-else, until-while-loop, for-loop\n- Send and accept actions\n- Change and time triggers\n- Compartments on actions and parameters\n"))))) (view-def (name "StateTransitionView") (short-name "stv") (modifiers) (specializes (typing (kind subclassification) (conjugated false) (implied false) (targets (ref r2)))) (body brace (doc (name none) (locale none) (body (span (offset 2929) (line 54) (column 15) (len 351)) (normalized "View definition to present states and their transitions.\nValid nodes and edges in a StateTransitionView are:\n- States with nested states\n- Entry, do, and exit actions\n- Transition usages with triggers, guards, and actions\n- Compartments on states\n"))))) (view-def (name "SequenceView") (short-name "sv") (modifiers) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 3337) (line 65) (column 15) (len 792)) (normalized "View definition to present time ordering of event occurrences on lifelines\nof exposed features.\nValid nodes and edges in a SequenceView are:\n- Features such as parts with their lifelines\n- Event occurrences on the lifelines\n- Messages sent from one part to another with and without a type of flow\n- Succession between event occurrences\n- Nested sequence view (e.g., a reference to a view)\n- Compartments\nThe typical rendering in graphical notation depicts the exposed features\nhorizontally along the top, with vertical lifelines. The time axis is\nvertical, with time increasing from top to bottom.\n"))))) (view-def (name "GeometryView") (short-name "gev") (modifiers) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 4187) (line 82) (column 15) (len 960)) (normalized "View definition to present a visualization of exposed spatial items in two\nor three dimensions\nValid nodes and edges in a GeometryView are:\n- Spatial item, including shape\n- Coordinate frame\n- Feature related to spatial item, such as a quantity (e.g. temperature)\n  of which values are to be rendered on a color scale\nThe typical rendering in graphical notation would include a number of\nvisualization parameters, such as:\n- 2D or 3D view\n- viewing direction\n- zoom level\n- light sources\n- object projection mode, e.g., isometric, perspective, orthographic\n- object rendering mode, e.g., shaded, wireframe, hidden line\n- object pan (placement) and rotate (orientation) settings\n- color maps\n"))))) (view-def (name "GridView") (short-name "grv") (modifiers) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 5201) (line 104) (column 15) (len 407)) (normalized "View definition to present exposed model elements and their relationships,\narranged in a rectangular grid.\nGridView is the generalization of the following more specialized views:\n- Tabular view\n- Data value tabular view\n- Relationship matrix view, e.g. presenting allocation or dependency relationships\n"))))) (view-def (name "BrowserView") (short-name "bv") (modifiers) (specializes none) (body brace (doc (name none) (locale none) (body (span (offset 5664) (line 115) (column 15) (len 440)) (normalized "View definition to present the hierarchical membership structure of model\nelements starting from one or more exposed root elements.\nThe typical rendering in graphical notation is as an indented list of rows,\nconsisting of dynamically collapsible-expandable nodes that represent\nbranches and leaves of the tree, as in graphical user interface widgets.\n"))))))))
)
~~~
