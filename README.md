# DABShield Rust Driver

> [!WARNING]
> This has been currently abandoned<br>
> Nothing really works at the moment.

This is a rust driver for the [DABShield](https://www.dabshield.com/home) (based on the si468x chip). 

The low level code to drive the SI468x chip (the "peripheral access crate" or PAC) has been generated from  [ddgen](https://github.com/adoble/ddgen) version 0.2.1

# ToDos 

- [ ] Provide a way to genericaly handle the SI468x approach to errors. This consists of reading the response to a command and checking if the `err_cmd` flag is set. If it is, then send a `RD_REPLY`command and read the error code from its response. Maybe this needs to be supported in a generic way by `ddgen`.

- [ ] Use the [type state pattern](https://docs.rust-embedded.org/book/static-guarantees/typestate-programming.html) to ensure that only valid commands can be called (e.g. initialisation has been first performed). 

- [ ] Complete the functionality. Currently only the chip initialization has been partially completed. 

