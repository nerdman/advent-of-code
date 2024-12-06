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

fn solvePart1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var levels = std.ArrayList(i64).init(allocator);
    defer levels.deinit();
    var total: u64 = 0;

    var line_it = std.mem.tokenizeScalar(u8, input, '\n');
    while (line_it.next()) |line| {
        var value_it = std.mem.tokenizeScalar(u8, line, ' ');
        while (value_it.next()) |val| {
            const level = try std.fmt.parseInt(i64, val, 10);

            try levels.append(level);
        }
        const safe = checkReport(levels.items, null);
        if (safe) {
            total += 1;
        }
        levels.clearRetainingCapacity();
    }

    return total;
}

fn solvePart2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var levels = std.ArrayList(i64).init(allocator);
    defer levels.deinit();
    var total: u64 = 0;

    var line_it = std.mem.tokenizeScalar(u8, input, '\n');
    while (line_it.next()) |line| {
        var value_it = std.mem.tokenizeScalar(u8, line, ' ');
        while (value_it.next()) |val| {
            const level = try std.fmt.parseInt(i64, val, 10);

            try levels.append(level);
        }

        var safe = checkReport(levels.items, null);

        // If the first pass is unsafe, see if removing any single level makes
        // the report safe
        if (!safe) {
            for (0..levels.items.len) |i| {
                if (checkReport(levels.items, i)) {
                    safe = true;
                    break;
                }
            }
        }
        if (safe) {
            total += 1;
        }
        levels.clearRetainingCapacity();
    }

    return total;
}

fn checkReport(levels: []const i64, skip: ?usize) bool {
    var prev_increasing: ?bool = null;
    var prev_val: ?i64 = null;

    for (levels, 0..) |level, i| {
        if (skip == i) continue;

        if (prev_val) |pv| {
            if (pv == level) return false;

            if (pv > level) {
                if (prev_increasing == true) return false;

                if (pv - level > 3) return false;

                prev_increasing = false;
            }

            if (pv < level) {
                if (prev_increasing == false) return false;

                if (level - pv > 3) return false;

                prev_increasing = true;
            }
        }
        prev_val = level;
    }

    return true;
}

test "part 1 example" {
    const example =
        "7 6 4 2 1\n" ++
        "1 2 7 8 9\n" ++
        "9 7 6 2 1\n" ++
        "1 3 2 4 5\n" ++
        "8 6 4 4 1\n" ++
        "1 3 6 7 9";
    try std.testing.expectEqual(@as(u64, 2), try solvePart1(std.testing.allocator, example));
}

test "part 2 example" {
    const example =
        "7 6 4 2 1\n" ++
        "1 2 7 8 9\n" ++
        "9 7 6 2 1\n" ++
        "1 3 2 4 5\n" ++
        "8 6 4 4 1\n" ++
        "1 3 6 7 9";

    try std.testing.expectEqual(@as(u64, 4), try solvePart2(std.testing.allocator, example));
}
