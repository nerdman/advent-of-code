const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    const input = @embedFile("input");

    // Part 1
    const part1 = try solvePart1(input);
    print("Part 1: {d}\n", .{part1});

    // Part 2
    const part2 = try solvePart2(input);
    print("Part 2: {d}\n", .{part2});
}

fn solvePart1(input: []const u8) !u64 {
    var total: u64 = 0;

    const instruction = "mul(";

    var line_it = std.mem.tokenizeScalar(u8, input, '\n');

    while (line_it.next()) |line| {
        const len = line.len;
        var pos: usize = 0;
        while (std.mem.indexOfPos(u8, line, pos, instruction)) |start_pos| {
            pos = start_pos + instruction.len;

            var lhs_operand: u32 = 0;
            var digits: i32 = 0;

            while (pos < len) {
                const digit = line[pos];
                if (!std.ascii.isDigit(digit) or digits > 2) break;
                const digit_value: u32 = @intCast(digit - '0');
                lhs_operand = lhs_operand * 10 + digit_value;
                digits += 1;
                pos += 1;
            }

            if (line[pos] == ',') pos += 1 else continue;

            var rhs_operand: u32 = 0;
            digits = 0;

            while (pos < len) {
                const digit = line[pos];
                if (!std.ascii.isDigit(digit) or digits > 2) break;
                const digit_value: u32 = @intCast(digit - '0');
                rhs_operand = rhs_operand * 10 + digit_value;
                digits += 1;
                pos += 1;
            }

            if (line[pos] != ')') {
                continue;
            } else {
                pos += 1;
            }

            total += lhs_operand * rhs_operand;
        }
    }

    return total;
}

fn solvePart2(input: []const u8) !u64 {
    var total: u64 = 0;

    const instructions = [_][]const u8{ "mul(", "do()", "don't()" };
    var do_instruction = true;

    var line_it = std.mem.tokenizeScalar(u8, input, '\n');
    while (line_it.next()) |line| {
        const len = line.len;
        var pos: usize = 0;

        while (pos < len) {
            var matched_instr: ?[]const u8 = null;
            for (instructions) |instr| {
                if (std.mem.startsWith(u8, line[pos..], instr)) {
                    matched_instr = instr;
                    break;
                }
            }

            if (matched_instr) |instr| {
                if (std.mem.eql(u8, instr, "mul(")) {
                    pos += instr.len;

                    if (!do_instruction) continue;

                    var lhs_operand: u32 = 0;
                    var digits: i32 = 0;

                    while (pos < len) {
                        const digit = line[pos];
                        if (!std.ascii.isDigit(digit) or digits > 2) break;
                        const digit_value: u32 = @intCast(digit - '0');
                        lhs_operand = lhs_operand * 10 + digit_value;
                        digits += 1;
                        pos += 1;
                    }

                    if (line[pos] == ',') pos += 1 else continue;

                    var rhs_operand: u32 = 0;
                    digits = 0;

                    while (pos < len) {
                        const digit = line[pos];
                        if (!std.ascii.isDigit(digit) or digits > 2) break;
                        const digit_value: u32 = @intCast(digit - '0');
                        rhs_operand = rhs_operand * 10 + digit_value;
                        digits += 1;
                        pos += 1;
                    }

                    if (line[pos] != ')') {
                        continue;
                    } else {
                        pos += 1;
                    }

                    total += lhs_operand * rhs_operand;
                } else if (std.mem.eql(u8, instr, "do()")) {
                    pos += instr.len;
                    do_instruction = true;
                } else if (std.mem.eql(u8, instr, "don't()")) {
                    pos += instr.len;
                    do_instruction = false;
                }
            } else {
                pos += 1;
            }
        }
    }

    return total;
}

test "part 1 example" {
    const example = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    try std.testing.expectEqual(@as(u64, 161), try solvePart1(example));
}

test "part 2 example" {
    const example = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    try std.testing.expectEqual(@as(u64, 48), try solvePart2(example));
}
