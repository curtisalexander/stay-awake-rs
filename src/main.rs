use std::ffi::c_void;
use std::fmt;
use std::mem::transmute;

#[macro_use]
extern crate bitflags;

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
        let prev_execution_state = (self.ste)(next_thread_exec_state);
        println!(
            "Reset thread execution state:\n    From ==> {}\n      To ==> {}",
            prev_execution_state, next_thread_exec_state
        );
    }
}

fn main() {
    // init
    let sa = StayAwake::new();
    let ste = sa.ste;

    // state to set
    let next_thread_exec_state =
        ExecutionState::ES_CONTINUOUS | ExecutionState::ES_DISPLAY_REQUIRED;

    // set thread execution state
    let prev_thread_exec_state: ExecutionState = ste(next_thread_exec_state);
    println!(
        "Set thread execution state:\n    From ==> {}\n      To ==> {}",
        prev_thread_exec_state, next_thread_exec_state
    );

    // After exiting main, StayAwake instance is dropped and the thread execution
    // state is reset to ES_CONTINUOUS
}
