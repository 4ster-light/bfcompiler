const std = @import("std");

const MAX_PROG_SIZE = 30_000;

const BrainfuckError = error{
    MemoryOutOfBounds,
    UnmatchedBracket,
    FileNotFound,
    InvalidInput,
    OutOfMemory,
    InputOutput,
    AccessDenied,
    BrokenPipe,
    SystemResources,
    OperationAborted,
    WouldBlock,
    ConnectionResetByPeer,
    Unexpected,
    IsDir,
    ConnectionTimedOut,
    NotOpenForReading,
    SocketNotConnected,
    EndOfStream,
    DiskQuota,
    FileTooBig,
    NoSpaceLeft,
    DeviceBusy,
    InvalidArgument,
    NotOpenForWriting,
    LockViolation,
};

fn checkBounds(ptr: usize, array: *[MAX_PROG_SIZE]u8) BrainfuckError!void {
    if (ptr >= array.len) return BrainfuckError.MemoryOutOfBounds;
}

fn findMatchingBrackets(allocator: std.mem.Allocator, code: []const u8) BrainfuckError!std.AutoHashMap(usize, usize) {
    var brackets = std.AutoHashMap(usize, usize).init(allocator);
    var stack = std.ArrayList(usize).init(allocator);
    defer stack.deinit();

    for (code, 0..) |char, i| {
        switch (char) {
            '[' => try stack.append(i),
            ']' => {
                if (stack.items.len == 0) {
                    brackets.deinit();
                    return BrainfuckError.UnmatchedBracket;
                }
                const open_pos = stack.pop();
                try brackets.put(open_pos, i);
                try brackets.put(i, open_pos);
            },
            else => {},
        }
    }

    if (stack.items.len > 0) {
        brackets.deinit();
        return BrainfuckError.UnmatchedBracket;
    }

    return brackets;
}

fn interpretBf(allocator: std.mem.Allocator, code: []const u8) BrainfuckError!void {
    var array = [_]u8{0} ** MAX_PROG_SIZE;
    var ptr: usize = 0;
    var code_ptr: usize = 0;

    var brackets = try findMatchingBrackets(allocator, code);
    defer brackets.deinit();

    while (code_ptr < code.len) {
        try checkBounds(ptr, &array);

        switch (code[code_ptr]) {
            '+' => array[ptr] +%= 1,
            '-' => array[ptr] -%= 1,
            '<' => ptr = if (ptr > 0) ptr - 1 else 0,
            '>' => ptr += 1,
            ',' => {
                const stdin = std.io.getStdIn().reader();
                const input = stdin.readByte() catch |err| {
                    if (err == error.EndOfStream) {
                        array[ptr] = 0;
                        continue;
                    }
                    return err;
                };
                array[ptr] = input;
            },
            '.' => try std.io.getStdOut().writer().writeByte(array[ptr]),
            '[' => if (array[ptr] == 0) {
                code_ptr = brackets.get(code_ptr).?;
            },
            ']' => if (array[ptr] != 0) {
                code_ptr = brackets.get(code_ptr).?;
            },
            else => {},
        }

        code_ptr += 1;
    }
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len != 2) {
        try std.io.getStdErr().writer().print("Usage: {s} <filename>\n", .{args[0]});
        std.process.exit(1);
    }

    const file = std.fs.cwd().openFile(args[1], .{}) catch {
        try std.io.getStdErr().writer().print("Error: Could not open file '{s}'\n", .{args[1]});
        std.process.exit(1);
    };
    defer file.close();

    const code = try file.readToEndAlloc(allocator, 1024 * 1024);
    defer allocator.free(code);

    interpretBf(allocator, code) catch |err| {
        try std.io.getStdErr().writer().print("Error: {s}\n", .{@errorName(err)});
        std.process.exit(1);
    };
}
