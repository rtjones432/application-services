# Configuring Clion for Rust-only development
These are the steps required to build the application-services Rust Components in  Jetbrains Clion:
This works on Ubuntu 18.04 and OS Mojave 10.14.6, but will probably work on anything.

First you will need to install the [IntelliJ Rust plugin.](https://plugins.jetbrains.com/plugin/8182-rust?_ga=2.3358832.1169745618.1582237573-208076843.1581265366)

#### Update the Cardo Command template with the required environment variables:
> APPSERVICES_PLATFORM_DIR = "/path/to/application-services/libs/desktop/{darwin|linux-x86-64}"  
> SQLCIPHER_LIB_DIR = "/path/to/application-services/libs/desktop/{darwin|linux-x86-64}/sqlcipher/lib"  
> SQLCIPHER_INCLUDE_DIR = "/path/to/application-services/libs/desktop/{darwin|linux-x86-64}/sqlcipher/include"  
> NSS_STATIC = "1"  
> NSS_DIR = "$"/path/to/application-services/libs/desktop/{darwin|linux-x86-64}/nss"  

#### Now create a build configuration using the Cargo Command template. For example, by using `test --all` as the command.


