;.data

;.text

;.global test_asm
test_asm:
	mov rax, 3
	add rax, rdi
	ret

;.global init
init:
	ret

step:

