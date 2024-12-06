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

pub fn createGrid(allocator: std.mem.Allocator, input: []const u8) ![][]u8 {
    // find dimensions of ascii grid
    const cols = std.mem.indexOfScalar(u8, input, '\n') orelse return error.InvalidFormat;
    const rows = (input.len + 1) / (cols + 1);

    // row pointers for grid
    const grid = try allocator.alloc([]u8, rows);
    errdefer allocator.free(grid);

    // one block for all data
    const data = try allocator.alloc(u8, rows * cols);
    errdefer allocator.free(data);

    // row slices to point into the data block
    var i: usize = 0;
    while (i < rows) : (i += 1) {
        grid[i] = data[i * cols .. (i + 1) * cols];
    }

    // copy the input data into the grid
    var row: usize = 0;
    while (row < rows) : (row += 1) {
        const src_start = row * (cols + 1);
        @memcpy(grid[row], input[src_start..][0..cols]);
    }

    return grid;
}

pub fn freeGrid(allocator: std.mem.Allocator, grid: [][]u8) void {
    if (grid.len > 0) {
        const data = grid[0].ptr[0 .. grid.len * grid[0].len];

        allocator.free(data);
    }
    allocator.free(grid);
}

fn checkDirection(grid: [][]u8, row: usize, col: usize, delta_row: isize, delta_col: isize) bool {
    const mas = "MAS";
    var cur_row: isize = @intCast(row);
    var cur_col: isize = @intCast(col);

    for (mas) |letter| {
        cur_row += delta_row;
        cur_col += delta_col;
        if (cur_row < 0 or cur_row >= grid.len) return false;

        if (cur_col < 0 or cur_col >= grid[0].len) return false;

        if (grid[@intCast(cur_row)][@intCast(cur_col)] != letter) return false;
    }

    return true;
}

fn solvePart1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    const grid = try createGrid(allocator, input);
    defer freeGrid(allocator, grid);

    const directions = [_][2]isize{
        .{ 0, -1 }, // left
        .{ 0, 1 }, // right
        .{ -1, 0 }, // up
        .{ 1, 0 }, // down
        .{ -1, -1 }, // up left
        .{ -1, 1 }, // up right
        .{ 1, -1 }, // down left
        .{ 1, 1 }, // down right
    };

    const rows = grid.len;
    const cols = grid[0].len;
    var total: u64 = 0;
    for (0..rows) |r| {
        for (0..cols) |c| {
            if (grid[r][c] == 'X') {
                for (directions) |dir| {
                    if (checkDirection(grid, r, c, dir[0], dir[1])) {
                        total += 1;
                    }
                }
            }
        }
    }
    return total;
}

fn solvePart2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    const grid = try createGrid(allocator, input);
    defer freeGrid(allocator, grid);

    const positions = [_][2]isize{ .{ -1, -1 }, .{ -1, 1 }, .{ 1, -1 }, .{ 1, 1 } };

    const patterns = [_][4]u8{
        .{ 'M', 'M', 'S', 'S' },
        .{ 'M', 'S', 'M', 'S' },
        .{ 'S', 'M', 'S', 'M' },
        .{ 'S', 'S', 'M', 'M' },
    };

    var total: u64 = 0;

    const rows = grid.len;
    const cols = grid[0].len;

    for (0..rows) |r| {
        for (0..cols) |c| {
            if (grid[r][c] == 'A') {
                for (patterns) |pattern| {
                    var matched: bool = true;

                    for (0..pattern.len) |i| {
                        const next_r = @as(isize, @intCast(r)) + positions[i][0];
                        const next_c = @as(isize, @intCast(c)) + positions[i][1];
                        if (next_r < 0 or next_r >= rows or
                            next_c < 0 or next_c >= cols or
                            grid[@intCast(next_r)][@intCast(next_c)] != pattern[i])
                        {
                            matched = false;
                            break;
                        }
                    }
                    if (matched) {
                        total += 1;
                    }
                }
            }
        }
    }
    return total;
}

test "part 1 example" {
    const example =
        \\MMMSXXMASM
        \\MSAMXMSMSA
        \\AMXSXMAAMM
        \\MSAMASMSMX
        \\XMASAMXAMM
        \\XXAMMXXAMA
        \\SMSMSASXSS
        \\SAXAMASAAA
        \\MAMMMXMMMM
        \\MXMXAXMASX
    ;
    try std.testing.expectEqual(@as(u64, 18), try solvePart1(std.testing.allocator, example));
}

test "part 2 example" {
    const example =
        \\MMMSXXMASM
        \\MSAMXMSMSA
        \\AMXSXMAAMM
        \\MSAMASMSMX
        \\XMASAMXAMM
        \\XXAMMXXAMA
        \\SMSMSASXSS
        \\SAXAMASAAA
        \\MAMMMXMMMM
        \\MXMXAXMASX
    ;

    try std.testing.expectEqual(@as(u64, 9), try solvePart2(std.testing.allocator, example));
}
