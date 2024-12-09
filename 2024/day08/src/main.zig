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

pub const Position = struct {
    row: isize,
    col: isize,

    pub fn hash(self: Position) u64 {
        var hasher = std.hash.Wyhash.init(0);
        std.hash.autoHash(&hasher, self.row);
        std.hash.autoHash(&hasher, self.col);
        return hasher.final();
    }
};

// reused from day 6...
pub const Grid = struct {
    data: []u8,
    rows: [][]u8,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, input: []const u8) !Grid {

        // find dimensions of ascii grid
        const cols = std.mem.indexOfScalar(u8, input, '\n') orelse return error.InvalidFormat;
        const rows = (input.len + 1) / (cols + 1);

        // row pointers for grid
        const grid_rows = try allocator.alloc([]u8, rows);
        errdefer allocator.free(grid_rows);

        // one block for all data
        const data = try allocator.alloc(u8, rows * cols);
        errdefer allocator.free(data);

        // row slices to point into the data block
        var i: usize = 0;
        while (i < rows) : (i += 1) {
            grid_rows[i] = data[i * cols .. (i + 1) * cols];
        }

        // copy the input data into the grid
        var row: usize = 0;
        while (row < rows) : (row += 1) {
            const src_start = row * (cols + 1);
            @memcpy(grid_rows[row], input[src_start..][0..cols]);
        }

        return Grid{
            .data = data,
            .rows = grid_rows,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Grid) void {
        self.allocator.free(self.data);
        self.allocator.free(self.rows);
    }

    pub fn isOutOfBounds(self: Grid, pos: Position) bool {
        return pos.row < 0 or
            pos.row >= self.rows.len or
            pos.col < 0 or
            pos.col >= self.rows[0].len;
    }
};

fn findAntinodes(grid: Grid, node1: Position, node2: Position, antinodes: *std.AutoHashMap(Position, void)) !void {
    const dx = node2.col - node1.col;
    const dy = node2.row - node1.row;

    const a1 = Position{ .col = node1.col - dx, .row = node1.row - dy };

    const a2 = Position{ .col = node2.col + dx, .row = node2.row + dy };

    if (!grid.isOutOfBounds(a1)) {
        try antinodes.put(a1, {});
    }

    if (!grid.isOutOfBounds(a2)) {
        try antinodes.put(a2, {});
    }
}

fn findAllAntinodes(grid: Grid, node1: Position, node2: Position, antinodes: *std.AutoHashMap(Position, void)) !void {
    const dx = node2.col - node1.col;
    const dy = node2.row - node1.row;

    var i: isize = 0;
    while (true) {
        const a1 = Position{ .col = node1.col - dx * i, .row = node1.row - dy * i };
        if (grid.isOutOfBounds(a1)) break;
        try antinodes.put(a1, {});
        i += 1;
    }

    i = 0;
    while (true) {
        const a2 = Position{ .col = node1.col + dx * i, .row = node1.row + dy * i };
        if (grid.isOutOfBounds(a2)) break;
        try antinodes.put(a2, {});
        i += 1;
    }
}

pub fn findAntennas(grid: Grid, antennas: *std.AutoHashMap(u8, std.ArrayList(Position))) !void {
    for (grid.rows, 0..) |row, row_idx| {
        for (row, 0..) |char, col_idx| {
            if (char != '.') {
                try addAntenna(antennas, char, Position{ .row = @intCast(row_idx), .col = @intCast(col_idx) });
            }
        }
    }
}

fn addAntenna(map: *std.AutoHashMap(u8, std.ArrayList(Position)), key: u8, pos: Position) !void {
    if (map.getPtr(key)) |list| {
        try list.append(pos);
    } else {
        var list = std.ArrayList(Position).init(map.allocator);
        try list.append(pos);
        try map.put(key, list);
    }
}

fn solvePart1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var grid = try Grid.init(allocator, input);
    defer grid.deinit();

    var antennas = std.AutoHashMap(u8, std.ArrayList(Position)).init(allocator);

    try findAntennas(grid, &antennas);

    var antinodes = std.AutoHashMap(Position, void).init(allocator);
    defer antinodes.deinit();
    var antennas_it = antennas.valueIterator();

    while (antennas_it.next()) |positions| {
        for (positions.items, 0..) |node, idx| {
            for (positions.items, 0..) |pair, pair_idx| {
                if (pair_idx != idx) {
                    try findAntinodes(grid, node, pair, &antinodes);
                }
            }
        }
    }

    var it = antennas.iterator();
    while (it.next()) |entry| {
        entry.value_ptr.deinit();
    }
    antennas.deinit();
    return antinodes.count();
}

fn solvePart2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var grid = try Grid.init(allocator, input);
    defer grid.deinit();

    var antennas = std.AutoHashMap(u8, std.ArrayList(Position)).init(allocator);

    try findAntennas(grid, &antennas);

    var antinodes = std.AutoHashMap(Position, void).init(allocator);
    defer antinodes.deinit();
    var antennas_it = antennas.valueIterator();

    while (antennas_it.next()) |positions| {
        for (positions.items, 0..) |node, idx| {
            for (positions.items, 0..) |pair, pair_idx| {
                if (pair_idx != idx) {
                    try findAllAntinodes(grid, node, pair, &antinodes);
                }
            }
        }
    }

    var it = antennas.iterator();
    while (it.next()) |entry| {
        entry.value_ptr.deinit();
    }
    antennas.deinit();
    return antinodes.count();
}

test "part 1 example" {
    const example =
        \\............
        \\........0...
        \\.....0......
        \\.......0....
        \\....0.......
        \\......A.....
        \\............
        \\............
        \\........A...
        \\.........A..
        \\............
        \\............
    ;
    try std.testing.expectEqual(@as(u64, 14), try solvePart1(std.testing.allocator, example));
}

test "part 2 example" {
    const example =
        \\............
        \\........0...
        \\.....0......
        \\.......0....
        \\....0.......
        \\......A.....
        \\............
        \\............
        \\........A...
        \\.........A..
        \\............
        \\............
    ;
    try std.testing.expectEqual(@as(u64, 34), try solvePart2(std.testing.allocator, example));
}
