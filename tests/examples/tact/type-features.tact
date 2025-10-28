// if (true) { require(false); }
// let z: Int = 1 + 2;
// if (1 < 2) { let y: Int = 3; }
// Math::add(10, 20);
// while (true) { break; }
struct User { id: Int, name: String }

fun makeUser(id: Int, name: String): User { return User{ id: id, name: name }; }

contract Types {
    let maybe: String? = null;
    fun getOrDefault(v: String?): String { return v ?? ""; }
}