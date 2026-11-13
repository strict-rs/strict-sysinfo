### Linux on Docker & Windows Subsystem for Linux (WSL)

Virtual Linux systems, such as those run through Docker and Windows Subsystem for Linux (WSL), do
not receive host hardware information via `/sys/class/hwmon` or `/sys/class/thermal`. As such,
querying for components may return no results (or unexpected results) when using this library on
virtual systems.
