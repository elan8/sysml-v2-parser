use sysml_v2_parser::ast::{
    PackageBody, PackageBodyElement, RootElement, StateDefBody, StateDefBodyElement,
};
use sysml_v2_parser::parse_with_diagnostics;

#[test]
fn failed_state_action_production_does_not_publish_its_target() {
    let result = parse_with_diagnostics(concat!(
        "package P {\n",
        "    state def Machine {\n",
        "        entry Ghost::broken unexpected;\n",
        "        do Live::later;\n",
        "    }\n",
        "}\n",
    ));
    assert_eq!(
        result.document.qualified_references.len(),
        1,
        "failed entry production must roll back Ghost::broken"
    );
    let live_id = match &result.document.root.elements[0].value {
        RootElement::Package(package) => match &package.value.body {
            PackageBody::Brace { elements, .. } => match &elements[0].value {
                PackageBodyElement::StateDef(state) => match &state.value.body {
                    StateDefBody::Brace { elements, .. } => elements.iter().find_map(|element| {
                        if let StateDefBodyElement::Do(action) = &element.value {
                            action.value.action_reference
                        } else {
                            None
                        }
                    }),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
    .expect("valid later target must remain in the recovered tree");
    let reference = result
        .document
        .qualified_reference(live_id)
        .expect("valid later target must remain published");
    assert_eq!(reference.authored_text(), "Live::later");
}
