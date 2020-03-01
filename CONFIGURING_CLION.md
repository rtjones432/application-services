# Configuring Clion for Rust-only development
These are the steps to build the application-services desktop in clion:
This works on Ubuntu 18.04 and OSx10.0, but will probably work on anything.

First you will need to install the IntelliJ Rust plugin. https://plugins.jetbrains.com/plugin/8182-rust?_ga=2.3358832.1169745618.1582237573-208076843.1581265366

When opening the project, Clion will probably automatically try to run the .gradle file. By creating our own build configuration in Clion we can avoid this get a more flexibal work environment. To build the project with Clion we can to do the following:

### Create a new build configuration.
Use the configuration template "Cargo Command".
In the command field type test --all

### In the configuration you will need to set sevral environment variables:
> APPSERVICES_PLATFORM_DIR = "${application-services dirsctory}/libs/desktop/linux-x86-64"
> SQLCIPHER_LIB_DIR = "${APPSERVICES_PLATFORM_DIR}/sqlcipher/lib"
> SQLCIPHER_INCLUDE_DIR = "${APPSERVICES_PLATFORM_DIR}/sqlcipher/include"
> NSS_STATIC = "1"
> NSS_DIR = "${APPSERVICES_PLATFORM_DIR}/nss"

After running the script
> source ./libs/bootstrap-desktop.sh 

you can display the full path in the terminal using: echo ${variable}
Then just add each of these (without quotes) to your build configuration under "Environment Variables".
