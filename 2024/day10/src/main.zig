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

    pub fn eql(self: Position, other: Position) bool {
        return self.row == other.row and self.col == other.col;
    }
};

// Another grid reused from day 6...
pub const Grid = struct {
    data: []u8,
    rows: [][]u8,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, input: []const u8) !Grid {
        const cols = std.mem.indexOfScalar(u8, input, '\n') orelse return error.InvalidFormat;
        const rows = (input.len + 1) / (cols + 1);

        const grid_rows = try allocator.alloc([]u8, rows);
        errdefer allocator.free(grid_rows);

        const data = try allocator.alloc(u8, rows * cols);
        errdefer allocator.free(data);

        var i: usize = 0;
        while (i < rows) : (i += 1) {
            grid_rows[i] = data[i * cols .. (i + 1) * cols];
        }

        var row: usize = 0;
        while (row < rows) : (row += 1) {
            const src_start = row * (cols + 1);
            var col: usize = 0;
            while (col < cols) : (col += 1) {
                const c = input[src_start + col];

                grid_rows[row][col] = c - '0';
            }
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

pub const directions = [_][2]isize{
    .{ -1, 0 },
    .{ 0, 1 },
    .{ 1, 0 },
    .{ 0, -1 },
};

fn scoreTrail(grid: Grid, pos: Position, elevation: u8, reached_nines: *std.AutoHashMap(Position, void), visited: *std.AutoHashMap(Position, void)) !void {
    if (elevation == 9) {
        if (reached_nines.contains(pos) == false) {
            try reached_nines.put(pos, {});
        }
        return;
    }

    if (visited.contains(pos) == true) {
        return;
    }
    try visited.put(pos, {});

    for (directions) |dir| {
        const next_pos = Position{ .row = pos.row + dir[0], .col = pos.col + dir[1] };
        if (!grid.isOutOfBounds(next_pos) and grid.rows[@intCast(next_pos.row)][@intCast(next_pos.col)] == elevation + 1) {
            try scoreTrail(grid, next_pos, elevation + 1, reached_nines, visited);
        }
    }
    _ = visited.remove(pos);
}

fn solvePart1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var grid = try Grid.init(allocator, input);
    defer grid.deinit();

    var total: u64 = 0;
    var reached_nines = std.AutoHashMap(Position, void).init(grid.allocator);
    defer reached_nines.deinit();

    var visited = std.AutoHashMap(Position, void).init(grid.allocator);
    defer visited.deinit();

    for (grid.rows, 0..) |row, row_idx| {
        for (row, 0..) |elevation, col_idx| {
            if (elevation == 0) {
                reached_nines.clearRetainingCapacity();
                visited.clearRetainingCapacity();
                const trailhead = Position{ .row = @intCast(row_idx), .col = @intCast(col_idx) };
                try scoreTrail(grid, trailhead, elevation, &reached_nines, &visited);
                total += reached_nines.count();
            }
        }
    }

    return total;
}

fn rateTrail(grid: Grid, pos: Position, elevation: u8, reached_nines: *std.AutoHashMap(Position, usize)) !void {
    if (elevation == 9) {
        const existing_ptr = reached_nines.getPtr(pos);
        if (existing_ptr) |val_ptr| {
            val_ptr.* += 1;
        } else {
            try reached_nines.put(pos, 1);
        }
        return;
    }

    for (directions) |dir| {
        const next_pos = Position{ .row = pos.row + dir[0], .col = pos.col + dir[1] };
        if (!grid.isOutOfBounds(next_pos) and grid.rows[@intCast(next_pos.row)][@intCast(next_pos.col)] == elevation + 1) {
            try rateTrail(grid, next_pos, elevation + 1, reached_nines);
        }
    }
}

fn solvePart2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var grid = try Grid.init(allocator, input);
    defer grid.deinit();

    var total: u64 = 0;
    var reached_nines = std.AutoHashMap(Position, usize).init(grid.allocator);
    defer reached_nines.deinit();

    for (grid.rows, 0..) |row, row_idx| {
        for (row, 0..) |elevation, col_idx| {
            if (elevation == 0) {
                reached_nines.clearRetainingCapacity();
                const trailhead = Position{ .row = @intCast(row_idx), .col = @intCast(col_idx) };

                try rateTrail(grid, trailhead, elevation, &reached_nines);
                var value_iterator = reached_nines.valueIterator();
                while (value_iterator.next()) |value| {
                    total += value.*;
                }
            }
        }
    }

    return total;
}

test "part 1 example" {
    const example =
        \\89010123
        \\78121874
        \\87430965
        \\96549874
        \\45678903
        \\32019012
        \\01329801
        \\10456732
    ;
    try std.testing.expectEqual(@as(u64, 36), try solvePart1(std.testing.allocator, example));
}

test "part 2 example" {
    const example =
        \\89010123
        \\78121874
        \\87430965
        \\96549874
        \\45678903
        \\32019012
        \\01329801
        \\10456732
    ;

    try std.testing.expectEqual(@as(u64, 81), try solvePart2(std.testing.allocator, example));
}
