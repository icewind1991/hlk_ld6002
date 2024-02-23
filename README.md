# HLK-LD6002

A library for community with [HLK-LD6002](https://www.hlktech.net/index.php?id=1180) radar respiratory and heartbeat sensors.

Supports both sync and async serial ports using `embedded-io` or `embedded-io-async`.

## A note about serial adapters.

The sensor use 1.382.400 baud UART for communicating, not all serial adapters support baud rates this high.
Using an adapter that doesn't support this baud rate (like the common CP210 based adapters) can lead to silent failures.