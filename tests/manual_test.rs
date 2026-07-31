use rustyscope::NanoscopeFile;

const TEST_FILE_PATH: &str = "good_test_file.001";

#[test]
#[ignore = "manual only"]
fn test_load() -> std::io::Result<()> {
    let ns_file = NanoscopeFile::load(TEST_FILE_PATH);
    if let Ok(ns) = ns_file {
        let lines = ns.data;
        let (line_x, line_height) = &lines[0];
        let mut wtr = csv::Writer::from_path("out.csv")?;
        wtr.write_record(["X", "Height"])?;
        wtr.flush()?;
        for i in 0..line_x.len() {
            wtr.write_record([
                line_x[i].to_string().as_str(),
                line_height[i].to_string().as_str(),
            ])?;
            wtr.flush()?;
        }
    }
    Ok(())
    // println!("{:#?}", buffer);
}
