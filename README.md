# EthCC-Workshop

## Getting Started

1. Download [the latest `forc` binaries for your system](https://github.com/FuelLabs/sway/releases/latest)

    and extract them:

    ```bash
    $ tar -xvf <filename>
    ```

2. Add the binaries to your `PATH`:

    ```bash
    $ export PATH=$PATH:$(pwd)/forc-binaries/
    ```

3. Check for correct setup:

    ```bash
    $ forc --version
    forc 0.18.1
    ```

    *if that doesn't work*

    ![open system preferences](images/system_preferences.png)
    ![click allow](images/allow_forc.png)

4. Download [the latest `fuel-core` binaries for your system](https://github.com/FuelLabs/fuel-core/releases/latest)

    and extract them:

    ```bash
    $ tar -xvf <filename>
    ```

5. Set up your environment variables:

    ```bash
    $ export FUEL_CORE_BIN=<binary dir>/fuel-core
    ```

    For me this looked like: "export FUEL_CORE_BIN=fuel-core-0.9.6-x86_64-apple-darwin/fuel-core"