// Simple Tolk example for mutation testing

fun add(a: int, b: int): int {
    return a + b;
}

fun isPositive(x: int): bool {
    if (x > 0) {
        return true;
    }
    return false;
}

fun calculate(x: int, y: int): int {
    var result = 0;

    if (x > y) {
        result = x - y;
    } else {
        result = y - x;
    }

    while (result > 10) {
        result = result / 2;
    }

    return result;
}

fun processValue(val: int): int {
    var counter = 0;
    var active = true;

    if (active && val > 0) {
        counter = counter + 1;
    }

    if (val >= 100 || val <= 0) {
        return 0;
    }

    return add(counter, val);
}

fun loopTest(n: int): int {
    var sum = 0;
    var i = 0;

    while (i < n) {
        if (i % 2 == 0) {
            sum = sum + i;
        } else {
            continue;
        }
        i = i + 1;
    }

    return sum;
}
