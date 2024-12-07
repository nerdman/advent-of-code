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

const Rule = struct {
    a: usize,
    b: usize,

    pub fn hash(self: @This()) u64 {
        var hasher = std.hash.Wyhash.init(0);
        std.hash.autoHash(&hasher, self.a);
        std.hash.autoHash(&hasher, self.b);
        return hasher.final();
    }

    pub fn eql(self: @This(), other: @This()) bool {
        return self.a == other.a and self.b == other.b;
    }
};

fn solvePart1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var total: u64 = 0;

    var rules = std.AutoHashMap(Rule, void).init(allocator);
    defer rules.deinit();

    var pages = std.ArrayList(usize).init(allocator);
    defer pages.deinit();

    var lines = std.mem.tokenizeScalar(u8, input, '\n');

    while (lines.next()) |line| {
        if (std.mem.indexOfScalar(u8, line, '|')) |idx| {
            const first = try std.fmt.parseInt(usize, line[0..idx], 10);
            const second = try std.fmt.parseInt(usize, line[idx + 1 ..], 10);

            const rule = Rule{ .a = first, .b = second };
            try rules.put(rule, {});
        } else if (std.mem.indexOfScalar(u8, line, ',')) |_| {
            var itr = std.mem.splitScalar(u8, line, ',');
            while (itr.next()) |page| {
                try pages.append(try std.fmt.parseInt(usize, page, 10));
            }

            var pages_ordered = false;
            outer_loop: for (0..pages.items.len - 1) |i| {
                pages_ordered = true;
                for (i + 1..pages.items.len) |j| {
                    const rule = Rule{ .a = pages.items[i], .b = pages.items[j] };
                    if (rules.contains(rule) == false) {
                        pages_ordered = false;
                        break :outer_loop;
                    }
                }
            }
            if (pages_ordered) {
                const idx = pages.items.len / 2;
                total += pages.items[idx];
            }
            pages.clearRetainingCapacity();
        }
    }

    return total;
}

fn solvePart2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var total: u64 = 0;

    var rules = std.AutoHashMap(Rule, void).init(allocator);
    defer rules.deinit();

    var pages = std.ArrayList(usize).init(allocator);
    defer pages.deinit();

    var lines = std.mem.tokenizeScalar(u8, input, '\n');

    while (lines.next()) |line| {
        if (std.mem.indexOfScalar(u8, line, '|')) |idx| {
            const first = try std.fmt.parseInt(usize, line[0..idx], 10);
            const second = try std.fmt.parseInt(usize, line[idx + 1 ..], 10);

            const rule = Rule{ .a = first, .b = second };
            try rules.put(rule, {});
        } else if (std.mem.indexOfScalar(u8, line, ',')) |_| {
            var itr = std.mem.splitScalar(u8, line, ',');
            while (itr.next()) |page| {
                try pages.append(try std.fmt.parseInt(usize, page, 10));
            }

            var pages_sorted = false;
            for (0..pages.items.len - 1) |i| {
                for (i + 1..pages.items.len) |j| {
                    const rule = Rule{ .a = pages.items[i], .b = pages.items[j] };
                    if (rules.contains(rule) == false) {
                        pages_sorted = true;
                        const tmp = pages.items[i];
                        pages.items[i] = pages.items[j];
                        pages.items[j] = tmp;
                    }
                }
            }
            if (pages_sorted) {
                const idx = pages.items.len / 2;
                total += pages.items[idx];
            }
            pages.clearRetainingCapacity();
        }
    }

    return total;
}

test "part 1 example" {
    const example =
        \\47|53
        \\97|13
        \\97|61
        \\97|47
        \\75|29
        \\61|13
        \\75|53
        \\29|13
        \\97|29
        \\53|29
        \\61|53
        \\97|53
        \\61|29
        \\47|13
        \\75|47
        \\97|75
        \\47|61
        \\75|61
        \\47|29
        \\75|13
        \\53|13
        \\
        \\75,47,61,53,29
        \\97,61,53,29,13
        \\75,29,13
        \\75,97,47,61,53
        \\61,13,29
        \\97,13,75,29,47
    ;
    try std.testing.expectEqual(@as(u64, 143), try solvePart1(std.testing.allocator, example));
}

test "part 2 example" {
    const example =
        \\47|53
        \\97|13
        \\97|61
        \\97|47
        \\75|29
        \\61|13
        \\75|53
        \\29|13
        \\97|29
        \\53|29
        \\61|53
        \\97|53
        \\61|29
        \\47|13
        \\75|47
        \\97|75
        \\47|61
        \\75|61
        \\47|29
        \\75|13
        \\53|13
        \\
        \\75,47,61,53,29
        \\97,61,53,29,13
        \\75,29,13
        \\75,97,47,61,53
        \\61,13,29
        \\97,13,75,29,47
    ;
    try std.testing.expectEqual(@as(u64, 123), try solvePart2(std.testing.allocator, example));
}
