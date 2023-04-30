use phper::{
    alloc::ToRefOwned,
    classes::{ClassEntity, Visibility},
    functions::Argument,
};
use rolling_dual_crc::{DualCrc, Zeros};

pub const DUAL_CRC_CLASS_NAME: &str = "Crc\\DualCrc";

pub fn make_dual_crc_class() -> ClassEntity <DualCrc> {
    let mut class =
        ClassEntity::<DualCrc>::new_with_default_state_constructor(DUAL_CRC_CLASS_NAME);

    class
        .add_method("update", Visibility::Public, |this, arguments| {
            let update_str = arguments[0].expect_z_str()?.to_str().unwrap();
            let crc = this.as_mut_state();
            crc.update(update_str);
            Ok::<_, phper::Error>(this.to_ref_owned())
        })
        .argument(Argument::by_val("data"));

    class
        .add_method("updateWithZeros", Visibility::Public, |this, arguments| {
            let size = arguments[0].expect_long().unwrap();
            let crc = this.as_mut_state();
            crc.update_with_zeros(&Zeros::new(size as usize));
            Ok::<_, phper::Error>(this.to_ref_owned())
        })
        .argument(Argument::by_val("size"));

    class
        .add_method("get32", Visibility::Public, |this, _| {
            let crc = this.as_mut_state();
            let res = crc.get32() as i64;
            Ok::<_, phper::Error>(res)
        });

    class
        .add_method("get64", Visibility::Public, |this, _| {
            let crc = this.as_mut_state();
            let res = crc.get64() as i64;
            Ok::<_, phper::Error>(res)
        });

    class
}