# CTF MetaRed Chile-TIC 2024

This is a simple Rust code made for the sole purpose of decoding an input given in MetaRed Chile's CTF competition.

# Programming Languages Freak

My friend is a total "programming languages freak," to the point that he decided to create one himself!

The language works like this:

The language is stack-based (similar to an array).
Initially, the stack consists of a single element with a value of 0. Allowed operations:

"-" : Decreases the value of the stack's last element by 1.

"+" : Increases the value of the stack's last element by 1.

">" : Moves the first element to the end of the stack, shifting all others down.

"<" : Moves the last element to the beginning of the stack, shifting all others up.

"@" : Swaps the last two elements of the stack.

"." : Duplicates the last element of the stack and adds it to the end.

"€" : Outputs the ASCII representation of each element in the stack (from first to last).

# Example

For the following commands: .+.-->.<@

The stack evolves as follows:

Init: [0]

"." : [0, 0]

"+" : [0, 1]

"." : [0, 1, 1]

"-" : [0, 1, 0]

"-" : [0, 1, -1]

">" : [1, -1, 0]

"." : [1, -1, 0, 0]

"<" : [0, 1, -1, 0]

"@" : [0, 1, 0, -1]

---

Your Task: What will be the printed ASCII output?

input:

++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.++.------------------------------------------------------------------.+++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++.--------------------------------------------------------------.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++..------------------------------------------------------------.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.-------.--------.+++++++++++++++++.++.------------------------------------------------------------------.+++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++.--------------------------------------------------------------.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++..------------------------------------------------------------.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.-------.--------.+++++++++++++.--------------------------------------------------------.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.-------.++++++++++++++.-----------------------------------------------------------------.+++++++++++++++++++++++++++++++++++++++++++++++++++.----------------------------------------------------.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.----------------------------------------------------€

output/flag:

pr0gr4mm1ng_pr0gr4mm1ng_l4ngu4g3s?
