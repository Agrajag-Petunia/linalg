##linalg - Linear Algebra package for Rust##

A set of basic linear algebra functions and classes I use for graphics applications
implemented in Rust.  Mostly, as an excuse to learn Rust in my free time.

#####NOTES:##### 
I didn't realize till recently but [cgmath](https://github.com/bjz/cgmath-rs) is a very similar library which is also being actively worked on.
So, depending on what you want it might be worth checking out.

###Design Characteristics###
* Provide copy, mutate and query versions of methods. Ex) <name>! == <name>_mut and <name>? == <name>_eq
* Converting between math types should be simple and intuitive with from and to methods
* API should be predictable (ie: I don't want to have to look up methods names etc)
* If something can't be unit tested it probably shouldn't be included

###To Do###
#####Iteration 1:#####
* Initial project structure and code [done]
* Write base test cases
* Check that code works with fresh rust install
* Set up Travis CI
