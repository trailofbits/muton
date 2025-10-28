import "stdlib.tact";

contract Complex {
	const VERSION: Int = 1;
	let counter: Int = 0;
	struct Pair { left: Int, right: Int }

	init() {
        // if (true) { require(false); }
        // let z: Int = 1 + 2;
        // if (1 < 2) { let y: Int = 3; }
        // this.callMe(10, 20);
        // while (true) { break; }
		let p: Pair = Pair{ left: 1, right: 2 };
		if (p.left < p.right) { counter = 1; } else { counter = 2; }
	}

	receive("inc") { counter = counter + 1; }
	bounced(msg: String) { let _x: Int = 0; }
	external("ping") { return; }

	fun add(a: Int, b: Int): Int { return a + b; }
} 