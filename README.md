# Wiper PoC (Proof-of-Concept) - in Rust 

![Wiper PoC Banner](https://img.shields.io/badge/Purpose-Educational%20%26%20Testing-blue.svg)
![Language](https://img.shields.io/badge/Language-Rust-orange.svg)
![Platform](https://img.shields.io/badge/Platform-Windows-lightgrey.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

## ⚠️ EXTREME DANGER - READ CAREFULLY ⚠️

**This project is a Proof-of-Concept (PoC) of a wiper malware. It is designed for educational and testing purposes ONLY. DO NOT run in your machine use a VM**

**Running this software will:**
*   **Irreversibly delete and overwrite data** on your system.
*   **may render your operating system unbootable or highly unstable.**
*   **Cause permanent data loss.**

**I REPEAT DO NOT RUN THIS ON YOUR HOST MACHINE, ANY PRODUCTION SYSTEM, OR ANY SYSTEM CONTAINING VALUABLE DATA.**

**Only execute this code in a fully isolated virtual machine (VM) that has a snapshot taken immediately before execution, allowing for easy rollback.**

---

##  Project Overview

In order to learn Rust I've developed This simple Wiper PoC that simulates the destructive capabilities of malware such as NotPetya, Shamoon, or Olympic Destroyer:

*   **The impact of such attacks** on a Windows operating system.
*   **Techniques for data destruction** (file deletion, overwriting).
*   **Basic string obfuscation** to make static analysis slightly more challenging (though this PoC uses a very simple method maybe I'll work on that more).
*   **Disabling recovery mechanisms** (e.g., Volume Shadow Copies).

The primary goal is to provide a hands-on experience with the destructive power of wipers in a controlled, safe environment to inform defensive strategies.

##  Features

*   **Annoyance Phase:** Spawns multiple Notepads to create initial chaos (purely for "prank" effect lol).
*   **Volume Shadow Copy Deletion:** Attempts to remove VSS to prevent system recovery.
*   **Targeted Data Wipe:** Recursively deletes and overwrites files and directories in critical user and program data paths
*   **Critical OS File Attempts:** Makes a best-effort attempt to delete/overwrite core Windows system files *Note: These attempts are expected to fail on a running OS due to Windows' built-in protections, but I'll try to improve that by either making it run from a seperate disk or by bringing a vulnerable driver.*
*   **System Reboot:** Forces an immediate system reboot to finalize the destruction and maybe leave the system in an unbootable state.

## 🛠️ Building the Project

### Prerequisites

*   **Rust:** Install Rust and Cargo using `rustup`: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
*   **Windows Target:** Ensure you have the `x86_64-pc-windows-msvc` toolchain installed:
    ```bash
    rustup target add x86_64-pc-windows-msvc
    ```
*   **`walkdir` Crate:** This project relies on the `walkdir` crate. Add it to your project:
    ```bash
    cargo add walkdir
    ```

### Compilation

1.  **Build the executable:**
    ```bash
    cargo build --release --target x86_64-pc-windows-msvc
    ```
    This command will compile the project in release mode for Windows, generating an optimized executable.

## Usage (Extremely Carefully!)

1.  **Prepare a Dedicated Virtual Machine (VM):**
    *   Create a fresh Windows 10/11 VM.
    *   **TAKE A SNAPSHOT OF THE VM IMMEDIATELY BEFORE PROCEEDING.** 

2.  **Transfer the Executable:**
    *   Copy the compiled executable (`target\x86_64-pc-windows-msvc\release\wipe_poc_rs.exe` - or whatever your project is named) into your VM.

3.  **Execute as Administrator:**
    *   In the VM, navigate to where you copied the executable.
    *   **Right-click on the executable and select "Run as administrator."** This is crucial, as the wiper requires elevated privileges but I'll work on that also in the Future

4.  **Observe the Chaos:**
    *   The console will display "prank" messages.
    *   Numerous Notepad windows will appear.
    *   The wiper will then proceed to attempt to delete and overwrite files.
    *   Finally, the system will reboot, likely into an unbootable state.

5.  **Revert the VM:**
    *   After observing the effects, revert your VM to the snapshot you took earlier to restore its clean state.
