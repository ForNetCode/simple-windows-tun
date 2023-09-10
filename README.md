# simple-windows-tun
Very simple windows tun, base on NetAdapterCx.

This is used for [ForNet](https://github.com/ForNetCode/fornet), It now passed simple test case in Windows 11 X86 Virtual Machine. But It has lots of job to do.

- [ ] Get a company to buy code sign.
- [ ] Pass Windows Test.
- [ ] Release the signed driver files at GitHub Release.
- [ ] Release ForTunCli To crate.
## How to Use it
1. Download the signed driver from the release page.
2. run command `cargo add simple-windows-tun`

The Below is an example code:

```rust

```

## Support Platform
Windows Version: Newer than Windows 10, version 2004.  

Architecture Platform:  x86_64.

We may support `aarch64` later.



### how to build it
Because of the EV certifacte, It may be very hard for developer to build the certificated driver. We just write this for a guide of our own to release the driver.
```powershell
# after build the driver.
cp .\ForTun\fortun.ddf .\ForTun\x64\Release\ForTun\
cp .\ForTun\x64\Release\ForTun.pdb  .\ForTun\x64\Release\ForTun\ForTun.pdb
cd .\ForTun\x64\Realse\ForTun
MakeCab /f "fortun.ddf"
# sign dist1/fortun.cab with EV certifacter
# go to https://partner.microsoft.com/zh-cn/dashboard/hardware/Search to submit
```


