// pub fn hh_mm_to_i64(hh_mm: &str) -> Result<i64, String> {
//     if hh_mm.is_empty() {Err("Clocks are empty!".into())}
//     let hh: i64 = hh_mm.get(0..=1).unwrap().trim().parse()?;
//     let mm: i64 = hh_mm.get(3..=4).unwrap().trim().parse()?;
//     Ok(chrono::Duration::seconds(60*(hh*60+mm)).num_seconds().into())
// }