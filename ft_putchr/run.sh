#!/bin/zsh

dir="target/"
name="tmp.out"
output="$dir$name"

# remove tmp file containing all the shell cmd output
rm -f $output; 

# recreate tmp file
mkdir -p $dir
touch $output;

echo "----------------------------------------------------------------------------------------------------------------- cargo check ----------------------------------------------------------------------------------------------------------------- \n" &>> $output;

# compile src & dump output to tmp outputfile
cargo check &> $output;  

# append ascii divider symbol to tmp file
echo "----------------------------------------------------------------------------------------------------------------- cargo run ----------------------------------------------------------------------------------------------------------------- \n" &>> $output;

# exec bin & append execution output to tmp outputfile
cargo run &>> $output; 

# copy tmp outputfile contents to clipboard
head -n 10000 $output | pbcopy;

# print output file using
cat $output; # aliased (bat -- prettier package for printing output that's better than cat)