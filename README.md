# File Encryption/Decryption with integrated file explorer in Rust
Project start date: 18.10.2023  
Project status: Mostly completed (25.10.2023)  

![fileProtection-Rust](https://github.com/WiktorB2004/File_Protection-Rust/assets/62223421/70fe4352-80e8-47d9-b0d5-9da9c1874344)

## TODO
- [x] Create custom File Explorer
- [x] Implement basic Encryption/Decryption methods
- [x] Create File Handler to encrypt/decrypt/modify files
- [x] Create user friendly UI for the application
- [ ] Create different encryption methods handling
- [ ] Implement advanced encryption methods
- [ ] (?) Switch between file explorer and action selector using TAB instead of w/s and arrows



## About the project
### Key Features
#### 1. File Encryption  
The core feature of this project is file encryption. With this application, you can encrypt any file you choose. The encryption is executed using strong cryptographic algorithms, making it virtually impossible for anyone without the decryption key to access your data.

#### 2. File Decryption    
Decrypting files encrypted with this application is just as simple as the encryption process. You can quickly and securely decrypt your files, restoring them to their original state. Only those with the proper decryption key will be able to access the data.

#### 3. Integrated File Explorer   
To enhance user convenience, I've created a custom file explorer for the application. This allows you to easily browse your local files and select the ones you want to encrypt or decrypt. No need to switch between different tools.

### Why Rust?
Rust was chosen as the programming language for this project due to its emphasis on safety, performance, and reliability. It offers a robust type system that helps prevent common programming errors, making it an ideal choice for a project focused on data security. Additionally, Rust's cross-platform support ensures that this application can be used on a wide range of operating systems.

## User Manual
#### File/Directory navigation:
| Key | Action |
| ------------- | ------------- |
| <kbd>&uparrow;</kbd> | focus up  |
| <kbd>&downarrow;</kbd> | focus down |
| <kbd>d</kbd> | directory down |
| <kbd>q</kbd> | quit |
| <kbd>Enter</kbd> | directory up / perform selected action on file |

#### Action selection
| Key | Action |
| ------------- | ------------- |
| <kbd>w</kbd> | focus up |
| <kbd>s</kbd> | focus down |

- - - -
Thank you for exploring this project, and I hope it demonstrates my dedication to the craft of software development and my commitment to delivering high-quality solutions. If you have any questions or would like to collaborate with me, please feel free to get in touch.








