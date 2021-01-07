#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::unreadable_literal)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Initialize data lane, input mode, data type.
#[cfg(any(sdkver = "2.0.3.0", sdkver = "2.0.3.1"))]
pub const HI_MIPI_SET_DEV_ATTR: u32 = 1_114_402_049;

/// Initialize data lane, input mode, data type.
#[cfg(not(any(sdkver = "2.0.3.0", sdkver = "2.0.3.1")))]
pub const HI_MIPI_SET_DEV_ATTR: u32 = 1_113_615_617;

/// Set phy common mode voltage mode.
pub const HI_MIPI_SET_PHY_CMVMODE: u32 = 1_074_294_020;

/// Reset sensor.
pub const HI_MIPI_RESET_SENSOR: u32 = 1_074_031_877;

/// Unreset sensor.
pub const HI_MIPI_UNRESET_SENSOR: u32 = 1_074_031_878;

/// Reset mipi.
pub const HI_MIPI_RESET_MIPI: u32 = 1_074_031_879;

/// Unreset mipi.
pub const HI_MIPI_UNRESET_MIPI: u32 = 1_074_031_880;

/// Reset slvs.
pub const HI_MIPI_RESET_SLVS: u32 = 1_074_031_881;

/// Unreset slvs.
pub const HI_MIPI_UNRESET_SLVS: u32 = 1_074_031_882;

/// Set mipi hs_mode.
pub const HI_MIPI_SET_HS_MODE: u32 = 1_074_031_883;

/// Enable mipi clock.
pub const HI_MIPI_ENABLE_MIPI_CLOCK: u32 = 1_074_031_884;

/// Disable mipi clock.
pub const HI_MIPI_DISABLE_MIPI_CLOCK: u32 = 1_074_031_885;

/// Enable slvs clock.
pub const HI_MIPI_ENABLE_SLVS_CLOCK: u32 = 1_074_031_886;

/// Disable slvs clock.
pub const HI_MIPI_DISABLE_SLVS_CLOCK: u32 = 1_074_031_887;

/// Enable sensor clock.
pub const HI_MIPI_ENABLE_SENSOR_CLOCK: u32 = 1_074_031_888;

/// Disable sensor clock.
pub const HI_MIPI_DISABLE_SENSOR_CLOCK: u32 = 1_074_031_889;

/// Clear all states of the combo device.
pub const HI_MIPI_CLEAR: u32 = 1_074_031_890;

impl std::fmt::Debug for mipi_dev_attr_t {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mipi_dev_attr_t {{ input_data_type: {:?}, wdr_mode: {:?}, lane_id: {:?} }}",
            self.input_data_type, self.wdr_mode, self.lane_id
        )
    }
}

impl std::fmt::Debug for combo_dev_attr_t {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let comm = format!(
            "devno: {:?}, input_mode: {:?}, data_rate: {:?}, img_rect: {:?}",
            self.devno, self.input_mode, self.data_rate, self.img_rect
        );
        let spec = match self.input_mode {
            input_mode_t::INPUT_MODE_MIPI => unsafe { format!("{:?}", self.un1.mipi_attr) },
            _ => String::new(),
        };
        write!(f, "combo_dev_attr_t {{ {}, {} }}", comm, spec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::{close, ioctl, open, O_RDWR};
    use std::ffi::CString;

    #[test]
    fn set_hs_mode() {
        unsafe {
            let dev = CString::new("/dev/hi_mipi").unwrap();
            let fd = open(dev.as_ptr(), O_RDWR);
            assert!(fd > 0);
            let val: lane_divide_mode_t = lane_divide_mode_t::LANE_DIVIDE_MODE_0;
            let err = ioctl(fd, HI_MIPI_SET_HS_MODE.into(), &val);
            assert_eq!(0, err);
            close(fd);
        }
    }
}
