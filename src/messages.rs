pub fn welcome_message() {
    println!(r"   __________    ______________     ___");
    println!(r"  /      \   \  /   /      \   |   |   |");
    println!(r" /    ____\   \/   /   _____\  |___|   |");
    println!(r"|    /     \      /        \           |");
    println!(r"|    \____ /      \_____    \   ___    |");
    println!(r" \        /   /\   \        /  |   |   |");
    println!(r"  \______/___/  \___\______/___|   |___|");
    println!(r"");
    println!(r"Welcome to cxsh!");
    println!(r"Type 'help' for instructions on how to use cxsh.");
    println!(r"source code/bug report: https://github.com/cordx56/cxsh");
    println!(r"");
}

pub fn help() {
    println!(r"available builtin commands: cd, help, exit");
}
