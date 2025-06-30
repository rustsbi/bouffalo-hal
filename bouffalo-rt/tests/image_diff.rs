mod tests {
    use bouffalo_rt::soc::bl808::HalBootheader;
    use treediff::{diff, tools::ChangeType};

    #[test]
    fn test_image_fusion_from_bytes() {
        let m0_bytes = include_bytes!("./multicore-demo-mcu.bin");
        let d0_bytes = include_bytes!("./multicore-demo-dsp.bin");
        let fused_bytes = include_bytes!("./whole_img.bin");

        let m0 = HalBootheader::from_bytes(m0_bytes).expect("Failed to parse M0 boot header");
        let d0 = HalBootheader::from_bytes(d0_bytes).expect("Failed to parse D0 boot header");
        let fused =
            HalBootheader::from_bytes(fused_bytes).expect("Failed to parse fused boot header");

        let m0_structured_flag = m0.structured_flag();
        let d0_structured_flag = d0.structured_flag();
        let fused_structured_flag = fused.structured_flag();

        let m0_flag_json =
            serde_json::to_value(&m0_structured_flag).expect("Failed to serialize M0 flag");
        let d0_flag_json =
            serde_json::to_value(&d0_structured_flag).expect("Failed to serialize D0 flag");
        let fused_flag_json =
            serde_json::to_value(&fused_structured_flag).expect("Failed to serialize Fused flag");

        let mut m0_flag_diff = treediff::tools::Recorder::default();
        diff(&m0_flag_json, &fused_flag_json, &mut m0_flag_diff);
        let changed_calls = m0_flag_diff
            .calls
            .iter()
            .filter(|c| !matches!(c, ChangeType::Unchanged(_, _)));
        println!("M0 Flag Diff: {:#?}", changed_calls);
        let mut d0_flag_diff = treediff::tools::Recorder::default();
        diff(&d0_flag_json, &fused_flag_json, &mut d0_flag_diff);
        let changed_calls = d0_flag_diff
            .calls
            .iter()
            .filter(|c| !matches!(c, ChangeType::Unchanged(_, _)));
        println!("D0 Flag Diff: {:#?}", changed_calls);

        // --- Start: Dump headers to JSON files ---

        // Create a directory for the output in the target folder

        // Dump m0 header to JSON
        let m0_json = serde_json::to_value(&m0).expect("Failed to serialize M0 header");
        // Dump d0 header to JSON
        let d0_json = serde_json::to_value(&d0).expect("Failed to serialize D0 header");
        // Dump fused header to JSON
        let fused_json = serde_json::to_value(&fused).expect("Failed to serialize Fused header");
        // --- End: Dump headers to JSON files ---
        let mut m0_diff = treediff::tools::Recorder::default();

        diff(&m0_json, &fused_json, &mut m0_diff);
        let changed_calls = m0_diff
            .calls
            .iter()
            .filter(|c| !matches!(c, ChangeType::Unchanged(_, _)));
        // println!("M0 Diff: {:#?}", changed_calls);

        let mut d0_diff = treediff::tools::Recorder::default();
        diff(&d0_json, &fused_json, &mut d0_diff);
        let changed_calls = d0_diff
            .calls
            .iter()
            .filter(|c| !matches!(c, ChangeType::Unchanged(_, _)));
        // println!("D0 Diff: {:#?}", changed_calls);
    }
}
