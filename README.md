# Unacer

an alternative i made for acersense on the acer aspire 14 ai 52mt

i reverse engineered acersense. this just flips the performance mode (silent, normal, performance) by talking to the laptop's embedded controller over hid.

performance modes are just the fan curves i think

acer made it so only acersense could set these fan curves using its weird protocol

i just reverse engineered it and put it into a linux cli. it can also read the cpu fan tachometer

acersense didnt even have the fan rpm thing

the fan tachometer thing i got from digging in the EC code from a bios update zip (top 10 things i would rather not do again)

acersense is windows only. this ones for linux (idk it might work on windows)

## ONLY USE THIS ON AN ACER ASPIRE 14 AI 52MT

IDK WHAT HAPPENS FOR OTHER LAPTOPS

but if u do try it out on another laptop tell me what happens i lwk wanna know

## usage

```
cargo build --release
./target/release/unacer mode silent
./target/release/unacer mode normal
./target/release/unacer mode performance
./target/release/unacer get-rpm
```

`get-rpm` prints the cpu fan speed in rpm (just the number, nothing else).

you need permission to open the hid device (`1025:174b`). sudo works. or put this in `/etc/udev/rules.d/99-acer-ec.rules` so anyone in `wheel` can:

```
KERNEL=="hidraw*", SUBSYSTEM=="hidraw", KERNELS=="*1025174B:*", GROUP="wheel", MODE="0660"
```

then reload udev (`sudo udevadm control --reload-rules && sudo udevadm trigger`).

if the ec doesn't acknowledge, it prints the response and exits. an ack doesn't always mean the mode actually changed (idk tho)
