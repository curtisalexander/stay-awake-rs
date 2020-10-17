# stay-awake-rs
Keep a Windows machine awake

![stay-awake workflow](https://github.com/curtisalexander/stay-awake-rs/workflows/stay%20awake/badge.svg)

### Get
Executable binaries for Windows may be found at the [Release](https://github.com/curtisalexander/stay-awake-rs/releases) page.

### Usage

The executable `stay-awake.exe` is intended to be run in a terminal in order to keep one's Windows machine awake.

There are two modes one may choose from:
- **System** [Default] &rarr; the machine will not go to sleep but the display could still turn off
- **Display** &rarr; the machine will not go to sleep and the display will remain on

The simplest use case is to run the executable without any switches.

```pwsh
stay-awake.exe
```

This will prevent the machine from going to sleep and will await ``Enter`` to be pressed within the terminal before resetting the machine state.

#### Details

```
stay-awake 0.2.0
stay awake ==> keep a Windows machine awake

USAGE:
    stay-awake.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --awake-mode <awake-mode>    Awake mode [default: System]  [possible values: Display, System]
```

### Testing
In order to test, open PowerShell with elevated (admin) privileges. After executing the program, run the following.

```pwsh
powercfg -requests
```

### Win32 Docs
> https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setthreadexecutionstate

### Inspiration
Following along with Amos's ping series:
> https://fasterthanli.me/series/making-our-own-ping 

### See Also
Created an earlier version in `C#`
> https://github.com/curtisalexander/stay-awake-cs