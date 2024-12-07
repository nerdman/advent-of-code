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

const Direction = enum(usize) {
    north = 0,
    east = 1,
    south = 2,
    west = 3,

    pub const moves = [_][2]isize{
        .{ -1, 0 }, // NORTH
        .{ 0, 1 }, // EAST
        .{ 1, 0 }, // SOUTH
        .{ 0, -1 }, // WEST
    };

    pub fn getMove(self: Direction) [2]isize {
        return moves[@intFromEnum(self)];
    }

    pub fn turnRight(self: Direction) Direction {
        return @enumFromInt((@intFromEnum(self) + 1) % 4);
    }
};

pub const GuardState = struct {
    pos: Position,
    dir: Direction,

    pub fn hash(self: GuardState) u64 {
        var hasher = std.hash.Wyhash.init(0);
        std.hash.autoHash(&hasher, self.pos.hash());
        std.hash.autoHash(&hasher, @intFromEnum(self.dir));
        return hasher.final();
    }

    pub fn eql(self: GuardState, other: GuardState) bool {
        return self.pos.row == other.pos.row and
            self.pos.col == other.pos.col and
            self.dir == other.dir;
    }
};

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

    pub fn findChar(self: Grid, target: u8) ?[2]usize {
        for (self.rows, 0..) |row, row_idx| {
            for (row, 0..) |char, col_idx| {
                if (char == target) {
                    return .{ row_idx, col_idx };
                }
            }
        }
        return null;
    }

    pub fn countChar(self: Grid, target: u8) usize {
        var total: usize = 0;
        for (self.rows) |row| {
            for (row) |char| {
                if (char == target) {
                    total += 1;
                }
            }
        }
        return total;
    }

    pub fn isOutOfBounds(self: Grid, pos: Position) bool {
        return pos.row < 0 or
            pos.row >= self.rows.len or
            pos.col < 0 or
            pos.col >= self.rows[0].len;
    }

    pub fn mark(self: *Grid, pos: Position, marker: u8) void {
        if (!self.isOutOfBounds(pos)) {
            self.rows[@intCast(pos.row)][@intCast(pos.col)] = marker;
        }
    }

    pub fn canMove(self: Grid, pos: Position) bool {
        if (self.isOutOfBounds(pos)) {
            return false;
        }
        return self.rows[@intCast(pos.row)][@intCast(pos.col)] != '#';
    }
};

fn solvePart1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var grid = try Grid.init(allocator, input);
    defer grid.deinit();

    const start_pos = grid.findChar('^') orelse return error.CharacterNotFound;
    var pos = Position{ .row = @intCast(start_pos[0]), .col = @intCast(start_pos[1]) };
    var current_dir = Direction.north;

    while (true) {
        const move = current_dir.getMove();
        const next_pos = Position{ .row = pos.row + move[0], .col = pos.col + move[1] };

        if (grid.isOutOfBounds(next_pos)) {
            break;
        } else if (grid.canMove(next_pos)) {
            pos = next_pos;
            grid.mark(pos, 'X');
        } else {
            current_dir = current_dir.turnRight();
        }
    }

    const total = grid.countChar('X');
    return total;
}

fn detectLoop(grid: Grid, start_pos: Position, start_dir: Direction, visited: *std.AutoHashMap(GuardState, void)) !bool {
    var pos = start_pos;
    var current_dir = start_dir;

    // reusing the hashmap for each check cuts the runtime in half...
    visited.clearRetainingCapacity();

    while (true) {
        const state = GuardState{ .pos = pos, .dir = current_dir };
        if (visited.contains(state) == true) {
            return true;
        }
        try visited.put(state, {});

        const move = current_dir.getMove();
        const next_pos = Position{ .row = pos.row + move[0], .col = pos.col + move[1] };

        if (grid.isOutOfBounds(next_pos)) {
            break;
        } else if (grid.canMove(next_pos)) {
            pos = next_pos;
        } else {
            current_dir = current_dir.turnRight();
        }
    }
    return false;
}

fn solvePart2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var grid = try Grid.init(allocator, input);
    defer grid.deinit();

    const start_pos = grid.findChar('^') orelse return error.CharacterNotFound;
    const start_dir = Direction.north;
    const pos = Position{ .row = @intCast(start_pos[0]), .col = @intCast(start_pos[1]) };

    var total: u64 = 0;
    var visited = std.AutoHashMap(GuardState, void).init(grid.allocator);
    defer visited.deinit();

    for (grid.rows, 0..) |row, row_idx| {
        for (row, 0..) |char, col_idx| {
            if (char == '.') {
                grid.mark(Position{ .row = @intCast(row_idx), .col = @intCast(col_idx) }, '#');
                if (try detectLoop(grid, pos, start_dir, &visited)) {
                    total += 1;
                }
                grid.mark(Position{ .row = @intCast(row_idx), .col = @intCast(col_idx) }, '.');
            }
        }
    }

    return total;
}

test "part 1 example" {
    const example =
        \\....#.....
        \\.........#
        \\..........
        \\..#.......
        \\.......#..
        \\..........
        \\.#..^.....
        \\........#.
        \\#.........
        \\......#...
    ;
    try std.testing.expectEqual(@as(u64, 41), try solvePart1(std.testing.allocator, example));
}

test "part 2 example" {
    const example =
        \\....#.....
        \\.........#
        \\..........
        \\..#.......
        \\.......#..
        \\..........
        \\.#..^.....
        \\........#.
        \\#.........
        \\......#...
    ;

    try std.testing.expectEqual(@as(u64, 6), try solvePart2(std.testing.allocator, example));
}
