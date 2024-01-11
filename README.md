# Password Manager

## Usage

1. Each time you start the app, you will be asked for the master key. It is used for encrypting your passwords saved there. Select a safe password so it can't be bruitforced with a fixed time. But remember that if you lose this password you would not be able to access any saved one.

> Note that all passwords, for security reasons, are not displayed when you input them.

2. After the first launch, you will be asked for a path to password storage file (if it doesn't exist, it will be created).

3. Then a terminal-like interface will appear. Here are available commands (none of them accept parameters):

    - `list` lists all resources for which you have saved password
    - `add` adds new password
    - `get` selects password, copy it to clipboard
    - `del` deletes selected password
    - `exit` syncs and exits

> Passwords should have unique names

## Security
- AES-256 encryption is used there.
- All passwords are encrypted together
- Unencrypted data is stored only in RAM. On disk they are always encrypted.
- When a variable with an unencrypted password is no longer used, it is zeroed.