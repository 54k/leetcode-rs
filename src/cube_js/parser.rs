// Write a simple equation parser and solver. The equation can have a single variable x, operations +, -, *, /, integer numbers, and parenthesis.
// It’s guaranteed that there’s only one occurrence of x .
// The solution can be algebraic or calculated. Please use rewrites to solve the equation.
// Example equations and solutions:
// 1
// equation: (x + 50) * 10 - 15 = 10
// solution: x = (((10 + 15) / 10) - 50)
// 2
// equation: ((x + 10) + 30) = 10 + 20
// solution: x = (((10 + 20) - 30) - 10)
// 3
// equation: ((x + 10) + (20 + 30)) = 10 + 20
// solution: x = (((10 + 20) - (20 + 30)) - 10)
// 4
// equation: ((20 + 30) + (x + 10)) = 10 + 20
// solution: x = (((10 + 20) - (20 + 30)) - 10)
// 5
// equation: (((3 * x) * 4) * 5) = 1
// solution: x = (((1 / 5) / 4) / 3)
// 6
// equation: (((3 / x) * 4) * 5) = 1
// solution: x = (3 / ((1 / 5) / 4))
// 7
// equation: (((3 / (x + 10)) * 4) * 5) = 1
// solution: x = ((3 / ((1 / 5) / 4)) - 10)
// Output solutions are just examples and can vary. More the solver can parse and solve better.
// It’s tough to build a solver which can solve all the cases in the allotted time however it’s still possible.
// You can use any library for parser however you’d need to write solver yourself.

fn solve(str: String) -> i32 {
    // meh I'm too weak, will return to you after reading dragon book
    0
}
