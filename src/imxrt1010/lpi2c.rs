//! I2C pin implementations

use super::pads::{gpio::*, gpio_ad::*, gpio_sd::*};
use crate::{
    consts::*,
    lpi2c::{Pin, Scl, Sda},
    Daisy,
};

//
// I2C1
//

// SCL
i2c!(module: U1, alt: 0, pad: GPIO_AD_14,    signal: Scl, daisy: Some(DAISY_LPI2C1_SCL_GPIO_AD_14));
i2c!(module: U1, alt: 1, pad: GPIO_SD_06,    signal: Scl, daisy: Some(DAISY_LPI2C1_SCL_GPIO_SD_06));
i2c!(module: U1, alt: 1, pad: GPIO_12,       signal: Scl, daisy: Some(DAISY_LPI2C1_SCL_GPIO_12));
i2c!(module: U1, alt: 3, pad: GPIO_02,       signal: Scl, daisy: Some(DAISY_LPI2C1_SCL_GPIO_02));

// SDA
i2c!(module: U1, alt: 0, pad: GPIO_AD_13,    signal: Sda, daisy: Some(DAISY_LPI2C1_SDA_GPIO_AD_13));
i2c!(module: U1, alt: 1, pad: GPIO_SD_05,    signal: Sda, daisy: Some(DAISY_LPI2C1_SDA_GPIO_SD_05));
i2c!(module: U1, alt: 1, pad: GPIO_11,       signal: Sda, daisy: Some(DAISY_LPI2C1_SDA_GPIO_11));
i2c!(module: U1, alt: 3, pad: GPIO_01,       signal: Sda, daisy: Some(DAISY_LPI2C1_SDA_GPIO_01));

//
// I2C2
//

// SCL
i2c!(module: U2, alt: 0, pad: GPIO_AD_08,    signal: Scl, daisy: Some(DAISY_LPI2C2_SCL_GPIO_AD_08));
i2c!(module: U2, alt: 1, pad: GPIO_SD_08,    signal: Scl, daisy: Some(DAISY_LPI2C2_SCL_GPIO_SD_08));
i2c!(module: U2, alt: 3, pad: GPIO_AD_02,    signal: Scl, daisy: Some(DAISY_LPI2C2_SCL_GPIO_AD_02));
i2c!(module: U2, alt: 3, pad: GPIO_10,       signal: Scl, daisy: Some(DAISY_LPI2C2_SCL_GPIO_10));

// SDA
i2c!(module: U2, alt: 0, pad: GPIO_AD_07,    signal: Sda, daisy: Some(DAISY_LPI2C2_SDA_GPIO_AD_07));
i2c!(module: U2, alt: 1, pad: GPIO_SD_07,    signal: Sda, daisy: Some(DAISY_LPI2C2_SDA_GPIO_SD_07));
i2c!(module: U2, alt: 3, pad: GPIO_AD_01,    signal: Sda, daisy: Some(DAISY_LPI2C2_SDA_GPIO_AD_01));
i2c!(module: U2, alt: 3, pad: GPIO_09,       signal: Sda, daisy: Some(DAISY_LPI2C2_SDA_GPIO_09));

mod daisy {
    #![allow(unused)]

    use super::Daisy;
    pub const DAISY_LPI2C1_HREQ_GPIO_AD_06: Daisy = Daisy::new(0x401f81bc as *mut u32, 0);
    pub const DAISY_LPI2C1_HREQ_GPIO_10: Daisy = Daisy::new(0x401f81bc as *mut u32, 1);
    pub const DAISY_LPI2C1_SCL_GPIO_AD_14: Daisy = Daisy::new(0x401f81c0 as *mut u32, 0);
    pub const DAISY_LPI2C1_SCL_GPIO_SD_06: Daisy = Daisy::new(0x401f81c0 as *mut u32, 1);
    pub const DAISY_LPI2C1_SCL_GPIO_12: Daisy = Daisy::new(0x401f81c0 as *mut u32, 2);
    pub const DAISY_LPI2C1_SCL_GPIO_02: Daisy = Daisy::new(0x401f81c0 as *mut u32, 3);
    pub const DAISY_LPI2C1_SDA_GPIO_AD_13: Daisy = Daisy::new(0x401f81c4 as *mut u32, 0);
    pub const DAISY_LPI2C1_SDA_GPIO_SD_05: Daisy = Daisy::new(0x401f81c4 as *mut u32, 1);
    pub const DAISY_LPI2C1_SDA_GPIO_11: Daisy = Daisy::new(0x401f81c4 as *mut u32, 2);
    pub const DAISY_LPI2C1_SDA_GPIO_01: Daisy = Daisy::new(0x401f81c4 as *mut u32, 3);
    pub const DAISY_LPI2C2_SCL_GPIO_AD_08: Daisy = Daisy::new(0x401f81c8 as *mut u32, 0);
    pub const DAISY_LPI2C2_SCL_GPIO_AD_02: Daisy = Daisy::new(0x401f81c8 as *mut u32, 1);
    pub const DAISY_LPI2C2_SCL_GPIO_SD_08: Daisy = Daisy::new(0x401f81c8 as *mut u32, 2);
    pub const DAISY_LPI2C2_SCL_GPIO_10: Daisy = Daisy::new(0x401f81c8 as *mut u32, 3);
    pub const DAISY_LPI2C2_SDA_GPIO_AD_07: Daisy = Daisy::new(0x401f81cc as *mut u32, 0);
    pub const DAISY_LPI2C2_SDA_GPIO_AD_01: Daisy = Daisy::new(0x401f81cc as *mut u32, 1);
    pub const DAISY_LPI2C2_SDA_GPIO_SD_07: Daisy = Daisy::new(0x401f81cc as *mut u32, 2);
    pub const DAISY_LPI2C2_SDA_GPIO_09: Daisy = Daisy::new(0x401f81cc as *mut u32, 3);
}
use daisy::*;
