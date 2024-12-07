// We need to find the nice and naughty kids for santa

// Each good deed is worth 1 point and each bad deed is worth 2 points
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;
pub const NICE_IF_GEQ: f32 = 0.75;

pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    if good_deeds == 0 && bad_deeds == 0 {
        return false;
    }
    let good_deeds = good_deeds as f32;
    let bad_deeds = bad_deeds as f32;
    let ratio = good_deeds / (GOOD_WEIGHT * good_deeds + (BAD_WEIGHT * bad_deeds));
    ratio >= NICE_IF_GEQ
}
