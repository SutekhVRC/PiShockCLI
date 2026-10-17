use clap::{Parser, Subcommand};
use pishock_rawrs::handle::serial::{
    PiShockSerialHandle,
    commands::{PiShockCommand, PiShockOperationParams},
};

#[derive(Debug, Subcommand)]
enum CliCommand {
    Beep {
        id: u32,
        duration: u32,
    },
    Vibrate {
        id: u32,
        duration: u32,
        intensity: u8,
    },
    Shock {
        id: u32,
        duration: u32,
        intensity: u8,
    },
    End {
        id: u32,
    },
    Info,
    Restart,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct CliArgs {
    #[command(subcommand)]
    command: CliCommand,

    #[arg(short, long)]
    serial_device: String,
}

fn do_beep(pishock_handle: &mut PiShockSerialHandle, id: u32, duration: u32) {
    let op_params = PiShockOperationParams::Beep { id, duration };
    let _ = pishock_handle
        .send_command(PiShockCommand::Operate { op: op_params })
        .unwrap();
}

fn do_vibrate(pishock_handle: &mut PiShockSerialHandle, id: u32, duration: u32, intensity: u8) {
    let op_params = PiShockOperationParams::Vibrate {
        id,
        duration,
        intensity,
    };
    let _ = pishock_handle
        .send_command(PiShockCommand::Operate { op: op_params })
        .unwrap();
}

fn do_shock(pishock_handle: &mut PiShockSerialHandle, id: u32, duration: u32, intensity: u8) {
    let op_params = PiShockOperationParams::Shock {
        id,
        duration,
        intensity,
    };
    let _ = pishock_handle
        .send_command(PiShockCommand::Operate { op: op_params })
        .unwrap();
}

fn do_end(pishock_handle: &mut PiShockSerialHandle, id: u32) {
    let op_params = PiShockOperationParams::End { id };
    let _ = pishock_handle
        .send_command(PiShockCommand::Operate { op: op_params })
        .unwrap();
}

fn do_info(pishock_handle: &mut PiShockSerialHandle) {
    let res_data = pishock_handle.send_command(PiShockCommand::Info).unwrap();
    println!("{}", res_data);
}

fn do_restart(pishock_handle: &mut PiShockSerialHandle) {
    let _ = pishock_handle.send_command(PiShockCommand::Info).unwrap();
}

fn dispatch_op(pishock_handle: &mut PiShockSerialHandle, op: CliCommand) {
    match op {
        CliCommand::Beep { id, duration } => do_beep(pishock_handle, id, duration),
        CliCommand::Vibrate {
            id,
            duration,
            intensity,
        } => do_vibrate(pishock_handle, id, duration, intensity),
        CliCommand::Shock {
            id,
            duration,
            intensity,
        } => do_shock(pishock_handle, id, duration, intensity),
        CliCommand::End { id } => do_end(pishock_handle, id),
        CliCommand::Info => do_info(pishock_handle),
        CliCommand::Restart => do_restart(pishock_handle),
    }
}

fn main() {
    let args = CliArgs::parse();

    let mut pishock_handle = PiShockSerialHandle::new(&args.serial_device).unwrap();

    dispatch_op(&mut pishock_handle, args.command);
}
