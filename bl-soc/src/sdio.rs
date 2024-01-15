//! Secure Digital Input/Output peripheral.

use volatile_register::RW;

/// Secure Digital Input/Output peripheral registers.
#[repr(C)]
pub struct RegisterBlock {
    // todo fields
    /// 32-bit Block Count / (SDMA System Address) Register.
    pub system_addres: RW<SystemAddress>,
    /// Configuration register for number of bytes in a data block.
    pub block_size: RW<BlockSize>,
    /// Configuration register for number of data blocks.
    pub block_count: RW<BlockCount>,
    /// Register that contains the SD Command Argument.
    pub argument: RW<Argument>,
    /// Control register for the operation of data transfers.
    pub transfer_mode: RW<TransferMode>,
    /// Command register.
    pub command: RW<Command>,
    /// Register that stores responses from SD cards.
    pub response: RW<Response>,
    /// 32-bit data port register to accesses internal buffer.
    pub buffer_data_port: RW<BufferDataPort>,
    /// 32-bit read only register to get status of the Host Controller.
    pub present_state: RW<PresentState>,
    /// Host Control 1 Register.
    pub host_control_1: RW<HostControl1>,
    /// Power Control Register.
    pub powercontrol: RW<PowerControl>,
    /// Block Gap Control Register.
    pub block_gap: RW<BlockGap>,
    /// Register which is mandatory for the Host Controller.
    pub wakeup_control: RW<WakeupControl>,
    /// Control register for SDCLK in SD Mode and RCLK in UHS-II Mode.
    pub clock_control: RW<ClockControl>,
    /// Timeout Control Register.
    pub timeout_control: RW<TimeoutControl>,
    /// Writting 1 to each bit of this register to generate a reset pulse.
    pub software_reset: RW<SoftwareReset>,
    /// The reads of register are affected by the Normal Interrupt Status Enable.  
    pub normal_interrupt_status: RW<NormalInterruptStatus>,
    /// Register that enables the defined signals.
    pub error_interrupt_status: RW<ErrorInterruptStatus>,
    /// Register that sets to 1 enables Interrupt Status.
    pub normal_interrupt_status_enable: RW<NormalInterruptStatusEnable>,
    /// Register that sets to 1 enables Interrupt Status.
    pub error_interrupt_status_enable: RW<ErrorInterruptStatusEnable>,
    /// Register that selects which interrupt status is indicated to the Host System as the interrupt.
    pub normal_interrupt_signal_enable: RW<NormalInterruptSignalEnable>,
    /// Register that selects which interrupt status is notified to the Host System as the interrupt.
    pub error_interrupt_signal_enable: RW<ErrorInterruptSignalEnable>,
    /// Register that indicates CMD12 response error of Auto CMD12 and CMD23 response error of Auto CMD23.
    pub auto_cmd_error_status: RW<AutoCMDErrorStatus>,
    /// Host Control 2 Register.
    pub host_control_2: RW<HostControl2>,
    /// Register that provides the Host Driver with information specific to the Host Controller implementation.
    pub capabilities: RW<Capabilities>,
    /// Registers that indicates maximum current capability fo each voltage.
    pub max_current_capabilities: RW<MaxCurrentCapabilities>,
    /// Register that simplifies test of the Auto CMD Error Status register.
    pub force_event_auto_cmd_error_status: RW<ForceEventAutoCMDErrorStatus>,
    /// Register that simplifies test of the Error Interrupt Status register.
    pub force_event_error_interrupt_status: RW<ForceEventErrorInterruptStatus>,
    /// Register that holds the ADMA state when ADMA Error Interrupt is occurred.
    pub adma_error_status: RW<ADMAErrorStatus>,
    /// Register that contains the physical Descriptor address used for ADMA data transfer.
    pub adma_system_address: RW<ADMASystemAddress>,
    /// Perest Value Registers.
    pub preset_value: RW<PresetValue>,
    _reserved0: [u8; 8],
    /// ADMA3 Intergrated Descriptor Address Register.
    pub adma3_integrated_descriptor_address: RW<ADMA3IntegratedDescriptorAddress>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SystemAddress(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BlockSize(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BlockCount(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Argument(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransferMode(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Command(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Response(u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BufferDataPort(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PresentState(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HostControl1(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PowerControl(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BlockGap(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct WakeupControl(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClockControl(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TimeoutControl(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SoftwareReset(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NormalInterruptStatus(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ErrorInterruptStatus(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NormalInterruptStatusEnable(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ErrorInterruptStatusEnable(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NormalInterruptSignalEnable(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ErrorInterruptSignalEnable(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AutoCMDErrorStatus(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HostControl2(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Capabilities(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MaxCurrentCapabilities(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ForceEventAutoCMDErrorStatus(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ForceEventErrorInterruptStatus(u16);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ADMAErrorStatus(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ADMASystemAddress(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PresetValue(u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ADMA3IntegratedDescriptorAddress(u64);

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use memoffset::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, system_addres), 0x0);
        assert_eq!(offset_of!(RegisterBlock, block_size), 0x4);
        assert_eq!(offset_of!(RegisterBlock, block_count), 0x06);
        assert_eq!(offset_of!(RegisterBlock, argument), 0x08);
        assert_eq!(offset_of!(RegisterBlock, transfer_mode), 0x0c);
        assert_eq!(offset_of!(RegisterBlock, command), 0x0e);
        assert_eq!(offset_of!(RegisterBlock, response), 0x10);
        assert_eq!(offset_of!(RegisterBlock, buffer_data_port), 0x20);
        assert_eq!(offset_of!(RegisterBlock, present_state), 0x24);
        assert_eq!(offset_of!(RegisterBlock, host_control_1), 0x28);
        assert_eq!(offset_of!(RegisterBlock, powercontrol), 0x29);
        assert_eq!(offset_of!(RegisterBlock, block_gap), 0x2a);
        assert_eq!(offset_of!(RegisterBlock, wakeup_control), 0x2b);
        assert_eq!(offset_of!(RegisterBlock, clock_control), 0x2c);
        assert_eq!(offset_of!(RegisterBlock, timeout_control), 0x2e);
        assert_eq!(offset_of!(RegisterBlock, software_reset), 0x2f);
        assert_eq!(offset_of!(RegisterBlock, normal_interrupt_status), 0x30);
        assert_eq!(offset_of!(RegisterBlock, error_interrupt_status), 0x32);
        assert_eq!(
            offset_of!(RegisterBlock, normal_interrupt_status_enable),
            0x34
        );
        assert_eq!(
            offset_of!(RegisterBlock, error_interrupt_status_enable),
            0x36
        );
        assert_eq!(
            offset_of!(RegisterBlock, normal_interrupt_signal_enable),
            0x38
        );
        assert_eq!(
            offset_of!(RegisterBlock, error_interrupt_signal_enable),
            0x3a
        );
        assert_eq!(offset_of!(RegisterBlock, auto_cmd_error_status), 0x3c);
        assert_eq!(offset_of!(RegisterBlock, host_control_2), 0x3e);
        assert_eq!(offset_of!(RegisterBlock, capabilities), 0x40);
        assert_eq!(offset_of!(RegisterBlock, max_current_capabilities), 0x48);
        assert_eq!(
            offset_of!(RegisterBlock, force_event_auto_cmd_error_status),
            0x50
        );
        assert_eq!(
            offset_of!(RegisterBlock, force_event_error_interrupt_status),
            0x52
        );
        assert_eq!(offset_of!(RegisterBlock, adma_error_status), 0x54);
        assert_eq!(offset_of!(RegisterBlock, adma_system_address), 0x58);
        assert_eq!(offset_of!(RegisterBlock, preset_value), 0x60);
        assert_eq!(
            offset_of!(RegisterBlock, adma3_integrated_descriptor_address),
            0x78
        );
    }
}
