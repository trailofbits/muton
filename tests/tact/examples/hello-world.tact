// if (true) { require(false); }
// let x: Int = 1 + 2;
// if (1 < 2) { let y: Int = 3; }
// this.callMe(10, 20);
// while (true) { break; }
contract HelloWorld {
    init() { /* return; */ }
    receive("hello") { /* require(false); */ }
    fun greet(name: String): String { return "Hello, " + name; }
}