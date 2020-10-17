# stay-awake-rs
Keep a Windows machine awake

### Testing
In order to test, open PowerShell with elevated (admin) privleges. After executing the program, run the following.

```pwsh
powercfg -requests
```

### Win32 Docs
> https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setthreadexecutionstate

### Inspiration
Following along with Amos's ping series:
> https://fasterthanli.me/series/making-our-own-ping 