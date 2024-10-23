

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

[log-level][timestamp][duration]
function_name(param_name param_type(param_value), ...) -> return_name(return_type(return_value@return_mem_ptr&return_value_mem_byte_size)) {
        // add some space & all should look like the function here

        [log-level][timestamp][duration]
        function_name(param_name param_type(param_value), ...) -> return_name(return_type(return_value@return_mem_ptr&return_value_mem_byte_size)) {
                // add some space & all should look like the function here

                
                [log-level][timestamp][duration]
                function_name(param_name param_type(param_value), ...) -> return_name(return_type(return_value@return_mem_ptr&return_value_mem_byte_size)) {
                        // add some space & all should look like the function here

                
                        // add some space & all should look like the function here


                }
                // add some space & all should look like the function here


        }


        // add some space & all should look like the function here


}

fulfillment

i am yet to find out what it is, what it means and its meaning in my life.

Pointless days, i have sought employment, traded hours for wages and at times traded nothing for welfare support.

Either way, they were all pointless days, why? i did not grow in character, spirit and understanding of the mysteries that i have noticed thus far a person my age.

Why is that important? From my perspective, life looks like a messy puzzle and i feel compelled to gather and arrange the pieces as the days go by. It is a way for me to look forward to the days far ahead if the lord wills it to be. Admittedly, i hate being unaware and uncertain deep down i admire the order around me and want to do as much as possible in my own life to bring about that vague small ideal i have dubbed `fulfillment`.

