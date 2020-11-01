//! Components for the LSM303AGR sensor.
//!
//! I2C Interface
//!
//! Usage
//! -----
//! ```rust
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
//! );
//! ```
use capsules::lsm303agr::Lsm303agrI2C;
use capsules::virtual_i2c::I2CDevice;
use core::mem::MaybeUninit;
use kernel::component::Component;
use kernel::static_init_half;

// Setup static space for the objects.
#[macro_export]
macro_rules! lsm303agr_i2c_component_helper {
    ($i2c_mux: expr) => {{
        use capsules::lsm303agr::Lsm303agrI2C;
        use capsules::virtual_i2c::I2CDevice;
        use core::mem::MaybeUninit;
        let accelerometer_i2c = components::i2c::I2CComponent::new($i2c_mux, 0x19)
            .finalize(components::i2c_component_helper!());
        let magnetometer_i2c = components::i2c::I2CComponent::new($i2c_mux, 0x1e)
            .finalize(components::i2c_component_helper!());
        static mut lsm303agr: MaybeUninit<Lsm303agrI2C<'static>> = MaybeUninit::uninit();
        (&accelerometer_i2c, &magnetometer_i2c, &mut lsm303agr)
    };};
}

pub struct Lsm303agrI2CComponent {}

impl Lsm303agrI2CComponent {
    pub fn new() -> Lsm303agrI2CComponent {
        Lsm303agrI2CComponent {}
    }
}

impl Component for Lsm303agrI2CComponent {
    type StaticInput = (
        &'static I2CDevice<'static>,
        &'static I2CDevice<'static>,
        &'static mut MaybeUninit<Lsm303agrI2C<'static>>,
    );
    type Output = &'static Lsm303agrI2C<'static>;

    unsafe fn finalize(self, static_buffer: Self::StaticInput) -> Self::Output {
        let lsm303agr = static_init_half!(
            static_buffer.2,
            Lsm303agrI2C<'static>,
            Lsm303agrI2C::new(
                static_buffer.0,
                static_buffer.1,
                &mut capsules::lsm303agr::BUFFER
            )
        );
        static_buffer.0.set_client(lsm303agr);
        static_buffer.1.set_client(lsm303agr);

        lsm303agr
    }
}
