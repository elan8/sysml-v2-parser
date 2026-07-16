fn brace_errors(input: &str) -> Vec<(Option<String>, String)> {
    let result = sysml_v2_parser::parse_for_editor(input);
    result
        .errors
        .iter()
        .filter(|d| {
            d.code.as_deref().is_some_and(|c| c.contains("brace")) || d.message.contains("brace")
        })
        .map(|d| (d.code.clone(), d.message.clone()))
        .collect()
}

#[test]
fn vacuuming_types_debris_port_legacy_shape() {
    let input = r#"package P {
  port def DebrisPort {
    inout item debris{
      attribute vol:> SI::'m³';
      attribute mass :> SI::kg;
    }
  }
}"#;
    assert!(brace_errors(input).is_empty(), "{:?}", brace_errors(input));
}

#[test]
fn vacuuming_types_suction_and_rotation_ports() {
    let input = r#"package P {
  port def SuctionLevelPort :> BatterySignals::VacuumSystemPowerOutPort {
    out attribute redefines suctionPower :> ISQMechanics::power;
  }
  port def RotationLevelPort :> BatterySignals::VacuumSystemPowerOutPort {
    out attribute redefines brushPower :> ISQMechanics::power;
  }
}"#;
    assert!(brace_errors(input).is_empty(), "{:?}", brace_errors(input));
}

#[test]
fn vacuuming_types_all_ports_without_block_comment() {
    let input = r#"package P {
  package PortDefinitions {
    port def SuctionLevelPort :> BatterySignals::VacuumSystemPowerOutPort {
      out attribute redefines suctionPower :> ISQMechanics::power;
    }
    port def RotationLevelPort :> BatterySignals::VacuumSystemPowerOutPort {
      out attribute redefines brushPower :> ISQMechanics::power;
    }
    port def PowerInOutVac :> Signals::BatterySignals::PowerInOutPort;
    port def AirPort {
      out volume :> ISQSpaceTime::volume;
    }
    port def DebrisPort {
      inout item debris{
        attribute vol:> SI::'m³';
        attribute mass :> SI::kg;
      }
    }
    port def FillStatePort :> NumericSignal;
  }
}"#;
    assert!(brace_errors(input).is_empty(), "{:?}", brace_errors(input));
}

#[test]
fn vacuuming_types_block_comment_with_nested_braces() {
    let input = r#"package P {
  package PortDefinitions {
    port def AirPort {
      out volume :> ISQSpaceTime::volume;
    }
    /*	port def ExternalAirPort {
      in item externalAir{
        attribute volume :> ISQSpaceTime::volume;
      }
    }
    port def InternalAirPort {
      out item internalAir{
        attribute volume :> ISQSpaceTime::volume;
      }
    }*/
    port def DebrisPort {
      inout item debris{
        attribute vol:> SI::'m³';
      }
    }
  }
}"#;
    assert!(brace_errors(input).is_empty(), "{:?}", brace_errors(input));
}

#[test]
fn vacuuming_types_line_comments_with_braces_do_not_break_parse() {
    let input = r#"package P {
  package PortDefinitions {
    port def SuctionLevelPort :> Base {
      out attribute redefines suctionPower :> ISQ::power;
    }
    // {
    //	out item suctionLevel {
    port def AirPort {
      out volume :> ISQSpaceTime::volume;
    }
  }
}"#;
    assert!(brace_errors(input).is_empty(), "{:?}", brace_errors(input));
}

fn vacuuming_types_sysml_path() -> Option<std::path::PathBuf> {
    let root = std::env::var_os("MBSE_VACUUM_EXAMPLE_DIR")?;
    let path = std::path::PathBuf::from(root)
        .join("Functions/legacy/VacuumingSystem/VacuumingTypes.sysml");
    path.exists().then_some(path)
}

#[test]
#[ignore = "requires MBSE_VACUUM_EXAMPLE_DIR pointing at the public example checkout"]
fn vacuuming_types_port_definitions_package_without_block_comment() {
    let Some(path) = vacuuming_types_sysml_path() else {
        return;
    };
    let input = std::fs::read_to_string(&path).expect("read");
    let port_defs_start = input.find("package PortDefinitions").expect("port defs");
    let port_defs_end = input.find("// Interfaces between").expect("interfaces");
    let mut chunk = input[port_defs_start..port_defs_end].to_string();
    while let (Some(start), Some(end)) = (chunk.find("/*"), chunk.find("*/")) {
        chunk.replace_range(start..end + 2, "");
    }
    let wrapped = format!("package VacuumingTypes {{\n{chunk}\n}}");
    let errors = brace_errors(&wrapped);
    assert!(
        errors.is_empty(),
        "port definitions (no block comment) brace errors: {errors:?}"
    );
}

#[test]
#[ignore = "requires MBSE_VACUUM_EXAMPLE_DIR pointing at the public example checkout"]
fn vacuuming_types_port_definitions_package() {
    let Some(path) = vacuuming_types_sysml_path() else {
        return;
    };
    let input = std::fs::read_to_string(&path).expect("read");
    let port_defs_start = input.find("package PortDefinitions").expect("port defs");
    let port_defs_end = input.find("// Interfaces between").expect("interfaces");
    let chunk = format!(
        "package VacuumingTypes {{\n{}\n}}",
        &input[port_defs_start..port_defs_end]
    );
    let errors = brace_errors(&chunk);
    assert!(
        errors.is_empty(),
        "port definitions brace errors: {errors:?}"
    );
}
