# hwisak-rs

**Abbreviation Expansion:** Hardware Information Swiss Army Knife for Rust. 

_Mistaken Translation:_ Whispering (Korean)

A library that contains detailed information about your computer parts. 
This library can help you with your Rust projects. Intel and AMD CPU Data is 
taken from the Git repositories (under the folder res/cpu) as submodules and turned into SQLite Databases which include 
basically all the information in the ark.intel.com website. 

### Motivation
This is inspired by the [hwinfo](https://github.com/lfreist/hwinfo) C++ library, however I did not know how to use C++ 
code in Rust, so I decided to make my own library but more "jam-packed".

## Example

## How to use
This project is both a library and a binary. The library can be used by other Rust projects, while the binary can be 
run. 

> [!NOTE]
> It is yet to be published to crates.io. If you want to use this library, good luck (I don't know how to). 
> This information is just for when I do publish it. 

To use as a library, add it as a dependency. 

```toml
[dependencies]
hwisak-rs = "*"
```
or
```shell
cargo add hwisak-rs
```
[//]: # (I could not be fucked to update the snippet lol. also hi if you see this!)

The binary only just contains the information that you could fetch as structs. It is not formatted, just enough 
for you to feast your eyes upon. 

To use the binary, run the cargo install command. 
```shell
cargo install hwisak-rs
```

## Documentation
I need to make documentation before publishing, so expect it to be on the way. 

## Contributions
Anybody is free to contribute and is appreciated to fixing the spaghetti shitshow of code in this repo. 

To play with the library, here is how to run it. 
```shell
git clone https://github.com/tk-rs/hwisak-rs
cd hwisak-rs
git submodule update --init --recursive

cargo run # If you want to use the binary
cargo build # To use the lib or binary

cargo test # Not implemented yet, might do soon
```

## Licence
I took the dependencies as this library is just a compilation of all the popular system information crates. Therefore, 
it wouldn't be nice to impose a license

License is the "Do whatever the fuck you want" license (which is what the name suggests), however it is always nice
to contribute or sponsor the project if you are a massive company.

## Appreciations

I would like to thank the following crates for providing the information: 

- wgpu
- sysinfo (if you read this, my lib depends on yours. keep working on it plz)
- os_info
- anything else on my Cargo.toml file that I didn't mention

I would also like to thank the repository for providing me the goldmine worth of intel cpu information to 
make the database: 

- intel-processors: https://github.com/toUpperCase78/intel-processors (check it out if you have the time)
- amd-processors: https://github.com/tk-rs/AMDCpuData (my repo lol)

