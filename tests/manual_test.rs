use rustyscope::NanoscopeFile;

const TEST_FILE_PATH: &str = r"C:\Users\omerk\Documents\GitHub\mINE\rust\2026\nanoscope\QCJCD_W3_SiteSite1_Die_X0_Die_Y-4_23_A_22_20260708_010638.001";

#[test]
#[ignore = "manual only"]
fn test_load() -> std::io::Result<()> {
    let ns_file = NanoscopeFile::load(TEST_FILE_PATH);
    if let Ok(ns) = ns_file {
        let lines = ns.get_scan_lines().unwrap();
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
