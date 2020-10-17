# stay-awake-rs
Keep a Windows machine awake

![stay-awake workflow](https://github.com/curtisalexander/stay-awake-rs/workflows/stay%20awake/badge.svg)

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
Created another version in `C#`
> https://github.com/curtisalexander/stay-awake-cs