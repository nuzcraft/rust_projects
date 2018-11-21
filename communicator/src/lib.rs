
// USING MODULES TO REUSE AND ORGANIZE CODE
// a module is a namespace that contins definitions of functions or types, and you can
// choose whether those definitions are visible outside their module (public) or not
// (private)

// The mod keyword declares a new module. Code within the module appears either immediately
// following this declarartion within curly brackets or in another file

// By default, functions, types, constants, and modules are private. The pub keyword makes
// an item public and therefore visible outside its namespace

// the use keyword brings modules, or the definitions inside modules, into scope so it's
// easier to refer to them

// MODULE DEFINITIONS
// notice that this is a library crate, not a binary crate, and there is no main() function
// cargo run will not run anything, but cargo build will compile our code

// for our communicator networking library, we'll define a module name network that contains
// the definition of a functions called connect. Every module in Rust starts with the mod
// keyword

mod network1 {
    fn connect() {

    }
}

// everything in the block is inside the namespace network. If we wanted to call the connect
// function, we would need to specify the module and use the namespace syntax. For example:
// network1::connect()

// Multiple modules can exist side by side Let's add a client module with a connect function

mod client1 {
    fn connect() {

    }
}

// now we have a network1::connect and client1::connect, which can have completely different 
// functionality, and the function names do not confilct becasue they're in differnt modules

// in this case, because we're building a library, the file that serves as the entry point
// for building our library is src/lib.rs. In respect to creating modules, there's nothing 
// special about src/lib.rs. We could also create modules in src/main.rs for a binary crate
// in the same way as we are currently doing for a library crate. We can put modules inside
// of modules, which can be useful as your modules grow to keep related functionality
// organized together and separate functionality apart. We could move the client module
// inside the network module if that made more sense to users

mod network2 {
    fn connect() {

    }

    mod client {
        fn connect() {

        }
    }
}

// now, the functions network2::connect and network2::client::connect exist, and do not
// confilict since they are still in separate namespaces

// Here is the hierarchy from the first set of examples
// communicator
//  ├── network
//  └── client

// and here is the heirarcy from the second set of examples
//  communicator
//  └── network
//      └── client

// MOVING MODULES TO OTHER FILES
// We can split things up so that not everything lives in src/lib.rs or src/main.rs
// let's replace the client mod with only a declaration of the client module

mod client2;

mod network3 {
    fn connect() {

    }

    mod server {
        fn connect() {
            
        }
    }
}

// declaring the client mod like this tells Rust to look in another location for the code
// defined within the client scope. We can create a client.rs file in the src/ directory
// to hold this code.
// mod client;
// essential means
// mod client {
//    // contents of client.rs
// }

// now we can extract the network module into its own file
mod client3;

mod network4; // enter this file for more info

// extracting the server mod out of the network mod in the following 
mod client5;

mod network5;
// note that in the file explorer, network5 is a directory within the src directory. the
// code that was previously in the network.rs file is now in the mod.rs file.
// communicator
//  ├── client
//  └── network
//      └── server
// The corresponding file layout now looks like this:


// └── src
//     ├── client.rs
//     ├── lib.rs
//     └── network
//         ├── mod.rs
//         └── server.rs
// we do this so that rust can absolutely know which modules are sub-modules of other modules
// and we can avoid any issues surrounding modules with similar names (such as if we wanted)
// a client module within the network module.

// RULES OF MODULE FILESYSTEMS
// If a module named foo has no submodules, you should put the declarations for foo in a file
// named foo.rs.
// if a module named foo does have submodules, you should put the declarations for foo in a
// file named foo/mod.rs
// these rules apply recursively, so if a module named foo has a submodule named bar and bar
// does not have any submodules, you should have the following in your src directory:
// └── foo
//     ├── bar.rs (contains the declarations in `foo::bar`)
//     └── mod.rs (contains the declarations in `foo`, including `mod bar`)
// the modules shouold be declared in their parent module's file using the mod keyword
// next, we'll talk about the pub keyword and get rid of those warnings!


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
