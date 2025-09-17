use super::{AdcChannels, AdcCommand, AdcIntStatus, AdcResult, Gpip, RegisterBlock};
use crate::hbn;
use core::{
    future::Future,
    ops::Deref,
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
    task::{Context, Poll},
};

/// Error type for ADC operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AdcError {
    /// Conversion timeout
    Timeout,
    /// Hardware error
    HardwareError,
}

/// Managed async/await ADC peripheral.
pub struct AsyncAdc<'a, G: Deref<Target = RegisterBlock>> {
    gpip: &'a mut Gpip<G>,
    state: &'a AdcState,
}

impl<'a, G: Deref<Target = RegisterBlock>> AsyncAdc<'a, G> {
    /// Creates the async/await ADC peripheral from owned GPIP structure and state.
    #[inline]
    pub fn new(gpip: &'a mut Gpip<G>, state: &'a AdcState) -> Self {
        // Store reference to GPIP register block for interrupt handling
        state
            .ref_to_gpip
            .store(&**gpip as *const _ as usize, Ordering::Release);

        AsyncAdc { gpip, state }
    }

    /// Start ADC conversion with async/await support.
    pub async fn convert(
        &mut self,
        channels: &[AdcChannels],
        hbn: &hbn::RegisterBlock,
        results: &mut [AdcResult],
    ) -> Result<usize, AdcError> {
        if channels.is_empty() || results.is_empty() {
            return Ok(0);
        }

        // Clear FIFO first
        self.gpip
            .adc_feature_control(AdcCommand::ClearFifo, false, hbn);

        // Configure channels
        self.gpip.adc_channel_config(channels, hbn);

        // Enable ADC interrupt
        self.gpip.adc_rxint_mask(false);

        // Start conversion
        self.gpip.adc_start_conversion(hbn);

        // Wait for conversion completion using true interrupt-driven async
        WaitForAdcReady::new(&**self.gpip, &self.state.adc_ready).await;

        // Read results
        let count = core::cmp::min(self.gpip.adc_get_complete_num() as usize, results.len());

        // If count is 0, conversion didn't complete normally, try polling mode as fallback
        let final_count = if count == 0 {
            // Fallback plan: short-term polling wait
            for _ in 0..10000 {
                let current_count = self.gpip.adc_get_complete_num() as usize;
                if current_count > 0 {
                    break;
                }
                core::hint::spin_loop();
            }
            core::cmp::min(self.gpip.adc_get_complete_num() as usize, results.len())
        } else {
            count
        };

        let mut actual_count = 0;

        for _ in 0..final_count {
            let raw_data = self.gpip.adc_get_raw_data();

            // Use the proper parsing method from GPIP
            let mut temp_results = [AdcResult {
                pos_chan: channels.get(0).map(|c| c.pos_ch),
                neg_chan: channels.get(0).map(|c| c.neg_ch),
                value: 0,
                millivolt: 0,
            }; 1];

            // Parse the result properly
            self.gpip
                .adc_parse_result(&[raw_data], &mut temp_results, hbn);

            results[actual_count] = temp_results[0];
            actual_count += 1;
        }

        // Clear interrupt
        let clear_flags = AdcIntStatus {
            adc_ready: true,
            fifo_underrun: false,
            fifo_overrun: false,
            neg_saturation: false,
            pos_saturation: false,
        };
        self.gpip.adc_int_clear(clear_flags, hbn);

        // Stop conversion and mask interrupt
        self.gpip.adc_stop_conversion(hbn);
        self.gpip.adc_rxint_mask(true);

        Ok(actual_count)
    }
}

/// Set of wakers as the state for an async/await ADC peripheral.
#[derive(Debug)]
pub struct AdcState {
    adc_ready: atomic_waker::AtomicWaker,
    ref_to_gpip: AtomicUsize,
}

impl AdcState {
    /// Creates the set of wakers for an ADC peripheral.
    #[inline]
    pub const fn new() -> AdcState {
        AdcState {
            adc_ready: atomic_waker::AtomicWaker::new(),
            ref_to_gpip: AtomicUsize::new(0),
        }
    }

    /// Use this waker set to handle ADC interrupt.
    #[inline]
    pub fn on_interrupt(&self) {
        let ptr = self.ref_to_gpip.load(Ordering::Acquire);
        if ptr == 0 {
            // Pointer is invalid; do not attempt to dereference.
            return;
        }
        let gpip = unsafe { &*(ptr as *const RegisterBlock) };

        // Check interrupt status
        let int_ready = gpip.gpadc_config.read().is_adc_ready();

        // Wake up waiting tasks if ADC is ready
        if int_ready {
            self.adc_ready.wake();
        }
    }
}

/// Future that waits for ADC ready interrupt.
struct WaitForAdcReady<'r> {
    gpip: &'r RegisterBlock,
    registry: &'r atomic_waker::AtomicWaker,
}

impl<'r> WaitForAdcReady<'r> {
    #[inline]
    pub const fn new(gpip: &'r RegisterBlock, registry: &'r atomic_waker::AtomicWaker) -> Self {
        Self { gpip, registry }
    }
}

impl Future for WaitForAdcReady<'_> {
    type Output = ();

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // First check if ADC has already completed
        let int_ready = self.gpip.gpadc_config.read().is_adc_ready();

        if int_ready {
            // ADC has completed, clear interrupt flag and return Ready
            Poll::Ready(())
        } else {
            // ADC not completed, register waker to wait for interrupt wake-up
            self.registry.register(cx.waker());
            Poll::Pending
        }
    }
}
