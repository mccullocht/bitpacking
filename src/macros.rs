macro_rules! pack_unpack_with_bits {

    ($name:ident, $n:expr, $cpufeature:meta) => {


        mod $name {

            use crunchy::unroll;
            use super::BLOCK_LEN;
            use super::{Sink, Transformer};
            use super::{DataType,
                set1,
                right_shift_32,
                left_shift_32,
                left_shift_insert_32,
                op_or,
                op_and,
                load_unaligned,
                store_unaligned};

            const NUM_BITS: usize = $n;
            const NUM_BYTES_PER_BLOCK: usize = NUM_BITS * BLOCK_LEN / 8;

            #[$cpufeature]
            pub(crate) unsafe fn pack<TDeltaComputer: Transformer>(input_arr: &[u32], output_arr: &mut [u8], mut delta_computer: TDeltaComputer) -> usize {
                assert_eq!(input_arr.len(), BLOCK_LEN, "Input block too small {}, (expected {})", input_arr.len(), BLOCK_LEN);
                assert!(output_arr.len() >= NUM_BYTES_PER_BLOCK, "Output array too small (numbits {}). {} <= {}", NUM_BITS, output_arr.len(), NUM_BYTES_PER_BLOCK);

                if NUM_BITS == 1 {
                    return pack1(input_arr, output_arr, delta_computer);
                }

                let input_ptr = input_arr.as_ptr() as *const DataType;
                let mut output_ptr = output_arr.as_mut_ptr() as *mut DataType;
                let mut out_register: DataType = delta_computer.transform(load_unaligned(input_ptr));

                unroll! {
                    for iter in 0..30 {
                        const i: usize = 1 + iter;

                        const bits_filled: usize = i * NUM_BITS;
                        const inner_cursor: usize = bits_filled % 32;
                        const remaining: usize = 32 - inner_cursor;

                        let offset_ptr = input_ptr.add(i);
                        let in_register: DataType = delta_computer.transform(load_unaligned(offset_ptr));

                        out_register =
                            if inner_cursor > 0 {
                                left_shift_insert_32::<{inner_cursor as i32}>(out_register, in_register)
                            } else {
                                in_register
                            };

                        if remaining <= NUM_BITS {
                            store_unaligned(output_ptr, out_register);
                            output_ptr = output_ptr.offset(1);
                            if 0 < remaining && remaining < NUM_BITS {
                                out_register = right_shift_32::<{remaining as i32}>(in_register);
                            }
                        }
                    }
                }
                let in_register: DataType = delta_computer.transform(load_unaligned(input_ptr.add(31)));
                out_register = if 32 - NUM_BITS > 0 {
                    left_shift_insert_32::<{32 - NUM_BITS as i32}>(out_register, in_register)
                } else {
                    op_or(out_register, in_register)
                };
                store_unaligned(output_ptr, out_register);

                NUM_BYTES_PER_BLOCK
            }

            #[$cpufeature]
            #[inline]
            pub(crate) unsafe fn pack1<TDeltaComputer: Transformer>(input_arr: &[u32], output_arr: &mut [u8], mut delta_computer: TDeltaComputer) -> usize {
                assert_eq!(input_arr.len(), BLOCK_LEN, "Input block too small {}, (expected {})", input_arr.len(), BLOCK_LEN);
                assert!(output_arr.len() >= NUM_BYTES_PER_BLOCK, "Output array too small (numbits {}). {} <= {}", NUM_BITS, output_arr.len(), NUM_BYTES_PER_BLOCK);

                let input_ptr = input_arr.as_ptr() as *const DataType;
                let output_ptr = output_arr.as_mut_ptr() as *mut DataType;
                let mut r0: DataType = delta_computer.transform(load_unaligned(input_ptr));
                r0 = left_shift_insert_32::<1>(r0, delta_computer.transform(load_unaligned(input_ptr.add(1))));

                let mut r2 = delta_computer.transform(load_unaligned(input_ptr.add(2)));
                r2 = left_shift_insert_32::<1>(r2, delta_computer.transform(load_unaligned(input_ptr.add(3))));
                r0 = left_shift_insert_32::<2>(r0, r2);

                let mut r4 = delta_computer.transform(load_unaligned(input_ptr.add(4)));
                r4 = left_shift_insert_32::<1>(r4, delta_computer.transform(load_unaligned(input_ptr.add(5))));

                let mut r6 = delta_computer.transform(load_unaligned(input_ptr.add(6)));
                r6 = left_shift_insert_32::<1>(r6, delta_computer.transform(load_unaligned(input_ptr.add(7))));
                r4 = left_shift_insert_32::<2>(r4, r6);
                r0 = left_shift_insert_32::<4>(r0, r4);

                let mut r8 = delta_computer.transform(load_unaligned(input_ptr.add(8)));
                r8 = left_shift_insert_32::<1>(r8, delta_computer.transform(load_unaligned(input_ptr.add(9))));

                let mut r10 = delta_computer.transform(load_unaligned(input_ptr.add(10)));
                r10 = left_shift_insert_32::<1>(r10, delta_computer.transform(load_unaligned(input_ptr.add(11))));
                r8 = left_shift_insert_32::<2>(r8, r10);

                let mut r12 = delta_computer.transform(load_unaligned(input_ptr.add(12)));
                r12 = left_shift_insert_32::<1>(r12, delta_computer.transform(load_unaligned(input_ptr.add(13))));

                let mut r14 = delta_computer.transform(load_unaligned(input_ptr.add(14)));
                r14 = left_shift_insert_32::<1>(r14, delta_computer.transform(load_unaligned(input_ptr.add(15))));
                r12 = left_shift_insert_32::<2>(r12, r14);
                r8 = left_shift_insert_32::<4>(r8, r12);
                r0 = left_shift_insert_32::<8>(r0, r8);

                let mut r16 = delta_computer.transform(load_unaligned(input_ptr.add(16)));
                r16 = left_shift_insert_32::<1>(r16, delta_computer.transform(load_unaligned(input_ptr.add(17))));

                let mut r18 = delta_computer.transform(load_unaligned(input_ptr.add(18)));
                r18 = left_shift_insert_32::<1>(r18, delta_computer.transform(load_unaligned(input_ptr.add(19))));
                r16 = left_shift_insert_32::<2>(r16, r18);

                let mut r20 = delta_computer.transform(load_unaligned(input_ptr.add(20)));
                r20 = left_shift_insert_32::<1>(r20, delta_computer.transform(load_unaligned(input_ptr.add(21))));

                let mut r22 = delta_computer.transform(load_unaligned(input_ptr.add(22)));
                r22 = left_shift_insert_32::<1>(r22, delta_computer.transform(load_unaligned(input_ptr.add(23))));
                r20 = left_shift_insert_32::<2>(r20, r22);
                r16 = left_shift_insert_32::<4>(r16, r20);

                let mut r24 = delta_computer.transform(load_unaligned(input_ptr.add(24)));
                r24 = left_shift_insert_32::<1>(r24, delta_computer.transform(load_unaligned(input_ptr.add(25))));

                let mut r26 = delta_computer.transform(load_unaligned(input_ptr.add(26)));
                r26 = left_shift_insert_32::<1>(r26, delta_computer.transform(load_unaligned(input_ptr.add(27))));
                r24 = left_shift_insert_32::<2>(r24, r26);

                let mut r28 = delta_computer.transform(load_unaligned(input_ptr.add(28)));
                r28 = left_shift_insert_32::<1>(r28, delta_computer.transform(load_unaligned(input_ptr.add(29))));

                let mut r30 = delta_computer.transform(load_unaligned(input_ptr.add(30)));
                r30 = left_shift_insert_32::<1>(r30, delta_computer.transform(load_unaligned(input_ptr.add(31))));
                r28 = left_shift_insert_32::<2>(r28, r30);
                r24 = left_shift_insert_32::<4>(r24, r28);
                r16 = left_shift_insert_32::<8>(r16, r24);

                r0 = left_shift_insert_32::<16>(r0, r16);

                store_unaligned(output_ptr, r0);

                NUM_BYTES_PER_BLOCK
            }

            #[$cpufeature]
            pub(crate) unsafe fn unpack<Output: Sink>(compressed: &[u8], mut output: Output) -> usize {

                assert!(compressed.len() >= NUM_BYTES_PER_BLOCK, "Compressed array seems too small. ({} < {}) ", compressed.len(), NUM_BYTES_PER_BLOCK);

                let mut input_ptr = compressed.as_ptr() as *const DataType;

                let mask_scalar: u32 = ((1u64 << NUM_BITS) - 1u64) as u32;
                let mask = set1(mask_scalar as i32);

                let mut in_register: DataType = load_unaligned(input_ptr);

                let out_register = op_and(in_register, mask);
                output.process(out_register);

                unroll! {
                    for iter in 0..31 {
                        const i: usize = iter + 1;

                        const inner_cursor: usize = (i * NUM_BITS) % 32;
                        const inner_capacity: usize = 32 - inner_cursor;

                        let shifted_in_register = if inner_cursor != 0 {
                            right_shift_32::<{inner_cursor as i32}>(in_register)
                        } else {
                            in_register
                        };
                        let mut out_register: DataType = op_and(shifted_in_register, mask);

                        // We consumed our current quadruplets entirely.
                        // We therefore read another one.
                        if inner_capacity <= NUM_BITS && i != 31 {
                            input_ptr = input_ptr.add(1);
                            in_register = load_unaligned(input_ptr);

                            // This quadruplets is actually cutting one of
                            // our `DataType`. We need to read the next one.
                            if inner_capacity < NUM_BITS {
                                let shifted = if inner_capacity != 0 {
                                    left_shift_32::<{inner_capacity as i32}>(in_register)
                                } else {
                                    in_register
                                };
                                let masked = op_and(shifted, mask);
                                out_register = op_or(out_register, masked);
                            }
                        }

                        output.process(out_register);
                    }
                }


                NUM_BYTES_PER_BLOCK
            }
        }
    }
}

macro_rules! pack_unpack_with_bits_32 {
    ($cpufeature:meta) => {
        mod pack_unpack_with_bits_32 {
            use super::BLOCK_LEN;
            use super::{load_unaligned, store_unaligned, DataType};
            use super::{Sink, Transformer};
            use crunchy::unroll;

            const NUM_BITS: usize = 32;
            const NUM_BYTES_PER_BLOCK: usize = NUM_BITS * BLOCK_LEN / 8;

            #[$cpufeature]
            pub(crate) unsafe fn pack<TDeltaComputer: Transformer>(
                input_arr: &[u32],
                output_arr: &mut [u8],
                mut delta_computer: TDeltaComputer,
            ) -> usize {
                assert_eq!(
                    input_arr.len(),
                    BLOCK_LEN,
                    "Input block too small {}, (expected {})",
                    input_arr.len(),
                    BLOCK_LEN
                );
                assert!(
                    output_arr.len() >= NUM_BYTES_PER_BLOCK,
                    "Output array too small (numbits {}). {} <= {}",
                    NUM_BITS,
                    output_arr.len(),
                    NUM_BYTES_PER_BLOCK
                );

                let input_ptr: *const DataType = input_arr.as_ptr() as *const DataType;
                let output_ptr = output_arr.as_mut_ptr() as *mut DataType;
                unroll! {
                    for i in 0..32 {
                        let input_offset_ptr = input_ptr.offset(i as isize);
                        let output_offset_ptr = output_ptr.offset(i as isize);
                        let input_register = load_unaligned(input_offset_ptr);
                        let output_register = delta_computer.transform(input_register);
                        store_unaligned(output_offset_ptr, output_register);
                    }
                }
                NUM_BYTES_PER_BLOCK
            }

            #[$cpufeature]
            pub(crate) unsafe fn unpack<Output: Sink>(
                compressed: &[u8],
                mut output: Output,
            ) -> usize {
                assert!(
                    compressed.len() >= NUM_BYTES_PER_BLOCK,
                    "Compressed array seems too small. ({} < {}) ",
                    compressed.len(),
                    NUM_BYTES_PER_BLOCK
                );
                let input_ptr = compressed.as_ptr() as *const DataType;
                for i in 0..32 {
                    let input_offset_ptr = input_ptr.offset(i as isize);
                    let in_register: DataType = load_unaligned(input_offset_ptr);
                    output.process(in_register);
                }
                NUM_BYTES_PER_BLOCK
            }
        }
    };
}

macro_rules! declare_bitpacker {
    ($cpufeature:meta) => {
        use super::super::UnsafeBitPacker;
        use crate::most_significant_bit;
        use crunchy::unroll;

        pack_unpack_with_bits!(pack_unpack_with_bits_1, 1, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_2, 2, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_3, 3, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_4, 4, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_5, 5, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_6, 6, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_7, 7, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_8, 8, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_9, 9, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_10, 10, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_11, 11, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_12, 12, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_13, 13, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_14, 14, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_15, 15, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_16, 16, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_17, 17, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_18, 18, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_19, 19, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_20, 20, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_21, 21, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_22, 22, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_23, 23, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_24, 24, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_25, 25, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_26, 26, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_27, 27, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_28, 28, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_29, 29, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_30, 30, $cpufeature);
        pack_unpack_with_bits!(pack_unpack_with_bits_31, 31, $cpufeature);
        pack_unpack_with_bits_32!($cpufeature);

        unsafe fn compress_generic<DeltaComputer: Transformer>(
            decompressed: &[u32],
            compressed: &mut [u8],
            num_bits: u8,
            delta_computer: DeltaComputer,
        ) -> usize {
            match num_bits {
                0 => 0,
                1 => pack_unpack_with_bits_1::pack(decompressed, compressed, delta_computer),
                2 => pack_unpack_with_bits_2::pack(decompressed, compressed, delta_computer),
                3 => pack_unpack_with_bits_3::pack(decompressed, compressed, delta_computer),
                4 => pack_unpack_with_bits_4::pack(decompressed, compressed, delta_computer),
                5 => pack_unpack_with_bits_5::pack(decompressed, compressed, delta_computer),
                6 => pack_unpack_with_bits_6::pack(decompressed, compressed, delta_computer),
                7 => pack_unpack_with_bits_7::pack(decompressed, compressed, delta_computer),
                8 => pack_unpack_with_bits_8::pack(decompressed, compressed, delta_computer),
                9 => pack_unpack_with_bits_9::pack(decompressed, compressed, delta_computer),
                10 => pack_unpack_with_bits_10::pack(decompressed, compressed, delta_computer),
                11 => pack_unpack_with_bits_11::pack(decompressed, compressed, delta_computer),
                12 => pack_unpack_with_bits_12::pack(decompressed, compressed, delta_computer),
                13 => pack_unpack_with_bits_13::pack(decompressed, compressed, delta_computer),
                14 => pack_unpack_with_bits_14::pack(decompressed, compressed, delta_computer),
                15 => pack_unpack_with_bits_15::pack(decompressed, compressed, delta_computer),
                16 => pack_unpack_with_bits_16::pack(decompressed, compressed, delta_computer),
                17 => pack_unpack_with_bits_17::pack(decompressed, compressed, delta_computer),
                18 => pack_unpack_with_bits_18::pack(decompressed, compressed, delta_computer),
                19 => pack_unpack_with_bits_19::pack(decompressed, compressed, delta_computer),
                20 => pack_unpack_with_bits_20::pack(decompressed, compressed, delta_computer),
                21 => pack_unpack_with_bits_21::pack(decompressed, compressed, delta_computer),
                22 => pack_unpack_with_bits_22::pack(decompressed, compressed, delta_computer),
                23 => pack_unpack_with_bits_23::pack(decompressed, compressed, delta_computer),
                24 => pack_unpack_with_bits_24::pack(decompressed, compressed, delta_computer),
                25 => pack_unpack_with_bits_25::pack(decompressed, compressed, delta_computer),
                26 => pack_unpack_with_bits_26::pack(decompressed, compressed, delta_computer),
                27 => pack_unpack_with_bits_27::pack(decompressed, compressed, delta_computer),
                28 => pack_unpack_with_bits_28::pack(decompressed, compressed, delta_computer),
                29 => pack_unpack_with_bits_29::pack(decompressed, compressed, delta_computer),
                30 => pack_unpack_with_bits_30::pack(decompressed, compressed, delta_computer),
                31 => pack_unpack_with_bits_31::pack(decompressed, compressed, delta_computer),
                32 => pack_unpack_with_bits_32::pack(decompressed, compressed, delta_computer),
                _ => {
                    panic!("Num bits must be <= 32. Was {}.", num_bits);
                }
            }
        }

        pub trait Transformer {
            unsafe fn transform(&mut self, data: DataType) -> DataType;
        }

        struct NoDelta;

        impl Transformer for NoDelta {
            #[inline]
            unsafe fn transform(&mut self, current: DataType) -> DataType {
                current
            }
        }

        struct DeltaComputer {
            pub previous: DataType,
        }

        impl Transformer for DeltaComputer {
            #[inline]
            unsafe fn transform(&mut self, current: DataType) -> DataType {
                let result = compute_delta(current, self.previous);
                self.previous = current;
                result
            }
        }

        pub trait Sink {
            unsafe fn process(&mut self, data_type: DataType);
        }

        struct Store {
            output_ptr: *mut DataType,
        }

        impl Store {
            fn new(output_ptr: *mut DataType) -> Store {
                Store { output_ptr }
            }
        }

        struct DeltaIntegrate {
            current: DataType,
            output_ptr: *mut DataType,
        }

        impl DeltaIntegrate {
            unsafe fn new(initial: u32, output_ptr: *mut DataType) -> DeltaIntegrate {
                DeltaIntegrate {
                    current: set1(initial as i32),
                    output_ptr,
                }
            }
        }

        impl Sink for DeltaIntegrate {
            #[inline]
            unsafe fn process(&mut self, delta: DataType) {
                self.current = integrate_delta(self.current, delta);
                store_unaligned(self.output_ptr, self.current);
                self.output_ptr = self.output_ptr.add(1);
            }
        }

        impl Sink for Store {
            #[inline]
            unsafe fn process(&mut self, out_register: DataType) {
                store_unaligned(self.output_ptr, out_register);
                self.output_ptr = self.output_ptr.add(1);
            }
        }

        #[inline]
        unsafe fn decompress_to<Output: Sink>(
            compressed: &[u8],
            mut sink: Output,
            num_bits: u8,
        ) -> usize {
            match num_bits {
                0 => {
                    let zero = set1(0i32);
                    for _ in 0..32 {
                        sink.process(zero);
                    }
                    0
                }
                1 => pack_unpack_with_bits_1::unpack(compressed, sink),
                2 => pack_unpack_with_bits_2::unpack(compressed, sink),
                3 => pack_unpack_with_bits_3::unpack(compressed, sink),
                4 => pack_unpack_with_bits_4::unpack(compressed, sink),
                5 => pack_unpack_with_bits_5::unpack(compressed, sink),
                6 => pack_unpack_with_bits_6::unpack(compressed, sink),
                7 => pack_unpack_with_bits_7::unpack(compressed, sink),
                8 => pack_unpack_with_bits_8::unpack(compressed, sink),
                9 => pack_unpack_with_bits_9::unpack(compressed, sink),
                10 => pack_unpack_with_bits_10::unpack(compressed, sink),
                11 => pack_unpack_with_bits_11::unpack(compressed, sink),
                12 => pack_unpack_with_bits_12::unpack(compressed, sink),
                13 => pack_unpack_with_bits_13::unpack(compressed, sink),
                14 => pack_unpack_with_bits_14::unpack(compressed, sink),
                15 => pack_unpack_with_bits_15::unpack(compressed, sink),
                16 => pack_unpack_with_bits_16::unpack(compressed, sink),
                17 => pack_unpack_with_bits_17::unpack(compressed, sink),
                18 => pack_unpack_with_bits_18::unpack(compressed, sink),
                19 => pack_unpack_with_bits_19::unpack(compressed, sink),
                20 => pack_unpack_with_bits_20::unpack(compressed, sink),
                21 => pack_unpack_with_bits_21::unpack(compressed, sink),
                22 => pack_unpack_with_bits_22::unpack(compressed, sink),
                23 => pack_unpack_with_bits_23::unpack(compressed, sink),
                24 => pack_unpack_with_bits_24::unpack(compressed, sink),
                25 => pack_unpack_with_bits_25::unpack(compressed, sink),
                26 => pack_unpack_with_bits_26::unpack(compressed, sink),
                27 => pack_unpack_with_bits_27::unpack(compressed, sink),
                28 => pack_unpack_with_bits_28::unpack(compressed, sink),
                29 => pack_unpack_with_bits_29::unpack(compressed, sink),
                30 => pack_unpack_with_bits_30::unpack(compressed, sink),
                31 => pack_unpack_with_bits_31::unpack(compressed, sink),
                32 => pack_unpack_with_bits_32::unpack(compressed, sink),
                _ => {
                    panic!("Num bits must be <= 32. Was {}.", num_bits);
                }
            }
        }

        pub struct UnsafeBitPackerImpl;

        impl UnsafeBitPacker for UnsafeBitPackerImpl {
            const BLOCK_LEN: usize = BLOCK_LEN;

            #[$cpufeature]
            unsafe fn compress(decompressed: &[u32], compressed: &mut [u8], num_bits: u8) -> usize {
                compress_generic(decompressed, compressed, num_bits, NoDelta)
            }

            #[$cpufeature]
            unsafe fn compress_sorted(
                initial: u32,
                decompressed: &[u32],
                compressed: &mut [u8],
                num_bits: u8,
            ) -> usize {
                let delta_computer = DeltaComputer {
                    previous: set1(initial as i32),
                };
                compress_generic(decompressed, compressed, num_bits, delta_computer)
            }

            #[$cpufeature]
            unsafe fn decompress(
                compressed: &[u8],
                decompressed: &mut [u32],
                num_bits: u8,
            ) -> usize {
                assert!(
                    decompressed.len() >= BLOCK_LEN,
                    "The output array is not large enough : ({} >= {})",
                    decompressed.len(),
                    BLOCK_LEN
                );
                let output_ptr = decompressed.as_mut_ptr() as *mut DataType;
                let output = Store::new(output_ptr);
                decompress_to(compressed, output, num_bits)
            }

            #[$cpufeature]
            unsafe fn decompress_sorted(
                initial: u32,
                compressed: &[u8],
                decompressed: &mut [u32],
                num_bits: u8,
            ) -> usize {
                assert!(
                    decompressed.len() >= BLOCK_LEN,
                    "The output array is not large enough : ({} >= {})",
                    decompressed.len(),
                    BLOCK_LEN
                );
                let output_ptr = decompressed.as_mut_ptr() as *mut DataType;
                let output = DeltaIntegrate::new(initial, output_ptr);
                decompress_to(compressed, output, num_bits)
            }

            #[$cpufeature]
            unsafe fn num_bits(decompressed: &[u32]) -> u8 {
                assert_eq!(
                    decompressed.len(),
                    BLOCK_LEN,
                    "`decompressed`'s len is not `BLOCK_LEN={}`",
                    BLOCK_LEN
                );
                let data: *const DataType = decompressed.as_ptr() as *const DataType;
                let mut accumulator = load_unaligned(data);
                unroll! {
                    for iter in 0..31 {
                        let i = iter + 1;
                        let newvec = load_unaligned(data.add(i));
                        accumulator = op_or(accumulator, newvec);
                    }
                }
                most_significant_bit(or_collapse_to_u32(accumulator))
            }

            #[$cpufeature]
            unsafe fn num_bits_sorted(initial: u32, decompressed: &[u32]) -> u8 {
                assert_eq!(
                    decompressed.len(),
                    BLOCK_LEN,
                    "`decompressed`'s len is not `BLOCK_LEN={}`",
                    BLOCK_LEN
                );
                let initial_vec = set1(initial as i32);
                let data: *const DataType = decompressed.as_ptr() as *const DataType;

                let first = load_unaligned(data);
                let mut accumulator = compute_delta(load_unaligned(data), initial_vec);
                let mut previous = first;

                unroll! {
                    for iter in 0..30 {
                        let i = iter + 1;
                        let current = load_unaligned(data.add(i));
                        let delta = compute_delta(current, previous);
                        accumulator =  op_or(accumulator, delta);
                        previous = current;
                    }
                }
                let current = load_unaligned(data.add(31));
                let delta = compute_delta(current, previous);
                accumulator = op_or(accumulator, delta);
                most_significant_bit(or_collapse_to_u32(accumulator))
            }
        }

        #[cfg(test)]
        mod tests {
            use super::UnsafeBitPackerImpl;
            use crate::tests::test_suite_compress_decompress;
            use crate::Available;
            use crate::UnsafeBitPacker;

            #[test]
            fn test_num_bits() {
                if UnsafeBitPackerImpl::available() {
                    for num_bits in 0..32 {
                        for pos in 0..32 {
                            let mut vals = [0u32; UnsafeBitPackerImpl::BLOCK_LEN];
                            if num_bits > 0 {
                                vals[pos] = 1 << (num_bits - 1);
                            }
                            assert_eq!(
                                unsafe { UnsafeBitPackerImpl::num_bits(&vals[..]) },
                                num_bits
                            );
                        }
                    }
                }
            }

            #[test]
            fn test_bitpacker() {
                if UnsafeBitPackerImpl::available() {
                    test_suite_compress_decompress::<UnsafeBitPackerImpl>(false);
                }
            }

            #[test]
            fn test_bitpacker_delta() {
                if UnsafeBitPackerImpl::available() {
                    test_suite_compress_decompress::<UnsafeBitPackerImpl>(true);
                }
            }
        }
    };
}
