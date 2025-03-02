# Todos
This is a list of things which I want or need to do still

- ## Workspaces
  - ### Security
    - [ ] Workspace authentication via group policy
    - [ ] At rest workspace manifest encryption
  - ### Importing
    - [ ] Git workspace imports
    - [ ] Rsync workspace imports
    - [ ] S3 workspace imports
  - ### Sync
    - [ ] Git workspace sync
    - [ ] Rsync workspace sync
    - [ ] S3 workspace sync

- ## Editor
  - everything.



# Ramblings

### Workspace authentication via group policy
Allow one, or many users to load a workspace via group policy, may need custom tooling, may 
be compatible with existing FOSS options. will depend on how it is implemented and what 
sort of use cases it ends up providing




## Workflows for encryption
### Personal
> Note: This is for personal encryption only, this workflow will not work for enterprise grade encryption.
> It is also important to note that this will mean any backups of your workspace will only be viewable on your current installation of your current OS.
> This data is incredibly easy to lose, so follow good data handling practices, because we cannot help you recover a workspace once it is encrypted.

1. Enable encryption on workspace
2. Workspace config becomes encrypted with a key unique to your system (generated at run time using known constants)
3. Workspace session encrypts all buffers in the workspace directory with a unique key per file
4. Workspace session encrypts all raw files in thr workspace with a unique key
5. Workspace session dumps itself to disk and zeroes the memory used to allocate that previous encrypted session
6. Workspace is now encrypted at rest, and is only decrypted to be worked on when the file is actively being used by the workspace (eg: when it is an open buffer)


### Enterprise
1. Client authenticates through OIDC provider
2. Client connects to the **Noot Vault Server** (NVS) via TCP
3. NVS imposes TLS on the connection, ensuring data is secured in transit
4. Client authenticates using OIDC with NVS, which verifies the credentials
5. NVS determines if a client is authorized to access any workspaces before returning a list of available vaults
6. Client selects vault, requesting from NVS
7. NVS sends the vault as an encrypted binary file over the transport
8. Client decrypts and unpacks encrypted binary to a temporary folder with restricted permissions where possible
9. Workspace session initializes, parsing the workspace as necessary
10. Workspace is now encrypted at rest on client and NVS, and all in flight data is also encrypted. 

Since workspace is encrypted at rest, in the event of a crash, or power failure, the files should remain secure.
In the event that the application fails to clean up after itself through a crash, bug, or some other issue, the files should remain secure.

If for some reason the worst happens and Noot fails to protect data effectively, it would be best to roll all security details related to each workspace.


## Plugins
### Personal

Plugins are installed into the `plugin-directory` as defined in the workspace configuration. This is usually `.plugins`.
They are run in a sandboxed permission scoped lua VM which 


### Enterprise

Plugins are disabled by default for privacy and security reasons. Workspace admins may 
opt in to enable plugins, which are then stored on the NVS and transmitted with the 
workspace file. Plugin configuration is synchronized across the workspace via the NVS,
allowing admins to modify it on their local machines at any time, and propogate the changes to 
all other noot instances loading the workspace.
