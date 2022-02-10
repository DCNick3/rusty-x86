mod mov {
    test_snippets! {

        /* test name */
        mov_eax_42: (
            /* test body */
            ; mov eax, 42

        /* optional list of flags to test */
        ) [CF ZF SF OF],
        mov_ebx_42: (
            ; mov ebx, 42
        ) [CF ZF SF OF],

        mov_al_42: (
            ; mov al, 42
        ) [CF ZF SF OF],

        mov_al_42_dirty: (
            ; mov eax, 0x41424344
            ; mov al, 42
        ) [CF ZF SF OF],

        mov_ax_42_dirty: (
            ; mov eax, 0x41424344
            ; mov ax, 42
        ) [CF ZF SF OF],

        // mov_ah_42_dirty: (
        //     ; mov eax, 0x41424344
        //     ; mov ah, 42
        // ) [CF ZF SF OF],
    }
}

// TODO: do we test all flag combinations (ditto for and)?
mod sub {
    test_snippets! {
        sub_borrow: (
            ; mov eax, 1
            ; sub eax, 2
        ) [CF ZF SF OF],
        sub_branch_sign: (
            ; mov eax, 1
            ; sub eax, 2
            ; js ->L1 // TODO: cmov is more concise?
            ; mov ebx, 1
            ; jmp ->R
            ; ->L1:
            ; mov ebx, 2
            ; ->R:
            ; mov edx, 1 // necessary because of funky control flow at the end of test snippets...
        ) [CF ZF SF OF],
        sub_cmov_sign: (
            ; mov eax, 1
            ; sub eax, 2
            ; mov ecx, 2
            ; cmovs ebx, ecx
        ) [CF ZF SF OF],
        sub_cmov_sign_2: (
            ; mov eax, 3
            ; sub eax, 2
            ; mov ecx, 2
            ; cmovs ebx, ecx
        ) [CF ZF SF OF],
    }
}

mod add {
    test_snippets! {
        add_borrow: (
            ; mov eax, 1
            ; add eax, 2
        ) [CF ZF SF OF],
        add_branch_sign: (
            ; mov eax, 1
            ; add eax, 2
            ; js ->L1 // TODO: cmov is more concise?
            ; mov ebx, 1
            ; jmp ->R
            ; ->L1:
            ; mov ebx, 2
            ; ->R:
            ; mov edx, 1 // necessary because of funky control flow at the end of test snippets...
        ) [CF ZF SF OF],
        add_cmov_sign: (
            ; mov eax, 1
            ; add eax, 2
            ; mov ecx, 2
            ; cmovs ebx, ecx
        ) [CF ZF SF OF],
        add_cmov_sign_2: (
            ; mov eax, 3
            ; add eax, 2
            ; mov ecx, 2
            ; cmovs ebx, ecx
        ) [CF ZF SF OF],
    }
}

mod cmp {
    test_snippets! {
        cmp_cmov_eq: (
            ; mov eax, 12
            ; cmp eax, 12
            ; mov ecx, 2
            ; cmovz ebx, ecx
        ) [CF ZF SF OF],
        cmp_cmov_eq_2: (
            ; mov eax, 12
            ; cmp eax, 13
            ; mov ecx, 2
            ; cmovz ebx, ecx
        ) [CF ZF SF OF],
        cmp_less: (
            ; mov eax, 11
            ; cmp eax, 13
        ) [CF ZF SF OF],
        cmp_neg_1: (
            ; mov eax, -1
            ; cmp eax, -2
        ) [CF ZF SF OF],
        cmp_neg_2: (
            ; mov eax, 0
            ; cmp eax, 1
        ) [CF ZF SF OF],
        cmp_neg_3: (
            ; mov eax, -0x80000000
            ; cmp eax, 1
        ) [CF ZF SF OF],
        cmp_rnd_1: (
            ; mov eax, 0x3e9c87ab
            ; cmp eax, 0x47f38608
        ) [CF ZF SF OF],
        cmp_rnd_2: (
            ; mov eax, -0x403f0352
            ; cmp eax, -0x4440a37e
        ) [CF ZF SF OF],
        cmp_rnd_3: (
            ; mov eax, 0x2600bb16
            ; cmp eax, 0x73fc32b6
        ) [CF ZF SF OF],
    }
}

mod lea {
    test_snippets! {
        lea_disp: (
            ; mov eax, 1228
            ; lea ecx, [eax + 7]
        ),
        lea_idx: (
            ; mov eax, 1228
            ; mov ebx, 337
            ; lea ecx, [eax + ebx*4]
        ),
        lea_idx_disp: (
            ; mov eax, 1228
            ; mov ebx, 337
            ; lea ecx, [eax + ebx*4 + 7]
        ),
    }
}

mod dec {
    test_snippets! {
        dec_0: (
            ; mov eax, 0
            ; dec eax
        ),
        dec_1: (
            ; mov eax, 1
            ; dec eax
        ),
        dec_neg_1: (
            ; mov eax, -1
            ; dec eax
        ),
        dec_neg_2: (
            ; mov eax, -2
            ; dec eax
        ),
        dec_neg_0x80000000: (
            ; mov eax, -0x80000000
            ; dec eax
        ),
        dec_0x7fffffff: (
            ; mov eax, 0x7fffffff
            ; dec eax
        ),
    }
    test_snippets! {
        dec_16_0: (
            ; mov ax, 0
            ; dec ax
        ),
        dec_16_1: (
            ; mov ax, 1
            ; dec ax
        ),
        dec_16_neg_1: (
            ; mov ax, -1
            ; dec ax
        ),
        dec_16_neg_2: (
            ; mov ax, -2
            ; dec ax
        ),
        dec_16_neg_0x8000: (
            ; mov ax, -0x8000
            ; dec ax
        ),
        dec_16_0x7fff: (
            ; mov ax, 0x7fff
            ; dec ax
        ),
    }
    test_snippets! {
        dec_8_0: (
            ; mov al, 0
            ; dec al
        ),
        dec_8_1: (
            ; mov al, 1
            ; dec al
        ),
        dec_8_neg_1: (
            ; mov al, -1
            ; dec al
        ),
        dec_8_neg_2: (
            ; mov al, -2
            ; dec al
        ),
        dec_8_neg_0x80: (
            ; mov al, -0x80
            ; dec al
        ),
        dec_8_0x7f: (
            ; mov al, 0x7f
            ; dec al
        ),
    }
}

mod inc {
    test_snippets! {
        inc_0: (
            ; mov eax, 0
            ; inc eax
        ),
        inc_1: (
            ; mov eax, 1
            ; inc eax
        ),
        inc_neg_1: (
            ; mov eax, -1
            ; inc eax
        ),
        inc_neg_2: (
            ; mov eax, -2
            ; inc eax
        ),
        inc_neg_0x80000000: (
            ; mov eax, -0x80000000
            ; inc eax
        ),
        inc_0x7fffffff: (
            ; mov eax, 0x7fffffff
            ; inc eax
        ),
    }
    test_snippets! {
        inc_16_0: (
            ; mov ax, 0
            ; inc ax
        ),
        inc_16_1: (
            ; mov ax, 1
            ; inc ax
        ),
        inc_16_neg_1: (
            ; mov ax, -1
            ; inc ax
        ),
        inc_16_neg_2: (
            ; mov ax, -2
            ; inc ax
        ),
        inc_16_neg_0x8000: (
            ; mov ax, -0x8000
            ; inc ax
        ),
        inc_16_0x7fff: (
            ; mov ax, 0x7fff
            ; inc ax
        ),
    }
    test_snippets! {
        inc_8_0: (
            ; mov al, 0
            ; inc al
        ),
        inc_8_1: (
            ; mov al, 1
            ; inc al
        ),
        inc_8_neg_1: (
            ; mov al, -1
            ; inc al
        ),
        inc_8_neg_2: (
            ; mov al, -2
            ; inc al
        ),
        inc_8_neg_0x80: (
            ; mov al, -0x80
            ; inc al
        ),
        inc_8_0x7f: (
            ; mov al, 0x7f
            ; inc al
        ),
    }
}

mod mem {
    use crate::common::MEM_ADDR;
    test_snippets! {
        mem_basic_rw: (
            ; mov eax, 42
            ; mov eax, [MEM_ADDR as i32]
            ; mov [MEM_ADDR as i32], ebx
        ),
    }
}

mod imul {
    test_snippets! {
        // imul_1op_eax_eax: (
        //     ; mov eax, 23
        //     ; imul eax
        // ) [CF OF],
        // imul_1op: (
        //     ; mov eax, 23
        //     ; mov ebx, 24
        //     ; imul ebx
        // ) [CF OF],
        // imul_1op_overflow: (
        //     ; mov eax, 0x7fffffff
        //     ; mov ebx, 0x7fffffff
        //     ; imul ebx
        // ) [CF OF],

        imul_2op_eax_eax: (
            ; mov eax, 23
            ; imul eax, eax
        ) [CF OF],
        imul_2op: (
            ; mov eax, 23
            ; mov ebx, 24
            ; imul eax, ebx
        ) [CF OF],
        imul_2op_overflow: (
            ; mov eax, 0x7fffffff
            ; mov ebx, 0x7fffffff
            ; imul eax, ebx
        ) [CF OF],
        imul_2op_rnd1: (
            ; mov eax, -0x2c333634
            ; mov ebx, 0x47ec9023
            ; imul eax, ebx
        ) [CF OF],
        imul_2op_rnd2: (
            ; mov eax, -0x23f11f0a
            ; mov ebx, -0x2073452e
            ; imul eax, ebx
        ) [CF OF],
        imul_2op_rnd3: (
            ; mov eax, 0x4f0e4a0c
            ; mov ebx, -0xefd25f
            ; imul eax, ebx
        ) [CF OF],

        // imul_3op_eax_eax: (
        //     ; mov eax, 23
        //     ; imul eax, eax, 24
        // ) [CF OF],
        // imul_3op: (
        //     ; mov ebx, 24
        //     ; imul eax, ebx, 23
        // ) [CF OF],
        // imul_3op_overflow: (
        //     ; mov ebx, 0x7fffffff
        //     ; imul eax, ebx, 0x7fffffff
        // ) [CF OF],
    }
}

mod xor {
    test_snippets! {
        xor_zero_eax: (
            ; mov eax, 228
            ; xor eax, eax
        ) [CF ZF SF OF],
        xor_zero_eax_with_ebx: (
            ; mov eax, 228
            ; mov ebx, 228
            ; xor eax, ebx
        ) [CF ZF SF OF],
        xor_eax_ebx_rnd1: (
            ; mov eax, 0x79d1e0e9
            ; mov ebx, -0x16d29593
            ; xor eax, ebx
        ) [CF ZF SF OF],
        xor_eax_ebx_rnd2: (
            ; mov eax, 0x79f9322a
            ; mov ebx, 0x801efd8
            ; xor eax, ebx
        ) [CF ZF SF OF],
    }
}

mod not {
    test_snippets! {
        not_228: (
            ; mov eax, 228
            ; not eax
        ) [CF ZF SF OF],
        not_zero: (
            ; mov eax, 0
            ; not eax
        ) [CF ZF SF OF],
        not_ffffffff: (
            ; mov eax, -1
            ; not eax
        ) [CF ZF SF OF],
        not_rnd: (
            ; mov eax, 0x79f9322a
            ; not eax
        ) [CF ZF SF OF],
        not_16_rnd: (
            ; mov eax, 0x79f9322a
            ; not ax
        ) [CF ZF SF OF],
        not_8_rnd: (
            ; mov eax, 0x79f9322a
            ; not al
        ) [CF ZF SF OF],
    }
}

mod and {
    test_snippets! {
        and_same_eax_eax: (
            ; mov eax, 228
            ; and eax, eax
        ) [CF ZF SF OF],
        and_same_eax_ebx: (
            ; mov eax, 228
            ; mov ebx, 228
            ; and eax, ebx
        ) [CF ZF SF OF],
        and_eax_ebx_rnd1: (
            ; mov eax, 0x79d1e0e9
            ; mov ebx, -0x16d29593
            ; and eax, ebx
        ) [CF ZF SF OF],
        and_eax_ebx_rnd2: (
            ; mov eax, 0x79f9322a
            ; mov ebx, 0x801efd8
            ; and eax, ebx
        ) [CF ZF SF OF],
    }
}

mod test {
    test_snippets! {
        test_same_eax_eax: (
            ; mov eax, 228
            ; test eax, eax
        ) [CF ZF SF OF],
        test_same_eax_ebx: (
            ; mov eax, 228
            ; mov ebx, 228
            ; test eax, ebx
        ) [CF ZF SF OF],
        test_eax_ebx_rnd1: (
            ; mov eax, 0x79d1e0e9
            ; mov ebx, -0x16d29593
            ; test eax, ebx
        ) [CF ZF SF OF],
        test_eax_ebx_rnd2: (
            ; mov eax, 0x79f9322a
            ; mov ebx, 0x801efd8
            ; test eax, ebx
        ) [CF ZF SF OF],
    }
}

mod shr {
    test_snippets! {
        shr_zero: (
            ; mov eax, 228
            ; shr eax, 0
        ) [CF ZF SF OF],

        shr_228_one: (
            ; mov eax, 228
            ; shr eax, 1
        ) [CF ZF SF OF],
        shr_229_one: (
            ; mov eax, 229
            ; shr eax, 1
        ) [CF ZF SF OF],
        shr_neg_228_one: (
            ; mov eax, -228
            ; shr eax, 1
        ) [CF ZF SF OF],
        shr_neg_229_one: (
            ; mov eax, -229
            ; shr eax, 1
        ) [CF ZF SF OF],

        shr_228_two: (
            ; mov eax, 228
            ; shr eax, 2
        ) [CF ZF SF],
        shr_229_two: (
            ; mov eax, 229
            ; shr eax, 2
        ) [CF ZF SF],
        shr_neg_228_two: (
            ; mov eax, -228
            ; shr eax, 2
        ) [CF ZF SF],
        shr_neg_229_two: (
            ; mov eax, -229
            ; shr eax, 2
        ) [CF ZF SF],

        shr_228_zero_wrap: (
            ; mov eax, 228
            ; shr eax, 32
        ) [CF ZF SF OF],

        shr_228_one_wrap: (
            ; mov eax, 228
            ; shr eax, 33
        ) [CF ZF SF OF],
        shr_229_one_wrap: (
            ; mov eax, 229
            ; shr eax, 33
        ) [CF ZF SF OF],
        shr_neg_228_one_wrap: (
            ; mov eax, -228
            ; shr eax, 33
        ) [CF ZF SF OF],
        shr_neg_229_one_wrap: (
            ; mov eax, -229
            ; shr eax, 33
        ) [CF ZF SF OF],

        shr_228_two_wrap: (
            ; mov eax, 228
            ; shr eax, 34
        ) [CF ZF SF],
        shr_229_two_wrap: (
            ; mov eax, 229
            ; shr eax, 34
        ) [CF ZF SF],
        shr_neg_228_two_wrap: (
            ; mov eax, -228
            ; shr eax, 34
        ) [CF ZF SF],
        shr_neg_229_two_wrap: (
            ; mov eax, -229
            ; shr eax, 34
        ) [CF ZF SF],
    }
}

mod sar {
    test_snippets! {
        sar_zero: (
            ; mov eax, 228
            ; sar eax, 0
        ) [CF ZF SF OF],

        sar_228_one: (
            ; mov eax, 228
            ; sar eax, 1
        ) [CF ZF SF OF],
        sar_229_one: (
            ; mov eax, 229
            ; sar eax, 1
        ) [CF ZF SF OF],
        sar_neg_228_one: (
            ; mov eax, -228
            ; sar eax, 1
        ) [CF ZF SF OF],
        sar_neg_229_one: (
            ; mov eax, -229
            ; sar eax, 1
        ) [CF ZF SF OF],

        sar_228_two: (
            ; mov eax, 228
            ; sar eax, 2
        ) [CF ZF SF],
        sar_229_two: (
            ; mov eax, 229
            ; sar eax, 2
        ) [CF ZF SF],
        sar_neg_228_two: (
            ; mov eax, -228
            ; sar eax, 2
        ) [CF ZF SF],
        sar_neg_229_two: (
            ; mov eax, -229
            ; sar eax, 2
        ) [CF ZF SF],

        sar_228_zero_wrap: (
            ; mov eax, 228
            ; sar eax, 32
        ) [CF ZF SF OF],

        sar_228_one_wrap: (
            ; mov eax, 228
            ; sar eax, 33
        ) [CF ZF SF OF],
        sar_229_one_wrap: (
            ; mov eax, 229
            ; sar eax, 33
        ) [CF ZF SF OF],
        sar_neg_228_one_wrap: (
            ; mov eax, -228
            ; sar eax, 33
        ) [CF ZF SF OF],
        sar_neg_229_one_wrap: (
            ; mov eax, -229
            ; sar eax, 33
        ) [CF ZF SF OF],

        sar_228_two_wrap: (
            ; mov eax, 228
            ; sar eax, 34
        ) [CF ZF SF],
        sar_229_two_wrap: (
            ; mov eax, 229
            ; sar eax, 34
        ) [CF ZF SF],
        sar_neg_228_two_wrap: (
            ; mov eax, -228
            ; sar eax, 34
        ) [CF ZF SF],
        sar_neg_229_two_wrap: (
            ; mov eax, -229
            ; sar eax, 34
        ) [CF ZF SF],

        // basically https://github.com/nepx/halfix/issues/7
        sar_edge_case_byte: (
            ; mov al, -0x08
            ; sar al, 0x09
        ) [CF ZF SF],
        sar_edge_case_word: (
            ; mov ax, -0x0888
            ; sar ax, 0x11
        ) [CF ZF SF],
        sar_edge_case_dword: (
            ; mov eax, -0x08888888
            ; sar eax, 0x21
        ) [CF ZF SF],
    }
}

mod div {
    test_snippets!(
        div_basic1: (
            ; mov eax, 42
            ; mov ebx, 24
            ; div ebx
        ),
        div_basic2: (
            ; mov eax, 1
            ; mov ebx, 888
            ; div ebx
        ),
        div_basic3: (
            ; mov eax, 888
            ; mov ebx, 1
            ; div ebx
        ),
        div_basic4: (
            ; mov eax, 1
            ; mov ebx, 2
            ; div ebx
        ),
        div_rnd1: (
            ; mov eax, -0x57549d35
            ; mov ebx, 0x4003cb02
            ; div ebx
        ),
        div_rnd2: (
            ; mov eax, 0x37ab7947
            ; mov ebx, -0x6d61d34
            ; div ebx
        ),
        div_rnd3: (
            ; mov eax, 0x3a64b162
            ; mov ebx, -0x502df7b4
            ; div ebx
        ),
        div_big1: (
            ; mov eax, 0
            ; mov edx, 1
            ; mov ebx, 2
            ; div ebx
        ),
        // this should cause a division error
        // TODO: how can we test this? (it's not how it behaves rn btw)
        // ditto for division by zero
        // div_big2: (
        //     ; mov eax, 0
        //     ; mov edx, 1
        //     ; mov ebx, 1
        //     ; div ebx
        // ),
        div_big_rnd1: (
            ; mov eax, -0x1895c25a
            ; mov edx, 0x6c8300d6
            ; mov ebx, 0x70a45624
            ; div ebx
        ),
        div_big_rnd2: (
            ; mov eax, -0x21c0f
            ; mov edx, 0x338001
            ; mov ebx, 0x90ed24d
            ; div ebx
        ),
        div_big_rnd3: (
            ; mov eax, 0x74f1d28c
            ; mov edx, 0x7507473a
            ; mov ebx, -0x7d79c77f
            ; div ebx
        ),
    );
}

mod stack {
    test_snippets!(
        push_eax_pop_ebx: (
            ; mov eax, 42
            ; push eax
            ; pop ebx
        ) [CF ZF SF OF],
        push_eax_ebx: (
            ; mov eax, 42
            ; push eax
            ; push ebx
        ) [CF ZF SF OF],

        // TODO: test leave instruction
        leave_fixed: (
            ; push DWORD 0x1337
            ; mov ebp, esp
            ; leave
            ; ret
        ) [CF ZF SF OF],

        enter_leave: (
            ; push ebp
            ; mov ebp, esp
            ; sub esp, 0x100

            ; leave
            ; ret
        ) [CF ZF SF OF],
    );
}

mod string {
    mod scas {
        use crate::common::MEM_ADDR;

        test_snippets! {
            scasb_eq: (
                ; mov BYTE [MEM_ADDR as i32], 0x11
                ; mov edi, MEM_ADDR as i32
                ; mov al, 0x11
                ; scasb
            ) [CF ZF SF OF],
            scasb_less: (
                ; mov BYTE [MEM_ADDR as i32], 0x11
                ; mov edi, MEM_ADDR as i32
                ; mov al, 0x10
                ; scasb
            ) [CF ZF SF OF],
            scasb_greater: (
                ; mov BYTE [MEM_ADDR as i32], 0x11
                ; mov edi, MEM_ADDR as i32
                ; mov al, 0x12
                ; scasb
            ) [CF ZF SF OF],

            scasb_less_signed: (
                ; mov BYTE [MEM_ADDR as i32], 0x11
                ; mov edi, MEM_ADDR as i32
                ; mov al, -1
                ; scasb
            ) [CF ZF SF OF],

            scasb_greater_signed: (
                ; mov BYTE [MEM_ADDR as i32], -1
                ; mov edi, MEM_ADDR as i32
                ; mov al, 2
                ; scasb
            ) [CF ZF SF OF],


            scasb_repe_4: (
                ; mov DWORD [MEM_ADDR as i32], 0x11121111
                ; mov edi, MEM_ADDR as i32
                ; mov al, 0x11
                ; mov ecx, 0x4
                ; repe scasb
            ) [CF ZF SF OF],
            scasb_repe_1: (
                ; mov DWORD [MEM_ADDR as i32], 0x11121111
                ; mov edi, MEM_ADDR as i32
                ; mov al, 0x11
                ; mov ecx, 0x1
                ; repe scasb
            ) [CF ZF SF OF],

            scasb_repne_4: (
                ; mov DWORD [MEM_ADDR as i32], 0x11001111
                ; mov edi, MEM_ADDR as i32
                ; mov al, 0x0
                ; mov ecx, 0x4
                ; repne scasb
            ) [CF ZF SF OF],
            scasb_repne_1: (
                ; mov DWORD [MEM_ADDR as i32], 0x11001111
                ; mov edi, MEM_ADDR as i32
                ; mov al, 0x0
                ; mov ecx, 0x1
                ; repne scasb
            ) [CF ZF SF OF],
        }
        test_snippets! {
            scasw_eq: (
                ; mov WORD [MEM_ADDR as i32], 0x11
                ; mov edi, MEM_ADDR as i32
                ; mov ax, 0x11
                ; scasw
            ) [CF ZF SF OF],
            scasw_less: (
                ; mov WORD [MEM_ADDR as i32], 0x11
                ; mov edi, MEM_ADDR as i32
                ; mov ax, 0x10
                ; scasw
            ) [CF ZF SF OF],
            scasw_greater: (
                ; mov WORD [MEM_ADDR as i32], 0x11
                ; mov edi, MEM_ADDR as i32
                ; mov ax, 0x12
                ; scasw
            ) [CF ZF SF OF],

            scasw_less_signed: (
                ; mov WORD [MEM_ADDR as i32], 0x11
                ; mov edi, MEM_ADDR as i32
                ; mov ax, -1
                ; scasw
            ) [CF ZF SF OF],

            scasw_greater_signed: (
                ; mov WORD [MEM_ADDR as i32], -1
                ; mov edi, MEM_ADDR as i32
                ; mov ax, 2
                ; scasw
            ) [CF ZF SF OF],


            scasw_repe_4: (
                ; mov DWORD [MEM_ADDR as i32], 0x00110011
                ; mov DWORD [MEM_ADDR as i32 + 4], 0x00110012
                ; mov edi, MEM_ADDR as i32
                ; mov ax, 0x11
                ; mov ecx, 0x4
                ; repe scasw
            ) [CF ZF SF OF],
            scasw_repe_1: (
                ; mov DWORD [MEM_ADDR as i32], 0x00110011
                ; mov DWORD [MEM_ADDR as i32 + 4], 0x00110012
                ; mov edi, MEM_ADDR as i32
                ; mov ax, 0x11
                ; mov ecx, 0x1
                ; repe scasw
            ) [CF ZF SF OF],

            scasw_repne_4: (
                ; mov DWORD [MEM_ADDR as i32], 0x00110011
                ; mov DWORD [MEM_ADDR as i32 + 4], 0x00110000
                ; mov edi, MEM_ADDR as i32
                ; mov ax, 0x0
                ; mov ecx, 0x4
                ; repne scasw
            ) [CF ZF SF OF],
            scasw_repne_1: (
                ; mov DWORD [MEM_ADDR as i32], 0x00110011
                ; mov DWORD [MEM_ADDR as i32 + 4], 0x00110000
                ; mov edi, MEM_ADDR as i32
                ; mov ax, 0x0
                ; mov ecx, 0x1
                ; repne scasw
            ) [CF ZF SF OF],
        }
        test_snippets! {
            scasd_eq: (
                ; mov DWORD [MEM_ADDR as i32], 0x11
                ; mov edi, MEM_ADDR as i32
                ; mov eax, 0x11
                ; scasd
            ) [CF ZF SF OF],
            scasd_less: (
                ; mov DWORD [MEM_ADDR as i32], 0x11
                ; mov edi, MEM_ADDR as i32
                ; mov eax, 0x10
                ; scasd
            ) [CF ZF SF OF],
            scasd_greater: (
                ; mov DWORD [MEM_ADDR as i32], 0x11
                ; mov edi, MEM_ADDR as i32
                ; mov eax, 0x12
                ; scasd
            ) [CF ZF SF OF],

            scasd_less_signed: (
                ; mov DWORD [MEM_ADDR as i32], 0x11
                ; mov edi, MEM_ADDR as i32
                ; mov eax, -1
                ; scasd
            ) [CF ZF SF OF],

            scasd_greater_signed: (
                ; mov DWORD [MEM_ADDR as i32], -1
                ; mov edi, MEM_ADDR as i32
                ; mov eax, 2
                ; scasd
            ) [CF ZF SF OF],


            scasd_repe_4: (
                ; mov DWORD [MEM_ADDR as i32], 0x00000011
                ; mov DWORD [MEM_ADDR as i32 + 4], 0x00000011
                ; mov DWORD [MEM_ADDR as i32 + 8], 0x00000012
                ; mov DWORD [MEM_ADDR as i32 + 12], 0x00000011
                ; mov edi, MEM_ADDR as i32
                ; mov eax, 0x11
                ; mov ecx, 0x4
                ; repe scasd
            ) [CF ZF SF OF],
            scasd_repe_1: (
                ; mov DWORD [MEM_ADDR as i32], 0x00000011
                ; mov DWORD [MEM_ADDR as i32 + 4], 0x00000011
                ; mov DWORD [MEM_ADDR as i32 + 8], 0x00000012
                ; mov DWORD [MEM_ADDR as i32 + 12], 0x00000011
                ; mov edi, MEM_ADDR as i32
                ; mov eax, 0x11
                ; mov ecx, 0x1
                ; repe scasd
            ) [CF ZF SF OF],

            scasd_repne_4: (
                ; mov DWORD [MEM_ADDR as i32], 0x00000011
                ; mov DWORD [MEM_ADDR as i32 + 4], 0x00000011
                ; mov DWORD [MEM_ADDR as i32 + 8], 0x00000000
                ; mov DWORD [MEM_ADDR as i32 + 12], 0x00000011
                ; mov edi, MEM_ADDR as i32
                ; mov eax, 0x0
                ; mov ecx, 0x4
                ; repne scasd
            ) [CF ZF SF OF],
            scasd_repne_1: (
                ; mov DWORD [MEM_ADDR as i32], 0x00000011
                ; mov DWORD [MEM_ADDR as i32 + 4], 0x00000011
                ; mov DWORD [MEM_ADDR as i32 + 8], 0x00000000
                ; mov DWORD [MEM_ADDR as i32 + 12], 0x00000011
                ; mov edi, MEM_ADDR as i32
                ; mov eax, 0x0
                ; mov ecx, 0x1
                ; repne scasd
            ) [CF ZF SF OF],
        }
    }
}
