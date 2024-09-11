#!/bin/zsh

dir="target/"
name="tmp.out"
output="$dir$name"

# remove tmp file containing all the shell cmd output
rm -f $output; 

# recreate tmp file
mkdir -p $dir
touch $output;

# format the files
echo "--------------------------------------------------------------------- cargo fmt --------------------------------------------------------------------- \n" &>> $output;
cargo fmt &>> $output; 

# lint the files
echo "--------------------------------------------------------------------- cargo clippy --------------------------------------------------------------------- \n" &>> $output;
cargo clippy &>> $output; 
cargo clippy --fix --lib -p libft
cargo fix --allow-dirty --allow-staged
echo "--------------------------------------------------------------------- cargo check --------------------------------------------------------------------- \n" &>> $output;
# compile src & dump output to tmp outputfile
cargo check &> $output;  

echo "--------------------------------------------------------------------- cargo run --------------------------------------------------------------------- \n" &>> $output;
# exec bin & append execution output to tmp outputfile
cargo run &>> $output; 

echo "--------------------------------------------------------------------- cargo test --------------------------------------------------------------------- \n" &>> $output;
# exec tests & append execution output to tmp outputfile
cargo test &>> $output; 

# copy tmp outputfile contents to clipboard
head -n 10000 $output src/*.rs | pbcopy;

# print output file using
cat $output; # aliased (bat -- prettier package for printing output that's better than cat)
