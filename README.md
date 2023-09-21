# simple-windows-tun
Very simple windows tun, base on NetAdapterCx. This is used for [ForNet](https://github.com/ForNetCode/fornet).

## How to Use it
1. Download the driver files at [Release Page](https://github.com/ForNetCode/simple-windows-tun/releases)
2. write rust code like [example](https://github.com/ForNetCode/simple-windows-tun/blob/main/FunTunCli/src/examples/simple.rs), replace guid with your own defined and inf_path to the proper path.

## Support Platform
Windows Version: Newer than Windows 10, version 2004.(We have tested with Windows 11)

Architecture Platform:  x86_64.

We may support `aarch64` and Win 10 later. If you have any idea, feel free to open issue.



### How To Build It
Because of the EV certifacte, It may be very hard for developer to build the certificated driver. We just write this for a guide of our own to release the driver.
```powershell
# after build the driver with Visual Studio.
cp .\ForTun\fortun.ddf .\ForTun\x64\Release\ForTun\
cp .\ForTun\x64\Release\ForTun.pdb  .\ForTun\x64\Release\ForTun\ForTun.pdb
cd .\ForTun\x64\Realse\ForTun
MakeCab /f "fortun.ddf"
# sign dist1/fortun.cab with EV certifacter
# go to https://partner.microsoft.com/zh-cn/dashboard/hardware/Search to submit and signed with Windows.

# There's also a simple [tutorial]() to tell you how develop and debug Windows driver in Chiease.
```


## TODO
1. release rust client code to crate.io
2. support win10 and test.
3. add Git Action to build unsigned files.