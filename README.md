# Attic-Redux
Attic...Again! Attic is a client side encrpyted file storage and sharing application, similar to dropbox, google drive, and the such.

## LibAttic
LibAttic provides mid level abstraction to the business logic behind attic. This can probably be broken down into several sub libraries that provide os level functionality and abstraction, and access to a given tent server. Libattic is responsible for:
* Tent service access
* Attic tent lib
  * Allows user to specify tent credentials
    * Login 
    * Logout
  * Pulls necessary meta data surrounding the specifics that the attic app needs
  * Abstracts Posts pushed by the file sentinel.
* Attic File Sentinel
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
      
