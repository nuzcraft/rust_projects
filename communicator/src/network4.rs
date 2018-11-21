fn connect() {

}

// at this point, we want to extract server into its own module, but extracting it in the
// same way won't work.

// essentially, lib.rs is special because it's at the root of the library. In order to do
// what we want, we essentially need to create a 'network' folder and make this file the
// root file of that folder, mod.rs

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
mod server {
    fn connect() {

    }
}