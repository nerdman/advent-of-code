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

fn parseFilesystem(input: []const u8, blocks: *std.ArrayList(?usize)) !void {
    var is_file = true;
    var file_id: usize = 0;
    for (input) |digit| {
        const block_size = digit - '0';
        if (is_file) {
            try blocks.appendNTimes(file_id, block_size);
            file_id += 1;
            is_file = false;
        } else {
            if (block_size > 0) {
                try blocks.appendNTimes(null, block_size);
            }
            is_file = true;
        }
    }
}

fn solvePart1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var blocks = std.ArrayList(?usize).init(allocator);
    defer blocks.deinit();

    try parseFilesystem(input, &blocks);

    var fwd_idx: usize = 0;
    var end_idx: usize = blocks.items.len - 1;

    while (blocks.items[fwd_idx] != null) {
        fwd_idx += 1;
    }

    while (blocks.items[end_idx] == null) {
        end_idx -= 1;
    }

    while (end_idx > fwd_idx) {
        blocks.items[fwd_idx] = blocks.items[end_idx];
        blocks.items[end_idx] = null;

        while (blocks.items[end_idx] == null) {
            end_idx -= 1;
        }

        while (blocks.items[fwd_idx] != null) {
            fwd_idx += 1;
        }
    }

    var total: u64 = 0;
    for (blocks.items, 0..) |block, i| {
        if (block == null) break;

        total += i * block.?;
    }

    return total;
}

const File = struct {
    size: usize,
    id: usize,
    start_index: usize,
};

const FilesystemIterator = struct {
    filesystem: []?usize,
    index: usize,

    fn next(self: *FilesystemIterator) ?File {
        if (self.index == 0) return null;

        // find the next non-null file id
        while (self.index > 0 and self.filesystem[self.index] == null) {
            self.index -= 1;
        }

        // set the current id
        const next_file_id = self.filesystem[self.index];
        const end_index = self.index;
        while (self.index > 0) {
            if (self.filesystem[self.index] != next_file_id) break;
            // final index will be positioned one past the current file
            self.index -= 1;
        }
        if (self.index == 0) return null;

        if (next_file_id != null) {
            return File{ .id = next_file_id.?, .size = end_index - self.index, .start_index = self.index + 1 };
        }
        return null;
    }
};

fn findSpaceOfSize(blocks: []?usize, file_size: usize) ?usize {
    var free_space: usize = 0;

    var start: usize = 0;
    for (blocks, 0..) |block, i| {
        if (block == null) {
            if (start == 0) {
                start = i;
            }
            free_space += 1;
            if (free_space == file_size) return start;
        } else {
            free_space = 0;
            start = 0;
        }
    }
    return null;
}

fn solvePart2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    var blocks = std.ArrayList(?usize).init(allocator);
    defer blocks.deinit();

    try parseFilesystem(input, &blocks);

    var iter = FilesystemIterator{
        .filesystem = blocks.items,
        .index = blocks.items.len - 1,
    };

    while (iter.next()) |file| {
        const free_space = findSpaceOfSize(blocks.items[0..file.start_index], file.size);

        if (free_space != null) {
            for (0..file.size) |i| {
                blocks.items[free_space.? + i] = blocks.items[file.start_index + i];
                blocks.items[file.start_index + i] = null;
            }
        }
    }

    var total: u64 = 0;
    for (blocks.items, 0..) |block, i| {
        if (block != null) {
            total += i * block.?;
        }
    }

    return total;
}

test "part 1 example" {
    const example = "2333133121414131402";
    try std.testing.expectEqual(@as(u64, 1928), try solvePart1(std.testing.allocator, example));
}

test "part 2 example" {
    const example = "2333133121414131402";

    try std.testing.expectEqual(@as(u64, 2858), try solvePart2(std.testing.allocator, example));
}
