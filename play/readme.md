

create a powerful stack tracing macro that will print stack trace data using the dbg! macro and whatever necessary to print the name of the function, the function parameters, the ouput of the function
as well as the meta data of duration of execution, time started and tiem ended, memory used, result's pointer address. it should also collect trace logs in target/logs/traces.log which will accept also outputs from println! or eprintln!


// usage
#[trace_and_log] 
fn a()
{

}

#[trace_and_log]
fn sum(a: u8, b: u8) -> u8
{
    a + b
}

#[trace_and_log]
fn main()
{
    a();
    b();
}

expected output

fn sum()
params: a: u8, b: u8
output: ()
exec_start_time: DD-MM-YYYY HH:MM:SS
exec_end_time: DD-MM-YYYY HH:MM:SS
exec_duration: Xms
mem usage: xxx
output ptr: xxx
ERROR_LOGS: 
WARN_LOGS: 
INFO_LOGS: 

fn a()
params: none
output: ()
exec_start_time: DD-MM-YYYY HH:MM:SS
exec_end_time: DD-MM-YYYY HH:MM:SS
exec_duration: Xms
mem usage: xxx
output ptr: xxx
ERROR_LOGS: 
WARN_LOGS: 
INFO_LOGS: 

fn main()
params: none
output: ()
exec_start_time: DD-MM-YYYY HH:MM:SS
exec_end_time: DD-MM-YYYY HH:MM:SS
exec_duration: Xms
mem usage: xxx
output ptr: xxx
ERROR_LOGS: 
WARN_LOGS: 
INFO_LOGS: 