use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

type DynError = Box<dyn std::error::Error>;

#[derive(Clone, Default)]
struct MyError(String);

impl MyError {
    pub fn new<S: AsRef<str>>(msg: S) -> Self {
        Self {
            0: String::from(msg.as_ref()),
        }
    }
}

impl std::fmt::Debug for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MyError {{ {} }}", self.0)
    }
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for MyError {}

macro_rules! MyErr {
    ($msg: expr) => {
        Err(Box::new(MyError::new($msg)))
    };
}

#[derive(Copy, Clone, Debug, Default)]
struct MyParseCallbacks;

impl bindgen::callbacks::ParseCallbacks for MyParseCallbacks {
    /// Allows to rename an item, replacing `original_item_name`.
    #[allow(clippy::single_match)]
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        // Special cases
        match original_item_name {
            "hifbDYNAMIC_RANGE_E" => {
                return Some(String::from("HIFB_DYNAMIC_RANGE_E"));
            }
            _ => {}
        }
        //
        let re = Regex::new(r"^(hi|hifb)([^a-z]+)$").unwrap();
        if let Some(cap) = re.captures_iter(original_item_name).next() {
            return Some(cap[2].to_string());
        }
        let re = Regex::new(r"^(hi|hifb)([^a-z]+)__bindgen_ty_(\d)$").unwrap();
        if let Some(cap) = re.captures_iter(original_item_name).next() {
            return Some(format!("{}_U{}", &cap[2], &cap[3]));
        }
        None
    }
}

fn detect_mpp_path(mpp_dir: &str) -> Result<PathBuf, DynError> {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut base_path = Path::new(&dir);
    for _a in 0..9 {
        let np = base_path.join(mpp_dir);
        let path = Path::new(&np);
        if path.exists() {
            return Ok(path.to_path_buf());
        }
        match base_path.parent() {
            Some(v) => base_path = v,
            None => break,
        }
    }
    MyErr!(format!("The `MPP_DIR={}` does not detected!", mpp_dir))
}

fn setup_envir() -> Result<(), DynError> {
    if let Ok(val) = env::var("TARGET") {
        if val == "x86_64-unknown-linux-gnu" {
            return MyErr!("Target not supported!");
        }
    }

    if env::var("MPP_DIR").is_err() {
        #[cfg(any(
            feature = "hi3516ev200",
            feature = "hi3516ev300",
            feature = "hi3518ev200",
            feature = "hi3518ev300"
        ))]
        env::set_var(
            "MPP_DIR",
            detect_mpp_path("vendor/mpp-lib-Hi3516EV200_V1.0.1.0").unwrap(),
        );

        #[cfg(feature = "hi3531v100")]
        env::set_var(
            "MPP_DIR",
            detect_mpp_path("vendor/mpp-lib-Hi3531V100_V1.0.D.0").unwrap(),
        );

        #[cfg(feature = "hi3559av100")]
        env::set_var(
            "MPP_DIR",
            detect_mpp_path("vendor/mpp-lib-Hi3559AV100_V2.0.2.0").unwrap(),
        );
    }

    if env::var("SYS_INCLUDE").is_err() {
        #[cfg(any(
            feature = "hi3516ev200",
            feature = "hi3516ev300",
            feature = "hi3518ev200",
            feature = "hi3518ev300"
        ))]
        env::set_var(
            "SYS_INCLUDE",
            "/opt/hisi-linux/x86-arm/arm-himix100-linux/target/usr/include",
        );

        #[cfg(feature = "hi3531v100")]
        env::set_var(
            "SYS_INCLUDE",
            "/opt/hisi-linux-nptl/arm-hisiv100-linux/target/usr/include",
        );

        #[cfg(feature = "hi3559av100")]
        env::set_var(
            "SYS_INCLUDE",
            "/opt/hisi-linux/x86-arm/aarch64-himix100-linux/aarch64-linux-gnu/sys-include",
        );
    }

    Ok(())
}

fn main() -> Result<(), DynError> {
    if cfg!(not(any(
        feature = "hi3516ev200",
        feature = "hi3516ev300",
        feature = "hi3518ev200",
        feature = "hi3518ev300",
        feature = "hi3519av100",
        feature = "hi3531v100",
        feature = "hi3559av100",
    ))) {
        return MyErr!("The target board does not specified!");
    }

    println!("cargo:rerun-if-env-changed=MPP_DIR");
    println!("cargo:rerun-if-env-changed=SYS_INCLUDE");
    println!("cargo:rerun-if-changed=build.rs");

    setup_envir()?;

    let mpp_dir = env::var("MPP_DIR").unwrap();
    if !Path::new(&mpp_dir).exists() {
        return MyErr!(format!("The `MPP_DIR={}` does not exists", mpp_dir));
    }

    println!("cargo:rustc-link-search=native={}/lib", mpp_dir);

    let wrapper_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("wrapper.h");
    let wrapper_path = wrapper_path.to_str().unwrap();
    let mut wrapper = File::create(wrapper_path).unwrap();
    writeln!(wrapper, "#include <hi_mipi.h>")?;

    let bindings = bindgen::Builder::default()
        .header(wrapper_path)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .anon_fields_prefix("un")
        .derive_debug(true)
        .impl_debug(false)
        .derive_default(true)
        .derive_partialeq(true)
        .derive_eq(true)
        .impl_partialeq(true)
        .whitelist_function("^HI_.*")
        .whitelist_type("combo_dev_t")
        .whitelist_type("sns_rst_source_t")
        .whitelist_type("sns_clk_source_t")
        .whitelist_type("lane_divide_mode_t")
        .whitelist_type("work_mode_t")
        .whitelist_type("input_mode_t")
        .whitelist_type("mipi_data_rate_t")
        .whitelist_type("img_rect_t")
        .whitelist_type("img_size_t")
        .whitelist_type("data_type_t")
        .whitelist_type("mipi_wdr_mode_t")
        .whitelist_type("slvs_lane_rate_t")
        .whitelist_type("mipi_dev_attr_t")
        .whitelist_type("wdr_mode_t")
        .whitelist_type("lvds_sync_mode_t")
        .whitelist_type("lvds_vsync_type_t")
        .whitelist_type("lvds_vsync_attr_t")
        .whitelist_type("lvds_fid_type_t")
        .whitelist_type("lvds_fid_attr_t")
        .whitelist_type("lvds_bit_endian_t")
        .whitelist_type("lvds_dev_attr_t")
        .whitelist_type("slvs_dev_attr_t")
        .whitelist_type("combo_dev_attr_t")
        .whitelist_type("phy_cmv_mode_t")
        .whitelist_type("phy_cmv_t")
        .whitelist_type("^HI_.*$")
        .whitelist_type("^MIPI_FRAME_INT_ERR")
        .whitelist_type("^MIPI_PKT_INT1_ERR")
        .whitelist_type("^MIPI_PKT_INT2_ERR")
        .whitelist_type("^LINK_INT_STAT")
        .whitelist_type("^MIPI_CTRL_INT_ERR")
        .whitelist_type("^LVDS_CTRL_INTR_ERR")
        .whitelist_type("^ALIGN_CTRL_INT_ERR")
        .whitelist_type("^SLVS_LINK_INT_STAT")
        .whitelist_var("^COMBO.*")
        .whitelist_var("^CMOS.*")
        .whitelist_var("^HI.*")
        .whitelist_var("^LVDS.*")
        .whitelist_var("^MAX.*")
        .whitelist_var("^MIPI.*")
        .whitelist_var("^SLVS.*")
        .whitelist_var("^SNS.*")
        .whitelist_var("^SYNC.*")
        .whitelist_var("^WDR.*")
        .use_core()
        .clang_arg(format!("-I{}/include", env::var("MPP_DIR").unwrap()))
        .clang_arg(format!("-I{}", env::var("SYS_INCLUDE").unwrap()))
        .parse_callbacks(Box::new(MyParseCallbacks::default()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
