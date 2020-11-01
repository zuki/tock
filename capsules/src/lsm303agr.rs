//! Driver for the LSM303AGR 3D accelerometer and 3D magnetometer sensor.
//!
//! May be used with NineDof and Temperature
//!
//! I2C Interface
//!
//! <https://www.st.com/resource/en/datasheet/lsm303agr.pdf>
//!
//! The syscall interface is described in [lsm303agr.md](https://github.com/tock/tock/tree/master/doc/syscalls/70006_lsm303agr.md)
//!
//! Usage
//! -----
//!
//! ```rust
//! let mux_i2c = components::i2c::I2CMuxComponent::new(&stm32f3xx::i2c::I2C1)
//!     .finalize(components::i2c_mux_component_helper!());
//!
//! let lsm303agr = components::lsm303agr::Lsm303agrI2CComponent::new()
//!    .finalize(components::lsm303agr_i2c_component_helper!(mux_i2c));
//!
//! lsm303agr.configure(
//!    lsm303agr::Lsm303agrAccelDataRate::DataRate25Hz,
//!    false,
//!    lsm303agr::Lsm303agrScale::Scale2G,
//!    false,
//!    true,
//!    lsm303agr::Lsm303agrMagnetoDataRate::DataRate3_0Hz,
//!    lsm303agr::Lsm303agrRange::Range4_7G,
//!);
//! ```
//!
//! NideDof Example
//!
//! ```rust
//! let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);
//! let grant_ninedof = board_kernel.create_grant(&grant_cap);
//!
//! // use as primary NineDof Sensor
//! let ninedof = static_init!(
//!    capsules::ninedof::NineDof<'static>,
//!    capsules::ninedof::NineDof::new(lsm303agr, grant_ninedof)
//! );
//!
//! hil::sensors::NineDof::set_client(lsm303agr, ninedof);
//!
//! // use as secondary NineDof Sensor
//! let lsm303agr_secondary = static_init!(
//!    capsules::ninedof::NineDofNode<'static, &'static dyn hil::sensors::NineDof>,
//!    capsules::ninedof::NineDofNode::new(lsm303agr)
//! );
//! ninedof.add_secondary_driver(lsm303agr_secondary);
//! hil::sensors::NineDof::set_client(lsm303agr, ninedof);
//! ```
//!
//! Temperature Example
//!
//! ```rust
//! let grant_cap = create_capability!(capabilities::MemoryAllocationCapability);
//! let grant_temp = board_kernel.create_grant(&grant_cap);
//!
//! lsm303agr.configure(
//!    lsm303agr::Lsm303agrAccelDataRate::DataRate25Hz,
//!    false,
//!    lsm303agr::Lsm303agrScale::Scale2G,
//!    false,
//!    true,
//!    lsm303agr::Lsm303agrMagnetoDataRate::DataRate3_0Hz,
//!    lsm303agr::Lsm303agrRange::Range4_7G,
//!);
//! let temp = static_init!(
//! capsules::temperature::TemperatureSensor<'static>,
//!     capsules::temperature::TemperatureSensor::new(lsm303agr, grant_temperature));
//! kernel::hil::sensors::TemperatureDriver::set_client(lsm303agr, temp);
//! ```
//!
//! Author: Alexandru Radovici <msg4alex@gmail.com>
//!

#![allow(non_camel_case_types)]

use core::cell::Cell;
use enum_primitive::cast::FromPrimitive;
use enum_primitive::enum_from_primitive;
use kernel::common::cells::{OptionalCell, TakeCell};
use kernel::common::registers::register_bitfields;
use kernel::hil::i2c::{self, Error};
use kernel::hil::sensors;
use kernel::{AppId, Callback, Driver, ReturnCode};

register_bitfields![u8,
    CTRL_REG1_A [
        /// Output data rate
        ODR OFFSET(4) NUMBITS(4) [],    // [0000]->[0011] Lsm303agrAccelDataRate
        /// Low Power enable
        LPEN OFFSET(3) NUMBITS(1) [],   // [0]->[-] 0: Normal, 1: LowPower （詳細はLPEN+HR） 
        /// Z enable
        ZEN OFFSET(2) NUMBITS(1) [],    // [1]->[-] Z-axis 0: 無効, 1: 有効
        /// Y enable
        YEN OFFSET(1) NUMBITS(1) [],    // [1]->[-] Y-axis 0: 無効, 1: 有効
        /// X enable
        XEN OFFSET(0) NUMBITS(1) []     // [1]->[-] Z-axis 0: 無効, 1: 有効
    ],
    CTRL_REG4_A [
        /// Block Data update
        BDU OFFSET(7) NUMBITS(1) [],    // [0]->[-] 0: 継続更新, 1: MSB/LSB読み込み後に更新
        /// Big Little Endian
        BLE OFFSET(6) NUMBITS(1) [],    // [0]->[-] 0: リトル, 1: ビッグ
        /// Full Scale selection
        FS OFFSET(4) NUMBITS(2) [],     // [00]->[-] 00: 2g, 01: 4g, 10: 8g, 11: 16g
        /// High Resolution                            Normal    HighReso    LowPower
        HR OFFSET(3) NUMBITS(1) [],     // [0]->[1] LPEN+HR 00; 10bit, 01: 12bit, 10: 8bit, 11: NotAllowed
        /// Self-test
        ST OFFSET(1) NUMBITS(2) [],     // [00]->[-] 00: 無効, 01: Test 0, 10: Test 11: NotAllowed
        /// 3-wire SPI Interface
        SPI OFFSET(0) NUMBITS(1) []     // [0]->[-] 0: 無効, 1: 有効 
    ],
    TEMP_CFG_REG_A [
        /// Temperature sensor enable
        TEMP_EN OFFSET(6) NUMBITS(2) [] // [00]->[11] 00: 無効, 11: 有効
    ],
    CFG_REG_A_M [
        /// Enable magnetometer temperature compensation
        COMP_TEMP_EN OFFSET(7) NUMBITS(1) [],  // [0]->[1] 0: 無効, 1: 有効
        /// Reboot magnetometer memory content
        REBOOT OFFSET(6) NUMBITS(1) [],     // [0]->[-] 0: 無効, 1: 有効 (Reboot memory content)
        /// Soft reset
        SOFT_RST OFFSET(5) NUMBITS(1) [],   // [0]->[-] 1: config and user regs reset
        /// Low-power mode enable
        LP OFFSET(4) NUMBITS(1) [],         // [0]->[-] 0: 高精度モード, 1: 低電力モード
        /// Output data rate (Hz)
        ODR OFFSET(2) NUMBITS(2) [],        // [00]->[-] 00: 10, 01: 20, 10: 50, 11: 100
        /// Device operation Mode Select
        MD OFFSET(0) NUMBITS(2) []          // [11]->[00] 00: 継続出力, 01: ワンショット, 10/11: アイドル
    ],
    CFG_REG_B_M [
        /// Enable offset cancellation in single mode
        OFF_CANC_ONE_SHOT OFFSET(4) NUMBITS(1) [],  // [0]->[-] 0: 無効, 1: 有効
        /// 
        INT_on_DataOFF OFFSET(3) NUMBITS(1) [],     // [0]->[-]
        /// Select frequency of set pulse
        Set_FREQ OFFSET(2) NUMBITS(1) [],           // [0]->[-]
        // Enables offset cancellation
        OFF_CANC OFFSET(1) NUMBITS(1) [],           // [0]->[1] 0: 無効, 1: 有効
        /// Low-pass filter enable
        LPF OFFSET(0) NUMBITS(1) []                 // [0]->[-] 0: 無効, 1: 有効
    ]
];

use crate::driver;

/// Syscall driver number.
pub const DRIVER_NUM: usize = driver::NUM::Lsm303dlch as usize;     // 暫定 0x71006

// Buffer to use for I2C messages
pub static mut BUFFER: [u8; 8] = [0; 8];

/// Register values
const REGISTER_AUTO_INCREMENT: u8 = 0x80;

// WHO_AM_I values
const LSM303AGR_WHO_AM_I_A: u8 = 0x33;
//const LSM303AGR_WHO_AM_I_M: u8 = 0x40;

enum_from_primitive! {
    enum AccelerometerRegisters {
        STATUS_REG_AUX_A = 0x07,
        OUT_TEMP_L_A = 0x0C,
        OUT_TEMP_H_A = 0x0D,
        WHO_AM_I_A = 0x0F,
        TEMP_CFG_REG_A = 0x1F,
        CTRL_REG1_A = 0x20,
        CTRL_REG4_A = 0x23,
        OUT_X_L_A = 0x28,
        OUT_X_H_A = 0x29,
        OUT_Y_L_A = 0x2A,
        OUT_Y_H_A = 0x2B,
        OUT_Z_L_A = 0x2C,
        OUT_Z_H_A = 0x2D,
    }
}

enum_from_primitive! {
    enum MagnetometerRegisters {
        WHO_AM_I_M = 0x4F,
        CFG_REG_A_M = 0x60,
        CFG_REG_B_M = 0x61,
        OUTX_L_REG_M = 0x68,
        OUTX_H_REG_M = 0x69,
        OUTY_L_REG_M = 0x6A,
        OUTY_H_REG_M = 0x6B,
        OUTZ_L_REG_M = 0x6C,
        OUTZ_H_REG_M = 0x6D,
    }
}

// Experimental
const TEMP_OFFSET: f32 = 25.0;

// Manual page Table 35, page 47
enum_from_primitive! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum Lsm303agrAccelDataRate {       // CRTL_REG_A[7:4]
        Off = 0,
        DataRate1Hz = 1,
        DataRate10Hz = 2,
        DataRate25Hz = 3,
        DataRate50Hz = 4,
        DataRate100Hz = 5,
        DataRate200Hz = 6,
        DataRate400Hz = 7,
        LowPower1620Hz = 8,
        Normal1344LowPower5376Hz = 9,
    }
}

// Manual table 72, page 25
enum_from_primitive! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum Lsm303agrMagnetoDataRate {     // CFG_REG_A_M[3:2]
        DataRate10Hz = 0,
        DataRate20Hz = 1,
        DataRate50Hz = 2,
        DataRate100Hz = 3,
    }
}

enum_from_primitive! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum Lsm303agrScale {           // CTRL_REG4_A[5:4]  
        Scale2G = 0,
        Scale4G = 1,
        Scale8G = 2,
        Scale16G = 3
    }
}

// Manual table 42, page 49 (milli-gauss)
const SCALE_FACTOR: [f32; 12] = [
    0.98, 1.95, 3.9, 11.72,         // High-resolution (12-bit)
    3.9, 7.82, 15.63, 46.9,         // Normal (10-bit)
    15.63, 31.26, 62.52, 187.58     // Low-power
];

// Manual table 3, page 13
const RANGE_FACTOR: f32 = 1.5;   // milii-gauss/LSB

#[derive(Clone, Copy, PartialEq)]
enum State {
    Idle,
    IsPresent,
    SetPowerMode,
    SetScaleAndResolution,
    ReadAccelerationXYZ,
    SetTemperature,
    SetMagDataRate,
    ReadTemperature,
    ReadMagnetometerXYZ,
}

pub struct Lsm303agrI2C<'a> {
    config_in_progress: Cell<bool>,             // 1: 初期設定中
    i2c_accelerometer: &'a dyn i2c::I2CDevice,
    i2c_magnetometer: &'a dyn i2c::I2CDevice,
    callback: OptionalCell<Callback>,
    state: Cell<State>,
    accel_scale: Cell<Lsm303agrScale>,              // 加速度測定スケール(CTRL_REG4_A[FS])
    accel_high_resolution: Cell<bool>,              // 加速度測定高精度(CTRL_REG4_A[HR]) 
    mag_data_rate: Cell<Lsm303agrMagnetoDataRate>,  // 磁気ODR(CFG_REG_A_M[ODR])
    accel_data_rate: Cell<Lsm303agrAccelDataRate>,  // 加速度ODR(CTRL_REG1_A[ODR])
    low_power: Cell<bool>,                          // 低電力モード(CTRL_REG1_A[LPEN])
    temperature: Cell<bool>,                        // 温度測定(TEMP_CFG_REG_A[TEMP_EN])
    buffer: TakeCell<'static, [u8]>,
    nine_dof_client: OptionalCell<&'a dyn sensors::NineDofClient>,
    temperature_client: OptionalCell<&'a dyn sensors::TemperatureClient>,
}

impl<'a> Lsm303agrI2C<'a> {
    pub fn new(
        i2c_accelerometer: &'a dyn i2c::I2CDevice,
        i2c_magnetometer: &'a dyn i2c::I2CDevice,
        buffer: &'static mut [u8],
    ) -> Lsm303agrI2C<'a> {
        // setup and return struct
        Lsm303agrI2C {
            config_in_progress: Cell::new(false),
            i2c_accelerometer: i2c_accelerometer,
            i2c_magnetometer: i2c_magnetometer,
            callback: OptionalCell::empty(),
            state: Cell::new(State::Idle),
            accel_scale: Cell::new(Lsm303agrScale::Scale2G),
            accel_high_resolution: Cell::new(false),
            mag_data_rate: Cell::new(Lsm303agrMagnetoDataRate::DataRate10Hz),
            accel_data_rate: Cell::new(Lsm303agrAccelDataRate::DataRate1Hz),
            low_power: Cell::new(false),
            temperature: Cell::new(false),
            buffer: TakeCell::new(buffer),
            nine_dof_client: OptionalCell::empty(),
            temperature_client: OptionalCell::empty(),
        }
    }

    pub fn configure(
        &self,
        accel_data_rate: Lsm303agrAccelDataRate,
        low_power: bool,
        accel_scale: Lsm303agrScale,
        accel_high_resolution: bool,
        temperature: bool,
        mag_data_rate: Lsm303agrMagnetoDataRate,
    ) {
        if self.state.get() == State::Idle {
            self.config_in_progress.set(true);

            self.accel_scale.set(accel_scale);
            self.accel_high_resolution.set(accel_high_resolution);
            self.low_power.set(low_power);
            self.temperature.set(temperature);
            self.mag_data_rate.set(mag_data_rate);
            self.accel_data_rate.set(accel_data_rate);

            self.set_power_mode(accel_data_rate, low_power);
        }
    }

    fn is_present(&self) {
        self.state.set(State::IsPresent);
        self.buffer.take().map(|buf| {
            buf[0] = AccelerometerRegisters::WHO_AM_I_A as u8;
            buf[1] = 0;
            self.i2c_accelerometer.write_read(buf, 1, 2);
        });
    }

    fn set_power_mode(&self, data_rate: Lsm303agrAccelDataRate, low_power: bool) {
        if self.state.get() == State::Idle {
            self.state.set(State::SetPowerMode);
            self.buffer.take().map(|buf| {
                buf[0] = AccelerometerRegisters::CTRL_REG1_A as u8;
                buf[1] = (CTRL_REG1_A::ODR.val(data_rate as u8)
                    + CTRL_REG1_A::LPEN.val(low_power as u8)
                    + CTRL_REG1_A::ZEN::SET
                    + CTRL_REG1_A::YEN::SET
                    + CTRL_REG1_A::XEN::SET)
                    .value;
                self.i2c_accelerometer.write(buf, 2);
            });
        }
    }

    fn set_scale_and_resolution(&self, scale: Lsm303agrScale, high_resolution: bool) {
        if self.state.get() == State::Idle {
            self.state.set(State::SetScaleAndResolution);
            // TODO move these in completed
            self.accel_scale.set(scale);
            self.accel_high_resolution.set(high_resolution);
            self.buffer.take().map(|buf| {
                buf[0] = AccelerometerRegisters::CTRL_REG4_A as u8;
                buf[1] = (CTRL_REG4_A::BDU::SET
                    + CTRL_REG4_A::FS.val(scale as u8)
                    + CTRL_REG4_A::HR.val(high_resolution as u8)).value;
                self.i2c_accelerometer.write(buf, 2);
            });
        }
    }

    fn read_acceleration_xyz(&self) {
        if self.state.get() == State::Idle {
            self.state.set(State::ReadAccelerationXYZ);
            self.buffer.take().map(|buf| {
                buf[0] = AccelerometerRegisters::OUT_X_L_A as u8 | REGISTER_AUTO_INCREMENT;
                self.i2c_accelerometer.write_read(buf, 1, 6);
            });
        }
    }

    fn set_temperature(
        &self,
        temperature: bool
    ) {
        if self.state.get() == State::Idle {
            self.state.set(State::SetTemperature);
            let temp_en: u8 = if temperature { 3 } else { 0 };
            self.buffer.take().map(|buf| {
                buf[0] = AccelerometerRegisters::TEMP_CFG_REG_A as u8;
                buf[1] = (TEMP_CFG_REG_A::TEMP_EN.val(temp_en)).value;
                self.i2c_accelerometer.write(buf, 2);
            });
        }
    }

    fn set_magneto_data_rate(
        &self,
        data_rate: Lsm303agrMagnetoDataRate,
    ) {
        if self.state.get() == State::Idle {
            self.state.set(State::SetMagDataRate);
            self.buffer.take().map(|buf| {
                buf[0] = MagnetometerRegisters::CFG_REG_A_M as u8;
                buf[1] = (CFG_REG_A_M::COMP_TEMP_EN::SET
                    + CFG_REG_A_M::ODR.val(data_rate as u8)
                    + CFG_REG_A_M::MD.val(0)).value;
                buf[2] = 0x02;      // CFG_REG_G_M::OFF_CANC::SET
                self.i2c_magnetometer.write(buf, 3);
            });
        }
    }

    fn read_temperature(&self) {
        if self.state.get() == State::Idle {
            self.state.set(State::ReadTemperature);
            self.buffer.take().map(|buf| {
                buf[0] = AccelerometerRegisters::OUT_TEMP_L_A as u8 | REGISTER_AUTO_INCREMENT;
                self.i2c_magnetometer.write_read(buf, 1, 2);
            });
        }
    }

    fn read_magnetometer_xyz(&self) {
        if self.state.get() == State::Idle {
            self.state.set(State::ReadMagnetometerXYZ);
            self.buffer.take().map(|buf| {
                buf[0] = MagnetometerRegisters::OUTX_L_REG_M as u8;
                self.i2c_magnetometer.write_read(buf, 1, 6);
            });
        }
    }

    fn get_accel_mode(&self) -> u8 {
        if self.accel_high_resolution.get() {
            0
        } else if self.low_power.get() {
            2
        } else {
            1
        }
    }
}

impl i2c::I2CClient for Lsm303agrI2C<'_> {
    fn command_complete(&self, buffer: &'static mut [u8], error: Error) {
        match self.state.get() {
            State::IsPresent => {
                let mut ret: usize = 0;
                 let present = if error == Error::CommandComplete && buffer[1] == LSM303AGR_WHO_AM_I_A {
                    ret = buffer[1] as usize;
                    true
                } else {
                    false
                };

                self.callback.map(|callback| {
                    callback.schedule(if present { 1 } else { 0 }, ret, 0);
                });
                self.buffer.replace(buffer);
                self.state.set(State::Idle);
            }
            State::SetPowerMode => {
                let set_power = error == Error::CommandComplete;

                self.callback.map(|callback| {
                    callback.schedule(if set_power { 1 } else { 0 }, 0, 0);
                });
                self.buffer.replace(buffer);
                self.state.set(State::Idle);
                if self.config_in_progress.get() {
                    self.set_scale_and_resolution(
                        self.accel_scale.get(),
                        self.accel_high_resolution.get(),
                    );
                }
            }
            State::SetScaleAndResolution => {
                let set_scale_and_resolution = error == Error::CommandComplete;

                self.callback.map(|callback| {
                    callback.schedule(if set_scale_and_resolution { 1 } else { 0 }, 0, 0);
                });
                self.buffer.replace(buffer);
                self.state.set(State::Idle);
                if self.config_in_progress.get() {
                    self.set_temperature(
                        self.temperature.get(),
                    );
                }
            }
            State::ReadAccelerationXYZ => {
                let mut x: usize = 0;
                let mut y: usize = 0;
                let mut z: usize = 0;
                let values = if error == Error::CommandComplete {
                    let mode = self.get_accel_mode() as usize;
                    self.nine_dof_client.map(|client| {
                        let scale_factor = self.accel_scale.get() as usize;
                        let idx = mode * 4 + scale_factor;
                        x = match mode {
                            0 => ((((buffer[0] as i16 | ((buffer[1] as i16) << 8)) >> 4) as f32)
                                * SCALE_FACTOR[idx]) as usize,
                            1 => ((((buffer[0] as i16 | ((buffer[1] as i16) << 8)) >> 6) as f32)
                                * SCALE_FACTOR[idx]) as usize,
                            _ => ((((buffer[0] as i16 | ((buffer[1] as i16) << 8)) >> 8) as f32)
                                * SCALE_FACTOR[idx]) as usize
                        };
                        y = match mode {
                            0 => ((((buffer[2] as i16 | ((buffer[3] as i16) << 8)) >> 4) as f32)
                                * SCALE_FACTOR[idx]) as usize,
                            1 => ((((buffer[2] as i16 | ((buffer[3] as i16) << 8)) >> 6) as f32)
                                * SCALE_FACTOR[idx]) as usize,
                            _ => ((((buffer[2] as i16 | ((buffer[3] as i16) << 8)) >> 8) as f32)
                                * SCALE_FACTOR[idx]) as usize
                        };
                        z = match mode {
                            0 => ((((buffer[4] as i16 | ((buffer[5] as i16) << 8)) >> 4) as f32)
                                * SCALE_FACTOR[idx]) as usize,
                            1 => ((((buffer[4] as i16 | ((buffer[5] as i16) << 8)) >> 6) as f32)
                                * SCALE_FACTOR[idx]) as usize,
                            _ => ((((buffer[4] as i16 | ((buffer[5] as i16) << 8)) >> 8) as f32)
                                * SCALE_FACTOR[idx]) as usize
                        };
                        client.callback(x, y, z);   // milli-gauss
                    });

                    x = match mode {
                        0 => (((buffer[0] as i16 | ((buffer[1] as i16) << 8)) >> 4) as usize),
                        1 => (((buffer[0] as i16 | ((buffer[1] as i16) << 8)) >> 6) as usize),
                        _ => (((buffer[0] as i16 | ((buffer[1] as i16) << 8)) >> 8) as usize)
                    };
                    y = match mode {
                        0 => (((buffer[2] as i16 | ((buffer[3] as i16) << 8)) >> 4) as usize),
                        1 => (((buffer[2] as i16 | ((buffer[3] as i16) << 8)) >> 6) as usize),
                        _ => (((buffer[2] as i16 | ((buffer[3] as i16) << 8)) >> 8) as usize)
                    };
                    z = match mode {
                        0 => (((buffer[4] as i16 | ((buffer[5] as i16) << 8)) >> 4) as usize),
                        1 => (((buffer[4] as i16 | ((buffer[5] as i16) << 8)) >> 6) as usize),
                        _ => (((buffer[4] as i16 | ((buffer[5] as i16) << 8)) >> 8) as usize)
                    };
                    true
                } else {
                    self.nine_dof_client.map(|client| {
                        client.callback(0, 0, 0);
                    });
                    false
                };
                if values {
                    self.callback.map(|callback| {
                        callback.schedule(x, y, z);
                    });
                } else {
                    self.callback.map(|callback| {
                        callback.schedule(0, 0, 0);
                    });
                }
                self.buffer.replace(buffer);
                self.state.set(State::Idle);
            }
            State::SetTemperature => {
                let set_temperature = error == Error::CommandComplete;

                self.callback.map(|callback| {
                    callback.schedule(
                        if set_temperature {
                            1
                        } else {
                            0
                        },
                        0,
                        0,
                    );
                });
                self.buffer.replace(buffer);
                self.state.set(State::Idle);
                if self.config_in_progress.get() {
                    self.set_magneto_data_rate(self.mag_data_rate.get());
                }
            }
            State::SetMagDataRate => {
                let set_magneto_data_rate = error == Error::CommandComplete;

                self.callback.map(|callback| {
                    callback.schedule(
                        if set_magneto_data_rate {
                            1
                        } else {
                            0
                        },
                        0,
                        0,
                    );
                });
                if self.config_in_progress.get() {
                    self.config_in_progress.set(false);
                }
                self.buffer.replace(buffer);
                self.state.set(State::Idle);
            }
            State::ReadTemperature => {
                let mut temp: i16 = 0;
                let values = if error == Error::CommandComplete {
                    temp = (buffer[1] as i16 | ((buffer[0] as i16) << 8)) >> 6;
                    self.temperature_client.map(|client| {
                        client.callback((temp as f32 / 4. + TEMP_OFFSET) as i16 as usize);
                    });
                    true
                } else {
                    self.temperature_client.map(|client| {//
                        client.callback(0);
                    });
                    false
                };
                if values {
                    self.callback.map(|callback| {
                        callback.schedule(temp as usize, 0, 0);
                    });
                } else {
                    self.callback.map(|callback| {
                        callback.schedule(0, 0, 0);
                    });
                }
                self.buffer.replace(buffer);
                self.state.set(State::Idle);
            }
            State::ReadMagnetometerXYZ => {
                let mut x: usize = 0;
                let mut y: usize = 0;
                let mut z: usize = 0;
                let values = if error == Error::CommandComplete {
                    self.nine_dof_client.map(|client| {
                        x = (((buffer[1] as i16 | ((buffer[0] as i16) << 8)) as f32)
                            * RANGE_FACTOR) as usize;
                        z = (((buffer[3] as i16 | ((buffer[2] as i16) << 8)) as f32)
                            * RANGE_FACTOR) as usize;
                        y = (((buffer[5] as i16 | ((buffer[4] as i16) << 8)) as f32)
                            * RANGE_FACTOR) as usize;
                        client.callback(x, y, z);   // nano gause
                    });

                    x = ((buffer[1] as u16 | ((buffer[0] as u16) << 8)) as i16) as usize;
                    z = ((buffer[3] as u16 | ((buffer[2] as u16) << 8)) as i16) as usize;
                    y = ((buffer[5] as u16 | ((buffer[4] as u16) << 8)) as i16) as usize;
                    true
                } else {
                    self.nine_dof_client.map(|client| {
                        client.callback(0, 0, 0);
                    });
                    false
                };
                if values {
                    self.callback.map(|callback| {
                        callback.schedule(x, y, z);
                    });
                } else {
                    self.callback.map(|callback| {
                        callback.schedule(0, 0, 0);
                    });
                }
                self.buffer.replace(buffer);
                self.state.set(State::Idle);
            }
            _ => {
                self.buffer.replace(buffer);
            }
        }
    }
}

impl Driver for Lsm303agrI2C<'_> {
    fn command(&self, command_num: usize, data1: usize, data2: usize, _: AppId) -> ReturnCode {
        match command_num {
            0 => ReturnCode::SUCCESS,
            // Check is sensor is correctly connected
            1 => {
                if self.state.get() == State::Idle {
                    self.is_present();
                    ReturnCode::SUCCESS
                } else {
                    ReturnCode::EBUSY
                }
            }
            // Set Accelerometer Power Mode
            2 => {
                if self.state.get() == State::Idle {
                    if let Some(data_rate) = Lsm303agrAccelDataRate::from_usize(data1) {
                        self.set_power_mode(data_rate, if data2 != 0 { true } else { false });
                        ReturnCode::SUCCESS
                    } else {
                        ReturnCode::EINVAL
                    }
                } else {
                    ReturnCode::EBUSY
                }
            }
            // Set Accelerometer Scale And Resolution
            3 => {
                if self.state.get() == State::Idle {
                    if let Some(scale) = Lsm303agrScale::from_usize(data1) {
                        self.set_scale_and_resolution(scale, if data2 != 0 { true } else { false });
                        ReturnCode::SUCCESS
                    } else {
                        ReturnCode::EINVAL
                    }
                } else {
                    ReturnCode::EBUSY
                }
            }
            // Set Magnetometer Temperature Enable
            4 => {
                if self.state.get() == State::Idle {
                    self.set_temperature(if data1 != 0 { true } else { false });
                    ReturnCode::SUCCESS
                } else {
                    ReturnCode::EBUSY
                }
            }
            // Set Magnetometer Data Rate
            5 => {
                if self.state.get() == State::Idle {
                    if let Some(data_rate) = Lsm303agrMagnetoDataRate::from_usize(data1) {
                        self.set_magneto_data_rate(data_rate);
                        ReturnCode::SUCCESS
                    } else {
                        ReturnCode::EINVAL
                    }
                } else {
                    ReturnCode::EBUSY
                }
            }
            // Read Acceleration XYZ
            6 => {
                if self.state.get() == State::Idle {
                    self.read_acceleration_xyz();
                    ReturnCode::SUCCESS
                } else {
                    ReturnCode::EBUSY
                }
            }
            // Read Temperature
            7 => {
                if self.state.get() == State::Idle {
                    self.read_temperature();
                    ReturnCode::SUCCESS
                } else {
                    ReturnCode::EBUSY
                }
            }
            // Read Mangetometer XYZ
            8 => {
                if self.state.get() == State::Idle {
                    self.read_magnetometer_xyz();
                    ReturnCode::SUCCESS
                } else {
                    ReturnCode::EBUSY
                }
            }
            // default
            _ => ReturnCode::ENOSUPPORT,
        }
    }

    fn subscribe(
        &self,
        subscribe_num: usize,
        callback: Option<Callback>,
        _app_id: AppId,
    ) -> ReturnCode {
        match subscribe_num {
            0 /* set the one shot callback */ => {
				self.callback.insert (callback);
				ReturnCode::SUCCESS
			},
            // default
            _ => ReturnCode::ENOSUPPORT,
        }
    }
}

impl<'a> sensors::NineDof<'a> for Lsm303agrI2C<'a> {
    fn set_client(&self, nine_dof_client: &'a dyn sensors::NineDofClient) {
        self.nine_dof_client.replace(nine_dof_client);
    }

    fn read_accelerometer(&self) -> ReturnCode {
        if self.state.get() == State::Idle {
            self.read_acceleration_xyz();
            ReturnCode::SUCCESS
        } else {
            ReturnCode::EBUSY
        }
    }

    fn read_magnetometer(&self) -> ReturnCode {
        if self.state.get() == State::Idle {
            self.read_magnetometer_xyz();
            ReturnCode::SUCCESS
        } else {
            ReturnCode::EBUSY
        }
    }
}

impl<'a> sensors::TemperatureDriver<'a> for Lsm303agrI2C<'a> {
    fn set_client(&self, temperature_client: &'a dyn sensors::TemperatureClient) {
        self.temperature_client.replace(temperature_client);
    }

    fn read_temperature(&self) -> ReturnCode {
        if self.state.get() == State::Idle {
            self.read_temperature();
            ReturnCode::SUCCESS
        } else {
            ReturnCode::EBUSY
        }
    }
}
