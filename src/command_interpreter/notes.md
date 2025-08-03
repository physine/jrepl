Commands can be defined by a 3 tuple:

1. symbol name
2. side effect (state change)
3. return value (return string, in the case of 'help' command)

---

Lexer(user_input) -> tokens
Parser(tokens) -> AST
Evaluation(AST) -> Effect

---

what I want:

    - Symbols are names bound (
        to data,
        commands which transform data,
        other constructs in the environment. ex: a file
    )

    - Symbols, a Command for instance can only interoperate with a sub set of other symbols.
    And in general, symbols can only interoperate with certain kinds of other symbols.

    Commands:
        help,
        list,
        undo,
        quit,
        search,
        pritty print,
        load json file,
        transform data,
        create symbol,
        set symbol

    - expr (
        "(list <symbol>)"
    )

    fn eval(state: AppState) // destruct to whats only needed
    fn undo(state: AppState) // destruct to whats only needed

---

When the user inputs something, ex: "(def mydata (filter places-ive-been-to "USA"))"

I

---

    Kinds of Symbols (either one or both)
        - Data Representation
        - Functional

1. Every Symbol requires a name bound to it.
2. Every Symbol represents Data and/or contains Functionality

struc Symbol {
name: String,
}

struct Functional {

    fn execute()
    fn undo()

}

struct Data {

    data: Value,

}

Commands:
create symbol: (<functional-symbol> <symbol-name> <value>)
help: (<functional-symbol>)
list: (<functional-symbol> [<symbol-name>])
load json file: (<functional-symbol> [<symbol-name>])
pritty print: (<functional-symbol> [<symbol-name>])
quit: (<functional-symbol>)
search: (<functional-symbol> [<symbol-name>])
set symbol: (<functional-symbol> <symbol-name> <value>)
transform data: (<functional-symbol> <symbol-name> <value>)
undo: (<functional-symbol>)
