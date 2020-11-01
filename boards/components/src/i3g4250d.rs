//! Components for the I3G4250D sensor.
//!
//! SPI Interface
//!
//! Usage
//! -----
//! ```rust
//! let i3g4250d = components::i3g4250d::I3g4250dSpiComponent::new().finalize(
//!     components::i3g4250d_spi_component_helper!(
//!         // spi type
//!         stm32f429zi::spi::Spi,
//!         // chip select
//!         stm32f429zi::gpio::PinId::PE03,
//!         // spi mux
//!         spi_mux
//!     )
//! );
//! ```
use capsules::i3g4250d::I3g4250dSpi;
use capsules::virtual_spi::VirtualSpiMasterDevice;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use kernel::component::Component;
use kernel::hil::spi;
use kernel::static_init_half;

// Setup static space for the objects.
#[macro_export]
macro_rules! i3g4250d_spi_component_helper {
    ($A:ty, $select: expr, $spi_mux: expr) => {{
        use capsules::i3g4250d::I3g4250dSpi;
        use capsules::virtual_spi::VirtualSpiMasterDevice;
        use core::mem::MaybeUninit;
        let mut i3g4250d_spi: &'static capsules::virtual_spi::VirtualSpiMasterDevice<'static, $A> =
            components::spi::SpiComponent::new($spi_mux, $select)
                .finalize(components::spi_component_helper!($A));
        static mut i3g4250dspi: MaybeUninit<I3g4250dSpi<'static>> = MaybeUninit::uninit();
        (&mut i3g4250d_spi, &mut i3g4250dspi)
    };};
}

pub struct I3g4250dSpiComponent<S: 'static + spi::SpiMaster> {
    _select: PhantomData<S>,
}

impl<S: 'static + spi::SpiMaster> I3g4250dSpiComponent<S> {
    pub fn new() -> I3g4250dSpiComponent<S> {
        I3g4250dSpiComponent {
            _select: PhantomData,
        }
    }
}

impl<S: 'static + spi::SpiMaster> Component for I3g4250dSpiComponent<S> {
    type StaticInput = (
        &'static VirtualSpiMasterDevice<'static, S>,
        &'static mut MaybeUninit<I3g4250dSpi<'static>>,
    );
    type Output = &'static I3g4250dSpi<'static>;

    unsafe fn finalize(self, static_buffer: Self::StaticInput) -> Self::Output {
        let i3g4250d = static_init_half!(
            static_buffer.1,
            I3g4250dSpi<'static>,
            I3g4250dSpi::new(
                static_buffer.0,
                &mut capsules::i3g4250d::TXBUFFER,
                &mut capsules::i3g4250d::RXBUFFER
            )
        );
        static_buffer.0.set_client(i3g4250d);
        i3g4250d.configure();

        i3g4250d
    }
}
