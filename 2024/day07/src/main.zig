const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const input = @embedFile("input");

    // Part 1
    const part1 = try solvePart1(allocator, input);
    print("Part 1: {d}\n", .{part1});

    // Part 2
    const part2 = try solvePart2(allocator, input);
    print("Part 2: {d}\n", .{part2});
}

fn safeAdd(a: usize, b: usize, target: usize) bool {
    return if (a > target or b > target) false else a + b <= target;
}

fn safeMul(a: usize, b: usize, target: usize) bool {
    if (a == 0 or b == 0) return false;

    if (a > target or b > target) return false;

    const product = std.math.mul(usize, a, b) catch return false;
    return product <= target;
}

// barf... but std.fmt.allocPrint was just WAY too slow...
fn concatNumbers(a: usize, b: usize) usize {
    var b_digits: usize = 1;
    var temp = b;
    while (temp >= 10) : (temp /= 10) {
        b_digits += 1;
    }
    var multiplier: usize = 1;
    var i: usize = 0;
    while (i < b_digits) : (i += 1) {
        multiplier *= 10;
    }
    return (a * multiplier) + b;
}

fn backtrack(numbers: std.ArrayList(usize), index: usize, target: usize, current_value: usize, do_concat: bool) bool {
    if (index >= numbers.items.len) {
        return (current_value == target);
    }

    const next_num = numbers.items[index];

    var add_result = false;
    if (safeAdd(current_value, next_num, target)) {
        add_result = backtrack(numbers, index + 1, target, current_value + next_num, do_concat);
    }

    var mul_result = false;
    if (safeMul(current_value, next_num, target)) {
        mul_result = backtrack(numbers, index + 1, target, current_value * next_num, do_concat);
    }

    var concat_result = false;
    if (do_concat and current_value <= target) {
        const concat_value = concatNumbers(current_value, next_num);
        if (concat_value <= target) {
            concat_result = backtrack(numbers, index + 1, target, concat_value, do_concat);
        }
    }

    return add_result or mul_result or concat_result;
}

fn findExpression(numbers: std.ArrayList(usize), target: usize, do_concat: bool) bool {
    return backtrack(numbers, 1, target, numbers.items[0], do_concat);
}

fn solve(allocator: std.mem.Allocator, input: []const u8, do_concat: bool) !u64 {
    var total: u64 = 0;

    var numbers = std.ArrayList(usize).init(allocator);
    defer numbers.deinit();
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    while (lines.next()) |line| {
        var equation = std.mem.tokenizeSequence(u8, line, ": ");
        const target = try std.fmt.parseInt(usize, equation.next().?, 10);
        var numbers_str = std.mem.tokenizeScalar(u8, equation.next().?, ' ');
        while (numbers_str.next()) |num| {
            try numbers.append(try std.fmt.parseInt(usize, num, 10));
        }
        if (findExpression(numbers, target, do_concat)) {
            total += target;
        }
        numbers.clearRetainingCapacity();
    }
    return total;
}

fn solvePart1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    return solve(allocator, input, false);
}

fn solvePart2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    return solve(allocator, input, true);
}

test "part 1 example" {
    const example =
        \\190: 10 19
        \\3267: 81 40 27
        \\83: 17 5
        \\156: 15 6
        \\7290: 6 8 6 15
        \\161011: 16 10 13
        \\192: 17 8 14
        \\21037: 9 7 18 13
        \\292: 11 6 16 20
    ;
    try std.testing.expectEqual(@as(u64, 3749), try solvePart1(std.testing.allocator, example));
}

test "part 2 example" {
    const example =
        \\190: 10 19
        \\3267: 81 40 27
        \\83: 17 5
        \\156: 15 6
        \\7290: 6 8 6 15
        \\161011: 16 10 13
        \\192: 17 8 14
        \\21037: 9 7 18 13
        \\292: 11 6 16 20
    ;

    try std.testing.expectEqual(@as(u64, 11387), try solvePart2(std.testing.allocator, example));
}
