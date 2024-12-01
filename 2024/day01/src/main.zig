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

fn solvePart1(allocator: std.mem.Allocator, input: []const u8) !i64 {
    var left = std.ArrayList(i64).init(allocator);
    var right = std.ArrayList(i64).init(allocator);
    defer left.deinit();
    defer right.deinit();

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        var split = std.mem.splitSequence(u8, line, "   ");
        try left.append(try std.fmt.parseInt(i64, split.first(), 10));
        try right.append(try std.fmt.parseInt(i64, split.rest(), 10));
    }
    std.mem.sort(i64, left.items, {}, comptime std.sort.asc(i64));
    std.mem.sort(i64, right.items, {}, comptime std.sort.asc(i64));

    var total: i64 = 0;
    for (left.items, right.items) |l, r| {
        total += if (l > r) l - r else r - l;
    }

    return total;
}

fn solvePart2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var left = std.ArrayList(u64).init(allocator);
    defer left.deinit();
    var counts = std.AutoHashMap(u64, usize).init(allocator);
    defer counts.deinit();

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        var split = std.mem.splitSequence(u8, line, "   ");
        try left.append(try std.fmt.parseInt(u64, split.first(), 10));
        const right = try std.fmt.parseInt(u64, split.rest(), 10);
        const count = try counts.getOrPutValue(right, 0);
        count.value_ptr.* += 1;
    }
    var total: u64 = 0;
    for (left.items) |l| {
        if (counts.get(l)) |count| {
            total += l * count;
        }
    }

    return total;
}

test "part 1 example" {
    const example = "3   4\n" ++
        "4   3\n" ++
        "2   5\n" ++
        "1   3\n" ++
        "3   9\n" ++
        "3   3";
    try std.testing.expectEqual(@as(i64, 11), try solvePart1(std.testing.allocator, example));
}

test "part 2 example" {
    const example = "3   4\n" ++
        "4   3\n" ++
        "2   5\n" ++
        "1   3\n" ++
        "3   9\n" ++
        "3   3";
    try std.testing.expectEqual(@as(u64, 31), try solvePart2(std.testing.allocator, example));
}
