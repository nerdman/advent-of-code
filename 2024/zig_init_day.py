#!/usr/bin/env python3
import os
import sys
import requests
from pathlib import Path
from datetime import datetime
from dotenv import load_dotenv

def create_zig_template():
    return '''const std = @import("std");
const print = std.debug.print;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const input = @embedFile("input");

    // Part 1
    const part1 = try solvePart1(allocator, input);
    print("Part 1: {d}\\n", .{part1});

    // Part 2
    const part2 = try solvePart2(allocator, input);
    print("Part 2: {d}\\n", .{part2});
}

fn solvePart1(allocator: std.mem.Allocator, input: []const u8) !u64 {
    _ = allocator;
    _ = input;
    return 0;
}

fn solvePart2(allocator: std.mem.Allocator, input: []const u8) !u64 {
    _ = allocator;
    _ = input;
    return 0;
}

test "part 1 example" {
    const example = "";
    try std.testing.expectEqual(@as(u64, 0), try solvePart1(std.testing.allocator, example));
}

test "part 2 example" {
    const example = "";
    try std.testing.expectEqual(@as(u64, 0), try solvePart2(std.testing.allocator, example));
}
'''

def create_build_template():
    return '''const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "solution",
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });
    exe.root_module.addAnonymousImport(
        "input",
        .{
            .root_source_file = b.path("input.txt")
        },
    );

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    const unit_tests = b.addTest(.{
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    const run_unit_tests = b.addRunArtifact(unit_tests);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&run_unit_tests.step);
}
'''

def setup_aoc_day(year: int, day: int):
    load_dotenv()
    session_token = os.getenv('AOC_SESSION_TOKEN')
    if not session_token:
        print("Error: AOC_SESSION_TOKEN not found in .env file")
        sys.exit(1)

    # Create directory structure
    base_dir = Path(f"day{day:02d}")
    src_dir = base_dir / "src"
    src_dir.mkdir(parents=True, exist_ok=True)

    # Download input
    url = f"https://adventofcode.com/{year}/day/{day}/input"
    headers = {"Cookie": f"session={session_token}"}
    
    try:
        response = requests.get(url, headers=headers)
        response.raise_for_status()
        input_path = base_dir / "input.txt"
        input_path.write_text(response.text.rstrip())
        
        print(f"Downloaded input to {input_path}")
        
    except requests.exceptions.RequestException as e:
        print(f"Error downloading input: {e}")
        sys.exit(1)

    # Create main.zig
    main_path = src_dir / "main.zig"
    main_path.write_text(create_zig_template())
    print(f"Created {main_path}")

    # Create build.zig
    build_path = base_dir / "build.zig"
    build_path.write_text(create_build_template())
    print(f"Created {build_path}")

if __name__ == "__main__":
    # Default to current year and day if not specified
    today = datetime.now()
    year = today.year
    day = today.day

    if len(sys.argv) > 2:
        year = int(sys.argv[1])
        day = int(sys.argv[2])
    elif len(sys.argv) > 1:
        day = int(sys.argv[1])

    setup_aoc_day(year, day)
    print(f"\nSetup complete for Advent of Code {year} Day {day}")

