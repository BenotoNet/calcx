use crate::Num;
use crate::calc::tokenize::misc_units::unit_to_num;

pub fn can_be_converted(base: Num, to_units: &str) -> bool {
    let to_units = unit_to_num(to_units).unwrap().get_units().clone();
    return base.div(&Num::from(1.0, to_units)).unwrap().is_unitless();
}
