#![no_std]

use heapless::String;

pub mod graphics;
pub mod implementations;

// 400x240 Aspect Ratio (Sharp Memory LCD 2.7")
pub const SCREEN_WIDTH: u32 = 200;
pub const SCREEN_HEIGHT: u32 = 200;

/// Represents a hardware input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonInput {
    Up,
    Down,
    Left,
    Right,
    Ok,
    Back,
    Num(u8),
}
pub struct PixPadApp {
    main_screen: MainScreen,
    info: PixClientInfo,
}

/// The current state of the app.
///
/// The application works like a simple state machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainScreen {
    InfoScreen,
    ReceiveScreen(ReceiveScreenState),
    TransferScreen(TransferScreenState),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceiveScreenState {
    SelectMethod { method: usize },
    ShowingQr,
}

impl Default for ReceiveScreenState {
    fn default() -> Self {
        ReceiveScreenState::SelectMethod { method: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransferScreenState {
    SelectMethod { method: usize },
    ScanningQr,
    InsertingPhoneNumber,
    InsertingCPF,
    Sending,
    Success,
}

impl Default for TransferScreenState {
    fn default() -> Self {
        TransferScreenState::SelectMethod { method: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PixClientInfo {
    name: String<32>,
}
