# PixPad
![Branding for PIX](https://thumb.wikimedia.org/wikipedia/commons/thumb/5/50/Pix_%28Brazil%29_logo.svg/500px-Pix_%28Brazil%29_logo.svg.png?utm_source=en.wikipedia.org&utm_campaign=parser&utm_content=thumbnail)

This repository contains firmware and software for "PixPad," a device for doing and receiving transations via [PIX](https://en.wikipedia.org/wiki/Pix_(payment_system)).

> [!WARNING]
> This repository is in early development... in fact, as of right now there is no firmware, only a simulator.

## Getting Started
Clone the repository. It's a workspace containing three crates: `core-ui`, `simulator` and `firmware`.

`core-ui` contains the real brain of the program, with all its UI and functionality. `firmware` and `simulator` interface the software with a real or virtual display, inputs and peripherals. `core-ui` and `firmware` are both `#![no_std]` crates, but `simulator` runs on a desktop.

From the workspace folder, you can run the simulator as such.

```bash
cargo run --package simulator
```

## Motivation
Seniors, children, low-income families and unhoused persons may struggle with the technical and economic barriers that come with PIX being available mainly through smartphone apps. "PixPad" aims to be cheap to manufacture and easy to use, not much different from a brick phone or a calculator, providing financia independence to those demographics.

With PixPad, anyone can open an account with any PIX or PixPad service provider, then set up the device with their informations and then perform transactions.

The device's design aims to be:
- Low on power consumption;
- Readable in outside situations as well as during the night;
- Safe and discreet;
- Small, but with chunky, highly usable buttons;
- Intuitive;
- (Most important) Affordable, under R$100.00 to manufacture, but the cheaper the better;

## Features
- [ ] Ability to check current balance;
- [ ] Ability to browse past transfers (in and out);
- [ ] Ability to transfer via PIX;
    - [ ] Ability to scan QR Codes with a builtin camera;
    - [ ] Ability to enter Phone, CPF and Random keys;
- [ ] Ability to receive transfers via PIX;
    - [ ] Receive notifications of transfers;
    - [ ] Manage and display keys;
- [ ] Standard security measures against physical and over-the-network hacking;
- [ ] PIN protection for transactions;

## Licence
The licence is the good old [GPL 3.0](https://www.gnu.org/licenses/gpl-3.0.en.html). The software/firmware and design of this device is free — that is, free of charge, free of proprietary ownership.

## Contributing
It's pretty early for contributions, but you can!