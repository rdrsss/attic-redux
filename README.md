# Attic-Redux
Attic...Again! Attic is a client side encrpyted file storage and sharing application, similar to dropbox, google drive, and the such.

## Technologies
* Base libraries implemented in rust.
* [Tent!](https://tent.io/)

## LibAttic
LibAttic provides mid level abstraction to the business logic behind attic. This can probably be broken down into several sub libraries that provide os level functionality and abstraction, and access to a given tent server. Libattic is responsible for:
* Tent service access
* **Attic tent lib**
  * Allows user to specify tent credentials
    * Login 
    * Logout
  * Pulls necessary meta data surrounding the specifics that the attic app needs
  * Abstracts Posts pushed by the file sentinel.
* **Attic File Sentinel**
  * Access
  * File monitoring
    * Os abstractions
      * Windows, OsX, Linux
  * File manipulation
    * Pipelines
      * Chunking
      * Assembly 
    * Crypto
      * Encryption
      * Decrpytion 
     * File integrity
      * Rolling hashes
      * Metadata aggregation
 
## Attic File Sentinel     
The file sentinel is attic's gateway to the underlying file system and file manipulation functions and pipelines.

**Note*** There will have to be quite a bit of interaction with the OS api, will need to look into best practices for interacting with the WinApi (or any other api's) directly from rust. The goal being to keep it mostly rusty.

### Links
* https://github.com/DaGenix/rust-crypto
* https://doc.rust-lang.org/std/fs/
* http://gekkio.fi/blog/2014-10-08-calling-win32-api-with-rust-ffi.html


## Attic Apps
This section describes functionality surrounding various apps utilizing lib attic.

* Windows
* Linux
* OsX
* Web




      
