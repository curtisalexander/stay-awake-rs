use colored::Colorize;
use std::error::Error;
use std::ffi::c_void;
use std::fmt;
use std::io::{self, Write};
use std::mem::transmute;
use structopt::clap::arg_enum;
use structopt::StructOpt;

#[macro_use]
extern crate bitflags;

#[derive(StructOpt, Debug)]
#[structopt(about = "keep a Windows machine awake")]
pub struct Args {
    /// Awake mode
    #[structopt(long, short, default_value = "System", possible_values = &AwakeMode::variants(), case_insensitive = true)]
    pub awake_mode: AwakeMode,
}

arg_enum! {
    #[derive(Debug)]
    pub enum AwakeMode {
        // Display ==>  prevent the machine from going to sleep AND keep the display on
        Display,
        // System ==> prevent the machine from going to sleep
        System
    }
}

// type aliases
type HModule = *const c_void;
type FarProc = *const c_void;
type SetThreadExecutionState = extern "stdcall" fn(esFlags: ExecutionState) -> ExecutionState;

// ExecutionState
bitflags! {
    pub struct ExecutionState: u32 {
    // Away mode should be used only by media-recording and media-distribution applications that must perform critical background processing on desktop computers while the computer appears to be sleeping.
    // const ES_AWAYMODE_REQUIRED = 0x00000040;
    // Informs the system that the state being set should remain in effect until the next call that uses ES_CONTINUOUS and one of the other state flags is cleared.
    const ES_CONTINUOUS = 0x80000000;
    // Forces the display to be on by resetting the display idle timer.
    const ES_DISPLAY_REQUIRED = 0x00000002;
    // Forces the system to be in the working state by resetting the system idle timer.
    const ES_SYSTEM_REQUIRED = 0x00000001;
    // This value is not supported. If ES_USER_PRESENT is combined with other esFlags values, the call will fail and none of the specified states will be set.
    // const ES_USER_PRESENT = 0x00000004;
    }
}

// Custom display
impl fmt::Display for ExecutionState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?} ({:#X})", &self, &self)
    }
}

// External functions
extern "stdcall" {
    fn LoadLibraryA(name: *const u8) -> HModule;
    fn GetProcAddress(module: HModule, name: *const u8) -> FarProc;
}

// Why wrap everything in a struct?
// Because I would like to ensure I always reset the thread execution state to ES_CONTINUOUS
//   and I ensure this via the Drop trait
struct StayAwake {
    ste: SetThreadExecutionState,
}

impl StayAwake {
    fn new() -> Self {
        // Get a handle - returns a void * (i.e. a memory address)
        let h = unsafe { LoadLibraryA("KERNEL32.dll\0".as_ptr()) };

        // Get the function SetThreadExecutionState
        let ste = unsafe { transmute(GetProcAddress(h, "SetThreadExecutionState\0".as_ptr())) };

        Self { ste }
    }
}

impl Drop for StayAwake {
    fn drop(&mut self) {
        let next_thread_exec_state = ExecutionState::ES_CONTINUOUS;
        let prev_thread_exec_state = (self.ste)(next_thread_exec_state);
        println!(
            "\nReset thread execution state:\n    {} ==> {}\n      {} ==> {}",
            String::from("From").red(),
            prev_thread_exec_state,
            String::from("To").blue(),
            next_thread_exec_state);
    }
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    // args
    let req_thread_exec_state = match &args.awake_mode {
        AwakeMode::Display => {
            println!("Running in ``{}`` mode ==> the machine will not go to sleep and the display will remain on", String::from("Display").green());
            ExecutionState::ES_DISPLAY_REQUIRED
        },
        AwakeMode::System => {
            println!("Running in ``{}`` mode ==> the machine will not go to sleep", String::from("System").green());
            ExecutionState::ES_SYSTEM_REQUIRED 
        }
    };

    // init
    let sa = StayAwake::new();
    let ste = sa.ste;

    // state to set - must combine ES_CONTINUOUS with another state
    let next_thread_exec_state =
        ExecutionState::ES_CONTINUOUS | req_thread_exec_state;

    // set thread execution state
    let prev_thread_exec_state: ExecutionState = ste(next_thread_exec_state);

    // print
    println!(
        "\nSet thread execution state:\n    {} ==> {}\n      {} ==> {}",
        String::from("From").purple(),
        prev_thread_exec_state,
        String::from("To").cyan(),
        next_thread_exec_state
    );

    print!("\nPress ``{}`` key to reset ", String::from("Enter").yellow());
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer)?;

    // After exiting main, StayAwake instance is dropped and the thread execution
    // state is reset to ES_CONTINUOUS
    Ok(())
}
