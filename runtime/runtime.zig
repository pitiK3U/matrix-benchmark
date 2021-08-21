const std = @import("std");

fn Matrix(comptime T: type, comptime M: usize, comptime N: usize) type {
    return struct {
        const This = @This();
        const inner_type = T;
        const M = M;
        const N = N;

        inner: [M][N]T,

        fn init(comptime value: inner_type) This {
            var a = This{.inner = undefined};
            for (a.inner) |*row| {
                for (row.*) |*cell| {
                    cell.* = value;
                }
            }
            return a;
        }

        fn print(this: *const This) void {
            for (this.inner) |row| {
                for (row) |cell| {
                    std.debug.print("{} ", .{cell});
                }
                std.debug.print("\n", .{});
            }
        }

        fn cross(comptime K: inner_type, left: Matrix(inner_type, M, K), right: Matrix(inner_type, K, N)) This {
            var result = This{.inner = undefined};

            for (result.inner) |*row, row_index| {
                for (row.*) |*cell, col_index| {
                    var sum: inner_type = 0;
                    var j: usize = 0;
                    while (j < K) : (j += 1) {
                        sum += left.inner[row_index][j] * right.inner[j][col_index];
                    }
                    cell.* = sum;
                }
            }
            return result;
        }
    };
}



pub fn main() anyerror!void {
    const left = Matrix(i64, 50, 20).init(5);
    const right = Matrix(i64, 20, 15).init(2);

    const start = std.time.nanoTimestamp();

    var result = Matrix(i64, 50, 15).cross(20, left, right);

    const elapsed = std.time.nanoTimestamp() - start;

    result.print();
    std.debug.print("{} ns\n", .{elapsed});
}
