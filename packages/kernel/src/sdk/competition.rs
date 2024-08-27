//! Competition Control

pub extern "C" fn vexCompetitionStatus() -> u32 {
    Default::default()
}
pub extern "C" fn vexCompetitionControl(data: u32) {}
