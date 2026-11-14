### Good practice / Performance tips

Most of the time, you don't want all information provided by `sysinfo` but just a subset of it.
In this case, it's recommended to use `refresh_specifics(...)` methods with only what you need
to have much better performance.

Another issue frequently encountered: unless you know what you're doing, it's almost all the
time better to instantiate the `System` struct once and use this one instance through your
program. The reason is because a lot of information needs a previous measure to be computed
(the CPU usage for example). Another example why it's much better: in case you want to list
all running processes, `sysinfo` needs to allocate all memory for the `Process` struct list,
which takes extra time on the first run.
