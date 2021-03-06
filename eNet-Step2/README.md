# eNet-Step2

#### Development

1. Desription

    This project contains 4 packages and 2 configuration files.

    - 2 libraries
        - Secure-sign Library

            This library provides API to use Falcon Cryptographic signature.

        - Node-network

            This library provides API to configure a "Node" in local, connect peers, communicate data each other with peers.

    - 2 test binary
        - test-network

            This binary is used to test Node-network library.

        - test-ui

            This binary is used to configure a Node with 3 hosts in local machine and test communication between each peers.

    - 2 configuration files

        This files are placed in folder "config".
        
        - conf.ini

            This file contains information about hosts to configure Node in local.
        - peerlist.csv

            This file contains information about peers for each host to configure in local.    
 
1. Build
    - On Window:
        - Install OpenSSL via https://slproweb.com/products/Win32OpenSSL.html.

        - Check and Modify OpenSSL install path in "suecure-sign/build.rs".

            ```
            .include("$INSTALL_PATH/include")
            ```
            ```
            println!("cargo:rustc-link-search=static=$INSTALL_PATH/
            lib");
            ```
            
        - Run "cargo build" in console.

2. Run
    - On Window:
        - Run "test-network" binary.
            > Run "cargo run -p test-network" in console.
        - Run "test-ui" binary.
            > Run "cargo run -p test-ui" in console.