use rustyscope::NanoscopeFile;

const TEST_FILE_PATH: &str = "good_test_file.001";

fn load_test_file() -> Result<NanoscopeFile, String> {
    NanoscopeFile::load(TEST_FILE_PATH).map_err(|e| format!("{e}"))
}

#[test]
fn test_num_channels() -> Result<(), String> {
    let ns_file = load_test_file()?;
    let num_channels = ns_file.channels.len();
    assert_eq!(
        num_channels, 2,
        "number of channels ({num_channels}) did not equal 2"
    );
    Ok(())
}

#[test]
fn test_channel_names() -> Result<(), String> {
    let ns_file = load_test_file()?;
    let correct_channel_names: Vec<&str> = vec!["Height", "Y scan"];
    let channel_names: Vec<&str> = ns_file.channels.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(
        channel_names, correct_channel_names,
        "channel names {:?} were not equal to {:?}",
        channel_names, correct_channel_names,
    );
    Ok(())
}

#[test]
fn has_all_correct_metadata() -> Result<(), String> {
    let ns_file = load_test_file()?;
    assert!(
        ns_file.equipment_metadata.is_some(),
        "missing equipment_metadata"
    );
    assert!(ns_file.hdsc_metadata.is_some(), "missing hdsc_metadata");
    assert!(ns_file.misc_metadata.is_some(), "missing misc_metadata");
    assert!(ns_file.engage_metadata.is_some(), "missing engage_metadata");
    assert!(ns_file.sweep_metadata.is_some(), "missing sweep_metadata");
    Ok(())
}
